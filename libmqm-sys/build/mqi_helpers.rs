use std::{io, path::PathBuf};

/// Source files that are built by cc to support the bindings
const SOURCE_FILES: &[crate::features::FeatureFilter<&str>] = &[
    crate::features::FeatureFilter {
        sources: &["src/c/strings.c"],
        features: Some(&["constant_lookup"]),
    }, // _STR functions
    crate::features::FeatureFilter {
        sources: &["src/c/mqi.c"],
        features: Some(&["struct_defaults"]),
    }, // MQI defaults
    crate::features::FeatureFilter {
        sources: &["src/c/exits.c"],
        features: Some(&["struct_defaults", "exits"]),
    }, // exit defaults
    crate::features::FeatureFilter {
        sources: &["src/c/pcf.c"],
        features: Some(&["struct_defaults", "pcf"]),
    }, // PCF defaults
];

pub fn build_c(mq_inc_path: &PathBuf) -> Result<(), io::Error> {
    let sources = crate::features::filtered(SOURCE_FILES).collect::<Vec<_>>();
    for source in &sources {
        println!("cargo:rerun-if-changed={source}");
    }
    println!("cargo:rerun-if-changed={}", mq_inc_path.display());

    cc::Build::new()
        .flag_if_supported("-nostartfiles")
        .include(mq_inc_path)
        .files(sources)
        .warnings(true)
        .try_compile("mqi_helpers")
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to compile c files"))
}
