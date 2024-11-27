use std::{fs::File, io::{self, Write}, mem::transmute};

use libmqm_sys::lib;

fn main() -> Result<(), io::Error> {
    // Generate and write the serialised defaults
    let out_path =
        std::path::PathBuf::from(std::env::var("OUT_DIR").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?); // Mandatory OUT_DIR

    let defaults_path = out_path.join("defaults.rs");

    let mqmd = lib::MQMD::default();
    let mqmd_bytes = unsafe { transmute::<_, &[u8; lib::MQMD_CURRENT_LENGTH]>(&mqmd) };
    
    let mut defaults_writer = io::BufWriter::new(File::create(&defaults_path)?);

    for s_byte in mqmd_bytes {
        write!(defaults_writer, "{}", s_byte)?;
    }

    Ok(())
}
