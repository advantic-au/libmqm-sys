/// Defines the [`Default`] of an MQ structure by using the defines
/// from the MQI C library
macro_rules! mq_default {
    ($struc:ty, $cdefault:ident) => {
        unsafe extern "C" {
            static $cdefault: $struc;
        } // Refer to the compiled c function
        impl ::core::default::Default for $struc {
            fn default() -> Self {
                unsafe { $cdefault }
            }
        }
    };
}

#[cfg(feature = "pcf")]
mod pcf {
    use crate::pcf;

    mq_default!(pcf::MQCFH, mqcfh_default);
    mq_default!(pcf::MQCFBF, mqcfbf_default);
    mq_default!(pcf::MQCFBS, mqcfbs_default);
    mq_default!(pcf::MQCFGR, mqcfgr_default);
    mq_default!(pcf::MQCFIF, mqcfif_default);
    mq_default!(pcf::MQCFIL, mqcfil_default);
    mq_default!(pcf::MQCFIL64, mqcfil64_default);
    mq_default!(pcf::MQCFIN, mqcfin_default);
    mq_default!(pcf::MQCFIN64, mqcfin64_default);
    mq_default!(pcf::MQCFSF, mqcfsf_default);
    mq_default!(pcf::MQCFSL, mqcfsl_default);
    mq_default!(pcf::MQCFST, mqcfst_default);
    mq_default!(pcf::MQEPH, mqeph_default);
}

#[cfg(feature = "exits")]
mod exits {
    use crate::exits;

    mq_default!(exits::MQACH, mqach_default);
    mq_default!(exits::MQAXC, mqaxc_default);
    mq_default!(exits::MQAXP, mqaxp_default);
    mq_default!(exits::MQCXP, mqcxp_default);
    mq_default!(exits::MQDXP, mqdxp_default);
    mq_default!(exits::MQNXP, mqnxp_default);
    mq_default!(exits::MQPBC, mqpbc_default);
    mq_default!(exits::MQPSXP, mqpsxp_default);
    mq_default!(exits::MQSBC, mqsbc_default);
    mq_default!(exits::MQWCR, mqwcr_default);
    mq_default!(exits::MQWDR, mqwdr_default);
    mq_default!(exits::MQWDR1, mqwdr1_default);
    mq_default!(exits::MQWDR2, mqwdr2_default);
    mq_default!(exits::MQWQR, mqwqr_default);
    mq_default!(exits::MQWQR1, mqwqr1_default);
    mq_default!(exits::MQWQR2, mqwqr2_default);
    mq_default!(exits::MQWQR3, mqwqr3_default);
    #[cfg(feature = "mqc_9_3_1_0")]
    mq_default!(exits::MQWQR4, mqwqr4_default);
    mq_default!(exits::MQWXP, mqwxp_default);
    mq_default!(exits::MQWXP1, mqwxp1_default);
    mq_default!(exits::MQWXP2, mqwxp2_default);
    mq_default!(exits::MQWXP3, mqwxp3_default);
    mq_default!(exits::MQWXP4, mqwxp4_default);
    mq_default!(exits::MQXEPO, mqxepo_default);
    mq_default!(exits::MQIEP, mqiep_default);
    mq_default!(exits::MQZED, mqzed_default);
    mq_default!(exits::MQZAC, mqzac_default);
    mq_default!(exits::MQZAD, mqzad_default);
    mq_default!(exits::MQZFP, mqzfp_default);
    mq_default!(exits::MQZIC, mqzic_default);
}

mod mqi {
    use crate as mq;

    unsafe extern "C" {
        static mqcd_client_conn_default: mq::MQCD;
    }
    impl mq::MQCD {
        /// Default `MQCD` suitable for MQI client connections
        #[must_use]
        pub fn client_conn_default() -> Self {
            unsafe { mqcd_client_conn_default }
        }
    }

    mq_default!(mq::MQMD, mqmd_default);
    mq_default!(mq::MQMDE, mqmde_default);
    mq_default!(mq::MQMD1, mqmd1_default);
    mq_default!(mq::MQMD2, mqmd2_default);
    mq_default!(mq::MQPD, mqpd_default);
    mq_default!(mq::MQIMPO, mqimpo_default);
    mq_default!(mq::MQMHBO, mqmhbo_default);
    mq_default!(mq::MQBO, mqbo_default);
    mq_default!(mq::MQDMHO, mqdmho_default);
    mq_default!(mq::MQCMHO, mqcmho_default);
    mq_default!(mq::MQSRO, mqsro_default);
    mq_default!(mq::MQSD, mqsd_default);
    mq_default!(mq::MQGMO, mqgmo_default);
    mq_default!(mq::MQPMO, mqpmo_default);
    mq_default!(mq::MQOD, mqod_default);
    mq_default!(mq::MQCNO, mqcno_default);
    mq_default!(mq::MQCD, mqcd_default);
    mq_default!(mq::MQCSP, mqcsp_default);
    mq_default!(mq::MQSCO, mqsco_default);
    #[cfg(feature = "mqc_9_3_0_0")]
    mq_default!(mq::MQBNO, mqbno_default);
    mq_default!(mq::MQAIR, mqair_default);
    mq_default!(mq::MQBMHO, mqbmho_default);
    mq_default!(mq::MQCBD, mqcbd_default);
    mq_default!(mq::MQCHARV, mqcharv_default);
    mq_default!(mq::MQCIH, mqcih_default);
    mq_default!(mq::MQCTLO, mqctlo_default);
    mq_default!(mq::MQDH, mqdh_default);
    mq_default!(mq::MQDLH, mqdlh_default);
    mq_default!(mq::MQDMPO, mqdmpo_default);
    mq_default!(mq::MQIIH, mqiih_default);
    mq_default!(mq::MQOR, mqor_default);
    mq_default!(mq::MQRFH, mqrfh_default);
    mq_default!(mq::MQRFH2, mqrfh2_default);
    mq_default!(mq::MQRMH, mqrmh_default);
    mq_default!(mq::MQRR, mqrr_default);
    mq_default!(mq::MQSMPO, mqsmpo_default);
    mq_default!(mq::MQSTS, mqsts_default);
    mq_default!(mq::MQTM, mqtm_default);
    mq_default!(mq::MQTMC2, mqtmc2_default);
    mq_default!(mq::MQWIH, mqwih_default);
    mq_default!(mq::MQXQH, mqxqh_default);
}
