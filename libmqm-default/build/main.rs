#[allow(clippy::missing_errors_doc, clippy::too_many_lines)]
#[cfg(feature = "generate")]
pub fn generate_defaults(w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
    use libmqm_sys as mq;

    fn const_default<T>(
        write: &mut impl std::io::Write,
        const_name: &str,
        type_name: &str,
        value: &T,
    ) -> Result<(), std::io::Error> {
        let s = std::slice::from_ref(value);
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

    const_default(w, "MQMD_DEFAULT", "libmqm_sys::MQMD", &mq::MQMD::default())?;
    const_default(w, "MQMDE_DEFAULT", "libmqm_sys::MQMDE", &mq::MQMDE::default())?;
    const_default(w, "MQMD1_DEFAULT", "libmqm_sys::MQMD1", &mq::MQMD1::default())?;
    const_default(w, "MQMD2_DEFAULT", "libmqm_sys::MQMD2", &mq::MQMD2::default())?;
    const_default(w, "MQPD_DEFAULT", "libmqm_sys::MQPD", &mq::MQPD::default())?;
    const_default(w, "MQIMPO_DEFAULT", "libmqm_sys::MQIMPO", &mq::MQIMPO::default())?;
    const_default(w, "MQMHBO_DEFAULT", "libmqm_sys::MQMHBO", &mq::MQMHBO::default())?;
    const_default(w, "MQBO_DEFAULT", "libmqm_sys::MQBO", &mq::MQBO::default())?;
    const_default(w, "MQDMHO_DEFAULT", "libmqm_sys::MQDMHO", &mq::MQDMHO::default())?;
    const_default(w, "MQCMHO_DEFAULT", "libmqm_sys::MQCMHO", &mq::MQCMHO::default())?;
    const_default(w, "MQSRO_DEFAULT", "libmqm_sys::MQSRO", &mq::MQSRO::default())?;
    const_default(w, "MQSD_DEFAULT", "libmqm_sys::MQSD", &mq::MQSD::default())?;
    const_default(w, "MQGMO_DEFAULT", "libmqm_sys::MQGMO", &mq::MQGMO::default())?;
    const_default(w, "MQPMO_DEFAULT", "libmqm_sys::MQPMO", &mq::MQPMO::default())?;
    const_default(w, "MQOD_DEFAULT", "libmqm_sys::MQOD", &mq::MQOD::default())?;
    const_default(w, "MQCNO_DEFAULT", "libmqm_sys::MQCNO", &mq::MQCNO::default())?;
    const_default(w, "MQCD_DEFAULT", "libmqm_sys::MQCD", &mq::MQCD::default())?;
    const_default(
        w,
        "MQCD_CLIENT_CONN_DEFAULT",
        "libmqm_sys::MQCD",
        &mq::MQCD::client_conn_default(),
    )?;
    const_default(w, "MQCSP_DEFAULT", "libmqm_sys::MQCSP", &mq::MQCSP::default())?;
    const_default(w, "MQSCO_DEFAULT", "libmqm_sys::MQSCO", &mq::MQSCO::default())?;
    #[cfg(feature = "mqc_9_3_0_0")]
    {
        writeln!(w, "#[cfg(feature = \"mqc_9_3_0_0\")]")?;
        const_default(w, "MQBNO_DEFAULT", "libmqm_sys::MQBNO", &mq::MQBNO::default())?;
    }
    const_default(w, "MQAIR_DEFAULT", "libmqm_sys::MQAIR", &mq::MQAIR::default())?;
    const_default(w, "MQBMHO_DEFAULT", "libmqm_sys::MQBMHO", &mq::MQBMHO::default())?;
    const_default(w, "MQCBD_DEFAULT", "libmqm_sys::MQCBD", &mq::MQCBD::default())?;
    const_default(w, "MQCHARV_DEFAULT", "libmqm_sys::MQCHARV", &mq::MQCHARV::default())?;
    const_default(w, "MQCIH_DEFAULT", "libmqm_sys::MQCIH", &mq::MQCIH::default())?;
    const_default(w, "MQCTLO_DEFAULT", "libmqm_sys::MQCTLO", &mq::MQCTLO::default())?;
    const_default(w, "MQDH_DEFAULT", "libmqm_sys::MQDH", &mq::MQDH::default())?;
    const_default(w, "MQDLH_DEFAULT", "libmqm_sys::MQDLH", &mq::MQDLH::default())?;
    const_default(w, "MQDMPO_DEFAULT", "libmqm_sys::MQDMPO", &mq::MQDMPO::default())?;
    const_default(w, "MQIIH_DEFAULT", "libmqm_sys::MQIIH", &mq::MQIIH::default())?;
    const_default(w, "MQOR_DEFAULT", "libmqm_sys::MQOR", &mq::MQOR::default())?;
    const_default(w, "MQRFH_DEFAULT", "libmqm_sys::MQRFH", &mq::MQRFH::default())?;
    const_default(w, "MQRFH2_DEFAULT", "libmqm_sys::MQRFH2", &mq::MQRFH2::default())?;
    const_default(w, "MQRMH_DEFAULT", "libmqm_sys::MQRMH", &mq::MQRMH::default())?;
    const_default(w, "MQRR_DEFAULT", "libmqm_sys::MQRR", &mq::MQRR::default())?;
    const_default(w, "MQSMPO_DEFAULT", "libmqm_sys::MQSMPO", &mq::MQSMPO::default())?;
    const_default(w, "MQSTS_DEFAULT", "libmqm_sys::MQSTS", &mq::MQSTS::default())?;
    const_default(w, "MQTM_DEFAULT", "libmqm_sys::MQTM", &mq::MQTM::default())?;
    const_default(w, "MQTMC2_DEFAULT", "libmqm_sys::MQTMC2", &mq::MQTMC2::default())?;
    const_default(w, "MQWIH_DEFAULT", "libmqm_sys::MQWIH", &mq::MQWIH::default())?;
    const_default(w, "MQXQH_DEFAULT", "libmqm_sys::MQXQH", &mq::MQXQH::default())?;

    #[cfg(feature = "pcf")]
    {
        use libmqm_sys::pcf;
        writeln!(w, "#[cfg(feature = \"pcf\")]\npub mod pcf {{")?;
        const_default(w, "MQCFH_DEFAULT", "libmqm_sys::pcf::MQCFH", &pcf::MQCFH::default())?;
        const_default(w, "MQCFBF_DEFAULT", "libmqm_sys::pcf::MQCFBF", &pcf::MQCFBF::default())?;
        const_default(w, "MQCFBS_DEFAULT", "libmqm_sys::pcf::MQCFBS", &pcf::MQCFBS::default())?;
        const_default(w, "MQCFGR_DEFAULT", "libmqm_sys::pcf::MQCFGR", &pcf::MQCFGR::default())?;
        const_default(w, "MQCFIF_DEFAULT", "libmqm_sys::pcf::MQCFIF", &pcf::MQCFIF::default())?;
        const_default(w, "MQCFIL_DEFAULT", "libmqm_sys::pcf::MQCFIL", &pcf::MQCFIL::default())?;
        const_default(w, "MQCFIL64_DEFAULT", "libmqm_sys::pcf::MQCFIL64", &pcf::MQCFIL64::default())?;
        const_default(w, "MQCFIN_DEFAULT", "libmqm_sys::pcf::MQCFIN", &pcf::MQCFIN::default())?;
        const_default(w, "MQCFIN64_DEFAULT", "libmqm_sys::pcf::MQCFIN64", &pcf::MQCFIN64::default())?;
        const_default(w, "MQCFSF_DEFAULT", "libmqm_sys::pcf::MQCFSF", &pcf::MQCFSF::default())?;
        const_default(w, "MQCFSL_DEFAULT", "libmqm_sys::pcf::MQCFSL", &pcf::MQCFSL::default())?;
        const_default(w, "MQCFST_DEFAULT", "libmqm_sys::pcf::MQCFST", &pcf::MQCFST::default())?;
        const_default(w, "MQEPH_DEFAULT", "libmqm_sys::pcf::MQEPH", &pcf::MQEPH::default())?;
        writeln!(w, "}}")?;
    }

    #[cfg(feature = "exits")]
    {
        use libmqm_sys::exits;
        writeln!(w, "#[cfg(feature = \"exits\")]\npub mod exits {{")?;
        const_default(w, "MQACH_DEFAULT", "libmqm_sys::exits::MQACH", &exits::MQACH::default())?;
        const_default(w, "MQAXC_DEFAULT", "libmqm_sys::exits::MQAXC", &exits::MQAXC::default())?;
        const_default(w, "MQAXP_DEFAULT", "libmqm_sys::exits::MQAXP", &exits::MQAXP::default())?;
        const_default(w, "MQCXP_DEFAULT", "libmqm_sys::exits::MQCXP", &exits::MQCXP::default())?;
        const_default(w, "MQDXP_DEFAULT", "libmqm_sys::exits::MQDXP", &exits::MQDXP::default())?;
        const_default(w, "MQNXP_DEFAULT", "libmqm_sys::exits::MQNXP", &exits::MQNXP::default())?;
        const_default(w, "MQPBC_DEFAULT", "libmqm_sys::exits::MQPBC", &exits::MQPBC::default())?;
        const_default(w, "MQPSXP_DEFAULT", "libmqm_sys::exits::MQPSXP", &exits::MQPSXP::default())?;
        const_default(w, "MQSBC_DEFAULT", "libmqm_sys::exits::MQSBC", &exits::MQSBC::default())?;
        const_default(w, "MQWCR_DEFAULT", "libmqm_sys::exits::MQWCR", &exits::MQWCR::default())?;
        const_default(w, "MQWDR_DEFAULT", "libmqm_sys::exits::MQWDR", &exits::MQWDR::default())?;
        const_default(w, "MQWDR1_DEFAULT", "libmqm_sys::exits::MQWDR1", &exits::MQWDR1::default())?;
        const_default(w, "MQWDR2_DEFAULT", "libmqm_sys::exits::MQWDR2", &exits::MQWDR2::default())?;
        const_default(w, "MQWQR_DEFAULT", "libmqm_sys::exits::MQWQR", &exits::MQWQR::default())?;
        const_default(w, "MQWQR1_DEFAULT", "libmqm_sys::exits::MQWQR1", &exits::MQWQR1::default())?;
        const_default(w, "MQWQR2_DEFAULT", "libmqm_sys::exits::MQWQR2", &exits::MQWQR2::default())?;
        const_default(w, "MQWQR3_DEFAULT", "libmqm_sys::exits::MQWQR3", &exits::MQWQR3::default())?;
        #[cfg(feature = "mqc_9_3_1_0")]
        {
            writeln!(w, "#[cfg(feature = \"mqc_9_3_1_0\")]")?;
            const_default(w, "MQWQR4_DEFAULT", "libmqm_sys::exits::MQWQR4", &exits::MQWQR4::default())?;
        }
        const_default(w, "MQWXP_DEFAULT", "libmqm_sys::exits::MQWXP", &exits::MQWXP::default())?;
        const_default(w, "MQWXP1_DEFAULT", "libmqm_sys::exits::MQWXP1", &exits::MQWXP1::default())?;
        const_default(w, "MQWXP2_DEFAULT", "libmqm_sys::exits::MQWXP2", &exits::MQWXP2::default())?;
        const_default(w, "MQWXP3_DEFAULT", "libmqm_sys::exits::MQWXP3", &exits::MQWXP3::default())?;
        const_default(w, "MQWXP4_DEFAULT", "libmqm_sys::exits::MQWXP4", &exits::MQWXP4::default())?;
        const_default(w, "MQXEPO_DEFAULT", "libmqm_sys::exits::MQXEPO", &exits::MQXEPO::default())?;
        const_default(w, "MQIEP_DEFAULT", "libmqm_sys::exits::MQIEP", &exits::MQIEP::default())?;
        const_default(w, "MQZED_DEFAULT", "libmqm_sys::exits::MQZED", &exits::MQZED::default())?;
        const_default(w, "MQZAC_DEFAULT", "libmqm_sys::exits::MQZAC", &exits::MQZAC::default())?;
        const_default(w, "MQZAD_DEFAULT", "libmqm_sys::exits::MQZAD", &exits::MQZAD::default())?;
        const_default(w, "MQZFP_DEFAULT", "libmqm_sys::exits::MQZFP", &exits::MQZFP::default())?;
        const_default(w, "MQZIC_DEFAULT", "libmqm_sys::exits::MQZIC", &exits::MQZIC::default())?;
        writeln!(w, "}}")?;
    }

    Ok(())
}

#[cfg(feature = "generate")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, Write};

    pub const POINTER_WIDTH: usize = std::mem::size_of::<usize>() * 8;

    let target_endian = std::env::var("CARGO_CFG_TARGET_ENDIAN").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?; // Mandatory
    let target_pointer_width =
        std::env::var("CARGO_CFG_TARGET_POINTER_WIDTH").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?; // Mandatory

    #[cfg(target_endian = "little")]
    assert_eq!(
        target_endian, "little",
        "the build system endian must match the target platform endian"
    );
    #[cfg(target_endian = "big")]
    assert_eq!(
        target_endian, "big",
        "the build system endian must match the target platform endian"
    );

    assert_eq!(
        format!("{POINTER_WIDTH}"),
        target_pointer_width,
        "the build system pointer size must match the target platform size"
    );

    // Generate and write the serialised defaults
    let out_path =
        std::path::PathBuf::from(std::env::var("OUT_DIR").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?); // Mandatory OUT_DIR
    let defaults_path = out_path.join("defaults.rs");

    let mut defaults_write = Vec::new();
    generate_defaults(&mut defaults_write)?;

    let defaults_str = String::from_utf8(defaults_write)?;
    let defaults_syn = syn::parse_file(&defaults_str)?;
    let defaults_pretty = prettyplease::unparse(&defaults_syn);
    let defaults_file = std::fs::File::create(&defaults_path)?;
    let mut defaults_pretty_write = io::BufWriter::new(defaults_file);
    writeln!(
        defaults_pretty_write,
        "/* Generated with MQ client version {} */",
        libmqm_sys::version::CLIENT_BUILD_VERSION
    )?;
    defaults_pretty_write.write_all(defaults_pretty.as_ref())?;
    drop(defaults_pretty_write);

    #[cfg(feature = "pregen")]
    {
        use std::{fs, path};

        let target_os = std::env::var("CARGO_CFG_TARGET_OS").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?; // Mandatory
        let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?; // Mandatory

        fs::copy(
            defaults_path,
            path::PathBuf::from("src/pregen").join(format!(
                "{}-{}-defaults.rs",
                if target_os == "macos" { "any" } else { &target_arch },
                target_os
            )),
        )?;
    }

    Ok(())
}

#[cfg(not(feature = "generate"))]
fn main() {
    // Do nothing
}
