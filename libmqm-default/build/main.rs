use std::{
    fs::File,
    io::{self, Write},
    slice,
};

use libmqm_sys::lib;

fn const_default<T>(write: &mut impl Write, const_name: &str, type_name: &str, value: &T) -> Result<(), io::Error> {
    let s = slice::from_ref(value);

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

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), io::Error> {
    // Generate and write the serialised defaults
    let out_path =
        std::path::PathBuf::from(std::env::var("OUT_DIR").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?); // Mandatory OUT_DIR

    let defaults_path = out_path.join("defaults.rs");

    let mut w = io::BufWriter::new(File::create(&defaults_path)?);

    const_default(&mut w, "MQMD_DEFAULT", "libmqm_sys::lib::MQMD", &lib::MQMD::default())?;
    const_default(&mut w, "MQMDE_DEFAULT", "libmqm_sys::lib::MQMDE", &lib::MQMDE::default())?;
    const_default(&mut w, "MQMD1_DEFAULT", "libmqm_sys::lib::MQMD1", &lib::MQMD1::default())?;
    const_default(&mut w, "MQMD2_DEFAULT", "libmqm_sys::lib::MQMD2", &lib::MQMD2::default())?;
    const_default(&mut w, "MQPD_DEFAULT", "libmqm_sys::lib::MQPD", &lib::MQPD::default())?;
    const_default(&mut w, "MQIMPO_DEFAULT", "libmqm_sys::lib::MQIMPO", &lib::MQIMPO::default())?;
    const_default(&mut w, "MQMHBO_DEFAULT", "libmqm_sys::lib::MQMHBO", &lib::MQMHBO::default())?;
    const_default(&mut w, "MQBO_DEFAULT", "libmqm_sys::lib::MQBO", &lib::MQBO::default())?;
    const_default(&mut w, "MQDMHO_DEFAULT", "libmqm_sys::lib::MQDMHO", &lib::MQDMHO::default())?;
    const_default(&mut w, "MQCMHO_DEFAULT", "libmqm_sys::lib::MQCMHO", &lib::MQCMHO::default())?;
    const_default(&mut w, "MQSRO_DEFAULT", "libmqm_sys::lib::MQSRO", &lib::MQSRO::default())?;
    const_default(&mut w, "MQSD_DEFAULT", "libmqm_sys::lib::MQSD", &lib::MQSD::default())?;
    const_default(&mut w, "MQGMO_DEFAULT", "libmqm_sys::lib::MQGMO", &lib::MQGMO::default())?;
    const_default(&mut w, "MQPMO_DEFAULT", "libmqm_sys::lib::MQPMO", &lib::MQPMO::default())?;
    const_default(&mut w, "MQOD_DEFAULT", "libmqm_sys::lib::MQOD", &lib::MQOD::default())?;
    const_default(&mut w, "MQCNO_DEFAULT", "libmqm_sys::lib::MQCNO", &lib::MQCNO::default())?;
    const_default(&mut w, "MQCD_DEFAULT", "libmqm_sys::lib::MQCD", &lib::MQCD::default())?;
    const_default(
        &mut w,
        "MQCD_CLIENT_CONN_DEFAULT",
        "libmqm_sys::lib::MQCD",
        &lib::MQCD::client_conn_default(),
    )?;
    const_default(&mut w, "MQCSP_DEFAULT", "libmqm_sys::lib::MQCSP", &lib::MQCSP::default())?;
    const_default(&mut w, "MQSCO_DEFAULT", "libmqm_sys::lib::MQSCO", &lib::MQSCO::default())?;
    #[cfg(feature = "mqc_9_3_0_0")]
    const_default(&mut w, "MQBNO_DEFAULT", "libmqm_sys::lib::MQBNO", &lib::MQBNO::default())?;
    const_default(&mut w, "MQAIR_DEFAULT", "libmqm_sys::lib::MQAIR", &lib::MQAIR::default())?;
    const_default(&mut w, "MQBMHO_DEFAULT", "libmqm_sys::lib::MQBMHO", &lib::MQBMHO::default())?;
    const_default(&mut w, "MQCBD_DEFAULT", "libmqm_sys::lib::MQCBD", &lib::MQCBD::default())?;
    const_default(
        &mut w,
        "MQCHARV_DEFAULT",
        "libmqm_sys::lib::MQCHARV",
        &lib::MQCHARV::default(),
    )?;
    const_default(&mut w, "MQCIH_DEFAULT", "libmqm_sys::lib::MQCIH", &lib::MQCIH::default())?;
    const_default(&mut w, "MQCTLO_DEFAULT", "libmqm_sys::lib::MQCTLO", &lib::MQCTLO::default())?;
    const_default(&mut w, "MQDH_DEFAULT", "libmqm_sys::lib::MQDH", &lib::MQDH::default())?;
    const_default(&mut w, "MQDLH_DEFAULT", "libmqm_sys::lib::MQDLH", &lib::MQDLH::default())?;
    const_default(&mut w, "MQDMPO_DEFAULT", "libmqm_sys::lib::MQDMPO", &lib::MQDMPO::default())?;
    const_default(&mut w, "MQIIH_DEFAULT", "libmqm_sys::lib::MQIIH", &lib::MQIIH::default())?;
    const_default(&mut w, "MQOR_DEFAULT", "libmqm_sys::lib::MQOR", &lib::MQOR::default())?;
    const_default(&mut w, "MQRFH_DEFAULT", "libmqm_sys::lib::MQRFH", &lib::MQRFH::default())?;
    const_default(&mut w, "MQRFH2_DEFAULT", "libmqm_sys::lib::MQRFH2", &lib::MQRFH2::default())?;
    const_default(&mut w, "MQRMH_DEFAULT", "libmqm_sys::lib::MQRMH", &lib::MQRMH::default())?;
    const_default(&mut w, "MQRR_DEFAULT", "libmqm_sys::lib::MQRR", &lib::MQRR::default())?;
    const_default(&mut w, "MQSMPO_DEFAULT", "libmqm_sys::lib::MQSMPO", &lib::MQSMPO::default())?;
    const_default(&mut w, "MQSTS_DEFAULT", "libmqm_sys::lib::MQSTS", &lib::MQSTS::default())?;
    const_default(&mut w, "MQTM_DEFAULT", "libmqm_sys::lib::MQTM", &lib::MQTM::default())?;
    const_default(&mut w, "MQTMC2_DEFAULT", "libmqm_sys::lib::MQTMC2", &lib::MQTMC2::default())?;
    const_default(&mut w, "MQWIH_DEFAULT", "libmqm_sys::lib::MQWIH", &lib::MQWIH::default())?;
    const_default(&mut w, "MQXQH_DEFAULT", "libmqm_sys::lib::MQXQH", &lib::MQXQH::default())?;

    #[cfg(feature = "pcf")]
    {
        const_default(&mut w, "MQCFH_DEFAULT", "libmqm_sys::lib::MQCFH", &lib::MQCFH::default())?;
        const_default(&mut w, "MQCFBF_DEFAULT", "libmqm_sys::lib::MQCFBF", &lib::MQCFBF::default())?;
        const_default(&mut w, "MQCFBS_DEFAULT", "libmqm_sys::lib::MQCFBS", &lib::MQCFBS::default())?;
        const_default(&mut w, "MQCFGR_DEFAULT", "libmqm_sys::lib::MQCFGR", &lib::MQCFGR::default())?;
        const_default(&mut w, "MQCFIF_DEFAULT", "libmqm_sys::lib::MQCFIF", &lib::MQCFIF::default())?;
        const_default(&mut w, "MQCFIL_DEFAULT", "libmqm_sys::lib::MQCFIL", &lib::MQCFIL::default())?;
        const_default(
            &mut w,
            "MQCFIL64_DEFAULT",
            "libmqm_sys::lib::MQCFIL64",
            &lib::MQCFIL64::default(),
        )?;
        const_default(&mut w, "MQCFIN_DEFAULT", "libmqm_sys::lib::MQCFIN", &lib::MQCFIN::default())?;
        const_default(
            &mut w,
            "MQCFIN64_DEFAULT",
            "libmqm_sys::lib::MQCFIN64",
            &lib::MQCFIN64::default(),
        )?;
        const_default(&mut w, "MQCFSF_DEFAULT", "libmqm_sys::lib::MQCFSF", &lib::MQCFSF::default())?;
        const_default(&mut w, "MQCFSL_DEFAULT", "libmqm_sys::lib::MQCFSL", &lib::MQCFSL::default())?;
        const_default(&mut w, "MQCFST_DEFAULT", "libmqm_sys::lib::MQCFST", &lib::MQCFST::default())?;
        const_default(&mut w, "MQEPH_DEFAULT", "libmqm_sys::lib::MQEPH", &lib::MQEPH::default())?;
        const_default(&mut w, "MQZED_DEFAULT", "libmqm_sys::lib::MQZED", &lib::MQZED::default())?;
        const_default(&mut w, "MQZAC_DEFAULT", "libmqm_sys::lib::MQZAC", &lib::MQZAC::default())?;
        const_default(&mut w, "MQZAD_DEFAULT", "libmqm_sys::lib::MQZAD", &lib::MQZAD::default())?;
        const_default(&mut w, "MQZFP_DEFAULT", "libmqm_sys::lib::MQZFP", &lib::MQZFP::default())?;
        const_default(&mut w, "MQZIC_DEFAULT", "libmqm_sys::lib::MQZIC", &lib::MQZIC::default())?;
    }

    #[cfg(feature = "exits")]
    {
        const_default(&mut w, "MQACH_DEFAULT", "libmqm_sys::lib::MQACH", &lib::MQACH::default())?;
        const_default(&mut w, "MQAXC_DEFAULT", "libmqm_sys::lib::MQAXC", &lib::MQAXC::default())?;
        const_default(&mut w, "MQAXP_DEFAULT", "libmqm_sys::lib::MQAXP", &lib::MQAXP::default())?;
        const_default(&mut w, "MQCXP_DEFAULT", "libmqm_sys::lib::MQCXP", &lib::MQCXP::default())?;
        const_default(&mut w, "MQDXP_DEFAULT", "libmqm_sys::lib::MQDXP", &lib::MQDXP::default())?;
        const_default(&mut w, "MQNXP_DEFAULT", "libmqm_sys::lib::MQNXP", &lib::MQNXP::default())?;
        const_default(&mut w, "MQPBC_DEFAULT", "libmqm_sys::lib::MQPBC", &lib::MQPBC::default())?;
        const_default(&mut w, "MQPSXP_DEFAULT", "libmqm_sys::lib::MQPSXP", &lib::MQPSXP::default())?;
        const_default(&mut w, "MQSBC_DEFAULT", "libmqm_sys::lib::MQSBC", &lib::MQSBC::default())?;
        const_default(&mut w, "MQWCR_DEFAULT", "libmqm_sys::lib::MQWCR", &lib::MQWCR::default())?;
        const_default(&mut w, "MQWDR_DEFAULT", "libmqm_sys::lib::MQWDR", &lib::MQWDR::default())?;
        const_default(&mut w, "MQWDR1_DEFAULT", "libmqm_sys::lib::MQWDR1", &lib::MQWDR1::default())?;
        const_default(&mut w, "MQWDR2_DEFAULT", "libmqm_sys::lib::MQWDR2", &lib::MQWDR2::default())?;
        const_default(&mut w, "MQWQR_DEFAULT", "libmqm_sys::lib::MQWQR", &lib::MQWQR::default())?;
        const_default(&mut w, "MQWQR1_DEFAULT", "libmqm_sys::lib::MQWQR1", &lib::MQWQR1::default())?;
        const_default(&mut w, "MQWQR2_DEFAULT", "libmqm_sys::lib::MQWQR2", &lib::MQWQR2::default())?;
        const_default(&mut w, "MQWQR3_DEFAULT", "libmqm_sys::lib::MQWQR3", &lib::MQWQR3::default())?;
        #[cfg(feature = "mqc_9_3_1_0")]
        const_default(&mut w, "MQWQR4_DEFAULT", "libmqm_sys::lib::MQWQR4", &lib::MQWQR4::default())?;
        const_default(&mut w, "MQWXP_DEFAULT", "libmqm_sys::lib::MQWXP", &lib::MQWXP::default())?;
        const_default(&mut w, "MQWXP1_DEFAULT", "libmqm_sys::lib::MQWXP1", &lib::MQWXP1::default())?;
        const_default(&mut w, "MQWXP2_DEFAULT", "libmqm_sys::lib::MQWXP2", &lib::MQWXP2::default())?;
        const_default(&mut w, "MQWXP3_DEFAULT", "libmqm_sys::lib::MQWXP3", &lib::MQWXP3::default())?;
        const_default(&mut w, "MQWXP4_DEFAULT", "libmqm_sys::lib::MQWXP4", &lib::MQWXP4::default())?;
        const_default(&mut w, "MQXEPO_DEFAULT", "libmqm_sys::lib::MQXEPO", &lib::MQXEPO::default())?;
        const_default(&mut w, "MQIEP_DEFAULT", "libmqm_sys::lib::MQIEP", &lib::MQIEP::default())?;
    }

    Ok(())
}
