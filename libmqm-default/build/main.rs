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

    const_default::<lib::MQMD>(&mut w, "MQMD_DEFAULT", "libmqm_sys::lib::MQMD")?;
    const_default::<lib::MQMDE>(&mut w, "MQMDE_DEFAULT", "libmqm_sys::lib::MQMDE")?;
    const_default::<lib::MQMD1>(&mut w, "MQMD1_DEFAULT", "libmqm_sys::lib::MQMD1")?;
    const_default::<lib::MQMD2>(&mut w, "MQMD2_DEFAULT", "libmqm_sys::lib::MQMD2")?;
    const_default::<lib::MQPD>(&mut w, "MQPD_DEFAULT", "libmqm_sys::lib::MQPD")?;
    const_default::<lib::MQIMPO>(&mut w, "MQIMPO_DEFAULT", "libmqm_sys::lib::MQIMPO")?;
    const_default::<lib::MQMHBO>(&mut w, "MQMHBO_DEFAULT", "libmqm_sys::lib::MQMHBO")?;
    const_default::<lib::MQBO>(&mut w, "MQBO_DEFAULT", "libmqm_sys::lib::MQBO")?;
    const_default::<lib::MQDMHO>(&mut w, "MQDMHO_DEFAULT", "libmqm_sys::lib::MQDMHO")?;
    const_default::<lib::MQCMHO>(&mut w, "MQCMHO_DEFAULT", "libmqm_sys::lib::MQCMHO")?;
    const_default::<lib::MQSRO>(&mut w, "MQSRO_DEFAULT", "libmqm_sys::lib::MQSRO")?;
    const_default::<lib::MQSD>(&mut w, "MQSD_DEFAULT", "libmqm_sys::lib::MQSD")?;
    const_default::<lib::MQGMO>(&mut w, "MQGMO_DEFAULT", "libmqm_sys::lib::MQGMO")?;
    const_default::<lib::MQPMO>(&mut w, "MQPMO_DEFAULT", "libmqm_sys::lib::MQPMO")?;
    const_default::<lib::MQOD>(&mut w, "MQOD_DEFAULT", "libmqm_sys::lib::MQOD")?;
    const_default::<lib::MQCNO>(&mut w, "MQCNO_DEFAULT", "libmqm_sys::lib::MQCNO")?;
    const_default::<lib::MQCD>(&mut w, "MQCD_DEFAULT", "libmqm_sys::lib::MQCD")?;
    const_default::<lib::MQCSP>(&mut w, "MQCSP_DEFAULT", "libmqm_sys::lib::MQCSP")?;
    const_default::<lib::MQSCO>(&mut w, "MQSCO_DEFAULT", "libmqm_sys::lib::MQSCO")?;
    #[cfg(feature = "mqc_9_3_0_0")]
    const_default::<lib::MQBNO>(&mut w, "MQBNO_DEFAULT", "libmqm_sys::lib::MQBNO")?;
    const_default::<lib::MQAIR>(&mut w, "MQAIR_DEFAULT", "libmqm_sys::lib::MQAIR")?;
    const_default::<lib::MQBMHO>(&mut w, "MQBMHO_DEFAULT", "libmqm_sys::lib::MQBMHO")?;
    const_default::<lib::MQCBD>(&mut w, "MQCBD_DEFAULT", "libmqm_sys::lib::MQCBD")?;
    const_default::<lib::MQCHARV>(&mut w, "MQCHARV_DEFAULT", "libmqm_sys::lib::MQCHARV")?;
    const_default::<lib::MQCIH>(&mut w, "MQCIH_DEFAULT", "libmqm_sys::lib::MQCIH")?;
    const_default::<lib::MQCTLO>(&mut w, "MQCTLO_DEFAULT", "libmqm_sys::lib::MQCTLO")?;
    const_default::<lib::MQDH>(&mut w, "MQDH_DEFAULT", "libmqm_sys::lib::MQDH")?;
    const_default::<lib::MQDLH>(&mut w, "MQDLH_DEFAULT", "libmqm_sys::lib::MQDLH")?;
    const_default::<lib::MQDMPO>(&mut w, "MQDMPO_DEFAULT", "libmqm_sys::lib::MQDMPO")?;
    const_default::<lib::MQIIH>(&mut w, "MQIIH_DEFAULT", "libmqm_sys::lib::MQIIH")?;
    const_default::<lib::MQOR>(&mut w, "MQOR_DEFAULT", "libmqm_sys::lib::MQOR")?;
    const_default::<lib::MQRFH>(&mut w, "MQRFH_DEFAULT", "libmqm_sys::lib::MQRFH")?;
    const_default::<lib::MQRFH2>(&mut w, "MQRFH2_DEFAULT", "libmqm_sys::lib::MQRFH2")?;
    const_default::<lib::MQRMH>(&mut w, "MQRMH_DEFAULT", "libmqm_sys::lib::MQRMH")?;
    const_default::<lib::MQRR>(&mut w, "MQRR_DEFAULT", "libmqm_sys::lib::MQRR")?;
    const_default::<lib::MQSMPO>(&mut w, "MQSMPO_DEFAULT", "libmqm_sys::lib::MQSMPO")?;
    const_default::<lib::MQSTS>(&mut w, "MQSTS_DEFAULT", "libmqm_sys::lib::MQSTS")?;
    const_default::<lib::MQTM>(&mut w, "MQTM_DEFAULT", "libmqm_sys::lib::MQTM")?;
    const_default::<lib::MQTMC2>(&mut w, "MQTMC2_DEFAULT", "libmqm_sys::lib::MQTMC2")?;
    const_default::<lib::MQWIH>(&mut w, "MQWIH_DEFAULT", "libmqm_sys::lib::MQWIH")?;
    const_default::<lib::MQXQH>(&mut w, "MQXQH_DEFAULT", "libmqm_sys::lib::MQXQH")?;

    // PCF
    #[cfg(feature = "pcf")]
    {
        const_default::<lib::MQCFH>(&mut w, "MQCFH_DEFAULT", "libmqm_sys::lib::MQCFH")?;
        const_default::<lib::MQCFBF>(&mut w, "MQCFBF_DEFAULT", "libmqm_sys::lib::MQCFBF")?;
        const_default::<lib::MQCFBS>(&mut w, "MQCFBS_DEFAULT", "libmqm_sys::lib::MQCFBS")?;
        const_default::<lib::MQCFGR>(&mut w, "MQCFGR_DEFAULT", "libmqm_sys::lib::MQCFGR")?;
        const_default::<lib::MQCFIF>(&mut w, "MQCFIF_DEFAULT", "libmqm_sys::lib::MQCFIF")?;
        const_default::<lib::MQCFIL>(&mut w, "MQCFIL_DEFAULT", "libmqm_sys::lib::MQCFIL")?;
        const_default::<lib::MQCFIL64>(&mut w, "MQCFIL64_DEFAULT", "libmqm_sys::lib::MQCFIL64")?;
        const_default::<lib::MQCFIN>(&mut w, "MQCFIN_DEFAULT", "libmqm_sys::lib::MQCFIN")?;
        const_default::<lib::MQCFIN64>(&mut w, "MQCFIN64_DEFAULT", "libmqm_sys::lib::MQCFIN64")?;
        const_default::<lib::MQCFSF>(&mut w, "MQCFSF_DEFAULT", "libmqm_sys::lib::MQCFSF")?;
        const_default::<lib::MQCFSL>(&mut w, "MQCFSL_DEFAULT", "libmqm_sys::lib::MQCFSL")?;
        const_default::<lib::MQCFST>(&mut w, "MQCFST_DEFAULT", "libmqm_sys::lib::MQCFST")?;
        const_default::<lib::MQEPH>(&mut w, "MQEPH_DEFAULT", "libmqm_sys::lib::MQEPH")?;
        const_default::<lib::MQZED>(&mut w, "MQZED_DEFAULT", "libmqm_sys::lib::MQZED")?;
        const_default::<lib::MQZAC>(&mut w, "MQZAC_DEFAULT", "libmqm_sys::lib::MQZAC")?;
        const_default::<lib::MQZAD>(&mut w, "MQZAD_DEFAULT", "libmqm_sys::lib::MQZAD")?;
        const_default::<lib::MQZFP>(&mut w, "MQZFP_DEFAULT", "libmqm_sys::lib::MQZFP")?;
        const_default::<lib::MQZIC>(&mut w, "MQZIC_DEFAULT", "libmqm_sys::lib::MQZIC")?;
    }

    #[cfg(feature = "exits")]
    {
        const_default::<lib::MQACH>(&mut w, "MQACH_DEFAULT", "libmqm_sys::lib::MQACH")?;
        const_default::<lib::MQAXC>(&mut w, "MQAXC_DEFAULT", "libmqm_sys::lib::MQAXC")?;
        const_default::<lib::MQAXP>(&mut w, "MQAXP_DEFAULT", "libmqm_sys::lib::MQAXP")?;
        const_default::<lib::MQCXP>(&mut w, "MQCXP_DEFAULT", "libmqm_sys::lib::MQCXP")?;
        const_default::<lib::MQDXP>(&mut w, "MQDXP_DEFAULT", "libmqm_sys::lib::MQDXP")?;
        const_default::<lib::MQNXP>(&mut w, "MQNXP_DEFAULT", "libmqm_sys::lib::MQNXP")?;
        const_default::<lib::MQPBC>(&mut w, "MQPBC_DEFAULT", "libmqm_sys::lib::MQPBC")?;
        const_default::<lib::MQPSXP>(&mut w, "MQPSXP_DEFAULT", "libmqm_sys::lib::MQPSXP")?;
        const_default::<lib::MQSBC>(&mut w, "MQSBC_DEFAULT", "libmqm_sys::lib::MQSBC")?;
        const_default::<lib::MQWCR>(&mut w, "MQWCR_DEFAULT", "libmqm_sys::lib::MQWCR")?;
        const_default::<lib::MQWDR>(&mut w, "MQWDR_DEFAULT", "libmqm_sys::lib::MQWDR")?;
        const_default::<lib::MQWDR1>(&mut w, "MQWDR1_DEFAULT", "libmqm_sys::lib::MQWDR1")?;
        const_default::<lib::MQWDR2>(&mut w, "MQWDR2_DEFAULT", "libmqm_sys::lib::MQWDR2")?;
        const_default::<lib::MQWQR>(&mut w, "MQWQR_DEFAULT", "libmqm_sys::lib::MQWQR")?;
        const_default::<lib::MQWQR1>(&mut w, "MQWQR1_DEFAULT", "libmqm_sys::lib::MQWQR1")?;
        const_default::<lib::MQWQR2>(&mut w, "MQWQR2_DEFAULT", "libmqm_sys::lib::MQWQR2")?;
        const_default::<lib::MQWQR3>(&mut w, "MQWQR3_DEFAULT", "libmqm_sys::lib::MQWQR3")?;
        #[cfg(feature = "mqc_9_3_1_0")]
        const_default::<lib::MQWQR4>(&mut w, "MQWQR4_DEFAULT", "libmqm_sys::lib::MQWQR4")?;
        const_default::<lib::MQWXP>(&mut w, "MQWXP_DEFAULT", "libmqm_sys::lib::MQWXP")?;
        const_default::<lib::MQWXP1>(&mut w, "MQWXP1_DEFAULT", "libmqm_sys::lib::MQWXP1")?;
        const_default::<lib::MQWXP2>(&mut w, "MQWXP2_DEFAULT", "libmqm_sys::lib::MQWXP2")?;
        const_default::<lib::MQWXP3>(&mut w, "MQWXP3_DEFAULT", "libmqm_sys::lib::MQWXP3")?;
        const_default::<lib::MQWXP4>(&mut w, "MQWXP4_DEFAULT", "libmqm_sys::lib::MQWXP4")?;
        const_default::<lib::MQXEPO>(&mut w, "MQXEPO_DEFAULT", "libmqm_sys::lib::MQXEPO")?;
        const_default::<lib::MQIEP>(&mut w, "MQIEP_DEFAULT", "libmqm_sys::lib::MQIEP")?;
    }

    Ok(())
}
