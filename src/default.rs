/// Defines the [`Default`] of an MQ structure by using the defines
/// from the MQI C library
macro_rules! mq_default {
    ($struc:ty, $cdefault:ident) => {
        extern "C" {
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
    use crate::lib;

    mq_default!(lib::MQCFH, mqcfh_default);
    mq_default!(lib::MQCFBF, mqcfbf_default);
    mq_default!(lib::MQCFBS, mqcfbs_default);
    mq_default!(lib::MQCFGR, mqcfgr_default);
    mq_default!(lib::MQCFIF, mqcfif_default);
    mq_default!(lib::MQCFIL, mqcfil_default);
    mq_default!(lib::MQCFIL64, mqcfil64_default);
    mq_default!(lib::MQCFIN, mqcfin_default);
    mq_default!(lib::MQCFIN64, mqcfin64_default);
    mq_default!(lib::MQCFSF, mqcfsf_default);
    mq_default!(lib::MQCFSL, mqcfsl_default);
    mq_default!(lib::MQCFST, mqcfst_default);
    mq_default!(lib::MQEPH, mqeph_default);

    mq_default!(lib::MQZED, mqzed_default);
    mq_default!(lib::MQZAC, mqzac_default);
    mq_default!(lib::MQZAD, mqzad_default);
    mq_default!(lib::MQZFP, mqzfp_default);
    mq_default!(lib::MQZIC, mqzic_default);
}

#[cfg(feature = "exits")]
mod exits {
    use crate::lib;

    mq_default!(lib::MQACH, mqach_default);
    mq_default!(lib::MQAXC, mqaxc_default);
    mq_default!(lib::MQAXP, mqaxp_default);
    mq_default!(lib::MQCXP, mqcxp_default);
    mq_default!(lib::MQDXP, mqdxp_default);
    mq_default!(lib::MQNXP, mqnxp_default);
    mq_default!(lib::MQPBC, mqpbc_default);
    mq_default!(lib::MQPSXP, mqpsxp_default);
    mq_default!(lib::MQSBC, mqsbc_default);
    mq_default!(lib::MQWCR, mqwcr_default);
    mq_default!(lib::MQWDR, mqwdr_default);
    mq_default!(lib::MQWDR1, mqwdr1_default);
    mq_default!(lib::MQWDR2, mqwdr2_default);
    mq_default!(lib::MQWQR, mqwqr_default);
    mq_default!(lib::MQWQR1, mqwqr1_default);
    mq_default!(lib::MQWQR2, mqwqr2_default);
    mq_default!(lib::MQWQR3, mqwqr3_default);
    mq_default!(lib::MQWQR4, mqwqr4_default);
    mq_default!(lib::MQWXP, mqwxp_default);
    mq_default!(lib::MQWXP1, mqwxp1_default);
    mq_default!(lib::MQWXP2, mqwxp2_default);
    mq_default!(lib::MQWXP3, mqwxp3_default);
    mq_default!(lib::MQWXP4, mqwxp4_default);
    mq_default!(lib::MQXEPO, mqxepo_default);
}

mod mqi {
    use crate::lib;

    extern "C" {
        static mqcd_client_conn_default: lib::MQCD;
    }
    impl lib::MQCD {
        /// Default `MQCD` suitable for MQI client connections
        #[must_use]
        pub fn client_conn_default() -> Self {
            unsafe { mqcd_client_conn_default }
        }
    }

    mq_default!(lib::MQMD, mqmd_default);
    mq_default!(lib::MQMDE, mqmde_default);
    mq_default!(lib::MQMD1, mqmd1_default);
    mq_default!(lib::MQMD2, mqmd2_default);
    mq_default!(lib::MQPD, mqpd_default);
    mq_default!(lib::MQIMPO, mqimpo_default);
    mq_default!(lib::MQMHBO, mqmhbo_default);
    mq_default!(lib::MQBO, mqbo_default);
    mq_default!(lib::MQDMHO, mqdmho_default);
    mq_default!(lib::MQCMHO, mqcmho_default);
    mq_default!(lib::MQSRO, mqsro_default);
    mq_default!(lib::MQSD, mqsd_default);
    mq_default!(lib::MQGMO, mqgmo_default);
    mq_default!(lib::MQPMO, mqpmo_default);
    mq_default!(lib::MQOD, mqod_default);
    mq_default!(lib::MQCNO, mqcno_default);
    mq_default!(lib::MQCD, mqcd_default);
    mq_default!(lib::MQCSP, mqcsp_default);
    mq_default!(lib::MQSCO, mqsco_default);
    mq_default!(lib::MQBNO, mqbno_default);
    mq_default!(lib::MQAIR, mqair_default);
    mq_default!(lib::MQBMHO, mqbmho_default);
    mq_default!(lib::MQCBD, mqcbd_default);
    mq_default!(lib::MQCHARV, mqcharv_default);
    mq_default!(lib::MQCIH, mqcih_default);
    mq_default!(lib::MQCTLO, mqctlo_default);
    mq_default!(lib::MQDH, mqdh_default);
    mq_default!(lib::MQDLH, mqdlh_default);
    mq_default!(lib::MQDMPO, mqdmpo_default);
    mq_default!(lib::MQIIH, mqiih_default);
    mq_default!(lib::MQOR, mqor_default);
    mq_default!(lib::MQRFH, mqrfh_default);
    mq_default!(lib::MQRFH2, mqrfh2_default);
    mq_default!(lib::MQRMH, mqrmh_default);
    mq_default!(lib::MQRR, mqrr_default);
    mq_default!(lib::MQSMPO, mqsmpo_default);
    mq_default!(lib::MQSTS, mqsts_default);
    mq_default!(lib::MQTM, mqtm_default);
    mq_default!(lib::MQTMC2, mqtmc2_default);
    mq_default!(lib::MQWIH, mqwih_default);
    mq_default!(lib::MQXQH, mqxqh_default);
}
