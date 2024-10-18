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
    use std::{io, panic::catch_unwind, path::PathBuf};

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
        catch_unwind(|| {
            cc::Build::new()
                .static_flag(false)
                .flag_if_supported("-nostartfiles")
                .include(mq_inc_path)
                .files(sources)
                .warnings(true)
                .compile("mq_functions")
        })
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to compile c files"))
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
        use std::process::{Command, Stdio};

        // Generate and write the bindings file
        let out_path =
            std::path::PathBuf::from(std::env::var("OUT_DIR").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?); // Mandatory OUT_DIR
        let out_bindings = out_path.join("bindings.rs");

        let dspmqver_output = String::from_utf8(Command::new(mq_path::dspmqver()).stdout(Stdio::piped()).output()?.stdout)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mq_version = Regex::new(r"\s*Version:\s+(?<version>.*)")
            .expect("valid regex")
            .captures(&dspmqver_output)
            .map(|m| m["version"].to_owned())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "could not extract version from dspmqver"))?;

        mqi_bindgen::generate_bindings(&mq_path::mq_inc_path(), &mq_version)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .write_to_file(out_bindings.clone())?;

        #[cfg(feature = "pregen")]
        {
            use std::{env::consts as env_consts, fs, path};

            fs::copy(
                out_bindings,
                path::PathBuf::from("./src/lib/pregen").join(format!(
                    "{}-{}-bindings.rs",
                    if env_consts::OS == "macOS" { "any" } else { env_consts::ARCH },
                    env_consts::OS
                )),
            )?;
        }
    }

    Ok(())
}
