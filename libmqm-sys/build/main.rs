use std::io;

#[cfg(any(feature = "bindgen", feature = "constant_lookup"))]
mod mqi_bindgen;

#[cfg(feature = "bindgen")]
mod doc_comments;

#[cfg(feature = "bindgen")]
mod rustify;

#[cfg(feature = "bindgen")]
mod mq_trait;

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
        pregen_copy_dir(&out_version, &std::path::PathBuf::from("./src/pregen"))?;

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
            use syn::{parse_quote, visit_mut::VisitMut};

            use crate::{
                doc_comments::{DescriptionRegex, FnParamExtract, StructFieldExtract},
                mq_trait::{PrefixMqTypes, WrapperGenerator},
            };

            let source_trait: &[(_, syn::Ident, Option<syn::LitStr>, bool)] = &[
                ("mqi.rs", syn::parse_quote!(Mqi), None, true),
                ("mqai.rs", syn::parse_quote!(Mqai), Some(syn::parse_quote!("mqai")), true),
                ("exits.rs", syn::parse_quote!(Exits), Some(syn::parse_quote!("exits")), false),
            ];

            let mut traits = vec![];
            let mut mock_impls = vec![];
            let mut dlopen_impls = vec![];
            let mut link_impls = vec![];

            let mq_mod = parse_quote!(lib);
            let mock_name = parse_quote!(Mq);
            let link_name = parse_quote!(LinkedMq);
            let wrapper_name = parse_quote!(MqWrapper);
            let dlopen2_name = parse_quote!(::dlopen2::wrapper::Container<#wrapper_name>);

            let mq_inc_path = mq_path::mq_inc_path();
            let builder = mqi_bindgen::bindgen_builder(&mq_inc_path);

            let comments = doc_comments::extract_from_headers(&mq_inc_path, &DescriptionRegex::default())?;
            assert_ne!(comments.len(), 0);

            let fields = doc_comments::extract_from_headers(&mq_inc_path, &StructFieldExtract::default())?;
            assert_ne!(fields.len(), 0);

            let parameters = doc_comments::extract_from_headers(&mq_inc_path, &FnParamExtract::default())?;
            assert_ne!(parameters.len(), 0);

            let mut wrapper_fields = vec![];

            // Generate and write the bindings file
            for (name, mut generated) in mqi_bindgen::mqi::mqi_bindgen_generate(&builder, &mq_inc_path)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            {
                use syn::{parse_quote, visit::Visit, visit_mut::VisitMut as _};

                use crate::{
                    doc_comments::{DocCommentArgs, DocCommentFields, DocCommentType},
                    mq_trait::TraitGenerator,
                    rustify::{FnArgType, MqLongConstWrap},
                };

                let mut arg_type = FnArgType::new()
                    .replace_all("pCompCode", parse_quote!(&mut MQLONG))
                    .replace_all("pReason", parse_quote!(&mut MQLONG))
                    .replace_all("pHconn", parse_quote!(&mut MQHCONN))
                    .replace_all("pHobj", parse_quote!(&mut MQHOBJ))
                    .replace_all("pHmsg", parse_quote!(&mut MQHMSG))
                    .replace_all("pDataLength", parse_quote!(&mut MQLONG))
                    .replace_all("pName", parse_quote!(&MQCHARV)) // TODO: fix pName mutability
                    .replace_all("pObjDesc", parse_quote!(&mut MQOD))
                    .replace_all("pPutMsgOpts", parse_quote!(&mut MQPMO))
                    .replace_all("pGetMsgOpts", parse_quote!(&mut MQGMO))
                    .replace_all("pPropDesc", parse_quote!(&mut MQPD))
                    .replace_all("pBag", parse_quote!(&mut MQHBAG))
                    .replace_all("pByteStringLength", parse_quote!(&mut MQLONG))
                    .replace_all("pStringLength", parse_quote!(&mut MQLONG))
                    .replace_all("pOperator", parse_quote!(&mut MQLONG))
                    .replace_all("pCodedCharSetId", parse_quote!(&mut MQLONG))
                    .replace_all("pExitOpts", parse_quote!(Option<&MQXEPO>))
                    .replace_all("pExitParms", parse_quote!(&mut MQAXP))
                    .replace_all("pExitContext", parse_quote!(&mut MQAXC))
                    .replace_fns(
                        ["MQCONN", "MQCONNX", "MQ_CONN_CALL", "MQ_CONNX_CALL"],
                        "pQMgrName",
                        &parse_quote!(&MQCHAR48),
                    )
                    .replace_fns(["MQCTL", "MQ_CTL_CALL"], "pControlOpts", &parse_quote!(&MQCTLO))
                    .replace_fns(["MQCRTMH", "MQ_CRTMH_CALL"], "pCrtMsgHOpts", &parse_quote!(&MQCMHO))
                    .replace_fns(["MQDLTMH", "MQ_DLTMH_CALL"], "pDltMsgHOpts", &parse_quote!(&MQDMHO))
                    .replace_fns(["MQDLTMP", "MQ_DLTMP_CALL"], "pDltPropOpts", &parse_quote!(&MQDMPO))
                    .replace_fns(["MQSUB", "MQ_SUB_CALL"], "pHobj", &parse_quote!(Option<&mut MQHOBJ>))
                    .replace_fns(["MQCONNX", "MQ_CONNX_CALL"], "pConnectOpts", &parse_quote!(&mut MQCNO))
                    .replace_fns(
                        ["MQBEGIN", "MQ_BEGIN_CALL"],
                        "pBeginOptions",
                        &parse_quote!(Option<&mut MQBO>),
                    )
                    .replace_fns(["MQBUFMH", "MQ_BUFMH_CALL"], "pBufMsgHOpts", &parse_quote!(&MQBMHO))
                    .replace_fns(["MQCB", "MQ_CB_CALL"], "pCallbackDesc", &parse_quote!(Option<&MQCBD>))
                    .replace_fns(["MQCB", "MQ_CB_CALL"], "pGetMsgOpts", &parse_quote!(Option<&MQGMO>))
                    .replace_fns(["MQINQMP", "MQ_INQMP_CALL"], "pType", &parse_quote!(&mut MQLONG))
                    .replace_fns(["MQINQMP", "MQ_INQMP_CALL"], "pInqPropOpts", &parse_quote!(&mut MQIMPO)) // TODO: inconsistent (header says input only, but doc says input/output
                    .replace_fns(["MQINQMP", "MQ_INQMP_CALL"], "pPropDesc", &parse_quote!(&mut MQPD))
                    .replace_fns(["MQMHBUF", "MQ_MHBUF_CALL"], "pMsgHBufOpts", &parse_quote!(&MQMHBO))
                    .replace_fns(["MQSETMP", "MQ_SETMP_CALL"], "pSetPropOpts", &parse_quote!(&MQSMPO))
                    .replace_fns(["MQSTAT", "MQ_STAT_CALL"], "pStatus", &parse_quote!(&mut MQSTS))
                    .replace_fns(["MQSUB", "MQ_SUB_CALL"], "pSubDesc", &parse_quote!(&mut MQSD))
                    .replace_fns(["MQSUB", "MQ_SUB_CALL"], "pHsub", &parse_quote!(&mut MQHOBJ))
                    .replace_fns(["MQSUBRQ", "MQ_SUBRQ_CALL"], "pSubRqOpts", &parse_quote!(Option<&mut MQSRO>))
                    .replace_fns(["mqCountItems"], "pItemCount", &parse_quote!(&mut MQLONG))
                    .replace_fns(["mqInquireInteger64"], "pItemValue", &parse_quote!(&mut MQINT64))
                    .replace_fns(
                        ["mqInquireInteger", "mqInquireIntegerFilter"],
                        "pItemValue",
                        &parse_quote!(&mut MQLONG),
                    )
                    .replace_fns(["mqInquireItemInfo"], "pOutSelector", &parse_quote!(&mut MQLONG))
                    .replace_fns(["mqInquireItemInfo"], "pItemType", &parse_quote!(&mut MQLONG))
                    .replace_fns(["mqInquireBag"], "pItemValue", &parse_quote!(&mut MQHBAG))
                    .replace_fns(
                        ["MQXCLWLN", "MQ_XCLWLN_CALL", "MQ_CLUSTER_WORKLOAD_EXIT"],
                        "pExitParms",
                        &parse_quote!(&mut MQWXP),
                    )
                    .replace_fns(["MQXCLWLN", "MQ_XCLWLN_CALL"], "pNextRecord", &parse_quote!(&mut MQPTR)) // TODO: MQPTR can be improved
                    .replace_fns(["MQXDX"], "pDataConvExitParms", &parse_quote!(&mut MQDXP))
                    .replace_fns(["MQ_PUBLISH_EXIT"], "pExitParms", &parse_quote!(&mut MQPSXP))
                    .replace_fns(["MQ_TRANSPORT_EXIT"], "pExitParms", &parse_quote!(PMQVOID))
                    .replace_fns(["MQ_PRECONNECT_EXIT"], "pExitParms", &parse_quote!(&mut MQNXP));

                DocCommentArgs(&parameters).visit_file_mut(&mut generated);
                DocCommentType(&comments).visit_file_mut(&mut generated);
                DocCommentFields(&fields).visit_file_mut(&mut generated);
                MqLongConstWrap.visit_file_mut(&mut generated);
                arg_type.visit_file_mut(&mut generated);

                if !source_trait.iter().any(|(source, .., dlopen)| *source == name && !dlopen) {
                    WrapperGenerator(|mut field: syn::Field| {
                        field.attrs.extend(
                            source_trait
                                .iter()
                                .find_map(|(source, .., feat, _)| {
                                    (*source == name && feat.is_some()).then(|| feat.as_ref().unwrap())
                                })
                                .map(|feat| parse_quote!(#[cfg(feature = #feat)])),
                        );
                        wrapper_fields.push(field);
                    })
                    .visit_file(&generated);
                }

                if let Some((_, trait_name, feature, dlopen)) = source_trait.iter().find(|(source, ..)| *source == name) {
                    // Generate the trait

                    use syn::{ImplItemFn, TraitItemFn};

                    use crate::mq_trait::{impl_dlopen2_fn, impl_link_fn, DesugarForMockall, ImplFnGenerator};

                    let feat_cfg = feature.as_ref().map(|feat| parse_quote!(#[cfg(feature = #feat)]));

                    let mut tg = TraitGenerator::default();
                    tg.visit_file(&generated);
                    let mut item_trait = tg.generate(trait_name);
                    item_trait
                        .attrs
                        .extend(feature.as_ref().map(|feat| parse_quote!(#[cfg(feature = #feat)])));
                    PrefixMqTypes(&mq_mod).visit_item_trait_mut(&mut item_trait);

                    // Generate the Mock struct
                    let mut mg = ImplFnGenerator::new(|_: &mut ImplItemFn, _: &TraitItemFn| {});
                    mg.visit_item_trait(&item_trait);
                    let mut mock_impl_trait = mg.generate(&parse_quote!(crate::#trait_name), &mock_name);
                    mock_impl_trait.attrs.extend(feat_cfg.clone());
                    PrefixMqTypes(&mq_mod).visit_item_impl_mut(&mut mock_impl_trait);
                    DesugarForMockall.visit_item_impl_mut(&mut mock_impl_trait);

                    if *dlopen {
                        // Generate the dlopen2 struct
                        let mut dg = ImplFnGenerator::new(impl_dlopen2_fn(&wrapper_name));
                        dg.visit_item_trait(&item_trait);
                        let mut dlopen2_impl_trait = dg.generate(&parse_quote!(crate::#trait_name), &dlopen2_name);
                        dlopen2_impl_trait.attrs.extend(feat_cfg.clone());
                        PrefixMqTypes(&mq_mod).visit_item_impl_mut(&mut dlopen2_impl_trait);
                        dlopen_impls.push(dlopen2_impl_trait);
                    }

                    // Generate the link struct
                    let mut lg = ImplFnGenerator::new(impl_link_fn(&mq_mod));
                    lg.visit_item_trait(&item_trait);
                    let mut link_impl_trait = lg.generate(&parse_quote!(crate::#trait_name), &link_name);
                    link_impl_trait.attrs.extend(feat_cfg.clone());
                    PrefixMqTypes(&mq_mod).visit_item_impl_mut(&mut link_impl_trait);

                    mock_impls.push(mock_impl_trait);
                    traits.push(item_trait);
                    link_impls.push(link_impl_trait);
                }

                let mut bindings_str = format!("/* Generated with MQ client version {mqc_version} */\n\n");
                bindings_str += &prettyplease::unparse(&generated);

                let out_bindings = out_path.join(name);
                let mut out_file = io::BufWriter::new(std::fs::File::create(&out_bindings)?);
                out_file.write_all(bindings_str.as_bytes())?;
                drop(out_file);

                #[cfg(feature = "pregen")]
                pregen_copy_dir(&out_bindings, &std::path::PathBuf::from("./src/pregen"))?;
            }

            let trait_file = parse_quote!(

                use crate::lib;
                #(
                    #[allow(clippy::missing_safety_doc, clippy::too_many_arguments, non_snake_case)]
                    #traits
                )*
            );

            #[rustfmt::skip]
            let mock_file = parse_quote!(
                use crate::lib;
                mockall::mock! {
                    pub #mock_name {}
                    #(
                        #mock_impls
                    )*
                }
            );

            let link_file = parse_quote!(
                use crate::lib;

                /// Provides access to compile time linked MQI and MQAI functions
                #[derive(Debug, Clone, Copy)]
                pub struct #link_name;

                #(
                    #link_impls
                )*

            );

            let mut wrapper_file = parse_quote!(
                use crate::lib;
                use ::dlopen2::wrapper::WrapperApi;

                #[derive(::dlopen2::wrapper::WrapperApi, Debug)]
                pub struct #wrapper_name {
                    #(#wrapper_fields,)*
                }

                #(
                    #dlopen_impls
                )*

            );
            PrefixMqTypes(&mq_mod).visit_file_mut(&mut wrapper_file);

            let out_mock = out_path.join("mock.rs");
            let mut out_file = io::BufWriter::new(std::fs::File::create(&out_mock)?);
            out_file.write_all(prettyplease::unparse(&mock_file).as_bytes())?;
            drop(out_file);

            let out_function = out_path.join("function.rs");
            let mut out_file = io::BufWriter::new(std::fs::File::create(&out_function)?);
            out_file.write_all(prettyplease::unparse(&trait_file).as_bytes())?;
            drop(out_file);

            let out_link = out_path.join("link.rs");
            let mut out_file = io::BufWriter::new(std::fs::File::create(&out_link)?);
            out_file.write_all(prettyplease::unparse(&link_file).as_bytes())?;
            drop(out_file);

            let out_wrapper = out_path.join("dlopen2.rs");
            let mut out_file = io::BufWriter::new(std::fs::File::create(&out_wrapper)?);
            out_file.write_all(prettyplease::unparse(&wrapper_file).as_bytes())?;
            drop(out_file);

            #[cfg(feature = "pregen")]
            {
                std::fs::copy(out_function, std::path::PathBuf::from("./src/pregen/function.rs"))?;
                std::fs::copy(out_mock, std::path::PathBuf::from("./src/pregen/mock.rs"))?;
                std::fs::copy(out_wrapper, std::path::PathBuf::from("./src/pregen/dlopen2.rs"))?;
                std::fs::copy(out_link, std::path::PathBuf::from("./src/pregen/link.rs"))?;
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
