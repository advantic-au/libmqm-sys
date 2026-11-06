use std::io;

#[cfg(any(feature = "bindgen", feature = "constant_lookup"))]
mod mqi_bindgen;

#[cfg(feature = "bindgen")]
mod doc_comments;

#[cfg(feature = "bindgen")]
mod rustify;

#[cfg(feature = "bindgen")]
mod mq_trait;

#[allow(dead_code)]
mod features;

#[cfg(any(feature = "constant_lookup", feature = "struct_defaults"))]
mod mqi_helpers;

#[cfg(feature = "versiongen")]
mod versions;

#[cfg(feature = "link_mqm")]
mod link_mqm;

#[allow(dead_code)]
mod mq_path;

/// Argument type overrides for the generated MQI, MQAI and exits bindings.
///
/// `replace_all` applies to an argument name wherever it appears; `replace_fns` restricts
/// the override to the listed functions.
#[cfg(feature = "bindgen")]
fn arg_type_replacements() -> crate::rustify::FnArgType<'static> {
    use syn::parse_quote;

    crate::rustify::FnArgType::new()
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
        .replace_fns(["MQ_PRECONNECT_EXIT"], "pExitParms", &parse_quote!(&mut MQNXP))
        .replace_fns(["MQCB_FUNCTION"], "pContext", &parse_quote!(&mut MQCBC))
}

