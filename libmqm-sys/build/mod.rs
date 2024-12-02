use std::io;

#[cfg(feature = "bindgen")]
mod mqi_bindgen;

#[cfg(any(feature = "mqi_helpers", feature = "bindgen"))]
mod features {
    use std::env;

    // Feature filter to bring consistency in managing lists that are filterable by enabled features
    pub type FeatureFilter<'a, T> = (&'a [T], Option<&'a [&'a str]>);

    fn is_enabled(name: &str) -> bool {
        env::var("CARGO_FEATURE_".to_string() + name.to_uppercase().as_str()).is_ok()
    }

    /// Filter iterator by selected build feature
    pub fn filtered<'a, T: 'a>(features: impl IntoIterator<Item = &'a FeatureFilter<'a, T>>) -> impl Iterator<Item = &'a T> {
        features
            .into_iter()
            .filter(|(.., feature)| feature.map_or(true, |names| names.iter().copied().any(is_enabled)))
            .flat_map(|(x, ..)| *x)
    }
}

#[cfg(feature = "mqi_helpers")]
mod mqi_helpers {
    use std::{io, path::PathBuf};

    /// Source files that are built by cc to support the bindings
    const SOURCE_FILES: &[super::features::FeatureFilter<&str>] = &[
        (&["src/c/defaults.c", "src/c/strings.c"], None), // MQI, MQAI, Strings
        (&["src/c/exits.c"], Some(&["exits"])),           // Exits
        (&["src/c/pcf.c"], Some(&["pcf"])),               // PCF
    ];

    pub fn build_c(mq_inc_path: &PathBuf) -> Result<(), io::Error> {
        let sources = super::features::filtered(SOURCE_FILES).collect::<Vec<_>>();
        for source in &sources {
            println!("cargo:rerun-if-changed={source}");
        }
        println!("cargo:rerun-if-changed={}", mq_inc_path.display());

        cc::Build::new()
            .static_flag(false)
            .flag_if_supported("-nostartfiles")
            .include(mq_inc_path)
            .files(sources)
            .warnings(true)
            .try_compile("mqi_helpers")
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to compile c files"))
    }
}

#[cfg(feature = "bindgen")]
mod versions {
    use regex_lite::Regex;

    fn ver_into_u32((a, b, c, d): (&str, &str, &str, &str)) -> Result<u32, std::num::ParseIntError> {
        Ok((a.parse::<u32>()? << 24) | (b.parse::<u32>()? << 16) | (c.parse::<u32>()? << 8) | (d.parse::<u32>()?))
    }

    pub fn parse_mqc_feature(feature: &str) -> Option<u32> {
        let mqc_regex = Regex::new(r"(?i)mqc_(\d+)_(\d+)_(\d+)_(\d+)").ok()?;
        let (_, version) = mqc_regex.captures(feature).map(|captures| captures.extract())?;
        ver_into_u32(version.into()).ok()
    }

    pub fn parse_version(version: &str) -> Option<u32> {
        const VERSION_PATTERN: &str = r"(\d+)\.(\d+)\.(\d+)\.(\d+)";
        let version_check = Regex::new(VERSION_PATTERN).expect("valid regex");

        let (_, version) = version_check.captures(version).map(|captures| captures.extract())?;
        ver_into_u32(version.into()).ok()
    }

    pub fn generate_version(target: &mut impl std::io::Write, version: &str) -> std::io::Result<()> {
        let version_int = parse_version(version)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid version: {version}")))?;

        writeln!(target, "pub const CLIENT_BUILD_VERSION: &str = \"{version}\";")?;
        writeln!(target, "pub const CLIENT_BUILD_VERSION_INT: u32 = {version_int:#010x};")?;

        Ok(())
    }
}

#[cfg(feature = "link_mqm")]
mod link_mqm {
    use std::env;

    pub fn link_lib() -> &'static str {
        env::var("CARGO_CFG_WINDOWS").map(|_| "mqm").unwrap_or("mqm_r")
    }

    pub fn lib_path() -> &'static str {
        env::var("CARGO_CFG_WINDOWS").map(|_| "tools/lib64").unwrap_or("lib64")
    }
}

#[cfg(any(feature = "link_mqm", feature = "mqi_helpers", feature = "bindgen"))]
mod mq_path {
    use std::{env, path::PathBuf};

