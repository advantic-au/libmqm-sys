use std::{env, io};

use constants::generate;
use libmqm_sys::lib as mqsys;

#[cfg(feature = "generate")]
mod constants {
    pub mod generate;
    mod list;
}

#[allow(clippy::unnecessary_wraps)] // reason = "when no features are enabled"
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[allow(unused_variables)] // reason = "when constantgen and pregen are not enabled"
    let path = std::path::Path::new(&env::var("OUT_DIR").expect("OUT_DIR is mandatory for builds")).join("mqconstants.rs");
    #[cfg(feature = "generate")]
    {
        use std::io::Write as _;
        let mut constants_write = std::io::BufWriter::new(Vec::new());
        constants::generate::generate_constants(|prefix_constants| {
            // Pick a lookup type based on the size of the constants for a prefix
            // TODO: Determine best ranges for performance
            for (prefix, (primary, ref extra)) in prefix_constants {
                let extra_array = generate::as_array(extra);
                write!(constants_write, "pub const {prefix}CONST: ")?;
                match primary.len() {
                    0..=63 => {
                        // Linear search array
                        writeln!(
                            constants_write,
                            "LinearSource = ConstSource(&{}, &{extra_array});",
                            generate::as_array(primary),
                        )?;
                    }
                    64..=255 => {
                        // Binary search array
                        writeln!(
                            constants_write,
                            "BinarySearchSource = ConstSource(BinarySearch(&{}), &{extra_array});",
                            generate::as_array(primary),
                        )?;
                    }
                    _ => {
                        // Perfect hash used for larger constant lists
                        writeln!(
                            constants_write,
                            "PhfSource = ConstSource(&{}, &{extra_array});",
                            generate::as_phf(primary),
                        )?;
                    }
                }
            }
            Ok::<(), io::Error>(())
        })?;

        let by_name_mqi = unsafe { &mqsys::MQI_BY_NAME_STR };
        let by_name = generate::by_name(by_name_mqi);    

        // Full MQI_BY_STRING
        let mut mqi_by_string = phf_codegen::Map::<&str>::new();
        for (name, value) in by_name {
            mqi_by_string.entry(name, &value.to_string());
        }
        writeln!(
            constants_write,
            "pub(crate) const MQI_BY_STRING: ::phf::Map<&'static str, ::libmqm_sys::lib::MQLONG> = {};",
            mqi_by_string.build()
        )?;

        let constants_str = String::from_utf8(constants_write.into_inner()?)?;
        let constants_syn = syn::parse_file(&constants_str)?;
        let constants_pretty = prettyplease::unparse(&constants_syn);
        let constants_file = std::fs::File::create(&path)?;
        let mut constants_pretty_write = std::io::BufWriter::new(constants_file);

        writeln!(
            &mut constants_pretty_write,
            "/* Generated with MQ client version {} */",
            libmqm_sys::version::CLIENT_BUILD_VERSION
        )?;
        constants_pretty_write.write_all(constants_pretty.as_bytes())?;
    }

    #[cfg(feature = "pregen")]
    {
        use std::{env::consts as env_consts, fs, path};

        fs::copy(
            &path,
            path::PathBuf::from("./src/mapping/pregen").join(format!(
                "{}-{}-mqconstants.rs",
                if env_consts::OS == "macos" { "any" } else { env_consts::ARCH },
                env_consts::OS
            )),
        )?;
    }

    Ok(())
}
