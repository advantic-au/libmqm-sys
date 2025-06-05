use std::io;

#[cfg(any(feature = "bindgen", feature = "constant_lookup"))]
mod mqi_bindgen;

#[cfg(any(feature = "struct_defaults", feature = "constant_lookup", feature = "bindgen"))]
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
            .filter(|(.., feature)| feature.is_none_or(|names| names.iter().copied().all(is_enabled)))
            .flat_map(|(x, ..)| *x)
    }
}

#[cfg(any(feature = "constant_lookup", feature = "struct_defaults"))]
mod mqi_helpers {
    use std::{io, path::PathBuf};

    /// Source files that are built by cc to support the bindings
    const SOURCE_FILES: &[super::features::FeatureFilter<&str>] = &[
        (&["src/c/strings.c"], Some(&["constant_lookup"])),        // _STR functions
        (&["src/c/mqi.c"], Some(&["struct_defaults"])),            // MQI defaults
        (&["src/c/exits.c"], Some(&["struct_defaults", "exits"])), // exit defaults
        (&["src/c/pcf.c"], Some(&["struct_defaults", "pcf"])),     // PCF defaults
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

#[cfg(feature = "versiongen")]
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

#[allow(dead_code)]
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

#[allow(clippy::unnecessary_wraps, clippy::too_many_lines)]
fn main() -> Result<(), io::Error> {
    println!("cargo:rerun-if-env-changed=MQ_HOME");

    #[cfg(feature = "link_mqm")]
    {
        let mq_lib_path = mq_path::home_path().join(link_mqm::lib_path());
        println!("cargo:rustc-link-search={}", mq_lib_path.display());
        println!("cargo:rustc-link-lib=dylib={}", link_mqm::link_lib());
    }

    #[cfg(any(feature = "struct_defaults", feature = "constant_lookup"))]
    mqi_helpers::build_c(&mq_path::mq_inc_path())?; // Build the c files

    #[cfg(feature = "versiongen")]
    {
        let out_path =
            std::path::PathBuf::from(std::env::var("OUT_DIR").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?); // Mandatory OUT_DIR

        let out_version = out_path.join("version.rs");

        let mqc_version = ["README.Redist", "README.Client", "README.Advanced"]
            .iter()
            .find_map(|file_name| {
                let file_content = std::fs::read_to_string(mq_path::home_path().join(file_name)).ok()?;
                regex_lite::Regex::new(r"(?m)^\s*Version:\s+(?<version>.*?)\s*$")
                    .ok()?
                    .captures(&file_content)
                    .map(|m| m["version"].to_owned())
            })
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "could not extract version from MQ README file"))?;

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
        pregen_copy(&out_version, &std::path::PathBuf::from("./src/version/pregen"))?;

        #[cfg(feature = "constant_lookup")]
        {
            let out_bindings = out_path.join("str.rs");
            let mq_inc_path = mq_path::mq_inc_path();

            let mut out_file = io::BufWriter::new(std::fs::File::create(&out_bindings)?);
            // Generate and write the bindings file
            mqi_bindgen::str::str_bindings_builder(mqi_bindgen::bindgen_builder(&mq_inc_path), &mq_inc_path)
                .generate()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
                .write(Box::new(&mut out_file))?;
        }

        #[cfg(feature = "bindgen")]
        {
            use io::Write as _;

            let mq_inc_path = mq_path::mq_inc_path();
            let builder = mqi_bindgen::bindgen_builder(&mq_inc_path);

            // Generate and write the bindings file
            for (name, generated) in mqi_bindgen::mqi::mqi_bindgen_generate(&builder, &mq_inc_path)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            {
                let mut bindings_str = format!("/* Generated with MQ client version {mqc_version} */\n\n");
                bindings_str += &prettyplease::unparse(&generated);

                // Replace MQLONGs that are too large with wrapped equivalent MQLONG's.
                let mqlong_replace = regex_lite::Regex::new(r"(:\s*MQLONG\s*=\s*)(\d+)\s*;").unwrap();
                let bindings_str = mqlong_replace.replace_all(&bindings_str, |caps: &regex_lite::Captures| {
                    if caps[2].parse::<i32>().is_err() {
                        let i = caps[2].parse::<u32>().unwrap();
                        #[allow(clippy::cast_possible_wrap)]
                        let wrapped = i as i32;
                        format!("{}{};", &caps[1], wrapped)
                    } else {
                        caps[0].to_string()
                    }
                });

                let out_bindings = out_path.join(name);
                let mut out_file = io::BufWriter::new(std::fs::File::create(&out_bindings)?);
                out_file.write_all(bindings_str.as_bytes())?;
                drop(out_file);

                #[cfg(feature = "pregen")]
                pregen_copy_dir(&out_bindings, &std::path::PathBuf::from("./src/lib/pregen"))?;
            }
        }
    }

    Ok(())
}

#[cfg(feature = "pregen")]
fn pregen_copy_dir(out_bindings: &std::path::PathBuf, target: &std::path::Path) -> Result<(), io::Error> {
    use std::fs;
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    fs::copy(
        out_bindings,
        target.join(format!(
            "{}-{}/{}",
            if target_os == "macos" { "any" } else { &target_arch },
            target_os,
            out_bindings
                .file_name()
                .expect("out_bindings includes filename")
                .to_string_lossy()
        )),
    )?;
    Ok(())
}

#[cfg(feature = "pregen")]
fn pregen_copy(out_bindings: &std::path::PathBuf, target: &std::path::Path) -> Result<(), io::Error> {
    use std::fs;
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    fs::copy(
        out_bindings,
        target.join(format!(
            "{}-{}-{}",
            if target_os == "macos" { "any" } else { &target_arch },
            target_os,
            out_bindings
                .file_name()
                .expect("out_bindings includes filename")
                .to_string_lossy()
        )),
    )?;
    Ok(())
}