/// Generates the MQI, MQAI, exits and PCF bindings, the traits, and the
/// `LinkedMq` / `dlopen2` / `mockall` implementations, writing them to `OUT_DIR`.
#[allow(clippy::too_many_lines)]
#[cfg(feature = "bindgen")]
fn generate_bindings(out_path: &std::path::Path, mqc_version: &str) -> Result<(), io::Error> {
    use indoc::formatdoc;
    use io::Write as _;
    use quote::ToTokens as _;
    use syn::parse_quote;

    use crate::{
        doc_comments::{DescriptionRegex, FnParamExtract, StructFieldExtract},
        mq_trait::WrapperGenerator,
        rustify::PrefixMqTypes,
    };

    fn write_file(file: &syn::File, target: &std::path::Path) -> io::Result<()> {
        let mut out_file = io::BufWriter::new(std::fs::File::create(target)?);
        out_file.write_all(prettyplease::unparse(file).as_bytes())
    }

    /// Builds one trait implementation, applying the feature-gate attribute and the
    /// type-path prefixing that every implementation strategy needs.
    fn generate_impl_trait(
        trait_name: &syn::Ident,
        path: &syn::Path,
        feature_cfg: Option<&syn::Attribute>,
        block_fn: impl Fn(&mut syn::ImplItemFn, &syn::TraitItemFn),
        self_ty: &syn::Path,
        trait_def: &syn::ItemTrait,
    ) -> syn::ItemImpl {
        use syn::visit::Visit as _;
        use syn::visit_mut::VisitMut as _;

        let mut generator = crate::mq_trait::ImplFnGenerator::new(block_fn);
        generator.visit_item_trait(trait_def);
        let mut item_impl = generator.generate(&parse_quote!(crate::#trait_name), self_ty);
        item_impl.attrs.extend(feature_cfg.cloned());
        PrefixMqTypes(path, &std::collections::HashSet::new()).visit_item_impl_mut(&mut item_impl);
        item_impl
    }

    #[derive(Debug, PartialEq)]
    enum TraitImpl {
        Dlopen2,
        Iep,
        Link,
    }
    type SourceTrait<'a, 'b> = (&'a str, Option<syn::Ident>, syn::Path, Option<syn::LitStr>, &'b [TraitImpl]);

    let source_trait: &[SourceTrait] = &[
        (
            "mqi.rs",
            syn::parse_quote!(Mqi),
            syn::parse_quote!(crate),
            None,
            &[TraitImpl::Dlopen2, TraitImpl::Iep, TraitImpl::Link],
        ),
        (
            "mqai.rs",
            syn::parse_quote!(Mqai),
            syn::parse_quote!(crate::mqai),
            syn::parse_quote!("mqai"),
            &[TraitImpl::Dlopen2, TraitImpl::Link],
        ),
        (
            "exits.rs",
            syn::parse_quote!(Exits),
            syn::parse_quote!(crate::exits),
            syn::parse_quote!("exits"),
            &[TraitImpl::Iep],
        ),
        (
            "pcf.rs",
            None,
            syn::parse_quote!(crate::pcf),
            syn::parse_quote!("pcf"),
            &[TraitImpl::Link],
        ),
    ];

    let mut arg_type = arg_type_replacements();

    let mut traits = vec![];
    let mut mock_impls = vec![];
    let mut dlopen_impls = vec![];
    let mut iep_impls = vec![];
    let mut link_impls = vec![];

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
    for (name, mut generated) in mqi_bindgen::mqi::mqi_bindgen_generate(&builder, &mq_inc_path)? {
        use syn::{parse_quote, visit::Visit, visit_mut::VisitMut as _};

        use crate::{
            doc_comments::{DocCommentArgs, DocCommentFields, DocCommentReference, DocCommentType},
            mq_trait::TraitGenerator,
            rustify::MqLongConstWrap,
        };

        DocCommentArgs(&parameters).visit_file_mut(&mut generated);
        DocCommentType(&comments).visit_file_mut(&mut generated);
        DocCommentFields(&fields).visit_file_mut(&mut generated);
        DocCommentReference.visit_file_mut(&mut generated);
        MqLongConstWrap.visit_file_mut(&mut generated);
        arg_type.visit_file_mut(&mut generated);
        let symbols = generated
            .items
            .iter()
            .flat_map(|item| match item {
                syn::Item::Const(item_const) => vec![item_const.ident.clone()],
                syn::Item::Static(item_static) => vec![item_static.ident.clone()],
                syn::Item::Struct(item_struct) => vec![item_struct.ident.clone()],
                syn::Item::Type(item_type) => vec![item_type.ident.clone()],
                syn::Item::ForeignMod(item_foreign) => item_foreign
                    .items
                    .iter()
                    .filter_map(|item| {
                        if let syn::ForeignItem::Fn(foreign_item_fn) = item {
                            Some(foreign_item_fn.sig.ident.clone())
                        } else {
                            None
                        }
                    })
                    .collect(),
                _ => vec![],
            })
            .collect();
        PrefixMqTypes(&parse_quote!(crate), &symbols).visit_file_mut(&mut generated);

        if let Some((.., path, feat, _)) = source_trait
            .iter()
            .find(|(source, .., impls)| *source == name && impls.contains(&TraitImpl::Dlopen2))
        {
            WrapperGenerator(|mut field: syn::Field| {
                if let Some(feat) = &feat {
                    field.attrs.push(parse_quote!(#[cfg(feature = #feat)]));
                }
                PrefixMqTypes(path, &std::collections::HashSet::new()).visit_field_mut(&mut field);
                wrapper_fields.push(field);
            })
            .visit_file(&generated);
        }

        if let Some((_, Some(trait_name), path, feature, impls)) = source_trait.iter().find(|(source, ..)| *source == name) {
            // Generate the trait

            use std::collections::HashSet;

            use syn::{ImplItemFn, TraitItemFn};

            use crate::mq_trait::{DesugarForMockall, ImplFnGenerator};

            let feat_cfg = feature.as_ref().map(|feat| parse_quote!(#[cfg(feature = #feat)]));

            let mut tg = TraitGenerator::default();
            tg.visit_file(&generated);
            let mut item_trait = tg.generate(trait_name);
            item_trait
                .attrs
                .extend(feature.as_ref().map(|feat| parse_quote!(#[cfg(feature = #feat)])));
            PrefixMqTypes(path, &HashSet::new()).visit_item_trait_mut(&mut item_trait);

            // Generate the Mock struct
            let mut mg = ImplFnGenerator::new(|_: &mut ImplItemFn, _: &TraitItemFn| {});
            mg.visit_item_trait(&item_trait);
            let mut mock_impl_trait = mg.generate(&parse_quote!(crate::#trait_name), &mock_name);
            mock_impl_trait.attrs.extend(feat_cfg.clone());
            PrefixMqTypes(path, &HashSet::new()).visit_item_impl_mut(&mut mock_impl_trait);
            DesugarForMockall.visit_item_impl_mut(&mut mock_impl_trait);

            if impls.contains(&TraitImpl::Dlopen2) {
                // Generate the dlopen2 struct
                let item_impl = generate_impl_trait(
                    trait_name,
                    path,
                    feat_cfg.as_ref(),
                    mq_trait::impl_dlopen2_fn(&wrapper_name),
                    &dlopen2_name,
                    &item_trait,
                );
                dlopen_impls.push(item_impl);
            }

            if impls.contains(&TraitImpl::Iep) {
                let item_impl = generate_impl_trait(
                    trait_name,
                    path,
                    feat_cfg.as_ref(),
                    mq_trait::impl_iep_fn,
                    &parse_quote!(crate::exits::MQIEP),
                    &item_trait,
                );
                iep_impls.push(item_impl);
            }

            if impls.contains(&TraitImpl::Link) {
                let item_impl = generate_impl_trait(
                    trait_name,
                    path,
                    feat_cfg.as_ref(),
                    mq_trait::impl_link_fn(path),
                    &link_name,
                    &item_trait,
                );
                link_impls.push(item_impl);
            }

            mock_impls.push(mock_impl_trait);
            traits.push(item_trait);
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
        #(
            #traits
        )*
    );

    let mock_impl_content = prettyplease::unparse(&parse_quote!(
        #(
            #mock_impls
        )*
    ));

    let mock_file_content = formatdoc! {"
        mockall::mock! {{
            pub {} {{}}
        {}
        }}
    ",
        mock_name.to_token_stream().to_string().replace(" ::", "::"),
        mock_impl_content.split_inclusive('\n').fold(String::new(), |mut impls, line| {
            impls += "    ";
            impls += line;
            impls
        })
    };

    let link_file = parse_quote!(
        /// Provides access to compile time linked MQI and MQAI functions
        #[derive(Debug, Clone, Copy)]
        pub struct #link_name;

        #(
            #link_impls
        )*

    );

    let wrapper_file = parse_quote!(
        use ::dlopen2::wrapper::WrapperApi;

        #[derive(WrapperApi, Debug)]
        pub struct #wrapper_name {
            #(#wrapper_fields,)*
        }

        #(
            #dlopen_impls
        )*

    );

    let iep_file = parse_quote!(
        #(
            #iep_impls
        )*
    );

    let out_mock = out_path.join("mock.rs");
    let mut out_file = io::BufWriter::new(std::fs::File::create(&out_mock)?);
    out_file.write_all(mock_file_content.as_bytes())?;
    drop(out_file);

    let out_function = out_path.join("function.rs");
    let out_link = out_path.join("link.rs");
    let out_wrapper = out_path.join("dlopen2.rs");
    let out_iep = out_path.join("iep.rs");
    write_file(&trait_file, &out_function)?;
    write_file(&link_file, &out_link)?;
    write_file(&wrapper_file, &out_wrapper)?;
    write_file(&iep_file, &out_iep)?;

    #[cfg(feature = "pregen")]
    {
        std::fs::copy(out_function, std::path::PathBuf::from("./src/pregen/function.rs"))?;
        std::fs::copy(out_mock, std::path::PathBuf::from("./src/pregen/mock.rs"))?;
        std::fs::copy(out_wrapper, std::path::PathBuf::from("./src/pregen/dlopen2.rs"))?;
        std::fs::copy(out_link, std::path::PathBuf::from("./src/pregen/link.rs"))?;
        std::fs::copy(out_iep, std::path::PathBuf::from("./src/pregen/iep.rs"))?;
    }

    Ok(())
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
                "MQC version {mqc_version} does not meet the minimum requirement for feature {feature}",
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
        generate_bindings(&out_path, &mqc_version)?;
    }

    Ok(())
}

#[cfg(feature = "pregen")]
fn pregen_copy_dir(out_bindings: &std::path::PathBuf, target: &std::path::Path) -> Result<(), io::Error> {
    use std::fs;
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let file_name = out_bindings.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("no file name in {}", out_bindings.display()),
        )
    })?;
    fs::copy(
        out_bindings,
        target.join(format!(
            "{}-{}/{}",
            if target_os == "macos" { "any" } else { &target_arch },
            target_os,
            file_name.to_string_lossy()
        )),
    )?;
    Ok(())
}
