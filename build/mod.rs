use std::io;

#[cfg(feature = "bindgen")]
mod mqi_bindgen;

#[cfg(any(feature = "mqi_helpers", feature = "bindgen"))]
mod features {
    use std::env;

    // Feature filter to bring consistency in managing lists that are filterable by enabled features
    pub type FeatureFilter<'a, T> = (&'a [T], Option<&'a [&'a str]>);

    fn feature_enabled(name: &str) -> bool {
        env::var("CARGO_FEATURE_".to_string() + name.to_uppercase().as_str()).is_ok()
    }

    /// Filter iterator by selected build feature
    pub fn filter_features<'a, T: 'a>(
        features: impl IntoIterator<Item = &'a FeatureFilter<'a, T>>,
    ) -> impl Iterator<Item = &'a T> {
        features
            .into_iter()
            .filter(|&(.., feature)| match feature {
                Some(names) => names.iter().copied().any(feature_enabled), // At least one feature must match
                None => true,                                              // None means always include
            })
            .flat_map(|&(x, ..)| x)
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
        let sources = super::features::filter_features(SOURCE_FILES).collect::<Vec<_>>();
        for source in &sources {
            println!("cargo:rerun-if-changed={source}")
        }

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
        let (_, [a, b, c, d]) = mqc_regex.captures(feature).map(|captures| captures.extract())?;
        ver_into_u32((a, b, c, d)).ok()
    }

    pub fn parse_version(version: &str) -> Option<u32> {
        const VERSION_PATTERN: &str = r"(\d+)\.(\d+)\.(\d+)\.(\d+)";
        let version_check = Regex::new(VERSION_PATTERN).expect("valid regex");

        let (_, [a, b, c, d]) = version_check.captures(version).map(|captures| captures.extract())?;
        ver_into_u32((a, b, c, d)).ok()
    }

    pub fn generate_version(target: &mut impl std::io::Write, version: &str) -> std::io::Result<()> {
        let version_int = parse_version(version)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid version: {}", version)))?;

        writeln!(target, "pub const CLIENT_BUILD_VERSION: &str = \"{}\";", version)?;
        writeln!(
            target,
            "pub(crate) const CLIENT_BUILD_VERSION_INT: u32 = {:#010x};",
            version_int
        )?;

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

    pub fn dspmqver() -> PathBuf {
        home_path().join(
            env::var("CARGO_CFG_WINDOWS")
                .map(|_| "bin64/dspmqver.exe")
                .unwrap_or("bin/dspmqver"),
        )
    }
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

    #[cfg(feature = "bindgen")]
    {
        use regex_lite::Regex;
        use std::fs::File;
        use std::process::{Command, Stdio};

        // Generate and write the bindings file
        let out_path =
            std::path::PathBuf::from(std::env::var("OUT_DIR").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?); // Mandatory OUT_DIR
        let out_bindings = out_path.join("bindings.rs");
        let out_version = out_path.join("version.rs");

        let dspmqver_path = mq_path::dspmqver();
        let dspmqver_raw = Command::new(&dspmqver_path).stdout(Stdio::piped()).output()?;
        let dspmqver_output =
            String::from_utf8(dspmqver_raw.stdout).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mqc_version = Regex::new(r"(?m)^\s*Version:\s+(?<version>.*?)\s*$")
            .expect("valid regex")
            .captures(&dspmqver_output)
            .map(|m| m["version"].to_owned())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "could not extract version from dspmqver"))?;

        let mqc_env = Regex::new(r"CARGO_FEATURE_(MQC_\d+_\d+_\d+_\d+)").expect("valid regex");
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
            if min_mqc > current_mqc {
                panic!(
                    "MQC version {} does not meet the minimum requirement for feature {}",
                    &current_mqc, &feature
                );
            }
        }

        {
            let mut version_writer = io::BufWriter::new(File::create(&out_version)?);
            versions::generate_version(&mut version_writer, &mqc_version)?;
        }

        mqi_bindgen::generate_bindings(&mq_path::mq_inc_path(), &mqc_version)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .write_to_file(out_bindings.clone())?;

        #[cfg(feature = "pregen")]
        {
            use std::{env::consts as env_consts, fs, path};

            fs::copy(
                out_bindings,
                path::PathBuf::from("./src/lib/pregen").join(format!(
                    "{}-{}-bindings.rs",
                    if env_consts::OS == "macos" { "any" } else { env_consts::ARCH },
                    env_consts::OS
                )),
            )?;

            fs::copy(out_version, path::PathBuf::from("src/lib/pregen").join("version.rs"))?;
        }
    }

    Ok(())
}
