use std::panic::catch_unwind;
use std::path::PathBuf;
use std::{env, io};

#[cfg(feature = "bindgen")]
mod mqi_bindgen;

// Feature filter to bring consistency in managing lists that are filterable by enabled features
type FeatureFilter<'a, T> = (&'a [T], Option<&'a [&'a str]>);

fn feature_enabled(name: &str) -> bool {
    env::var("CARGO_FEATURE_".to_string() + name.to_uppercase().as_str()).is_ok()
}

/// Filter iterator by selected build feature
fn filter_features<'a, T: 'a>(features: impl IntoIterator<Item = &'a FeatureFilter<'a, T>>) -> impl Iterator<Item = &'a T> {
    features
        .into_iter()
        .filter(|&(.., feature)| match feature {
            Some(names) => names.iter().copied().any(feature_enabled), // At least one feature must match
            None => true,                                              // None means always include
        })
        .flat_map(|&(x, ..)| x)
}

/// Source files that are built by cc to support the bindings
const SOURCE_FILES: &[FeatureFilter<&str>] = &[
    (&["src/c/defaults.c", "src/c/strings.c"], None), // MQI, MQAI, Strings
    (&["src/c/exits.c"], Some(&["exits"])),           // Exits
    (&["src/c/pcf.c"], Some(&["pcf"])),               // PCF
];

fn default_mq_home() -> &'static str {
    env::var("CARGO_CFG_WINDOWS")
        .map(|_| "c:/Program Files/IBM/MQ")
        .unwrap_or("/opt/mqm")
}

fn link_lib() -> &'static str {
    env::var("CARGO_CFG_WINDOWS").map(|_| "mqm").unwrap_or("mqm_r")
}

fn inc_path() -> &'static str {
    env::var("CARGO_CFG_WINDOWS").map(|_| "tools/c/include").unwrap_or("inc")
}

fn lib_path() -> &'static str {
    env::var("CARGO_CFG_WINDOWS").map(|_| "tools/lib64").unwrap_or("lib64")
}

fn build_c(mq_inc_path: &PathBuf) -> Result<(), io::Error> {
    let sources = filter_features(SOURCE_FILES).collect::<Vec<_>>();
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

fn main() -> Result<(), io::Error> {
    let mq_home_path = PathBuf::from(env::var("MQ_HOME").unwrap_or_else(|_| default_mq_home().to_string()));
    let mq_inc_path = mq_home_path.join(inc_path());

    println!("cargo:rerun-if-env-changed=MQ_HOME");

    #[cfg(feature = "link_mqm")]
    {
        let mq_lib_path = mq_home_path.join(lib_path());
        println!("cargo:rustc-link-search={}", mq_lib_path.display());
        println!("cargo:rustc-link-lib=dylib={}", link_lib());
    }

    #[cfg(feature = "mqi_helpers")]
    build_c(&mq_inc_path)?; // Build the c files

    #[cfg(feature = "bindgen")]
    {
        // Generate and write the bindings file
        let out_path = PathBuf::from(env::var("OUT_DIR").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?); // Mandatory OUT_DIR
        mqi_bindgen::generate_bindings(&mq_inc_path)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .write_to_file(out_path.join("bindings.rs"))?;
    }

    Ok(())
}