    fn default_home() -> &'static str {
        env::var("CARGO_CFG_WINDOWS")
            .map(|_| "c:/Program Files/IBM/MQ")
            .unwrap_or("/opt/mqm")
    }

    fn inc_sub_path() -> &'static str {
        env::var("CARGO_CFG_WINDOWS").map(|_| "tools/c/include").unwrap_or("inc")
    }

    pub fn home_path() -> PathBuf {
        PathBuf::from(env::var("MQ_HOME").unwrap_or_else(|_| default_home().to_string()))
    }

    pub fn mq_inc_path() -> PathBuf {
        home_path().join(inc_sub_path())
    }
}

#[cfg(feature = "versiongen")]
#[must_use]
fn dspmqver() -> std::path::PathBuf {
    mq_path::home_path().join(
        std::env::var("CARGO_CFG_WINDOWS")
            .map(|_| "bin64/dspmqver.exe")
            .unwrap_or("bin/dspmqver"),
    )
}

fn main() -> Result<(), io::Error> {
    println!("cargo:rerun-if-env-changed=MQ_HOME");

    #[cfg(feature = "link_mqm")]
    {
        let mq_lib_path = mq_path::home_path().join(link_mqm::lib_path());
        println!("cargo:rustc-link-search={}", mq_lib_path.display());
        println!("cargo:rustc-link-lib=dylib={}", link_mqm::link_lib());
    }

    #[cfg(feature = "mqi_helpers")]
    mqi_helpers::build_c(&mq_path::mq_inc_path())?; // Build the c files

    #[cfg(feature = "versiongen")]
    {
        let out_path =
            std::path::PathBuf::from(std::env::var("OUT_DIR").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?); // Mandatory OUT_DIR

        let out_version = out_path.join("version.rs");

        let dspmqver_path = dspmqver();
        let dspmqver_raw = std::process::Command::new(&dspmqver_path)
            .stdout(std::process::Stdio::piped())
            .output()?;
        let dspmqver_output =
            String::from_utf8(dspmqver_raw.stdout).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mqc_version = regex_lite::Regex::new(r"(?m)^\s*Version:\s+(?<version>.*?)\s*$")
            .expect("valid regex")
            .captures(&dspmqver_output)
            .map(|m| m["version"].to_owned())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "could not extract version from dspmqver"))?;

        let mqc_env = regex_lite::Regex::new(r"CARGO_FEATURE_(MQC_\d+_\d+_\d+_\d+)").expect("valid regex");
        let min_mqc_version = std::env::vars()
            .filter_map(|(name, _)| {
                mqc_env.captures(&name).and_then(|captures| {
                    captures.get(1).and_then(|matched| {
                        let feature = matched.as_str().to_lowercase();
                        versions::parse_mqc_feature(&feature).map(|ver| (ver, feature))
                    })
                })
            })
            .max_by_key(|(ver, _)| *ver);

        if let Some(((min_mqc, feature), current_mqc)) = min_mqc_version.zip(versions::parse_version(&mqc_version)) {
            assert!(
                min_mqc <= current_mqc,
                "MQC version {} does not meet the minimum requirement for feature {}",
                &mqc_version,
                &feature
            );
        }

        let mut version_writer = io::BufWriter::new(std::fs::File::create(&out_version)?);
        versions::generate_version(&mut version_writer, &mqc_version)?;
        drop(version_writer);

        #[cfg(feature = "pregen")]
        std::fs::copy(out_version, std::path::PathBuf::from("src/version").join("pregen.rs"))?;

        #[cfg(feature = "bindgen")]
        {
            let out_bindings = out_path.join("bindings.rs");

            // Generate and write the bindings file
            mqi_bindgen::generate_bindings(&mq_path::mq_inc_path(), &mqc_version)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
                .write_to_file(&out_bindings)?;

            #[cfg(feature = "pregen")]
            {
                use std::{env::consts, fs, path};

                fs::copy(
                    out_bindings,
                    path::PathBuf::from("./src/lib/pregen").join(format!(
                        "{}-{}-bindings.rs",
                        if consts::OS == "macos" { "any" } else { consts::ARCH },
                        consts::OS
                    )),
                )?;
            }
        }
    }

    Ok(())
}
