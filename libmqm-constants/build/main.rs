#[cfg(feature = "generate")]
mod constants {
    pub mod generate;
    mod list;
}

#[cfg(feature = "generate")]
fn feature_mod<W: std::io::Write, F: FnOnce(&mut W) -> Result<(), std::io::Error>>(
    feature_name: Option<&str>,
    w: W,
    f: F,
) -> Result<(), std::io::Error> {
    let mut w = w;
    if let Some(feature) = feature_name {
        writeln!(
            &mut w,
            "
            #[cfg(feature = \"{feature}\")]
            mod {feature} {{
        "
        )?;
        f(&mut w)?;
        writeln!(
            &mut w,
            "
            }}
            #[cfg(feature = \"{feature}\")]
            pub use {feature}::*;
        "
        )?;
    } else {
        f(&mut w)?;
    }
    Ok(())
}

#[allow(clippy::unnecessary_wraps, clippy::too_many_lines)] // reason = "when no features are enabled"
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "generate")]
    {
        use constants::generate;
        use libmqm_sys::str as mqstr;
        use std::collections::HashSet;
        use std::io;
        use std::io::Write as _;

        let by_name_mqi = unsafe { &mqstr::MQI_BY_NAME_STR };

        let mut unmapped_comment = Vec::new();
        let mut mapping_write: Vec<u8> = Vec::new();
        let mut new_type_mods_write: Vec<u8> = Vec::new();
        // let mut test_cases: Vec<u8> = Vec::new();
        generate::generate_constants(|prefix_constants| {
            let by_feature = prefix_constants.iter().fold(
                std::collections::BTreeMap::<_, Vec<_>>::new(),
                |mut hash, all @ (_, (feature, ..))| {
                    let list = hash.entry(*feature).or_default();
                    list.push(all);
                    hash
                },
            );

            let unassigned_filter: Vec<_> = generate::const_ignore_regex().collect();
            let const_set: HashSet<_> = prefix_constants
                .iter()
                .flat_map(|(.., (.., primary, extra))| primary.iter().chain(extra).map(|(_, constant, _)| *constant))
                .collect();

            let const_all: HashSet<_> = generate::by_name(by_name_mqi)
                .filter_map(|(name, ..)| unassigned_filter.iter().all(|r| !r.is_match(name)).then_some(name))
                .collect();

            let mut unmapped: Vec<_> = const_all.difference(&const_set).copied().collect();
            unmapped.sort_unstable();

            let mut new_type_write: Vec<u8> = Vec::new();
            let mut constant_write: Vec<u8> = Vec::new();

            // Generate the unmapped constants list
            if !unmapped.is_empty() {
                writeln!(unmapped_comment, "/*\n * Unmapped constants:")?;
                for constant in unmapped {
                    writeln!(unmapped_comment, " *  {constant}")?;
                }
                writeln!(unmapped_comment, " */")?;
            }

            for (feature, prefix_constants) in &by_feature {
                feature_mod(*feature, &mut new_type_write, |w| {
                    writeln!(
                        w,
                        "
                            use ::libmqm_sys::lib as mqsys;
                            use crate::mapping;
                            use crate::value::{{define_new_type, impl_value}};
                            use crate::bitflags::impl_bitflags;
                    "
                    )?;

                    for (prefix, (_, new_type, orig_type, usage, doc, ..)) in prefix_constants {
                        writeln!(
                            w,
                            "
                                define_new_type!(pub {new_type}, mqsys::{orig_type}, mapping::{prefix}MAPSTR{});
                                impl_{usage}!({new_type}, mqsys::{orig_type});
                            ",
                            doc.map_or(String::new(), |doc_lines| format!(", r##\"{doc_lines}\"##"))
                        )?;
                    }
                    Ok(())
                })?;
            }

            for (feature, prefix_constants) in &by_feature {
                feature_mod(*feature, &mut constant_write, |w| {
                    writeln!(w, "use crate::types;")?;
                    for (_, (_, new_type, _, _, _, primary, extra)) in prefix_constants {
                        for (value, constant, doc) in primary.iter().chain(extra) {
                            if let Some(doc_lines) = doc {
                                writeln!(w, "#[doc = r##\"{doc_lines}\"##]",)?;
                            }
                            writeln!(w, "pub const {constant}: types::{new_type} = types::{new_type}({value});")?;
                            // writeln!(
                            //     test_cases,
                            //     "assert_eq!(constants::{constant}, types::{new_type}(sys::{constant}));"
                            // )?;
                        }
                    }
                    Ok(())
                })?;
            }

            write!(
                new_type_mods_write,
                "
                pub mod types {{
                    {}
                }}
            ",
                String::from_utf8_lossy(&new_type_write)
            )?;
            write!(
                new_type_mods_write,
                "
                pub mod constants {{
                    {}
                }}
            ",
                String::from_utf8_lossy(&constant_write)
            )?;

            writeln!(mapping_write, "use crate::lookup::*;")?;
            // Pick a lookup type based on the size of the constants for a prefix
            // TODO: Determine best ranges for performance
            for (prefix, (.., primary, extra)) in prefix_constants {
                let extra_array = generate::as_array(extra);
                write!(mapping_write, "pub const {prefix}MAPSTR: ")?;
                match primary.len() {
                    0..=63 => {
                        // Linear search array
                        writeln!(
                            mapping_write,
                            "LinearSource = ConstSource(&{}, &{extra_array});",
                            generate::as_array(primary),
                        )?;
                    }
                    64..=255 => {
                        // Binary search array
                        writeln!(
                            mapping_write,
                            "BinarySearchSource = ConstSource(BinarySearch(&{}), &{extra_array});",
                            generate::as_array(primary),
                        )?;
                    }
                    _ => {
                        // Perfect hash used for larger constant lists
                        writeln!(
                            mapping_write,
                            "PhfSource = ConstSource(&{}, &{extra_array});",
                            generate::as_phf(primary),
                        )?;
                    }
                }
            }
            Ok::<(), io::Error>(())
        })?;

        // Full MQI_BY_STRING
        let mut mqi_by_string = phf_codegen::Map::<&str>::new();
        for (name, value) in generate::by_name(by_name_mqi) {
            mqi_by_string.entry(name, &value.to_string());
        }
        writeln!(
            mapping_write,
            "pub const MQI_BY_STRING: ::phf::Map<&'static str, ::libmqm_sys::lib::MQLONG> = {};",
            mqi_by_string.build()
        )?;

        let outdir = std::env::var("OUT_DIR").expect("OUT_DIR is mandatory for builds");
        let outdir_path = std::path::Path::new(&outdir);
        let version_stamp = format!(
            "/* Generated with MQ client version {} */",
            libmqm_sys::version::CLIENT_BUILD_VERSION
        );
        for (filename, prelude, buffer) in [
            ("mapping.rs", [&*version_stamp].as_slice(), mapping_write),
            (
                "new_types.rs",
                [&version_stamp, &*String::from_utf8_lossy(&unmapped_comment)].as_slice(),
                new_type_mods_write,
            ),
        ] {
            let gen_str = String::from_utf8(buffer)?;
            let gen_syn = syn::parse_file(&gen_str)?;
            let gen_pretty = prettyplease::unparse(&gen_syn);
            let gen_path = outdir_path.join(filename);
            {
                let gen_file = std::fs::File::create(&gen_path)?;
                let mut gen_pretty_write = std::io::BufWriter::new(gen_file);
                for prelude_item in prelude {
                    writeln!(&mut gen_pretty_write, "{prelude_item}")?;
                }
                gen_pretty_write.write_all(gen_pretty.as_bytes())?;
            }
            #[cfg(feature = "pregen")]
            {
                use std::{fs, path};

                let target_os =
                    std::env::var("CARGO_CFG_TARGET_OS").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?; // Mandatory
                let target_arch =
                    std::env::var("CARGO_CFG_TARGET_ARCH").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?; // Mandatory

                fs::copy(
                    &gen_path,
                    path::PathBuf::from("./src/pregen").join(format!(
                        "{}-{}-{}",
                        if target_os == "macos" { "any" } else { &target_arch },
                        target_os,
                        filename
                    )),
                )?;
            }
        }
    }

    Ok(())
}
