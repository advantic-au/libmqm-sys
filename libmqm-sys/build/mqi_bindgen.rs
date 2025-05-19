use std::path::Path;

#[cfg(feature = "bindgen")]
pub mod mqi {
    use std::path::Path;

    use bindgen::callbacks::{IntKind, ParseCallbacks};
    use regex_lite::Regex;

    use crate::features::{filtered, FeatureFilter};

    pub fn mqi_bindgen_builder(builder: bindgen::Builder, mq_inc_path: &Path) -> bindgen::Builder {
        let chooser = MQCTypeChooser(
            MQI_DEF_CONST
                .iter()
                .map(|(re_list, kind)| {
                    (
                        re_list
                            .iter()
                            .map(|re| Regex::new(re).expect("regular expression to be valid"))
                            .collect(),
                        *kind,
                    )
                })
                .collect(),
        );
        let builder = builder
            .parse_callbacks(Box::new(chooser))
            // Allow all constants
            .allowlist_var(".*")
            .header("src/c/ccsid.h");

        // Choose the IBM MQI c headers
        let builder = filtered(MQI_HEADER_FILES)
            // Add all the header files
            .fold(builder, |builder, header| {
                builder.header(mq_inc_path.join(header).to_str().expect("header to be valid"))
            });

        // Choose the types
        let builder = filtered(MQI_STRUCTS).fold(builder, |builder, &struc| builder.allowlist_type(struc));

        // Choose the functions
        let builder = filtered(MQI_FUNCTIONS).fold(builder, |builder, &func| builder.allowlist_function(func));

        builder
    }

    /// Header files that bindgen uses to generate bindings
    const MQI_HEADER_FILES: &[FeatureFilter<&str>] = &[
        (
            &[
                "cmqc.h",  // MQI
                "cmqxc.h", // MQCD (required for MQI)
            ],
            None,
        ), // MQI
        (&["cmqbc.h", "cmqcfc.h"], Some(&["mqai"])),            // MQAI
        (&["cmqcfc.h"], Some(&["pcf"])),                        // PCF
        (&["cmqec.h", "cmqxc.h", "cmqzc.h"], Some(&["exits"])), // IEP, exits and installable services
    ];

    /// Functions that have bindings generated
    const MQI_FUNCTIONS: &[FeatureFilter<&str>] = &[(&["MQ.+"], None), (&["mq.+"], Some(&["mqai"]))];

    // Note: Any additions to TYPES should be matched in the libmqm-default build script
    /// Structures that have bindings generated
    const MQI_STRUCTS: &[FeatureFilter<&str>] = &[
        (
            &[
                "MQMD", "MQMDE", "MQMD1", "MQMD2", "MQPD", "MQIMPO", "MQMHBO", "MQBO", "MQDMHO", "MQCMHO", "MQSRO", "MQSD",
                "MQGMO", "MQPMO", "MQOD", "MQCNO", "MQCD", "MQCSP", "MQSCO", "MQBNO", "MQAIR", "MQBMHO", "MQCBC", "MQCBD",
                "MQCHARV", "MQCIH", "MQCTLO", "MQDH", "MQDLH", "MQDMPO", "MQIIH", "MQOR", "MQRFH", "MQRFH2", "MQRMH", "MQRR",
                "MQSMPO", "MQSTS", "MQTM", "MQTMC2", "MQWIH", "MQXQH",
            ],
            None,
        ),
        (
            &[
                "MQCFH", "MQCFBF", "MQCFBS", "MQCFGR", "MQCFIF", "MQCFIL", "MQCFIL64", "MQCFIN", "MQCFIN64", "MQCFSF", "MQCFSL",
                "MQCFST", "MQEPH",
            ],
            Some(&["pcf"]),
        ),
        (
            &[
                "MQACH", "MQAXC", "MQAXP", "MQCXP", "MQDXP", "MQNXP", "MQPBC", "MQPSXP", "MQSBC", "MQWCR", "MQWDR", "MQWDR1",
                "MQWDR2", "MQWQR", "MQWQR1", "MQWQR2", "MQWQR3", "MQWQR4", "MQWXP", "MQWXP1", "MQWXP2", "MQWXP3", "MQWXP4",
                "MQXEPO", "MQIEP", "MQZED", "MQZAC", "MQZAD", "MQZFP", "MQZIC",
            ],
            Some(&["exits"]),
        ),
    ];

    /// Rules sequentually applied to constants to determine target rust type
    pub const MQI_DEF_CONST: &[(&[&str], IntKind)] = &[
        (
            &["^MQ.*_ERROR$", "^MQIA_.+", "^MQRC_.+"],
            IntKind::Custom {
                name: "MQLONG",
                is_signed: true,
            },
        ),
        (
            &[
                ".+_CURRENT_LENGTH$",
                ".+_STRUC.*_LENGTH",
                ".+_LENGTH_[0-9]+$",
                "^MQ_.+_LEN(GTH)?$",
            ], // All lengths should be usize
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
        (
            &["^MQ_?[A-Z0-9]{1,12}_.+"], // All remaining constants should be MQLONG
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
}

#[cfg(feature = "constant_lookup")]
pub mod str {
    use std::path::Path;

    const STR_HEADER_FILES: &[&str] = &["cmqc.h", "cmqstrc.h"];

    #[cfg(feature = "constant_lookup")]
    pub fn str_bindings_builder(builder: bindgen::Builder, mq_inc_path: &Path) -> bindgen::Builder {
        STR_HEADER_FILES
            .iter()
            .fold(builder, |builder, header| {
                builder.header(mq_inc_path.join(header).to_str().expect("header to be valid"))
            })
            .allowlist_function("MQ.+_STR")
            .allowlist_var("MQ.+_STR")
    }
}

pub fn bindgen_builder(mq_inc_path: &Path, mq_version: &str) -> bindgen::Builder {
    #[allow(deprecated, reason = "RustTarget::Stable_1_82 is deprecated.")]
    bindgen::builder()
        .rust_target(bindgen::RustTarget::Stable_1_82)
        .clang_arg(format!("-I{}", mq_inc_path.display()))
        .raw_line(format!("/* Generated with MQ client version {mq_version} */"))
        .sort_semantically(true)
        .merge_extern_blocks(true)
        .generate_cstr(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
}
