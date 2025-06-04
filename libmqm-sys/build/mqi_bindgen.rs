use std::path::Path;

#[cfg(feature = "bindgen")]
pub mod mqi {
    use std::{
        collections::{HashMap, HashSet},
        path::Path,
    };

    use bindgen::{
        callbacks::{IntKind, ParseCallbacks},
        BindgenError,
    };
    use regex_lite::Regex;
    use syn::{
        ForeignItem, ForeignItemFn, ForeignItemStatic, ForeignItemType, Item, ItemConst, ItemFn, ItemStatic, ItemStruct,
        ItemType, Signature,
    };

    #[derive(Debug, Clone, Copy)]
    struct AllowList<'a> {
        functions: Option<&'a str>,
        variables: Option<&'a str>,
        types: Option<&'a str>
    }

    const ALL_ALLOW: AllowList = AllowList {
        functions: Some(".*"),
        variables: Some(".*"),
        types: Some("P?MQ.*"),
    };

    struct HeaderFeature<'a> {
        name: &'static str,
        headers: &'a [&'static str],
        allow_list: AllowList<'a>,
        target_list: &'a [&'static str],
    }

    const HEADER_FEATURE: &[HeaderFeature] = &[
        HeaderFeature {
            name: "base.rs",
            headers: &["cmqc.h"],
            allow_list: ALL_ALLOW,
            target_list: &[],
        },
        HeaderFeature {
            name: "mqi.rs",
            headers: &["cmqc.h", "cmqxc.h"],
            allow_list: AllowList { functions: None, variables: Some("MQCD_.*"), types: Some(".*MQCD") },
            target_list: &[],
        },
        HeaderFeature {
            name: "exits.rs",
            headers: &["cmqc.h", "cmqec.h", "cmqxc.h", "cmqzc.h"],
            allow_list: ALL_ALLOW,
            target_list: &[".*MQIEP.*", ".*MQHCONFIG.*", "P?MQ_.*_CALL"],
        },
        HeaderFeature {
            name: "pcf.rs",
            headers: &["cmqc.h", "cmqcfc.h"],
            allow_list: AllowList { functions: None, variables: Some(".*"), types: Some("P?MQ.*") },
            target_list: &[],
        },
        HeaderFeature {
            name: "mqai.rs",
            headers: &["cmqc.h", "cmqbc.h", "cmqcfc.h"],
            allow_list: ALL_ALLOW,
            target_list: &[],
        },
    ];

    fn filtered_items(items: Vec<Item>, names: &HashSet<String>) -> (HashSet<String>, Vec<Item>) {
        let mut hs = HashSet::new();
        let filtered = items
            .into_iter()
            .filter_map(|item| {
                let name = match &item {
                    Item::Type(ItemType { ident, .. })
                    | Item::Struct(ItemStruct { ident, .. })
                    | Item::Static(ItemStatic { ident, .. })
                    | Item::Fn(ItemFn {
                        sig: Signature { ident, .. },
                        ..
                    })
                    | Item::Const(ItemConst { ident, .. }) => Some(format!("{ident}")),
                    _other => None,
                };
                if let Some(name) = &name {
                    hs.insert(name.clone());
                }
                match (name, item) {
                    (Some(name), item) => (!names.contains(name.as_str())).then_some(item),
                    (None, Item::ForeignMod(mut fm)) => {
                        fm.items = fm
                            .items
                            .into_iter()
                            .filter_map(|item| {
                                let name = match &item {
                                    ForeignItem::Type(ForeignItemType { ident, .. })
                                    | ForeignItem::Static(ForeignItemStatic { ident, .. })
                                    | ForeignItem::Fn(ForeignItemFn {
                                        sig: Signature { ident, .. },
                                        ..
                                    }) => Some(format!("{ident}")),
                                    _other => None,
                                };
                                match name {
                                    Some(name) => {
                                        hs.insert(name.clone());
                                        (!names.contains(name.as_str())).then_some(item)
                                    },
                                    None => Some(item),
                                }
                            })
                            .collect();
                        Some(Item::ForeignMod(fm))
                    }
                    (None, item) => Some(item),
                }
            })
            .collect();
        (hs, filtered)
    }

    pub fn mqi_bindgen_generate(
        builder: &bindgen::Builder,
        mq_inc_path: &Path,
    ) -> Result<HashMap<&'static str, syn::File>, BindgenError> {
        let mut result = HashMap::new();
        let mut items_acc = HashSet::new();

        for (
            pos,
            &HeaderFeature {
                name,
                headers,
                allow_list,
                target_list,
                ..
            },
        ) in HEADER_FEATURE.iter().enumerate()
        {
            // Get the subsequent target_list and make this part of the blocklist
            let blhf = HEADER_FEATURE.get((pos + 1)..);
            let blocklist_other_target = blhf.iter().flat_map(|h| h.iter().flat_map(|h| h.target_list)).copied();

            // Execute the bindgen
            let generated = mqi_bindgen_builder(
                builder.clone(),
                mq_inc_path,
                headers,
                &allow_list,
                target_list.iter().copied(),
                blocklist_other_target, // Block other target items
            )
            .generate()?;
            let mut gen_bytes: Vec<u8> = Vec::new();
            generated.write(Box::new(&mut gen_bytes)).expect("write to buffer should always succeed");
            let mut ast = syn::parse_file(&String::from_utf8_lossy(&gen_bytes)).expect("bindgen generated code should parse");
            let (new_ns, items) = filtered_items(ast.items, &items_acc);
            ast.items = items;
            items_acc.extend(new_ns); 
            assert!(result.insert(name, ast).is_none());
        }
        Ok(result)
    }

    fn mqi_bindgen_builder<'a>(
        builder: bindgen::Builder,
        mq_inc_path: &Path,
        headers: &[&str],
        allow: &AllowList,
        allowlist_item: impl IntoIterator<Item = &'a str>,
        blocklist_item: impl IntoIterator<Item = &'a str>,
    ) -> bindgen::Builder {
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

        let builder = headers.iter().fold(builder, |builder, header| {
            builder.header(mq_inc_path.join(header).to_str().expect("header to be valid"))
        });

        let builder = blocklist_item.into_iter().fold(builder, bindgen::Builder::blocklist_item);
        let builder = allowlist_item.into_iter().fold(builder, bindgen::Builder::allowlist_item);

        // .header("src/c/ccsid.h");

        // let filter_mq_headers = MQI_HEADER_FILES.iter().filter(|(_, features)| match (requested, features) {
        //     (Some(requested), Some(feature_slice)) => feature_slice.contains(&requested),
        //     _ => true, // Required headers
        // });

        // // Choose the IBM MQI c headers
        // let builder = filtered(filter_mq_headers)
        //     // Add all the header files
        //     .fold(builder, |builder, header| {
        //         builder.header(mq_inc_path.join(header).to_str().expect("header to be valid"))
        //     });

        let builder = allow.functions.iter().fold(builder, bindgen::Builder::allowlist_function);
        let builder = allow.variables.iter().fold(builder, bindgen::Builder::allowlist_var);
        let builder = allow.types.iter().fold(builder, bindgen::Builder::allowlist_type);

        builder.parse_callbacks(Box::new(chooser))
    }

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
        .layout_tests(false)
    // .formatter(bindgen::Formatter::Prettyplease)
}
