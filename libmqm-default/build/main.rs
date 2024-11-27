use std::{
    fs::File,
    io::{self, Write},
    slice,
};

use libmqm_sys::lib;

fn const_default<T: Default>(write: &mut impl Write, const_name: &str, type_name: &str) -> Result<(), io::Error> {
    let dflt = T::default();
    let s = slice::from_ref(&dflt);

    let (prefix, bytes, suffix) = unsafe { s.align_to::<u8>() };
    assert_eq!(prefix.len(), 0);
    assert_eq!(suffix.len(), 0);
    write!(
        write,
        "pub const {}: {} = unsafe {{ std::mem::transmute::<[u8; {}], _>([",
        const_name,
        type_name,
        bytes.len()
    )?;
    for s_byte in bytes {
        if s_byte.is_ascii_alphanumeric() || *s_byte == 0x20 {
            write!(write, "b'{}',", *s_byte as char)?;
        } else {
            write!(write, "{s_byte:#02x},")?;
        }
    }

    writeln!(write, "]) }};")?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    // Generate and write the serialised defaults
    let out_path =
        std::path::PathBuf::from(std::env::var("OUT_DIR").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?); // Mandatory OUT_DIR

    let defaults_path = out_path.join("defaults.rs");

    let mut w = io::BufWriter::new(File::create(&defaults_path)?);

    const_default::<lib::MQCNO>(&mut w, "MQCNO_DEFAULT", "libmqm_sys::lib::MQCNO")?;
    const_default::<lib::MQMD>(&mut w, "MQMD_DEFAULT", "libmqm_sys::lib::MQMD")?;
    const_default::<lib::MQMD2>(&mut w, "MQMD2_DEFAULT", "libmqm_sys::lib::MQMD2")?;
    const_default::<lib::MQGMO>(&mut w, "MQGMO_DEFAULT", "libmqm_sys::lib::MQGMO")?;

    Ok(())
}
