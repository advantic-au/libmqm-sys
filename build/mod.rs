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

mod support {
    use regex_lite::Regex;

    pub fn parse_version(version: &str) -> Option<u32> {
        const VERSION_PATTERN: &str = r"(\d+)\.(\d+)\.(\d+)\.(\d+)";
        let version_check = Regex::new(VERSION_PATTERN).expect("valid regex");

        let (_, [a, b, c, d]) = version_check.captures(version).map(|captures| captures.extract())?;

        Some(
            (a.parse::<u32>().ok()? << 24)
                | (b.parse::<u32>().ok()? << 16)
                | (c.parse::<u32>().ok()? << 8)
                | (d.parse::<u32>().ok()?),
        )
    }

    pub fn parse_cfg_version((a, b, c, d): (u8, u8, u8, u8)) -> u32 {
        (u32::from(a) << 24) | (u32::from(b) << 16) | (u32::from(c) << 8) | u32::from(d)
    }

    pub fn cfg_version_fmt((a, b, c, d): (u8, u8, u8, u8)) -> String {
        format!("mqc_{a}_{b}_{c}_{d}")
    }

    pub const CFG_VERSIONS: &[(u8, u8, u8, u8)] = &[
        (9, 2, 0, 0),
        (9, 2, 0, 5),
        (9, 2, 0, 6),
        (9, 2, 0, 7),
        (9, 2, 0, 10),
        (9, 2, 0, 11),
        (9, 2, 0, 15),
        (9, 2, 0, 16),
        (9, 2, 0, 20),
        (9, 2, 0, 21),
        (9, 2, 0, 22),
        (9, 2, 0, 25),
        (9, 2, 0, 26),
        (9, 2, 0, 27),
        (9, 3, 0, 0),
        (9, 3, 0, 1),
        (9, 3, 0, 2),
        (9, 3, 0, 4),
        (9, 3, 0, 5),
        (9, 3, 0, 6),
        (9, 3, 0, 10),
        (9, 3, 0, 11),
        (9, 3, 0, 15),
        (9, 3, 0, 16),
        (9, 3, 0, 17),
        (9, 3, 0, 20),
        (9, 3, 0, 21),
        (9, 3, 1, 0),
        (9, 3, 2, 0),
        (9, 3, 2, 1),
        (9, 3, 3, 0),
        (9, 3, 3, 1),
        (9, 3, 4, 0), // Auth token support 
        (9, 3, 4, 1),
        (9, 3, 5, 0),
        (9, 3, 5, 1),
        (9, 4, 0, 0),
        (9, 4, 0, 5),
    ];
    pub const CFG_CHECKS: &[(&str, &str)] = &[("mqc_mqwqr4", "./build/c/mqwqr4.c"), ("mqc_mqbno", "./build/c/mqbno.c")];

    #[cfg(feature = "bindgen")]
    pub fn check_compile(cfg: &str, src: &str, mq_inc_path: &std::path::PathBuf) -> Result<(), cc::Error> {
        cc::Build::new()
            .static_flag(false)
            .flag_if_supported("-nostartfiles")
            .include(mq_inc_path)
            .file(src)
            .cargo_output(false)
            .cargo_warnings(false)
            .cargo_metadata(false)
            .cargo_debug(false)
            .try_compile(cfg)
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

    for &cfg in support::CFG_VERSIONS {
        println!("cargo:rustc-check-cfg=cfg({})", support::cfg_version_fmt(cfg));
    }

    for &(cfg, _) in support::CFG_CHECKS {
        println!("cargo:rustc-check-cfg=cfg({cfg})");
        #[cfg(not(feature = "bindgen"))]
        println!("cargo:rustc-cfg={cfg}");
    }

    #[cfg(feature = "bindgen")]
    {
        use regex_lite::Regex;
        use std::process::{Command, Stdio};

        let inc_path = &mq_path::mq_inc_path();

        let supported_cfg = support::CFG_CHECKS
            .iter()
            .flat_map(|&(cfg, src)| support::check_compile(cfg, src, inc_path).map(|_| cfg));

        for cfg in supported_cfg {
            println!("cargo:rustc-cfg={cfg}");
        }

        // Generate and write the bindings file
        let out_path =
            std::path::PathBuf::from(std::env::var("OUT_DIR").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?); // Mandatory OUT_DIR
        let out_bindings = out_path.join("bindings.rs");

        let dspmqver_path = mq_path::dspmqver();
        let dspmqver_raw = Command::new(&dspmqver_path).stdout(Stdio::piped()).output()?;
        let dspmqver_output =
            String::from_utf8(dspmqver_raw.stdout).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mq_version = Regex::new(r"(?m)^\s*Version:\s+(?<version>.*?)\s*$")
            .expect("valid regex")
            .captures(&dspmqver_output)
            .map(|m| m["version"].to_owned())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "could not extract version from dspmqver"))?;

        if let Some(parse_mq_version) = support::parse_version(&mq_version) {
            for version_cfg in support::CFG_VERSIONS
                .iter()
                .filter(|&cfg| support::parse_cfg_version(*cfg) <= parse_mq_version)
            {
                println!("cargo:rustc-cfg={}", support::cfg_version_fmt(*version_cfg));
            }
        }

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
                    if env_consts::OS == "macos" { "any" } else { env_consts::ARCH },
                    env_consts::OS
                )),
            )?;
        }
    }

    Ok(())
}
