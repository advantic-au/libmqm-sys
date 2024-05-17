extern crate bindgen;

use regex::Regex;
use std::panic::catch_unwind;
use std::path::{Path, PathBuf};
use std::{env, io};

use bindgen::callbacks::{IntKind, ParseCallbacks};

// Feature filter to bring consistency in managing lists that are filterable by enabled features
type FeatureFilter<'a, T> = (&'a [T], Option<&'a [&'a str]>);

/// Filter iterator by selected build feature
fn filter_features<'a, T: 'a>(features: impl Iterator<Item = &'a FeatureFilter<'a, T>>) -> impl Iterator<Item = &'a T> {
    features
        .filter(|&(.., feature)| match feature {
            Some(names) => names
                .iter()
                .map(|name| env::var("CARGO_FEATURE_".to_string() + &(name.to_uppercase()))) // Retrieve the feature env variable
                .any(|f| f.is_ok()), // At least one feature must match
            None => true, // None means always include
        })
        .flat_map(|&(x, ..)| x)
}

/// Source files that are built by cc to support the bindings
const SOURCE_FILES: &[FeatureFilter<&str>] = &[
    (&["src/c/defaults.c", "src/c/strings.c"], None), // MQI, MQAI, Strings
    (&["src/c/exits.c"], Some(&["exits"])),           // Exits
    (&["src/c/pcf.c"], Some(&["pcf"])),               // PCF
];

/// Header files that bindgen uses to generate bindings
const HEADER_FILES: &[FeatureFilter<&str>] = &[
    (
        &[
            "cmqc.h",    // MQI
            "cmqxc.h",   // Exits and MQCD (required for MQI)
            "cmqstrc.h", // Strings
        ],
        None,
    ), // MQI
    (&["cmqbc.h"], Some(&["mqai"])), // MQAI
    (&["cmqcfc.h"], Some(&["pcf"])), // PCF
];

/// Functions that have bindings generated
const FUNCTIONS: &[FeatureFilter<&str>] = &[(&["MQ.+"], None), (&["mq.+"], Some(&["mqai"]))];

/// Structures that have bindings generated
const TYPES: &[FeatureFilter<&str>] = &[
    (
        &[
            "MQMD", "MQMDE", "MQMD1", "MQMD2", "MQPD", "MQIMPO", "MQMHBO", "MQBO", "MQDMHO", "MQCMHO", "MQSRO", "MQSD",
            "MQGMO", "MQPMO", "MQOD", "MQCNO", "MQCD", "MQCSP", "MQSCO", "MQBNO", "MQAIR", "MQBMHO", "MQCBD",
            "MQCHARV", "MQCIH", "MQCTLO", "MQDH", "MQDLH", "MQDMPO", "MQIIH", "MQOR", "MQRFH", "MQRFH2", "MQRMH",
            "MQRR", "MQSMPO", "MQSTS", "MQTM", "MQTMC2", "MQWIH", "MQXQH",
        ],
        None,
    ),
    (
        &[
            "MQCFH", "MQCFBF", "MQCFBS", "MQCFGR", "MQCFIF", "MQCFIL", "MQCFIL64", "MQCFIN", "MQCFIN64", "MQCFSF",
            "MQCFSL", "MQCFST", "MQEPH",
        ],
        Some(&["pcf"]),
    ),
    (
        &[
            "MQACH", "MQAXC", "MQAXP", "MQCXP", "MQDXP", "MQNXP", "MQPBC", "MQPSXP", "MQSBC", "MQWCR", "MQWDR",
            "MQWDR1", "MQWDR2", "MQWQR", "MQWQR1", "MQWQR2", "MQWQR3", "MQWQR4", "MQWXP", "MQWXP1", "MQWXP2", "MQWXP3",
            "MQWXP4", "MQXEPO",
        ],
        Some(&["exits"]),
    ),
];

