extern crate bindgen;

use regex::Regex;
use std::panic::catch_unwind;
use std::path::PathBuf;
use std::{env, io};

use bindgen::callbacks::{IntKind, ParseCallbacks};

type FeatureFilter<'a, T> = (T, Option<&'a str>);

/// Source files that are built by cc to support the bindings
const SOURCE_FILES: &[FeatureFilter<&str>] = &[
    ("src/c/defaults.c", None),       // MQI, MQAI
    ("src/c/strings.c", None),        // Strings
    ("src/c/exits.c", Some("exits")), // Exits
    ("src/c/pcf.c", Some("pcf")),     // PCF
];

/// Header files that bindgen uses to generate bindings
const HEADER_FILES: &[FeatureFilter<&str>] = &[
    ("cmqc.h", None),          // MQI
    ("cmqxc.h", None),         // Exits and MQCD (required for MQI)
    ("cmqstrc.h", None),       // Strings
    ("cmqbc.h", Some("mqai")), // MQAI
    ("cmqcfc.h", Some("pcf")), // PCF
];

/// Functions that have bindings generated
const FUNCTIONS: &[FeatureFilter<&str>] = &[
    ("MQ.+", None),
    ("mq.+", Some("mqai"))
];

/// Structures that have bindings generated
const TYPES: &[FeatureFilter<&[&str]>] = &[
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
        Some("pcf"),
    ),
    (
        &[
            "MQACH", "MQAXC", "MQAXP", "MQCXP", "MQDXP", "MQNXP", "MQPBC", "MQPSXP", "MQSBC", "MQWCR", "MQWDR",
            "MQWDR1", "MQWDR2", "MQWQR", "MQWQR1", "MQWQR2", "MQWQR3", "MQWQR4", "MQWXP", "MQWXP1", "MQWXP2", "MQWXP3",
            "MQWXP4", "MQXEPO",
        ],
        Some("exits"),
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
        &["^MQ[A-Z]{2,12}_.+"], // All remaining constants should be MQLONG
        IntKind::Custom {
            name: "MQLONG",
            is_signed: true,
        },
    ),
];

#[derive(Debug)]
struct MQCTypeChooser;
impl ParseCallbacks for MQCTypeChooser {
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        DEF_CONST
            .iter()
            .find(|&(matchers, _)| matchers.iter().any(|r| Regex::new(r).unwrap().is_match(name)))
            .map(|&(_, int_kind)| int_kind)
    }
}

/// Predicate to filter by selected build features
fn feature_filter<T>((.., feature): &(T, Option<&str>)) -> bool {
    match feature {
        Some(name) => env::var("CARGO_FEATURE_".to_string() + &(name.to_uppercase())).is_ok(),
        None => true,
    }
}

fn main() -> Result<(), io::Error> {
    let mq_home_path = PathBuf::from(env::var("MQ_HOME").unwrap_or_else(|_| "/opt/mqm".to_owned()));
    let out_path = PathBuf::from(env::var("OUT_DIR").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?); // Mandatory OUT_DIR

    if env::var("CARGO_FEATURE_LINK").is_ok() {
        let mq_lib_path = mq_home_path.join("lib64");
        println!("cargo:rustc-link-search={}", mq_lib_path.display());
        println!("cargo:rustc-link-lib=mqm_r");
    }

    let mq_inc_path = mq_home_path.join("inc");

    let sources = SOURCE_FILES
        .iter()
        .filter(|t| feature_filter(t))
        .map(|(source, ..)| source);

    catch_unwind(|| {
        cc::Build::new()
            .static_flag(false)
            .flag_if_supported("-nostartfiles")
            .include(&mq_inc_path)
            .files(sources)
            .warnings(true)
            .compile("defaults")
    })
    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to compile c files"))?;

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::builder()
        .generate_cstr(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Chooose the correct type for the various MQC constants
        .parse_callbacks(Box::new(MQCTypeChooser))
        // Allow all constants
        .allowlist_var(".*");

    // Choose the IBM MQI c headers
    let builder = HEADER_FILES
        .iter()
        .filter(|t| feature_filter(t))
        // Add all the header files
        .try_fold(builder, |builder, (header, ..)| {
            mq_inc_path
                .join(header)
                .to_str()
                .ok_or(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("\"{header}\" is not valid"),
                ))
                .map(|h| builder.header(h))
        })?;
    
    // Choose the types
    let builder = TYPES
        .iter()
        .filter(|t| feature_filter(t))
        .flat_map(|(struc, ..)| *struc)
        .fold(builder, |builder, struc| builder.allowlist_type(struc));

    // Choose the functions
    let builder = FUNCTIONS
        .iter()
        .filter(|t| feature_filter(t))
        .fold(builder, |builder, (func, ..)| builder.allowlist_function(func));

    // Generate the bindings file
    builder
        .generate()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))? // Generate the bindings
        .write_to_file(out_path.join("bindings.rs")) // Write the bindings file
}
