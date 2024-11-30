use std::path::Path;

use bindgen::callbacks::{IntKind, ParseCallbacks};
use regex_lite::Regex;

use super::features::{filter_features, FeatureFilter};

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
    (&["cmqbc.h", "cmqcfc.h"], Some(&["mqai"])), // MQAI
    (&["cmqec.h", "cmqcfc.h"], Some(&["pcf"])),  // PCF
];

/// Functions that have bindings generated
const FUNCTIONS: &[FeatureFilter<&str>] = &[(&["MQ.+"], None), (&["mq.+"], Some(&["mqai"]))];

// Note: Any additions to TYPES should be matched in the libmqm-default build script
/// Structures that have bindings generated
const TYPES: &[FeatureFilter<&str>] = &[
    (
        &[
            "MQMD", "MQMDE", "MQMD1", "MQMD2", "MQPD", "MQIMPO", "MQMHBO", "MQBO", "MQDMHO", "MQCMHO", "MQSRO", "MQSD", "MQGMO",
            "MQPMO", "MQOD", "MQCNO", "MQCD", "MQCSP", "MQSCO", "MQBNO", "MQAIR", "MQBMHO", "MQCBC", "MQCBD", "MQCHARV", "MQCIH",
            "MQCTLO", "MQDH", "MQDLH", "MQDMPO", "MQIIH", "MQOR", "MQRFH", "MQRFH2", "MQRMH", "MQRR", "MQSMPO", "MQSTS", "MQTM",
            "MQTMC2", "MQWIH", "MQXQH",
        ],
        None,
    ),
    (
        &[
            "MQCFH", "MQCFBF", "MQCFBS", "MQCFGR", "MQCFIF", "MQCFIL", "MQCFIL64", "MQCFIN", "MQCFIN64", "MQCFSF", "MQCFSL",
            "MQCFST", "MQEPH", "MQZED", "MQZAC", "MQZAD", "MQZFP", "MQZIC",
        ],
        Some(&["pcf"]),
    ),
    (
        &[
            "MQACH", "MQAXC", "MQAXP", "MQCXP", "MQDXP", "MQNXP", "MQPBC", "MQPSXP", "MQSBC", "MQWCR", "MQWDR", "MQWDR1",
            "MQWDR2", "MQWQR", "MQWQR1", "MQWQR2", "MQWQR3", "MQWQR4", "MQWXP", "MQWXP1", "MQWXP2", "MQWXP3", "MQWXP4", "MQXEPO",
            "MQIEP",
        ],
        Some(&["exits"]),
    ),
];

/// Rules sequentually applied to constants to determine target rust type
pub const DEF_CONST: &[(&[&str], IntKind)] = &[
    (
        &["^MQ.*_ERROR$", "^MQIA_.+"],
        IntKind::Custom {
            name: "MQLONG",
            is_signed: true,
        },
    ),
    (
        &[".+_LENGTH(_.)?", ".+_LEN$"], // All lengths should be usize
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
pub struct MQCTypeChooser(Vec<(Vec<Regex>, IntKind)>);
impl ParseCallbacks for MQCTypeChooser {
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        let Self(chooser) = self;
        chooser
            .iter()
            .find(|(matchers, ..)| matchers.iter().any(|r| r.is_match(name)))
            .map(|(.., int_kind)| *int_kind)
    }
}

pub fn generate_bindings(mq_inc_path: &Path, mq_version: &str) -> Result<bindgen::Bindings, bindgen::BindgenError> {
    let chooser = MQCTypeChooser(
        DEF_CONST
            .iter()
            .map(|(re_list, kind)| {
                (
                    re_list
                        .iter()
                        .map(|re| Regex::new(re).expect("\"{re}\" to be valid"))
                        .collect(),
                    *kind,
                )
            })
            .collect(),
    );

    let ccsid_inc_path = Path::new("src/c");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::builder()
        .rust_target(bindgen::RustTarget::Stable_1_77)
        .clang_arg(format!("-I{}", mq_inc_path.display()))
        .clang_arg(format!("-I{}", ccsid_inc_path.display()))
        .raw_line(format!("/* Generated with MQ client version {mq_version} */"))
        .sort_semantically(true)
        .merge_extern_blocks(true)
        .generate_cstr(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Chooose the correct type for the various MQC constants
        .parse_callbacks(Box::new(chooser))
        // Allow all constants
        .allowlist_var(".*")
        .header("ccsid.h");

    // Choose the IBM MQI c headers
    let builder = filter_features(HEADER_FILES)
        // Add all the header files
        .fold(builder, |builder, header| {
            builder.header(mq_inc_path.join(header).to_str().expect("\"{header}\" is not valid"))
        });

    // Choose the types
    let builder = filter_features(TYPES).fold(builder, |builder, &struc| builder.allowlist_type(struc));

    // Choose the functions
    let builder = filter_features(FUNCTIONS).fold(builder, |builder, &func| builder.allowlist_function(func));

    // Generate the bindings file
    builder.generate()
}