/// Rules sequentually applied to constants to determine target rust type
const DEF_CONST: &[(&[&str], IntKind)] = &[
    (
        &["^MQ.*_ERROR$"], // Errors are always MQLONG
        IntKind::Custom {
            name: "MQLONG",
            is_signed: true,
        },
    ),
    (
        &[".+_LENGTH(_.)?"], // All lengths should be usize
        IntKind::Custom {
            name: "usize",
            is_signed: false,
        },
    ),
    (
        &["^MQHM_.+"], // Message handles
        IntKind::Custom {
            name: "MQHMSG",
            is_signed: true,
        },
    ),
    (
        &["^MQHO_.+"], // Object handles
        IntKind::Custom {
            name: "MQHOBJ",
            is_signed: true,
        },
    ),
    (
        &["^MQHC_.+"], // Connection handles
        IntKind::Custom {
            name: "MQHCONN",
            is_signed: true,
        },
    ),
    (&["^MQ.+_MASK$"], IntKind::U32), // _MASKS's are frequently defined outside of the i32 range
    (
        &["^MQ_?[A-Z]{2,12}_.+"], // All remaining constants should be MQLONG
        IntKind::Custom {
            name: "MQLONG",
            is_signed: true,
        },
    ),
];
#[derive(Debug)]
struct MQCTypeChooser(Vec<(Vec<Regex>, IntKind)>);
impl ParseCallbacks for MQCTypeChooser {
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        let Self(chooser) = self;
        chooser
            .iter()
            .find(|&(matchers, ..)| matchers.iter().any(|r| r.is_match(name)))
            .map(|&(.., int_kind)| int_kind)
    }
}

fn default_mq_home() -> &'static str {
    env::var("CARGO_CFG_WINDOWS").map(|_| "c:/Program Files/IBM/MQ").unwrap_or("/opt/mqm")
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
    catch_unwind(|| {
        cc::Build::new()
            .static_flag(false)
            .flag_if_supported("-nostartfiles")
            .include(mq_inc_path)
            .files(filter_features(SOURCE_FILES.iter()))
            .warnings(true)
            .compile("mq_functions")
    })
    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to compile c files"))
}

fn generate_bindings(mq_inc_path: &Path) -> Result<bindgen::Bindings, bindgen::BindgenError> {
    let chooser = MQCTypeChooser(
        DEF_CONST
            .iter()
            .map(|&(re_list, kind)| {
                (
                    re_list
                        .iter()
                        .map(|re| Regex::new(re).expect("\"{re}\" is not valid"))
                        .collect(),
                    kind,
                )
            })
            .collect(),
    );

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::builder()
        .generate_cstr(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Chooose the correct type for the various MQC constants
        .parse_callbacks(Box::new(chooser))
        // Allow all constants
        .allowlist_var(".*");

    // Choose the IBM MQI c headers
    let builder = filter_features(HEADER_FILES.iter())
        // Add all the header files
        .fold(builder, |builder, header| {
            builder.header(mq_inc_path.join(header).to_str().expect("\"{header}\" is not valid"))
        });

    // Choose the types
    let builder = filter_features(TYPES.iter()).fold(builder, |builder, &struc| builder.allowlist_type(struc));

    // Choose the functions
    let builder = filter_features(FUNCTIONS.iter()).fold(builder, |builder, &func| builder.allowlist_function(func));

    // Generate the bindings file
    builder.generate()
}

fn main() -> Result<(), io::Error> {
    let mq_home_path = PathBuf::from(env::var("MQ_HOME").unwrap_or_else(|_| default_mq_home().to_string()));
    let out_path = PathBuf::from(env::var("OUT_DIR").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?); // Mandatory OUT_DIR
    let mq_inc_path = mq_home_path.join(inc_path());

    if env::var("CARGO_FEATURE_LINK").is_ok() {
        let mq_lib_path = mq_home_path.join(lib_path());
        println!("cargo:rustc-link-search={}", mq_lib_path.display());
        println!("cargo:rustc-link-lib=dylib={}", link_lib());
    }

    build_c(&mq_inc_path)?; // Build the c files

    // Generate and write the bindings file
    generate_bindings(&mq_inc_path)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        .write_to_file(out_path.join("bindings.rs"))
}
