/* Generated with MQ client version 9.4.3.0 */
/*
 * Unmapped constants:
 *  MQAUSC_ALLCHECKS
 *  MQAUSC_ALLCONNS
 *  MQAUSC_FAILURES
 *  MQBA_FIRST
 *  MQBA_LAST
 *  MQGA_FIRST
 *  MQGA_LAST
 *  MQOA_FIRST
 *  MQOA_LAST
 *  MQOTEL_PCTL_AS_PARENT
 *  MQOTEL_PCTL_AUTO
 *  MQOTEL_PCTL_MANUAL
 *  MQOTEL_PCTL_QMGR
 *  MQOTEL_TRACE_AS_PARENT
 *  MQOTEL_TRACE_NONE
 *  MQOTEL_TRACE_OFF
 *  MQOTEL_TRACE_ON
 *  MQOTEL_TRACE_QMGR
 *  MQUA_FIRST
 *  MQUA_LAST
 */

pub mod types {
    use ::libmqm_sys::lib as mqsys;
    use crate::mapping;
    use crate::value::{define_new_type, impl_value};
    use crate::bitflags::impl_bitflags;
    define_new_type!(
        pub MQACTP, mqsys::MQLONG, mapping::MQACTP_MAPSTR, r##"Property Action"##
    );
    impl_value!(MQACTP, mqsys::MQLONG);
    define_new_type!(
        pub MQACTV, mqsys::MQLONG, mapping::MQACTV_MAPSTR,
        r##"Application Activity Trace Detail"##
    );
    impl_value!(MQACTV, mqsys::MQLONG);
    define_new_type!(
        pub MQADOPT_CHECK, mqsys::MQLONG, mapping::MQADOPT_CHECK_MAPSTR,
        r##"Adopt New MCA Checks"##
    );
    impl_value!(MQADOPT_CHECK, mqsys::MQLONG);
    define_new_type!(
        pub MQADOPT_TYPE, mqsys::MQLONG, mapping::MQADOPT_TYPE_MAPSTR,
        r##"Adopt New MCA Types"##
    );
    impl_value!(MQADOPT_TYPE, mqsys::MQLONG);
    define_new_type!(
        pub MQAIT, mqsys::MQLONG, mapping::MQAIT_MAPSTR,
        r##"Authentication Information Type"##
    );
    impl_value!(MQAIT, mqsys::MQLONG);
    define_new_type!(
        pub MQAT, mqsys::MQLONG, mapping::MQAT_MAPSTR, r##"Put Application Types"##
    );
    impl_value!(MQAT, mqsys::MQLONG);
    define_new_type!(
        pub MQAUTO, mqsys::MQLONG, mapping::MQAUTO_MAPSTR, r##"Autostart"##
    );
    impl_value!(MQAUTO, mqsys::MQLONG);
    define_new_type!(
        pub MQBMHO, mqsys::MQLONG, mapping::MQBMHO_MAPSTR,
        r##"Buffer To Message Handle Options"##
    );
    impl_bitflags!(MQBMHO, mqsys::MQLONG);
    define_new_type!(
        pub MQBND, mqsys::MQLONG, mapping::MQBND_MAPSTR, r##"Default Bindings"##
    );
    impl_value!(MQBND, mqsys::MQLONG);
    define_new_type!(
        pub MQBNO_BALTYPE, mqsys::MQLONG, mapping::MQBNO_BALTYPE_MAPSTR,
        r##"MQ Balancing Application Type"##
    );
    impl_value!(MQBNO_BALTYPE, mqsys::MQLONG);
    define_new_type!(
        pub MQBNO_OPTIONS, mqsys::MQLONG, mapping::MQBNO_OPTIONS_MAPSTR,
        r##"MQ Balancing Options"##
    );
    impl_value!(MQBNO_OPTIONS, mqsys::MQLONG);
    define_new_type!(
        pub MQBNO_TIMEOUT, mqsys::MQLONG, mapping::MQBNO_TIMEOUT_MAPSTR,
        r##"MQ Balancing Timeout"##
    );
    impl_value!(MQBNO_TIMEOUT, mqsys::MQLONG);
    define_new_type!(
        pub MQBO, mqsys::MQLONG, mapping::MQBO_MAPSTR, r##"Begin Options"##
    );
    impl_bitflags!(MQBO, mqsys::MQLONG);
    define_new_type!(
        pub MQCADSD, mqsys::MQLONG, mapping::MQCADSD_MAPSTR, r##"ADS Descriptors"##
    );
    impl_value!(MQCADSD, mqsys::MQLONG);
    define_new_type!(
        pub MQCAFTY, mqsys::MQLONG, mapping::MQCAFTY_MAPSTR,
        r##"Connection Affinity Values"##
    );
    impl_value!(MQCAFTY, mqsys::MQLONG);
    define_new_type!(pub MQCAP, mqsys::MQLONG, mapping::MQCAP_MAPSTR, r##"Capability"##);
    impl_value!(MQCAP, mqsys::MQLONG);
    define_new_type!(
        pub MQCA, mqsys::MQLONG, mapping::MQCA_MAPSTR,
        r##"Character Attribute Selectors"##
    );
    impl_value!(MQCA, mqsys::MQLONG);
    define_new_type!(
        pub MQCBCF, mqsys::MQLONG, mapping::MQCBCF_MAPSTR, r##"Callback Context Flags"##
    );
    impl_value!(MQCBCF, mqsys::MQLONG);
    define_new_type!(
        pub MQCBCT, mqsys::MQLONG, mapping::MQCBCT_MAPSTR, r##"Callback type"##
    );
    impl_value!(MQCBCT, mqsys::MQLONG);
    define_new_type!(
        pub MQCBDO, mqsys::MQLONG, mapping::MQCBDO_MAPSTR, r##"Callback Options"##
    );
    impl_bitflags!(MQCBDO, mqsys::MQLONG);
    define_new_type!(
        pub MQCBD, mqsys::MQLONG, mapping::MQCBD_MAPSTR, r##"Buffer size values"##
    );
    impl_value!(MQCBD, mqsys::MQLONG);
    define_new_type!(
        pub MQCBT, mqsys::MQLONG, mapping::MQCBT_MAPSTR,
        r##"This is the type of the Callback Function"##
    );
    impl_value!(MQCBT, mqsys::MQLONG);
    define_new_type!(
        pub MQCCSI, mqsys::MQLONG, mapping::MQCCSI_MAPSTR,
        r##"Coded Character Set Identifiers"##
    );
    impl_value!(MQCCSI, mqsys::MQLONG);
    define_new_type!(
        pub MQCCT, mqsys::MQLONG, mapping::MQCCT_MAPSTR,
        r##"Conversational Task Options"##
    );
    impl_value!(MQCCT, mqsys::MQLONG);
    define_new_type!(
        pub MQCC, mqsys::MQLONG, mapping::MQCC_MAPSTR, r##"Completion Codes"##
    );
    impl_value!(MQCC, mqsys::MQLONG);
    define_new_type!(
        pub MQCEX, mqsys::MQLONG, mapping::MQCEX_MAPSTR, r##"Queue cap expiry values"##
    );
    impl_value!(MQCEX, mqsys::MQLONG);
    define_new_type!(
        pub MQCFCONLOS, mqsys::MQLONG, mapping::MQCFCONLOS_MAPSTR,
        r##"CF Loss of Connectivity Action"##
    );
    impl_value!(MQCFCONLOS, mqsys::MQLONG);
    define_new_type!(
        pub MQCFOFFLD, mqsys::MQLONG, mapping::MQCFOFFLD_MAPSTR, r##"OffldUse Options"##
    );
    impl_value!(MQCFOFFLD, mqsys::MQLONG);
    define_new_type!(
        pub MQCFR, mqsys::MQLONG, mapping::MQCFR_MAPSTR, r##"CF Recoverability"##
    );
    impl_value!(MQCFR, mqsys::MQLONG);
    define_new_type!(
        pub MQCGWI, mqsys::MQLONG, mapping::MQCGWI_MAPSTR, r##"Get Wait Interval"##
    );
    impl_value!(MQCGWI, mqsys::MQLONG);
    define_new_type!(
        pub MQCHAD, mqsys::MQLONG, mapping::MQCHAD_MAPSTR, r##"Channel Auto Definition"##
    );
    impl_value!(MQCHAD, mqsys::MQLONG);
    define_new_type!(
        pub MQCHT, mqsys::MQLONG, mapping::MQCHT_MAPSTR, r##"Channel Types"##
    );
    impl_value!(MQCHT, mqsys::MQLONG);
    define_new_type!(
        pub MQCIH, mqsys::MQLONG, mapping::MQCIH_MAPSTR, r##"MQCIH Flags"##
    );
    impl_value!(MQCIH, mqsys::MQLONG);
    define_new_type!(
        pub MQCIT, mqsys::MQLONG, mapping::MQCIT_MAPSTR, r##"CommInfo Type"##
    );
    impl_value!(MQCIT, mqsys::MQLONG);
    define_new_type!(pub MQCLT, mqsys::MQLONG, mapping::MQCLT_MAPSTR, r##"Link Types"##);
    impl_value!(MQCLT, mqsys::MQLONG);
    define_new_type!(
        pub MQCLWL, mqsys::MQLONG, mapping::MQCLWL_MAPSTR, r##"Cluster Workload"##
    );
    impl_value!(MQCLWL, mqsys::MQLONG);
    define_new_type!(
        pub MQCMDL, mqsys::MQLONG, mapping::MQCMDL_MAPSTR, r##"Command Levels"##
    );
    impl_value!(MQCMDL, mqsys::MQLONG);
    define_new_type!(
        pub MQCMHO, mqsys::MQLONG, mapping::MQCMHO_MAPSTR,
        r##"Create Message Handle Options"##
    );
    impl_value!(MQCMHO, mqsys::MQLONG);
    define_new_type!(
        pub MQCNO, mqsys::MQLONG, mapping::MQCNO_MAPSTR, r##"Connect Options"##
    );
    impl_bitflags!(MQCNO, mqsys::MQLONG);
    define_new_type!(
        pub MQCODL, mqsys::MQLONG, mapping::MQCODL_MAPSTR, r##"Output Data Length"##
    );
    impl_value!(MQCODL, mqsys::MQLONG);
    define_new_type!(
        pub MQCOPY, mqsys::MQLONG, mapping::MQCOPY_MAPSTR, r##"Property Copy Options"##
    );
    impl_bitflags!(MQCOPY, mqsys::MQLONG);
    define_new_type!(
        pub MQCO, mqsys::MQLONG, mapping::MQCO_MAPSTR, r##"Close Options"##
    );
    impl_bitflags!(MQCO, mqsys::MQLONG);
    define_new_type!(
        pub MQCQT, mqsys::MQLONG, mapping::MQCQT_MAPSTR, r##"Cluster Queue Types"##
    );
    impl_value!(MQCQT, mqsys::MQLONG);
    define_new_type!(
        pub MQCRC, mqsys::MQLONG, mapping::MQCRC_MAPSTR, r##"Return Codes"##
    );
    impl_value!(MQCRC, mqsys::MQLONG);
    define_new_type!(
        pub MQCSP, mqsys::MQLONG, mapping::MQCSP_MAPSTR, r##"Authentication Types"##
    );
    impl_value!(MQCSP, mqsys::MQLONG);
    define_new_type!(
        pub MQCSRV_CONVERT, mqsys::MQLONG, mapping::MQCSRV_CONVERT_MAPSTR,
        r##"Command Server Options"##
    );
    impl_value!(MQCSRV_CONVERT, mqsys::MQLONG);
    define_new_type!(
        pub MQCSRV_DLQ, mqsys::MQLONG, mapping::MQCSRV_DLQ_MAPSTR,
        r##"Command Server Options"##
    );
    impl_value!(MQCSRV_DLQ, mqsys::MQLONG);
    define_new_type!(
        pub MQCS, mqsys::MQLONG, mapping::MQCS_MAPSTR, r##"Consumer state"##
    );
    impl_value!(MQCS, mqsys::MQLONG);
    define_new_type!(
        pub MQCTES, mqsys::MQLONG, mapping::MQCTES_MAPSTR, r##"Task End Status"##
    );
    impl_value!(MQCTES, mqsys::MQLONG);
    define_new_type!(
        pub MQCTLO, mqsys::MQLONG, mapping::MQCTLO_MAPSTR,
        r##"Consumer Control Options"##
    );
    impl_bitflags!(MQCTLO, mqsys::MQLONG);
    define_new_type!(
        pub MQCUOWC, mqsys::MQLONG, mapping::MQCUOWC_MAPSTR, r##"Unit-of-Work Controls"##
    );
    impl_value!(MQCUOWC, mqsys::MQLONG);
    define_new_type!(
        pub MQDC, mqsys::MQLONG, mapping::MQDC_MAPSTR, r##"Destination Class"##
    );
    impl_value!(MQDC, mqsys::MQLONG);
    define_new_type!(pub MQDHF, mqsys::MQLONG, mapping::MQDHF_MAPSTR, r##"MQDH Flags"##);
    impl_value!(MQDHF, mqsys::MQLONG);
    define_new_type!(
        pub MQDLV, mqsys::MQLONG, mapping::MQDLV_MAPSTR,
        r##"Persistent/Non-persistent Message Delivery"##
    );
    impl_value!(MQDLV, mqsys::MQLONG);
    define_new_type!(
        pub MQDL, mqsys::MQLONG, mapping::MQDL_MAPSTR, r##"Distribution Lists"##
    );
    impl_value!(MQDL, mqsys::MQLONG);
    define_new_type!(
        pub MQDMHO, mqsys::MQLONG, mapping::MQDMHO_MAPSTR,
        r##"Delete Message Handle Options"##
    );
    impl_value!(MQDMHO, mqsys::MQLONG);
    define_new_type!(
        pub MQDMPO, mqsys::MQLONG, mapping::MQDMPO_MAPSTR,
        r##"Delete Message Property Options"##
    );
    impl_value!(MQDMPO, mqsys::MQLONG);
    define_new_type!(
        pub MQDNSWLM, mqsys::MQLONG, mapping::MQDNSWLM_MAPSTR, r##"DNS WLM"##
    );
    impl_value!(MQDNSWLM, mqsys::MQLONG);
    define_new_type!(
        pub MQDSB, mqsys::MQLONG, mapping::MQDSB_MAPSTR, r##"DSBlock Options"##
    );
    impl_value!(MQDSB, mqsys::MQLONG);
    define_new_type!(
        pub MQDSE, mqsys::MQLONG, mapping::MQDSE_MAPSTR, r##"DSExpand Options"##
    );
    impl_value!(MQDSE, mqsys::MQLONG);
    define_new_type!(
        pub MQEC, mqsys::MQLONG, mapping::MQEC_MAPSTR, r##"Signal Values"##
    );
    impl_value!(MQEC, mqsys::MQLONG);
    define_new_type!(pub MQEI, mqsys::MQLONG, mapping::MQEI_MAPSTR, r##"Expiry"##);
    impl_value!(MQEI, mqsys::MQLONG);
    define_new_type!(pub MQENC, mqsys::MQLONG, mapping::MQENC_MAPSTR, r##"Encoding"##);
    impl_bitflags!(MQENC, mqsys::MQLONG);
    define_new_type!(
        pub MQEXPI, mqsys::MQLONG, mapping::MQEXPI_MAPSTR,
        r##"Expiration Scan Interval"##
    );
    impl_value!(MQEXPI, mqsys::MQLONG);
    define_new_type!(
        pub MQFB, mqsys::MQLONG, mapping::MQFB_MAPSTR, r##"Feedback Values"##
    );
    impl_value!(MQFB, mqsys::MQLONG);
    define_new_type!(pub MQFIELD_WQR, mqsys::MQLONG, mapping::MQFIELD_WQR_MAPSTR);
    impl_value!(MQFIELD_WQR, mqsys::MQLONG);
    define_new_type!(
        pub MQFUN, mqsys::MQLONG, mapping::MQFUN_MAPSTR,
        r##"Application Function Types"##
    );
    impl_value!(MQFUN, mqsys::MQLONG);
    define_new_type!(
        pub MQGMO, mqsys::MQLONG, mapping::MQGMO_MAPSTR, r##"Get Message Options"##
    );
    impl_bitflags!(MQGMO, mqsys::MQLONG);
    define_new_type!(
        pub MQHC, mqsys::MQHCONN, mapping::MQHC_MAPSTR, r##"Connection Handles"##
    );
    impl_value!(MQHC, mqsys::MQHCONN);
    define_new_type!(
        pub MQHM, mqsys::MQHMSG, mapping::MQHM_MAPSTR, r##"Message handle"##
    );
    impl_value!(MQHM, mqsys::MQHMSG);
    define_new_type!(
        pub MQHO, mqsys::MQHOBJ, mapping::MQHO_MAPSTR, r##"Object Handle"##
    );
    impl_value!(MQHO, mqsys::MQHOBJ);
    define_new_type!(
        pub MQIAV, mqsys::MQLONG, mapping::MQIAV_MAPSTR, r##"Integer Attribute Values"##
    );
    impl_value!(MQIAV, mqsys::MQLONG);
    define_new_type!(
        pub MQIA, mqsys::MQLONG, mapping::MQIA_MAPSTR, r##"Integer Attribute Selectors"##
    );
    impl_value!(MQIA, mqsys::MQLONG);
    define_new_type!(
        pub MQIGQPA, mqsys::MQLONG, mapping::MQIGQPA_MAPSTR,
        r##"Intra-Group Queuing Put Authority"##
    );
    impl_value!(MQIGQPA, mqsys::MQLONG);
    define_new_type!(
        pub MQIGQ, mqsys::MQLONG, mapping::MQIGQ_MAPSTR, r##"Intra-Group Queuing"##
    );
    impl_value!(MQIGQ, mqsys::MQLONG);
    define_new_type!(
        pub MQIIH, mqsys::MQLONG, mapping::MQIIH_MAPSTR, r##"MQIIH Flags"##
    );
    impl_value!(MQIIH, mqsys::MQLONG);
    define_new_type!(
        pub MQIMGRCOV, mqsys::MQLONG, mapping::MQIMGRCOV_MAPSTR,
        r##"Media Image Recoverability"##
    );
    impl_value!(MQIMGRCOV, mqsys::MQLONG);
    define_new_type!(
        pub MQIMPO, mqsys::MQLONG, mapping::MQIMPO_MAPSTR,
        r##"Inquire Message Property Options"##
    );
    impl_bitflags!(MQIMPO, mqsys::MQLONG);
    define_new_type!(
        pub MQIPADDR, mqsys::MQLONG, mapping::MQIPADDR_MAPSTR, r##"IP Address Versions"##
    );
    impl_value!(MQIPADDR, mqsys::MQLONG);
    define_new_type!(pub MQIT, mqsys::MQLONG, mapping::MQIT_MAPSTR, r##"Index Types"##);
    impl_value!(MQIT, mqsys::MQLONG);
    define_new_type!(
        pub MQKAI, mqsys::MQLONG, mapping::MQKAI_MAPSTR, r##"KeepAlive Interval"##
    );
    impl_value!(MQKAI, mqsys::MQLONG);
    define_new_type!(
        pub MQKEY, mqsys::MQLONG, mapping::MQKEY_MAPSTR, r##"Key reuse count"##
    );
    impl_value!(MQKEY, mqsys::MQLONG);
    define_new_type!(
        pub MQMASTER, mqsys::MQLONG, mapping::MQMASTER_MAPSTR,
        r##"Master administration"##
    );
    impl_value!(MQMASTER, mqsys::MQLONG);
    define_new_type!(
        pub MQMCB, mqsys::MQLONG, mapping::MQMCB_MAPSTR, r##"CommInfo Bridge"##
    );
    impl_value!(MQMCB, mqsys::MQLONG);
    define_new_type!(pub MQMC, mqsys::MQLONG, mapping::MQMC_MAPSTR, r##"Multicast"##);
    impl_value!(MQMC, mqsys::MQLONG);
    define_new_type!(
        pub MQMDEF, mqsys::MQLONG, mapping::MQMDEF_MAPSTR, r##"MQMDE Flags"##
    );
    impl_value!(MQMDEF, mqsys::MQLONG);
    define_new_type!(
        pub MQMDS, mqsys::MQLONG, mapping::MQMDS_MAPSTR, r##"Message Delivery Sequence"##
    );
    impl_value!(MQMDS, mqsys::MQLONG);
    define_new_type!(
        pub MQMEDIMGINTVL, mqsys::MQLONG, mapping::MQMEDIMGINTVL_MAPSTR,
        r##"Automatic Media Image Interval"##
    );
    impl_value!(MQMEDIMGINTVL, mqsys::MQLONG);
    define_new_type!(
        pub MQMEDIMGLOGLN, mqsys::MQLONG, mapping::MQMEDIMGLOGLN_MAPSTR,
        r##"Automatic Media Image Log Length"##
    );
    impl_value!(MQMEDIMGLOGLN, mqsys::MQLONG);
    define_new_type!(
        pub MQMEDIMGSCHED, mqsys::MQLONG, mapping::MQMEDIMGSCHED_MAPSTR,
        r##"Media Image Scheduling"##
    );
    impl_value!(MQMEDIMGSCHED, mqsys::MQLONG);
    define_new_type!(
        pub MQMF, mqsys::MQLONG, mapping::MQMF_MAPSTR, r##"Message Flags"##
    );
    impl_value!(MQMF, mqsys::MQLONG);
    define_new_type!(
        pub MQMHBO, mqsys::MQLONG, mapping::MQMHBO_MAPSTR,
        r##"Message Handle To Buffer Options"##
    );
    impl_bitflags!(MQMHBO, mqsys::MQLONG);
    define_new_type!(
        pub MQMMBI, mqsys::MQLONG, mapping::MQMMBI_MAPSTR,
        r##"Message Mark-Browse Interval"##
    );
    impl_value!(MQMMBI, mqsys::MQLONG);
    define_new_type!(
        pub MQMON, mqsys::MQLONG, mapping::MQMON_MAPSTR, r##"Monitoring Values"##
    );
    impl_value!(MQMON, mqsys::MQLONG);
    define_new_type!(
        pub MQMON_AVAILABILITY, mqsys::MQLONG, mapping::MQMON_AVAILABILITY_MAPSTR,
        r##"Monitoring Values"##
    );
    impl_value!(MQMON_AVAILABILITY, mqsys::MQLONG);
    define_new_type!(
        pub MQMON_OVERRIDE, mqsys::MQLONG, mapping::MQMON_OVERRIDE_MAPSTR,
        r##"Monitoring Values"##
    );
    impl_value!(MQMON_OVERRIDE, mqsys::MQLONG);
    define_new_type!(
        pub MQMO, mqsys::MQLONG, mapping::MQMO_MAPSTR, r##"Match Options"##
    );
    impl_bitflags!(MQMO, mqsys::MQLONG);
    define_new_type!(
        pub MQMT, mqsys::MQLONG, mapping::MQMT_MAPSTR, r##"Message Types"##
    );
    impl_value!(MQMT, mqsys::MQLONG);
    define_new_type!(pub MQNC, mqsys::MQLONG, mapping::MQNC_MAPSTR, r##"Name Count"##);
    impl_value!(MQNC, mqsys::MQLONG);
    define_new_type!(
        pub MQNPM, mqsys::MQLONG, mapping::MQNPM_MAPSTR,
        r##"Nonpersistent Message Class"##
    );
    impl_value!(MQNPM, mqsys::MQLONG);
    define_new_type!(
        pub MQNT, mqsys::MQLONG, mapping::MQNT_MAPSTR, r##"Namelist Types"##
    );
    impl_value!(MQNT, mqsys::MQLONG);
    define_new_type!(
        pub MQOL, mqsys::MQLONG, mapping::MQOL_MAPSTR, r##"Original Length"##
    );
    impl_value!(MQOL, mqsys::MQLONG);
    define_new_type!(
        pub MQOM, mqsys::MQLONG, mapping::MQOM_MAPSTR,
        r##"Obsolete DB2 Messages options on Inquire Group"##
    );
    impl_value!(MQOM, mqsys::MQLONG);
    define_new_type!(pub MQOO, mqsys::MQLONG, mapping::MQOO_MAPSTR, r##"Open Options"##);
    impl_bitflags!(MQOO, mqsys::MQLONG);
    define_new_type!(
        pub MQOP, mqsys::MQLONG, mapping::MQOP_MAPSTR,
        r##"Operation codes for MQCTL and MQCB"##
    );
    impl_bitflags!(MQOP, mqsys::MQLONG);
    define_new_type!(
        pub MQOT, mqsys::MQLONG, mapping::MQOT_MAPSTR, r##"Extended Object Types"##
    );
    impl_value!(MQOT, mqsys::MQLONG);
    define_new_type!(
        pub MQPD, mqsys::MQLONG, mapping::MQPD_MAPSTR, r##"Property Context"##
    );
    impl_value!(MQPD, mqsys::MQLONG);
    define_new_type!(
        pub MQPER, mqsys::MQLONG, mapping::MQPER_MAPSTR, r##"Persistence Values"##
    );
    impl_value!(MQPER, mqsys::MQLONG);
    define_new_type!(pub MQPL, mqsys::MQLONG, mapping::MQPL_MAPSTR, r##"Platforms"##);
    impl_value!(MQPL, mqsys::MQLONG);
    define_new_type!(
        pub MQPMO, mqsys::MQLONG, mapping::MQPMO_MAPSTR,
        r##"Put Message Options for publish mask"##
    );
    impl_bitflags!(MQPMO, mqsys::MQLONG);
    define_new_type!(
        pub MQPMRF, mqsys::MQLONG, mapping::MQPMRF_MAPSTR,
        r##"Put Message Record Fields"##
    );
    impl_value!(MQPMRF, mqsys::MQLONG);
    define_new_type!(pub MQPRI, mqsys::MQLONG, mapping::MQPRI_MAPSTR, r##"Priority"##);
    impl_value!(MQPRI, mqsys::MQLONG);
    define_new_type!(
        pub MQPROP, mqsys::MQLONG, mapping::MQPROP_MAPSTR,
        r##"Queue and Channel Property Control Values"##
    );
    impl_value!(MQPROP, mqsys::MQLONG);
    define_new_type!(
        pub MQPRT, mqsys::MQLONG, mapping::MQPRT_MAPSTR, r##"Put Response Values"##
    );
    impl_value!(MQPRT, mqsys::MQLONG);
    define_new_type!(
        pub MQPSCLUS, mqsys::MQLONG, mapping::MQPSCLUS_MAPSTR, r##"Pub/Sub clusters"##
    );
    impl_value!(MQPSCLUS, mqsys::MQLONG);
    define_new_type!(
        pub MQPSM, mqsys::MQLONG, mapping::MQPSM_MAPSTR, r##"Pub/Sub Mode"##
    );
    impl_value!(MQPSM, mqsys::MQLONG);
    define_new_type!(
        pub MQPSPROP, mqsys::MQLONG, mapping::MQPSPROP_MAPSTR,
        r##"Pub/Sub Message Properties"##
    );
    impl_value!(MQPSPROP, mqsys::MQLONG);
    define_new_type!(
        pub MQQA_BACKOUT, mqsys::MQLONG, mapping::MQQA_BACKOUT_MAPSTR,
        r##"Back-Out Hardening"##
    );
    impl_value!(MQQA_BACKOUT, mqsys::MQLONG);
    define_new_type!(
        pub MQQA_GET, mqsys::MQLONG, mapping::MQQA_GET_MAPSTR, r##"Inhibit Get Values"##
    );
    impl_value!(MQQA_GET, mqsys::MQLONG);
    define_new_type!(
        pub MQQA_PUT, mqsys::MQLONG, mapping::MQQA_PUT_MAPSTR, r##"Inhibit Put Values"##
    );
    impl_value!(MQQA_PUT, mqsys::MQLONG);
    define_new_type!(
        pub MQQA_SHAREABLE, mqsys::MQLONG, mapping::MQQA_SHAREABLE_MAPSTR,
        r##"Queue Shareability"##
    );
    impl_value!(MQQA_SHAREABLE, mqsys::MQLONG);
    define_new_type!(
        pub MQQDT, mqsys::MQLONG, mapping::MQQDT_MAPSTR, r##"Queue Definition Types"##
    );
    impl_value!(MQQDT, mqsys::MQLONG);
    define_new_type!(
        pub MQQFS, mqsys::MQLONG, mapping::MQQFS_MAPSTR,
        r##"Max queue file size values"##
    );
    impl_value!(MQQFS, mqsys::MQLONG);
    define_new_type!(
        pub MQQMOPT, mqsys::MQLONG, mapping::MQQMOPT_MAPSTR, r##"Control Options"##
    );
    impl_value!(MQQMOPT, mqsys::MQLONG);
    define_new_type!(
        pub MQQSGD, mqsys::MQLONG, mapping::MQQSGD_MAPSTR,
        r##"Queue Sharing Group Dispositions"##
    );
    impl_value!(MQQSGD, mqsys::MQLONG);
    define_new_type!(
        pub MQQT, mqsys::MQLONG, mapping::MQQT_MAPSTR, r##"Extended Queue Types"##
    );
    impl_value!(MQQT, mqsys::MQLONG);
    define_new_type!(
        pub MQRCN, mqsys::MQLONG, mapping::MQRCN_MAPSTR, r##"Client Reconnect"##
    );
    impl_value!(MQRCN, mqsys::MQLONG);
    define_new_type!(
        pub MQRCVTIME, mqsys::MQLONG, mapping::MQRCVTIME_MAPSTR,
        r##"Receive Timeout Types"##
    );
    impl_value!(MQRCVTIME, mqsys::MQLONG);
    define_new_type!(pub MQRC, mqsys::MQLONG, mapping::MQRC_MAPSTR, r##"Reason Codes"##);
    impl_value!(MQRC, mqsys::MQLONG);
    define_new_type!(
        pub MQRD, mqsys::MQLONG, mapping::MQRD_MAPSTR, r##"Reconnect delay"##
    );
    impl_value!(MQRD, mqsys::MQLONG);
    define_new_type!(
        pub MQREADA, mqsys::MQLONG, mapping::MQREADA_MAPSTR, r##"Read Ahead Values"##
    );
    impl_value!(MQREADA, mqsys::MQLONG);
    define_new_type!(
        pub MQRECAUTO, mqsys::MQLONG, mapping::MQRECAUTO_MAPSTR,
        r##"CF Automatic Recovery"##
    );
    impl_value!(MQRECAUTO, mqsys::MQLONG);
    define_new_type!(
        pub MQRECORDING, mqsys::MQLONG, mapping::MQRECORDING_MAPSTR,
        r##"Recording Options"##
    );
    impl_value!(MQRECORDING, mqsys::MQLONG);
    define_new_type!(
        pub MQREORG, mqsys::MQLONG, mapping::MQREORG_MAPSTR,
        r##"Reorganization Controls"##
    );
    impl_value!(MQREORG, mqsys::MQLONG);
    define_new_type!(
        pub MQRFH, mqsys::MQLONG, mapping::MQRFH_MAPSTR, r##"MQRFH Flags"##
    );
    impl_value!(MQRFH, mqsys::MQLONG);
    define_new_type!(
        pub MQRL, mqsys::MQLONG, mapping::MQRL_MAPSTR, r##"Returned Length"##
    );
    impl_value!(MQRL, mqsys::MQLONG);
    define_new_type!(
        pub MQRMHF, mqsys::MQLONG, mapping::MQRMHF_MAPSTR, r##"MQRMH Flags"##
    );
    impl_value!(MQRMHF, mqsys::MQLONG);
    define_new_type!(
        pub MQRO, mqsys::MQLONG, mapping::MQRO_MAPSTR, r##"Report Options Masks"##
    );
    impl_bitflags!(MQRO, mqsys::MQLONG);
    define_new_type!(pub MQRU, mqsys::MQLONG, mapping::MQRU_MAPSTR, r##"Request Only"##);
    impl_value!(MQRU, mqsys::MQLONG);
    define_new_type!(
        pub MQSCOPE, mqsys::MQLONG, mapping::MQSCOPE_MAPSTR, r##"Publish scope"##
    );
    impl_value!(MQSCOPE, mqsys::MQLONG);
    define_new_type!(
        pub MQSCO, mqsys::MQLONG, mapping::MQSCO_MAPSTR, r##"Key Reset Count"##
    );
    impl_value!(MQSCO, mqsys::MQLONG);
    define_new_type!(
        pub MQSCYC, mqsys::MQLONG, mapping::MQSCYC_MAPSTR, r##"Security Case"##
    );
    impl_value!(MQSCYC, mqsys::MQLONG);
    define_new_type!(
        pub MQSMPO, mqsys::MQLONG, mapping::MQSMPO_MAPSTR,
        r##"Set Message Property Options"##
    );
    impl_value!(MQSMPO, mqsys::MQLONG);
    define_new_type!(
        pub MQSO, mqsys::MQLONG, mapping::MQSO_MAPSTR, r##"Subscribe Options"##
    );
    impl_bitflags!(MQSO, mqsys::MQLONG);
    define_new_type!(
        pub MQSP, mqsys::MQLONG, mapping::MQSP_MAPSTR, r##"Syncpoint Availability"##
    );
    impl_value!(MQSP, mqsys::MQLONG);
    define_new_type!(
        pub MQSQQM, mqsys::MQLONG, mapping::MQSQQM_MAPSTR,
        r##"Shared Queue Queue Manager Name"##
    );
    impl_value!(MQSQQM, mqsys::MQLONG);
    define_new_type!(
        pub MQSRO, mqsys::MQLONG, mapping::MQSRO_MAPSTR,
        r##"Subscription Request Options"##
    );
    impl_bitflags!(MQSRO, mqsys::MQLONG);
    define_new_type!(
        pub MQSR, mqsys::MQLONG, mapping::MQSR_MAPSTR, r##"Subscription Request Action"##
    );
    impl_value!(MQSR, mqsys::MQLONG);
    define_new_type!(
        pub MQSSL, mqsys::MQLONG, mapping::MQSSL_MAPSTR, r##"SSL FIPS Requirements"##
    );
    impl_value!(MQSSL, mqsys::MQLONG);
    define_new_type!(
        pub MQSTAT, mqsys::MQLONG, mapping::MQSTAT_MAPSTR, r##"Stat Options"##
    );
    impl_value!(MQSTAT, mqsys::MQLONG);
    define_new_type!(
        pub MQST, mqsys::MQLONG, mapping::MQST_MAPSTR,
        r##"Streaming Queue Quality of Service Values"##
    );
    impl_value!(MQST, mqsys::MQLONG);
    define_new_type!(
        pub MQSUB, mqsys::MQLONG, mapping::MQSUB_MAPSTR, r##"Durable subscriptions"##
    );
    impl_value!(MQSUB, mqsys::MQLONG);
    define_new_type!(
        pub MQSUB_DURABILITY, mqsys::MQLONG, mapping::MQSUB_DURABILITY_MAPSTR,
        r##"Durable Subscriptions"##
    );
    impl_value!(MQSUB_DURABILITY, mqsys::MQLONG);
    define_new_type!(
        pub MQSVC_CONTROL, mqsys::MQLONG, mapping::MQSVC_CONTROL_MAPSTR,
        r##"Service Controls"##
    );
    impl_value!(MQSVC_CONTROL, mqsys::MQLONG);
    define_new_type!(
        pub MQSVC_STATUS, mqsys::MQLONG, mapping::MQSVC_STATUS_MAPSTR,
        r##"Service Status"##
    );
    impl_value!(MQSVC_STATUS, mqsys::MQLONG);
    define_new_type!(
        pub MQSVC_TYPE, mqsys::MQLONG, mapping::MQSVC_TYPE_MAPSTR, r##"Service Types"##
    );
    impl_value!(MQSVC_TYPE, mqsys::MQLONG);
    define_new_type!(
        pub MQTA, mqsys::MQLONG, mapping::MQTA_MAPSTR, r##"Topic Attributes"##
    );
    impl_value!(MQTA, mqsys::MQLONG);
    define_new_type!(
        pub MQTA_PROXY, mqsys::MQLONG, mapping::MQTA_PROXY_MAPSTR,
        r##"Proxy Sub Propagation"##
    );
    impl_value!(MQTA_PROXY, mqsys::MQLONG);
    define_new_type!(
        pub MQTA_PUB, mqsys::MQLONG, mapping::MQTA_PUB_MAPSTR,
        r##"Publications Allowed"##
    );
    impl_value!(MQTA_PUB, mqsys::MQLONG);
    define_new_type!(
        pub MQTA_SUB, mqsys::MQLONG, mapping::MQTA_SUB_MAPSTR,
        r##"Subscriptions Allowed"##
    );
    impl_value!(MQTA_SUB, mqsys::MQLONG);
    define_new_type!(
        pub MQTCPKEEP, mqsys::MQLONG, mapping::MQTCPKEEP_MAPSTR, r##"TCP Keepalive"##
    );
    impl_value!(MQTCPKEEP, mqsys::MQLONG);
    define_new_type!(
        pub MQTCPSTACK, mqsys::MQLONG, mapping::MQTCPSTACK_MAPSTR, r##"TCP Stack Types"##
    );
    impl_value!(MQTCPSTACK, mqsys::MQLONG);
    define_new_type!(
        pub MQTC, mqsys::MQLONG, mapping::MQTC_MAPSTR, r##"Trigger Controls"##
    );
    impl_value!(MQTC, mqsys::MQLONG);
    define_new_type!(
        pub MQTOPT, mqsys::MQLONG, mapping::MQTOPT_MAPSTR, r##"Topic Type"##
    );
    impl_value!(MQTOPT, mqsys::MQLONG);
    define_new_type!(
        pub MQTRAXSTR, mqsys::MQLONG, mapping::MQTRAXSTR_MAPSTR,
        r##"Channel Initiator Trace Autostart"##
    );
    impl_value!(MQTRAXSTR, mqsys::MQLONG);
    define_new_type!(
        pub MQTRIGGER, mqsys::MQLONG, mapping::MQTRIGGER_MAPSTR, r##"Trigger Restart"##
    );
    impl_value!(MQTRIGGER, mqsys::MQLONG);
    define_new_type!(
        pub MQTSCOPE, mqsys::MQLONG, mapping::MQTSCOPE_MAPSTR, r##"Subscription Scope"##
    );
    impl_value!(MQTSCOPE, mqsys::MQLONG);
    define_new_type!(
        pub MQTT, mqsys::MQLONG, mapping::MQTT_MAPSTR, r##"Trigger Types"##
    );
    impl_value!(MQTT, mqsys::MQLONG);
    define_new_type!(
        pub MQTYPE, mqsys::MQLONG, mapping::MQTYPE_MAPSTR, r##"Property data types"##
    );
    impl_value!(MQTYPE, mqsys::MQLONG);
    define_new_type!(
        pub MQUSEDLQ, mqsys::MQLONG, mapping::MQUSEDLQ_MAPSTR,
        r##"Use Dead Letter Queue Options"##
    );
    impl_value!(MQUSEDLQ, mqsys::MQLONG);
    define_new_type!(
        pub MQUSRC, mqsys::MQLONG, mapping::MQUSRC_MAPSTR, r##"User Source Options"##
    );
    impl_value!(MQUSRC, mqsys::MQLONG);
    define_new_type!(pub MQUS, mqsys::MQLONG, mapping::MQUS_MAPSTR, r##"Queue Usages"##);
    impl_value!(MQUS, mqsys::MQLONG);
    define_new_type!(
        pub MQVL, mqsys::MQLONG, mapping::MQVL_MAPSTR, r##"Property value lengths"##
    );
    impl_value!(MQVL, mqsys::MQLONG);
    define_new_type!(
        pub MQVS, mqsys::MQLONG, mapping::MQVS_MAPSTR, r##"Variable String Length"##
    );
    impl_value!(MQVS, mqsys::MQLONG);
    define_new_type!(
        pub MQVU, mqsys::MQLONG, mapping::MQVU_MAPSTR, r##"Variable User ID"##
    );
    impl_value!(MQVU, mqsys::MQLONG);
    define_new_type!(
        pub MQWARN, mqsys::MQLONG, mapping::MQWARN_MAPSTR, r##"Warn Options"##
    );
    impl_value!(MQWARN, mqsys::MQLONG);
    define_new_type!(
        pub MQWIH, mqsys::MQLONG, mapping::MQWIH_MAPSTR, r##"MQWIH Flags"##
    );
    impl_value!(MQWIH, mqsys::MQLONG);
    define_new_type!(
        pub MQWI, mqsys::MQLONG, mapping::MQWI_MAPSTR, r##"Wait Interval"##
    );
    impl_value!(MQWI, mqsys::MQLONG);
    define_new_type!(
        pub MQWS, mqsys::MQLONG, mapping::MQWS_MAPSTR, r##"Wildcard Schema"##
    );
    impl_value!(MQWS, mqsys::MQLONG);
    define_new_type!(
        pub MQXC, mqsys::MQLONG, mapping::MQXC_MAPSTR, r##"Exit Commands"##
    );
    impl_value!(MQXC, mqsys::MQLONG);
    define_new_type!(
        pub MQXPT, mqsys::MQLONG, mapping::MQXPT_MAPSTR, r##"Transport Types"##
    );
    impl_value!(MQXPT, mqsys::MQLONG);
    define_new_type!(
        pub MQ_CERT, mqsys::MQLONG, mapping::MQ_CERT_MAPSTR,
        r##"Certificate Validation Policy Type"##
    );
    impl_value!(MQ_CERT, mqsys::MQLONG);
    define_new_type!(
        pub MQ_HTTPSCERTREV, mqsys::MQLONG, mapping::MQ_HTTPSCERTREV_MAPSTR,
        r##"HTTPS Certificate Revocation Type"##
    );
    impl_value!(MQ_HTTPSCERTREV, mqsys::MQLONG);
    define_new_type!(
        pub MQ_HTTPSCERTVAL, mqsys::MQLONG, mapping::MQ_HTTPSCERTVAL_MAPSTR,
        r##"HTTPS Certificate Validation Type"##
    );
    impl_value!(MQ_HTTPSCERTVAL, mqsys::MQLONG);
    define_new_type!(
        pub MQ_MQTT, mqsys::MQLONG, mapping::MQ_MQTT_MAPSTR, r##"General Constants"##
    );
    impl_value!(MQ_MQTT, mqsys::MQLONG);
    define_new_type!(
        pub MQ_SUITE, mqsys::MQLONG, mapping::MQ_SUITE_MAPSTR, r##"SuiteB Type"##
    );
    impl_value!(MQ_SUITE, mqsys::MQLONG);
    #[cfg(feature = "exits")]
    mod exits {
        use ::libmqm_sys::lib as mqsys;
        use crate::mapping;
        use crate::value::{define_new_type, impl_value};
        use crate::bitflags::impl_bitflags;
        define_new_type!(
            pub MQCDC, mqsys::MQLONG, mapping::MQCDC_MAPSTR,
            r##"Channel Data Conversion"##
        );
        impl_value!(MQCDC, mqsys::MQLONG);
        define_new_type!(
            pub MQCF, mqsys::MQLONG, mapping::MQCF_MAPSTR, r##"Capability Flags"##
        );
        impl_value!(MQCF, mqsys::MQLONG);
        define_new_type!(
            pub MQCLCT, mqsys::MQLONG, mapping::MQCLCT_MAPSTR, r##"Cluster Cache Types"##
        );
        impl_value!(MQCLCT, mqsys::MQLONG);
        define_new_type!(
            pub MQCOMPRESS, mqsys::MQLONG, mapping::MQCOMPRESS_MAPSTR,
            r##"Channel Compression"##
        );
        impl_value!(MQCOMPRESS, mqsys::MQLONG);
        define_new_type!(
            pub MQDCC, mqsys::MQLONG, mapping::MQDCC_MAPSTR,
            r##"Conversion Options Masks and Factors"##
        );
        impl_bitflags!(MQDCC, mqsys::MQLONG);
        define_new_type!(
            pub MQIEPF, mqsys::MQLONG, mapping::MQIEPF_MAPSTR, r##"IEP Flags"##
        );
        impl_value!(MQIEPF, mqsys::MQLONG);
        define_new_type!(
            pub MQMCAT, mqsys::MQLONG, mapping::MQMCAT_MAPSTR, r##"MCA Types"##
        );
        impl_value!(MQMCAT, mqsys::MQLONG);
        define_new_type!(
            pub MQMCEV, mqsys::MQLONG, mapping::MQMCEV_MAPSTR, r##"Multicast Events"##
        );
        impl_value!(MQMCEV, mqsys::MQLONG);
        define_new_type!(
            pub MQNPMS, mqsys::MQLONG, mapping::MQNPMS_MAPSTR,
            r##"NonPersistent-Message Speeds"##
        );
        impl_value!(MQNPMS, mqsys::MQLONG);
        define_new_type!(
            pub MQPA, mqsys::MQLONG, mapping::MQPA_MAPSTR, r##"Put Authority"##
        );
        impl_value!(MQPA, mqsys::MQLONG);
        define_new_type!(
            pub MQPROTO, mqsys::MQLONG, mapping::MQPROTO_MAPSTR, r##"Protocol"##
        );
        impl_value!(MQPROTO, mqsys::MQLONG);
        define_new_type!(
            pub MQQF, mqsys::MQLONG, mapping::MQQF_MAPSTR, r##"Queue Flags"##
        );
        impl_value!(MQQF, mqsys::MQLONG);
        define_new_type!(
            pub MQQMF, mqsys::MQLONG, mapping::MQQMF_MAPSTR, r##"Queue Manager Flags"##
        );
        impl_bitflags!(MQQMF, mqsys::MQLONG);
        define_new_type!(
            pub MQSCA, mqsys::MQLONG, mapping::MQSCA_MAPSTR,
            r##"SSL Client Authentication"##
        );
        impl_value!(MQSCA, mqsys::MQLONG);
        define_new_type!(
            pub MQSECPROT, mqsys::MQLONG, mapping::MQSECPROT_MAPSTR,
            r##"Security Protocol"##
        );
        impl_value!(MQSECPROT, mqsys::MQLONG);
        define_new_type!(
            pub MQSPL, mqsys::MQLONG, mapping::MQSPL_MAPSTR, r##"SPL Protection"##
        );
        impl_value!(MQSPL, mqsys::MQLONG);
        define_new_type!(
            pub MQWXP, mqsys::MQLONG, mapping::MQWXP_MAPSTR,
            r##"Cluster Workload Flags"##
        );
        impl_value!(MQWXP, mqsys::MQLONG);
        define_new_type!(
            pub MQXACT, mqsys::MQLONG, mapping::MQXACT_MAPSTR, r##"API Caller Types"##
        );
        impl_value!(MQXACT, mqsys::MQLONG);
        define_new_type!(
            pub MQXCC, mqsys::MQLONG, mapping::MQXCC_MAPSTR, r##"Exit Responses"##
        );
        impl_value!(MQXCC, mqsys::MQLONG);
        define_new_type!(
            pub MQXDR, mqsys::MQLONG, mapping::MQXDR_MAPSTR, r##"Exit Response"##
        );
        impl_value!(MQXDR, mqsys::MQLONG);
        define_new_type!(
            pub MQXEPO, mqsys::MQLONG, mapping::MQXEPO_MAPSTR, r##"Exit Options"##
        );
        impl_value!(MQXEPO, mqsys::MQLONG);
        define_new_type!(
            pub MQXE, mqsys::MQLONG, mapping::MQXE_MAPSTR, r##"Environments"##
        );
        impl_value!(MQXE, mqsys::MQLONG);
        define_new_type!(
            pub MQXF, mqsys::MQLONG, mapping::MQXF_MAPSTR,
            r##"API Function Identifiers"##
        );
        impl_value!(MQXF, mqsys::MQLONG);
        define_new_type!(
            pub MQXR2, mqsys::MQLONG, mapping::MQXR2_MAPSTR, r##"Exit Response 2"##
        );
        impl_value!(MQXR2, mqsys::MQLONG);
        define_new_type!(
            pub MQXR, mqsys::MQLONG, mapping::MQXR_MAPSTR, r##"Exit Reasons"##
        );
        impl_value!(MQXR, mqsys::MQLONG);
        define_new_type!(
            pub MQXT, mqsys::MQLONG, mapping::MQXT_MAPSTR, r##"Exit Identifiers"##
        );
        impl_value!(MQXT, mqsys::MQLONG);
        define_new_type!(
            pub MQZAET, mqsys::MQLONG, mapping::MQZAET_MAPSTR, r##"Entity Types"##
        );
        impl_value!(MQZAET, mqsys::MQLONG);
        define_new_type!(
            pub MQZAO, mqsys::MQLONG, mapping::MQZAO_MAPSTR, r##"Authorizations"##
        );
        impl_bitflags!(MQZAO, mqsys::MQLONG);
        define_new_type!(
            pub MQZAT, mqsys::MQLONG, mapping::MQZAT_MAPSTR, r##"Authentication Types"##
        );
        impl_value!(MQZAT, mqsys::MQLONG);
        define_new_type!(
            pub MQZCI, mqsys::MQLONG, mapping::MQZCI_MAPSTR,
            r##"Continuation Indicator"##
        );
        impl_value!(MQZCI, mqsys::MQLONG);
        define_new_type!(
            pub MQZID, mqsys::MQLONG, mapping::MQZID_MAPSTR,
            r##"Function ids common to all services"##
        );
        impl_value!(MQZID, mqsys::MQLONG);
        define_new_type!(
            pub MQZID_AUTHORITY, mqsys::MQLONG, mapping::MQZID_AUTHORITY_MAPSTR,
            r##"Function ids for Authority service"##
        );
        impl_value!(MQZID_AUTHORITY, mqsys::MQLONG);
        define_new_type!(
            pub MQZID_NAME, mqsys::MQLONG, mapping::MQZID_NAME_MAPSTR,
            r##"Function ids for Name service"##
        );
        impl_value!(MQZID_NAME, mqsys::MQLONG);
        define_new_type!(
            pub MQZID_USERID, mqsys::MQLONG, mapping::MQZID_USERID_MAPSTR,
            r##"Function ids for Userid service"##
        );
        impl_value!(MQZID_USERID, mqsys::MQLONG);
        define_new_type!(
            pub MQZIO, mqsys::MQLONG, mapping::MQZIO_MAPSTR,
            r##"Initialization Options"##
        );
        impl_value!(MQZIO, mqsys::MQLONG);
        define_new_type!(
            pub MQZSE, mqsys::MQLONG, mapping::MQZSE_MAPSTR,
            r##"Start-Enumeration Indicator"##
        );
        impl_value!(MQZSE, mqsys::MQLONG);
        define_new_type!(
            pub MQZSL, mqsys::MQLONG, mapping::MQZSL_MAPSTR, r##"Selector Indicator"##
        );
        impl_value!(MQZSL, mqsys::MQLONG);
        define_new_type!(
            pub MQZTO, mqsys::MQLONG, mapping::MQZTO_MAPSTR, r##"Termination Options"##
        );
        impl_value!(MQZTO, mqsys::MQLONG);
    }
    #[cfg(feature = "exits")]
    pub use exits::*;
    #[cfg(feature = "mqai")]
    mod mqai {
        use ::libmqm_sys::lib as mqsys;
        use crate::mapping;
        use crate::value::{define_new_type, impl_value};
        use crate::bitflags::impl_bitflags;
        define_new_type!(
            pub MQBL, mqsys::MQLONG, mapping::MQBL_MAPSTR,
            r##"Buffer Length for mqAddString and mqSetString"##
        );
        impl_value!(MQBL, mqsys::MQLONG);
        define_new_type!(
            pub MQCBO, mqsys::MQLONG, mapping::MQCBO_MAPSTR,
            r##"Create-Bag Options for mqCreateBag"##
        );
        impl_bitflags!(MQCBO, mqsys::MQLONG);
        define_new_type!(
            pub MQHA, mqsys::MQLONG, mapping::MQHA_MAPSTR, r##"Handle Selectors"##
        );
        impl_value!(MQHA, mqsys::MQLONG);
        define_new_type!(
            pub MQHB, mqsys::MQLONG, mapping::MQHB_MAPSTR, r##"Bag Handles"##
        );
        impl_value!(MQHB, mqsys::MQLONG);
        define_new_type!(
            pub MQIASY, mqsys::MQLONG, mapping::MQIASY_MAPSTR,
            r##"Integer System Selectors"##
        );
        impl_value!(MQIASY, mqsys::MQLONG);
        define_new_type!(
            pub MQIND, mqsys::MQLONG, mapping::MQIND_MAPSTR, r##"Special Index Values"##
        );
        impl_value!(MQIND, mqsys::MQLONG);
        define_new_type!(
            pub MQITEM, mqsys::MQLONG, mapping::MQITEM_MAPSTR,
            r##"Item Types for mqInquireItemInfo"##
        );
        impl_value!(MQITEM, mqsys::MQLONG);
        define_new_type!(
            pub MQSEL_ALL, mqsys::MQLONG, mapping::MQSEL_ALL_MAPSTR,
            r##"Special Selector Values"##
        );
        impl_value!(MQSEL_ALL, mqsys::MQLONG);
        define_new_type!(
            pub MQSEL_ANY, mqsys::MQLONG, mapping::MQSEL_ANY_MAPSTR,
            r##"Special Selector Values"##
        );
        impl_value!(MQSEL_ANY, mqsys::MQLONG);
    }
    #[cfg(feature = "mqai")]
    pub use mqai::*;
    #[cfg(feature = "pcf")]
    mod pcf {
        use ::libmqm_sys::lib as mqsys;
        use crate::mapping;
        use crate::value::{define_new_type, impl_value};
        use crate::bitflags::impl_bitflags;
        define_new_type!(
            pub MQACTIVE, mqsys::MQLONG, mapping::MQACTIVE_MAPSTR, r##"Active Options"##
        );
        impl_value!(MQACTIVE, mqsys::MQLONG);
        define_new_type!(
            pub MQACT, mqsys::MQLONG, mapping::MQACT_MAPSTR, r##"Action Options"##
        );
        impl_value!(MQACT, mqsys::MQLONG);
        define_new_type!(
            pub MQADPCTX, mqsys::MQLONG, mapping::MQADPCTX_MAPSTR,
            r##"Authentication Adoption Context"##
        );
        impl_value!(MQADPCTX, mqsys::MQLONG);
        define_new_type!(
            pub MQAPPL, mqsys::MQLONG, mapping::MQAPPL_MAPSTR, r##"Movable Options"##
        );
        impl_value!(MQAPPL, mqsys::MQLONG);
        define_new_type!(
            pub MQAS, mqsys::MQLONG, mapping::MQAS_MAPSTR,
            r##"Asynchronous State Values"##
        );
        impl_value!(MQAS, mqsys::MQLONG);
        define_new_type!(
            pub MQAUTHENTICATE, mqsys::MQLONG, mapping::MQAUTHENTICATE_MAPSTR,
            r##"Authentication Method"##
        );
        impl_value!(MQAUTHENTICATE, mqsys::MQLONG);
        define_new_type!(
            pub MQAUTHOPT, mqsys::MQLONG, mapping::MQAUTHOPT_MAPSTR,
            r##"Authority Options"##
        );
        impl_bitflags!(MQAUTHOPT, mqsys::MQLONG);
        define_new_type!(
            pub MQAUTH, mqsys::MQLONG, mapping::MQAUTH_MAPSTR, r##"Authority Values"##
        );
        impl_value!(MQAUTH, mqsys::MQLONG);
        define_new_type!(
            pub MQAUTOCLUS, mqsys::MQLONG, mapping::MQAUTOCLUS_MAPSTR,
            r##"Automatic Cluster Types"##
        );
        impl_value!(MQAUTOCLUS, mqsys::MQLONG);
        define_new_type!(
            pub MQBACF, mqsys::MQLONG, mapping::MQBACF_MAPSTR,
            r##"Byte Parameter Types"##
        );
        impl_value!(MQBACF, mqsys::MQLONG);
        define_new_type!(
            pub MQBALANCED, mqsys::MQLONG, mapping::MQBALANCED_MAPSTR,
            r##"Balance Options"##
        );
        impl_value!(MQBALANCED, mqsys::MQLONG);
        define_new_type!(
            pub MQBALSTATE, mqsys::MQLONG, mapping::MQBALSTATE_MAPSTR,
            r##"Balance State"##
        );
        impl_value!(MQBALSTATE, mqsys::MQLONG);
        define_new_type!(
            pub MQBPLOCATION, mqsys::MQLONG, mapping::MQBPLOCATION_MAPSTR,
            r##"Values for MQIACF_BUFFER_POOL_LOCATION."##
        );
        impl_value!(MQBPLOCATION, mqsys::MQLONG);
        define_new_type!(
            pub MQBT, mqsys::MQLONG, mapping::MQBT_MAPSTR, r##"Bridge Types"##
        );
        impl_value!(MQBT, mqsys::MQLONG);
        define_new_type!(
            pub MQCACF, mqsys::MQLONG, mapping::MQCACF_MAPSTR,
            r##"Character Parameter Types"##
        );
        impl_value!(MQCACF, mqsys::MQLONG);
        define_new_type!(
            pub MQCACH, mqsys::MQLONG, mapping::MQCACH_MAPSTR,
            r##"Character Channel Parameter Types"##
        );
        impl_value!(MQCACH, mqsys::MQLONG);
        define_new_type!(
            pub MQCAMO, mqsys::MQLONG, mapping::MQCAMO_MAPSTR,
            r##"Character Monitoring Parameter Types"##
        );
        impl_value!(MQCAMO, mqsys::MQLONG);
        define_new_type!(
            pub MQCAUT, mqsys::MQLONG, mapping::MQCAUT_MAPSTR, r##"CHLAUTH Type"##
        );
        impl_value!(MQCAUT, mqsys::MQLONG);
        define_new_type!(
            pub MQCFACCESS, mqsys::MQLONG, mapping::MQCFACCESS_MAPSTR,
            r##"Access Options"##
        );
        impl_value!(MQCFACCESS, mqsys::MQLONG);
        define_new_type!(
            pub MQCFC, mqsys::MQLONG, mapping::MQCFC_MAPSTR, r##"Control Options"##
        );
        impl_value!(MQCFC, mqsys::MQLONG);
        define_new_type!(
            pub MQCFOP, mqsys::MQLONG, mapping::MQCFOP_MAPSTR, r##"Filter Operators"##
        );
        impl_value!(MQCFOP, mqsys::MQLONG);
        define_new_type!(
            pub MQCFO_REFRESH, mqsys::MQLONG, mapping::MQCFO_REFRESH_MAPSTR,
            r##"Refresh Repository Options"##
        );
        impl_value!(MQCFO_REFRESH, mqsys::MQLONG);
        define_new_type!(
            pub MQCFO_REMOVE, mqsys::MQLONG, mapping::MQCFO_REMOVE_MAPSTR,
            r##"Remove Queues Options"##
        );
        impl_value!(MQCFO_REMOVE, mqsys::MQLONG);
        define_new_type!(
            pub MQCFSTATUS, mqsys::MQLONG, mapping::MQCFSTATUS_MAPSTR, r##"CF Status"##
        );
        impl_value!(MQCFSTATUS, mqsys::MQLONG);
        define_new_type!(
            pub MQCFTYPE, mqsys::MQLONG, mapping::MQCFTYPE_MAPSTR, r##"CF Types"##
        );
        impl_value!(MQCFTYPE, mqsys::MQLONG);
        define_new_type!(
            pub MQCFT, mqsys::MQLONG, mapping::MQCFT_MAPSTR, r##"Types of Structure"##
        );
        impl_value!(MQCFT, mqsys::MQLONG);
        define_new_type!(
            pub MQCHIDS, mqsys::MQLONG, mapping::MQCHIDS_MAPSTR, r##"Indoubt Status"##
        );
        impl_value!(MQCHIDS, mqsys::MQLONG);
        define_new_type!(
            pub MQCHK, mqsys::MQLONG, mapping::MQCHK_MAPSTR,
            r##"Authentication Validation Types"##
        );
        impl_value!(MQCHK, mqsys::MQLONG);
        define_new_type!(
            pub MQCHLA, mqsys::MQLONG, mapping::MQCHLA_MAPSTR, r##"CHLAUTH QMGR State"##
        );
        impl_value!(MQCHLA, mqsys::MQLONG);
        define_new_type!(
            pub MQCHLD, mqsys::MQLONG, mapping::MQCHLD_MAPSTR,
            r##"Channel Dispositions"##
        );
        impl_value!(MQCHLD, mqsys::MQLONG);
        define_new_type!(
            pub MQCHRR, mqsys::MQLONG, mapping::MQCHRR_MAPSTR,
            r##"Channel reset requested"##
        );
        impl_value!(MQCHRR, mqsys::MQLONG);
        define_new_type!(
            pub MQCHSH, mqsys::MQLONG, mapping::MQCHSH_MAPSTR,
            r##"Channel Shared Restart Options"##
        );
        impl_value!(MQCHSH, mqsys::MQLONG);
        define_new_type!(
            pub MQCHSR, mqsys::MQLONG, mapping::MQCHSR_MAPSTR,
            r##"Channel Stop Options"##
        );
        impl_value!(MQCHSR, mqsys::MQLONG);
        define_new_type!(
            pub MQCHSSTATE, mqsys::MQLONG, mapping::MQCHSSTATE_MAPSTR,
            r##"Channel Substates"##
        );
        impl_value!(MQCHSSTATE, mqsys::MQLONG);
        define_new_type!(
            pub MQCHS, mqsys::MQLONG, mapping::MQCHS_MAPSTR, r##"Channel Status"##
        );
        impl_value!(MQCHS, mqsys::MQLONG);
        define_new_type!(
            pub MQCHTAB, mqsys::MQLONG, mapping::MQCHTAB_MAPSTR,
            r##"Channel Table Types"##
        );
        impl_value!(MQCHTAB, mqsys::MQLONG);
        define_new_type!(
            pub MQCLROUTE, mqsys::MQLONG, mapping::MQCLROUTE_MAPSTR,
            r##"CLROUTE Topic State"##
        );
        impl_value!(MQCLROUTE, mqsys::MQLONG);
        define_new_type!(
            pub MQCLRS, mqsys::MQLONG, mapping::MQCLRS_MAPSTR,
            r##"Clear Topic String Scope"##
        );
        impl_value!(MQCLRS, mqsys::MQLONG);
        define_new_type!(
            pub MQCLRT, mqsys::MQLONG, mapping::MQCLRT_MAPSTR,
            r##"Clear Topic String Type"##
        );
        impl_value!(MQCLRT, mqsys::MQLONG);
        define_new_type!(
            pub MQCLST, mqsys::MQLONG, mapping::MQCLST_MAPSTR,
            r##"CLSTATE Clustered Topic Definition State"##
        );
        impl_value!(MQCLST, mqsys::MQLONG);
        define_new_type!(
            pub MQCLXQ, mqsys::MQLONG, mapping::MQCLXQ_MAPSTR,
            r##"Transmission queue types"##
        );
        impl_value!(MQCLXQ, mqsys::MQLONG);
        define_new_type!(
            pub MQCMDI, mqsys::MQLONG, mapping::MQCMDI_MAPSTR,
            r##"Command Information Values"##
        );
        impl_value!(MQCMDI, mqsys::MQLONG);
        define_new_type!(
            pub MQCMD, mqsys::MQLONG, mapping::MQCMD_MAPSTR, r##"Command Codes"##
        );
        impl_value!(MQCMD, mqsys::MQLONG);
        define_new_type!(
            pub MQDELO, mqsys::MQLONG, mapping::MQDELO_MAPSTR, r##"Delete Options"##
        );
        impl_value!(MQDELO, mqsys::MQLONG);
        define_new_type!(
            pub MQDISCONNECT, mqsys::MQLONG, mapping::MQDISCONNECT_MAPSTR,
            r##"Disconnect Types"##
        );
        impl_value!(MQDISCONNECT, mqsys::MQLONG);
        define_new_type!(
            pub MQDOPT, mqsys::MQLONG, mapping::MQDOPT_MAPSTR,
            r##"Display Subscription Types"##
        );
        impl_value!(MQDOPT, mqsys::MQLONG);
        define_new_type!(
            pub MQEPH, mqsys::MQLONG, mapping::MQEPH_MAPSTR, r##"MQEPH Flags"##
        );
        impl_value!(MQEPH, mqsys::MQLONG);
        define_new_type!(
            pub MQET, mqsys::MQLONG, mapping::MQET_MAPSTR, r##"Escape Types"##
        );
        impl_value!(MQET, mqsys::MQLONG);
        define_new_type!(
            pub MQEVO, mqsys::MQLONG, mapping::MQEVO_MAPSTR, r##"Event Origins"##
        );
        impl_value!(MQEVO, mqsys::MQLONG);
        define_new_type!(
            pub MQEVR, mqsys::MQLONG, mapping::MQEVR_MAPSTR, r##"Event Recording"##
        );
        impl_value!(MQEVR, mqsys::MQLONG);
        define_new_type!(
            pub MQEXTATTRS, mqsys::MQLONG, mapping::MQEXTATTRS_MAPSTR,
            r##"Export Attrs"##
        );
        impl_value!(MQEXTATTRS, mqsys::MQLONG);
        define_new_type!(
            pub MQEXT, mqsys::MQLONG, mapping::MQEXT_MAPSTR, r##"Export Type"##
        );
        impl_value!(MQEXT, mqsys::MQLONG);
        define_new_type!(
            pub MQFC, mqsys::MQLONG, mapping::MQFC_MAPSTR, r##"Force Options"##
        );
        impl_value!(MQFC, mqsys::MQLONG);
        define_new_type!(
            pub MQFSENC, mqsys::MQLONG, mapping::MQFSENC_MAPSTR,
            r##"File System Encryption Values"##
        );
        impl_value!(MQFSENC, mqsys::MQLONG);
        define_new_type!(
            pub MQFS, mqsys::MQLONG, mapping::MQFS_MAPSTR,
            r##"File System Sharing Values"##
        );
        impl_value!(MQFS, mqsys::MQLONG);
        define_new_type!(
            pub MQGACF, mqsys::MQLONG, mapping::MQGACF_MAPSTR,
            r##"Group Parameter Types"##
        );
        impl_value!(MQGACF, mqsys::MQLONG);
        define_new_type!(
            pub MQGUR, mqsys::MQLONG, mapping::MQGUR_MAPSTR,
            r##"Grouped Units of Recovery"##
        );
        impl_value!(MQGUR, mqsys::MQLONG);
        define_new_type!(
            pub MQHSTATE, mqsys::MQLONG, mapping::MQHSTATE_MAPSTR, r##"Handle States"##
        );
        impl_value!(MQHSTATE, mqsys::MQLONG);
        define_new_type!(
            pub MQIACF, mqsys::MQLONG, mapping::MQIACF_MAPSTR,
            r##"Integer Parameter Types"##
        );
        impl_value!(MQIACF, mqsys::MQLONG);
        define_new_type!(
            pub MQIACH, mqsys::MQLONG, mapping::MQIACH_MAPSTR,
            r##"Integer Channel Types"##
        );
        impl_value!(MQIACH, mqsys::MQLONG);
        define_new_type!(
            pub MQIAMO64, mqsys::MQLONG, mapping::MQIAMO64_MAPSTR,
            r##"Integer Monitoring Parameter Types"##
        );
        impl_value!(MQIAMO64, mqsys::MQLONG);
        define_new_type!(
            pub MQIAMO, mqsys::MQLONG, mapping::MQIAMO_MAPSTR,
            r##"Integer Monitoring Parameter Types"##
        );
        impl_value!(MQIAMO, mqsys::MQLONG);
        define_new_type!(
            pub MQIAMO_MONITOR_DATATYPE, mqsys::MQLONG,
            mapping::MQIAMO_MONITOR_DATATYPE_MAPSTR,
            r##"Defined values for MQIAMO_MONITOR_DATATYPE"##
        );
        impl_value!(MQIAMO_MONITOR_DATATYPE, mqsys::MQLONG);
        define_new_type!(
            pub MQIAMO_MONITOR_FLAGS, mqsys::MQLONG,
            mapping::MQIAMO_MONITOR_FLAGS_MAPSTR,
            r##"Defined values for MQIAMO_MONITOR_FLAGS"##
        );
        impl_value!(MQIAMO_MONITOR_FLAGS, mqsys::MQLONG);
        define_new_type!(
            pub MQIDO, mqsys::MQLONG, mapping::MQIDO_MAPSTR, r##"Indoubt Options"##
        );
        impl_value!(MQIDO, mqsys::MQLONG);
        define_new_type!(
            pub MQIMMREASON, mqsys::MQLONG, mapping::MQIMMREASON_MAPSTR,
            r##"Immovable Reasons"##
        );
        impl_value!(MQIMMREASON, mqsys::MQLONG);
        define_new_type!(
            pub MQINBD, mqsys::MQLONG, mapping::MQINBD_MAPSTR,
            r##"Inbound Dispositions"##
        );
        impl_value!(MQINBD, mqsys::MQLONG);
        define_new_type!(
            pub MQIS, mqsys::MQLONG, mapping::MQIS_MAPSTR, r##"State Options"##
        );
        impl_value!(MQIS, mqsys::MQLONG);
        define_new_type!(
            pub MQLDAPC, mqsys::MQLONG, mapping::MQLDAPC_MAPSTR,
            r##"QMgr LDAP Connection Status"##
        );
        impl_value!(MQLDAPC, mqsys::MQLONG);
        define_new_type!(
            pub MQLDAP_AUTHORMD, mqsys::MQLONG, mapping::MQLDAP_AUTHORMD_MAPSTR,
            r##"LDAP Authorisation Method"##
        );
        impl_value!(MQLDAP_AUTHORMD, mqsys::MQLONG);
        define_new_type!(
            pub MQLDAP_NESTGRP, mqsys::MQLONG, mapping::MQLDAP_NESTGRP_MAPSTR,
            r##"LDAP Nested Group Policy"##
        );
        impl_value!(MQLDAP_NESTGRP, mqsys::MQLONG);
        define_new_type!(
            pub MQLOGTYPE, mqsys::MQLONG, mapping::MQLOGTYPE_MAPSTR, r##"Log Types"##
        );
        impl_value!(MQLOGTYPE, mqsys::MQLONG);
        define_new_type!(
            pub MQLR, mqsys::MQLONG, mapping::MQLR_MAPSTR, r##"Reduce Log Options"##
        );
        impl_value!(MQLR, mqsys::MQLONG);
        define_new_type!(
            pub MQMATCH, mqsys::MQLONG, mapping::MQMATCH_MAPSTR, r##"Match Types"##
        );
        impl_value!(MQMATCH, mqsys::MQLONG);
        define_new_type!(
            pub MQMCAS, mqsys::MQLONG, mapping::MQMCAS_MAPSTR,
            r##"Message Channel Agent Status"##
        );
        impl_value!(MQMCAS, mqsys::MQLONG);
        define_new_type!(
            pub MQMCP, mqsys::MQLONG, mapping::MQMCP_MAPSTR,
            r##"Multicast Properties Options"##
        );
        impl_value!(MQMCP, mqsys::MQLONG);
        define_new_type!(
            pub MQMLP_ENCRYPTION, mqsys::MQLONG, mapping::MQMLP_ENCRYPTION_MAPSTR,
            r##"Message Level Protection"##
        );
        impl_value!(MQMLP_ENCRYPTION, mqsys::MQLONG);
        define_new_type!(
            pub MQMLP_SIGN, mqsys::MQLONG, mapping::MQMLP_SIGN_MAPSTR,
            r##"Message Level Protection"##
        );
        impl_value!(MQMLP_SIGN, mqsys::MQLONG);
        define_new_type!(
            pub MQMLP_TOLERATE, mqsys::MQLONG, mapping::MQMLP_TOLERATE_MAPSTR,
            r##"Message Level Protection"##
        );
        impl_value!(MQMLP_TOLERATE, mqsys::MQLONG);
        define_new_type!(
            pub MQMODE, mqsys::MQLONG, mapping::MQMODE_MAPSTR, r##"Mode Options"##
        );
        impl_value!(MQMODE, mqsys::MQLONG);
        define_new_type!(
            pub MQMULC, mqsys::MQLONG, mapping::MQMULC_MAPSTR,
            r##"Measured usage by API"##
        );
        impl_value!(MQMULC, mqsys::MQLONG);
        define_new_type!(
            pub MQNHABACKLOG, mqsys::MQLONG, mapping::MQNHABACKLOG_MAPSTR,
            r##"Native HA Backlog Values"##
        );
        impl_value!(MQNHABACKLOG, mqsys::MQLONG);
        define_new_type!(
            pub MQNHACONNACTV, mqsys::MQLONG, mapping::MQNHACONNACTV_MAPSTR,
            r##"Native HA Active Connection Values"##
        );
        impl_value!(MQNHACONNACTV, mqsys::MQLONG);
        define_new_type!(
            pub MQNHACONNGRP, mqsys::MQLONG, mapping::MQNHACONNGRP_MAPSTR,
            r##"Native HA Group Connected Values"##
        );
        impl_value!(MQNHACONNGRP, mqsys::MQLONG);
        define_new_type!(
            pub MQNHAGRPROLE, mqsys::MQLONG, mapping::MQNHAGRPROLE_MAPSTR,
            r##"Native HA Group Roles"##
        );
        impl_value!(MQNHAGRPROLE, mqsys::MQLONG);
        define_new_type!(
            pub MQNHAINSYNC, mqsys::MQLONG, mapping::MQNHAINSYNC_MAPSTR,
            r##"Native HA In Sync Values"##
        );
        impl_value!(MQNHAINSYNC, mqsys::MQLONG);
        define_new_type!(
            pub MQNHAROLE, mqsys::MQLONG, mapping::MQNHAROLE_MAPSTR,
            r##"Native HA Instance Roles"##
        );
        impl_value!(MQNHAROLE, mqsys::MQLONG);
        define_new_type!(
            pub MQNHASTATUS, mqsys::MQLONG, mapping::MQNHASTATUS_MAPSTR,
            r##"Native HA Status Values"##
        );
        impl_value!(MQNHASTATUS, mqsys::MQLONG);
        define_new_type!(
            pub MQNHATYPE, mqsys::MQLONG, mapping::MQNHATYPE_MAPSTR,
            r##"Native HA Types"##
        );
        impl_value!(MQNHATYPE, mqsys::MQLONG);
        define_new_type!(
            pub MQNSH, mqsys::MQLONG, mapping::MQNSH_MAPSTR,
            r##"Multicast New Subscriber History Options"##
        );
        impl_value!(MQNSH, mqsys::MQLONG);
        define_new_type!(
            pub MQOPER, mqsys::MQLONG, mapping::MQOPER_MAPSTR, r##"Activity Operations"##
        );
        impl_value!(MQOPER, mqsys::MQLONG);
        define_new_type!(
            pub MQOPMODE, mqsys::MQLONG, mapping::MQOPMODE_MAPSTR,
            r##"Major Release Function"##
        );
        impl_value!(MQOPMODE, mqsys::MQLONG);
        define_new_type!(
            pub MQPAGECLAS, mqsys::MQLONG, mapping::MQPAGECLAS_MAPSTR,
            r##"Values for MQIACF_PAGECLAS."##
        );
        impl_value!(MQPAGECLAS, mqsys::MQLONG);
        define_new_type!(
            pub MQPO, mqsys::MQLONG, mapping::MQPO_MAPSTR, r##"Purge Options"##
        );
        impl_value!(MQPO, mqsys::MQLONG);
        define_new_type!(
            pub MQPSCT, mqsys::MQLONG, mapping::MQPSCT_MAPSTR,
            r##"Pub/Sub Status Counts"##
        );
        impl_value!(MQPSCT, mqsys::MQLONG);
        define_new_type!(
            pub MQPSST, mqsys::MQLONG, mapping::MQPSST_MAPSTR, r##"Pub/Sub Status Type"##
        );
        impl_value!(MQPSST, mqsys::MQLONG);
        define_new_type!(
            pub MQPS, mqsys::MQLONG, mapping::MQPS_MAPSTR, r##"Pub/Sub Status"##
        );
        impl_value!(MQPS, mqsys::MQLONG);
        define_new_type!(
            pub MQPUBO, mqsys::MQLONG, mapping::MQPUBO_MAPSTR, r##"Publication Options"##
        );
        impl_bitflags!(MQPUBO, mqsys::MQLONG);
        define_new_type!(
            pub MQQMDT, mqsys::MQLONG, mapping::MQQMDT_MAPSTR,
            r##"Queue Manager Definition Types"##
        );
        impl_value!(MQQMDT, mqsys::MQLONG);
        define_new_type!(
            pub MQQMFAC, mqsys::MQLONG, mapping::MQQMFAC_MAPSTR,
            r##"Queue Manager Facility"##
        );
        impl_value!(MQQMFAC, mqsys::MQLONG);
        define_new_type!(
            pub MQQMSTA, mqsys::MQLONG, mapping::MQQMSTA_MAPSTR,
            r##"Queue Manager Status"##
        );
        impl_value!(MQQMSTA, mqsys::MQLONG);
        define_new_type!(
            pub MQQMT, mqsys::MQLONG, mapping::MQQMT_MAPSTR, r##"Queue Manager Types"##
        );
        impl_value!(MQQMT, mqsys::MQLONG);
        define_new_type!(
            pub MQQO, mqsys::MQLONG, mapping::MQQO_MAPSTR, r##"Quiesce Options"##
        );
        impl_value!(MQQO, mqsys::MQLONG);
        define_new_type!(
            pub MQQSGS, mqsys::MQLONG, mapping::MQQSGS_MAPSTR, r##"QSG Status"##
        );
        impl_value!(MQQSGS, mqsys::MQLONG);
        define_new_type!(
            pub MQQSIE, mqsys::MQLONG, mapping::MQQSIE_MAPSTR,
            r##"Queue Service-Interval Events"##
        );
        impl_value!(MQQSIE, mqsys::MQLONG);
        define_new_type!(
            pub MQQSOT, mqsys::MQLONG, mapping::MQQSOT_MAPSTR,
            r##"Queue Status Open Types"##
        );
        impl_value!(MQQSOT, mqsys::MQLONG);
        define_new_type!(
            pub MQQSO, mqsys::MQLONG, mapping::MQQSO_MAPSTR,
            r##"Queue Status Open Options for SET, BROWSE, INPUT"##
        );
        impl_value!(MQQSO, mqsys::MQLONG);
        define_new_type!(
            pub MQQSUM, mqsys::MQLONG, mapping::MQQSUM_MAPSTR,
            r##"Queue Status Uncommitted Messages"##
        );
        impl_value!(MQQSUM, mqsys::MQLONG);
        define_new_type!(
            pub MQRAR, mqsys::MQLONG, mapping::MQRAR_MAPSTR,
            r##"Remove Authority Record Options"##
        );
        impl_value!(MQRAR, mqsys::MQLONG);
        define_new_type!(
            pub MQRCCF, mqsys::MQLONG, mapping::MQRCCF_MAPSTR, r##"Reason Codes"##
        );
        impl_value!(MQRCCF, mqsys::MQLONG);
        define_new_type!(
            pub MQRDNS, mqsys::MQLONG, mapping::MQRDNS_MAPSTR, r##"REVDNS QMGR State"##
        );
        impl_value!(MQRDNS, mqsys::MQLONG);
        define_new_type!(
            pub MQREGO, mqsys::MQLONG, mapping::MQREGO_MAPSTR,
            r##"Registration Options"##
        );
        impl_bitflags!(MQREGO, mqsys::MQLONG);
        define_new_type!(
            pub MQROUTE, mqsys::MQLONG, mapping::MQROUTE_MAPSTR,
            r##"Trace-route Accumulation (MQIACF_ROUTE_ACCUMULATION)"##
        );
        impl_value!(MQROUTE, mqsys::MQLONG);
        define_new_type!(
            pub MQRP, mqsys::MQLONG, mapping::MQRP_MAPSTR, r##"Replace Options"##
        );
        impl_value!(MQRP, mqsys::MQLONG);
        define_new_type!(
            pub MQRQ, mqsys::MQLONG, mapping::MQRQ_MAPSTR, r##"Reason Qualifiers"##
        );
        impl_value!(MQRQ, mqsys::MQLONG);
        define_new_type!(
            pub MQRT, mqsys::MQLONG, mapping::MQRT_MAPSTR, r##"Refresh Types"##
        );
        impl_value!(MQRT, mqsys::MQLONG);
        define_new_type!(
            pub MQSECCOMM, mqsys::MQLONG, mapping::MQSECCOMM_MAPSTR,
            r##"LDAP SSL/TLS Connection State"##
        );
        impl_value!(MQSECCOMM, mqsys::MQLONG);
        define_new_type!(
            pub MQSECITEM, mqsys::MQLONG, mapping::MQSECITEM_MAPSTR,
            r##"Security Items"##
        );
        impl_value!(MQSECITEM, mqsys::MQLONG);
        define_new_type!(
            pub MQSECSW, mqsys::MQLONG, mapping::MQSECSW_MAPSTR,
            r##"Security Switch States"##
        );
        impl_value!(MQSECSW, mqsys::MQLONG);
        define_new_type!(
            pub MQSECTYPE, mqsys::MQLONG, mapping::MQSECTYPE_MAPSTR,
            r##"Security Types"##
        );
        impl_value!(MQSECTYPE, mqsys::MQLONG);
        define_new_type!(
            pub MQSELTYPE, mqsys::MQLONG, mapping::MQSELTYPE_MAPSTR,
            r##"Selector types"##
        );
        impl_value!(MQSELTYPE, mqsys::MQLONG);
        define_new_type!(
            pub MQSTDBY, mqsys::MQLONG, mapping::MQSTDBY_MAPSTR,
            r##"Multi-instance Queue Managers"##
        );
        impl_value!(MQSTDBY, mqsys::MQLONG);
        define_new_type!(
            pub MQSUBTYPE, mqsys::MQLONG, mapping::MQSUBTYPE_MAPSTR,
            r##"Subscription Types"##
        );
        impl_value!(MQSUBTYPE, mqsys::MQLONG);
        define_new_type!(
            pub MQSUS, mqsys::MQLONG, mapping::MQSUS_MAPSTR, r##"Suspend Status"##
        );
        impl_value!(MQSUS, mqsys::MQLONG);
        define_new_type!(
            pub MQSYNCPOINT, mqsys::MQLONG, mapping::MQSYNCPOINT_MAPSTR,
            r##"Syncpoint values for Pub/Sub migration"##
        );
        impl_value!(MQSYNCPOINT, mqsys::MQLONG);
        define_new_type!(
            pub MQSYSOBJ, mqsys::MQLONG, mapping::MQSYSOBJ_MAPSTR, r##"System Objects"##
        );
        impl_value!(MQSYSOBJ, mqsys::MQLONG);
        define_new_type!(
            pub MQSYSP, mqsys::MQLONG, mapping::MQSYSP_MAPSTR,
            r##"System Parameter Values"##
        );
        impl_value!(MQSYSP, mqsys::MQLONG);
        define_new_type!(
            pub MQS_AVAIL, mqsys::MQLONG, mapping::MQS_AVAIL_MAPSTR,
            r##"SMDS Availability Options"##
        );
        impl_value!(MQS_AVAIL, mqsys::MQLONG);
        define_new_type!(
            pub MQS_EXPANDST, mqsys::MQLONG, mapping::MQS_EXPANDST_MAPSTR,
            r##"Expandst Options"##
        );
        impl_value!(MQS_EXPANDST, mqsys::MQLONG);
        define_new_type!(
            pub MQS_OPENMODE, mqsys::MQLONG, mapping::MQS_OPENMODE_MAPSTR,
            r##"Open Mode Options"##
        );
        impl_value!(MQS_OPENMODE, mqsys::MQLONG);
        define_new_type!(
            pub MQS_STATUS, mqsys::MQLONG, mapping::MQS_STATUS_MAPSTR,
            r##"SMDS Status Options"##
        );
        impl_value!(MQS_STATUS, mqsys::MQLONG);
        define_new_type!(
            pub MQTIME, mqsys::MQLONG, mapping::MQTIME_MAPSTR, r##"Time units"##
        );
        impl_value!(MQTIME, mqsys::MQLONG);
        define_new_type!(
            pub MQUCI, mqsys::MQLONG, mapping::MQUCI_MAPSTR, r##"Use ClientID"##
        );
        impl_value!(MQUCI, mqsys::MQLONG);
        define_new_type!(
            pub MQUIDSUPP, mqsys::MQLONG, mapping::MQUIDSUPP_MAPSTR,
            r##"User ID Support"##
        );
        impl_value!(MQUIDSUPP, mqsys::MQLONG);
        define_new_type!(
            pub MQUNDELIVERED, mqsys::MQLONG, mapping::MQUNDELIVERED_MAPSTR,
            r##"Undelivered values for Pub/Sub migration"##
        );
        impl_value!(MQUNDELIVERED, mqsys::MQLONG);
        define_new_type!(
            pub MQUOWST, mqsys::MQLONG, mapping::MQUOWST_MAPSTR, r##"UOW States"##
        );
        impl_value!(MQUOWST, mqsys::MQLONG);
        define_new_type!(
            pub MQUOWT, mqsys::MQLONG, mapping::MQUOWT_MAPSTR, r##"UOW Types"##
        );
        impl_value!(MQUOWT, mqsys::MQLONG);
        define_new_type!(
            pub MQUSAGE_DS, mqsys::MQLONG, mapping::MQUSAGE_DS_MAPSTR,
            r##"Data Set Usage Values"##
        );
        impl_value!(MQUSAGE_DS, mqsys::MQLONG);
        define_new_type!(
            pub MQUSAGE_EXPAND, mqsys::MQLONG, mapping::MQUSAGE_EXPAND_MAPSTR,
            r##"Expand Usage Values"##
        );
        impl_value!(MQUSAGE_EXPAND, mqsys::MQLONG);
        define_new_type!(
            pub MQUSAGE_PS, mqsys::MQLONG, mapping::MQUSAGE_PS_MAPSTR,
            r##"Page Set Usage Values"##
        );
        impl_value!(MQUSAGE_PS, mqsys::MQLONG);
        define_new_type!(
            pub MQUSAGE_SMDS, mqsys::MQLONG, mapping::MQUSAGE_SMDS_MAPSTR,
            r##"Usage SMDS Options"##
        );
        impl_value!(MQUSAGE_SMDS, mqsys::MQLONG);
    }
    #[cfg(feature = "pcf")]
    pub use pcf::*;
}
pub mod constants {
    use crate::types;
    pub const MQACTP_NEW: types::MQACTP = types::MQACTP(0);
    pub const MQACTP_FORWARD: types::MQACTP = types::MQACTP(1);
    pub const MQACTP_REPLY: types::MQACTP = types::MQACTP(2);
    pub const MQACTP_REPORT: types::MQACTP = types::MQACTP(3);
    pub const MQACTV_DETAIL_LOW: types::MQACTV = types::MQACTV(1);
    pub const MQACTV_DETAIL_MEDIUM: types::MQACTV = types::MQACTV(2);
    pub const MQACTV_DETAIL_HIGH: types::MQACTV = types::MQACTV(3);
    pub const MQADOPT_CHECK_NONE: types::MQADOPT_CHECK = types::MQADOPT_CHECK(0);
    pub const MQADOPT_CHECK_ALL: types::MQADOPT_CHECK = types::MQADOPT_CHECK(1);
    pub const MQADOPT_CHECK_Q_MGR_NAME: types::MQADOPT_CHECK = types::MQADOPT_CHECK(2);
    pub const MQADOPT_CHECK_NET_ADDR: types::MQADOPT_CHECK = types::MQADOPT_CHECK(4);
    pub const MQADOPT_CHECK_CHANNEL_NAME: types::MQADOPT_CHECK = types::MQADOPT_CHECK(8);
    pub const MQADOPT_TYPE_NO: types::MQADOPT_TYPE = types::MQADOPT_TYPE(0);
    pub const MQADOPT_TYPE_ALL: types::MQADOPT_TYPE = types::MQADOPT_TYPE(1);
    pub const MQADOPT_TYPE_SVR: types::MQADOPT_TYPE = types::MQADOPT_TYPE(2);
    pub const MQADOPT_TYPE_SDR: types::MQADOPT_TYPE = types::MQADOPT_TYPE(4);
    pub const MQADOPT_TYPE_RCVR: types::MQADOPT_TYPE = types::MQADOPT_TYPE(8);
    pub const MQADOPT_TYPE_CLUSRCVR: types::MQADOPT_TYPE = types::MQADOPT_TYPE(16);
    pub const MQAIT_ALL: types::MQAIT = types::MQAIT(0);
    pub const MQAIT_CRL_LDAP: types::MQAIT = types::MQAIT(1);
    pub const MQAIT_OCSP: types::MQAIT = types::MQAIT(2);
    pub const MQAIT_IDPW_OS: types::MQAIT = types::MQAIT(3);
    pub const MQAIT_IDPW_LDAP: types::MQAIT = types::MQAIT(4);
    pub const MQAT_UNKNOWN: types::MQAT = types::MQAT(-1);
    pub const MQAT_NO_CONTEXT: types::MQAT = types::MQAT(0);
    pub const MQAT_CICS: types::MQAT = types::MQAT(1);
    pub const MQAT_ZOS: types::MQAT = types::MQAT(2);
    pub const MQAT_IMS: types::MQAT = types::MQAT(3);
    pub const MQAT_OS2: types::MQAT = types::MQAT(4);
    pub const MQAT_DOS: types::MQAT = types::MQAT(5);
    pub const MQAT_UNIX: types::MQAT = types::MQAT(6);
    pub const MQAT_QMGR: types::MQAT = types::MQAT(7);
    pub const MQAT_OS400: types::MQAT = types::MQAT(8);
    pub const MQAT_WINDOWS: types::MQAT = types::MQAT(9);
    pub const MQAT_CICS_VSE: types::MQAT = types::MQAT(10);
    pub const MQAT_WINDOWS_NT: types::MQAT = types::MQAT(11);
    pub const MQAT_VMS: types::MQAT = types::MQAT(12);
    pub const MQAT_NSK: types::MQAT = types::MQAT(13);
    pub const MQAT_VOS: types::MQAT = types::MQAT(14);
    pub const MQAT_OPEN_TP1: types::MQAT = types::MQAT(15);
    pub const MQAT_VM: types::MQAT = types::MQAT(18);
    pub const MQAT_IMS_BRIDGE: types::MQAT = types::MQAT(19);
    pub const MQAT_XCF: types::MQAT = types::MQAT(20);
    pub const MQAT_CICS_BRIDGE: types::MQAT = types::MQAT(21);
    pub const MQAT_NOTES_AGENT: types::MQAT = types::MQAT(22);
    pub const MQAT_TPF: types::MQAT = types::MQAT(23);
    pub const MQAT_USER: types::MQAT = types::MQAT(25);
    pub const MQAT_QMGR_PUBLISH: types::MQAT = types::MQAT(26);
    pub const MQAT_JAVA: types::MQAT = types::MQAT(28);
    pub const MQAT_DQM: types::MQAT = types::MQAT(29);
    pub const MQAT_CHANNEL_INITIATOR: types::MQAT = types::MQAT(30);
    pub const MQAT_WLM: types::MQAT = types::MQAT(31);
    pub const MQAT_BATCH: types::MQAT = types::MQAT(32);
    pub const MQAT_RRS_BATCH: types::MQAT = types::MQAT(33);
    pub const MQAT_SIB: types::MQAT = types::MQAT(34);
    pub const MQAT_SYSTEM_EXTENSION: types::MQAT = types::MQAT(35);
    pub const MQAT_MCAST_PUBLISH: types::MQAT = types::MQAT(36);
    pub const MQAT_AMQP: types::MQAT = types::MQAT(37);
    pub const MQAT_MVS: types::MQAT = types::MQAT(2);
    pub const MQAT_OS390: types::MQAT = types::MQAT(2);
    pub const MQAT_AIX: types::MQAT = types::MQAT(6);
    pub const MQAT_DEFAULT: types::MQAT = types::MQAT(6);
    pub const MQAT_GUARDIAN: types::MQAT = types::MQAT(13);
    pub const MQAT_BROKER: types::MQAT = types::MQAT(26);
    pub const MQAT_USER_FIRST: types::MQAT = types::MQAT(65536);
    pub const MQAT_USER_LAST: types::MQAT = types::MQAT(999999999);
    pub const MQAUTO_START_NO: types::MQAUTO = types::MQAUTO(0);
    pub const MQAUTO_START_YES: types::MQAUTO = types::MQAUTO(1);
    pub const MQBMHO_NONE: types::MQBMHO = types::MQBMHO(0);
    pub const MQBMHO_DELETE_PROPERTIES: types::MQBMHO = types::MQBMHO(1);
    pub const MQBND_BIND_ON_OPEN: types::MQBND = types::MQBND(0);
    pub const MQBND_BIND_NOT_FIXED: types::MQBND = types::MQBND(1);
    pub const MQBND_BIND_ON_GROUP: types::MQBND = types::MQBND(2);
    pub const MQBNO_BALTYPE_SIMPLE: types::MQBNO_BALTYPE = types::MQBNO_BALTYPE(0);
    pub const MQBNO_BALTYPE_REQREP: types::MQBNO_BALTYPE = types::MQBNO_BALTYPE(1);
    pub const MQBNO_BALTYPE_RA_MANAGED: types::MQBNO_BALTYPE = types::MQBNO_BALTYPE(
        65536,
    );
    pub const MQBNO_OPTIONS_NONE: types::MQBNO_OPTIONS = types::MQBNO_OPTIONS(0);
    pub const MQBNO_OPTIONS_IGNORE_TRANS: types::MQBNO_OPTIONS = types::MQBNO_OPTIONS(1);
    pub const MQBNO_TIMEOUT_NEVER: types::MQBNO_TIMEOUT = types::MQBNO_TIMEOUT(-2);
    pub const MQBNO_TIMEOUT_AS_DEFAULT: types::MQBNO_TIMEOUT = types::MQBNO_TIMEOUT(-1);
    pub const MQBNO_TIMEOUT_IMMEDIATE: types::MQBNO_TIMEOUT = types::MQBNO_TIMEOUT(0);
    pub const MQBO_NONE: types::MQBO = types::MQBO(0);
    pub const MQCADSD_NONE: types::MQCADSD = types::MQCADSD(0);
    pub const MQCADSD_SEND: types::MQCADSD = types::MQCADSD(1);
    pub const MQCADSD_RECV: types::MQCADSD = types::MQCADSD(16);
    pub const MQCADSD_MSGFORMAT: types::MQCADSD = types::MQCADSD(256);
    pub const MQCAFTY_NONE: types::MQCAFTY = types::MQCAFTY(0);
    pub const MQCAFTY_PREFERRED: types::MQCAFTY = types::MQCAFTY(1);
    pub const MQCAP_NOT_SUPPORTED: types::MQCAP = types::MQCAP(0);
    pub const MQCAP_SUPPORTED: types::MQCAP = types::MQCAP(1);
    pub const MQCAP_EXPIRED: types::MQCAP = types::MQCAP(2);
    pub const MQCA_APPL_ID: types::MQCA = types::MQCA(2001);
    pub const MQCA_BASE_OBJECT_NAME: types::MQCA = types::MQCA(2002);
    pub const MQCA_COMMAND_INPUT_Q_NAME: types::MQCA = types::MQCA(2003);
    pub const MQCA_CREATION_DATE: types::MQCA = types::MQCA(2004);
    pub const MQCA_CREATION_TIME: types::MQCA = types::MQCA(2005);
    pub const MQCA_DEAD_LETTER_Q_NAME: types::MQCA = types::MQCA(2006);
    pub const MQCA_ENV_DATA: types::MQCA = types::MQCA(2007);
    pub const MQCA_INITIATION_Q_NAME: types::MQCA = types::MQCA(2008);
    pub const MQCA_NAMELIST_DESC: types::MQCA = types::MQCA(2009);
    pub const MQCA_NAMELIST_NAME: types::MQCA = types::MQCA(2010);
    pub const MQCA_PROCESS_DESC: types::MQCA = types::MQCA(2011);
    pub const MQCA_PROCESS_NAME: types::MQCA = types::MQCA(2012);
    pub const MQCA_Q_DESC: types::MQCA = types::MQCA(2013);
    pub const MQCA_Q_MGR_DESC: types::MQCA = types::MQCA(2014);
    pub const MQCA_Q_MGR_NAME: types::MQCA = types::MQCA(2015);
    pub const MQCA_Q_NAME: types::MQCA = types::MQCA(2016);
    pub const MQCA_REMOTE_Q_MGR_NAME: types::MQCA = types::MQCA(2017);
    pub const MQCA_REMOTE_Q_NAME: types::MQCA = types::MQCA(2018);
    pub const MQCA_BACKOUT_REQ_Q_NAME: types::MQCA = types::MQCA(2019);
    pub const MQCA_NAMES: types::MQCA = types::MQCA(2020);
    pub const MQCA_USER_DATA: types::MQCA = types::MQCA(2021);
    pub const MQCA_STORAGE_CLASS: types::MQCA = types::MQCA(2022);
    pub const MQCA_TRIGGER_DATA: types::MQCA = types::MQCA(2023);
    pub const MQCA_XMIT_Q_NAME: types::MQCA = types::MQCA(2024);
    pub const MQCA_DEF_XMIT_Q_NAME: types::MQCA = types::MQCA(2025);
    pub const MQCA_CHANNEL_AUTO_DEF_EXIT: types::MQCA = types::MQCA(2026);
    pub const MQCA_ALTERATION_DATE: types::MQCA = types::MQCA(2027);
    pub const MQCA_ALTERATION_TIME: types::MQCA = types::MQCA(2028);
    pub const MQCA_CLUSTER_NAME: types::MQCA = types::MQCA(2029);
    pub const MQCA_CLUSTER_NAMELIST: types::MQCA = types::MQCA(2030);
    pub const MQCA_CLUSTER_Q_MGR_NAME: types::MQCA = types::MQCA(2031);
    pub const MQCA_Q_MGR_IDENTIFIER: types::MQCA = types::MQCA(2032);
    pub const MQCA_CLUSTER_WORKLOAD_EXIT: types::MQCA = types::MQCA(2033);
    pub const MQCA_CLUSTER_WORKLOAD_DATA: types::MQCA = types::MQCA(2034);
    pub const MQCA_REPOSITORY_NAME: types::MQCA = types::MQCA(2035);
    pub const MQCA_REPOSITORY_NAMELIST: types::MQCA = types::MQCA(2036);
    pub const MQCA_CLUSTER_DATE: types::MQCA = types::MQCA(2037);
    pub const MQCA_CLUSTER_TIME: types::MQCA = types::MQCA(2038);
    pub const MQCA_CF_STRUC_NAME: types::MQCA = types::MQCA(2039);
    pub const MQCA_QSG_NAME: types::MQCA = types::MQCA(2040);
    pub const MQCA_IGQ_USER_ID: types::MQCA = types::MQCA(2041);
    pub const MQCA_STORAGE_CLASS_DESC: types::MQCA = types::MQCA(2042);
    pub const MQCA_XCF_GROUP_NAME: types::MQCA = types::MQCA(2043);
    pub const MQCA_XCF_MEMBER_NAME: types::MQCA = types::MQCA(2044);
    pub const MQCA_AUTH_INFO_NAME: types::MQCA = types::MQCA(2045);
    pub const MQCA_AUTH_INFO_DESC: types::MQCA = types::MQCA(2046);
    pub const MQCA_LDAP_USER_NAME: types::MQCA = types::MQCA(2047);
    pub const MQCA_LDAP_PASSWORD: types::MQCA = types::MQCA(2048);
    pub const MQCA_SSL_KEY_REPOSITORY: types::MQCA = types::MQCA(2049);
    pub const MQCA_SSL_CRL_NAMELIST: types::MQCA = types::MQCA(2050);
    pub const MQCA_SSL_CRYPTO_HARDWARE: types::MQCA = types::MQCA(2051);
    pub const MQCA_CF_STRUC_DESC: types::MQCA = types::MQCA(2052);
    pub const MQCA_AUTH_INFO_CONN_NAME: types::MQCA = types::MQCA(2053);
    pub const MQCA_INITIAL_KEY: types::MQCA = types::MQCA(2054);
    pub const MQCA_SSL_KEY_REPO_PASSWORD: types::MQCA = types::MQCA(2055);
    pub const MQCA_CICS_FILE_NAME: types::MQCA = types::MQCA(2060);
    pub const MQCA_TRIGGER_TRANS_ID: types::MQCA = types::MQCA(2061);
    pub const MQCA_TRIGGER_PROGRAM_NAME: types::MQCA = types::MQCA(2062);
    pub const MQCA_TRIGGER_TERM_ID: types::MQCA = types::MQCA(2063);
    pub const MQCA_TRIGGER_CHANNEL_NAME: types::MQCA = types::MQCA(2064);
    pub const MQCA_SYSTEM_LOG_Q_NAME: types::MQCA = types::MQCA(2065);
    pub const MQCA_MONITOR_Q_NAME: types::MQCA = types::MQCA(2066);
    pub const MQCA_COMMAND_REPLY_Q_NAME: types::MQCA = types::MQCA(2067);
    pub const MQCA_BATCH_INTERFACE_ID: types::MQCA = types::MQCA(2068);
    pub const MQCA_SSL_KEY_LIBRARY: types::MQCA = types::MQCA(2069);
    pub const MQCA_SSL_KEY_MEMBER: types::MQCA = types::MQCA(2070);
    pub const MQCA_DNS_GROUP: types::MQCA = types::MQCA(2071);
    pub const MQCA_LU_GROUP_NAME: types::MQCA = types::MQCA(2072);
    pub const MQCA_LU_NAME: types::MQCA = types::MQCA(2073);
    pub const MQCA_LU62_ARM_SUFFIX: types::MQCA = types::MQCA(2074);
    pub const MQCA_TCP_NAME: types::MQCA = types::MQCA(2075);
    pub const MQCA_CHINIT_SERVICE_PARM: types::MQCA = types::MQCA(2076);
    pub const MQCA_SERVICE_NAME: types::MQCA = types::MQCA(2077);
    pub const MQCA_SERVICE_DESC: types::MQCA = types::MQCA(2078);
    pub const MQCA_SERVICE_START_COMMAND: types::MQCA = types::MQCA(2079);
    pub const MQCA_SERVICE_START_ARGS: types::MQCA = types::MQCA(2080);
    pub const MQCA_SERVICE_STOP_COMMAND: types::MQCA = types::MQCA(2081);
    pub const MQCA_SERVICE_STOP_ARGS: types::MQCA = types::MQCA(2082);
    pub const MQCA_STDOUT_DESTINATION: types::MQCA = types::MQCA(2083);
    pub const MQCA_STDERR_DESTINATION: types::MQCA = types::MQCA(2084);
    pub const MQCA_TPIPE_NAME: types::MQCA = types::MQCA(2085);
    pub const MQCA_PASS_TICKET_APPL: types::MQCA = types::MQCA(2086);
    pub const MQCA_AUTO_REORG_START_TIME: types::MQCA = types::MQCA(2090);
    pub const MQCA_AUTO_REORG_CATALOG: types::MQCA = types::MQCA(2091);
    pub const MQCA_TOPIC_NAME: types::MQCA = types::MQCA(2092);
    pub const MQCA_TOPIC_DESC: types::MQCA = types::MQCA(2093);
    pub const MQCA_TOPIC_STRING: types::MQCA = types::MQCA(2094);
    pub const MQCA_MODEL_DURABLE_Q: types::MQCA = types::MQCA(2096);
    pub const MQCA_MODEL_NON_DURABLE_Q: types::MQCA = types::MQCA(2097);
    pub const MQCA_RESUME_DATE: types::MQCA = types::MQCA(2098);
    pub const MQCA_RESUME_TIME: types::MQCA = types::MQCA(2099);
    pub const MQCA_CHILD: types::MQCA = types::MQCA(2101);
    pub const MQCA_PARENT: types::MQCA = types::MQCA(2102);
    pub const MQCA_ADMIN_TOPIC_NAME: types::MQCA = types::MQCA(2105);
    pub const MQCA_TOPIC_STRING_FILTER: types::MQCA = types::MQCA(2108);
    pub const MQCA_AUTH_INFO_OCSP_URL: types::MQCA = types::MQCA(2109);
    pub const MQCA_COMM_INFO_NAME: types::MQCA = types::MQCA(2110);
    pub const MQCA_COMM_INFO_DESC: types::MQCA = types::MQCA(2111);
    pub const MQCA_POLICY_NAME: types::MQCA = types::MQCA(2112);
    pub const MQCA_SIGNER_DN: types::MQCA = types::MQCA(2113);
    pub const MQCA_RECIPIENT_DN: types::MQCA = types::MQCA(2114);
    pub const MQCA_INSTALLATION_DESC: types::MQCA = types::MQCA(2115);
    pub const MQCA_INSTALLATION_NAME: types::MQCA = types::MQCA(2116);
    pub const MQCA_INSTALLATION_PATH: types::MQCA = types::MQCA(2117);
    pub const MQCA_CHLAUTH_DESC: types::MQCA = types::MQCA(2118);
    pub const MQCA_CUSTOM: types::MQCA = types::MQCA(2119);
    pub const MQCA_VERSION: types::MQCA = types::MQCA(2120);
    pub const MQCA_CERT_LABEL: types::MQCA = types::MQCA(2121);
    pub const MQCA_XR_VERSION: types::MQCA = types::MQCA(2122);
    pub const MQCA_XR_SSL_CIPHER_SUITES: types::MQCA = types::MQCA(2123);
    pub const MQCA_CLUS_CHL_NAME: types::MQCA = types::MQCA(2124);
    pub const MQCA_CONN_AUTH: types::MQCA = types::MQCA(2125);
    pub const MQCA_LDAP_BASE_DN_USERS: types::MQCA = types::MQCA(2126);
    pub const MQCA_LDAP_SHORT_USER_FIELD: types::MQCA = types::MQCA(2127);
    pub const MQCA_LDAP_USER_OBJECT_CLASS: types::MQCA = types::MQCA(2128);
    pub const MQCA_LDAP_USER_ATTR_FIELD: types::MQCA = types::MQCA(2129);
    pub const MQCA_SSL_CERT_ISSUER_NAME: types::MQCA = types::MQCA(2130);
    pub const MQCA_QSG_CERT_LABEL: types::MQCA = types::MQCA(2131);
    pub const MQCA_LDAP_BASE_DN_GROUPS: types::MQCA = types::MQCA(2132);
    pub const MQCA_LDAP_GROUP_OBJECT_CLASS: types::MQCA = types::MQCA(2133);
    pub const MQCA_LDAP_GROUP_ATTR_FIELD: types::MQCA = types::MQCA(2134);
    pub const MQCA_LDAP_FIND_GROUP_FIELD: types::MQCA = types::MQCA(2135);
    pub const MQCA_AMQP_VERSION: types::MQCA = types::MQCA(2136);
    pub const MQCA_AMQP_SSL_CIPHER_SUITES: types::MQCA = types::MQCA(2137);
    pub const MQCA_STREAM_QUEUE_NAME: types::MQCA = types::MQCA(2138);
    pub const MQCA_USER_LIST: types::MQCA = types::MQCA(4000);
    pub const MQCA_FIRST: types::MQCA = types::MQCA(2001);
    pub const MQCA_BASE_Q_NAME: types::MQCA = types::MQCA(2002);
    pub const MQCA_LAST_USED: types::MQCA = types::MQCA(2138);
    pub const MQCA_LAST: types::MQCA = types::MQCA(4000);
    pub const MQCBCF_NONE: types::MQCBCF = types::MQCBCF(0);
    pub const MQCBCF_READA_BUFFER_EMPTY: types::MQCBCF = types::MQCBCF(1);
    pub const MQCBCT_START_CALL: types::MQCBCT = types::MQCBCT(1);
    pub const MQCBCT_STOP_CALL: types::MQCBCT = types::MQCBCT(2);
    pub const MQCBCT_REGISTER_CALL: types::MQCBCT = types::MQCBCT(3);
    pub const MQCBCT_DEREGISTER_CALL: types::MQCBCT = types::MQCBCT(4);
    pub const MQCBCT_EVENT_CALL: types::MQCBCT = types::MQCBCT(5);
    pub const MQCBCT_MSG_REMOVED: types::MQCBCT = types::MQCBCT(6);
    pub const MQCBCT_MSG_NOT_REMOVED: types::MQCBCT = types::MQCBCT(7);
    pub const MQCBCT_MC_EVENT_CALL: types::MQCBCT = types::MQCBCT(8);
    pub const MQCBDO_NONE: types::MQCBDO = types::MQCBDO(0);
    pub const MQCBDO_START_CALL: types::MQCBDO = types::MQCBDO(1);
    pub const MQCBDO_STOP_CALL: types::MQCBDO = types::MQCBDO(4);
    pub const MQCBDO_REGISTER_CALL: types::MQCBDO = types::MQCBDO(256);
    pub const MQCBDO_DEREGISTER_CALL: types::MQCBDO = types::MQCBDO(512);
    pub const MQCBDO_FAIL_IF_QUIESCING: types::MQCBDO = types::MQCBDO(8192);
    pub const MQCBDO_EVENT_CALL: types::MQCBDO = types::MQCBDO(16384);
    pub const MQCBDO_MC_EVENT_CALL: types::MQCBDO = types::MQCBDO(32768);
    pub const MQCBD_FULL_MSG_LENGTH: types::MQCBD = types::MQCBD(-1);
    pub const MQCBT_MESSAGE_CONSUMER: types::MQCBT = types::MQCBT(1);
    pub const MQCBT_EVENT_HANDLER: types::MQCBT = types::MQCBT(2);
    pub const MQCCSI_AS_PUBLISHED: types::MQCCSI = types::MQCCSI(-4);
    pub const MQCCSI_APPL: types::MQCCSI = types::MQCCSI(-3);
    pub const MQCCSI_INHERIT: types::MQCCSI = types::MQCCSI(-2);
    pub const MQCCSI_EMBEDDED: types::MQCCSI = types::MQCCSI(-1);
    pub const MQCCSI_DEFAULT: types::MQCCSI = types::MQCCSI(0);
    pub const MQCCSI_Q_MGR: types::MQCCSI = types::MQCCSI(0);
    pub const MQCCSI_UNDEFINED: types::MQCCSI = types::MQCCSI(0);
    pub const MQCCT_NO: types::MQCCT = types::MQCCT(0);
    pub const MQCCT_YES: types::MQCCT = types::MQCCT(1);
    pub const MQCC_UNKNOWN: types::MQCC = types::MQCC(-1);
    pub const MQCC_OK: types::MQCC = types::MQCC(0);
    pub const MQCC_WARNING: types::MQCC = types::MQCC(1);
    pub const MQCC_FAILED: types::MQCC = types::MQCC(2);
    pub const MQCEX_AS_PARENT: types::MQCEX = types::MQCEX(-2);
    pub const MQCEX_NOLIMIT: types::MQCEX = types::MQCEX(-1);
    pub const MQCFCONLOS_TERMINATE: types::MQCFCONLOS = types::MQCFCONLOS(0);
    pub const MQCFCONLOS_TOLERATE: types::MQCFCONLOS = types::MQCFCONLOS(1);
    pub const MQCFCONLOS_ASQMGR: types::MQCFCONLOS = types::MQCFCONLOS(2);
    pub const MQCFOFFLD_NONE: types::MQCFOFFLD = types::MQCFOFFLD(0);
    pub const MQCFOFFLD_SMDS: types::MQCFOFFLD = types::MQCFOFFLD(1);
    pub const MQCFOFFLD_DB2: types::MQCFOFFLD = types::MQCFOFFLD(2);
    pub const MQCFOFFLD_BOTH: types::MQCFOFFLD = types::MQCFOFFLD(3);
    pub const MQCFR_NO: types::MQCFR = types::MQCFR(0);
    pub const MQCFR_YES: types::MQCFR = types::MQCFR(1);
    pub const MQCGWI_DEFAULT: types::MQCGWI = types::MQCGWI(-2);
    pub const MQCHAD_DISABLED: types::MQCHAD = types::MQCHAD(0);
    pub const MQCHAD_ENABLED: types::MQCHAD = types::MQCHAD(1);
    pub const MQCHT_SENDER: types::MQCHT = types::MQCHT(1);
    pub const MQCHT_SERVER: types::MQCHT = types::MQCHT(2);
    pub const MQCHT_RECEIVER: types::MQCHT = types::MQCHT(3);
    pub const MQCHT_REQUESTER: types::MQCHT = types::MQCHT(4);
    pub const MQCHT_ALL: types::MQCHT = types::MQCHT(5);
    pub const MQCHT_CLNTCONN: types::MQCHT = types::MQCHT(6);
    pub const MQCHT_SVRCONN: types::MQCHT = types::MQCHT(7);
    pub const MQCHT_CLUSRCVR: types::MQCHT = types::MQCHT(8);
    pub const MQCHT_CLUSSDR: types::MQCHT = types::MQCHT(9);
    pub const MQCHT_MQTT: types::MQCHT = types::MQCHT(10);
    pub const MQCHT_AMQP: types::MQCHT = types::MQCHT(11);
    pub const MQCIH_NONE: types::MQCIH = types::MQCIH(0);
    pub const MQCIH_PASS_EXPIRATION: types::MQCIH = types::MQCIH(1);
    pub const MQCIH_REPLY_WITHOUT_NULLS: types::MQCIH = types::MQCIH(2);
    pub const MQCIH_SYNC_ON_RETURN: types::MQCIH = types::MQCIH(4);
    pub const MQCIH_NO_SYNC_ON_RETURN: types::MQCIH = types::MQCIH(0);
    pub const MQCIH_REPLY_WITH_NULLS: types::MQCIH = types::MQCIH(0);
    pub const MQCIH_UNLIMITED_EXPIRATION: types::MQCIH = types::MQCIH(0);
    pub const MQCIT_MULTICAST: types::MQCIT = types::MQCIT(1);
    pub const MQCLT_PROGRAM: types::MQCLT = types::MQCLT(1);
    pub const MQCLT_TRANSACTION: types::MQCLT = types::MQCLT(2);
    pub const MQCLWL_USEQ_AS_Q_MGR: types::MQCLWL = types::MQCLWL(-3);
    pub const MQCLWL_USEQ_LOCAL: types::MQCLWL = types::MQCLWL(0);
    pub const MQCLWL_USEQ_ANY: types::MQCLWL = types::MQCLWL(1);
    pub const MQCMDL_LEVEL_1: types::MQCMDL = types::MQCMDL(100);
    pub const MQCMDL_LEVEL_101: types::MQCMDL = types::MQCMDL(101);
    pub const MQCMDL_LEVEL_110: types::MQCMDL = types::MQCMDL(110);
    pub const MQCMDL_LEVEL_114: types::MQCMDL = types::MQCMDL(114);
    pub const MQCMDL_LEVEL_120: types::MQCMDL = types::MQCMDL(120);
    pub const MQCMDL_LEVEL_200: types::MQCMDL = types::MQCMDL(200);
    pub const MQCMDL_LEVEL_201: types::MQCMDL = types::MQCMDL(201);
    pub const MQCMDL_LEVEL_210: types::MQCMDL = types::MQCMDL(210);
    pub const MQCMDL_LEVEL_211: types::MQCMDL = types::MQCMDL(211);
    pub const MQCMDL_LEVEL_220: types::MQCMDL = types::MQCMDL(220);
    pub const MQCMDL_LEVEL_221: types::MQCMDL = types::MQCMDL(221);
    pub const MQCMDL_LEVEL_230: types::MQCMDL = types::MQCMDL(230);
    pub const MQCMDL_LEVEL_320: types::MQCMDL = types::MQCMDL(320);
    pub const MQCMDL_LEVEL_420: types::MQCMDL = types::MQCMDL(420);
    pub const MQCMDL_LEVEL_500: types::MQCMDL = types::MQCMDL(500);
    pub const MQCMDL_LEVEL_510: types::MQCMDL = types::MQCMDL(510);
    pub const MQCMDL_LEVEL_520: types::MQCMDL = types::MQCMDL(520);
    pub const MQCMDL_LEVEL_530: types::MQCMDL = types::MQCMDL(530);
    pub const MQCMDL_LEVEL_531: types::MQCMDL = types::MQCMDL(531);
    pub const MQCMDL_LEVEL_600: types::MQCMDL = types::MQCMDL(600);
    pub const MQCMDL_LEVEL_700: types::MQCMDL = types::MQCMDL(700);
    pub const MQCMDL_LEVEL_701: types::MQCMDL = types::MQCMDL(701);
    pub const MQCMDL_LEVEL_710: types::MQCMDL = types::MQCMDL(710);
    pub const MQCMDL_LEVEL_711: types::MQCMDL = types::MQCMDL(711);
    pub const MQCMDL_LEVEL_750: types::MQCMDL = types::MQCMDL(750);
    pub const MQCMDL_LEVEL_800: types::MQCMDL = types::MQCMDL(800);
    pub const MQCMDL_LEVEL_801: types::MQCMDL = types::MQCMDL(801);
    pub const MQCMDL_LEVEL_802: types::MQCMDL = types::MQCMDL(802);
    pub const MQCMDL_LEVEL_900: types::MQCMDL = types::MQCMDL(900);
    pub const MQCMDL_LEVEL_901: types::MQCMDL = types::MQCMDL(901);
    pub const MQCMDL_LEVEL_902: types::MQCMDL = types::MQCMDL(902);
    pub const MQCMDL_LEVEL_903: types::MQCMDL = types::MQCMDL(903);
    pub const MQCMDL_LEVEL_904: types::MQCMDL = types::MQCMDL(904);
    pub const MQCMDL_LEVEL_905: types::MQCMDL = types::MQCMDL(905);
    pub const MQCMDL_LEVEL_910: types::MQCMDL = types::MQCMDL(910);
    pub const MQCMDL_LEVEL_911: types::MQCMDL = types::MQCMDL(911);
    pub const MQCMDL_LEVEL_912: types::MQCMDL = types::MQCMDL(912);
    pub const MQCMDL_LEVEL_913: types::MQCMDL = types::MQCMDL(913);
    pub const MQCMDL_LEVEL_914: types::MQCMDL = types::MQCMDL(914);
    pub const MQCMDL_LEVEL_915: types::MQCMDL = types::MQCMDL(915);
    pub const MQCMDL_LEVEL_920: types::MQCMDL = types::MQCMDL(920);
    pub const MQCMDL_LEVEL_921: types::MQCMDL = types::MQCMDL(921);
    pub const MQCMDL_LEVEL_922: types::MQCMDL = types::MQCMDL(922);
    pub const MQCMDL_LEVEL_923: types::MQCMDL = types::MQCMDL(923);
    pub const MQCMDL_LEVEL_924: types::MQCMDL = types::MQCMDL(924);
    pub const MQCMDL_LEVEL_925: types::MQCMDL = types::MQCMDL(925);
    pub const MQCMDL_LEVEL_930: types::MQCMDL = types::MQCMDL(930);
    pub const MQCMDL_LEVEL_931: types::MQCMDL = types::MQCMDL(931);
    pub const MQCMDL_LEVEL_932: types::MQCMDL = types::MQCMDL(932);
    pub const MQCMDL_LEVEL_933: types::MQCMDL = types::MQCMDL(933);
    pub const MQCMDL_LEVEL_934: types::MQCMDL = types::MQCMDL(934);
    pub const MQCMDL_LEVEL_935: types::MQCMDL = types::MQCMDL(935);
    pub const MQCMDL_LEVEL_940: types::MQCMDL = types::MQCMDL(940);
    pub const MQCMDL_LEVEL_941: types::MQCMDL = types::MQCMDL(941);
    pub const MQCMDL_LEVEL_942: types::MQCMDL = types::MQCMDL(942);
    pub const MQCMDL_LEVEL_943: types::MQCMDL = types::MQCMDL(943);
    pub const MQCMDL_CURRENT_LEVEL: types::MQCMDL = types::MQCMDL(943);
    pub const MQCMHO_NONE: types::MQCMHO = types::MQCMHO(0);
    pub const MQCMHO_NO_VALIDATION: types::MQCMHO = types::MQCMHO(1);
    pub const MQCMHO_VALIDATE: types::MQCMHO = types::MQCMHO(2);
    pub const MQCMHO_DEFAULT_VALIDATION: types::MQCMHO = types::MQCMHO(0);
    pub const MQCNO_NONE: types::MQCNO = types::MQCNO(0);
    pub const MQCNO_FASTPATH_BINDING: types::MQCNO = types::MQCNO(1);
    pub const MQCNO_SERIALIZE_CONN_TAG_Q_MGR: types::MQCNO = types::MQCNO(2);
    pub const MQCNO_SERIALIZE_CONN_TAG_QSG: types::MQCNO = types::MQCNO(4);
    pub const MQCNO_RESTRICT_CONN_TAG_Q_MGR: types::MQCNO = types::MQCNO(8);
    pub const MQCNO_RESTRICT_CONN_TAG_QSG: types::MQCNO = types::MQCNO(16);
    pub const MQCNO_HANDLE_SHARE_NONE: types::MQCNO = types::MQCNO(32);
    pub const MQCNO_HANDLE_SHARE_BLOCK: types::MQCNO = types::MQCNO(64);
    pub const MQCNO_HANDLE_SHARE_NO_BLOCK: types::MQCNO = types::MQCNO(128);
    pub const MQCNO_SHARED_BINDING: types::MQCNO = types::MQCNO(256);
    pub const MQCNO_ISOLATED_BINDING: types::MQCNO = types::MQCNO(512);
    pub const MQCNO_LOCAL_BINDING: types::MQCNO = types::MQCNO(1024);
    pub const MQCNO_CLIENT_BINDING: types::MQCNO = types::MQCNO(2048);
    pub const MQCNO_ACCOUNTING_MQI_ENABLED: types::MQCNO = types::MQCNO(4096);
    pub const MQCNO_ACCOUNTING_MQI_DISABLED: types::MQCNO = types::MQCNO(8192);
    pub const MQCNO_ACCOUNTING_Q_ENABLED: types::MQCNO = types::MQCNO(16384);
    pub const MQCNO_ACCOUNTING_Q_DISABLED: types::MQCNO = types::MQCNO(32768);
    pub const MQCNO_NO_CONV_SHARING: types::MQCNO = types::MQCNO(65536);
    pub const MQCNO_ALL_CONVS_SHARE: types::MQCNO = types::MQCNO(262144);
    pub const MQCNO_CD_FOR_OUTPUT_ONLY: types::MQCNO = types::MQCNO(524288);
    pub const MQCNO_USE_CD_SELECTION: types::MQCNO = types::MQCNO(1048576);
    pub const MQCNO_GENERATE_CONN_TAG: types::MQCNO = types::MQCNO(2097152);
    pub const MQCNO_RECONNECT: types::MQCNO = types::MQCNO(16777216);
    pub const MQCNO_RECONNECT_DISABLED: types::MQCNO = types::MQCNO(33554432);
    pub const MQCNO_RECONNECT_Q_MGR: types::MQCNO = types::MQCNO(67108864);
    pub const MQCNO_ACTIVITY_TRACE_ENABLED: types::MQCNO = types::MQCNO(134217728);
    pub const MQCNO_ACTIVITY_TRACE_DISABLED: types::MQCNO = types::MQCNO(268435456);
    pub const MQCNO_RECONNECT_AS_DEF: types::MQCNO = types::MQCNO(0);
    ///The application and the local-queue-manager agent (the component that manages queuing operations) run in separate units of execution
    pub const MQCNO_STANDARD_BINDING: types::MQCNO = types::MQCNO(0);
    pub const MQCODL_AS_INPUT: types::MQCODL = types::MQCODL(-1);
    pub const MQCOPY_NONE: types::MQCOPY = types::MQCOPY(0);
    pub const MQCOPY_ALL: types::MQCOPY = types::MQCOPY(1);
    pub const MQCOPY_FORWARD: types::MQCOPY = types::MQCOPY(2);
    pub const MQCOPY_PUBLISH: types::MQCOPY = types::MQCOPY(4);
    pub const MQCOPY_REPLY: types::MQCOPY = types::MQCOPY(8);
    pub const MQCOPY_REPORT: types::MQCOPY = types::MQCOPY(16);
    pub const MQCOPY_DEFAULT: types::MQCOPY = types::MQCOPY(22);
    pub const MQCO_NONE: types::MQCO = types::MQCO(0);
    pub const MQCO_DELETE: types::MQCO = types::MQCO(1);
    pub const MQCO_DELETE_PURGE: types::MQCO = types::MQCO(2);
    pub const MQCO_KEEP_SUB: types::MQCO = types::MQCO(4);
    pub const MQCO_REMOVE_SUB: types::MQCO = types::MQCO(8);
    pub const MQCO_QUIESCE: types::MQCO = types::MQCO(32);
    pub const MQCO_IMMEDIATE: types::MQCO = types::MQCO(0);
    pub const MQCQT_LOCAL_Q: types::MQCQT = types::MQCQT(1);
    pub const MQCQT_ALIAS_Q: types::MQCQT = types::MQCQT(2);
    pub const MQCQT_REMOTE_Q: types::MQCQT = types::MQCQT(3);
    pub const MQCQT_Q_MGR_ALIAS: types::MQCQT = types::MQCQT(4);
    pub const MQCRC_OK: types::MQCRC = types::MQCRC(0);
    pub const MQCRC_CICS_EXEC_ERROR: types::MQCRC = types::MQCRC(1);
    pub const MQCRC_MQ_API_ERROR: types::MQCRC = types::MQCRC(2);
    pub const MQCRC_BRIDGE_ERROR: types::MQCRC = types::MQCRC(3);
    pub const MQCRC_BRIDGE_ABEND: types::MQCRC = types::MQCRC(4);
    pub const MQCRC_APPLICATION_ABEND: types::MQCRC = types::MQCRC(5);
    pub const MQCRC_SECURITY_ERROR: types::MQCRC = types::MQCRC(6);
    pub const MQCRC_PROGRAM_NOT_AVAILABLE: types::MQCRC = types::MQCRC(7);
    pub const MQCRC_BRIDGE_TIMEOUT: types::MQCRC = types::MQCRC(8);
    pub const MQCRC_TRANSID_NOT_AVAILABLE: types::MQCRC = types::MQCRC(9);
    pub const MQCSP_AUTH_NONE: types::MQCSP = types::MQCSP(0);
    pub const MQCSP_AUTH_USER_ID_AND_PWD: types::MQCSP = types::MQCSP(1);
    pub const MQCSP_AUTH_ID_TOKEN: types::MQCSP = types::MQCSP(2);
    pub const MQCSRV_CONVERT_NO: types::MQCSRV_CONVERT = types::MQCSRV_CONVERT(0);
    pub const MQCSRV_CONVERT_YES: types::MQCSRV_CONVERT = types::MQCSRV_CONVERT(1);
    pub const MQCSRV_DLQ_NO: types::MQCSRV_DLQ = types::MQCSRV_DLQ(0);
    pub const MQCSRV_DLQ_YES: types::MQCSRV_DLQ = types::MQCSRV_DLQ(1);
    pub const MQCS_NONE: types::MQCS = types::MQCS(0);
    pub const MQCS_SUSPENDED_TEMPORARY: types::MQCS = types::MQCS(1);
    pub const MQCS_SUSPENDED_USER_ACTION: types::MQCS = types::MQCS(2);
    pub const MQCS_SUSPENDED: types::MQCS = types::MQCS(3);
    pub const MQCS_STOPPED: types::MQCS = types::MQCS(4);
    pub const MQCTES_NOSYNC: types::MQCTES = types::MQCTES(0);
    pub const MQCTES_COMMIT: types::MQCTES = types::MQCTES(256);
    pub const MQCTES_BACKOUT: types::MQCTES = types::MQCTES(4352);
    pub const MQCTES_ENDTASK: types::MQCTES = types::MQCTES(65536);
    pub const MQCTLO_NONE: types::MQCTLO = types::MQCTLO(0);
    pub const MQCTLO_THREAD_AFFINITY: types::MQCTLO = types::MQCTLO(1);
    pub const MQCTLO_FAIL_IF_QUIESCING: types::MQCTLO = types::MQCTLO(8192);
    pub const MQCUOWC_MIDDLE: types::MQCUOWC = types::MQCUOWC(16);
    pub const MQCUOWC_COMMIT: types::MQCUOWC = types::MQCUOWC(256);
    pub const MQCUOWC_ONLY: types::MQCUOWC = types::MQCUOWC(273);
    pub const MQCUOWC_BACKOUT: types::MQCUOWC = types::MQCUOWC(4352);
    pub const MQCUOWC_CONTINUE: types::MQCUOWC = types::MQCUOWC(65536);
    pub const MQCUOWC_FIRST: types::MQCUOWC = types::MQCUOWC(17);
    pub const MQCUOWC_LAST: types::MQCUOWC = types::MQCUOWC(272);
    pub const MQDC_MANAGED: types::MQDC = types::MQDC(1);
    pub const MQDC_PROVIDED: types::MQDC = types::MQDC(2);
    pub const MQDHF_NONE: types::MQDHF = types::MQDHF(0);
    pub const MQDHF_NEW_MSG_IDS: types::MQDHF = types::MQDHF(1);
    pub const MQDLV_AS_PARENT: types::MQDLV = types::MQDLV(0);
    pub const MQDLV_ALL: types::MQDLV = types::MQDLV(1);
    pub const MQDLV_ALL_DUR: types::MQDLV = types::MQDLV(2);
    pub const MQDLV_ALL_AVAIL: types::MQDLV = types::MQDLV(3);
    pub const MQDL_NOT_SUPPORTED: types::MQDL = types::MQDL(0);
    pub const MQDL_SUPPORTED: types::MQDL = types::MQDL(1);
    pub const MQDMHO_NONE: types::MQDMHO = types::MQDMHO(0);
    pub const MQDMPO_NONE: types::MQDMPO = types::MQDMPO(0);
    pub const MQDMPO_DEL_PROP_UNDER_CURSOR: types::MQDMPO = types::MQDMPO(1);
    pub const MQDMPO_DEL_FIRST: types::MQDMPO = types::MQDMPO(0);
    pub const MQDNSWLM_NO: types::MQDNSWLM = types::MQDNSWLM(0);
    pub const MQDNSWLM_YES: types::MQDNSWLM = types::MQDNSWLM(1);
    pub const MQDSB_DEFAULT: types::MQDSB = types::MQDSB(0);
    pub const MQDSB_8K: types::MQDSB = types::MQDSB(1);
    pub const MQDSB_16K: types::MQDSB = types::MQDSB(2);
    pub const MQDSB_32K: types::MQDSB = types::MQDSB(3);
    pub const MQDSB_64K: types::MQDSB = types::MQDSB(4);
    pub const MQDSB_128K: types::MQDSB = types::MQDSB(5);
    pub const MQDSB_256K: types::MQDSB = types::MQDSB(6);
    pub const MQDSB_512K: types::MQDSB = types::MQDSB(7);
    pub const MQDSB_1M: types::MQDSB = types::MQDSB(8);
    pub const MQDSB_1024K: types::MQDSB = types::MQDSB(8);
    pub const MQDSE_DEFAULT: types::MQDSE = types::MQDSE(0);
    pub const MQDSE_YES: types::MQDSE = types::MQDSE(1);
    pub const MQDSE_NO: types::MQDSE = types::MQDSE(2);
    pub const MQEC_MSG_ARRIVED: types::MQEC = types::MQEC(2);
    pub const MQEC_WAIT_INTERVAL_EXPIRED: types::MQEC = types::MQEC(3);
    pub const MQEC_WAIT_CANCELED: types::MQEC = types::MQEC(4);
    pub const MQEC_Q_MGR_QUIESCING: types::MQEC = types::MQEC(5);
    pub const MQEC_CONNECTION_QUIESCING: types::MQEC = types::MQEC(6);
    pub const MQEI_UNLIMITED: types::MQEI = types::MQEI(-1);
    pub const MQENC_RESERVED_MASK: types::MQENC = types::MQENC(-4096);
    pub const MQENC_AS_PUBLISHED: types::MQENC = types::MQENC(-1);
    pub const MQENC_INTEGER_NORMAL: types::MQENC = types::MQENC(1);
    pub const MQENC_INTEGER_REVERSED: types::MQENC = types::MQENC(2);
    pub const MQENC_INTEGER_MASK: types::MQENC = types::MQENC(15);
    pub const MQENC_DECIMAL_NORMAL: types::MQENC = types::MQENC(16);
    pub const MQENC_DECIMAL_REVERSED: types::MQENC = types::MQENC(32);
    pub const MQENC_DECIMAL_MASK: types::MQENC = types::MQENC(240);
    pub const MQENC_FLOAT_IEEE_NORMAL: types::MQENC = types::MQENC(256);
    pub const MQENC_NORMAL: types::MQENC = types::MQENC(273);
    pub const MQENC_FLOAT_IEEE_REVERSED: types::MQENC = types::MQENC(512);
    pub const MQENC_REVERSED: types::MQENC = types::MQENC(546);
    pub const MQENC_FLOAT_S390: types::MQENC = types::MQENC(768);
    pub const MQENC_S390: types::MQENC = types::MQENC(785);
    pub const MQENC_FLOAT_TNS: types::MQENC = types::MQENC(1024);
    pub const MQENC_TNS: types::MQENC = types::MQENC(1041);
    pub const MQENC_FLOAT_MASK: types::MQENC = types::MQENC(3840);
    pub const MQENC_DECIMAL_UNDEFINED: types::MQENC = types::MQENC(0);
    pub const MQENC_FLOAT_UNDEFINED: types::MQENC = types::MQENC(0);
    pub const MQENC_INTEGER_UNDEFINED: types::MQENC = types::MQENC(0);
    pub const MQENC_NATIVE: types::MQENC = types::MQENC(546);
    pub const MQEXPI_OFF: types::MQEXPI = types::MQEXPI(0);
    pub const MQFB_NONE: types::MQFB = types::MQFB(0);
    pub const MQFB_QUIT: types::MQFB = types::MQFB(256);
    pub const MQFB_EXPIRATION: types::MQFB = types::MQFB(258);
    pub const MQFB_COA: types::MQFB = types::MQFB(259);
    pub const MQFB_COD: types::MQFB = types::MQFB(260);
    pub const MQFB_CHANNEL_COMPLETED: types::MQFB = types::MQFB(262);
    pub const MQFB_CHANNEL_FAIL_RETRY: types::MQFB = types::MQFB(263);
    pub const MQFB_CHANNEL_FAIL: types::MQFB = types::MQFB(264);
    pub const MQFB_APPL_CANNOT_BE_STARTED: types::MQFB = types::MQFB(265);
    pub const MQFB_TM_ERROR: types::MQFB = types::MQFB(266);
    pub const MQFB_APPL_TYPE_ERROR: types::MQFB = types::MQFB(267);
    pub const MQFB_STOPPED_BY_MSG_EXIT: types::MQFB = types::MQFB(268);
    pub const MQFB_ACTIVITY: types::MQFB = types::MQFB(269);
    pub const MQFB_XMIT_Q_MSG_ERROR: types::MQFB = types::MQFB(271);
    pub const MQFB_PAN: types::MQFB = types::MQFB(275);
    pub const MQFB_NAN: types::MQFB = types::MQFB(276);
    pub const MQFB_STOPPED_BY_CHAD_EXIT: types::MQFB = types::MQFB(277);
    pub const MQFB_STOPPED_BY_PUBSUB_EXIT: types::MQFB = types::MQFB(279);
    pub const MQFB_NOT_A_REPOSITORY_MSG: types::MQFB = types::MQFB(280);
    pub const MQFB_BIND_OPEN_CLUSRCVR_DEL: types::MQFB = types::MQFB(281);
    pub const MQFB_MAX_ACTIVITIES: types::MQFB = types::MQFB(282);
    pub const MQFB_NOT_FORWARDED: types::MQFB = types::MQFB(283);
    pub const MQFB_NOT_DELIVERED: types::MQFB = types::MQFB(284);
    pub const MQFB_UNSUPPORTED_FORWARDING: types::MQFB = types::MQFB(285);
    pub const MQFB_UNSUPPORTED_DELIVERY: types::MQFB = types::MQFB(286);
    pub const MQFB_DATA_LENGTH_ZERO: types::MQFB = types::MQFB(291);
    pub const MQFB_DATA_LENGTH_NEGATIVE: types::MQFB = types::MQFB(292);
    pub const MQFB_DATA_LENGTH_TOO_BIG: types::MQFB = types::MQFB(293);
    pub const MQFB_BUFFER_OVERFLOW: types::MQFB = types::MQFB(294);
    pub const MQFB_LENGTH_OFF_BY_ONE: types::MQFB = types::MQFB(295);
    pub const MQFB_IIH_ERROR: types::MQFB = types::MQFB(296);
    pub const MQFB_NOT_AUTHORIZED_FOR_IMS: types::MQFB = types::MQFB(298);
    pub const MQFB_DATA_LENGTH_TOO_SHORT: types::MQFB = types::MQFB(299);
    pub const MQFB_IMS_ERROR: types::MQFB = types::MQFB(300);
    pub const MQFB_CICS_INTERNAL_ERROR: types::MQFB = types::MQFB(401);
    pub const MQFB_CICS_NOT_AUTHORIZED: types::MQFB = types::MQFB(402);
    pub const MQFB_CICS_BRIDGE_FAILURE: types::MQFB = types::MQFB(403);
    pub const MQFB_CICS_CORREL_ID_ERROR: types::MQFB = types::MQFB(404);
    pub const MQFB_CICS_CCSID_ERROR: types::MQFB = types::MQFB(405);
    pub const MQFB_CICS_ENCODING_ERROR: types::MQFB = types::MQFB(406);
    pub const MQFB_CICS_CIH_ERROR: types::MQFB = types::MQFB(407);
    pub const MQFB_CICS_UOW_ERROR: types::MQFB = types::MQFB(408);
    pub const MQFB_CICS_COMMAREA_ERROR: types::MQFB = types::MQFB(409);
    pub const MQFB_CICS_APPL_NOT_STARTED: types::MQFB = types::MQFB(410);
    pub const MQFB_CICS_APPL_ABENDED: types::MQFB = types::MQFB(411);
    pub const MQFB_CICS_DLQ_ERROR: types::MQFB = types::MQFB(412);
    pub const MQFB_CICS_UOW_BACKED_OUT: types::MQFB = types::MQFB(413);
    pub const MQFB_PUBLICATIONS_ON_REQUEST: types::MQFB = types::MQFB(501);
    pub const MQFB_SUBSCRIBER_IS_PUBLISHER: types::MQFB = types::MQFB(502);
    pub const MQFB_MSG_SCOPE_MISMATCH: types::MQFB = types::MQFB(503);
    pub const MQFB_SELECTOR_MISMATCH: types::MQFB = types::MQFB(504);
    pub const MQFB_NOT_A_GROUPUR_MSG: types::MQFB = types::MQFB(505);
    pub const MQFB_SYSTEM_FIRST: types::MQFB = types::MQFB(1);
    pub const MQFB_IMS_FIRST: types::MQFB = types::MQFB(301);
    pub const MQFB_IMS_LAST: types::MQFB = types::MQFB(399);
    pub const MQFB_IMS_NACK_1A_REASON_FIRST: types::MQFB = types::MQFB(600);
    pub const MQFB_IMS_NACK_1A_REASON_LAST: types::MQFB = types::MQFB(855);
    pub const MQFB_SYSTEM_LAST: types::MQFB = types::MQFB(65535);
    pub const MQFB_APPL_FIRST: types::MQFB = types::MQFB(65536);
    pub const MQFB_APPL_LAST: types::MQFB = types::MQFB(999999999);
    pub const MQFIELD_WQR_StrucId: types::MQFIELD_WQR = types::MQFIELD_WQR(8000);
    pub const MQFIELD_WQR_Version: types::MQFIELD_WQR = types::MQFIELD_WQR(8001);
    pub const MQFIELD_WQR_StrucLength: types::MQFIELD_WQR = types::MQFIELD_WQR(8002);
    pub const MQFIELD_WQR_QFlags: types::MQFIELD_WQR = types::MQFIELD_WQR(8003);
    pub const MQFIELD_WQR_QName: types::MQFIELD_WQR = types::MQFIELD_WQR(8004);
    pub const MQFIELD_WQR_QMgrIdentifier: types::MQFIELD_WQR = types::MQFIELD_WQR(8005);
    pub const MQFIELD_WQR_ClusterRecOffset: types::MQFIELD_WQR = types::MQFIELD_WQR(
        8006,
    );
    pub const MQFIELD_WQR_QType: types::MQFIELD_WQR = types::MQFIELD_WQR(8007);
    pub const MQFIELD_WQR_QDesc: types::MQFIELD_WQR = types::MQFIELD_WQR(8008);
    pub const MQFIELD_WQR_DefBind: types::MQFIELD_WQR = types::MQFIELD_WQR(8009);
    pub const MQFIELD_WQR_DefPersistence: types::MQFIELD_WQR = types::MQFIELD_WQR(8010);
    pub const MQFIELD_WQR_DefPriority: types::MQFIELD_WQR = types::MQFIELD_WQR(8011);
    pub const MQFIELD_WQR_InhibitPut: types::MQFIELD_WQR = types::MQFIELD_WQR(8012);
    pub const MQFIELD_WQR_CLWLQueuePriority: types::MQFIELD_WQR = types::MQFIELD_WQR(
        8013,
    );
    pub const MQFIELD_WQR_CLWLQueueRank: types::MQFIELD_WQR = types::MQFIELD_WQR(8014);
    pub const MQFIELD_WQR_DefPutResponse: types::MQFIELD_WQR = types::MQFIELD_WQR(8015);
    pub const MQFIELD_WQR_CapExpiry: types::MQFIELD_WQR = types::MQFIELD_WQR(8016);
    pub const MQFUN_TYPE_UNKNOWN: types::MQFUN = types::MQFUN(0);
    pub const MQFUN_TYPE_JVM: types::MQFUN = types::MQFUN(1);
    pub const MQFUN_TYPE_PROGRAM: types::MQFUN = types::MQFUN(2);
    pub const MQFUN_TYPE_PROCEDURE: types::MQFUN = types::MQFUN(3);
    pub const MQFUN_TYPE_USERDEF: types::MQFUN = types::MQFUN(4);
    pub const MQFUN_TYPE_COMMAND: types::MQFUN = types::MQFUN(5);
    pub const MQGMO_NONE: types::MQGMO = types::MQGMO(0);
    pub const MQGMO_WAIT: types::MQGMO = types::MQGMO(1);
    pub const MQGMO_SYNCPOINT: types::MQGMO = types::MQGMO(2);
    pub const MQGMO_NO_SYNCPOINT: types::MQGMO = types::MQGMO(4);
    pub const MQGMO_SET_SIGNAL: types::MQGMO = types::MQGMO(8);
    pub const MQGMO_BROWSE_FIRST: types::MQGMO = types::MQGMO(16);
    pub const MQGMO_BROWSE_NEXT: types::MQGMO = types::MQGMO(32);
    pub const MQGMO_ACCEPT_TRUNCATED_MSG: types::MQGMO = types::MQGMO(64);
    pub const MQGMO_MARK_SKIP_BACKOUT: types::MQGMO = types::MQGMO(128);
    pub const MQGMO_MSG_UNDER_CURSOR: types::MQGMO = types::MQGMO(256);
    pub const MQGMO_LOCK: types::MQGMO = types::MQGMO(512);
    pub const MQGMO_UNLOCK: types::MQGMO = types::MQGMO(1024);
    pub const MQGMO_BROWSE_MSG_UNDER_CURSOR: types::MQGMO = types::MQGMO(2048);
    pub const MQGMO_SYNCPOINT_IF_PERSISTENT: types::MQGMO = types::MQGMO(4096);
    pub const MQGMO_FAIL_IF_QUIESCING: types::MQGMO = types::MQGMO(8192);
    pub const MQGMO_CONVERT: types::MQGMO = types::MQGMO(16384);
    pub const MQGMO_LOGICAL_ORDER: types::MQGMO = types::MQGMO(32768);
    pub const MQGMO_COMPLETE_MSG: types::MQGMO = types::MQGMO(65536);
    pub const MQGMO_ALL_MSGS_AVAILABLE: types::MQGMO = types::MQGMO(131072);
    pub const MQGMO_ALL_SEGMENTS_AVAILABLE: types::MQGMO = types::MQGMO(262144);
    pub const MQGMO_MARK_BROWSE_HANDLE: types::MQGMO = types::MQGMO(1048576);
    pub const MQGMO_MARK_BROWSE_CO_OP: types::MQGMO = types::MQGMO(2097152);
    pub const MQGMO_UNMARK_BROWSE_CO_OP: types::MQGMO = types::MQGMO(4194304);
    pub const MQGMO_UNMARK_BROWSE_HANDLE: types::MQGMO = types::MQGMO(8388608);
    pub const MQGMO_UNMARKED_BROWSE_MSG: types::MQGMO = types::MQGMO(16777216);
    pub const MQGMO_BROWSE_HANDLE: types::MQGMO = types::MQGMO(17825808);
    pub const MQGMO_BROWSE_CO_OP: types::MQGMO = types::MQGMO(18874384);
    pub const MQGMO_PROPERTIES_FORCE_MQRFH2: types::MQGMO = types::MQGMO(33554432);
    pub const MQGMO_NO_PROPERTIES: types::MQGMO = types::MQGMO(67108864);
    pub const MQGMO_PROPERTIES_IN_HANDLE: types::MQGMO = types::MQGMO(134217728);
    pub const MQGMO_PROPERTIES_COMPATIBILITY: types::MQGMO = types::MQGMO(268435456);
    pub const MQGMO_NO_WAIT: types::MQGMO = types::MQGMO(0);
    pub const MQGMO_PROPERTIES_AS_Q_DEF: types::MQGMO = types::MQGMO(0);
    pub const MQHC_UNASSOCIATED_HCONN: types::MQHC = types::MQHC(-3);
    pub const MQHC_UNUSABLE_HCONN: types::MQHC = types::MQHC(-1);
    pub const MQHC_DEF_HCONN: types::MQHC = types::MQHC(0);
    pub const MQHM_UNUSABLE_HMSG: types::MQHM = types::MQHM(-1);
    pub const MQHM_NONE: types::MQHM = types::MQHM(0);
    pub const MQHO_UNUSABLE_HOBJ: types::MQHO = types::MQHO(-1);
    pub const MQHO_NONE: types::MQHO = types::MQHO(0);
    pub const MQIAV_UNDEFINED: types::MQIAV = types::MQIAV(-2);
    pub const MQIAV_NOT_APPLICABLE: types::MQIAV = types::MQIAV(-1);
    pub const MQIA_APPL_TYPE: types::MQIA = types::MQIA(1);
    pub const MQIA_CODED_CHAR_SET_ID: types::MQIA = types::MQIA(2);
    pub const MQIA_CURRENT_Q_DEPTH: types::MQIA = types::MQIA(3);
    pub const MQIA_DEF_INPUT_OPEN_OPTION: types::MQIA = types::MQIA(4);
    pub const MQIA_DEF_PERSISTENCE: types::MQIA = types::MQIA(5);
    pub const MQIA_DEF_PRIORITY: types::MQIA = types::MQIA(6);
    pub const MQIA_DEFINITION_TYPE: types::MQIA = types::MQIA(7);
    pub const MQIA_HARDEN_GET_BACKOUT: types::MQIA = types::MQIA(8);
    pub const MQIA_INHIBIT_GET: types::MQIA = types::MQIA(9);
    pub const MQIA_INHIBIT_PUT: types::MQIA = types::MQIA(10);
    pub const MQIA_MAX_HANDLES: types::MQIA = types::MQIA(11);
    pub const MQIA_USAGE: types::MQIA = types::MQIA(12);
    pub const MQIA_MAX_MSG_LENGTH: types::MQIA = types::MQIA(13);
    pub const MQIA_MAX_PRIORITY: types::MQIA = types::MQIA(14);
    pub const MQIA_MAX_Q_DEPTH: types::MQIA = types::MQIA(15);
    pub const MQIA_MSG_DELIVERY_SEQUENCE: types::MQIA = types::MQIA(16);
    pub const MQIA_OPEN_INPUT_COUNT: types::MQIA = types::MQIA(17);
    pub const MQIA_OPEN_OUTPUT_COUNT: types::MQIA = types::MQIA(18);
    pub const MQIA_NAME_COUNT: types::MQIA = types::MQIA(19);
    pub const MQIA_Q_TYPE: types::MQIA = types::MQIA(20);
    pub const MQIA_RETENTION_INTERVAL: types::MQIA = types::MQIA(21);
    pub const MQIA_BACKOUT_THRESHOLD: types::MQIA = types::MQIA(22);
    pub const MQIA_SHAREABILITY: types::MQIA = types::MQIA(23);
    pub const MQIA_TRIGGER_CONTROL: types::MQIA = types::MQIA(24);
    pub const MQIA_TRIGGER_INTERVAL: types::MQIA = types::MQIA(25);
    pub const MQIA_TRIGGER_MSG_PRIORITY: types::MQIA = types::MQIA(26);
    pub const MQIA_CPI_LEVEL: types::MQIA = types::MQIA(27);
    pub const MQIA_TRIGGER_TYPE: types::MQIA = types::MQIA(28);
    pub const MQIA_TRIGGER_DEPTH: types::MQIA = types::MQIA(29);
    pub const MQIA_SYNCPOINT: types::MQIA = types::MQIA(30);
    pub const MQIA_COMMAND_LEVEL: types::MQIA = types::MQIA(31);
    pub const MQIA_PLATFORM: types::MQIA = types::MQIA(32);
    pub const MQIA_MAX_UNCOMMITTED_MSGS: types::MQIA = types::MQIA(33);
    pub const MQIA_DIST_LISTS: types::MQIA = types::MQIA(34);
    pub const MQIA_TIME_SINCE_RESET: types::MQIA = types::MQIA(35);
    pub const MQIA_HIGH_Q_DEPTH: types::MQIA = types::MQIA(36);
    pub const MQIA_MSG_ENQ_COUNT: types::MQIA = types::MQIA(37);
    pub const MQIA_MSG_DEQ_COUNT: types::MQIA = types::MQIA(38);
    pub const MQIA_EXPIRY_INTERVAL: types::MQIA = types::MQIA(39);
    pub const MQIA_Q_DEPTH_HIGH_LIMIT: types::MQIA = types::MQIA(40);
    pub const MQIA_Q_DEPTH_LOW_LIMIT: types::MQIA = types::MQIA(41);
    pub const MQIA_Q_DEPTH_MAX_EVENT: types::MQIA = types::MQIA(42);
    pub const MQIA_Q_DEPTH_HIGH_EVENT: types::MQIA = types::MQIA(43);
    pub const MQIA_Q_DEPTH_LOW_EVENT: types::MQIA = types::MQIA(44);
    pub const MQIA_SCOPE: types::MQIA = types::MQIA(45);
    pub const MQIA_Q_SERVICE_INTERVAL_EVENT: types::MQIA = types::MQIA(46);
    pub const MQIA_AUTHORITY_EVENT: types::MQIA = types::MQIA(47);
    pub const MQIA_INHIBIT_EVENT: types::MQIA = types::MQIA(48);
    pub const MQIA_LOCAL_EVENT: types::MQIA = types::MQIA(49);
    pub const MQIA_REMOTE_EVENT: types::MQIA = types::MQIA(50);
    pub const MQIA_CONFIGURATION_EVENT: types::MQIA = types::MQIA(51);
    pub const MQIA_START_STOP_EVENT: types::MQIA = types::MQIA(52);
    pub const MQIA_PERFORMANCE_EVENT: types::MQIA = types::MQIA(53);
    pub const MQIA_Q_SERVICE_INTERVAL: types::MQIA = types::MQIA(54);
    pub const MQIA_CHANNEL_AUTO_DEF: types::MQIA = types::MQIA(55);
    pub const MQIA_CHANNEL_AUTO_DEF_EVENT: types::MQIA = types::MQIA(56);
    pub const MQIA_INDEX_TYPE: types::MQIA = types::MQIA(57);
    pub const MQIA_CLUSTER_WORKLOAD_LENGTH: types::MQIA = types::MQIA(58);
    pub const MQIA_CLUSTER_Q_TYPE: types::MQIA = types::MQIA(59);
    pub const MQIA_ARCHIVE: types::MQIA = types::MQIA(60);
    pub const MQIA_DEF_BIND: types::MQIA = types::MQIA(61);
    pub const MQIA_PAGESET_ID: types::MQIA = types::MQIA(62);
    pub const MQIA_QSG_DISP: types::MQIA = types::MQIA(63);
    pub const MQIA_INTRA_GROUP_QUEUING: types::MQIA = types::MQIA(64);
    pub const MQIA_IGQ_PUT_AUTHORITY: types::MQIA = types::MQIA(65);
    pub const MQIA_AUTH_INFO_TYPE: types::MQIA = types::MQIA(66);
    pub const MQIA_MSG_MARK_BROWSE_INTERVAL: types::MQIA = types::MQIA(68);
    pub const MQIA_SSL_TASKS: types::MQIA = types::MQIA(69);
    pub const MQIA_CF_LEVEL: types::MQIA = types::MQIA(70);
    pub const MQIA_CF_RECOVER: types::MQIA = types::MQIA(71);
    pub const MQIA_NAMELIST_TYPE: types::MQIA = types::MQIA(72);
    pub const MQIA_CHANNEL_EVENT: types::MQIA = types::MQIA(73);
    pub const MQIA_BRIDGE_EVENT: types::MQIA = types::MQIA(74);
    pub const MQIA_SSL_EVENT: types::MQIA = types::MQIA(75);
    pub const MQIA_SSL_RESET_COUNT: types::MQIA = types::MQIA(76);
    pub const MQIA_SHARED_Q_Q_MGR_NAME: types::MQIA = types::MQIA(77);
    pub const MQIA_NPM_CLASS: types::MQIA = types::MQIA(78);
    pub const MQIA_MAX_OPEN_Q: types::MQIA = types::MQIA(80);
    pub const MQIA_MONITOR_INTERVAL: types::MQIA = types::MQIA(81);
    pub const MQIA_Q_USERS: types::MQIA = types::MQIA(82);
    pub const MQIA_MAX_GLOBAL_LOCKS: types::MQIA = types::MQIA(83);
    pub const MQIA_MAX_LOCAL_LOCKS: types::MQIA = types::MQIA(84);
    pub const MQIA_LISTENER_PORT_NUMBER: types::MQIA = types::MQIA(85);
    pub const MQIA_BATCH_INTERFACE_AUTO: types::MQIA = types::MQIA(86);
    pub const MQIA_CMD_SERVER_AUTO: types::MQIA = types::MQIA(87);
    pub const MQIA_CMD_SERVER_CONVERT_MSG: types::MQIA = types::MQIA(88);
    pub const MQIA_CMD_SERVER_DLQ_MSG: types::MQIA = types::MQIA(89);
    pub const MQIA_MAX_Q_TRIGGERS: types::MQIA = types::MQIA(90);
    pub const MQIA_TRIGGER_RESTART: types::MQIA = types::MQIA(91);
    pub const MQIA_SSL_FIPS_REQUIRED: types::MQIA = types::MQIA(92);
    pub const MQIA_IP_ADDRESS_VERSION: types::MQIA = types::MQIA(93);
    pub const MQIA_LOGGER_EVENT: types::MQIA = types::MQIA(94);
    pub const MQIA_CLWL_Q_RANK: types::MQIA = types::MQIA(95);
    pub const MQIA_CLWL_Q_PRIORITY: types::MQIA = types::MQIA(96);
    pub const MQIA_CLWL_MRU_CHANNELS: types::MQIA = types::MQIA(97);
    pub const MQIA_CLWL_USEQ: types::MQIA = types::MQIA(98);
    pub const MQIA_COMMAND_EVENT: types::MQIA = types::MQIA(99);
    pub const MQIA_ACTIVE_CHANNELS: types::MQIA = types::MQIA(100);
    pub const MQIA_CHINIT_ADAPTERS: types::MQIA = types::MQIA(101);
    pub const MQIA_ADOPTNEWMCA_CHECK: types::MQIA = types::MQIA(102);
    pub const MQIA_ADOPTNEWMCA_TYPE: types::MQIA = types::MQIA(103);
    pub const MQIA_ADOPTNEWMCA_INTERVAL: types::MQIA = types::MQIA(104);
    pub const MQIA_CHINIT_DISPATCHERS: types::MQIA = types::MQIA(105);
    pub const MQIA_DNS_WLM: types::MQIA = types::MQIA(106);
    pub const MQIA_LISTENER_TIMER: types::MQIA = types::MQIA(107);
    pub const MQIA_LU62_CHANNELS: types::MQIA = types::MQIA(108);
    pub const MQIA_MAX_CHANNELS: types::MQIA = types::MQIA(109);
    pub const MQIA_OUTBOUND_PORT_MIN: types::MQIA = types::MQIA(110);
    pub const MQIA_RECEIVE_TIMEOUT: types::MQIA = types::MQIA(111);
    pub const MQIA_RECEIVE_TIMEOUT_TYPE: types::MQIA = types::MQIA(112);
    pub const MQIA_RECEIVE_TIMEOUT_MIN: types::MQIA = types::MQIA(113);
    pub const MQIA_TCP_CHANNELS: types::MQIA = types::MQIA(114);
    pub const MQIA_TCP_KEEP_ALIVE: types::MQIA = types::MQIA(115);
    pub const MQIA_TCP_STACK_TYPE: types::MQIA = types::MQIA(116);
    pub const MQIA_CHINIT_TRACE_AUTO_START: types::MQIA = types::MQIA(117);
    pub const MQIA_CHINIT_TRACE_TABLE_SIZE: types::MQIA = types::MQIA(118);
    pub const MQIA_CHINIT_CONTROL: types::MQIA = types::MQIA(119);
    pub const MQIA_CMD_SERVER_CONTROL: types::MQIA = types::MQIA(120);
    pub const MQIA_SERVICE_TYPE: types::MQIA = types::MQIA(121);
    pub const MQIA_MONITORING_CHANNEL: types::MQIA = types::MQIA(122);
    pub const MQIA_MONITORING_Q: types::MQIA = types::MQIA(123);
    pub const MQIA_MONITORING_AUTO_CLUSSDR: types::MQIA = types::MQIA(124);
    pub const MQIA_STATISTICS_MQI: types::MQIA = types::MQIA(127);
    pub const MQIA_STATISTICS_Q: types::MQIA = types::MQIA(128);
    pub const MQIA_STATISTICS_CHANNEL: types::MQIA = types::MQIA(129);
    pub const MQIA_STATISTICS_AUTO_CLUSSDR: types::MQIA = types::MQIA(130);
    pub const MQIA_STATISTICS_INTERVAL: types::MQIA = types::MQIA(131);
    pub const MQIA_ACCOUNTING_MQI: types::MQIA = types::MQIA(133);
    pub const MQIA_ACCOUNTING_Q: types::MQIA = types::MQIA(134);
    pub const MQIA_ACCOUNTING_INTERVAL: types::MQIA = types::MQIA(135);
    pub const MQIA_ACCOUNTING_CONN_OVERRIDE: types::MQIA = types::MQIA(136);
    pub const MQIA_TRACE_ROUTE_RECORDING: types::MQIA = types::MQIA(137);
    pub const MQIA_ACTIVITY_RECORDING: types::MQIA = types::MQIA(138);
    pub const MQIA_SERVICE_CONTROL: types::MQIA = types::MQIA(139);
    pub const MQIA_OUTBOUND_PORT_MAX: types::MQIA = types::MQIA(140);
    pub const MQIA_SECURITY_CASE: types::MQIA = types::MQIA(141);
    pub const MQIA_QMOPT_CSMT_ON_ERROR: types::MQIA = types::MQIA(150);
    pub const MQIA_QMOPT_CONS_INFO_MSGS: types::MQIA = types::MQIA(151);
    pub const MQIA_QMOPT_CONS_WARNING_MSGS: types::MQIA = types::MQIA(152);
    pub const MQIA_QMOPT_CONS_ERROR_MSGS: types::MQIA = types::MQIA(153);
    pub const MQIA_QMOPT_CONS_CRITICAL_MSGS: types::MQIA = types::MQIA(154);
    pub const MQIA_QMOPT_CONS_COMMS_MSGS: types::MQIA = types::MQIA(155);
    pub const MQIA_QMOPT_CONS_REORG_MSGS: types::MQIA = types::MQIA(156);
    pub const MQIA_QMOPT_CONS_SYSTEM_MSGS: types::MQIA = types::MQIA(157);
    pub const MQIA_QMOPT_LOG_INFO_MSGS: types::MQIA = types::MQIA(158);
    pub const MQIA_QMOPT_LOG_WARNING_MSGS: types::MQIA = types::MQIA(159);
    pub const MQIA_QMOPT_LOG_ERROR_MSGS: types::MQIA = types::MQIA(160);
    pub const MQIA_QMOPT_LOG_CRITICAL_MSGS: types::MQIA = types::MQIA(161);
    pub const MQIA_QMOPT_LOG_COMMS_MSGS: types::MQIA = types::MQIA(162);
    pub const MQIA_QMOPT_LOG_REORG_MSGS: types::MQIA = types::MQIA(163);
    pub const MQIA_QMOPT_LOG_SYSTEM_MSGS: types::MQIA = types::MQIA(164);
    pub const MQIA_QMOPT_TRACE_MQI_CALLS: types::MQIA = types::MQIA(165);
    pub const MQIA_QMOPT_TRACE_COMMS: types::MQIA = types::MQIA(166);
    pub const MQIA_QMOPT_TRACE_REORG: types::MQIA = types::MQIA(167);
    pub const MQIA_QMOPT_TRACE_CONVERSION: types::MQIA = types::MQIA(168);
    pub const MQIA_QMOPT_TRACE_SYSTEM: types::MQIA = types::MQIA(169);
    pub const MQIA_QMOPT_INTERNAL_DUMP: types::MQIA = types::MQIA(170);
    pub const MQIA_MAX_RECOVERY_TASKS: types::MQIA = types::MQIA(171);
    pub const MQIA_MAX_CLIENTS: types::MQIA = types::MQIA(172);
    pub const MQIA_AUTO_REORGANIZATION: types::MQIA = types::MQIA(173);
    pub const MQIA_AUTO_REORG_INTERVAL: types::MQIA = types::MQIA(174);
    pub const MQIA_DURABLE_SUB: types::MQIA = types::MQIA(175);
    pub const MQIA_MULTICAST: types::MQIA = types::MQIA(176);
    pub const MQIA_INHIBIT_PUB: types::MQIA = types::MQIA(181);
    pub const MQIA_INHIBIT_SUB: types::MQIA = types::MQIA(182);
    pub const MQIA_TREE_LIFE_TIME: types::MQIA = types::MQIA(183);
    pub const MQIA_DEF_PUT_RESPONSE_TYPE: types::MQIA = types::MQIA(184);
    pub const MQIA_TOPIC_DEF_PERSISTENCE: types::MQIA = types::MQIA(185);
    pub const MQIA_MASTER_ADMIN: types::MQIA = types::MQIA(186);
    pub const MQIA_PUBSUB_MODE: types::MQIA = types::MQIA(187);
    pub const MQIA_DEF_READ_AHEAD: types::MQIA = types::MQIA(188);
    pub const MQIA_READ_AHEAD: types::MQIA = types::MQIA(189);
    pub const MQIA_PROPERTY_CONTROL: types::MQIA = types::MQIA(190);
    pub const MQIA_MAX_PROPERTIES_LENGTH: types::MQIA = types::MQIA(192);
    pub const MQIA_BASE_TYPE: types::MQIA = types::MQIA(193);
    pub const MQIA_PM_DELIVERY: types::MQIA = types::MQIA(195);
    pub const MQIA_NPM_DELIVERY: types::MQIA = types::MQIA(196);
    pub const MQIA_PROXY_SUB: types::MQIA = types::MQIA(199);
    pub const MQIA_PUBSUB_NP_MSG: types::MQIA = types::MQIA(203);
    pub const MQIA_SUB_COUNT: types::MQIA = types::MQIA(204);
    pub const MQIA_PUBSUB_NP_RESP: types::MQIA = types::MQIA(205);
    pub const MQIA_PUBSUB_MAXMSG_RETRY_COUNT: types::MQIA = types::MQIA(206);
    pub const MQIA_PUBSUB_SYNC_PT: types::MQIA = types::MQIA(207);
    pub const MQIA_TOPIC_TYPE: types::MQIA = types::MQIA(208);
    pub const MQIA_PUB_COUNT: types::MQIA = types::MQIA(215);
    pub const MQIA_WILDCARD_OPERATION: types::MQIA = types::MQIA(216);
    pub const MQIA_SUB_SCOPE: types::MQIA = types::MQIA(218);
    pub const MQIA_PUB_SCOPE: types::MQIA = types::MQIA(219);
    pub const MQIA_GROUP_UR: types::MQIA = types::MQIA(221);
    pub const MQIA_UR_DISP: types::MQIA = types::MQIA(222);
    pub const MQIA_COMM_INFO_TYPE: types::MQIA = types::MQIA(223);
    pub const MQIA_CF_OFFLOAD: types::MQIA = types::MQIA(224);
    pub const MQIA_CF_OFFLOAD_THRESHOLD1: types::MQIA = types::MQIA(225);
    pub const MQIA_CF_OFFLOAD_THRESHOLD2: types::MQIA = types::MQIA(226);
    pub const MQIA_CF_OFFLOAD_THRESHOLD3: types::MQIA = types::MQIA(227);
    pub const MQIA_CF_SMDS_BUFFERS: types::MQIA = types::MQIA(228);
    pub const MQIA_CF_OFFLDUSE: types::MQIA = types::MQIA(229);
    pub const MQIA_MAX_RESPONSES: types::MQIA = types::MQIA(230);
    pub const MQIA_RESPONSE_RESTART_POINT: types::MQIA = types::MQIA(231);
    pub const MQIA_COMM_EVENT: types::MQIA = types::MQIA(232);
    pub const MQIA_MCAST_BRIDGE: types::MQIA = types::MQIA(233);
    pub const MQIA_USE_DEAD_LETTER_Q: types::MQIA = types::MQIA(234);
    pub const MQIA_TOLERATE_UNPROTECTED: types::MQIA = types::MQIA(235);
    pub const MQIA_SIGNATURE_ALGORITHM: types::MQIA = types::MQIA(236);
    pub const MQIA_ENCRYPTION_ALGORITHM: types::MQIA = types::MQIA(237);
    pub const MQIA_POLICY_VERSION: types::MQIA = types::MQIA(238);
    pub const MQIA_ACTIVITY_CONN_OVERRIDE: types::MQIA = types::MQIA(239);
    pub const MQIA_ACTIVITY_TRACE: types::MQIA = types::MQIA(240);
    pub const MQIA_SUB_CONFIGURATION_EVENT: types::MQIA = types::MQIA(242);
    pub const MQIA_XR_CAPABILITY: types::MQIA = types::MQIA(243);
    pub const MQIA_CF_RECAUTO: types::MQIA = types::MQIA(244);
    pub const MQIA_QMGR_CFCONLOS: types::MQIA = types::MQIA(245);
    pub const MQIA_CF_CFCONLOS: types::MQIA = types::MQIA(246);
    pub const MQIA_SUITE_B_STRENGTH: types::MQIA = types::MQIA(247);
    pub const MQIA_CHLAUTH_RECORDS: types::MQIA = types::MQIA(248);
    pub const MQIA_PUBSUB_CLUSTER: types::MQIA = types::MQIA(249);
    pub const MQIA_DEF_CLUSTER_XMIT_Q_TYPE: types::MQIA = types::MQIA(250);
    pub const MQIA_PROT_POLICY_CAPABILITY: types::MQIA = types::MQIA(251);
    pub const MQIA_CERT_VAL_POLICY: types::MQIA = types::MQIA(252);
    pub const MQIA_TOPIC_NODE_COUNT: types::MQIA = types::MQIA(253);
    pub const MQIA_REVERSE_DNS_LOOKUP: types::MQIA = types::MQIA(254);
    pub const MQIA_CLUSTER_PUB_ROUTE: types::MQIA = types::MQIA(255);
    pub const MQIA_CLUSTER_OBJECT_STATE: types::MQIA = types::MQIA(256);
    pub const MQIA_CHECK_LOCAL_BINDING: types::MQIA = types::MQIA(257);
    pub const MQIA_CHECK_CLIENT_BINDING: types::MQIA = types::MQIA(258);
    pub const MQIA_AUTHENTICATION_FAIL_DELAY: types::MQIA = types::MQIA(259);
    pub const MQIA_ADOPT_CONTEXT: types::MQIA = types::MQIA(260);
    pub const MQIA_LDAP_SECURE_COMM: types::MQIA = types::MQIA(261);
    pub const MQIA_DISPLAY_TYPE: types::MQIA = types::MQIA(262);
    pub const MQIA_LDAP_AUTHORMD: types::MQIA = types::MQIA(263);
    pub const MQIA_LDAP_NESTGRP: types::MQIA = types::MQIA(264);
    pub const MQIA_AMQP_CAPABILITY: types::MQIA = types::MQIA(265);
    pub const MQIA_AUTHENTICATION_METHOD: types::MQIA = types::MQIA(266);
    pub const MQIA_KEY_REUSE_COUNT: types::MQIA = types::MQIA(267);
    pub const MQIA_MEDIA_IMAGE_SCHEDULING: types::MQIA = types::MQIA(268);
    pub const MQIA_MEDIA_IMAGE_INTERVAL: types::MQIA = types::MQIA(269);
    pub const MQIA_MEDIA_IMAGE_LOG_LENGTH: types::MQIA = types::MQIA(270);
    pub const MQIA_MEDIA_IMAGE_RECOVER_OBJ: types::MQIA = types::MQIA(271);
    pub const MQIA_MEDIA_IMAGE_RECOVER_Q: types::MQIA = types::MQIA(272);
    pub const MQIA_ADVANCED_CAPABILITY: types::MQIA = types::MQIA(273);
    pub const MQIA_MAX_Q_FILE_SIZE: types::MQIA = types::MQIA(274);
    pub const MQIA_STREAM_QUEUE_QOS: types::MQIA = types::MQIA(275);
    pub const MQIA_CAP_EXPIRY: types::MQIA = types::MQIA(276);
    pub const MQIA_AUTHOREV_SCOPE: types::MQIA = types::MQIA(277);
    pub const MQIA_OTEL_TRACE: types::MQIA = types::MQIA(278);
    pub const MQIA_OTEL_PROPAGATION_CONTROL: types::MQIA = types::MQIA(279);
    pub const MQIA_USER_LIST: types::MQIA = types::MQIA(2000);
    pub const MQIA_FIRST: types::MQIA = types::MQIA(1);
    pub const MQIA_LAST_USED: types::MQIA = types::MQIA(279);
    pub const MQIA_LAST: types::MQIA = types::MQIA(2000);
    pub const MQIGQPA_DEFAULT: types::MQIGQPA = types::MQIGQPA(1);
    pub const MQIGQPA_CONTEXT: types::MQIGQPA = types::MQIGQPA(2);
    pub const MQIGQPA_ONLY_IGQ: types::MQIGQPA = types::MQIGQPA(3);
    pub const MQIGQPA_ALTERNATE_OR_IGQ: types::MQIGQPA = types::MQIGQPA(4);
    pub const MQIGQ_DISABLED: types::MQIGQ = types::MQIGQ(0);
    pub const MQIGQ_ENABLED: types::MQIGQ = types::MQIGQ(1);
    pub const MQIIH_NONE: types::MQIIH = types::MQIIH(0);
    pub const MQIIH_PASS_EXPIRATION: types::MQIIH = types::MQIIH(1);
    pub const MQIIH_REPLY_FORMAT_NONE: types::MQIIH = types::MQIIH(8);
    pub const MQIIH_IGNORE_PURG: types::MQIIH = types::MQIIH(16);
    pub const MQIIH_CM0_REQUEST_RESPONSE: types::MQIIH = types::MQIIH(32);
    pub const MQIIH_UNLIMITED_EXPIRATION: types::MQIIH = types::MQIIH(0);
    pub const MQIMGRCOV_NO: types::MQIMGRCOV = types::MQIMGRCOV(0);
    pub const MQIMGRCOV_YES: types::MQIMGRCOV = types::MQIMGRCOV(1);
    pub const MQIMGRCOV_AS_Q_MGR: types::MQIMGRCOV = types::MQIMGRCOV(2);
    pub const MQIMPO_NONE: types::MQIMPO = types::MQIMPO(0);
    pub const MQIMPO_CONVERT_TYPE: types::MQIMPO = types::MQIMPO(2);
    pub const MQIMPO_QUERY_LENGTH: types::MQIMPO = types::MQIMPO(4);
    pub const MQIMPO_INQ_NEXT: types::MQIMPO = types::MQIMPO(8);
    pub const MQIMPO_INQ_PROP_UNDER_CURSOR: types::MQIMPO = types::MQIMPO(16);
    pub const MQIMPO_CONVERT_VALUE: types::MQIMPO = types::MQIMPO(32);
    pub const MQIMPO_INQ_FIRST: types::MQIMPO = types::MQIMPO(0);
    pub const MQIPADDR_IPV4: types::MQIPADDR = types::MQIPADDR(0);
    pub const MQIPADDR_IPV6: types::MQIPADDR = types::MQIPADDR(1);
    pub const MQIT_NONE: types::MQIT = types::MQIT(0);
    pub const MQIT_MSG_ID: types::MQIT = types::MQIT(1);
    pub const MQIT_CORREL_ID: types::MQIT = types::MQIT(2);
    pub const MQIT_MSG_TOKEN: types::MQIT = types::MQIT(4);
    pub const MQIT_GROUP_ID: types::MQIT = types::MQIT(5);
    pub const MQIT_INTEGER: types::MQIT = types::MQIT(1);
    pub const MQIT_STRING: types::MQIT = types::MQIT(2);
    pub const MQIT_BAG: types::MQIT = types::MQIT(3);
    pub const MQKAI_AUTO: types::MQKAI = types::MQKAI(-1);
    pub const MQKEY_REUSE_UNLIMITED: types::MQKEY = types::MQKEY(-1);
    pub const MQKEY_REUSE_DISABLED: types::MQKEY = types::MQKEY(0);
    pub const MQMASTER_NO: types::MQMASTER = types::MQMASTER(0);
    pub const MQMASTER_YES: types::MQMASTER = types::MQMASTER(1);
    pub const MQMCB_DISABLED: types::MQMCB = types::MQMCB(0);
    pub const MQMCB_ENABLED: types::MQMCB = types::MQMCB(1);
    pub const MQMC_AS_PARENT: types::MQMC = types::MQMC(0);
    pub const MQMC_ENABLED: types::MQMC = types::MQMC(1);
    pub const MQMC_DISABLED: types::MQMC = types::MQMC(2);
    pub const MQMC_ONLY: types::MQMC = types::MQMC(3);
    pub const MQMDEF_NONE: types::MQMDEF = types::MQMDEF(0);
    pub const MQMDS_PRIORITY: types::MQMDS = types::MQMDS(0);
    pub const MQMDS_FIFO: types::MQMDS = types::MQMDS(1);
    pub const MQMEDIMGINTVL_OFF: types::MQMEDIMGINTVL = types::MQMEDIMGINTVL(0);
    pub const MQMEDIMGLOGLN_OFF: types::MQMEDIMGLOGLN = types::MQMEDIMGLOGLN(0);
    pub const MQMEDIMGSCHED_MANUAL: types::MQMEDIMGSCHED = types::MQMEDIMGSCHED(0);
    pub const MQMEDIMGSCHED_AUTO: types::MQMEDIMGSCHED = types::MQMEDIMGSCHED(1);
    pub const MQMF_ACCEPT_UNSUP_MASK: types::MQMF = types::MQMF(-1048576);
    pub const MQMF_NONE: types::MQMF = types::MQMF(0);
    pub const MQMF_SEGMENTATION_ALLOWED: types::MQMF = types::MQMF(1);
    pub const MQMF_SEGMENT: types::MQMF = types::MQMF(2);
    pub const MQMF_LAST_SEGMENT: types::MQMF = types::MQMF(4);
    pub const MQMF_MSG_IN_GROUP: types::MQMF = types::MQMF(8);
    pub const MQMF_LAST_MSG_IN_GROUP: types::MQMF = types::MQMF(16);
    pub const MQMF_REJECT_UNSUP_MASK: types::MQMF = types::MQMF(4095);
    pub const MQMF_ACCEPT_UNSUP_IF_XMIT_MASK: types::MQMF = types::MQMF(1044480);
    pub const MQMF_SEGMENTATION_INHIBITED: types::MQMF = types::MQMF(0);
    pub const MQMHBO_NONE: types::MQMHBO = types::MQMHBO(0);
    pub const MQMHBO_PROPERTIES_IN_MQRFH2: types::MQMHBO = types::MQMHBO(1);
    pub const MQMHBO_DELETE_PROPERTIES: types::MQMHBO = types::MQMHBO(2);
    pub const MQMMBI_UNLIMITED: types::MQMMBI = types::MQMMBI(-1);
    pub const MQMON_Q_MGR: types::MQMON = types::MQMON(-3);
    pub const MQMON_NONE: types::MQMON = types::MQMON(-1);
    pub const MQMON_OFF: types::MQMON = types::MQMON(0);
    pub const MQMON_ON: types::MQMON = types::MQMON(1);
    pub const MQMON_LOW: types::MQMON = types::MQMON(17);
    pub const MQMON_MEDIUM: types::MQMON = types::MQMON(33);
    pub const MQMON_HIGH: types::MQMON = types::MQMON(65);
    pub const MQMON_NOT_AVAILABLE: types::MQMON_AVAILABILITY = types::MQMON_AVAILABILITY(
        -1,
    );
    pub const MQMON_DISABLED: types::MQMON_OVERRIDE = types::MQMON_OVERRIDE(0);
    pub const MQMON_ENABLED: types::MQMON_OVERRIDE = types::MQMON_OVERRIDE(1);
    pub const MQMO_NONE: types::MQMO = types::MQMO(0);
    pub const MQMO_MATCH_MSG_ID: types::MQMO = types::MQMO(1);
    pub const MQMO_MATCH_CORREL_ID: types::MQMO = types::MQMO(2);
    pub const MQMO_MATCH_GROUP_ID: types::MQMO = types::MQMO(4);
    pub const MQMO_MATCH_MSG_SEQ_NUMBER: types::MQMO = types::MQMO(8);
    pub const MQMO_MATCH_OFFSET: types::MQMO = types::MQMO(16);
    pub const MQMO_MATCH_MSG_TOKEN: types::MQMO = types::MQMO(32);
    pub const MQMT_REQUEST: types::MQMT = types::MQMT(1);
    pub const MQMT_REPLY: types::MQMT = types::MQMT(2);
    pub const MQMT_REPORT: types::MQMT = types::MQMT(4);
    pub const MQMT_DATAGRAM: types::MQMT = types::MQMT(8);
    pub const MQMT_MQE_FIELDS_FROM_MQE: types::MQMT = types::MQMT(112);
    pub const MQMT_MQE_FIELDS: types::MQMT = types::MQMT(113);
    pub const MQMT_SYSTEM_FIRST: types::MQMT = types::MQMT(1);
    pub const MQMT_SYSTEM_LAST: types::MQMT = types::MQMT(65535);
    pub const MQMT_APPL_FIRST: types::MQMT = types::MQMT(65536);
    pub const MQMT_APPL_LAST: types::MQMT = types::MQMT(999999999);
    pub const MQNC_MAX_NAMELIST_NAME_COUNT: types::MQNC = types::MQNC(256);
    pub const MQNPM_CLASS_NORMAL: types::MQNPM = types::MQNPM(0);
    pub const MQNPM_CLASS_HIGH: types::MQNPM = types::MQNPM(10);
    pub const MQNT_NONE: types::MQNT = types::MQNT(0);
    pub const MQNT_Q: types::MQNT = types::MQNT(1);
    pub const MQNT_CLUSTER: types::MQNT = types::MQNT(2);
    pub const MQNT_AUTH_INFO: types::MQNT = types::MQNT(4);
    pub const MQNT_ALL: types::MQNT = types::MQNT(1001);
    pub const MQOL_UNDEFINED: types::MQOL = types::MQOL(-1);
    pub const MQOM_NO: types::MQOM = types::MQOM(0);
    pub const MQOM_YES: types::MQOM = types::MQOM(1);
    pub const MQOO_READ_AHEAD_AS_Q_DEF: types::MQOO = types::MQOO(0);
    pub const MQOO_INPUT_AS_Q_DEF: types::MQOO = types::MQOO(1);
    pub const MQOO_INPUT_SHARED: types::MQOO = types::MQOO(2);
    pub const MQOO_INPUT_EXCLUSIVE: types::MQOO = types::MQOO(4);
    pub const MQOO_BROWSE: types::MQOO = types::MQOO(8);
    pub const MQOO_OUTPUT: types::MQOO = types::MQOO(16);
    pub const MQOO_INQUIRE: types::MQOO = types::MQOO(32);
    pub const MQOO_SET: types::MQOO = types::MQOO(64);
    pub const MQOO_SAVE_ALL_CONTEXT: types::MQOO = types::MQOO(128);
    pub const MQOO_PASS_IDENTITY_CONTEXT: types::MQOO = types::MQOO(256);
    pub const MQOO_PASS_ALL_CONTEXT: types::MQOO = types::MQOO(512);
    pub const MQOO_SET_IDENTITY_CONTEXT: types::MQOO = types::MQOO(1024);
    pub const MQOO_SET_ALL_CONTEXT: types::MQOO = types::MQOO(2048);
    pub const MQOO_ALTERNATE_USER_AUTHORITY: types::MQOO = types::MQOO(4096);
    pub const MQOO_FAIL_IF_QUIESCING: types::MQOO = types::MQOO(8192);
    pub const MQOO_BIND_ON_OPEN: types::MQOO = types::MQOO(16384);
    pub const MQOO_BIND_NOT_FIXED: types::MQOO = types::MQOO(32768);
    pub const MQOO_RESOLVE_NAMES: types::MQOO = types::MQOO(65536);
    pub const MQOO_CO_OP: types::MQOO = types::MQOO(131072);
    pub const MQOO_RESOLVE_LOCAL_Q: types::MQOO = types::MQOO(262144);
    pub const MQOO_NO_READ_AHEAD: types::MQOO = types::MQOO(524288);
    pub const MQOO_READ_AHEAD: types::MQOO = types::MQOO(1048576);
    pub const MQOO_NO_MULTICAST: types::MQOO = types::MQOO(2097152);
    pub const MQOO_BIND_ON_GROUP: types::MQOO = types::MQOO(4194304);
    pub const MQOO_BIND_AS_Q_DEF: types::MQOO = types::MQOO(0);
    pub const MQOO_RESOLVE_LOCAL_TOPIC: types::MQOO = types::MQOO(262144);
    pub const MQOP_START: types::MQOP = types::MQOP(1);
    pub const MQOP_START_WAIT: types::MQOP = types::MQOP(2);
    pub const MQOP_STOP: types::MQOP = types::MQOP(4);
    pub const MQOP_REGISTER: types::MQOP = types::MQOP(256);
    pub const MQOP_DEREGISTER: types::MQOP = types::MQOP(512);
    pub const MQOP_SUSPEND: types::MQOP = types::MQOP(65536);
    pub const MQOP_RESUME: types::MQOP = types::MQOP(131072);
    pub const MQOT_NONE: types::MQOT = types::MQOT(0);
    pub const MQOT_Q: types::MQOT = types::MQOT(1);
    pub const MQOT_NAMELIST: types::MQOT = types::MQOT(2);
    pub const MQOT_PROCESS: types::MQOT = types::MQOT(3);
    pub const MQOT_STORAGE_CLASS: types::MQOT = types::MQOT(4);
    pub const MQOT_Q_MGR: types::MQOT = types::MQOT(5);
    pub const MQOT_CHANNEL: types::MQOT = types::MQOT(6);
    pub const MQOT_AUTH_INFO: types::MQOT = types::MQOT(7);
    pub const MQOT_TOPIC: types::MQOT = types::MQOT(8);
    pub const MQOT_COMM_INFO: types::MQOT = types::MQOT(9);
    pub const MQOT_CF_STRUC: types::MQOT = types::MQOT(10);
    pub const MQOT_LISTENER: types::MQOT = types::MQOT(11);
    pub const MQOT_SERVICE: types::MQOT = types::MQOT(12);
    pub const MQOT_RESERVED_1: types::MQOT = types::MQOT(999);
    pub const MQOT_ALL: types::MQOT = types::MQOT(1001);
    pub const MQOT_ALIAS_Q: types::MQOT = types::MQOT(1002);
    pub const MQOT_MODEL_Q: types::MQOT = types::MQOT(1003);
    pub const MQOT_LOCAL_Q: types::MQOT = types::MQOT(1004);
    pub const MQOT_REMOTE_Q: types::MQOT = types::MQOT(1005);
    pub const MQOT_SENDER_CHANNEL: types::MQOT = types::MQOT(1007);
    pub const MQOT_SERVER_CHANNEL: types::MQOT = types::MQOT(1008);
    pub const MQOT_REQUESTER_CHANNEL: types::MQOT = types::MQOT(1009);
    pub const MQOT_RECEIVER_CHANNEL: types::MQOT = types::MQOT(1010);
    pub const MQOT_CURRENT_CHANNEL: types::MQOT = types::MQOT(1011);
    pub const MQOT_SAVED_CHANNEL: types::MQOT = types::MQOT(1012);
    pub const MQOT_SVRCONN_CHANNEL: types::MQOT = types::MQOT(1013);
    pub const MQOT_CLNTCONN_CHANNEL: types::MQOT = types::MQOT(1014);
    pub const MQOT_SHORT_CHANNEL: types::MQOT = types::MQOT(1015);
    pub const MQOT_CHLAUTH: types::MQOT = types::MQOT(1016);
    pub const MQOT_REMOTE_Q_MGR_NAME: types::MQOT = types::MQOT(1017);
    pub const MQOT_PROT_POLICY: types::MQOT = types::MQOT(1019);
    pub const MQOT_TT_CHANNEL: types::MQOT = types::MQOT(1020);
    pub const MQOT_AMQP_CHANNEL: types::MQOT = types::MQOT(1021);
    pub const MQOT_AUTH_REC: types::MQOT = types::MQOT(1022);
    pub const MQPD_REJECT_UNSUP_MASK: types::MQPD = types::MQPD(-1048576);
    pub const MQPD_NONE: types::MQPD = types::MQPD(0);
    pub const MQPD_SUPPORT_OPTIONAL: types::MQPD = types::MQPD(1);
    pub const MQPD_ACCEPT_UNSUP_MASK: types::MQPD = types::MQPD(1023);
    pub const MQPD_SUPPORT_REQUIRED_IF_LOCAL: types::MQPD = types::MQPD(1024);
    pub const MQPD_ACCEPT_UNSUP_IF_XMIT_MASK: types::MQPD = types::MQPD(1047552);
    pub const MQPD_SUPPORT_REQUIRED: types::MQPD = types::MQPD(1048576);
    pub const MQPD_NO_CONTEXT: types::MQPD = types::MQPD(0);
    pub const MQPD_USER_CONTEXT: types::MQPD = types::MQPD(1);
    pub const MQPER_PERSISTENCE_AS_PARENT: types::MQPER = types::MQPER(-1);
    pub const MQPER_NOT_PERSISTENT: types::MQPER = types::MQPER(0);
    pub const MQPER_PERSISTENT: types::MQPER = types::MQPER(1);
    pub const MQPER_PERSISTENCE_AS_Q_DEF: types::MQPER = types::MQPER(2);
    pub const MQPER_PERSISTENCE_AS_TOPIC_DEF: types::MQPER = types::MQPER(2);
    pub const MQPL_ZOS: types::MQPL = types::MQPL(1);
    pub const MQPL_OS2: types::MQPL = types::MQPL(2);
    pub const MQPL_UNIX: types::MQPL = types::MQPL(3);
    pub const MQPL_OS400: types::MQPL = types::MQPL(4);
    pub const MQPL_WINDOWS: types::MQPL = types::MQPL(5);
    pub const MQPL_WINDOWS_NT: types::MQPL = types::MQPL(11);
    pub const MQPL_VMS: types::MQPL = types::MQPL(12);
    pub const MQPL_NSK: types::MQPL = types::MQPL(13);
    pub const MQPL_OPEN_TP1: types::MQPL = types::MQPL(15);
    pub const MQPL_VM: types::MQPL = types::MQPL(18);
    pub const MQPL_TPF: types::MQPL = types::MQPL(23);
    pub const MQPL_VSE: types::MQPL = types::MQPL(27);
    pub const MQPL_APPLIANCE: types::MQPL = types::MQPL(28);
    pub const MQPL_MVS: types::MQPL = types::MQPL(1);
    pub const MQPL_OS390: types::MQPL = types::MQPL(1);
    pub const MQPL_AIX: types::MQPL = types::MQPL(3);
    pub const MQPL_NSS: types::MQPL = types::MQPL(13);
    pub const MQPMO_NONE: types::MQPMO = types::MQPMO(0);
    pub const MQPMO_SYNCPOINT: types::MQPMO = types::MQPMO(2);
    pub const MQPMO_NO_SYNCPOINT: types::MQPMO = types::MQPMO(4);
    pub const MQPMO_DEFAULT_CONTEXT: types::MQPMO = types::MQPMO(32);
    pub const MQPMO_NEW_MSG_ID: types::MQPMO = types::MQPMO(64);
    pub const MQPMO_NEW_CORREL_ID: types::MQPMO = types::MQPMO(128);
    pub const MQPMO_PASS_IDENTITY_CONTEXT: types::MQPMO = types::MQPMO(256);
    pub const MQPMO_PASS_ALL_CONTEXT: types::MQPMO = types::MQPMO(512);
    pub const MQPMO_SET_IDENTITY_CONTEXT: types::MQPMO = types::MQPMO(1024);
    pub const MQPMO_SET_ALL_CONTEXT: types::MQPMO = types::MQPMO(2048);
    pub const MQPMO_ALTERNATE_USER_AUTHORITY: types::MQPMO = types::MQPMO(4096);
    pub const MQPMO_FAIL_IF_QUIESCING: types::MQPMO = types::MQPMO(8192);
    pub const MQPMO_NO_CONTEXT: types::MQPMO = types::MQPMO(16384);
    pub const MQPMO_LOGICAL_ORDER: types::MQPMO = types::MQPMO(32768);
    pub const MQPMO_ASYNC_RESPONSE: types::MQPMO = types::MQPMO(65536);
    pub const MQPMO_SYNC_RESPONSE: types::MQPMO = types::MQPMO(131072);
    pub const MQPMO_RESOLVE_LOCAL_Q: types::MQPMO = types::MQPMO(262144);
    pub const MQPMO_WARN_IF_NO_SUBS_MATCHED: types::MQPMO = types::MQPMO(524288);
    pub const MQPMO_RETAIN: types::MQPMO = types::MQPMO(2097152);
    pub const MQPMO_MD_FOR_OUTPUT_ONLY: types::MQPMO = types::MQPMO(8388608);
    pub const MQPMO_SCOPE_QMGR: types::MQPMO = types::MQPMO(67108864);
    pub const MQPMO_SUPPRESS_REPLYTO: types::MQPMO = types::MQPMO(134217728);
    pub const MQPMO_NOT_OWN_SUBS: types::MQPMO = types::MQPMO(268435456);
    pub const MQPMO_RESPONSE_AS_Q_DEF: types::MQPMO = types::MQPMO(0);
    pub const MQPMO_RESPONSE_AS_TOPIC_DEF: types::MQPMO = types::MQPMO(0);
    pub const MQPMO_PUB_OPTIONS_MASK: types::MQPMO = types::MQPMO(2097152);
    pub const MQPMRF_NONE: types::MQPMRF = types::MQPMRF(0);
    pub const MQPMRF_MSG_ID: types::MQPMRF = types::MQPMRF(1);
    pub const MQPMRF_CORREL_ID: types::MQPMRF = types::MQPMRF(2);
    pub const MQPMRF_GROUP_ID: types::MQPMRF = types::MQPMRF(4);
    pub const MQPMRF_FEEDBACK: types::MQPMRF = types::MQPMRF(8);
    pub const MQPMRF_ACCOUNTING_TOKEN: types::MQPMRF = types::MQPMRF(16);
    pub const MQPRI_PRIORITY_AS_PUBLISHED: types::MQPRI = types::MQPRI(-3);
    pub const MQPRI_PRIORITY_AS_PARENT: types::MQPRI = types::MQPRI(-2);
    pub const MQPRI_PRIORITY_AS_Q_DEF: types::MQPRI = types::MQPRI(-1);
    pub const MQPRI_PRIORITY_AS_TOPIC_DEF: types::MQPRI = types::MQPRI(-1);
    pub const MQPROP_UNRESTRICTED_LENGTH: types::MQPROP = types::MQPROP(-1);
    pub const MQPROP_COMPATIBILITY: types::MQPROP = types::MQPROP(0);
    pub const MQPROP_NONE: types::MQPROP = types::MQPROP(1);
    pub const MQPROP_ALL: types::MQPROP = types::MQPROP(2);
    pub const MQPROP_FORCE_MQRFH2: types::MQPROP = types::MQPROP(3);
    pub const MQPROP_V6COMPAT: types::MQPROP = types::MQPROP(4);
    pub const MQPRT_RESPONSE_AS_PARENT: types::MQPRT = types::MQPRT(0);
    pub const MQPRT_SYNC_RESPONSE: types::MQPRT = types::MQPRT(1);
    pub const MQPRT_ASYNC_RESPONSE: types::MQPRT = types::MQPRT(2);
    pub const MQPSCLUS_DISABLED: types::MQPSCLUS = types::MQPSCLUS(0);
    pub const MQPSCLUS_ENABLED: types::MQPSCLUS = types::MQPSCLUS(1);
    pub const MQPSM_DISABLED: types::MQPSM = types::MQPSM(0);
    pub const MQPSM_COMPAT: types::MQPSM = types::MQPSM(1);
    pub const MQPSM_ENABLED: types::MQPSM = types::MQPSM(2);
    pub const MQPSPROP_NONE: types::MQPSPROP = types::MQPSPROP(0);
    pub const MQPSPROP_COMPAT: types::MQPSPROP = types::MQPSPROP(1);
    pub const MQPSPROP_RFH2: types::MQPSPROP = types::MQPSPROP(2);
    pub const MQPSPROP_MSGPROP: types::MQPSPROP = types::MQPSPROP(3);
    pub const MQQA_BACKOUT_NOT_HARDENED: types::MQQA_BACKOUT = types::MQQA_BACKOUT(0);
    pub const MQQA_BACKOUT_HARDENED: types::MQQA_BACKOUT = types::MQQA_BACKOUT(1);
    pub const MQQA_GET_ALLOWED: types::MQQA_GET = types::MQQA_GET(0);
    pub const MQQA_GET_INHIBITED: types::MQQA_GET = types::MQQA_GET(1);
    pub const MQQA_PUT_ALLOWED: types::MQQA_PUT = types::MQQA_PUT(0);
    pub const MQQA_PUT_INHIBITED: types::MQQA_PUT = types::MQQA_PUT(1);
    pub const MQQA_NOT_SHAREABLE: types::MQQA_SHAREABLE = types::MQQA_SHAREABLE(0);
    pub const MQQA_SHAREABLE: types::MQQA_SHAREABLE = types::MQQA_SHAREABLE(1);
    pub const MQQDT_PREDEFINED: types::MQQDT = types::MQQDT(1);
    pub const MQQDT_PERMANENT_DYNAMIC: types::MQQDT = types::MQQDT(2);
    pub const MQQDT_TEMPORARY_DYNAMIC: types::MQQDT = types::MQQDT(3);
    pub const MQQDT_SHARED_DYNAMIC: types::MQQDT = types::MQQDT(4);
    pub const MQQFS_DEFAULT: types::MQQFS = types::MQQFS(-1);
    pub const MQQMOPT_DISABLED: types::MQQMOPT = types::MQQMOPT(0);
    pub const MQQMOPT_ENABLED: types::MQQMOPT = types::MQQMOPT(1);
    pub const MQQMOPT_REPLY: types::MQQMOPT = types::MQQMOPT(2);
    pub const MQQSGD_ALL: types::MQQSGD = types::MQQSGD(-1);
    pub const MQQSGD_Q_MGR: types::MQQSGD = types::MQQSGD(0);
    pub const MQQSGD_COPY: types::MQQSGD = types::MQQSGD(1);
    pub const MQQSGD_SHARED: types::MQQSGD = types::MQQSGD(2);
    pub const MQQSGD_GROUP: types::MQQSGD = types::MQQSGD(3);
    pub const MQQSGD_PRIVATE: types::MQQSGD = types::MQQSGD(4);
    pub const MQQSGD_LIVE: types::MQQSGD = types::MQQSGD(6);
    pub const MQQT_LOCAL: types::MQQT = types::MQQT(1);
    pub const MQQT_MODEL: types::MQQT = types::MQQT(2);
    pub const MQQT_ALIAS: types::MQQT = types::MQQT(3);
    pub const MQQT_REMOTE: types::MQQT = types::MQQT(6);
    pub const MQQT_CLUSTER: types::MQQT = types::MQQT(7);
    pub const MQQT_ALL: types::MQQT = types::MQQT(1001);
    pub const MQRCN_NO: types::MQRCN = types::MQRCN(0);
    pub const MQRCN_YES: types::MQRCN = types::MQRCN(1);
    pub const MQRCN_Q_MGR: types::MQRCN = types::MQRCN(2);
    pub const MQRCN_DISABLED: types::MQRCN = types::MQRCN(3);
    pub const MQRCVTIME_MULTIPLY: types::MQRCVTIME = types::MQRCVTIME(0);
    pub const MQRCVTIME_ADD: types::MQRCVTIME = types::MQRCVTIME(1);
    pub const MQRCVTIME_EQUAL: types::MQRCVTIME = types::MQRCVTIME(2);
    pub const MQRC_NONE: types::MQRC = types::MQRC(0);
    pub const MQRC_ALIAS_BASE_Q_TYPE_ERROR: types::MQRC = types::MQRC(2001);
    pub const MQRC_ALREADY_CONNECTED: types::MQRC = types::MQRC(2002);
    pub const MQRC_BACKED_OUT: types::MQRC = types::MQRC(2003);
    pub const MQRC_BUFFER_ERROR: types::MQRC = types::MQRC(2004);
    pub const MQRC_BUFFER_LENGTH_ERROR: types::MQRC = types::MQRC(2005);
    pub const MQRC_CHAR_ATTR_LENGTH_ERROR: types::MQRC = types::MQRC(2006);
    pub const MQRC_CHAR_ATTRS_ERROR: types::MQRC = types::MQRC(2007);
    pub const MQRC_CHAR_ATTRS_TOO_SHORT: types::MQRC = types::MQRC(2008);
    pub const MQRC_CONNECTION_BROKEN: types::MQRC = types::MQRC(2009);
    pub const MQRC_DATA_LENGTH_ERROR: types::MQRC = types::MQRC(2010);
    pub const MQRC_DYNAMIC_Q_NAME_ERROR: types::MQRC = types::MQRC(2011);
    pub const MQRC_ENVIRONMENT_ERROR: types::MQRC = types::MQRC(2012);
    pub const MQRC_EXPIRY_ERROR: types::MQRC = types::MQRC(2013);
    pub const MQRC_FEEDBACK_ERROR: types::MQRC = types::MQRC(2014);
    pub const MQRC_GET_INHIBITED: types::MQRC = types::MQRC(2016);
    pub const MQRC_HANDLE_NOT_AVAILABLE: types::MQRC = types::MQRC(2017);
    pub const MQRC_HCONN_ERROR: types::MQRC = types::MQRC(2018);
    pub const MQRC_HOBJ_ERROR: types::MQRC = types::MQRC(2019);
    pub const MQRC_INHIBIT_VALUE_ERROR: types::MQRC = types::MQRC(2020);
    pub const MQRC_INT_ATTR_COUNT_ERROR: types::MQRC = types::MQRC(2021);
    pub const MQRC_INT_ATTR_COUNT_TOO_SMALL: types::MQRC = types::MQRC(2022);
    pub const MQRC_INT_ATTRS_ARRAY_ERROR: types::MQRC = types::MQRC(2023);
    pub const MQRC_SYNCPOINT_LIMIT_REACHED: types::MQRC = types::MQRC(2024);
    pub const MQRC_MAX_CONNS_LIMIT_REACHED: types::MQRC = types::MQRC(2025);
    pub const MQRC_MD_ERROR: types::MQRC = types::MQRC(2026);
    pub const MQRC_MISSING_REPLY_TO_Q: types::MQRC = types::MQRC(2027);
    pub const MQRC_MSG_TYPE_ERROR: types::MQRC = types::MQRC(2029);
    pub const MQRC_MSG_TOO_BIG_FOR_Q: types::MQRC = types::MQRC(2030);
    pub const MQRC_MSG_TOO_BIG_FOR_Q_MGR: types::MQRC = types::MQRC(2031);
    pub const MQRC_NO_MSG_AVAILABLE: types::MQRC = types::MQRC(2033);
    pub const MQRC_NO_MSG_UNDER_CURSOR: types::MQRC = types::MQRC(2034);
    pub const MQRC_NOT_AUTHORIZED: types::MQRC = types::MQRC(2035);
    pub const MQRC_NOT_OPEN_FOR_BROWSE: types::MQRC = types::MQRC(2036);
    pub const MQRC_NOT_OPEN_FOR_INPUT: types::MQRC = types::MQRC(2037);
    pub const MQRC_NOT_OPEN_FOR_INQUIRE: types::MQRC = types::MQRC(2038);
    pub const MQRC_NOT_OPEN_FOR_OUTPUT: types::MQRC = types::MQRC(2039);
    pub const MQRC_NOT_OPEN_FOR_SET: types::MQRC = types::MQRC(2040);
    pub const MQRC_OBJECT_CHANGED: types::MQRC = types::MQRC(2041);
    pub const MQRC_OBJECT_IN_USE: types::MQRC = types::MQRC(2042);
    pub const MQRC_OBJECT_TYPE_ERROR: types::MQRC = types::MQRC(2043);
    pub const MQRC_OD_ERROR: types::MQRC = types::MQRC(2044);
    pub const MQRC_OPTION_NOT_VALID_FOR_TYPE: types::MQRC = types::MQRC(2045);
    pub const MQRC_OPTIONS_ERROR: types::MQRC = types::MQRC(2046);
    pub const MQRC_PERSISTENCE_ERROR: types::MQRC = types::MQRC(2047);
    pub const MQRC_PERSISTENT_NOT_ALLOWED: types::MQRC = types::MQRC(2048);
    pub const MQRC_PRIORITY_EXCEEDS_MAXIMUM: types::MQRC = types::MQRC(2049);
    pub const MQRC_PRIORITY_ERROR: types::MQRC = types::MQRC(2050);
    pub const MQRC_PUT_INHIBITED: types::MQRC = types::MQRC(2051);
    pub const MQRC_Q_DELETED: types::MQRC = types::MQRC(2052);
    pub const MQRC_Q_FULL: types::MQRC = types::MQRC(2053);
    pub const MQRC_Q_NOT_EMPTY: types::MQRC = types::MQRC(2055);
    pub const MQRC_Q_SPACE_NOT_AVAILABLE: types::MQRC = types::MQRC(2056);
    pub const MQRC_Q_TYPE_ERROR: types::MQRC = types::MQRC(2057);
    pub const MQRC_Q_MGR_NAME_ERROR: types::MQRC = types::MQRC(2058);
    pub const MQRC_Q_MGR_NOT_AVAILABLE: types::MQRC = types::MQRC(2059);
    pub const MQRC_REPORT_OPTIONS_ERROR: types::MQRC = types::MQRC(2061);
    pub const MQRC_SECOND_MARK_NOT_ALLOWED: types::MQRC = types::MQRC(2062);
    pub const MQRC_SECURITY_ERROR: types::MQRC = types::MQRC(2063);
    pub const MQRC_TOKEN_TIMESTAMP_NOT_VALID: types::MQRC = types::MQRC(2064);
    pub const MQRC_SELECTOR_COUNT_ERROR: types::MQRC = types::MQRC(2065);
    pub const MQRC_SELECTOR_LIMIT_EXCEEDED: types::MQRC = types::MQRC(2066);
    pub const MQRC_SELECTOR_ERROR: types::MQRC = types::MQRC(2067);
    pub const MQRC_SELECTOR_NOT_FOR_TYPE: types::MQRC = types::MQRC(2068);
    pub const MQRC_SIGNAL_OUTSTANDING: types::MQRC = types::MQRC(2069);
    pub const MQRC_SIGNAL_REQUEST_ACCEPTED: types::MQRC = types::MQRC(2070);
    pub const MQRC_STORAGE_NOT_AVAILABLE: types::MQRC = types::MQRC(2071);
    pub const MQRC_SYNCPOINT_NOT_AVAILABLE: types::MQRC = types::MQRC(2072);
    pub const MQRC_TRIGGER_CONTROL_ERROR: types::MQRC = types::MQRC(2075);
    pub const MQRC_TRIGGER_DEPTH_ERROR: types::MQRC = types::MQRC(2076);
    pub const MQRC_TRIGGER_MSG_PRIORITY_ERR: types::MQRC = types::MQRC(2077);
    pub const MQRC_TRIGGER_TYPE_ERROR: types::MQRC = types::MQRC(2078);
    pub const MQRC_TRUNCATED_MSG_ACCEPTED: types::MQRC = types::MQRC(2079);
    pub const MQRC_TRUNCATED_MSG_FAILED: types::MQRC = types::MQRC(2080);
    pub const MQRC_UNKNOWN_ALIAS_BASE_Q: types::MQRC = types::MQRC(2082);
    pub const MQRC_UNKNOWN_OBJECT_NAME: types::MQRC = types::MQRC(2085);
    pub const MQRC_UNKNOWN_OBJECT_Q_MGR: types::MQRC = types::MQRC(2086);
    pub const MQRC_UNKNOWN_REMOTE_Q_MGR: types::MQRC = types::MQRC(2087);
    pub const MQRC_WAIT_INTERVAL_ERROR: types::MQRC = types::MQRC(2090);
    pub const MQRC_XMIT_Q_TYPE_ERROR: types::MQRC = types::MQRC(2091);
    pub const MQRC_XMIT_Q_USAGE_ERROR: types::MQRC = types::MQRC(2092);
    pub const MQRC_NOT_OPEN_FOR_PASS_ALL: types::MQRC = types::MQRC(2093);
    pub const MQRC_NOT_OPEN_FOR_PASS_IDENT: types::MQRC = types::MQRC(2094);
    pub const MQRC_NOT_OPEN_FOR_SET_ALL: types::MQRC = types::MQRC(2095);
    pub const MQRC_NOT_OPEN_FOR_SET_IDENT: types::MQRC = types::MQRC(2096);
    pub const MQRC_CONTEXT_HANDLE_ERROR: types::MQRC = types::MQRC(2097);
    pub const MQRC_CONTEXT_NOT_AVAILABLE: types::MQRC = types::MQRC(2098);
    pub const MQRC_SIGNAL1_ERROR: types::MQRC = types::MQRC(2099);
    pub const MQRC_OBJECT_ALREADY_EXISTS: types::MQRC = types::MQRC(2100);
    pub const MQRC_OBJECT_DAMAGED: types::MQRC = types::MQRC(2101);
    pub const MQRC_RESOURCE_PROBLEM: types::MQRC = types::MQRC(2102);
    pub const MQRC_ANOTHER_Q_MGR_CONNECTED: types::MQRC = types::MQRC(2103);
    pub const MQRC_UNKNOWN_REPORT_OPTION: types::MQRC = types::MQRC(2104);
    pub const MQRC_STORAGE_CLASS_ERROR: types::MQRC = types::MQRC(2105);
    pub const MQRC_COD_NOT_VALID_FOR_XCF_Q: types::MQRC = types::MQRC(2106);
    pub const MQRC_XWAIT_CANCELED: types::MQRC = types::MQRC(2107);
    pub const MQRC_XWAIT_ERROR: types::MQRC = types::MQRC(2108);
    pub const MQRC_SUPPRESSED_BY_EXIT: types::MQRC = types::MQRC(2109);
    pub const MQRC_FORMAT_ERROR: types::MQRC = types::MQRC(2110);
    pub const MQRC_SOURCE_CCSID_ERROR: types::MQRC = types::MQRC(2111);
    pub const MQRC_SOURCE_INTEGER_ENC_ERROR: types::MQRC = types::MQRC(2112);
    pub const MQRC_SOURCE_DECIMAL_ENC_ERROR: types::MQRC = types::MQRC(2113);
    pub const MQRC_SOURCE_FLOAT_ENC_ERROR: types::MQRC = types::MQRC(2114);
    pub const MQRC_TARGET_CCSID_ERROR: types::MQRC = types::MQRC(2115);
    pub const MQRC_TARGET_INTEGER_ENC_ERROR: types::MQRC = types::MQRC(2116);
    pub const MQRC_TARGET_DECIMAL_ENC_ERROR: types::MQRC = types::MQRC(2117);
    pub const MQRC_TARGET_FLOAT_ENC_ERROR: types::MQRC = types::MQRC(2118);
    pub const MQRC_NOT_CONVERTED: types::MQRC = types::MQRC(2119);
    pub const MQRC_CONVERTED_MSG_TOO_BIG: types::MQRC = types::MQRC(2120);
    pub const MQRC_NO_EXTERNAL_PARTICIPANTS: types::MQRC = types::MQRC(2121);
    pub const MQRC_PARTICIPANT_NOT_AVAILABLE: types::MQRC = types::MQRC(2122);
    pub const MQRC_OUTCOME_MIXED: types::MQRC = types::MQRC(2123);
    pub const MQRC_OUTCOME_PENDING: types::MQRC = types::MQRC(2124);
    pub const MQRC_BRIDGE_STARTED: types::MQRC = types::MQRC(2125);
    pub const MQRC_BRIDGE_STOPPED: types::MQRC = types::MQRC(2126);
    pub const MQRC_ADAPTER_STORAGE_SHORTAGE: types::MQRC = types::MQRC(2127);
    pub const MQRC_UOW_IN_PROGRESS: types::MQRC = types::MQRC(2128);
    pub const MQRC_ADAPTER_CONN_LOAD_ERROR: types::MQRC = types::MQRC(2129);
    pub const MQRC_ADAPTER_SERV_LOAD_ERROR: types::MQRC = types::MQRC(2130);
    pub const MQRC_ADAPTER_DEFS_ERROR: types::MQRC = types::MQRC(2131);
    pub const MQRC_ADAPTER_DEFS_LOAD_ERROR: types::MQRC = types::MQRC(2132);
    pub const MQRC_ADAPTER_CONV_LOAD_ERROR: types::MQRC = types::MQRC(2133);
    pub const MQRC_BO_ERROR: types::MQRC = types::MQRC(2134);
    pub const MQRC_DH_ERROR: types::MQRC = types::MQRC(2135);
    pub const MQRC_MULTIPLE_REASONS: types::MQRC = types::MQRC(2136);
    pub const MQRC_OPEN_FAILED: types::MQRC = types::MQRC(2137);
    pub const MQRC_ADAPTER_DISC_LOAD_ERROR: types::MQRC = types::MQRC(2138);
    pub const MQRC_CNO_ERROR: types::MQRC = types::MQRC(2139);
    pub const MQRC_CICS_WAIT_FAILED: types::MQRC = types::MQRC(2140);
    pub const MQRC_DLH_ERROR: types::MQRC = types::MQRC(2141);
    pub const MQRC_HEADER_ERROR: types::MQRC = types::MQRC(2142);
    pub const MQRC_SOURCE_LENGTH_ERROR: types::MQRC = types::MQRC(2143);
    pub const MQRC_TARGET_LENGTH_ERROR: types::MQRC = types::MQRC(2144);
    pub const MQRC_SOURCE_BUFFER_ERROR: types::MQRC = types::MQRC(2145);
    pub const MQRC_TARGET_BUFFER_ERROR: types::MQRC = types::MQRC(2146);
    pub const MQRC_INCOMPLETE_TRANSACTION: types::MQRC = types::MQRC(2147);
    pub const MQRC_IIH_ERROR: types::MQRC = types::MQRC(2148);
    pub const MQRC_PCF_ERROR: types::MQRC = types::MQRC(2149);
    pub const MQRC_DBCS_ERROR: types::MQRC = types::MQRC(2150);
    pub const MQRC_OBJECT_NAME_ERROR: types::MQRC = types::MQRC(2152);
    pub const MQRC_OBJECT_Q_MGR_NAME_ERROR: types::MQRC = types::MQRC(2153);
    pub const MQRC_RECS_PRESENT_ERROR: types::MQRC = types::MQRC(2154);
    pub const MQRC_OBJECT_RECORDS_ERROR: types::MQRC = types::MQRC(2155);
    pub const MQRC_RESPONSE_RECORDS_ERROR: types::MQRC = types::MQRC(2156);
    pub const MQRC_ASID_MISMATCH: types::MQRC = types::MQRC(2157);
    pub const MQRC_PMO_RECORD_FLAGS_ERROR: types::MQRC = types::MQRC(2158);
    pub const MQRC_PUT_MSG_RECORDS_ERROR: types::MQRC = types::MQRC(2159);
    pub const MQRC_CONN_ID_IN_USE: types::MQRC = types::MQRC(2160);
    pub const MQRC_Q_MGR_QUIESCING: types::MQRC = types::MQRC(2161);
    pub const MQRC_Q_MGR_STOPPING: types::MQRC = types::MQRC(2162);
    pub const MQRC_DUPLICATE_RECOV_COORD: types::MQRC = types::MQRC(2163);
    pub const MQRC_PMO_ERROR: types::MQRC = types::MQRC(2173);
    pub const MQRC_API_EXIT_NOT_FOUND: types::MQRC = types::MQRC(2182);
    pub const MQRC_API_EXIT_LOAD_ERROR: types::MQRC = types::MQRC(2183);
    pub const MQRC_REMOTE_Q_NAME_ERROR: types::MQRC = types::MQRC(2184);
    pub const MQRC_INCONSISTENT_PERSISTENCE: types::MQRC = types::MQRC(2185);
    pub const MQRC_GMO_ERROR: types::MQRC = types::MQRC(2186);
    pub const MQRC_CICS_BRIDGE_RESTRICTION: types::MQRC = types::MQRC(2187);
    pub const MQRC_STOPPED_BY_CLUSTER_EXIT: types::MQRC = types::MQRC(2188);
    pub const MQRC_CLUSTER_RESOLUTION_ERROR: types::MQRC = types::MQRC(2189);
    pub const MQRC_CONVERTED_STRING_TOO_BIG: types::MQRC = types::MQRC(2190);
    pub const MQRC_TMC_ERROR: types::MQRC = types::MQRC(2191);
    pub const MQRC_STORAGE_MEDIUM_FULL: types::MQRC = types::MQRC(2192);
    pub const MQRC_PAGESET_ERROR: types::MQRC = types::MQRC(2193);
    pub const MQRC_NAME_NOT_VALID_FOR_TYPE: types::MQRC = types::MQRC(2194);
    pub const MQRC_UNEXPECTED_ERROR: types::MQRC = types::MQRC(2195);
    pub const MQRC_UNKNOWN_XMIT_Q: types::MQRC = types::MQRC(2196);
    pub const MQRC_UNKNOWN_DEF_XMIT_Q: types::MQRC = types::MQRC(2197);
    pub const MQRC_DEF_XMIT_Q_TYPE_ERROR: types::MQRC = types::MQRC(2198);
    pub const MQRC_DEF_XMIT_Q_USAGE_ERROR: types::MQRC = types::MQRC(2199);
    pub const MQRC_MSG_MARKED_BROWSE_CO_OP: types::MQRC = types::MQRC(2200);
    pub const MQRC_NAME_IN_USE: types::MQRC = types::MQRC(2201);
    pub const MQRC_CONNECTION_QUIESCING: types::MQRC = types::MQRC(2202);
    pub const MQRC_CONNECTION_STOPPING: types::MQRC = types::MQRC(2203);
    pub const MQRC_ADAPTER_NOT_AVAILABLE: types::MQRC = types::MQRC(2204);
    pub const MQRC_MSG_ID_ERROR: types::MQRC = types::MQRC(2206);
    pub const MQRC_CORREL_ID_ERROR: types::MQRC = types::MQRC(2207);
    pub const MQRC_FILE_SYSTEM_ERROR: types::MQRC = types::MQRC(2208);
    pub const MQRC_NO_MSG_LOCKED: types::MQRC = types::MQRC(2209);
    pub const MQRC_SOAP_DOTNET_ERROR: types::MQRC = types::MQRC(2210);
    pub const MQRC_SOAP_AXIS_ERROR: types::MQRC = types::MQRC(2211);
    pub const MQRC_SOAP_URL_ERROR: types::MQRC = types::MQRC(2212);
    pub const MQRC_FILE_NOT_AUDITED: types::MQRC = types::MQRC(2216);
    pub const MQRC_CONNECTION_NOT_AUTHORIZED: types::MQRC = types::MQRC(2217);
    pub const MQRC_MSG_TOO_BIG_FOR_CHANNEL: types::MQRC = types::MQRC(2218);
    pub const MQRC_CALL_IN_PROGRESS: types::MQRC = types::MQRC(2219);
    pub const MQRC_RMH_ERROR: types::MQRC = types::MQRC(2220);
    pub const MQRC_Q_MGR_ACTIVE: types::MQRC = types::MQRC(2222);
    pub const MQRC_Q_MGR_NOT_ACTIVE: types::MQRC = types::MQRC(2223);
    pub const MQRC_Q_DEPTH_HIGH: types::MQRC = types::MQRC(2224);
    pub const MQRC_Q_DEPTH_LOW: types::MQRC = types::MQRC(2225);
    pub const MQRC_Q_SERVICE_INTERVAL_HIGH: types::MQRC = types::MQRC(2226);
    pub const MQRC_Q_SERVICE_INTERVAL_OK: types::MQRC = types::MQRC(2227);
    pub const MQRC_RFH_HEADER_FIELD_ERROR: types::MQRC = types::MQRC(2228);
    pub const MQRC_RAS_PROPERTY_ERROR: types::MQRC = types::MQRC(2229);
    pub const MQRC_UNIT_OF_WORK_NOT_STARTED: types::MQRC = types::MQRC(2232);
    pub const MQRC_CHANNEL_AUTO_DEF_OK: types::MQRC = types::MQRC(2233);
    pub const MQRC_CHANNEL_AUTO_DEF_ERROR: types::MQRC = types::MQRC(2234);
    pub const MQRC_CFH_ERROR: types::MQRC = types::MQRC(2235);
    pub const MQRC_CFIL_ERROR: types::MQRC = types::MQRC(2236);
    pub const MQRC_CFIN_ERROR: types::MQRC = types::MQRC(2237);
    pub const MQRC_CFSL_ERROR: types::MQRC = types::MQRC(2238);
    pub const MQRC_CFST_ERROR: types::MQRC = types::MQRC(2239);
    pub const MQRC_INCOMPLETE_GROUP: types::MQRC = types::MQRC(2241);
    pub const MQRC_INCOMPLETE_MSG: types::MQRC = types::MQRC(2242);
    pub const MQRC_INCONSISTENT_CCSIDS: types::MQRC = types::MQRC(2243);
    pub const MQRC_INCONSISTENT_ENCODINGS: types::MQRC = types::MQRC(2244);
    pub const MQRC_INCONSISTENT_UOW: types::MQRC = types::MQRC(2245);
    pub const MQRC_INVALID_MSG_UNDER_CURSOR: types::MQRC = types::MQRC(2246);
    pub const MQRC_MATCH_OPTIONS_ERROR: types::MQRC = types::MQRC(2247);
    pub const MQRC_MDE_ERROR: types::MQRC = types::MQRC(2248);
    pub const MQRC_MSG_FLAGS_ERROR: types::MQRC = types::MQRC(2249);
    pub const MQRC_MSG_SEQ_NUMBER_ERROR: types::MQRC = types::MQRC(2250);
    pub const MQRC_OFFSET_ERROR: types::MQRC = types::MQRC(2251);
    pub const MQRC_ORIGINAL_LENGTH_ERROR: types::MQRC = types::MQRC(2252);
    pub const MQRC_SEGMENT_LENGTH_ZERO: types::MQRC = types::MQRC(2253);
    pub const MQRC_UOW_NOT_AVAILABLE: types::MQRC = types::MQRC(2255);
    pub const MQRC_WRONG_GMO_VERSION: types::MQRC = types::MQRC(2256);
    pub const MQRC_WRONG_MD_VERSION: types::MQRC = types::MQRC(2257);
    pub const MQRC_GROUP_ID_ERROR: types::MQRC = types::MQRC(2258);
    pub const MQRC_INCONSISTENT_BROWSE: types::MQRC = types::MQRC(2259);
    pub const MQRC_XQH_ERROR: types::MQRC = types::MQRC(2260);
    pub const MQRC_SRC_ENV_ERROR: types::MQRC = types::MQRC(2261);
    pub const MQRC_SRC_NAME_ERROR: types::MQRC = types::MQRC(2262);
    pub const MQRC_DEST_ENV_ERROR: types::MQRC = types::MQRC(2263);
    pub const MQRC_DEST_NAME_ERROR: types::MQRC = types::MQRC(2264);
    pub const MQRC_TM_ERROR: types::MQRC = types::MQRC(2265);
    pub const MQRC_CLUSTER_EXIT_ERROR: types::MQRC = types::MQRC(2266);
    pub const MQRC_CLUSTER_EXIT_LOAD_ERROR: types::MQRC = types::MQRC(2267);
    pub const MQRC_CLUSTER_PUT_INHIBITED: types::MQRC = types::MQRC(2268);
    pub const MQRC_CLUSTER_RESOURCE_ERROR: types::MQRC = types::MQRC(2269);
    pub const MQRC_NO_DESTINATIONS_AVAILABLE: types::MQRC = types::MQRC(2270);
    pub const MQRC_CONN_TAG_IN_USE: types::MQRC = types::MQRC(2271);
    pub const MQRC_PARTIALLY_CONVERTED: types::MQRC = types::MQRC(2272);
    pub const MQRC_CONNECTION_ERROR: types::MQRC = types::MQRC(2273);
    pub const MQRC_OPTION_ENVIRONMENT_ERROR: types::MQRC = types::MQRC(2274);
    pub const MQRC_CD_ERROR: types::MQRC = types::MQRC(2277);
    pub const MQRC_CLIENT_CONN_ERROR: types::MQRC = types::MQRC(2278);
    pub const MQRC_CHANNEL_STOPPED_BY_USER: types::MQRC = types::MQRC(2279);
    pub const MQRC_HCONFIG_ERROR: types::MQRC = types::MQRC(2280);
    pub const MQRC_FUNCTION_ERROR: types::MQRC = types::MQRC(2281);
    pub const MQRC_CHANNEL_STARTED: types::MQRC = types::MQRC(2282);
    pub const MQRC_CHANNEL_STOPPED: types::MQRC = types::MQRC(2283);
    pub const MQRC_CHANNEL_CONV_ERROR: types::MQRC = types::MQRC(2284);
    pub const MQRC_SERVICE_NOT_AVAILABLE: types::MQRC = types::MQRC(2285);
    pub const MQRC_INITIALIZATION_FAILED: types::MQRC = types::MQRC(2286);
    pub const MQRC_TERMINATION_FAILED: types::MQRC = types::MQRC(2287);
    pub const MQRC_UNKNOWN_Q_NAME: types::MQRC = types::MQRC(2288);
    pub const MQRC_SERVICE_ERROR: types::MQRC = types::MQRC(2289);
    pub const MQRC_Q_ALREADY_EXISTS: types::MQRC = types::MQRC(2290);
    pub const MQRC_USER_ID_NOT_AVAILABLE: types::MQRC = types::MQRC(2291);
    pub const MQRC_UNKNOWN_ENTITY: types::MQRC = types::MQRC(2292);
    pub const MQRC_UNKNOWN_AUTH_ENTITY: types::MQRC = types::MQRC(2293);
    pub const MQRC_UNKNOWN_REF_OBJECT: types::MQRC = types::MQRC(2294);
    pub const MQRC_CHANNEL_ACTIVATED: types::MQRC = types::MQRC(2295);
    pub const MQRC_CHANNEL_NOT_ACTIVATED: types::MQRC = types::MQRC(2296);
    pub const MQRC_UOW_CANCELED: types::MQRC = types::MQRC(2297);
    pub const MQRC_FUNCTION_NOT_SUPPORTED: types::MQRC = types::MQRC(2298);
    pub const MQRC_SELECTOR_TYPE_ERROR: types::MQRC = types::MQRC(2299);
    pub const MQRC_COMMAND_TYPE_ERROR: types::MQRC = types::MQRC(2300);
    pub const MQRC_MULTIPLE_INSTANCE_ERROR: types::MQRC = types::MQRC(2301);
    pub const MQRC_SYSTEM_ITEM_NOT_ALTERABLE: types::MQRC = types::MQRC(2302);
    pub const MQRC_BAG_CONVERSION_ERROR: types::MQRC = types::MQRC(2303);
    pub const MQRC_SELECTOR_OUT_OF_RANGE: types::MQRC = types::MQRC(2304);
    pub const MQRC_SELECTOR_NOT_UNIQUE: types::MQRC = types::MQRC(2305);
    pub const MQRC_INDEX_NOT_PRESENT: types::MQRC = types::MQRC(2306);
    pub const MQRC_STRING_ERROR: types::MQRC = types::MQRC(2307);
    pub const MQRC_ENCODING_NOT_SUPPORTED: types::MQRC = types::MQRC(2308);
    pub const MQRC_SELECTOR_NOT_PRESENT: types::MQRC = types::MQRC(2309);
    pub const MQRC_OUT_SELECTOR_ERROR: types::MQRC = types::MQRC(2310);
    pub const MQRC_STRING_TRUNCATED: types::MQRC = types::MQRC(2311);
    pub const MQRC_SELECTOR_WRONG_TYPE: types::MQRC = types::MQRC(2312);
    pub const MQRC_INCONSISTENT_ITEM_TYPE: types::MQRC = types::MQRC(2313);
    pub const MQRC_INDEX_ERROR: types::MQRC = types::MQRC(2314);
    pub const MQRC_SYSTEM_BAG_NOT_ALTERABLE: types::MQRC = types::MQRC(2315);
    pub const MQRC_ITEM_COUNT_ERROR: types::MQRC = types::MQRC(2316);
    pub const MQRC_FORMAT_NOT_SUPPORTED: types::MQRC = types::MQRC(2317);
    pub const MQRC_SELECTOR_NOT_SUPPORTED: types::MQRC = types::MQRC(2318);
    pub const MQRC_ITEM_VALUE_ERROR: types::MQRC = types::MQRC(2319);
    pub const MQRC_HBAG_ERROR: types::MQRC = types::MQRC(2320);
    pub const MQRC_PARAMETER_MISSING: types::MQRC = types::MQRC(2321);
    pub const MQRC_CMD_SERVER_NOT_AVAILABLE: types::MQRC = types::MQRC(2322);
    pub const MQRC_STRING_LENGTH_ERROR: types::MQRC = types::MQRC(2323);
    pub const MQRC_INQUIRY_COMMAND_ERROR: types::MQRC = types::MQRC(2324);
    pub const MQRC_NESTED_BAG_NOT_SUPPORTED: types::MQRC = types::MQRC(2325);
    pub const MQRC_BAG_WRONG_TYPE: types::MQRC = types::MQRC(2326);
    pub const MQRC_ITEM_TYPE_ERROR: types::MQRC = types::MQRC(2327);
    pub const MQRC_SYSTEM_BAG_NOT_DELETABLE: types::MQRC = types::MQRC(2328);
    pub const MQRC_SYSTEM_ITEM_NOT_DELETABLE: types::MQRC = types::MQRC(2329);
    pub const MQRC_CODED_CHAR_SET_ID_ERROR: types::MQRC = types::MQRC(2330);
    pub const MQRC_MSG_TOKEN_ERROR: types::MQRC = types::MQRC(2331);
    pub const MQRC_MISSING_WIH: types::MQRC = types::MQRC(2332);
    pub const MQRC_WIH_ERROR: types::MQRC = types::MQRC(2333);
    pub const MQRC_RFH_ERROR: types::MQRC = types::MQRC(2334);
    pub const MQRC_RFH_STRING_ERROR: types::MQRC = types::MQRC(2335);
    pub const MQRC_RFH_COMMAND_ERROR: types::MQRC = types::MQRC(2336);
    pub const MQRC_RFH_PARM_ERROR: types::MQRC = types::MQRC(2337);
    pub const MQRC_RFH_DUPLICATE_PARM: types::MQRC = types::MQRC(2338);
    pub const MQRC_RFH_PARM_MISSING: types::MQRC = types::MQRC(2339);
    pub const MQRC_CHAR_CONVERSION_ERROR: types::MQRC = types::MQRC(2340);
    pub const MQRC_UCS2_CONVERSION_ERROR: types::MQRC = types::MQRC(2341);
    pub const MQRC_DB2_NOT_AVAILABLE: types::MQRC = types::MQRC(2342);
    pub const MQRC_OBJECT_NOT_UNIQUE: types::MQRC = types::MQRC(2343);
    pub const MQRC_CONN_TAG_NOT_RELEASED: types::MQRC = types::MQRC(2344);
    pub const MQRC_CF_NOT_AVAILABLE: types::MQRC = types::MQRC(2345);
    pub const MQRC_CF_STRUC_IN_USE: types::MQRC = types::MQRC(2346);
    pub const MQRC_CF_STRUC_LIST_HDR_IN_USE: types::MQRC = types::MQRC(2347);
    pub const MQRC_CF_STRUC_AUTH_FAILED: types::MQRC = types::MQRC(2348);
    pub const MQRC_CF_STRUC_ERROR: types::MQRC = types::MQRC(2349);
    pub const MQRC_CONN_TAG_NOT_USABLE: types::MQRC = types::MQRC(2350);
    pub const MQRC_GLOBAL_UOW_CONFLICT: types::MQRC = types::MQRC(2351);
    pub const MQRC_LOCAL_UOW_CONFLICT: types::MQRC = types::MQRC(2352);
    pub const MQRC_HANDLE_IN_USE_FOR_UOW: types::MQRC = types::MQRC(2353);
    pub const MQRC_UOW_ENLISTMENT_ERROR: types::MQRC = types::MQRC(2354);
    pub const MQRC_UOW_MIX_NOT_SUPPORTED: types::MQRC = types::MQRC(2355);
    pub const MQRC_WXP_ERROR: types::MQRC = types::MQRC(2356);
    pub const MQRC_CURRENT_RECORD_ERROR: types::MQRC = types::MQRC(2357);
    pub const MQRC_NEXT_OFFSET_ERROR: types::MQRC = types::MQRC(2358);
    pub const MQRC_NO_RECORD_AVAILABLE: types::MQRC = types::MQRC(2359);
    pub const MQRC_OBJECT_LEVEL_INCOMPATIBLE: types::MQRC = types::MQRC(2360);
    pub const MQRC_NEXT_RECORD_ERROR: types::MQRC = types::MQRC(2361);
    pub const MQRC_BACKOUT_THRESHOLD_REACHED: types::MQRC = types::MQRC(2362);
    pub const MQRC_MSG_NOT_MATCHED: types::MQRC = types::MQRC(2363);
    pub const MQRC_JMS_FORMAT_ERROR: types::MQRC = types::MQRC(2364);
    pub const MQRC_SEGMENTS_NOT_SUPPORTED: types::MQRC = types::MQRC(2365);
    pub const MQRC_WRONG_CF_LEVEL: types::MQRC = types::MQRC(2366);
    pub const MQRC_CONFIG_CREATE_OBJECT: types::MQRC = types::MQRC(2367);
    pub const MQRC_CONFIG_CHANGE_OBJECT: types::MQRC = types::MQRC(2368);
    pub const MQRC_CONFIG_DELETE_OBJECT: types::MQRC = types::MQRC(2369);
    pub const MQRC_CONFIG_REFRESH_OBJECT: types::MQRC = types::MQRC(2370);
    pub const MQRC_CHANNEL_SSL_ERROR: types::MQRC = types::MQRC(2371);
    pub const MQRC_PARTICIPANT_NOT_DEFINED: types::MQRC = types::MQRC(2372);
    pub const MQRC_CF_STRUC_FAILED: types::MQRC = types::MQRC(2373);
    pub const MQRC_API_EXIT_ERROR: types::MQRC = types::MQRC(2374);
    pub const MQRC_API_EXIT_INIT_ERROR: types::MQRC = types::MQRC(2375);
    pub const MQRC_API_EXIT_TERM_ERROR: types::MQRC = types::MQRC(2376);
    pub const MQRC_EXIT_REASON_ERROR: types::MQRC = types::MQRC(2377);
    pub const MQRC_RESERVED_VALUE_ERROR: types::MQRC = types::MQRC(2378);
    pub const MQRC_NO_DATA_AVAILABLE: types::MQRC = types::MQRC(2379);
    pub const MQRC_SCO_ERROR: types::MQRC = types::MQRC(2380);
    pub const MQRC_KEY_REPOSITORY_ERROR: types::MQRC = types::MQRC(2381);
    pub const MQRC_CRYPTO_HARDWARE_ERROR: types::MQRC = types::MQRC(2382);
    pub const MQRC_AUTH_INFO_REC_COUNT_ERROR: types::MQRC = types::MQRC(2383);
    pub const MQRC_AUTH_INFO_REC_ERROR: types::MQRC = types::MQRC(2384);
    pub const MQRC_AIR_ERROR: types::MQRC = types::MQRC(2385);
    pub const MQRC_AUTH_INFO_TYPE_ERROR: types::MQRC = types::MQRC(2386);
    pub const MQRC_AUTH_INFO_CONN_NAME_ERROR: types::MQRC = types::MQRC(2387);
    pub const MQRC_LDAP_USER_NAME_ERROR: types::MQRC = types::MQRC(2388);
    pub const MQRC_LDAP_USER_NAME_LENGTH_ERR: types::MQRC = types::MQRC(2389);
    pub const MQRC_LDAP_PASSWORD_ERROR: types::MQRC = types::MQRC(2390);
    pub const MQRC_SSL_ALREADY_INITIALIZED: types::MQRC = types::MQRC(2391);
    pub const MQRC_SSL_CONFIG_ERROR: types::MQRC = types::MQRC(2392);
    pub const MQRC_SSL_INITIALIZATION_ERROR: types::MQRC = types::MQRC(2393);
    pub const MQRC_Q_INDEX_TYPE_ERROR: types::MQRC = types::MQRC(2394);
    pub const MQRC_CFBS_ERROR: types::MQRC = types::MQRC(2395);
    pub const MQRC_SSL_NOT_ALLOWED: types::MQRC = types::MQRC(2396);
    pub const MQRC_JSSE_ERROR: types::MQRC = types::MQRC(2397);
    pub const MQRC_SSL_PEER_NAME_MISMATCH: types::MQRC = types::MQRC(2398);
    pub const MQRC_SSL_PEER_NAME_ERROR: types::MQRC = types::MQRC(2399);
    pub const MQRC_UNSUPPORTED_CIPHER_SUITE: types::MQRC = types::MQRC(2400);
    pub const MQRC_SSL_CERTIFICATE_REVOKED: types::MQRC = types::MQRC(2401);
    pub const MQRC_SSL_CERT_STORE_ERROR: types::MQRC = types::MQRC(2402);
    pub const MQRC_CLIENT_EXIT_LOAD_ERROR: types::MQRC = types::MQRC(2406);
    pub const MQRC_CLIENT_EXIT_ERROR: types::MQRC = types::MQRC(2407);
    pub const MQRC_UOW_COMMITTED: types::MQRC = types::MQRC(2408);
    pub const MQRC_SSL_KEY_RESET_ERROR: types::MQRC = types::MQRC(2409);
    pub const MQRC_UNKNOWN_COMPONENT_NAME: types::MQRC = types::MQRC(2410);
    pub const MQRC_LOGGER_STATUS: types::MQRC = types::MQRC(2411);
    pub const MQRC_COMMAND_MQSC: types::MQRC = types::MQRC(2412);
    pub const MQRC_COMMAND_PCF: types::MQRC = types::MQRC(2413);
    pub const MQRC_CFIF_ERROR: types::MQRC = types::MQRC(2414);
    pub const MQRC_CFSF_ERROR: types::MQRC = types::MQRC(2415);
    pub const MQRC_CFGR_ERROR: types::MQRC = types::MQRC(2416);
    pub const MQRC_MSG_NOT_ALLOWED_IN_GROUP: types::MQRC = types::MQRC(2417);
    pub const MQRC_FILTER_OPERATOR_ERROR: types::MQRC = types::MQRC(2418);
    pub const MQRC_NESTED_SELECTOR_ERROR: types::MQRC = types::MQRC(2419);
    pub const MQRC_EPH_ERROR: types::MQRC = types::MQRC(2420);
    pub const MQRC_RFH_FORMAT_ERROR: types::MQRC = types::MQRC(2421);
    pub const MQRC_CFBF_ERROR: types::MQRC = types::MQRC(2422);
    pub const MQRC_CLIENT_CHANNEL_CONFLICT: types::MQRC = types::MQRC(2423);
    pub const MQRC_SD_ERROR: types::MQRC = types::MQRC(2424);
    pub const MQRC_TOPIC_STRING_ERROR: types::MQRC = types::MQRC(2425);
    pub const MQRC_STS_ERROR: types::MQRC = types::MQRC(2426);
    pub const MQRC_NO_SUBSCRIPTION: types::MQRC = types::MQRC(2428);
    pub const MQRC_SUBSCRIPTION_IN_USE: types::MQRC = types::MQRC(2429);
    pub const MQRC_STAT_TYPE_ERROR: types::MQRC = types::MQRC(2430);
    pub const MQRC_SUB_USER_DATA_ERROR: types::MQRC = types::MQRC(2431);
    pub const MQRC_SUB_ALREADY_EXISTS: types::MQRC = types::MQRC(2432);
    pub const MQRC_IDENTITY_MISMATCH: types::MQRC = types::MQRC(2434);
    pub const MQRC_ALTER_SUB_ERROR: types::MQRC = types::MQRC(2435);
    pub const MQRC_DURABILITY_NOT_ALLOWED: types::MQRC = types::MQRC(2436);
    pub const MQRC_NO_RETAINED_MSG: types::MQRC = types::MQRC(2437);
    pub const MQRC_SRO_ERROR: types::MQRC = types::MQRC(2438);
    pub const MQRC_SUB_NAME_ERROR: types::MQRC = types::MQRC(2440);
    pub const MQRC_OBJECT_STRING_ERROR: types::MQRC = types::MQRC(2441);
    pub const MQRC_PROPERTY_NAME_ERROR: types::MQRC = types::MQRC(2442);
    pub const MQRC_SEGMENTATION_NOT_ALLOWED: types::MQRC = types::MQRC(2443);
    pub const MQRC_CBD_ERROR: types::MQRC = types::MQRC(2444);
    pub const MQRC_CTLO_ERROR: types::MQRC = types::MQRC(2445);
    pub const MQRC_NO_CALLBACKS_ACTIVE: types::MQRC = types::MQRC(2446);
    pub const MQRC_CALLBACK_NOT_REGISTERED: types::MQRC = types::MQRC(2448);
    pub const MQRC_OPTIONS_CHANGED: types::MQRC = types::MQRC(2457);
    pub const MQRC_READ_AHEAD_MSGS: types::MQRC = types::MQRC(2458);
    pub const MQRC_SELECTOR_SYNTAX_ERROR: types::MQRC = types::MQRC(2459);
    pub const MQRC_HMSG_ERROR: types::MQRC = types::MQRC(2460);
    pub const MQRC_CMHO_ERROR: types::MQRC = types::MQRC(2461);
    pub const MQRC_DMHO_ERROR: types::MQRC = types::MQRC(2462);
    pub const MQRC_SMPO_ERROR: types::MQRC = types::MQRC(2463);
    pub const MQRC_IMPO_ERROR: types::MQRC = types::MQRC(2464);
    pub const MQRC_PROPERTY_NAME_TOO_BIG: types::MQRC = types::MQRC(2465);
    pub const MQRC_PROP_VALUE_NOT_CONVERTED: types::MQRC = types::MQRC(2466);
    pub const MQRC_PROP_TYPE_NOT_SUPPORTED: types::MQRC = types::MQRC(2467);
    pub const MQRC_PROPERTY_VALUE_TOO_BIG: types::MQRC = types::MQRC(2469);
    pub const MQRC_PROP_CONV_NOT_SUPPORTED: types::MQRC = types::MQRC(2470);
    pub const MQRC_PROPERTY_NOT_AVAILABLE: types::MQRC = types::MQRC(2471);
    pub const MQRC_PROP_NUMBER_FORMAT_ERROR: types::MQRC = types::MQRC(2472);
    pub const MQRC_PROPERTY_TYPE_ERROR: types::MQRC = types::MQRC(2473);
    pub const MQRC_PROPERTIES_TOO_BIG: types::MQRC = types::MQRC(2478);
    pub const MQRC_PUT_NOT_RETAINED: types::MQRC = types::MQRC(2479);
    pub const MQRC_ALIAS_TARGTYPE_CHANGED: types::MQRC = types::MQRC(2480);
    pub const MQRC_DMPO_ERROR: types::MQRC = types::MQRC(2481);
    pub const MQRC_PD_ERROR: types::MQRC = types::MQRC(2482);
    pub const MQRC_CALLBACK_TYPE_ERROR: types::MQRC = types::MQRC(2483);
    pub const MQRC_CBD_OPTIONS_ERROR: types::MQRC = types::MQRC(2484);
    pub const MQRC_MAX_MSG_LENGTH_ERROR: types::MQRC = types::MQRC(2485);
    pub const MQRC_CALLBACK_ROUTINE_ERROR: types::MQRC = types::MQRC(2486);
    pub const MQRC_CALLBACK_LINK_ERROR: types::MQRC = types::MQRC(2487);
    pub const MQRC_OPERATION_ERROR: types::MQRC = types::MQRC(2488);
    pub const MQRC_BMHO_ERROR: types::MQRC = types::MQRC(2489);
    pub const MQRC_UNSUPPORTED_PROPERTY: types::MQRC = types::MQRC(2490);
    pub const MQRC_MSG_LENGTH_ERROR: types::MQRC = types::MQRC(2491);
    pub const MQRC_PROP_NAME_NOT_CONVERTED: types::MQRC = types::MQRC(2492);
    pub const MQRC_GET_ENABLED: types::MQRC = types::MQRC(2494);
    pub const MQRC_MODULE_NOT_FOUND: types::MQRC = types::MQRC(2495);
    pub const MQRC_MODULE_INVALID: types::MQRC = types::MQRC(2496);
    pub const MQRC_MODULE_ENTRY_NOT_FOUND: types::MQRC = types::MQRC(2497);
    pub const MQRC_MIXED_CONTENT_NOT_ALLOWED: types::MQRC = types::MQRC(2498);
    pub const MQRC_MSG_HANDLE_IN_USE: types::MQRC = types::MQRC(2499);
    pub const MQRC_HCONN_ASYNC_ACTIVE: types::MQRC = types::MQRC(2500);
    pub const MQRC_MHBO_ERROR: types::MQRC = types::MQRC(2501);
    pub const MQRC_PUBLICATION_FAILURE: types::MQRC = types::MQRC(2502);
    pub const MQRC_SUB_INHIBITED: types::MQRC = types::MQRC(2503);
    pub const MQRC_SELECTOR_ALWAYS_FALSE: types::MQRC = types::MQRC(2504);
    pub const MQRC_XEPO_ERROR: types::MQRC = types::MQRC(2507);
    pub const MQRC_DURABILITY_NOT_ALTERABLE: types::MQRC = types::MQRC(2509);
    pub const MQRC_TOPIC_NOT_ALTERABLE: types::MQRC = types::MQRC(2510);
    pub const MQRC_SUBLEVEL_NOT_ALTERABLE: types::MQRC = types::MQRC(2512);
    pub const MQRC_PROPERTY_NAME_LENGTH_ERR: types::MQRC = types::MQRC(2513);
    pub const MQRC_DUPLICATE_GROUP_SUB: types::MQRC = types::MQRC(2514);
    pub const MQRC_GROUPING_NOT_ALTERABLE: types::MQRC = types::MQRC(2515);
    pub const MQRC_SELECTOR_INVALID_FOR_TYPE: types::MQRC = types::MQRC(2516);
    pub const MQRC_HOBJ_QUIESCED: types::MQRC = types::MQRC(2517);
    pub const MQRC_HOBJ_QUIESCED_NO_MSGS: types::MQRC = types::MQRC(2518);
    pub const MQRC_SELECTION_STRING_ERROR: types::MQRC = types::MQRC(2519);
    pub const MQRC_RES_OBJECT_STRING_ERROR: types::MQRC = types::MQRC(2520);
    pub const MQRC_CONNECTION_SUSPENDED: types::MQRC = types::MQRC(2521);
    pub const MQRC_INVALID_DESTINATION: types::MQRC = types::MQRC(2522);
    pub const MQRC_INVALID_SUBSCRIPTION: types::MQRC = types::MQRC(2523);
    pub const MQRC_SELECTOR_NOT_ALTERABLE: types::MQRC = types::MQRC(2524);
    pub const MQRC_RETAINED_MSG_Q_ERROR: types::MQRC = types::MQRC(2525);
    pub const MQRC_RETAINED_NOT_DELIVERED: types::MQRC = types::MQRC(2526);
    pub const MQRC_RFH_RESTRICTED_FORMAT_ERR: types::MQRC = types::MQRC(2527);
    pub const MQRC_CONNECTION_STOPPED: types::MQRC = types::MQRC(2528);
    pub const MQRC_ASYNC_UOW_CONFLICT: types::MQRC = types::MQRC(2529);
    pub const MQRC_ASYNC_XA_CONFLICT: types::MQRC = types::MQRC(2530);
    pub const MQRC_PUBSUB_INHIBITED: types::MQRC = types::MQRC(2531);
    pub const MQRC_MSG_HANDLE_COPY_FAILURE: types::MQRC = types::MQRC(2532);
    pub const MQRC_DEST_CLASS_NOT_ALTERABLE: types::MQRC = types::MQRC(2533);
    pub const MQRC_OPERATION_NOT_ALLOWED: types::MQRC = types::MQRC(2534);
    pub const MQRC_ACTION_ERROR: types::MQRC = types::MQRC(2535);
    pub const MQRC_CHANNEL_NOT_AVAILABLE: types::MQRC = types::MQRC(2537);
    pub const MQRC_HOST_NOT_AVAILABLE: types::MQRC = types::MQRC(2538);
    pub const MQRC_CHANNEL_CONFIG_ERROR: types::MQRC = types::MQRC(2539);
    pub const MQRC_UNKNOWN_CHANNEL_NAME: types::MQRC = types::MQRC(2540);
    pub const MQRC_LOOPING_PUBLICATION: types::MQRC = types::MQRC(2541);
    pub const MQRC_ALREADY_JOINED: types::MQRC = types::MQRC(2542);
    pub const MQRC_STANDBY_Q_MGR: types::MQRC = types::MQRC(2543);
    pub const MQRC_RECONNECTING: types::MQRC = types::MQRC(2544);
    pub const MQRC_RECONNECTED: types::MQRC = types::MQRC(2545);
    pub const MQRC_RECONNECT_QMID_MISMATCH: types::MQRC = types::MQRC(2546);
    pub const MQRC_RECONNECT_INCOMPATIBLE: types::MQRC = types::MQRC(2547);
    pub const MQRC_RECONNECT_FAILED: types::MQRC = types::MQRC(2548);
    pub const MQRC_CALL_INTERRUPTED: types::MQRC = types::MQRC(2549);
    pub const MQRC_NO_SUBS_MATCHED: types::MQRC = types::MQRC(2550);
    pub const MQRC_SELECTION_NOT_AVAILABLE: types::MQRC = types::MQRC(2551);
    pub const MQRC_CHANNEL_SSL_WARNING: types::MQRC = types::MQRC(2552);
    pub const MQRC_OCSP_URL_ERROR: types::MQRC = types::MQRC(2553);
    pub const MQRC_CONTENT_ERROR: types::MQRC = types::MQRC(2554);
    pub const MQRC_RECONNECT_Q_MGR_REQD: types::MQRC = types::MQRC(2555);
    pub const MQRC_RECONNECT_TIMED_OUT: types::MQRC = types::MQRC(2556);
    pub const MQRC_PUBLISH_EXIT_ERROR: types::MQRC = types::MQRC(2557);
    pub const MQRC_COMMINFO_ERROR: types::MQRC = types::MQRC(2558);
    pub const MQRC_DEF_SYNCPOINT_INHIBITED: types::MQRC = types::MQRC(2559);
    pub const MQRC_MULTICAST_ONLY: types::MQRC = types::MQRC(2560);
    pub const MQRC_DATA_SET_NOT_AVAILABLE: types::MQRC = types::MQRC(2561);
    pub const MQRC_GROUPING_NOT_ALLOWED: types::MQRC = types::MQRC(2562);
    pub const MQRC_GROUP_ADDRESS_ERROR: types::MQRC = types::MQRC(2563);
    pub const MQRC_MULTICAST_CONFIG_ERROR: types::MQRC = types::MQRC(2564);
    pub const MQRC_MULTICAST_INTERFACE_ERROR: types::MQRC = types::MQRC(2565);
    pub const MQRC_MULTICAST_SEND_ERROR: types::MQRC = types::MQRC(2566);
    pub const MQRC_MULTICAST_INTERNAL_ERROR: types::MQRC = types::MQRC(2567);
    pub const MQRC_CONNECTION_NOT_AVAILABLE: types::MQRC = types::MQRC(2568);
    pub const MQRC_SYNCPOINT_NOT_ALLOWED: types::MQRC = types::MQRC(2569);
    pub const MQRC_SSL_ALT_PROVIDER_REQUIRED: types::MQRC = types::MQRC(2570);
    pub const MQRC_MCAST_PUB_STATUS: types::MQRC = types::MQRC(2571);
    pub const MQRC_MCAST_SUB_STATUS: types::MQRC = types::MQRC(2572);
    pub const MQRC_PRECONN_EXIT_LOAD_ERROR: types::MQRC = types::MQRC(2573);
    pub const MQRC_PRECONN_EXIT_NOT_FOUND: types::MQRC = types::MQRC(2574);
    pub const MQRC_PRECONN_EXIT_ERROR: types::MQRC = types::MQRC(2575);
    pub const MQRC_CD_ARRAY_ERROR: types::MQRC = types::MQRC(2576);
    pub const MQRC_CHANNEL_BLOCKED: types::MQRC = types::MQRC(2577);
    pub const MQRC_CHANNEL_BLOCKED_WARNING: types::MQRC = types::MQRC(2578);
    pub const MQRC_SUBSCRIPTION_CREATE: types::MQRC = types::MQRC(2579);
    pub const MQRC_SUBSCRIPTION_DELETE: types::MQRC = types::MQRC(2580);
    pub const MQRC_SUBSCRIPTION_CHANGE: types::MQRC = types::MQRC(2581);
    pub const MQRC_SUBSCRIPTION_REFRESH: types::MQRC = types::MQRC(2582);
    pub const MQRC_INSTALLATION_MISMATCH: types::MQRC = types::MQRC(2583);
    pub const MQRC_NOT_PRIVILEGED: types::MQRC = types::MQRC(2584);
    pub const MQRC_PROPERTIES_DISABLED: types::MQRC = types::MQRC(2586);
    pub const MQRC_HMSG_NOT_AVAILABLE: types::MQRC = types::MQRC(2587);
    pub const MQRC_EXIT_PROPS_NOT_SUPPORTED: types::MQRC = types::MQRC(2588);
    pub const MQRC_INSTALLATION_MISSING: types::MQRC = types::MQRC(2589);
    pub const MQRC_FASTPATH_NOT_AVAILABLE: types::MQRC = types::MQRC(2590);
    pub const MQRC_CIPHER_SPEC_NOT_SUITE_B: types::MQRC = types::MQRC(2591);
    pub const MQRC_SUITE_B_ERROR: types::MQRC = types::MQRC(2592);
    pub const MQRC_CERT_VAL_POLICY_ERROR: types::MQRC = types::MQRC(2593);
    pub const MQRC_PASSWORD_PROTECTION_ERROR: types::MQRC = types::MQRC(2594);
    pub const MQRC_CSP_ERROR: types::MQRC = types::MQRC(2595);
    pub const MQRC_CERT_LABEL_NOT_ALLOWED: types::MQRC = types::MQRC(2596);
    pub const MQRC_ADMIN_TOPIC_STRING_ERROR: types::MQRC = types::MQRC(2598);
    pub const MQRC_AMQP_NOT_AVAILABLE: types::MQRC = types::MQRC(2599);
    pub const MQRC_CCDT_URL_ERROR: types::MQRC = types::MQRC(2600);
    pub const MQRC_Q_MGR_RECONNECT_REQUESTED: types::MQRC = types::MQRC(2601);
    pub const MQRC_BNO_ERROR: types::MQRC = types::MQRC(2602);
    pub const MQRC_OUTBOUND_SNI_NOT_VALID: types::MQRC = types::MQRC(2603);
    pub const MQRC_HTTPS_KEYSTORE_ERROR: types::MQRC = types::MQRC(2604);
    pub const MQRC_REOPEN_EXCL_INPUT_ERROR: types::MQRC = types::MQRC(6100);
    pub const MQRC_REOPEN_INQUIRE_ERROR: types::MQRC = types::MQRC(6101);
    pub const MQRC_REOPEN_SAVED_CONTEXT_ERR: types::MQRC = types::MQRC(6102);
    pub const MQRC_REOPEN_TEMPORARY_Q_ERROR: types::MQRC = types::MQRC(6103);
    pub const MQRC_ATTRIBUTE_LOCKED: types::MQRC = types::MQRC(6104);
    pub const MQRC_CURSOR_NOT_VALID: types::MQRC = types::MQRC(6105);
    pub const MQRC_ENCODING_ERROR: types::MQRC = types::MQRC(6106);
    pub const MQRC_STRUC_ID_ERROR: types::MQRC = types::MQRC(6107);
    pub const MQRC_NULL_POINTER: types::MQRC = types::MQRC(6108);
    pub const MQRC_NO_CONNECTION_REFERENCE: types::MQRC = types::MQRC(6109);
    pub const MQRC_NO_BUFFER: types::MQRC = types::MQRC(6110);
    pub const MQRC_BINARY_DATA_LENGTH_ERROR: types::MQRC = types::MQRC(6111);
    pub const MQRC_BUFFER_NOT_AUTOMATIC: types::MQRC = types::MQRC(6112);
    pub const MQRC_INSUFFICIENT_BUFFER: types::MQRC = types::MQRC(6113);
    pub const MQRC_INSUFFICIENT_DATA: types::MQRC = types::MQRC(6114);
    pub const MQRC_DATA_TRUNCATED: types::MQRC = types::MQRC(6115);
    pub const MQRC_ZERO_LENGTH: types::MQRC = types::MQRC(6116);
    pub const MQRC_NEGATIVE_LENGTH: types::MQRC = types::MQRC(6117);
    pub const MQRC_NEGATIVE_OFFSET: types::MQRC = types::MQRC(6118);
    pub const MQRC_INCONSISTENT_FORMAT: types::MQRC = types::MQRC(6119);
    pub const MQRC_INCONSISTENT_OBJECT_STATE: types::MQRC = types::MQRC(6120);
    pub const MQRC_CONTEXT_OBJECT_NOT_VALID: types::MQRC = types::MQRC(6121);
    pub const MQRC_CONTEXT_OPEN_ERROR: types::MQRC = types::MQRC(6122);
    pub const MQRC_STRUC_LENGTH_ERROR: types::MQRC = types::MQRC(6123);
    pub const MQRC_NOT_CONNECTED: types::MQRC = types::MQRC(6124);
    pub const MQRC_NOT_OPEN: types::MQRC = types::MQRC(6125);
    pub const MQRC_DISTRIBUTION_LIST_EMPTY: types::MQRC = types::MQRC(6126);
    pub const MQRC_INCONSISTENT_OPEN_OPTIONS: types::MQRC = types::MQRC(6127);
    pub const MQRC_WRONG_VERSION: types::MQRC = types::MQRC(6128);
    pub const MQRC_REFERENCE_ERROR: types::MQRC = types::MQRC(6129);
    pub const MQRC_XR_NOT_AVAILABLE: types::MQRC = types::MQRC(6130);
    pub const MQRC_SUB_JOIN_NOT_ALTERABLE: types::MQRC = types::MQRC(29440);
    pub const MQRC_APPL_FIRST: types::MQRC = types::MQRC(900);
    pub const MQRC_APPL_LAST: types::MQRC = types::MQRC(999);
    pub const MQRD_NO_RECONNECT: types::MQRD = types::MQRD(-1);
    pub const MQRD_NO_DELAY: types::MQRD = types::MQRD(0);
    pub const MQREADA_NO: types::MQREADA = types::MQREADA(0);
    pub const MQREADA_YES: types::MQREADA = types::MQREADA(1);
    pub const MQREADA_DISABLED: types::MQREADA = types::MQREADA(2);
    pub const MQREADA_INHIBITED: types::MQREADA = types::MQREADA(3);
    pub const MQREADA_BACKLOG: types::MQREADA = types::MQREADA(4);
    pub const MQRECAUTO_NO: types::MQRECAUTO = types::MQRECAUTO(0);
    pub const MQRECAUTO_YES: types::MQRECAUTO = types::MQRECAUTO(1);
    pub const MQRECORDING_DISABLED: types::MQRECORDING = types::MQRECORDING(0);
    pub const MQRECORDING_Q: types::MQRECORDING = types::MQRECORDING(1);
    pub const MQRECORDING_MSG: types::MQRECORDING = types::MQRECORDING(2);
    pub const MQREORG_DISABLED: types::MQREORG = types::MQREORG(0);
    pub const MQREORG_ENABLED: types::MQREORG = types::MQREORG(1);
    pub const MQRFH_FLAGS_RESTRICTED_MASK: types::MQRFH = types::MQRFH(-65536);
    pub const MQRFH_NONE: types::MQRFH = types::MQRFH(0);
    pub const MQRFH_NO_FLAGS: types::MQRFH = types::MQRFH(0);
    pub const MQRL_UNDEFINED: types::MQRL = types::MQRL(-1);
    pub const MQRMHF_NOT_LAST: types::MQRMHF = types::MQRMHF(0);
    pub const MQRMHF_LAST: types::MQRMHF = types::MQRMHF(1);
    pub const MQRO_ACCEPT_UNSUP_MASK: types::MQRO = types::MQRO(-270532353);
    pub const MQRO_NONE: types::MQRO = types::MQRO(0);
    pub const MQRO_PAN: types::MQRO = types::MQRO(1);
    pub const MQRO_NAN: types::MQRO = types::MQRO(2);
    pub const MQRO_ACTIVITY: types::MQRO = types::MQRO(4);
    pub const MQRO_PASS_CORREL_ID: types::MQRO = types::MQRO(64);
    pub const MQRO_PASS_MSG_ID: types::MQRO = types::MQRO(128);
    pub const MQRO_COA: types::MQRO = types::MQRO(256);
    pub const MQRO_COA_WITH_DATA: types::MQRO = types::MQRO(768);
    pub const MQRO_COA_WITH_FULL_DATA: types::MQRO = types::MQRO(1792);
    pub const MQRO_COD: types::MQRO = types::MQRO(2048);
    pub const MQRO_COD_WITH_DATA: types::MQRO = types::MQRO(6144);
    pub const MQRO_COD_WITH_FULL_DATA: types::MQRO = types::MQRO(14336);
    pub const MQRO_PASS_DISCARD_AND_EXPIRY: types::MQRO = types::MQRO(16384);
    pub const MQRO_ACCEPT_UNSUP_IF_XMIT_MASK: types::MQRO = types::MQRO(261888);
    pub const MQRO_EXPIRATION: types::MQRO = types::MQRO(2097152);
    pub const MQRO_EXPIRATION_WITH_DATA: types::MQRO = types::MQRO(6291456);
    pub const MQRO_EXPIRATION_WITH_FULL_DATA: types::MQRO = types::MQRO(14680064);
    pub const MQRO_EXCEPTION: types::MQRO = types::MQRO(16777216);
    pub const MQRO_EXCEPTION_WITH_DATA: types::MQRO = types::MQRO(50331648);
    pub const MQRO_EXCEPTION_WITH_FULL_DATA: types::MQRO = types::MQRO(117440512);
    pub const MQRO_DISCARD_MSG: types::MQRO = types::MQRO(134217728);
    pub const MQRO_REJECT_UNSUP_MASK: types::MQRO = types::MQRO(270270464);
    pub const MQRO_COPY_MSG_ID_TO_CORREL_ID: types::MQRO = types::MQRO(0);
    pub const MQRO_DEAD_LETTER_Q: types::MQRO = types::MQRO(0);
    pub const MQRO_NEW_MSG_ID: types::MQRO = types::MQRO(0);
    pub const MQRU_PUBLISH_ON_REQUEST: types::MQRU = types::MQRU(1);
    pub const MQRU_PUBLISH_ALL: types::MQRU = types::MQRU(2);
    pub const MQSCOPE_ALL: types::MQSCOPE = types::MQSCOPE(0);
    pub const MQSCOPE_AS_PARENT: types::MQSCOPE = types::MQSCOPE(1);
    pub const MQSCOPE_QMGR: types::MQSCOPE = types::MQSCOPE(4);
    pub const MQSCO_Q_MGR: types::MQSCO = types::MQSCO(1);
    pub const MQSCO_CELL: types::MQSCO = types::MQSCO(2);
    pub const MQSCO_RESET_COUNT_DEFAULT: types::MQSCO = types::MQSCO(0);
    pub const MQSCYC_UPPER: types::MQSCYC = types::MQSCYC(0);
    pub const MQSCYC_MIXED: types::MQSCYC = types::MQSCYC(1);
    pub const MQSMPO_NONE: types::MQSMPO = types::MQSMPO(0);
    pub const MQSMPO_SET_PROP_UNDER_CURSOR: types::MQSMPO = types::MQSMPO(1);
    pub const MQSMPO_SET_PROP_AFTER_CURSOR: types::MQSMPO = types::MQSMPO(2);
    pub const MQSMPO_APPEND_PROPERTY: types::MQSMPO = types::MQSMPO(4);
    pub const MQSMPO_SET_PROP_BEFORE_CURSOR: types::MQSMPO = types::MQSMPO(8);
    pub const MQSMPO_SET_FIRST: types::MQSMPO = types::MQSMPO(0);
    pub const MQSO_NONE: types::MQSO = types::MQSO(0);
    pub const MQSO_ALTER: types::MQSO = types::MQSO(1);
    pub const MQSO_CREATE: types::MQSO = types::MQSO(2);
    pub const MQSO_RESUME: types::MQSO = types::MQSO(4);
    pub const MQSO_DURABLE: types::MQSO = types::MQSO(8);
    pub const MQSO_GROUP_SUB: types::MQSO = types::MQSO(16);
    pub const MQSO_MANAGED: types::MQSO = types::MQSO(32);
    pub const MQSO_SET_IDENTITY_CONTEXT: types::MQSO = types::MQSO(64);
    pub const MQSO_NO_MULTICAST: types::MQSO = types::MQSO(128);
    pub const MQSO_FIXED_USERID: types::MQSO = types::MQSO(256);
    pub const MQSO_ANY_USERID: types::MQSO = types::MQSO(512);
    pub const MQSO_PUBLICATIONS_ON_REQUEST: types::MQSO = types::MQSO(2048);
    pub const MQSO_NEW_PUBLICATIONS_ONLY: types::MQSO = types::MQSO(4096);
    pub const MQSO_FAIL_IF_QUIESCING: types::MQSO = types::MQSO(8192);
    pub const MQSO_ALTERNATE_USER_AUTHORITY: types::MQSO = types::MQSO(262144);
    pub const MQSO_WILDCARD_CHAR: types::MQSO = types::MQSO(1048576);
    pub const MQSO_WILDCARD_TOPIC: types::MQSO = types::MQSO(2097152);
    pub const MQSO_SET_CORREL_ID: types::MQSO = types::MQSO(4194304);
    pub const MQSO_SCOPE_QMGR: types::MQSO = types::MQSO(67108864);
    pub const MQSO_NO_READ_AHEAD: types::MQSO = types::MQSO(134217728);
    pub const MQSO_READ_AHEAD: types::MQSO = types::MQSO(268435456);
    pub const MQSO_NON_DURABLE: types::MQSO = types::MQSO(0);
    pub const MQSO_READ_AHEAD_AS_Q_DEF: types::MQSO = types::MQSO(0);
    pub const MQSP_NOT_AVAILABLE: types::MQSP = types::MQSP(0);
    pub const MQSP_AVAILABLE: types::MQSP = types::MQSP(1);
    pub const MQSQQM_USE: types::MQSQQM = types::MQSQQM(0);
    pub const MQSQQM_IGNORE: types::MQSQQM = types::MQSQQM(1);
    pub const MQSRO_NONE: types::MQSRO = types::MQSRO(0);
    pub const MQSRO_FAIL_IF_QUIESCING: types::MQSRO = types::MQSRO(8192);
    pub const MQSR_ACTION_PUBLICATION: types::MQSR = types::MQSR(1);
    pub const MQSSL_FIPS_NO: types::MQSSL = types::MQSSL(0);
    pub const MQSSL_FIPS_YES: types::MQSSL = types::MQSSL(1);
    pub const MQSTAT_TYPE_ASYNC_ERROR: types::MQSTAT = types::MQSTAT(0);
    pub const MQSTAT_TYPE_RECONNECTION: types::MQSTAT = types::MQSTAT(1);
    pub const MQSTAT_TYPE_RECONNECTION_ERROR: types::MQSTAT = types::MQSTAT(2);
    pub const MQST_BEST_EFFORT: types::MQST = types::MQST(0);
    pub const MQST_MUST_DUP: types::MQST = types::MQST(1);
    pub const MQSUB_DURABLE_AS_PARENT: types::MQSUB = types::MQSUB(0);
    pub const MQSUB_DURABLE_ALLOWED: types::MQSUB = types::MQSUB(1);
    pub const MQSUB_DURABLE_INHIBITED: types::MQSUB = types::MQSUB(2);
    pub const MQSUB_DURABLE_ALL: types::MQSUB_DURABILITY = types::MQSUB_DURABILITY(-1);
    pub const MQSUB_DURABLE_YES: types::MQSUB_DURABILITY = types::MQSUB_DURABILITY(1);
    pub const MQSUB_DURABLE_NO: types::MQSUB_DURABILITY = types::MQSUB_DURABILITY(2);
    pub const MQSVC_CONTROL_Q_MGR: types::MQSVC_CONTROL = types::MQSVC_CONTROL(0);
    pub const MQSVC_CONTROL_Q_MGR_START: types::MQSVC_CONTROL = types::MQSVC_CONTROL(1);
    pub const MQSVC_CONTROL_MANUAL: types::MQSVC_CONTROL = types::MQSVC_CONTROL(2);
    pub const MQSVC_STATUS_STOPPED: types::MQSVC_STATUS = types::MQSVC_STATUS(0);
    pub const MQSVC_STATUS_STARTING: types::MQSVC_STATUS = types::MQSVC_STATUS(1);
    pub const MQSVC_STATUS_RUNNING: types::MQSVC_STATUS = types::MQSVC_STATUS(2);
    pub const MQSVC_STATUS_STOPPING: types::MQSVC_STATUS = types::MQSVC_STATUS(3);
    pub const MQSVC_STATUS_RETRYING: types::MQSVC_STATUS = types::MQSVC_STATUS(4);
    pub const MQSVC_TYPE_COMMAND: types::MQSVC_TYPE = types::MQSVC_TYPE(0);
    pub const MQSVC_TYPE_SERVER: types::MQSVC_TYPE = types::MQSVC_TYPE(1);
    pub const MQTA_BLOCK: types::MQTA = types::MQTA(1);
    pub const MQTA_PASSTHRU: types::MQTA = types::MQTA(2);
    pub const MQTA_PROXY_SUB_FORCE: types::MQTA_PROXY = types::MQTA_PROXY(1);
    pub const MQTA_PROXY_SUB_FIRSTUSE: types::MQTA_PROXY = types::MQTA_PROXY(2);
    pub const MQTA_PUB_AS_PARENT: types::MQTA_PUB = types::MQTA_PUB(0);
    pub const MQTA_PUB_INHIBITED: types::MQTA_PUB = types::MQTA_PUB(1);
    pub const MQTA_PUB_ALLOWED: types::MQTA_PUB = types::MQTA_PUB(2);
    pub const MQTA_SUB_AS_PARENT: types::MQTA_SUB = types::MQTA_SUB(0);
    pub const MQTA_SUB_INHIBITED: types::MQTA_SUB = types::MQTA_SUB(1);
    pub const MQTA_SUB_ALLOWED: types::MQTA_SUB = types::MQTA_SUB(2);
    pub const MQTCPKEEP_NO: types::MQTCPKEEP = types::MQTCPKEEP(0);
    pub const MQTCPKEEP_YES: types::MQTCPKEEP = types::MQTCPKEEP(1);
    pub const MQTCPSTACK_SINGLE: types::MQTCPSTACK = types::MQTCPSTACK(0);
    pub const MQTCPSTACK_MULTIPLE: types::MQTCPSTACK = types::MQTCPSTACK(1);
    pub const MQTC_OFF: types::MQTC = types::MQTC(0);
    pub const MQTC_ON: types::MQTC = types::MQTC(1);
    pub const MQTOPT_LOCAL: types::MQTOPT = types::MQTOPT(0);
    pub const MQTOPT_CLUSTER: types::MQTOPT = types::MQTOPT(1);
    pub const MQTOPT_ALL: types::MQTOPT = types::MQTOPT(2);
    pub const MQTRAXSTR_NO: types::MQTRAXSTR = types::MQTRAXSTR(0);
    pub const MQTRAXSTR_YES: types::MQTRAXSTR = types::MQTRAXSTR(1);
    pub const MQTRIGGER_RESTART_NO: types::MQTRIGGER = types::MQTRIGGER(0);
    pub const MQTRIGGER_RESTART_YES: types::MQTRIGGER = types::MQTRIGGER(1);
    pub const MQTSCOPE_QMGR: types::MQTSCOPE = types::MQTSCOPE(1);
    pub const MQTSCOPE_ALL: types::MQTSCOPE = types::MQTSCOPE(2);
    pub const MQTT_NONE: types::MQTT = types::MQTT(0);
    pub const MQTT_FIRST: types::MQTT = types::MQTT(1);
    pub const MQTT_EVERY: types::MQTT = types::MQTT(2);
    pub const MQTT_DEPTH: types::MQTT = types::MQTT(3);
    pub const MQTYPE_AS_SET: types::MQTYPE = types::MQTYPE(0);
    pub const MQTYPE_NULL: types::MQTYPE = types::MQTYPE(2);
    pub const MQTYPE_BOOLEAN: types::MQTYPE = types::MQTYPE(4);
    pub const MQTYPE_BYTE_STRING: types::MQTYPE = types::MQTYPE(8);
    pub const MQTYPE_INT8: types::MQTYPE = types::MQTYPE(16);
    pub const MQTYPE_INT16: types::MQTYPE = types::MQTYPE(32);
    pub const MQTYPE_INT32: types::MQTYPE = types::MQTYPE(64);
    pub const MQTYPE_INT64: types::MQTYPE = types::MQTYPE(128);
    pub const MQTYPE_FLOAT32: types::MQTYPE = types::MQTYPE(256);
    pub const MQTYPE_FLOAT64: types::MQTYPE = types::MQTYPE(512);
    pub const MQTYPE_STRING: types::MQTYPE = types::MQTYPE(1024);
    pub const MQTYPE_LONG: types::MQTYPE = types::MQTYPE(64);
    pub const MQUSEDLQ_AS_PARENT: types::MQUSEDLQ = types::MQUSEDLQ(0);
    pub const MQUSEDLQ_NO: types::MQUSEDLQ = types::MQUSEDLQ(1);
    pub const MQUSEDLQ_YES: types::MQUSEDLQ = types::MQUSEDLQ(2);
    pub const MQUSRC_MAP: types::MQUSRC = types::MQUSRC(0);
    pub const MQUSRC_NOACCESS: types::MQUSRC = types::MQUSRC(1);
    pub const MQUSRC_CHANNEL: types::MQUSRC = types::MQUSRC(2);
    pub const MQUS_NORMAL: types::MQUS = types::MQUS(0);
    pub const MQUS_TRANSMISSION: types::MQUS = types::MQUS(1);
    pub const MQVL_NULL_TERMINATED: types::MQVL = types::MQVL(-1);
    pub const MQVL_EMPTY_STRING: types::MQVL = types::MQVL(0);
    pub const MQVS_NULL_TERMINATED: types::MQVS = types::MQVS(-1);
    pub const MQVU_FIXED_USER: types::MQVU = types::MQVU(1);
    pub const MQVU_ANY_USER: types::MQVU = types::MQVU(2);
    pub const MQWARN_NO: types::MQWARN = types::MQWARN(0);
    pub const MQWARN_YES: types::MQWARN = types::MQWARN(1);
    pub const MQWIH_NONE: types::MQWIH = types::MQWIH(0);
    pub const MQWI_UNLIMITED: types::MQWI = types::MQWI(-1);
    pub const MQWS_DEFAULT: types::MQWS = types::MQWS(0);
    pub const MQWS_CHAR: types::MQWS = types::MQWS(1);
    pub const MQWS_TOPIC: types::MQWS = types::MQWS(2);
    pub const MQXC_MQOPEN: types::MQXC = types::MQXC(1);
    pub const MQXC_MQCLOSE: types::MQXC = types::MQXC(2);
    pub const MQXC_MQGET: types::MQXC = types::MQXC(3);
    pub const MQXC_MQPUT: types::MQXC = types::MQXC(4);
    pub const MQXC_MQPUT1: types::MQXC = types::MQXC(5);
    pub const MQXC_MQINQ: types::MQXC = types::MQXC(6);
    pub const MQXC_MQSET: types::MQXC = types::MQXC(8);
    pub const MQXC_MQBACK: types::MQXC = types::MQXC(9);
    pub const MQXC_MQCMIT: types::MQXC = types::MQXC(10);
    pub const MQXC_MQSUB: types::MQXC = types::MQXC(42);
    pub const MQXC_MQSUBRQ: types::MQXC = types::MQXC(43);
    pub const MQXC_MQCB: types::MQXC = types::MQXC(44);
    pub const MQXC_MQCTL: types::MQXC = types::MQXC(45);
    pub const MQXC_MQSTAT: types::MQXC = types::MQXC(46);
    pub const MQXC_CALLBACK: types::MQXC = types::MQXC(48);
    pub const MQXPT_ALL: types::MQXPT = types::MQXPT(-1);
    pub const MQXPT_LOCAL: types::MQXPT = types::MQXPT(0);
    pub const MQXPT_LU62: types::MQXPT = types::MQXPT(1);
    pub const MQXPT_TCP: types::MQXPT = types::MQXPT(2);
    pub const MQXPT_NETBIOS: types::MQXPT = types::MQXPT(3);
    pub const MQXPT_SPX: types::MQXPT = types::MQXPT(4);
    pub const MQXPT_DECNET: types::MQXPT = types::MQXPT(5);
    pub const MQXPT_UDP: types::MQXPT = types::MQXPT(6);
    pub const MQ_CERT_VAL_POLICY_ANY: types::MQ_CERT = types::MQ_CERT(0);
    pub const MQ_CERT_VAL_POLICY_RFC5280: types::MQ_CERT = types::MQ_CERT(1);
    pub const MQ_CERT_VAL_POLICY_NONE: types::MQ_CERT = types::MQ_CERT(2);
    pub const MQ_CERT_VAL_POLICY_DEFAULT: types::MQ_CERT = types::MQ_CERT(0);
    pub const MQ_HTTPSCERTREV_DEFAULT: types::MQ_HTTPSCERTREV = types::MQ_HTTPSCERTREV(
        0,
    );
    pub const MQ_HTTPSCERTREV_REQUIRED: types::MQ_HTTPSCERTREV = types::MQ_HTTPSCERTREV(
        1,
    );
    pub const MQ_HTTPSCERTREV_DISABLED: types::MQ_HTTPSCERTREV = types::MQ_HTTPSCERTREV(
        2,
    );
    pub const MQ_HTTPSCERTREV_OPTIONAL: types::MQ_HTTPSCERTREV = types::MQ_HTTPSCERTREV(
        3,
    );
    pub const MQ_HTTPSCERTVAL_DEFAULT: types::MQ_HTTPSCERTVAL = types::MQ_HTTPSCERTVAL(
        0,
    );
    pub const MQ_HTTPSCERTVAL_ANY: types::MQ_HTTPSCERTVAL = types::MQ_HTTPSCERTVAL(1);
    pub const MQ_HTTPSCERTVAL_NONE: types::MQ_HTTPSCERTVAL = types::MQ_HTTPSCERTVAL(2);
    pub const MQ_HTTPSCERTVAL_HOSTNAMECN: types::MQ_HTTPSCERTVAL = types::MQ_HTTPSCERTVAL(
        3,
    );
    pub const MQ_MQTT_MAX_KEEP_ALIVE: types::MQ_MQTT = types::MQ_MQTT(65536);
    pub const MQ_SUITE_B_NOT_AVAILABLE: types::MQ_SUITE = types::MQ_SUITE(0);
    pub const MQ_SUITE_B_NONE: types::MQ_SUITE = types::MQ_SUITE(1);
    pub const MQ_SUITE_B_128_BIT: types::MQ_SUITE = types::MQ_SUITE(2);
    pub const MQ_SUITE_B_192_BIT: types::MQ_SUITE = types::MQ_SUITE(4);
    pub const MQ_SUITE_B_SIZE: types::MQ_SUITE = types::MQ_SUITE(4);
    #[cfg(feature = "exits")]
    mod exits {
        use crate::types;
        pub const MQCDC_NO_SENDER_CONVERSION: types::MQCDC = types::MQCDC(0);
        pub const MQCDC_SENDER_CONVERSION: types::MQCDC = types::MQCDC(1);
        pub const MQCF_NONE: types::MQCF = types::MQCF(0);
        pub const MQCF_DIST_LISTS: types::MQCF = types::MQCF(1);
        pub const MQCLCT_STATIC: types::MQCLCT = types::MQCLCT(0);
        pub const MQCLCT_DYNAMIC: types::MQCLCT = types::MQCLCT(1);
        pub const MQCOMPRESS_NOT_AVAILABLE: types::MQCOMPRESS = types::MQCOMPRESS(-1);
        pub const MQCOMPRESS_NONE: types::MQCOMPRESS = types::MQCOMPRESS(0);
        pub const MQCOMPRESS_RLE: types::MQCOMPRESS = types::MQCOMPRESS(1);
        pub const MQCOMPRESS_ZLIBFAST: types::MQCOMPRESS = types::MQCOMPRESS(2);
        pub const MQCOMPRESS_ZLIBHIGH: types::MQCOMPRESS = types::MQCOMPRESS(4);
        pub const MQCOMPRESS_SYSTEM: types::MQCOMPRESS = types::MQCOMPRESS(8);
        pub const MQCOMPRESS_LZ4FAST: types::MQCOMPRESS = types::MQCOMPRESS(16);
        pub const MQCOMPRESS_LZ4HIGH: types::MQCOMPRESS = types::MQCOMPRESS(32);
        pub const MQCOMPRESS_ANY: types::MQCOMPRESS = types::MQCOMPRESS(268435455);
        pub const MQDCC_NONE: types::MQDCC = types::MQDCC(0);
        pub const MQDCC_DEFAULT_CONVERSION: types::MQDCC = types::MQDCC(1);
        pub const MQDCC_FILL_TARGET_BUFFER: types::MQDCC = types::MQDCC(2);
        pub const MQDCC_INT_DEFAULT_CONVERSION: types::MQDCC = types::MQDCC(4);
        pub const MQDCC_SOURCE_ENC_NORMAL: types::MQDCC = types::MQDCC(16);
        pub const MQDCC_SOURCE_ENC_REVERSED: types::MQDCC = types::MQDCC(32);
        pub const MQDCC_SOURCE_ENC_MASK: types::MQDCC = types::MQDCC(240);
        pub const MQDCC_TARGET_ENC_NORMAL: types::MQDCC = types::MQDCC(256);
        pub const MQDCC_TARGET_ENC_REVERSED: types::MQDCC = types::MQDCC(512);
        pub const MQDCC_TARGET_ENC_MASK: types::MQDCC = types::MQDCC(3840);
        pub const MQDCC_SOURCE_ENC_UNDEFINED: types::MQDCC = types::MQDCC(0);
        pub const MQDCC_TARGET_ENC_UNDEFINED: types::MQDCC = types::MQDCC(0);
        pub const MQDCC_SOURCE_ENC_FACTOR: types::MQDCC = types::MQDCC(16);
        pub const MQDCC_SOURCE_ENC_NATIVE: types::MQDCC = types::MQDCC(32);
        pub const MQDCC_TARGET_ENC_FACTOR: types::MQDCC = types::MQDCC(256);
        pub const MQDCC_TARGET_ENC_NATIVE: types::MQDCC = types::MQDCC(512);
        pub const MQIEPF_NONE: types::MQIEPF = types::MQIEPF(0);
        pub const MQIEPF_THREADED_LIBRARY: types::MQIEPF = types::MQIEPF(1);
        pub const MQIEPF_LOCAL_LIBRARY: types::MQIEPF = types::MQIEPF(2);
        pub const MQIEPF_CLIENT_LIBRARY: types::MQIEPF = types::MQIEPF(0);
        pub const MQIEPF_NON_THREADED_LIBRARY: types::MQIEPF = types::MQIEPF(0);
        pub const MQMCAT_PROCESS: types::MQMCAT = types::MQMCAT(1);
        pub const MQMCAT_THREAD: types::MQMCAT = types::MQMCAT(2);
        pub const MQMCEV_PACKET_LOSS: types::MQMCEV = types::MQMCEV(1);
        pub const MQMCEV_HEARTBEAT_TIMEOUT: types::MQMCEV = types::MQMCEV(2);
        pub const MQMCEV_VERSION_CONFLICT: types::MQMCEV = types::MQMCEV(3);
        pub const MQMCEV_RELIABILITY: types::MQMCEV = types::MQMCEV(4);
        pub const MQMCEV_CLOSED_TRANS: types::MQMCEV = types::MQMCEV(5);
        pub const MQMCEV_STREAM_ERROR: types::MQMCEV = types::MQMCEV(6);
        pub const MQMCEV_NEW_SOURCE: types::MQMCEV = types::MQMCEV(10);
        pub const MQMCEV_RECEIVE_QUEUE_TRIMMED: types::MQMCEV = types::MQMCEV(11);
        pub const MQMCEV_PACKET_LOSS_NACK_EXPIRE: types::MQMCEV = types::MQMCEV(12);
        pub const MQMCEV_ACK_RETRIES_EXCEEDED: types::MQMCEV = types::MQMCEV(13);
        pub const MQMCEV_STREAM_SUSPEND_NACK: types::MQMCEV = types::MQMCEV(14);
        pub const MQMCEV_STREAM_RESUME_NACK: types::MQMCEV = types::MQMCEV(15);
        pub const MQMCEV_STREAM_EXPELLED: types::MQMCEV = types::MQMCEV(16);
        pub const MQMCEV_FIRST_MESSAGE: types::MQMCEV = types::MQMCEV(20);
        pub const MQMCEV_LATE_JOIN_FAILURE: types::MQMCEV = types::MQMCEV(21);
        pub const MQMCEV_MESSAGE_LOSS: types::MQMCEV = types::MQMCEV(22);
        pub const MQMCEV_SEND_PACKET_FAILURE: types::MQMCEV = types::MQMCEV(23);
        pub const MQMCEV_REPAIR_DELAY: types::MQMCEV = types::MQMCEV(24);
        pub const MQMCEV_MEMORY_ALERT_ON: types::MQMCEV = types::MQMCEV(25);
        pub const MQMCEV_MEMORY_ALERT_OFF: types::MQMCEV = types::MQMCEV(26);
        pub const MQMCEV_NACK_ALERT_ON: types::MQMCEV = types::MQMCEV(27);
        pub const MQMCEV_NACK_ALERT_OFF: types::MQMCEV = types::MQMCEV(28);
        pub const MQMCEV_REPAIR_ALERT_ON: types::MQMCEV = types::MQMCEV(29);
        pub const MQMCEV_REPAIR_ALERT_OFF: types::MQMCEV = types::MQMCEV(30);
        pub const MQMCEV_RELIABILITY_CHANGED: types::MQMCEV = types::MQMCEV(31);
        pub const MQMCEV_SHM_DEST_UNUSABLE: types::MQMCEV = types::MQMCEV(80);
        pub const MQMCEV_SHM_PORT_UNUSABLE: types::MQMCEV = types::MQMCEV(81);
        pub const MQMCEV_CCT_GETTIME_FAILED: types::MQMCEV = types::MQMCEV(110);
        pub const MQMCEV_DEST_INTERFACE_FAILURE: types::MQMCEV = types::MQMCEV(120);
        pub const MQMCEV_DEST_INTERFACE_FAILOVER: types::MQMCEV = types::MQMCEV(121);
        pub const MQMCEV_PORT_INTERFACE_FAILURE: types::MQMCEV = types::MQMCEV(122);
        pub const MQMCEV_PORT_INTERFACE_FAILOVER: types::MQMCEV = types::MQMCEV(123);
        pub const MQNPMS_NORMAL: types::MQNPMS = types::MQNPMS(1);
        pub const MQNPMS_FAST: types::MQNPMS = types::MQNPMS(2);
        pub const MQPA_DEFAULT: types::MQPA = types::MQPA(1);
        pub const MQPA_CONTEXT: types::MQPA = types::MQPA(2);
        pub const MQPA_ONLY_MCA: types::MQPA = types::MQPA(3);
        pub const MQPA_ALTERNATE_OR_MCA: types::MQPA = types::MQPA(4);
        pub const MQPROTO_MQTTV3: types::MQPROTO = types::MQPROTO(1);
        pub const MQPROTO_HTTP: types::MQPROTO = types::MQPROTO(2);
        pub const MQPROTO_AMQP: types::MQPROTO = types::MQPROTO(3);
        pub const MQPROTO_MQTTV311: types::MQPROTO = types::MQPROTO(4);
        pub const MQQF_LOCAL_Q: types::MQQF = types::MQQF(1);
        pub const MQQF_CLWL_USEQ_ANY: types::MQQF = types::MQQF(64);
        pub const MQQF_CLWL_USEQ_LOCAL: types::MQQF = types::MQQF(128);
        pub const MQQMF_REPOSITORY_Q_MGR: types::MQQMF = types::MQQMF(2);
        pub const MQQMF_CLUSSDR_USER_DEFINED: types::MQQMF = types::MQQMF(8);
        pub const MQQMF_CLUSSDR_AUTO_DEFINED: types::MQQMF = types::MQQMF(16);
        pub const MQQMF_AVAILABLE: types::MQQMF = types::MQQMF(32);
        pub const MQSCA_REQUIRED: types::MQSCA = types::MQSCA(0);
        pub const MQSCA_OPTIONAL: types::MQSCA = types::MQSCA(1);
        pub const MQSCA_NEVER_REQUIRED: types::MQSCA = types::MQSCA(2);
        pub const MQSECPROT_NONE: types::MQSECPROT = types::MQSECPROT(0);
        pub const MQSECPROT_SSLV30: types::MQSECPROT = types::MQSECPROT(1);
        pub const MQSECPROT_TLSV10: types::MQSECPROT = types::MQSECPROT(2);
        pub const MQSECPROT_TLSV12: types::MQSECPROT = types::MQSECPROT(4);
        pub const MQSECPROT_TLSV13: types::MQSECPROT = types::MQSECPROT(8);
        pub const MQSPL_PASSTHRU: types::MQSPL = types::MQSPL(0);
        pub const MQSPL_REMOVE: types::MQSPL = types::MQSPL(1);
        pub const MQSPL_AS_POLICY: types::MQSPL = types::MQSPL(2);
        pub const MQWXP_PUT_BY_CLUSTER_CHL: types::MQWXP = types::MQWXP(2);
        pub const MQXACT_EXTERNAL: types::MQXACT = types::MQXACT(1);
        pub const MQXACT_INTERNAL: types::MQXACT = types::MQXACT(2);
        pub const MQXCC_FAILED: types::MQXCC = types::MQXCC(-8);
        pub const MQXCC_REQUEST_ACK: types::MQXCC = types::MQXCC(-7);
        pub const MQXCC_CLOSE_CHANNEL: types::MQXCC = types::MQXCC(-6);
        pub const MQXCC_SUPPRESS_EXIT: types::MQXCC = types::MQXCC(-5);
        pub const MQXCC_SEND_SEC_MSG: types::MQXCC = types::MQXCC(-4);
        pub const MQXCC_SEND_AND_REQUEST_SEC_MSG: types::MQXCC = types::MQXCC(-3);
        pub const MQXCC_SKIP_FUNCTION: types::MQXCC = types::MQXCC(-2);
        pub const MQXCC_SUPPRESS_FUNCTION: types::MQXCC = types::MQXCC(-1);
        pub const MQXCC_OK: types::MQXCC = types::MQXCC(0);
        pub const MQXDR_OK: types::MQXDR = types::MQXDR(0);
        pub const MQXDR_CONVERSION_FAILED: types::MQXDR = types::MQXDR(1);
        pub const MQXEPO_NONE: types::MQXEPO = types::MQXEPO(0);
        pub const MQXE_OTHER: types::MQXE = types::MQXE(0);
        pub const MQXE_MCA: types::MQXE = types::MQXE(1);
        pub const MQXE_MCA_SVRCONN: types::MQXE = types::MQXE(2);
        pub const MQXE_COMMAND_SERVER: types::MQXE = types::MQXE(3);
        pub const MQXE_MQSC: types::MQXE = types::MQXE(4);
        pub const MQXE_MCA_CLNTCONN: types::MQXE = types::MQXE(5);
        pub const MQXF_INIT: types::MQXF = types::MQXF(1);
        pub const MQXF_TERM: types::MQXF = types::MQXF(2);
        pub const MQXF_CONN: types::MQXF = types::MQXF(3);
        pub const MQXF_CONNX: types::MQXF = types::MQXF(4);
        pub const MQXF_DISC: types::MQXF = types::MQXF(5);
        pub const MQXF_OPEN: types::MQXF = types::MQXF(6);
        pub const MQXF_CLOSE: types::MQXF = types::MQXF(7);
        pub const MQXF_PUT1: types::MQXF = types::MQXF(8);
        pub const MQXF_PUT: types::MQXF = types::MQXF(9);
        pub const MQXF_GET: types::MQXF = types::MQXF(10);
        pub const MQXF_DATA_CONV_ON_GET: types::MQXF = types::MQXF(11);
        pub const MQXF_INQ: types::MQXF = types::MQXF(12);
        pub const MQXF_SET: types::MQXF = types::MQXF(13);
        pub const MQXF_BEGIN: types::MQXF = types::MQXF(14);
        pub const MQXF_CMIT: types::MQXF = types::MQXF(15);
        pub const MQXF_BACK: types::MQXF = types::MQXF(16);
        pub const MQXF_STAT: types::MQXF = types::MQXF(18);
        pub const MQXF_CB: types::MQXF = types::MQXF(19);
        pub const MQXF_CTL: types::MQXF = types::MQXF(20);
        pub const MQXF_CALLBACK: types::MQXF = types::MQXF(21);
        pub const MQXF_SUB: types::MQXF = types::MQXF(22);
        pub const MQXF_SUBRQ: types::MQXF = types::MQXF(23);
        pub const MQXF_XACLOSE: types::MQXF = types::MQXF(24);
        pub const MQXF_XACOMMIT: types::MQXF = types::MQXF(25);
        pub const MQXF_XACOMPLETE: types::MQXF = types::MQXF(26);
        pub const MQXF_XAEND: types::MQXF = types::MQXF(27);
        pub const MQXF_XAFORGET: types::MQXF = types::MQXF(28);
        pub const MQXF_XAOPEN: types::MQXF = types::MQXF(29);
        pub const MQXF_XAPREPARE: types::MQXF = types::MQXF(30);
        pub const MQXF_XARECOVER: types::MQXF = types::MQXF(31);
        pub const MQXF_XAROLLBACK: types::MQXF = types::MQXF(32);
        pub const MQXF_XASTART: types::MQXF = types::MQXF(33);
        pub const MQXF_AXREG: types::MQXF = types::MQXF(34);
        pub const MQXF_AXUNREG: types::MQXF = types::MQXF(35);
        pub const MQXR2_DEFAULT_CONTINUATION: types::MQXR2 = types::MQXR2(0);
        pub const MQXR2_PUT_WITH_DEF_USERID: types::MQXR2 = types::MQXR2(1);
        pub const MQXR2_PUT_WITH_MSG_USERID: types::MQXR2 = types::MQXR2(2);
        pub const MQXR2_USE_EXIT_BUFFER: types::MQXR2 = types::MQXR2(4);
        pub const MQXR2_CONTINUE_CHAIN: types::MQXR2 = types::MQXR2(8);
        pub const MQXR2_SUPPRESS_CHAIN: types::MQXR2 = types::MQXR2(16);
        pub const MQXR2_DYNAMIC_CACHE: types::MQXR2 = types::MQXR2(32);
        pub const MQXR2_PUT_WITH_DEF_ACTION: types::MQXR2 = types::MQXR2(0);
        pub const MQXR2_STATIC_CACHE: types::MQXR2 = types::MQXR2(0);
        pub const MQXR2_USE_AGENT_BUFFER: types::MQXR2 = types::MQXR2(0);
        pub const MQXR_BEFORE: types::MQXR = types::MQXR(1);
        pub const MQXR_AFTER: types::MQXR = types::MQXR(2);
        pub const MQXR_CONNECTION: types::MQXR = types::MQXR(3);
        pub const MQXR_BEFORE_CONVERT: types::MQXR = types::MQXR(4);
        pub const MQXR_INIT: types::MQXR = types::MQXR(11);
        pub const MQXR_TERM: types::MQXR = types::MQXR(12);
        pub const MQXR_MSG: types::MQXR = types::MQXR(13);
        pub const MQXR_XMIT: types::MQXR = types::MQXR(14);
        pub const MQXR_SEC_MSG: types::MQXR = types::MQXR(15);
        pub const MQXR_INIT_SEC: types::MQXR = types::MQXR(16);
        pub const MQXR_RETRY: types::MQXR = types::MQXR(17);
        pub const MQXR_AUTO_CLUSSDR: types::MQXR = types::MQXR(18);
        pub const MQXR_AUTO_RECEIVER: types::MQXR = types::MQXR(19);
        pub const MQXR_CLWL_OPEN: types::MQXR = types::MQXR(20);
        pub const MQXR_CLWL_PUT: types::MQXR = types::MQXR(21);
        pub const MQXR_CLWL_MOVE: types::MQXR = types::MQXR(22);
        pub const MQXR_CLWL_REPOS: types::MQXR = types::MQXR(23);
        pub const MQXR_CLWL_REPOS_MOVE: types::MQXR = types::MQXR(24);
        pub const MQXR_END_BATCH: types::MQXR = types::MQXR(25);
        pub const MQXR_ACK_RECEIVED: types::MQXR = types::MQXR(26);
        pub const MQXR_AUTO_SVRCONN: types::MQXR = types::MQXR(27);
        pub const MQXR_AUTO_CLUSRCVR: types::MQXR = types::MQXR(28);
        pub const MQXR_SEC_PARMS: types::MQXR = types::MQXR(29);
        pub const MQXR_PUBLICATION: types::MQXR = types::MQXR(30);
        pub const MQXR_PRECONNECT: types::MQXR = types::MQXR(31);
        pub const MQXT_API_CROSSING_EXIT: types::MQXT = types::MQXT(1);
        pub const MQXT_API_EXIT: types::MQXT = types::MQXT(2);
        pub const MQXT_CHANNEL_SEC_EXIT: types::MQXT = types::MQXT(11);
        pub const MQXT_CHANNEL_MSG_EXIT: types::MQXT = types::MQXT(12);
        pub const MQXT_CHANNEL_SEND_EXIT: types::MQXT = types::MQXT(13);
        pub const MQXT_CHANNEL_RCV_EXIT: types::MQXT = types::MQXT(14);
        pub const MQXT_CHANNEL_MSG_RETRY_EXIT: types::MQXT = types::MQXT(15);
        pub const MQXT_CHANNEL_AUTO_DEF_EXIT: types::MQXT = types::MQXT(16);
        pub const MQXT_CLUSTER_WORKLOAD_EXIT: types::MQXT = types::MQXT(20);
        pub const MQXT_PUBSUB_ROUTING_EXIT: types::MQXT = types::MQXT(21);
        pub const MQXT_PUBLISH_EXIT: types::MQXT = types::MQXT(22);
        pub const MQXT_PRECONNECT_EXIT: types::MQXT = types::MQXT(23);
        pub const MQZAET_NONE: types::MQZAET = types::MQZAET(0);
        pub const MQZAET_PRINCIPAL: types::MQZAET = types::MQZAET(1);
        pub const MQZAET_GROUP: types::MQZAET = types::MQZAET(2);
        pub const MQZAET_UNKNOWN: types::MQZAET = types::MQZAET(3);
        pub const MQZAO_NONE: types::MQZAO = types::MQZAO(0);
        pub const MQZAO_CONNECT: types::MQZAO = types::MQZAO(1);
        pub const MQZAO_BROWSE: types::MQZAO = types::MQZAO(2);
        pub const MQZAO_INPUT: types::MQZAO = types::MQZAO(4);
        pub const MQZAO_OUTPUT: types::MQZAO = types::MQZAO(8);
        pub const MQZAO_INQUIRE: types::MQZAO = types::MQZAO(16);
        pub const MQZAO_SET: types::MQZAO = types::MQZAO(32);
        pub const MQZAO_PASS_IDENTITY_CONTEXT: types::MQZAO = types::MQZAO(64);
        pub const MQZAO_PASS_ALL_CONTEXT: types::MQZAO = types::MQZAO(128);
        pub const MQZAO_SET_IDENTITY_CONTEXT: types::MQZAO = types::MQZAO(256);
        pub const MQZAO_SET_ALL_CONTEXT: types::MQZAO = types::MQZAO(512);
        pub const MQZAO_ALTERNATE_USER_AUTHORITY: types::MQZAO = types::MQZAO(1024);
        pub const MQZAO_PUBLISH: types::MQZAO = types::MQZAO(2048);
        pub const MQZAO_SUBSCRIBE: types::MQZAO = types::MQZAO(4096);
        pub const MQZAO_RESUME: types::MQZAO = types::MQZAO(8192);
        pub const MQZAO_ALL_MQI: types::MQZAO = types::MQZAO(16383);
        pub const MQZAO_CREATE: types::MQZAO = types::MQZAO(65536);
        pub const MQZAO_DELETE: types::MQZAO = types::MQZAO(131072);
        pub const MQZAO_DISPLAY: types::MQZAO = types::MQZAO(262144);
        pub const MQZAO_CHANGE: types::MQZAO = types::MQZAO(524288);
        pub const MQZAO_CLEAR: types::MQZAO = types::MQZAO(1048576);
        pub const MQZAO_CONTROL: types::MQZAO = types::MQZAO(2097152);
        pub const MQZAO_CONTROL_EXTENDED: types::MQZAO = types::MQZAO(4194304);
        pub const MQZAO_AUTHORIZE: types::MQZAO = types::MQZAO(8388608);
        pub const MQZAO_ALL_ADMIN: types::MQZAO = types::MQZAO(16646144);
        pub const MQZAO_REMOVE: types::MQZAO = types::MQZAO(16777216);
        pub const MQZAO_SYSTEM: types::MQZAO = types::MQZAO(33554432);
        pub const MQZAO_ALL: types::MQZAO = types::MQZAO(50216959);
        pub const MQZAO_CREATE_ONLY: types::MQZAO = types::MQZAO(67108864);
        pub const MQZAT_INITIAL_CONTEXT: types::MQZAT = types::MQZAT(0);
        pub const MQZAT_CHANGE_CONTEXT: types::MQZAT = types::MQZAT(1);
        pub const MQZCI_CONTINUE: types::MQZCI = types::MQZCI(0);
        pub const MQZCI_STOP: types::MQZCI = types::MQZCI(1);
        pub const MQZCI_DEFAULT: types::MQZCI = types::MQZCI(0);
        pub const MQZID_INIT: types::MQZID = types::MQZID(0);
        pub const MQZID_TERM: types::MQZID = types::MQZID(1);
        pub const MQZID_INIT_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
            0,
        );
        pub const MQZID_TERM_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
            1,
        );
        pub const MQZID_CHECK_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
            2,
        );
        pub const MQZID_COPY_ALL_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
            3,
        );
        pub const MQZID_DELETE_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
            4,
        );
        pub const MQZID_SET_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
            5,
        );
        pub const MQZID_GET_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
            6,
        );
        pub const MQZID_GET_EXPLICIT_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
            7,
        );
        pub const MQZID_REFRESH_CACHE: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
            8,
        );
        pub const MQZID_ENUMERATE_AUTHORITY_DATA: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
            9,
        );
        pub const MQZID_AUTHENTICATE_USER: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
            10,
        );
        pub const MQZID_FREE_USER: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(11);
        pub const MQZID_INQUIRE: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(12);
        pub const MQZID_CHECK_PRIVILEGED: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
            13,
        );
        pub const MQZID_INIT_NAME: types::MQZID_NAME = types::MQZID_NAME(0);
        pub const MQZID_TERM_NAME: types::MQZID_NAME = types::MQZID_NAME(1);
        pub const MQZID_LOOKUP_NAME: types::MQZID_NAME = types::MQZID_NAME(2);
        pub const MQZID_INSERT_NAME: types::MQZID_NAME = types::MQZID_NAME(3);
        pub const MQZID_DELETE_NAME: types::MQZID_NAME = types::MQZID_NAME(4);
        pub const MQZID_INIT_USERID: types::MQZID_USERID = types::MQZID_USERID(0);
        pub const MQZID_TERM_USERID: types::MQZID_USERID = types::MQZID_USERID(1);
        pub const MQZID_FIND_USERID: types::MQZID_USERID = types::MQZID_USERID(2);
        pub const MQZIO_PRIMARY: types::MQZIO = types::MQZIO(0);
        pub const MQZIO_SECONDARY: types::MQZIO = types::MQZIO(1);
        pub const MQZSE_CONTINUE: types::MQZSE = types::MQZSE(0);
        pub const MQZSE_START: types::MQZSE = types::MQZSE(1);
        pub const MQZSL_NOT_RETURNED: types::MQZSL = types::MQZSL(0);
        pub const MQZSL_RETURNED: types::MQZSL = types::MQZSL(1);
        pub const MQZTO_PRIMARY: types::MQZTO = types::MQZTO(0);
        pub const MQZTO_SECONDARY: types::MQZTO = types::MQZTO(1);
    }
    #[cfg(feature = "exits")]
    pub use exits::*;
    #[cfg(feature = "mqai")]
    mod mqai {
        use crate::types;
        pub const MQBL_NULL_TERMINATED: types::MQBL = types::MQBL(-1);
        pub const MQCBO_NONE: types::MQCBO = types::MQCBO(0);
        pub const MQCBO_ADMIN_BAG: types::MQCBO = types::MQCBO(1);
        pub const MQCBO_LIST_FORM_ALLOWED: types::MQCBO = types::MQCBO(2);
        pub const MQCBO_REORDER_AS_REQUIRED: types::MQCBO = types::MQCBO(4);
        pub const MQCBO_CHECK_SELECTORS: types::MQCBO = types::MQCBO(8);
        pub const MQCBO_COMMAND_BAG: types::MQCBO = types::MQCBO(16);
        pub const MQCBO_SYSTEM_BAG: types::MQCBO = types::MQCBO(32);
        pub const MQCBO_GROUP_BAG: types::MQCBO = types::MQCBO(64);
        pub const MQCBO_DO_NOT_CHECK_SELECTORS: types::MQCBO = types::MQCBO(0);
        pub const MQCBO_DO_NOT_REORDER: types::MQCBO = types::MQCBO(0);
        pub const MQCBO_LIST_FORM_INHIBITED: types::MQCBO = types::MQCBO(0);
        pub const MQCBO_USER_BAG: types::MQCBO = types::MQCBO(0);
        pub const MQHA_BAG_HANDLE: types::MQHA = types::MQHA(4001);
        pub const MQHA_FIRST: types::MQHA = types::MQHA(4001);
        pub const MQHA_LAST_USED: types::MQHA = types::MQHA(4001);
        pub const MQHA_LAST: types::MQHA = types::MQHA(6000);
        pub const MQHB_NONE: types::MQHB = types::MQHB(-2);
        pub const MQHB_UNUSABLE_HBAG: types::MQHB = types::MQHB(-1);
        pub const MQIASY_VERSION: types::MQIASY = types::MQIASY(-9);
        pub const MQIASY_BAG_OPTIONS: types::MQIASY = types::MQIASY(-8);
        pub const MQIASY_REASON: types::MQIASY = types::MQIASY(-7);
        pub const MQIASY_COMP_CODE: types::MQIASY = types::MQIASY(-6);
        pub const MQIASY_CONTROL: types::MQIASY = types::MQIASY(-5);
        pub const MQIASY_MSG_SEQ_NUMBER: types::MQIASY = types::MQIASY(-4);
        pub const MQIASY_COMMAND: types::MQIASY = types::MQIASY(-3);
        pub const MQIASY_TYPE: types::MQIASY = types::MQIASY(-2);
        pub const MQIASY_CODED_CHAR_SET_ID: types::MQIASY = types::MQIASY(-1);
        pub const MQIASY_LAST: types::MQIASY = types::MQIASY(-2000);
        pub const MQIASY_LAST_USED: types::MQIASY = types::MQIASY(-9);
        pub const MQIASY_FIRST: types::MQIASY = types::MQIASY(-1);
        pub const MQIND_ALL: types::MQIND = types::MQIND(-2);
        pub const MQIND_NONE: types::MQIND = types::MQIND(-1);
        pub const MQITEM_INTEGER: types::MQITEM = types::MQITEM(1);
        pub const MQITEM_STRING: types::MQITEM = types::MQITEM(2);
        pub const MQITEM_BAG: types::MQITEM = types::MQITEM(3);
        pub const MQITEM_BYTE_STRING: types::MQITEM = types::MQITEM(4);
        pub const MQITEM_INTEGER_FILTER: types::MQITEM = types::MQITEM(5);
        pub const MQITEM_STRING_FILTER: types::MQITEM = types::MQITEM(6);
        pub const MQITEM_INTEGER64: types::MQITEM = types::MQITEM(7);
        pub const MQITEM_BYTE_STRING_FILTER: types::MQITEM = types::MQITEM(8);
        pub const MQSEL_ALL_SYSTEM_SELECTORS: types::MQSEL_ALL = types::MQSEL_ALL(
            -30003,
        );
        pub const MQSEL_ALL_USER_SELECTORS: types::MQSEL_ALL = types::MQSEL_ALL(-30002);
        pub const MQSEL_ALL_SELECTORS: types::MQSEL_ALL = types::MQSEL_ALL(-30001);
        pub const MQSEL_ANY_SYSTEM_SELECTOR: types::MQSEL_ANY = types::MQSEL_ANY(-30003);
        pub const MQSEL_ANY_USER_SELECTOR: types::MQSEL_ANY = types::MQSEL_ANY(-30002);
        pub const MQSEL_ANY_SELECTOR: types::MQSEL_ANY = types::MQSEL_ANY(-30001);
    }
    #[cfg(feature = "mqai")]
    pub use mqai::*;
    #[cfg(feature = "pcf")]
    mod pcf {
        use crate::types;
        pub const MQACTIVE_NO: types::MQACTIVE = types::MQACTIVE(0);
        pub const MQACTIVE_YES: types::MQACTIVE = types::MQACTIVE(1);
        pub const MQACT_FORCE_REMOVE: types::MQACT = types::MQACT(1);
        pub const MQACT_ADVANCE_LOG: types::MQACT = types::MQACT(2);
        pub const MQACT_COLLECT_STATISTICS: types::MQACT = types::MQACT(3);
        pub const MQACT_PUBSUB: types::MQACT = types::MQACT(4);
        pub const MQACT_ADD: types::MQACT = types::MQACT(5);
        pub const MQACT_REPLACE: types::MQACT = types::MQACT(6);
        pub const MQACT_REMOVE: types::MQACT = types::MQACT(7);
        pub const MQACT_REMOVEALL: types::MQACT = types::MQACT(8);
        pub const MQACT_FAIL: types::MQACT = types::MQACT(9);
        pub const MQACT_REDUCE_LOG: types::MQACT = types::MQACT(10);
        pub const MQACT_ARCHIVE_LOG: types::MQACT = types::MQACT(11);
        pub const MQADPCTX_NO: types::MQADPCTX = types::MQADPCTX(0);
        pub const MQADPCTX_YES: types::MQADPCTX = types::MQADPCTX(1);
        pub const MQAPPL_IMMOVABLE: types::MQAPPL = types::MQAPPL(0);
        pub const MQAPPL_MOVABLE: types::MQAPPL = types::MQAPPL(1);
        pub const MQAS_NONE: types::MQAS = types::MQAS(0);
        pub const MQAS_STARTED: types::MQAS = types::MQAS(1);
        pub const MQAS_START_WAIT: types::MQAS = types::MQAS(2);
        pub const MQAS_STOPPED: types::MQAS = types::MQAS(3);
        pub const MQAS_SUSPENDED: types::MQAS = types::MQAS(4);
        pub const MQAS_SUSPENDED_TEMPORARY: types::MQAS = types::MQAS(5);
        pub const MQAS_ACTIVE: types::MQAS = types::MQAS(6);
        pub const MQAS_INACTIVE: types::MQAS = types::MQAS(7);
        pub const MQAUTHENTICATE_OS: types::MQAUTHENTICATE = types::MQAUTHENTICATE(0);
        pub const MQAUTHENTICATE_PAM: types::MQAUTHENTICATE = types::MQAUTHENTICATE(1);
        pub const MQAUTHOPT_ENTITY_EXPLICIT: types::MQAUTHOPT = types::MQAUTHOPT(1);
        pub const MQAUTHOPT_ENTITY_SET: types::MQAUTHOPT = types::MQAUTHOPT(2);
        pub const MQAUTHOPT_NAME_EXPLICIT: types::MQAUTHOPT = types::MQAUTHOPT(16);
        pub const MQAUTHOPT_NAME_ALL_MATCHING: types::MQAUTHOPT = types::MQAUTHOPT(32);
        pub const MQAUTHOPT_NAME_AS_WILDCARD: types::MQAUTHOPT = types::MQAUTHOPT(64);
        pub const MQAUTHOPT_CUMULATIVE: types::MQAUTHOPT = types::MQAUTHOPT(256);
        pub const MQAUTHOPT_EXCLUDE_TEMP: types::MQAUTHOPT = types::MQAUTHOPT(512);
        pub const MQAUTH_ALL_MQI: types::MQAUTH = types::MQAUTH(-3);
        pub const MQAUTH_ALL_ADMIN: types::MQAUTH = types::MQAUTH(-2);
        pub const MQAUTH_ALL: types::MQAUTH = types::MQAUTH(-1);
        pub const MQAUTH_NONE: types::MQAUTH = types::MQAUTH(0);
        pub const MQAUTH_ALT_USER_AUTHORITY: types::MQAUTH = types::MQAUTH(1);
        pub const MQAUTH_BROWSE: types::MQAUTH = types::MQAUTH(2);
        pub const MQAUTH_CHANGE: types::MQAUTH = types::MQAUTH(3);
        pub const MQAUTH_CLEAR: types::MQAUTH = types::MQAUTH(4);
        pub const MQAUTH_CONNECT: types::MQAUTH = types::MQAUTH(5);
        pub const MQAUTH_CREATE: types::MQAUTH = types::MQAUTH(6);
        pub const MQAUTH_DELETE: types::MQAUTH = types::MQAUTH(7);
        pub const MQAUTH_DISPLAY: types::MQAUTH = types::MQAUTH(8);
        pub const MQAUTH_INPUT: types::MQAUTH = types::MQAUTH(9);
        pub const MQAUTH_INQUIRE: types::MQAUTH = types::MQAUTH(10);
        pub const MQAUTH_OUTPUT: types::MQAUTH = types::MQAUTH(11);
        pub const MQAUTH_PASS_ALL_CONTEXT: types::MQAUTH = types::MQAUTH(12);
        pub const MQAUTH_PASS_IDENTITY_CONTEXT: types::MQAUTH = types::MQAUTH(13);
        pub const MQAUTH_SET: types::MQAUTH = types::MQAUTH(14);
        pub const MQAUTH_SET_ALL_CONTEXT: types::MQAUTH = types::MQAUTH(15);
        pub const MQAUTH_SET_IDENTITY_CONTEXT: types::MQAUTH = types::MQAUTH(16);
        pub const MQAUTH_CONTROL: types::MQAUTH = types::MQAUTH(17);
        pub const MQAUTH_CONTROL_EXTENDED: types::MQAUTH = types::MQAUTH(18);
        pub const MQAUTH_PUBLISH: types::MQAUTH = types::MQAUTH(19);
        pub const MQAUTH_SUBSCRIBE: types::MQAUTH = types::MQAUTH(20);
        pub const MQAUTH_RESUME: types::MQAUTH = types::MQAUTH(21);
        pub const MQAUTH_SYSTEM: types::MQAUTH = types::MQAUTH(22);
        pub const MQAUTOCLUS_TYPE_NONE: types::MQAUTOCLUS = types::MQAUTOCLUS(0);
        pub const MQAUTOCLUS_TYPE_UNIFORM: types::MQAUTOCLUS = types::MQAUTOCLUS(1);
        pub const MQBACF_EVENT_ACCOUNTING_TOKEN: types::MQBACF = types::MQBACF(7001);
        pub const MQBACF_EVENT_SECURITY_ID: types::MQBACF = types::MQBACF(7002);
        pub const MQBACF_RESPONSE_SET: types::MQBACF = types::MQBACF(7003);
        pub const MQBACF_RESPONSE_ID: types::MQBACF = types::MQBACF(7004);
        pub const MQBACF_EXTERNAL_UOW_ID: types::MQBACF = types::MQBACF(7005);
        pub const MQBACF_CONNECTION_ID: types::MQBACF = types::MQBACF(7006);
        pub const MQBACF_GENERIC_CONNECTION_ID: types::MQBACF = types::MQBACF(7007);
        pub const MQBACF_ORIGIN_UOW_ID: types::MQBACF = types::MQBACF(7008);
        pub const MQBACF_Q_MGR_UOW_ID: types::MQBACF = types::MQBACF(7009);
        pub const MQBACF_ACCOUNTING_TOKEN: types::MQBACF = types::MQBACF(7010);
        pub const MQBACF_CORREL_ID: types::MQBACF = types::MQBACF(7011);
        pub const MQBACF_GROUP_ID: types::MQBACF = types::MQBACF(7012);
        pub const MQBACF_MSG_ID: types::MQBACF = types::MQBACF(7013);
        pub const MQBACF_CF_LEID: types::MQBACF = types::MQBACF(7014);
        pub const MQBACF_DESTINATION_CORREL_ID: types::MQBACF = types::MQBACF(7015);
        pub const MQBACF_SUB_ID: types::MQBACF = types::MQBACF(7016);
        pub const MQBACF_ALTERNATE_SECURITYID: types::MQBACF = types::MQBACF(7019);
        pub const MQBACF_MESSAGE_DATA: types::MQBACF = types::MQBACF(7020);
        pub const MQBACF_MQBO_STRUCT: types::MQBACF = types::MQBACF(7021);
        pub const MQBACF_MQCB_FUNCTION: types::MQBACF = types::MQBACF(7022);
        pub const MQBACF_MQCBC_STRUCT: types::MQBACF = types::MQBACF(7023);
        pub const MQBACF_MQCBD_STRUCT: types::MQBACF = types::MQBACF(7024);
        pub const MQBACF_MQCD_STRUCT: types::MQBACF = types::MQBACF(7025);
        pub const MQBACF_MQCNO_STRUCT: types::MQBACF = types::MQBACF(7026);
        pub const MQBACF_MQGMO_STRUCT: types::MQBACF = types::MQBACF(7027);
        pub const MQBACF_MQMD_STRUCT: types::MQBACF = types::MQBACF(7028);
        pub const MQBACF_MQPMO_STRUCT: types::MQBACF = types::MQBACF(7029);
        pub const MQBACF_MQSD_STRUCT: types::MQBACF = types::MQBACF(7030);
        pub const MQBACF_MQSTS_STRUCT: types::MQBACF = types::MQBACF(7031);
        pub const MQBACF_SUB_CORREL_ID: types::MQBACF = types::MQBACF(7032);
        pub const MQBACF_XA_XID: types::MQBACF = types::MQBACF(7033);
        pub const MQBACF_XQH_CORREL_ID: types::MQBACF = types::MQBACF(7034);
        pub const MQBACF_XQH_MSG_ID: types::MQBACF = types::MQBACF(7035);
        pub const MQBACF_REQUEST_ID: types::MQBACF = types::MQBACF(7036);
        pub const MQBACF_PROPERTIES_DATA: types::MQBACF = types::MQBACF(7037);
        pub const MQBACF_CONN_TAG: types::MQBACF = types::MQBACF(7038);
        pub const MQBACF_MQBNO_STRUCT: types::MQBACF = types::MQBACF(7039);
        pub const MQBACF_FIRST: types::MQBACF = types::MQBACF(7001);
        pub const MQBACF_LAST_USED: types::MQBACF = types::MQBACF(7039);
        pub const MQBALANCED_NO: types::MQBALANCED = types::MQBALANCED(0);
        pub const MQBALANCED_YES: types::MQBALANCED = types::MQBALANCED(1);
        pub const MQBALANCED_NOT_APPLICABLE: types::MQBALANCED = types::MQBALANCED(2);
        pub const MQBALANCED_UNKNOWN: types::MQBALANCED = types::MQBALANCED(3);
        pub const MQBALSTATE_NOT_APPLICABLE: types::MQBALSTATE = types::MQBALSTATE(0);
        pub const MQBALSTATE_LOW: types::MQBALSTATE = types::MQBALSTATE(1);
        pub const MQBALSTATE_OK: types::MQBALSTATE = types::MQBALSTATE(2);
        pub const MQBALSTATE_HIGH: types::MQBALSTATE = types::MQBALSTATE(3);
        pub const MQBALSTATE_UNKNOWN: types::MQBALSTATE = types::MQBALSTATE(4);
        pub const MQBPLOCATION_BELOW: types::MQBPLOCATION = types::MQBPLOCATION(0);
        pub const MQBPLOCATION_ABOVE: types::MQBPLOCATION = types::MQBPLOCATION(1);
        pub const MQBPLOCATION_SWITCHING_ABOVE: types::MQBPLOCATION = types::MQBPLOCATION(
            2,
        );
        pub const MQBPLOCATION_SWITCHING_BELOW: types::MQBPLOCATION = types::MQBPLOCATION(
            3,
        );
        pub const MQBT_OTMA: types::MQBT = types::MQBT(1);
        pub const MQCACF_FROM_Q_NAME: types::MQCACF = types::MQCACF(3001);
        pub const MQCACF_TO_Q_NAME: types::MQCACF = types::MQCACF(3002);
        pub const MQCACF_FROM_PROCESS_NAME: types::MQCACF = types::MQCACF(3003);
        pub const MQCACF_TO_PROCESS_NAME: types::MQCACF = types::MQCACF(3004);
        pub const MQCACF_FROM_NAMELIST_NAME: types::MQCACF = types::MQCACF(3005);
        pub const MQCACF_TO_NAMELIST_NAME: types::MQCACF = types::MQCACF(3006);
        pub const MQCACF_FROM_CHANNEL_NAME: types::MQCACF = types::MQCACF(3007);
        pub const MQCACF_TO_CHANNEL_NAME: types::MQCACF = types::MQCACF(3008);
        pub const MQCACF_FROM_AUTH_INFO_NAME: types::MQCACF = types::MQCACF(3009);
        pub const MQCACF_TO_AUTH_INFO_NAME: types::MQCACF = types::MQCACF(3010);
        pub const MQCACF_Q_NAMES: types::MQCACF = types::MQCACF(3011);
        pub const MQCACF_PROCESS_NAMES: types::MQCACF = types::MQCACF(3012);
        pub const MQCACF_NAMELIST_NAMES: types::MQCACF = types::MQCACF(3013);
        pub const MQCACF_ESCAPE_TEXT: types::MQCACF = types::MQCACF(3014);
        pub const MQCACF_LOCAL_Q_NAMES: types::MQCACF = types::MQCACF(3015);
        pub const MQCACF_MODEL_Q_NAMES: types::MQCACF = types::MQCACF(3016);
        pub const MQCACF_ALIAS_Q_NAMES: types::MQCACF = types::MQCACF(3017);
        pub const MQCACF_REMOTE_Q_NAMES: types::MQCACF = types::MQCACF(3018);
        pub const MQCACF_SENDER_CHANNEL_NAMES: types::MQCACF = types::MQCACF(3019);
        pub const MQCACF_SERVER_CHANNEL_NAMES: types::MQCACF = types::MQCACF(3020);
        pub const MQCACF_REQUESTER_CHANNEL_NAMES: types::MQCACF = types::MQCACF(3021);
        pub const MQCACF_RECEIVER_CHANNEL_NAMES: types::MQCACF = types::MQCACF(3022);
        pub const MQCACF_OBJECT_Q_MGR_NAME: types::MQCACF = types::MQCACF(3023);
        pub const MQCACF_APPL_NAME: types::MQCACF = types::MQCACF(3024);
        pub const MQCACF_USER_IDENTIFIER: types::MQCACF = types::MQCACF(3025);
        pub const MQCACF_AUX_ERROR_DATA_STR_1: types::MQCACF = types::MQCACF(3026);
        pub const MQCACF_AUX_ERROR_DATA_STR_2: types::MQCACF = types::MQCACF(3027);
        pub const MQCACF_AUX_ERROR_DATA_STR_3: types::MQCACF = types::MQCACF(3028);
        pub const MQCACF_BRIDGE_NAME: types::MQCACF = types::MQCACF(3029);
        pub const MQCACF_STREAM_NAME: types::MQCACF = types::MQCACF(3030);
        pub const MQCACF_TOPIC: types::MQCACF = types::MQCACF(3031);
        pub const MQCACF_PARENT_Q_MGR_NAME: types::MQCACF = types::MQCACF(3032);
        pub const MQCACF_CORREL_ID: types::MQCACF = types::MQCACF(3033);
        pub const MQCACF_PUBLISH_TIMESTAMP: types::MQCACF = types::MQCACF(3034);
        pub const MQCACF_STRING_DATA: types::MQCACF = types::MQCACF(3035);
        pub const MQCACF_SUPPORTED_STREAM_NAME: types::MQCACF = types::MQCACF(3036);
        pub const MQCACF_REG_TOPIC: types::MQCACF = types::MQCACF(3037);
        pub const MQCACF_REG_TIME: types::MQCACF = types::MQCACF(3038);
        pub const MQCACF_REG_USER_ID: types::MQCACF = types::MQCACF(3039);
        pub const MQCACF_CHILD_Q_MGR_NAME: types::MQCACF = types::MQCACF(3040);
        pub const MQCACF_REG_STREAM_NAME: types::MQCACF = types::MQCACF(3041);
        pub const MQCACF_REG_Q_MGR_NAME: types::MQCACF = types::MQCACF(3042);
        pub const MQCACF_REG_Q_NAME: types::MQCACF = types::MQCACF(3043);
        pub const MQCACF_REG_CORREL_ID: types::MQCACF = types::MQCACF(3044);
        pub const MQCACF_EVENT_USER_ID: types::MQCACF = types::MQCACF(3045);
        pub const MQCACF_OBJECT_NAME: types::MQCACF = types::MQCACF(3046);
        pub const MQCACF_EVENT_Q_MGR: types::MQCACF = types::MQCACF(3047);
        pub const MQCACF_AUTH_INFO_NAMES: types::MQCACF = types::MQCACF(3048);
        pub const MQCACF_EVENT_APPL_IDENTITY: types::MQCACF = types::MQCACF(3049);
        pub const MQCACF_EVENT_APPL_NAME: types::MQCACF = types::MQCACF(3050);
        pub const MQCACF_EVENT_APPL_ORIGIN: types::MQCACF = types::MQCACF(3051);
        pub const MQCACF_SUBSCRIPTION_NAME: types::MQCACF = types::MQCACF(3052);
        pub const MQCACF_REG_SUB_NAME: types::MQCACF = types::MQCACF(3053);
        pub const MQCACF_SUBSCRIPTION_IDENTITY: types::MQCACF = types::MQCACF(3054);
        pub const MQCACF_REG_SUB_IDENTITY: types::MQCACF = types::MQCACF(3055);
        pub const MQCACF_SUBSCRIPTION_USER_DATA: types::MQCACF = types::MQCACF(3056);
        pub const MQCACF_REG_SUB_USER_DATA: types::MQCACF = types::MQCACF(3057);
        pub const MQCACF_APPL_TAG: types::MQCACF = types::MQCACF(3058);
        pub const MQCACF_DATA_SET_NAME: types::MQCACF = types::MQCACF(3059);
        pub const MQCACF_UOW_START_DATE: types::MQCACF = types::MQCACF(3060);
        pub const MQCACF_UOW_START_TIME: types::MQCACF = types::MQCACF(3061);
        pub const MQCACF_UOW_LOG_START_DATE: types::MQCACF = types::MQCACF(3062);
        pub const MQCACF_UOW_LOG_START_TIME: types::MQCACF = types::MQCACF(3063);
        pub const MQCACF_UOW_LOG_EXTENT_NAME: types::MQCACF = types::MQCACF(3064);
        pub const MQCACF_PRINCIPAL_ENTITY_NAMES: types::MQCACF = types::MQCACF(3065);
        pub const MQCACF_GROUP_ENTITY_NAMES: types::MQCACF = types::MQCACF(3066);
        pub const MQCACF_AUTH_PROFILE_NAME: types::MQCACF = types::MQCACF(3067);
        pub const MQCACF_ENTITY_NAME: types::MQCACF = types::MQCACF(3068);
        pub const MQCACF_SERVICE_COMPONENT: types::MQCACF = types::MQCACF(3069);
        pub const MQCACF_RESPONSE_Q_MGR_NAME: types::MQCACF = types::MQCACF(3070);
        pub const MQCACF_CURRENT_LOG_EXTENT_NAME: types::MQCACF = types::MQCACF(3071);
        pub const MQCACF_RESTART_LOG_EXTENT_NAME: types::MQCACF = types::MQCACF(3072);
        pub const MQCACF_MEDIA_LOG_EXTENT_NAME: types::MQCACF = types::MQCACF(3073);
        pub const MQCACF_LOG_PATH: types::MQCACF = types::MQCACF(3074);
        pub const MQCACF_COMMAND_MQSC: types::MQCACF = types::MQCACF(3075);
        pub const MQCACF_Q_MGR_CPF: types::MQCACF = types::MQCACF(3076);
        pub const MQCACF_USAGE_LOG_RBA: types::MQCACF = types::MQCACF(3078);
        pub const MQCACF_USAGE_LOG_LRSN: types::MQCACF = types::MQCACF(3079);
        pub const MQCACF_COMMAND_SCOPE: types::MQCACF = types::MQCACF(3080);
        pub const MQCACF_ASID: types::MQCACF = types::MQCACF(3081);
        pub const MQCACF_PSB_NAME: types::MQCACF = types::MQCACF(3082);
        pub const MQCACF_PST_ID: types::MQCACF = types::MQCACF(3083);
        pub const MQCACF_TASK_NUMBER: types::MQCACF = types::MQCACF(3084);
        pub const MQCACF_TRANSACTION_ID: types::MQCACF = types::MQCACF(3085);
        pub const MQCACF_Q_MGR_UOW_ID: types::MQCACF = types::MQCACF(3086);
        pub const MQCACF_ORIGIN_NAME: types::MQCACF = types::MQCACF(3088);
        pub const MQCACF_ENV_INFO: types::MQCACF = types::MQCACF(3089);
        pub const MQCACF_SECURITY_PROFILE: types::MQCACF = types::MQCACF(3090);
        pub const MQCACF_CONFIGURATION_DATE: types::MQCACF = types::MQCACF(3091);
        pub const MQCACF_CONFIGURATION_TIME: types::MQCACF = types::MQCACF(3092);
        pub const MQCACF_FROM_CF_STRUC_NAME: types::MQCACF = types::MQCACF(3093);
        pub const MQCACF_TO_CF_STRUC_NAME: types::MQCACF = types::MQCACF(3094);
        pub const MQCACF_CF_STRUC_NAMES: types::MQCACF = types::MQCACF(3095);
        pub const MQCACF_FAIL_DATE: types::MQCACF = types::MQCACF(3096);
        pub const MQCACF_FAIL_TIME: types::MQCACF = types::MQCACF(3097);
        pub const MQCACF_BACKUP_DATE: types::MQCACF = types::MQCACF(3098);
        pub const MQCACF_BACKUP_TIME: types::MQCACF = types::MQCACF(3099);
        pub const MQCACF_SYSTEM_NAME: types::MQCACF = types::MQCACF(3100);
        pub const MQCACF_CF_STRUC_BACKUP_START: types::MQCACF = types::MQCACF(3101);
        pub const MQCACF_CF_STRUC_BACKUP_END: types::MQCACF = types::MQCACF(3102);
        pub const MQCACF_CF_STRUC_LOG_Q_MGRS: types::MQCACF = types::MQCACF(3103);
        pub const MQCACF_FROM_STORAGE_CLASS: types::MQCACF = types::MQCACF(3104);
        pub const MQCACF_TO_STORAGE_CLASS: types::MQCACF = types::MQCACF(3105);
        pub const MQCACF_STORAGE_CLASS_NAMES: types::MQCACF = types::MQCACF(3106);
        pub const MQCACF_DSG_NAME: types::MQCACF = types::MQCACF(3108);
        pub const MQCACF_DB2_NAME: types::MQCACF = types::MQCACF(3109);
        pub const MQCACF_SYSP_CMD_USER_ID: types::MQCACF = types::MQCACF(3110);
        pub const MQCACF_SYSP_OTMA_GROUP: types::MQCACF = types::MQCACF(3111);
        pub const MQCACF_SYSP_OTMA_MEMBER: types::MQCACF = types::MQCACF(3112);
        pub const MQCACF_SYSP_OTMA_DRU_EXIT: types::MQCACF = types::MQCACF(3113);
        pub const MQCACF_SYSP_OTMA_TPIPE_PFX: types::MQCACF = types::MQCACF(3114);
        pub const MQCACF_SYSP_ARCHIVE_PFX1: types::MQCACF = types::MQCACF(3115);
        pub const MQCACF_SYSP_ARCHIVE_UNIT1: types::MQCACF = types::MQCACF(3116);
        pub const MQCACF_SYSP_LOG_CORREL_ID: types::MQCACF = types::MQCACF(3117);
        pub const MQCACF_SYSP_UNIT_VOLSER: types::MQCACF = types::MQCACF(3118);
        pub const MQCACF_SYSP_Q_MGR_TIME: types::MQCACF = types::MQCACF(3119);
        pub const MQCACF_SYSP_Q_MGR_DATE: types::MQCACF = types::MQCACF(3120);
        pub const MQCACF_SYSP_Q_MGR_RBA: types::MQCACF = types::MQCACF(3121);
        pub const MQCACF_SYSP_LOG_RBA: types::MQCACF = types::MQCACF(3122);
        pub const MQCACF_SYSP_SERVICE: types::MQCACF = types::MQCACF(3123);
        pub const MQCACF_FROM_LISTENER_NAME: types::MQCACF = types::MQCACF(3124);
        pub const MQCACF_TO_LISTENER_NAME: types::MQCACF = types::MQCACF(3125);
        pub const MQCACF_FROM_SERVICE_NAME: types::MQCACF = types::MQCACF(3126);
        pub const MQCACF_TO_SERVICE_NAME: types::MQCACF = types::MQCACF(3127);
        pub const MQCACF_LAST_PUT_DATE: types::MQCACF = types::MQCACF(3128);
        pub const MQCACF_LAST_PUT_TIME: types::MQCACF = types::MQCACF(3129);
        pub const MQCACF_LAST_GET_DATE: types::MQCACF = types::MQCACF(3130);
        pub const MQCACF_LAST_GET_TIME: types::MQCACF = types::MQCACF(3131);
        pub const MQCACF_OPERATION_DATE: types::MQCACF = types::MQCACF(3132);
        pub const MQCACF_OPERATION_TIME: types::MQCACF = types::MQCACF(3133);
        pub const MQCACF_ACTIVITY_DESC: types::MQCACF = types::MQCACF(3134);
        pub const MQCACF_APPL_IDENTITY_DATA: types::MQCACF = types::MQCACF(3135);
        pub const MQCACF_APPL_ORIGIN_DATA: types::MQCACF = types::MQCACF(3136);
        pub const MQCACF_PUT_DATE: types::MQCACF = types::MQCACF(3137);
        pub const MQCACF_PUT_TIME: types::MQCACF = types::MQCACF(3138);
        pub const MQCACF_REPLY_TO_Q: types::MQCACF = types::MQCACF(3139);
        pub const MQCACF_REPLY_TO_Q_MGR: types::MQCACF = types::MQCACF(3140);
        pub const MQCACF_RESOLVED_Q_NAME: types::MQCACF = types::MQCACF(3141);
        pub const MQCACF_STRUC_ID: types::MQCACF = types::MQCACF(3142);
        pub const MQCACF_VALUE_NAME: types::MQCACF = types::MQCACF(3143);
        pub const MQCACF_SERVICE_START_DATE: types::MQCACF = types::MQCACF(3144);
        pub const MQCACF_SERVICE_START_TIME: types::MQCACF = types::MQCACF(3145);
        pub const MQCACF_SYSP_OFFLINE_RBA: types::MQCACF = types::MQCACF(3146);
        pub const MQCACF_SYSP_ARCHIVE_PFX2: types::MQCACF = types::MQCACF(3147);
        pub const MQCACF_SYSP_ARCHIVE_UNIT2: types::MQCACF = types::MQCACF(3148);
        pub const MQCACF_TO_TOPIC_NAME: types::MQCACF = types::MQCACF(3149);
        pub const MQCACF_FROM_TOPIC_NAME: types::MQCACF = types::MQCACF(3150);
        pub const MQCACF_TOPIC_NAMES: types::MQCACF = types::MQCACF(3151);
        pub const MQCACF_SUB_NAME: types::MQCACF = types::MQCACF(3152);
        pub const MQCACF_DESTINATION_Q_MGR: types::MQCACF = types::MQCACF(3153);
        pub const MQCACF_DESTINATION: types::MQCACF = types::MQCACF(3154);
        pub const MQCACF_SUB_USER_ID: types::MQCACF = types::MQCACF(3156);
        pub const MQCACF_SUB_USER_DATA: types::MQCACF = types::MQCACF(3159);
        pub const MQCACF_SUB_SELECTOR: types::MQCACF = types::MQCACF(3160);
        pub const MQCACF_LAST_PUB_DATE: types::MQCACF = types::MQCACF(3161);
        pub const MQCACF_LAST_PUB_TIME: types::MQCACF = types::MQCACF(3162);
        pub const MQCACF_FROM_SUB_NAME: types::MQCACF = types::MQCACF(3163);
        pub const MQCACF_TO_SUB_NAME: types::MQCACF = types::MQCACF(3164);
        pub const MQCACF_LAST_MSG_TIME: types::MQCACF = types::MQCACF(3167);
        pub const MQCACF_LAST_MSG_DATE: types::MQCACF = types::MQCACF(3168);
        pub const MQCACF_SUBSCRIPTION_POINT: types::MQCACF = types::MQCACF(3169);
        pub const MQCACF_FILTER: types::MQCACF = types::MQCACF(3170);
        pub const MQCACF_NONE: types::MQCACF = types::MQCACF(3171);
        pub const MQCACF_ADMIN_TOPIC_NAMES: types::MQCACF = types::MQCACF(3172);
        pub const MQCACF_ROUTING_FINGER_PRINT: types::MQCACF = types::MQCACF(3173);
        pub const MQCACF_APPL_DESC: types::MQCACF = types::MQCACF(3174);
        pub const MQCACF_Q_MGR_START_DATE: types::MQCACF = types::MQCACF(3175);
        pub const MQCACF_Q_MGR_START_TIME: types::MQCACF = types::MQCACF(3176);
        pub const MQCACF_FROM_COMM_INFO_NAME: types::MQCACF = types::MQCACF(3177);
        pub const MQCACF_TO_COMM_INFO_NAME: types::MQCACF = types::MQCACF(3178);
        pub const MQCACF_CF_OFFLOAD_SIZE1: types::MQCACF = types::MQCACF(3179);
        pub const MQCACF_CF_OFFLOAD_SIZE2: types::MQCACF = types::MQCACF(3180);
        pub const MQCACF_CF_OFFLOAD_SIZE3: types::MQCACF = types::MQCACF(3181);
        pub const MQCACF_CF_SMDS_GENERIC_NAME: types::MQCACF = types::MQCACF(3182);
        pub const MQCACF_CF_SMDS: types::MQCACF = types::MQCACF(3183);
        pub const MQCACF_RECOVERY_DATE: types::MQCACF = types::MQCACF(3184);
        pub const MQCACF_RECOVERY_TIME: types::MQCACF = types::MQCACF(3185);
        pub const MQCACF_CF_SMDSCONN: types::MQCACF = types::MQCACF(3186);
        pub const MQCACF_CF_STRUC_NAME: types::MQCACF = types::MQCACF(3187);
        pub const MQCACF_ALTERNATE_USERID: types::MQCACF = types::MQCACF(3188);
        pub const MQCACF_CHAR_ATTRS: types::MQCACF = types::MQCACF(3189);
        pub const MQCACF_DYNAMIC_Q_NAME: types::MQCACF = types::MQCACF(3190);
        pub const MQCACF_HOST_NAME: types::MQCACF = types::MQCACF(3191);
        pub const MQCACF_MQCB_NAME: types::MQCACF = types::MQCACF(3192);
        pub const MQCACF_OBJECT_STRING: types::MQCACF = types::MQCACF(3193);
        pub const MQCACF_RESOLVED_LOCAL_Q_MGR: types::MQCACF = types::MQCACF(3194);
        pub const MQCACF_RESOLVED_LOCAL_Q_NAME: types::MQCACF = types::MQCACF(3195);
        pub const MQCACF_RESOLVED_OBJECT_STRING: types::MQCACF = types::MQCACF(3196);
        pub const MQCACF_RESOLVED_Q_MGR: types::MQCACF = types::MQCACF(3197);
        pub const MQCACF_SELECTION_STRING: types::MQCACF = types::MQCACF(3198);
        pub const MQCACF_XA_INFO: types::MQCACF = types::MQCACF(3199);
        pub const MQCACF_APPL_FUNCTION: types::MQCACF = types::MQCACF(3200);
        pub const MQCACF_XQH_REMOTE_Q_NAME: types::MQCACF = types::MQCACF(3201);
        pub const MQCACF_XQH_REMOTE_Q_MGR: types::MQCACF = types::MQCACF(3202);
        pub const MQCACF_XQH_PUT_TIME: types::MQCACF = types::MQCACF(3203);
        pub const MQCACF_XQH_PUT_DATE: types::MQCACF = types::MQCACF(3204);
        pub const MQCACF_EXCL_OPERATOR_MESSAGES: types::MQCACF = types::MQCACF(3205);
        pub const MQCACF_CSP_USER_IDENTIFIER: types::MQCACF = types::MQCACF(3206);
        pub const MQCACF_AMQP_CLIENT_ID: types::MQCACF = types::MQCACF(3207);
        pub const MQCACF_ARCHIVE_LOG_EXTENT_NAME: types::MQCACF = types::MQCACF(3208);
        pub const MQCACF_APPL_IMMOVABLE_DATE: types::MQCACF = types::MQCACF(3209);
        pub const MQCACF_APPL_IMMOVABLE_TIME: types::MQCACF = types::MQCACF(3210);
        pub const MQCACF_NHA_INSTANCE_NAME: types::MQCACF = types::MQCACF(3211);
        pub const MQCACF_Q_MGR_DATA_PATH: types::MQCACF = types::MQCACF(3212);
        pub const MQCACF_UNIFORM_CLUSTER_NAME: types::MQCACF = types::MQCACF(3213);
        pub const MQCACF_LOG_START_DATE: types::MQCACF = types::MQCACF(3214);
        pub const MQCACF_LOG_START_LSN: types::MQCACF = types::MQCACF(3215);
        pub const MQCACF_LOG_START_TIME: types::MQCACF = types::MQCACF(3216);
        pub const MQCACF_NHA_GROUP_INITIAL_DATE: types::MQCACF = types::MQCACF(3217);
        pub const MQCACF_NHA_GROUP_INITIAL_LSN: types::MQCACF = types::MQCACF(3218);
        pub const MQCACF_NHA_GROUP_INITIAL_TIME: types::MQCACF = types::MQCACF(3219);
        pub const MQCACF_NHA_REPL_ADDRESS: types::MQCACF = types::MQCACF(3220);
        pub const MQCACF_DISK_WRITTEN_LSN: types::MQCACF = types::MQCACF(3221);
        pub const MQCACF_NHA_ACKNOWLEDGED_LSN: types::MQCACF = types::MQCACF(3222);
        pub const MQCACF_NHA_GROUP_ADDRESS: types::MQCACF = types::MQCACF(3223);
        pub const MQCACF_NHA_GROUP_SYNC_ISOTIME: types::MQCACF = types::MQCACF(3224);
        pub const MQCACF_NHA_GROUP_INIT_ISOTIME: types::MQCACF = types::MQCACF(3225);
        pub const MQCACF_NHA_GROUP_LIVE_ISOTIME: types::MQCACF = types::MQCACF(3226);
        pub const MQCACF_NHA_GROUP_LSN: types::MQCACF = types::MQCACF(3227);
        pub const MQCACF_NHA_GROUP_NAME: types::MQCACF = types::MQCACF(3228);
        pub const MQCACF_NHA_GROUP_RECOV_LSN: types::MQCACF = types::MQCACF(3229);
        pub const MQCACF_NHA_GROUP_RECOV_ISOTIME: types::MQCACF = types::MQCACF(3230);
        pub const MQCACF_NHA_SYNC_ISOTIME: types::MQCACF = types::MQCACF(3231);
        pub const MQCACF_EVENT_DUPLICATE_FROM: types::MQCACF = types::MQCACF(3232);
        pub const MQCACF_FIRST: types::MQCACF = types::MQCACF(3001);
        pub const MQCACF_LAST_USED: types::MQCACF = types::MQCACF(3232);
        pub const MQCACH_CHANNEL_NAME: types::MQCACH = types::MQCACH(3501);
        pub const MQCACH_DESC: types::MQCACH = types::MQCACH(3502);
        pub const MQCACH_MODE_NAME: types::MQCACH = types::MQCACH(3503);
        pub const MQCACH_TP_NAME: types::MQCACH = types::MQCACH(3504);
        pub const MQCACH_XMIT_Q_NAME: types::MQCACH = types::MQCACH(3505);
        pub const MQCACH_CONNECTION_NAME: types::MQCACH = types::MQCACH(3506);
        pub const MQCACH_MCA_NAME: types::MQCACH = types::MQCACH(3507);
        pub const MQCACH_SEC_EXIT_NAME: types::MQCACH = types::MQCACH(3508);
        pub const MQCACH_MSG_EXIT_NAME: types::MQCACH = types::MQCACH(3509);
        pub const MQCACH_SEND_EXIT_NAME: types::MQCACH = types::MQCACH(3510);
        pub const MQCACH_RCV_EXIT_NAME: types::MQCACH = types::MQCACH(3511);
        pub const MQCACH_CHANNEL_NAMES: types::MQCACH = types::MQCACH(3512);
        pub const MQCACH_SEC_EXIT_USER_DATA: types::MQCACH = types::MQCACH(3513);
        pub const MQCACH_MSG_EXIT_USER_DATA: types::MQCACH = types::MQCACH(3514);
        pub const MQCACH_SEND_EXIT_USER_DATA: types::MQCACH = types::MQCACH(3515);
        pub const MQCACH_RCV_EXIT_USER_DATA: types::MQCACH = types::MQCACH(3516);
        pub const MQCACH_USER_ID: types::MQCACH = types::MQCACH(3517);
        pub const MQCACH_PASSWORD: types::MQCACH = types::MQCACH(3518);
        pub const MQCACH_LOCAL_ADDRESS: types::MQCACH = types::MQCACH(3520);
        pub const MQCACH_LOCAL_NAME: types::MQCACH = types::MQCACH(3521);
        pub const MQCACH_LAST_MSG_TIME: types::MQCACH = types::MQCACH(3524);
        pub const MQCACH_LAST_MSG_DATE: types::MQCACH = types::MQCACH(3525);
        pub const MQCACH_MCA_USER_ID: types::MQCACH = types::MQCACH(3527);
        pub const MQCACH_CHANNEL_START_TIME: types::MQCACH = types::MQCACH(3528);
        pub const MQCACH_CHANNEL_START_DATE: types::MQCACH = types::MQCACH(3529);
        pub const MQCACH_MCA_JOB_NAME: types::MQCACH = types::MQCACH(3530);
        pub const MQCACH_LAST_LUWID: types::MQCACH = types::MQCACH(3531);
        pub const MQCACH_CURRENT_LUWID: types::MQCACH = types::MQCACH(3532);
        pub const MQCACH_FORMAT_NAME: types::MQCACH = types::MQCACH(3533);
        pub const MQCACH_MR_EXIT_NAME: types::MQCACH = types::MQCACH(3534);
        pub const MQCACH_MR_EXIT_USER_DATA: types::MQCACH = types::MQCACH(3535);
        pub const MQCACH_SSL_CIPHER_SPEC: types::MQCACH = types::MQCACH(3544);
        pub const MQCACH_SSL_PEER_NAME: types::MQCACH = types::MQCACH(3545);
        pub const MQCACH_SSL_HANDSHAKE_STAGE: types::MQCACH = types::MQCACH(3546);
        pub const MQCACH_SSL_SHORT_PEER_NAME: types::MQCACH = types::MQCACH(3547);
        pub const MQCACH_REMOTE_APPL_TAG: types::MQCACH = types::MQCACH(3548);
        pub const MQCACH_SSL_CERT_USER_ID: types::MQCACH = types::MQCACH(3549);
        pub const MQCACH_SSL_CERT_ISSUER_NAME: types::MQCACH = types::MQCACH(3550);
        pub const MQCACH_LU_NAME: types::MQCACH = types::MQCACH(3551);
        pub const MQCACH_IP_ADDRESS: types::MQCACH = types::MQCACH(3552);
        pub const MQCACH_TCP_NAME: types::MQCACH = types::MQCACH(3553);
        pub const MQCACH_LISTENER_NAME: types::MQCACH = types::MQCACH(3554);
        pub const MQCACH_LISTENER_DESC: types::MQCACH = types::MQCACH(3555);
        pub const MQCACH_LISTENER_START_DATE: types::MQCACH = types::MQCACH(3556);
        pub const MQCACH_LISTENER_START_TIME: types::MQCACH = types::MQCACH(3557);
        pub const MQCACH_SSL_KEY_RESET_DATE: types::MQCACH = types::MQCACH(3558);
        pub const MQCACH_SSL_KEY_RESET_TIME: types::MQCACH = types::MQCACH(3559);
        pub const MQCACH_REMOTE_VERSION: types::MQCACH = types::MQCACH(3560);
        pub const MQCACH_REMOTE_PRODUCT: types::MQCACH = types::MQCACH(3561);
        pub const MQCACH_GROUP_ADDRESS: types::MQCACH = types::MQCACH(3562);
        pub const MQCACH_JAAS_CONFIG: types::MQCACH = types::MQCACH(3563);
        pub const MQCACH_CLIENT_ID: types::MQCACH = types::MQCACH(3564);
        pub const MQCACH_SSL_KEY_PASSPHRASE: types::MQCACH = types::MQCACH(3565);
        pub const MQCACH_CONNECTION_NAME_LIST: types::MQCACH = types::MQCACH(3566);
        pub const MQCACH_CLIENT_USER_ID: types::MQCACH = types::MQCACH(3567);
        pub const MQCACH_MCA_USER_ID_LIST: types::MQCACH = types::MQCACH(3568);
        pub const MQCACH_SSL_CIPHER_SUITE: types::MQCACH = types::MQCACH(3569);
        pub const MQCACH_WEBCONTENT_PATH: types::MQCACH = types::MQCACH(3570);
        pub const MQCACH_TOPIC_ROOT: types::MQCACH = types::MQCACH(3571);
        pub const MQCACH_TEMPORARY_MODEL_Q: types::MQCACH = types::MQCACH(3572);
        pub const MQCACH_TEMPORARY_Q_PREFIX: types::MQCACH = types::MQCACH(3573);
        pub const MQCACH_FIRST: types::MQCACH = types::MQCACH(3501);
        pub const MQCACH_LAST_USED: types::MQCACH = types::MQCACH(3573);
        pub const MQCAMO_CLOSE_DATE: types::MQCAMO = types::MQCAMO(2701);
        pub const MQCAMO_CLOSE_TIME: types::MQCAMO = types::MQCAMO(2702);
        pub const MQCAMO_CONN_DATE: types::MQCAMO = types::MQCAMO(2703);
        pub const MQCAMO_CONN_TIME: types::MQCAMO = types::MQCAMO(2704);
        pub const MQCAMO_DISC_DATE: types::MQCAMO = types::MQCAMO(2705);
        pub const MQCAMO_DISC_TIME: types::MQCAMO = types::MQCAMO(2706);
        pub const MQCAMO_END_DATE: types::MQCAMO = types::MQCAMO(2707);
        pub const MQCAMO_END_TIME: types::MQCAMO = types::MQCAMO(2708);
        pub const MQCAMO_OPEN_DATE: types::MQCAMO = types::MQCAMO(2709);
        pub const MQCAMO_OPEN_TIME: types::MQCAMO = types::MQCAMO(2710);
        pub const MQCAMO_START_DATE: types::MQCAMO = types::MQCAMO(2711);
        pub const MQCAMO_START_TIME: types::MQCAMO = types::MQCAMO(2712);
        pub const MQCAMO_MONITOR_CLASS: types::MQCAMO = types::MQCAMO(2713);
        pub const MQCAMO_MONITOR_TYPE: types::MQCAMO = types::MQCAMO(2714);
        pub const MQCAMO_MONITOR_DESC: types::MQCAMO = types::MQCAMO(2715);
        pub const MQCAMO_FIRST: types::MQCAMO = types::MQCAMO(2701);
        pub const MQCAMO_LAST_USED: types::MQCAMO = types::MQCAMO(2715);
        pub const MQCAUT_ALL: types::MQCAUT = types::MQCAUT(0);
        pub const MQCAUT_BLOCKUSER: types::MQCAUT = types::MQCAUT(1);
        pub const MQCAUT_BLOCKADDR: types::MQCAUT = types::MQCAUT(2);
        pub const MQCAUT_SSLPEERMAP: types::MQCAUT = types::MQCAUT(3);
        pub const MQCAUT_ADDRESSMAP: types::MQCAUT = types::MQCAUT(4);
        pub const MQCAUT_USERMAP: types::MQCAUT = types::MQCAUT(5);
        pub const MQCAUT_QMGRMAP: types::MQCAUT = types::MQCAUT(6);
        pub const MQCFACCESS_ENABLED: types::MQCFACCESS = types::MQCFACCESS(0);
        pub const MQCFACCESS_SUSPENDED: types::MQCFACCESS = types::MQCFACCESS(1);
        pub const MQCFACCESS_DISABLED: types::MQCFACCESS = types::MQCFACCESS(2);
        pub const MQCFC_NOT_LAST: types::MQCFC = types::MQCFC(0);
        pub const MQCFC_LAST: types::MQCFC = types::MQCFC(1);
        pub const MQCFOP_LESS: types::MQCFOP = types::MQCFOP(1);
        pub const MQCFOP_EQUAL: types::MQCFOP = types::MQCFOP(2);
        pub const MQCFOP_NOT_GREATER: types::MQCFOP = types::MQCFOP(3);
        pub const MQCFOP_GREATER: types::MQCFOP = types::MQCFOP(4);
        pub const MQCFOP_NOT_EQUAL: types::MQCFOP = types::MQCFOP(5);
        pub const MQCFOP_NOT_LESS: types::MQCFOP = types::MQCFOP(6);
        pub const MQCFOP_CONTAINS: types::MQCFOP = types::MQCFOP(10);
        pub const MQCFOP_EXCLUDES: types::MQCFOP = types::MQCFOP(13);
        pub const MQCFOP_LIKE: types::MQCFOP = types::MQCFOP(18);
        pub const MQCFOP_NOT_LIKE: types::MQCFOP = types::MQCFOP(21);
        pub const MQCFOP_CONTAINS_GEN: types::MQCFOP = types::MQCFOP(26);
        pub const MQCFOP_EXCLUDES_GEN: types::MQCFOP = types::MQCFOP(29);
        pub const MQCFO_REFRESH_REPOSITORY_NO: types::MQCFO_REFRESH = types::MQCFO_REFRESH(
            0,
        );
        pub const MQCFO_REFRESH_REPOSITORY_YES: types::MQCFO_REFRESH = types::MQCFO_REFRESH(
            1,
        );
        pub const MQCFO_REMOVE_QUEUES_NO: types::MQCFO_REMOVE = types::MQCFO_REMOVE(0);
        pub const MQCFO_REMOVE_QUEUES_YES: types::MQCFO_REMOVE = types::MQCFO_REMOVE(1);
        pub const MQCFSTATUS_NOT_FOUND: types::MQCFSTATUS = types::MQCFSTATUS(0);
        pub const MQCFSTATUS_ACTIVE: types::MQCFSTATUS = types::MQCFSTATUS(1);
        pub const MQCFSTATUS_IN_RECOVER: types::MQCFSTATUS = types::MQCFSTATUS(2);
        pub const MQCFSTATUS_IN_BACKUP: types::MQCFSTATUS = types::MQCFSTATUS(3);
        pub const MQCFSTATUS_FAILED: types::MQCFSTATUS = types::MQCFSTATUS(4);
        pub const MQCFSTATUS_NONE: types::MQCFSTATUS = types::MQCFSTATUS(5);
        pub const MQCFSTATUS_UNKNOWN: types::MQCFSTATUS = types::MQCFSTATUS(6);
        pub const MQCFSTATUS_RECOVERED: types::MQCFSTATUS = types::MQCFSTATUS(7);
        pub const MQCFSTATUS_EMPTY: types::MQCFSTATUS = types::MQCFSTATUS(8);
        pub const MQCFSTATUS_NEW: types::MQCFSTATUS = types::MQCFSTATUS(9);
        pub const MQCFSTATUS_ADMIN_INCOMPLETE: types::MQCFSTATUS = types::MQCFSTATUS(20);
        pub const MQCFSTATUS_NEVER_USED: types::MQCFSTATUS = types::MQCFSTATUS(21);
        pub const MQCFSTATUS_NO_BACKUP: types::MQCFSTATUS = types::MQCFSTATUS(22);
        pub const MQCFSTATUS_NOT_FAILED: types::MQCFSTATUS = types::MQCFSTATUS(23);
        pub const MQCFSTATUS_NOT_RECOVERABLE: types::MQCFSTATUS = types::MQCFSTATUS(24);
        pub const MQCFSTATUS_XES_ERROR: types::MQCFSTATUS = types::MQCFSTATUS(25);
        pub const MQCFTYPE_APPL: types::MQCFTYPE = types::MQCFTYPE(0);
        pub const MQCFTYPE_ADMIN: types::MQCFTYPE = types::MQCFTYPE(1);
        pub const MQCFT_NONE: types::MQCFT = types::MQCFT(0);
        pub const MQCFT_COMMAND: types::MQCFT = types::MQCFT(1);
        pub const MQCFT_RESPONSE: types::MQCFT = types::MQCFT(2);
        pub const MQCFT_INTEGER: types::MQCFT = types::MQCFT(3);
        pub const MQCFT_STRING: types::MQCFT = types::MQCFT(4);
        pub const MQCFT_INTEGER_LIST: types::MQCFT = types::MQCFT(5);
        pub const MQCFT_STRING_LIST: types::MQCFT = types::MQCFT(6);
        pub const MQCFT_EVENT: types::MQCFT = types::MQCFT(7);
        pub const MQCFT_USER: types::MQCFT = types::MQCFT(8);
        pub const MQCFT_BYTE_STRING: types::MQCFT = types::MQCFT(9);
        pub const MQCFT_TRACE_ROUTE: types::MQCFT = types::MQCFT(10);
        pub const MQCFT_REPORT: types::MQCFT = types::MQCFT(12);
        pub const MQCFT_INTEGER_FILTER: types::MQCFT = types::MQCFT(13);
        pub const MQCFT_STRING_FILTER: types::MQCFT = types::MQCFT(14);
        pub const MQCFT_BYTE_STRING_FILTER: types::MQCFT = types::MQCFT(15);
        pub const MQCFT_COMMAND_XR: types::MQCFT = types::MQCFT(16);
        pub const MQCFT_XR_MSG: types::MQCFT = types::MQCFT(17);
        pub const MQCFT_XR_ITEM: types::MQCFT = types::MQCFT(18);
        pub const MQCFT_XR_SUMMARY: types::MQCFT = types::MQCFT(19);
        pub const MQCFT_GROUP: types::MQCFT = types::MQCFT(20);
        pub const MQCFT_STATISTICS: types::MQCFT = types::MQCFT(21);
        pub const MQCFT_ACCOUNTING: types::MQCFT = types::MQCFT(22);
        pub const MQCFT_INTEGER64: types::MQCFT = types::MQCFT(23);
        pub const MQCFT_INTEGER64_LIST: types::MQCFT = types::MQCFT(25);
        pub const MQCFT_APP_ACTIVITY: types::MQCFT = types::MQCFT(26);
        pub const MQCFT_STATUS: types::MQCFT = types::MQCFT(27);
        pub const MQCHIDS_NOT_INDOUBT: types::MQCHIDS = types::MQCHIDS(0);
        pub const MQCHIDS_INDOUBT: types::MQCHIDS = types::MQCHIDS(1);
        pub const MQCHK_OPTIONAL: types::MQCHK = types::MQCHK(0);
        pub const MQCHK_NONE: types::MQCHK = types::MQCHK(1);
        pub const MQCHK_REQUIRED_ADMIN: types::MQCHK = types::MQCHK(2);
        pub const MQCHK_REQUIRED: types::MQCHK = types::MQCHK(3);
        pub const MQCHK_AS_Q_MGR: types::MQCHK = types::MQCHK(4);
        pub const MQCHLA_DISABLED: types::MQCHLA = types::MQCHLA(0);
        pub const MQCHLA_ENABLED: types::MQCHLA = types::MQCHLA(1);
        pub const MQCHLD_ALL: types::MQCHLD = types::MQCHLD(-1);
        pub const MQCHLD_DEFAULT: types::MQCHLD = types::MQCHLD(1);
        pub const MQCHLD_SHARED: types::MQCHLD = types::MQCHLD(2);
        pub const MQCHLD_PRIVATE: types::MQCHLD = types::MQCHLD(4);
        pub const MQCHLD_FIXSHARED: types::MQCHLD = types::MQCHLD(5);
        pub const MQCHRR_RESET_NOT_REQUESTED: types::MQCHRR = types::MQCHRR(0);
        pub const MQCHSH_RESTART_NO: types::MQCHSH = types::MQCHSH(0);
        pub const MQCHSH_RESTART_YES: types::MQCHSH = types::MQCHSH(1);
        pub const MQCHSR_STOP_NOT_REQUESTED: types::MQCHSR = types::MQCHSR(0);
        pub const MQCHSR_STOP_REQUESTED: types::MQCHSR = types::MQCHSR(1);
        pub const MQCHSSTATE_OTHER: types::MQCHSSTATE = types::MQCHSSTATE(0);
        pub const MQCHSSTATE_END_OF_BATCH: types::MQCHSSTATE = types::MQCHSSTATE(100);
        pub const MQCHSSTATE_SENDING: types::MQCHSSTATE = types::MQCHSSTATE(200);
        pub const MQCHSSTATE_RECEIVING: types::MQCHSSTATE = types::MQCHSSTATE(300);
        pub const MQCHSSTATE_SERIALIZING: types::MQCHSSTATE = types::MQCHSSTATE(400);
        pub const MQCHSSTATE_RESYNCHING: types::MQCHSSTATE = types::MQCHSSTATE(500);
        pub const MQCHSSTATE_HEARTBEATING: types::MQCHSSTATE = types::MQCHSSTATE(600);
        pub const MQCHSSTATE_IN_SCYEXIT: types::MQCHSSTATE = types::MQCHSSTATE(700);
        pub const MQCHSSTATE_IN_RCVEXIT: types::MQCHSSTATE = types::MQCHSSTATE(800);
        pub const MQCHSSTATE_IN_SENDEXIT: types::MQCHSSTATE = types::MQCHSSTATE(900);
        pub const MQCHSSTATE_IN_MSGEXIT: types::MQCHSSTATE = types::MQCHSSTATE(1000);
        pub const MQCHSSTATE_IN_MREXIT: types::MQCHSSTATE = types::MQCHSSTATE(1100);
        pub const MQCHSSTATE_IN_CHADEXIT: types::MQCHSSTATE = types::MQCHSSTATE(1200);
        pub const MQCHSSTATE_NET_CONNECTING: types::MQCHSSTATE = types::MQCHSSTATE(1250);
        pub const MQCHSSTATE_SSL_HANDSHAKING: types::MQCHSSTATE = types::MQCHSSTATE(
            1300,
        );
        pub const MQCHSSTATE_NAME_SERVER: types::MQCHSSTATE = types::MQCHSSTATE(1400);
        pub const MQCHSSTATE_IN_MQPUT: types::MQCHSSTATE = types::MQCHSSTATE(1500);
        pub const MQCHSSTATE_IN_MQGET: types::MQCHSSTATE = types::MQCHSSTATE(1600);
        pub const MQCHSSTATE_IN_MQI_CALL: types::MQCHSSTATE = types::MQCHSSTATE(1700);
        pub const MQCHSSTATE_COMPRESSING: types::MQCHSSTATE = types::MQCHSSTATE(1800);
        pub const MQCHS_INACTIVE: types::MQCHS = types::MQCHS(0);
        pub const MQCHS_BINDING: types::MQCHS = types::MQCHS(1);
        pub const MQCHS_STARTING: types::MQCHS = types::MQCHS(2);
        pub const MQCHS_RUNNING: types::MQCHS = types::MQCHS(3);
        pub const MQCHS_STOPPING: types::MQCHS = types::MQCHS(4);
        pub const MQCHS_RETRYING: types::MQCHS = types::MQCHS(5);
        pub const MQCHS_STOPPED: types::MQCHS = types::MQCHS(6);
        pub const MQCHS_REQUESTING: types::MQCHS = types::MQCHS(7);
        pub const MQCHS_PAUSED: types::MQCHS = types::MQCHS(8);
        pub const MQCHS_DISCONNECTED: types::MQCHS = types::MQCHS(9);
        pub const MQCHS_INITIALIZING: types::MQCHS = types::MQCHS(13);
        pub const MQCHS_SWITCHING: types::MQCHS = types::MQCHS(14);
        pub const MQCHTAB_Q_MGR: types::MQCHTAB = types::MQCHTAB(1);
        pub const MQCHTAB_CLNTCONN: types::MQCHTAB = types::MQCHTAB(2);
        pub const MQCLROUTE_DIRECT: types::MQCLROUTE = types::MQCLROUTE(0);
        pub const MQCLROUTE_TOPIC_HOST: types::MQCLROUTE = types::MQCLROUTE(1);
        pub const MQCLROUTE_NONE: types::MQCLROUTE = types::MQCLROUTE(2);
        pub const MQCLRS_LOCAL: types::MQCLRS = types::MQCLRS(1);
        pub const MQCLRS_GLOBAL: types::MQCLRS = types::MQCLRS(2);
        pub const MQCLRT_RETAINED: types::MQCLRT = types::MQCLRT(1);
        pub const MQCLST_ACTIVE: types::MQCLST = types::MQCLST(0);
        pub const MQCLST_PENDING: types::MQCLST = types::MQCLST(1);
        pub const MQCLST_INVALID: types::MQCLST = types::MQCLST(2);
        pub const MQCLST_ERROR: types::MQCLST = types::MQCLST(3);
        pub const MQCLXQ_SCTQ: types::MQCLXQ = types::MQCLXQ(0);
        pub const MQCLXQ_CHANNEL: types::MQCLXQ = types::MQCLXQ(1);
        pub const MQCMDI_CMDSCOPE_ACCEPTED: types::MQCMDI = types::MQCMDI(1);
        pub const MQCMDI_CMDSCOPE_GENERATED: types::MQCMDI = types::MQCMDI(2);
        pub const MQCMDI_CMDSCOPE_COMPLETED: types::MQCMDI = types::MQCMDI(3);
        pub const MQCMDI_QSG_DISP_COMPLETED: types::MQCMDI = types::MQCMDI(4);
        pub const MQCMDI_COMMAND_ACCEPTED: types::MQCMDI = types::MQCMDI(5);
        pub const MQCMDI_CLUSTER_REQUEST_QUEUED: types::MQCMDI = types::MQCMDI(6);
        pub const MQCMDI_CHANNEL_INIT_STARTED: types::MQCMDI = types::MQCMDI(7);
        pub const MQCMDI_RECOVER_STARTED: types::MQCMDI = types::MQCMDI(11);
        pub const MQCMDI_BACKUP_STARTED: types::MQCMDI = types::MQCMDI(12);
        pub const MQCMDI_RECOVER_COMPLETED: types::MQCMDI = types::MQCMDI(13);
        pub const MQCMDI_SEC_TIMER_ZERO: types::MQCMDI = types::MQCMDI(14);
        pub const MQCMDI_REFRESH_CONFIGURATION: types::MQCMDI = types::MQCMDI(16);
        pub const MQCMDI_SEC_SIGNOFF_ERROR: types::MQCMDI = types::MQCMDI(17);
        pub const MQCMDI_IMS_BRIDGE_SUSPENDED: types::MQCMDI = types::MQCMDI(18);
        pub const MQCMDI_DB2_SUSPENDED: types::MQCMDI = types::MQCMDI(19);
        pub const MQCMDI_DB2_OBSOLETE_MSGS: types::MQCMDI = types::MQCMDI(20);
        pub const MQCMDI_SEC_UPPERCASE: types::MQCMDI = types::MQCMDI(21);
        pub const MQCMDI_SEC_MIXEDCASE: types::MQCMDI = types::MQCMDI(22);
        pub const MQCMD_NONE: types::MQCMD = types::MQCMD(0);
        pub const MQCMD_CHANGE_Q_MGR: types::MQCMD = types::MQCMD(1);
        pub const MQCMD_INQUIRE_Q_MGR: types::MQCMD = types::MQCMD(2);
        pub const MQCMD_CHANGE_PROCESS: types::MQCMD = types::MQCMD(3);
        pub const MQCMD_COPY_PROCESS: types::MQCMD = types::MQCMD(4);
        pub const MQCMD_CREATE_PROCESS: types::MQCMD = types::MQCMD(5);
        pub const MQCMD_DELETE_PROCESS: types::MQCMD = types::MQCMD(6);
        pub const MQCMD_INQUIRE_PROCESS: types::MQCMD = types::MQCMD(7);
        pub const MQCMD_CHANGE_Q: types::MQCMD = types::MQCMD(8);
        pub const MQCMD_CLEAR_Q: types::MQCMD = types::MQCMD(9);
        pub const MQCMD_COPY_Q: types::MQCMD = types::MQCMD(10);
        pub const MQCMD_CREATE_Q: types::MQCMD = types::MQCMD(11);
        pub const MQCMD_DELETE_Q: types::MQCMD = types::MQCMD(12);
        pub const MQCMD_INQUIRE_Q: types::MQCMD = types::MQCMD(13);
        pub const MQCMD_REFRESH_Q_MGR: types::MQCMD = types::MQCMD(16);
        pub const MQCMD_RESET_Q_STATS: types::MQCMD = types::MQCMD(17);
        pub const MQCMD_INQUIRE_Q_NAMES: types::MQCMD = types::MQCMD(18);
        pub const MQCMD_INQUIRE_PROCESS_NAMES: types::MQCMD = types::MQCMD(19);
        pub const MQCMD_INQUIRE_CHANNEL_NAMES: types::MQCMD = types::MQCMD(20);
        pub const MQCMD_CHANGE_CHANNEL: types::MQCMD = types::MQCMD(21);
        pub const MQCMD_COPY_CHANNEL: types::MQCMD = types::MQCMD(22);
        pub const MQCMD_CREATE_CHANNEL: types::MQCMD = types::MQCMD(23);
        pub const MQCMD_DELETE_CHANNEL: types::MQCMD = types::MQCMD(24);
        pub const MQCMD_INQUIRE_CHANNEL: types::MQCMD = types::MQCMD(25);
        pub const MQCMD_PING_CHANNEL: types::MQCMD = types::MQCMD(26);
        pub const MQCMD_RESET_CHANNEL: types::MQCMD = types::MQCMD(27);
        pub const MQCMD_START_CHANNEL: types::MQCMD = types::MQCMD(28);
        pub const MQCMD_STOP_CHANNEL: types::MQCMD = types::MQCMD(29);
        pub const MQCMD_START_CHANNEL_INIT: types::MQCMD = types::MQCMD(30);
        pub const MQCMD_START_CHANNEL_LISTENER: types::MQCMD = types::MQCMD(31);
        pub const MQCMD_CHANGE_NAMELIST: types::MQCMD = types::MQCMD(32);
        pub const MQCMD_COPY_NAMELIST: types::MQCMD = types::MQCMD(33);
        pub const MQCMD_CREATE_NAMELIST: types::MQCMD = types::MQCMD(34);
        pub const MQCMD_DELETE_NAMELIST: types::MQCMD = types::MQCMD(35);
        pub const MQCMD_INQUIRE_NAMELIST: types::MQCMD = types::MQCMD(36);
        pub const MQCMD_INQUIRE_NAMELIST_NAMES: types::MQCMD = types::MQCMD(37);
        pub const MQCMD_ESCAPE: types::MQCMD = types::MQCMD(38);
        pub const MQCMD_RESOLVE_CHANNEL: types::MQCMD = types::MQCMD(39);
        pub const MQCMD_PING_Q_MGR: types::MQCMD = types::MQCMD(40);
        pub const MQCMD_INQUIRE_Q_STATUS: types::MQCMD = types::MQCMD(41);
        pub const MQCMD_INQUIRE_CHANNEL_STATUS: types::MQCMD = types::MQCMD(42);
        pub const MQCMD_CONFIG_EVENT: types::MQCMD = types::MQCMD(43);
        pub const MQCMD_Q_MGR_EVENT: types::MQCMD = types::MQCMD(44);
        pub const MQCMD_PERFM_EVENT: types::MQCMD = types::MQCMD(45);
        pub const MQCMD_CHANNEL_EVENT: types::MQCMD = types::MQCMD(46);
        pub const MQCMD_DELETE_PUBLICATION: types::MQCMD = types::MQCMD(60);
        pub const MQCMD_DEREGISTER_PUBLISHER: types::MQCMD = types::MQCMD(61);
        pub const MQCMD_DEREGISTER_SUBSCRIBER: types::MQCMD = types::MQCMD(62);
        pub const MQCMD_PUBLISH: types::MQCMD = types::MQCMD(63);
        pub const MQCMD_REGISTER_PUBLISHER: types::MQCMD = types::MQCMD(64);
        pub const MQCMD_REGISTER_SUBSCRIBER: types::MQCMD = types::MQCMD(65);
        pub const MQCMD_REQUEST_UPDATE: types::MQCMD = types::MQCMD(66);
        pub const MQCMD_BROKER_INTERNAL: types::MQCMD = types::MQCMD(67);
        pub const MQCMD_ACTIVITY_MSG: types::MQCMD = types::MQCMD(69);
        pub const MQCMD_INQUIRE_CLUSTER_Q_MGR: types::MQCMD = types::MQCMD(70);
        pub const MQCMD_RESUME_Q_MGR_CLUSTER: types::MQCMD = types::MQCMD(71);
        pub const MQCMD_SUSPEND_Q_MGR_CLUSTER: types::MQCMD = types::MQCMD(72);
        pub const MQCMD_REFRESH_CLUSTER: types::MQCMD = types::MQCMD(73);
        pub const MQCMD_RESET_CLUSTER: types::MQCMD = types::MQCMD(74);
        pub const MQCMD_TRACE_ROUTE: types::MQCMD = types::MQCMD(75);
        pub const MQCMD_REFRESH_SECURITY: types::MQCMD = types::MQCMD(78);
        pub const MQCMD_CHANGE_AUTH_INFO: types::MQCMD = types::MQCMD(79);
        pub const MQCMD_COPY_AUTH_INFO: types::MQCMD = types::MQCMD(80);
        pub const MQCMD_CREATE_AUTH_INFO: types::MQCMD = types::MQCMD(81);
        pub const MQCMD_DELETE_AUTH_INFO: types::MQCMD = types::MQCMD(82);
        pub const MQCMD_INQUIRE_AUTH_INFO: types::MQCMD = types::MQCMD(83);
        pub const MQCMD_INQUIRE_AUTH_INFO_NAMES: types::MQCMD = types::MQCMD(84);
        pub const MQCMD_INQUIRE_CONNECTION: types::MQCMD = types::MQCMD(85);
        pub const MQCMD_STOP_CONNECTION: types::MQCMD = types::MQCMD(86);
        pub const MQCMD_INQUIRE_AUTH_RECS: types::MQCMD = types::MQCMD(87);
        pub const MQCMD_INQUIRE_ENTITY_AUTH: types::MQCMD = types::MQCMD(88);
        pub const MQCMD_DELETE_AUTH_REC: types::MQCMD = types::MQCMD(89);
        pub const MQCMD_SET_AUTH_REC: types::MQCMD = types::MQCMD(90);
        pub const MQCMD_LOGGER_EVENT: types::MQCMD = types::MQCMD(91);
        pub const MQCMD_RESET_Q_MGR: types::MQCMD = types::MQCMD(92);
        pub const MQCMD_CHANGE_LISTENER: types::MQCMD = types::MQCMD(93);
        pub const MQCMD_COPY_LISTENER: types::MQCMD = types::MQCMD(94);
        pub const MQCMD_CREATE_LISTENER: types::MQCMD = types::MQCMD(95);
        pub const MQCMD_DELETE_LISTENER: types::MQCMD = types::MQCMD(96);
        pub const MQCMD_INQUIRE_LISTENER: types::MQCMD = types::MQCMD(97);
        pub const MQCMD_INQUIRE_LISTENER_STATUS: types::MQCMD = types::MQCMD(98);
        pub const MQCMD_COMMAND_EVENT: types::MQCMD = types::MQCMD(99);
        pub const MQCMD_CHANGE_SECURITY: types::MQCMD = types::MQCMD(100);
        pub const MQCMD_CHANGE_CF_STRUC: types::MQCMD = types::MQCMD(101);
        pub const MQCMD_CHANGE_STG_CLASS: types::MQCMD = types::MQCMD(102);
        pub const MQCMD_CHANGE_TRACE: types::MQCMD = types::MQCMD(103);
        pub const MQCMD_ARCHIVE_LOG: types::MQCMD = types::MQCMD(104);
        pub const MQCMD_BACKUP_CF_STRUC: types::MQCMD = types::MQCMD(105);
        pub const MQCMD_CREATE_BUFFER_POOL: types::MQCMD = types::MQCMD(106);
        pub const MQCMD_CREATE_PAGE_SET: types::MQCMD = types::MQCMD(107);
        pub const MQCMD_CREATE_CF_STRUC: types::MQCMD = types::MQCMD(108);
        pub const MQCMD_CREATE_STG_CLASS: types::MQCMD = types::MQCMD(109);
        pub const MQCMD_COPY_CF_STRUC: types::MQCMD = types::MQCMD(110);
        pub const MQCMD_COPY_STG_CLASS: types::MQCMD = types::MQCMD(111);
        pub const MQCMD_DELETE_CF_STRUC: types::MQCMD = types::MQCMD(112);
        pub const MQCMD_DELETE_STG_CLASS: types::MQCMD = types::MQCMD(113);
        pub const MQCMD_INQUIRE_ARCHIVE: types::MQCMD = types::MQCMD(114);
        pub const MQCMD_INQUIRE_CF_STRUC: types::MQCMD = types::MQCMD(115);
        pub const MQCMD_INQUIRE_CF_STRUC_STATUS: types::MQCMD = types::MQCMD(116);
        pub const MQCMD_INQUIRE_CMD_SERVER: types::MQCMD = types::MQCMD(117);
        pub const MQCMD_INQUIRE_CHANNEL_INIT: types::MQCMD = types::MQCMD(118);
        pub const MQCMD_INQUIRE_QSG: types::MQCMD = types::MQCMD(119);
        pub const MQCMD_INQUIRE_LOG: types::MQCMD = types::MQCMD(120);
        pub const MQCMD_INQUIRE_SECURITY: types::MQCMD = types::MQCMD(121);
        pub const MQCMD_INQUIRE_STG_CLASS: types::MQCMD = types::MQCMD(122);
        pub const MQCMD_INQUIRE_SYSTEM: types::MQCMD = types::MQCMD(123);
        pub const MQCMD_INQUIRE_THREAD: types::MQCMD = types::MQCMD(124);
        pub const MQCMD_INQUIRE_TRACE: types::MQCMD = types::MQCMD(125);
        pub const MQCMD_INQUIRE_USAGE: types::MQCMD = types::MQCMD(126);
        pub const MQCMD_MOVE_Q: types::MQCMD = types::MQCMD(127);
        pub const MQCMD_RECOVER_BSDS: types::MQCMD = types::MQCMD(128);
        pub const MQCMD_RECOVER_CF_STRUC: types::MQCMD = types::MQCMD(129);
        pub const MQCMD_RESET_TPIPE: types::MQCMD = types::MQCMD(130);
        pub const MQCMD_RESOLVE_INDOUBT: types::MQCMD = types::MQCMD(131);
        pub const MQCMD_RESUME_Q_MGR: types::MQCMD = types::MQCMD(132);
        pub const MQCMD_REVERIFY_SECURITY: types::MQCMD = types::MQCMD(133);
        pub const MQCMD_SET_ARCHIVE: types::MQCMD = types::MQCMD(134);
        pub const MQCMD_SET_LOG: types::MQCMD = types::MQCMD(136);
        pub const MQCMD_SET_SYSTEM: types::MQCMD = types::MQCMD(137);
        pub const MQCMD_START_CMD_SERVER: types::MQCMD = types::MQCMD(138);
        pub const MQCMD_START_Q_MGR: types::MQCMD = types::MQCMD(139);
        pub const MQCMD_START_TRACE: types::MQCMD = types::MQCMD(140);
        pub const MQCMD_STOP_CHANNEL_INIT: types::MQCMD = types::MQCMD(141);
        pub const MQCMD_STOP_CHANNEL_LISTENER: types::MQCMD = types::MQCMD(142);
        pub const MQCMD_STOP_CMD_SERVER: types::MQCMD = types::MQCMD(143);
        pub const MQCMD_STOP_Q_MGR: types::MQCMD = types::MQCMD(144);
        pub const MQCMD_STOP_TRACE: types::MQCMD = types::MQCMD(145);
        pub const MQCMD_SUSPEND_Q_MGR: types::MQCMD = types::MQCMD(146);
        pub const MQCMD_INQUIRE_CF_STRUC_NAMES: types::MQCMD = types::MQCMD(147);
        pub const MQCMD_INQUIRE_STG_CLASS_NAMES: types::MQCMD = types::MQCMD(148);
        pub const MQCMD_CHANGE_SERVICE: types::MQCMD = types::MQCMD(149);
        pub const MQCMD_COPY_SERVICE: types::MQCMD = types::MQCMD(150);
        pub const MQCMD_CREATE_SERVICE: types::MQCMD = types::MQCMD(151);
        pub const MQCMD_DELETE_SERVICE: types::MQCMD = types::MQCMD(152);
        pub const MQCMD_INQUIRE_SERVICE: types::MQCMD = types::MQCMD(153);
        pub const MQCMD_INQUIRE_SERVICE_STATUS: types::MQCMD = types::MQCMD(154);
        pub const MQCMD_START_SERVICE: types::MQCMD = types::MQCMD(155);
        pub const MQCMD_STOP_SERVICE: types::MQCMD = types::MQCMD(156);
        pub const MQCMD_DELETE_BUFFER_POOL: types::MQCMD = types::MQCMD(157);
        pub const MQCMD_DELETE_PAGE_SET: types::MQCMD = types::MQCMD(158);
        pub const MQCMD_CHANGE_BUFFER_POOL: types::MQCMD = types::MQCMD(159);
        pub const MQCMD_CHANGE_PAGE_SET: types::MQCMD = types::MQCMD(160);
        pub const MQCMD_INQUIRE_Q_MGR_STATUS: types::MQCMD = types::MQCMD(161);
        pub const MQCMD_CREATE_LOG: types::MQCMD = types::MQCMD(162);
        pub const MQCMD_STATISTICS_MQI: types::MQCMD = types::MQCMD(164);
        pub const MQCMD_STATISTICS_Q: types::MQCMD = types::MQCMD(165);
        pub const MQCMD_STATISTICS_CHANNEL: types::MQCMD = types::MQCMD(166);
        pub const MQCMD_ACCOUNTING_MQI: types::MQCMD = types::MQCMD(167);
        pub const MQCMD_ACCOUNTING_Q: types::MQCMD = types::MQCMD(168);
        pub const MQCMD_INQUIRE_AUTH_SERVICE: types::MQCMD = types::MQCMD(169);
        pub const MQCMD_CHANGE_TOPIC: types::MQCMD = types::MQCMD(170);
        pub const MQCMD_COPY_TOPIC: types::MQCMD = types::MQCMD(171);
        pub const MQCMD_CREATE_TOPIC: types::MQCMD = types::MQCMD(172);
        pub const MQCMD_DELETE_TOPIC: types::MQCMD = types::MQCMD(173);
        pub const MQCMD_INQUIRE_TOPIC: types::MQCMD = types::MQCMD(174);
        pub const MQCMD_INQUIRE_TOPIC_NAMES: types::MQCMD = types::MQCMD(175);
        pub const MQCMD_INQUIRE_SUBSCRIPTION: types::MQCMD = types::MQCMD(176);
        pub const MQCMD_CREATE_SUBSCRIPTION: types::MQCMD = types::MQCMD(177);
        pub const MQCMD_CHANGE_SUBSCRIPTION: types::MQCMD = types::MQCMD(178);
        pub const MQCMD_DELETE_SUBSCRIPTION: types::MQCMD = types::MQCMD(179);
        pub const MQCMD_COPY_SUBSCRIPTION: types::MQCMD = types::MQCMD(181);
        pub const MQCMD_INQUIRE_SUB_STATUS: types::MQCMD = types::MQCMD(182);
        pub const MQCMD_INQUIRE_TOPIC_STATUS: types::MQCMD = types::MQCMD(183);
        pub const MQCMD_CLEAR_TOPIC_STRING: types::MQCMD = types::MQCMD(184);
        pub const MQCMD_INQUIRE_PUBSUB_STATUS: types::MQCMD = types::MQCMD(185);
        pub const MQCMD_INQUIRE_SMDS: types::MQCMD = types::MQCMD(186);
        pub const MQCMD_CHANGE_SMDS: types::MQCMD = types::MQCMD(187);
        pub const MQCMD_RESET_SMDS: types::MQCMD = types::MQCMD(188);
        pub const MQCMD_CREATE_COMM_INFO: types::MQCMD = types::MQCMD(190);
        pub const MQCMD_INQUIRE_COMM_INFO: types::MQCMD = types::MQCMD(191);
        pub const MQCMD_CHANGE_COMM_INFO: types::MQCMD = types::MQCMD(192);
        pub const MQCMD_COPY_COMM_INFO: types::MQCMD = types::MQCMD(193);
        pub const MQCMD_DELETE_COMM_INFO: types::MQCMD = types::MQCMD(194);
        pub const MQCMD_PURGE_CHANNEL: types::MQCMD = types::MQCMD(195);
        pub const MQCMD_MQXR_DIAGNOSTICS: types::MQCMD = types::MQCMD(196);
        pub const MQCMD_START_SMDSCONN: types::MQCMD = types::MQCMD(197);
        pub const MQCMD_STOP_SMDSCONN: types::MQCMD = types::MQCMD(198);
        pub const MQCMD_INQUIRE_SMDSCONN: types::MQCMD = types::MQCMD(199);
        pub const MQCMD_INQUIRE_MQXR_STATUS: types::MQCMD = types::MQCMD(200);
        pub const MQCMD_START_CLIENT_TRACE: types::MQCMD = types::MQCMD(201);
        pub const MQCMD_STOP_CLIENT_TRACE: types::MQCMD = types::MQCMD(202);
        pub const MQCMD_SET_CHLAUTH_REC: types::MQCMD = types::MQCMD(203);
        pub const MQCMD_INQUIRE_CHLAUTH_RECS: types::MQCMD = types::MQCMD(204);
        pub const MQCMD_INQUIRE_PROT_POLICY: types::MQCMD = types::MQCMD(205);
        pub const MQCMD_CREATE_PROT_POLICY: types::MQCMD = types::MQCMD(206);
        pub const MQCMD_DELETE_PROT_POLICY: types::MQCMD = types::MQCMD(207);
        pub const MQCMD_CHANGE_PROT_POLICY: types::MQCMD = types::MQCMD(208);
        pub const MQCMD_ACTIVITY_TRACE: types::MQCMD = types::MQCMD(209);
        pub const MQCMD_RESET_CF_STRUC: types::MQCMD = types::MQCMD(213);
        pub const MQCMD_INQUIRE_XR_CAPABILITY: types::MQCMD = types::MQCMD(214);
        pub const MQCMD_INQUIRE_AMQP_CAPABILITY: types::MQCMD = types::MQCMD(216);
        pub const MQCMD_AMQP_DIAGNOSTICS: types::MQCMD = types::MQCMD(217);
        pub const MQCMD_INTER_Q_MGR_STATUS: types::MQCMD = types::MQCMD(218);
        pub const MQCMD_INTER_Q_MGR_BALANCE: types::MQCMD = types::MQCMD(219);
        pub const MQCMD_INQUIRE_APPL_STATUS: types::MQCMD = types::MQCMD(220);
        pub const MQCMD_SET_PROT_POLICY: types::MQCMD = types::MQCMD(208);
        pub const MQDELO_NONE: types::MQDELO = types::MQDELO(0);
        pub const MQDELO_LOCAL: types::MQDELO = types::MQDELO(4);
        pub const MQDISCONNECT_NORMAL: types::MQDISCONNECT = types::MQDISCONNECT(0);
        pub const MQDISCONNECT_IMPLICIT: types::MQDISCONNECT = types::MQDISCONNECT(1);
        pub const MQDISCONNECT_Q_MGR: types::MQDISCONNECT = types::MQDISCONNECT(2);
        pub const MQDOPT_RESOLVED: types::MQDOPT = types::MQDOPT(0);
        pub const MQDOPT_DEFINED: types::MQDOPT = types::MQDOPT(1);
        pub const MQEPH_NONE: types::MQEPH = types::MQEPH(0);
        pub const MQEPH_CCSID_EMBEDDED: types::MQEPH = types::MQEPH(1);
        pub const MQET_MQSC: types::MQET = types::MQET(1);
        pub const MQEVO_OTHER: types::MQEVO = types::MQEVO(0);
        pub const MQEVO_CONSOLE: types::MQEVO = types::MQEVO(1);
        pub const MQEVO_INIT: types::MQEVO = types::MQEVO(2);
        pub const MQEVO_MSG: types::MQEVO = types::MQEVO(3);
        pub const MQEVO_MQSET: types::MQEVO = types::MQEVO(4);
        pub const MQEVO_INTERNAL: types::MQEVO = types::MQEVO(5);
        pub const MQEVO_MQSUB: types::MQEVO = types::MQEVO(6);
        pub const MQEVO_CTLMSG: types::MQEVO = types::MQEVO(7);
        pub const MQEVO_REST: types::MQEVO = types::MQEVO(8);
        pub const MQEVR_DISABLED: types::MQEVR = types::MQEVR(0);
        pub const MQEVR_ENABLED: types::MQEVR = types::MQEVR(1);
        pub const MQEVR_EXCEPTION: types::MQEVR = types::MQEVR(2);
        pub const MQEVR_NO_DISPLAY: types::MQEVR = types::MQEVR(3);
        pub const MQEVR_API_ONLY: types::MQEVR = types::MQEVR(4);
        pub const MQEVR_ADMIN_ONLY: types::MQEVR = types::MQEVR(5);
        pub const MQEVR_USER_ONLY: types::MQEVR = types::MQEVR(6);
        pub const MQEXTATTRS_ALL: types::MQEXTATTRS = types::MQEXTATTRS(0);
        pub const MQEXTATTRS_NONDEF: types::MQEXTATTRS = types::MQEXTATTRS(1);
        pub const MQEXT_ALL: types::MQEXT = types::MQEXT(0);
        pub const MQEXT_OBJECT: types::MQEXT = types::MQEXT(1);
        pub const MQEXT_AUTHORITY: types::MQEXT = types::MQEXT(2);
        pub const MQFC_NO: types::MQFC = types::MQFC(0);
        pub const MQFC_YES: types::MQFC = types::MQFC(1);
        pub const MQFSENC_NO: types::MQFSENC = types::MQFSENC(0);
        pub const MQFSENC_YES: types::MQFSENC = types::MQFSENC(1);
        pub const MQFSENC_UNKNOWN: types::MQFSENC = types::MQFSENC(2);
        pub const MQFS_SHARED: types::MQFS = types::MQFS(-1);
        pub const MQGACF_COMMAND_CONTEXT: types::MQGACF = types::MQGACF(8001);
        pub const MQGACF_COMMAND_DATA: types::MQGACF = types::MQGACF(8002);
        pub const MQGACF_TRACE_ROUTE: types::MQGACF = types::MQGACF(8003);
        pub const MQGACF_OPERATION: types::MQGACF = types::MQGACF(8004);
        pub const MQGACF_ACTIVITY: types::MQGACF = types::MQGACF(8005);
        pub const MQGACF_EMBEDDED_MQMD: types::MQGACF = types::MQGACF(8006);
        pub const MQGACF_MESSAGE: types::MQGACF = types::MQGACF(8007);
        pub const MQGACF_MQMD: types::MQGACF = types::MQGACF(8008);
        pub const MQGACF_VALUE_NAMING: types::MQGACF = types::MQGACF(8009);
        pub const MQGACF_Q_ACCOUNTING_DATA: types::MQGACF = types::MQGACF(8010);
        pub const MQGACF_Q_STATISTICS_DATA: types::MQGACF = types::MQGACF(8011);
        pub const MQGACF_CHL_STATISTICS_DATA: types::MQGACF = types::MQGACF(8012);
        pub const MQGACF_ACTIVITY_TRACE: types::MQGACF = types::MQGACF(8013);
        pub const MQGACF_APP_DIST_LIST: types::MQGACF = types::MQGACF(8014);
        pub const MQGACF_MONITOR_CLASS: types::MQGACF = types::MQGACF(8015);
        pub const MQGACF_MONITOR_TYPE: types::MQGACF = types::MQGACF(8016);
        pub const MQGACF_MONITOR_ELEMENT: types::MQGACF = types::MQGACF(8017);
        pub const MQGACF_APPL_STATUS: types::MQGACF = types::MQGACF(8018);
        pub const MQGACF_CHANGED_APPLS: types::MQGACF = types::MQGACF(8019);
        pub const MQGACF_ALL_APPLS: types::MQGACF = types::MQGACF(8020);
        pub const MQGACF_APPL_BALANCE: types::MQGACF = types::MQGACF(8021);
        pub const MQGACF_FIRST: types::MQGACF = types::MQGACF(8001);
        pub const MQGACF_LAST_USED: types::MQGACF = types::MQGACF(8021);
        pub const MQGUR_DISABLED: types::MQGUR = types::MQGUR(0);
        pub const MQGUR_ENABLED: types::MQGUR = types::MQGUR(1);
        pub const MQHSTATE_INACTIVE: types::MQHSTATE = types::MQHSTATE(0);
        pub const MQHSTATE_ACTIVE: types::MQHSTATE = types::MQHSTATE(1);
        pub const MQIACF_Q_MGR_ATTRS: types::MQIACF = types::MQIACF(1001);
        pub const MQIACF_Q_ATTRS: types::MQIACF = types::MQIACF(1002);
        pub const MQIACF_PROCESS_ATTRS: types::MQIACF = types::MQIACF(1003);
        pub const MQIACF_NAMELIST_ATTRS: types::MQIACF = types::MQIACF(1004);
        pub const MQIACF_FORCE: types::MQIACF = types::MQIACF(1005);
        pub const MQIACF_REPLACE: types::MQIACF = types::MQIACF(1006);
        pub const MQIACF_PURGE: types::MQIACF = types::MQIACF(1007);
        pub const MQIACF_QUIESCE: types::MQIACF = types::MQIACF(1008);
        pub const MQIACF_ALL: types::MQIACF = types::MQIACF(1009);
        pub const MQIACF_EVENT_APPL_TYPE: types::MQIACF = types::MQIACF(1010);
        pub const MQIACF_EVENT_ORIGIN: types::MQIACF = types::MQIACF(1011);
        pub const MQIACF_PARAMETER_ID: types::MQIACF = types::MQIACF(1012);
        pub const MQIACF_ERROR_ID: types::MQIACF = types::MQIACF(1013);
        pub const MQIACF_SELECTOR: types::MQIACF = types::MQIACF(1014);
        pub const MQIACF_CHANNEL_ATTRS: types::MQIACF = types::MQIACF(1015);
        pub const MQIACF_OBJECT_TYPE: types::MQIACF = types::MQIACF(1016);
        pub const MQIACF_ESCAPE_TYPE: types::MQIACF = types::MQIACF(1017);
        pub const MQIACF_ERROR_OFFSET: types::MQIACF = types::MQIACF(1018);
        pub const MQIACF_AUTH_INFO_ATTRS: types::MQIACF = types::MQIACF(1019);
        pub const MQIACF_REASON_QUALIFIER: types::MQIACF = types::MQIACF(1020);
        pub const MQIACF_COMMAND: types::MQIACF = types::MQIACF(1021);
        pub const MQIACF_OPEN_OPTIONS: types::MQIACF = types::MQIACF(1022);
        pub const MQIACF_OPEN_TYPE: types::MQIACF = types::MQIACF(1023);
        pub const MQIACF_PROCESS_ID: types::MQIACF = types::MQIACF(1024);
        pub const MQIACF_THREAD_ID: types::MQIACF = types::MQIACF(1025);
        pub const MQIACF_Q_STATUS_ATTRS: types::MQIACF = types::MQIACF(1026);
        pub const MQIACF_UNCOMMITTED_MSGS: types::MQIACF = types::MQIACF(1027);
        pub const MQIACF_HANDLE_STATE: types::MQIACF = types::MQIACF(1028);
        pub const MQIACF_AUX_ERROR_DATA_INT_1: types::MQIACF = types::MQIACF(1070);
        pub const MQIACF_AUX_ERROR_DATA_INT_2: types::MQIACF = types::MQIACF(1071);
        pub const MQIACF_CONV_REASON_CODE: types::MQIACF = types::MQIACF(1072);
        pub const MQIACF_BRIDGE_TYPE: types::MQIACF = types::MQIACF(1073);
        pub const MQIACF_INQUIRY: types::MQIACF = types::MQIACF(1074);
        pub const MQIACF_WAIT_INTERVAL: types::MQIACF = types::MQIACF(1075);
        pub const MQIACF_OPTIONS: types::MQIACF = types::MQIACF(1076);
        pub const MQIACF_BROKER_OPTIONS: types::MQIACF = types::MQIACF(1077);
        pub const MQIACF_REFRESH_TYPE: types::MQIACF = types::MQIACF(1078);
        pub const MQIACF_SEQUENCE_NUMBER: types::MQIACF = types::MQIACF(1079);
        pub const MQIACF_INTEGER_DATA: types::MQIACF = types::MQIACF(1080);
        pub const MQIACF_REGISTRATION_OPTIONS: types::MQIACF = types::MQIACF(1081);
        pub const MQIACF_PUBLICATION_OPTIONS: types::MQIACF = types::MQIACF(1082);
        pub const MQIACF_CLUSTER_INFO: types::MQIACF = types::MQIACF(1083);
        pub const MQIACF_Q_MGR_DEFINITION_TYPE: types::MQIACF = types::MQIACF(1084);
        pub const MQIACF_Q_MGR_TYPE: types::MQIACF = types::MQIACF(1085);
        pub const MQIACF_ACTION: types::MQIACF = types::MQIACF(1086);
        pub const MQIACF_SUSPEND: types::MQIACF = types::MQIACF(1087);
        pub const MQIACF_BROKER_COUNT: types::MQIACF = types::MQIACF(1088);
        pub const MQIACF_APPL_COUNT: types::MQIACF = types::MQIACF(1089);
        pub const MQIACF_ANONYMOUS_COUNT: types::MQIACF = types::MQIACF(1090);
        pub const MQIACF_REG_REG_OPTIONS: types::MQIACF = types::MQIACF(1091);
        pub const MQIACF_DELETE_OPTIONS: types::MQIACF = types::MQIACF(1092);
        pub const MQIACF_CLUSTER_Q_MGR_ATTRS: types::MQIACF = types::MQIACF(1093);
        pub const MQIACF_REFRESH_INTERVAL: types::MQIACF = types::MQIACF(1094);
        pub const MQIACF_REFRESH_REPOSITORY: types::MQIACF = types::MQIACF(1095);
        pub const MQIACF_REMOVE_QUEUES: types::MQIACF = types::MQIACF(1096);
        pub const MQIACF_OPEN_INPUT_TYPE: types::MQIACF = types::MQIACF(1098);
        pub const MQIACF_OPEN_OUTPUT: types::MQIACF = types::MQIACF(1099);
        pub const MQIACF_OPEN_SET: types::MQIACF = types::MQIACF(1100);
        pub const MQIACF_OPEN_INQUIRE: types::MQIACF = types::MQIACF(1101);
        pub const MQIACF_OPEN_BROWSE: types::MQIACF = types::MQIACF(1102);
        pub const MQIACF_Q_STATUS_TYPE: types::MQIACF = types::MQIACF(1103);
        pub const MQIACF_Q_HANDLE: types::MQIACF = types::MQIACF(1104);
        pub const MQIACF_Q_STATUS: types::MQIACF = types::MQIACF(1105);
        pub const MQIACF_SECURITY_TYPE: types::MQIACF = types::MQIACF(1106);
        pub const MQIACF_CONNECTION_ATTRS: types::MQIACF = types::MQIACF(1107);
        pub const MQIACF_CONNECT_OPTIONS: types::MQIACF = types::MQIACF(1108);
        pub const MQIACF_CONN_INFO_TYPE: types::MQIACF = types::MQIACF(1110);
        pub const MQIACF_CONN_INFO_CONN: types::MQIACF = types::MQIACF(1111);
        pub const MQIACF_CONN_INFO_HANDLE: types::MQIACF = types::MQIACF(1112);
        pub const MQIACF_CONN_INFO_ALL: types::MQIACF = types::MQIACF(1113);
        pub const MQIACF_AUTH_PROFILE_ATTRS: types::MQIACF = types::MQIACF(1114);
        pub const MQIACF_AUTHORIZATION_LIST: types::MQIACF = types::MQIACF(1115);
        pub const MQIACF_AUTH_ADD_AUTHS: types::MQIACF = types::MQIACF(1116);
        pub const MQIACF_AUTH_REMOVE_AUTHS: types::MQIACF = types::MQIACF(1117);
        pub const MQIACF_ENTITY_TYPE: types::MQIACF = types::MQIACF(1118);
        pub const MQIACF_COMMAND_INFO: types::MQIACF = types::MQIACF(1120);
        pub const MQIACF_CMDSCOPE_Q_MGR_COUNT: types::MQIACF = types::MQIACF(1121);
        pub const MQIACF_Q_MGR_SYSTEM: types::MQIACF = types::MQIACF(1122);
        pub const MQIACF_Q_MGR_EVENT: types::MQIACF = types::MQIACF(1123);
        pub const MQIACF_Q_MGR_DQM: types::MQIACF = types::MQIACF(1124);
        pub const MQIACF_Q_MGR_CLUSTER: types::MQIACF = types::MQIACF(1125);
        pub const MQIACF_QSG_DISPS: types::MQIACF = types::MQIACF(1126);
        pub const MQIACF_UOW_STATE: types::MQIACF = types::MQIACF(1128);
        pub const MQIACF_SECURITY_ITEM: types::MQIACF = types::MQIACF(1129);
        pub const MQIACF_CF_STRUC_STATUS: types::MQIACF = types::MQIACF(1130);
        pub const MQIACF_UOW_TYPE: types::MQIACF = types::MQIACF(1132);
        pub const MQIACF_CF_STRUC_ATTRS: types::MQIACF = types::MQIACF(1133);
        pub const MQIACF_EXCLUDE_INTERVAL: types::MQIACF = types::MQIACF(1134);
        pub const MQIACF_CF_STATUS_TYPE: types::MQIACF = types::MQIACF(1135);
        pub const MQIACF_CF_STATUS_SUMMARY: types::MQIACF = types::MQIACF(1136);
        pub const MQIACF_CF_STATUS_CONNECT: types::MQIACF = types::MQIACF(1137);
        pub const MQIACF_CF_STATUS_BACKUP: types::MQIACF = types::MQIACF(1138);
        pub const MQIACF_CF_STRUC_TYPE: types::MQIACF = types::MQIACF(1139);
        pub const MQIACF_CF_STRUC_SIZE_MAX: types::MQIACF = types::MQIACF(1140);
        pub const MQIACF_CF_STRUC_SIZE_USED: types::MQIACF = types::MQIACF(1141);
        pub const MQIACF_CF_STRUC_ENTRIES_MAX: types::MQIACF = types::MQIACF(1142);
        pub const MQIACF_CF_STRUC_ENTRIES_USED: types::MQIACF = types::MQIACF(1143);
        pub const MQIACF_CF_STRUC_BACKUP_SIZE: types::MQIACF = types::MQIACF(1144);
        pub const MQIACF_MOVE_TYPE: types::MQIACF = types::MQIACF(1145);
        pub const MQIACF_MOVE_TYPE_MOVE: types::MQIACF = types::MQIACF(1146);
        pub const MQIACF_MOVE_TYPE_ADD: types::MQIACF = types::MQIACF(1147);
        pub const MQIACF_Q_MGR_NUMBER: types::MQIACF = types::MQIACF(1148);
        pub const MQIACF_Q_MGR_STATUS: types::MQIACF = types::MQIACF(1149);
        pub const MQIACF_DB2_CONN_STATUS: types::MQIACF = types::MQIACF(1150);
        pub const MQIACF_SECURITY_ATTRS: types::MQIACF = types::MQIACF(1151);
        pub const MQIACF_SECURITY_TIMEOUT: types::MQIACF = types::MQIACF(1152);
        pub const MQIACF_SECURITY_INTERVAL: types::MQIACF = types::MQIACF(1153);
        pub const MQIACF_SECURITY_SWITCH: types::MQIACF = types::MQIACF(1154);
        pub const MQIACF_SECURITY_SETTING: types::MQIACF = types::MQIACF(1155);
        pub const MQIACF_STORAGE_CLASS_ATTRS: types::MQIACF = types::MQIACF(1156);
        pub const MQIACF_USAGE_TYPE: types::MQIACF = types::MQIACF(1157);
        pub const MQIACF_BUFFER_POOL_ID: types::MQIACF = types::MQIACF(1158);
        pub const MQIACF_USAGE_TOTAL_PAGES: types::MQIACF = types::MQIACF(1159);
        pub const MQIACF_USAGE_UNUSED_PAGES: types::MQIACF = types::MQIACF(1160);
        pub const MQIACF_USAGE_PERSIST_PAGES: types::MQIACF = types::MQIACF(1161);
        pub const MQIACF_USAGE_NONPERSIST_PAGES: types::MQIACF = types::MQIACF(1162);
        pub const MQIACF_USAGE_RESTART_EXTENTS: types::MQIACF = types::MQIACF(1163);
        pub const MQIACF_USAGE_EXPAND_COUNT: types::MQIACF = types::MQIACF(1164);
        pub const MQIACF_PAGESET_STATUS: types::MQIACF = types::MQIACF(1165);
        pub const MQIACF_USAGE_TOTAL_BUFFERS: types::MQIACF = types::MQIACF(1166);
        pub const MQIACF_USAGE_DATA_SET_TYPE: types::MQIACF = types::MQIACF(1167);
        pub const MQIACF_USAGE_PAGESET: types::MQIACF = types::MQIACF(1168);
        pub const MQIACF_USAGE_DATA_SET: types::MQIACF = types::MQIACF(1169);
        pub const MQIACF_USAGE_BUFFER_POOL: types::MQIACF = types::MQIACF(1170);
        pub const MQIACF_MOVE_COUNT: types::MQIACF = types::MQIACF(1171);
        pub const MQIACF_EXPIRY_Q_COUNT: types::MQIACF = types::MQIACF(1172);
        pub const MQIACF_CONFIGURATION_OBJECTS: types::MQIACF = types::MQIACF(1173);
        pub const MQIACF_CONFIGURATION_EVENTS: types::MQIACF = types::MQIACF(1174);
        pub const MQIACF_SYSP_TYPE: types::MQIACF = types::MQIACF(1175);
        pub const MQIACF_SYSP_DEALLOC_INTERVAL: types::MQIACF = types::MQIACF(1176);
        pub const MQIACF_SYSP_MAX_ARCHIVE: types::MQIACF = types::MQIACF(1177);
        pub const MQIACF_SYSP_MAX_READ_TAPES: types::MQIACF = types::MQIACF(1178);
        pub const MQIACF_SYSP_IN_BUFFER_SIZE: types::MQIACF = types::MQIACF(1179);
        pub const MQIACF_SYSP_OUT_BUFFER_SIZE: types::MQIACF = types::MQIACF(1180);
        pub const MQIACF_SYSP_OUT_BUFFER_COUNT: types::MQIACF = types::MQIACF(1181);
        pub const MQIACF_SYSP_ARCHIVE: types::MQIACF = types::MQIACF(1182);
        pub const MQIACF_SYSP_DUAL_ACTIVE: types::MQIACF = types::MQIACF(1183);
        pub const MQIACF_SYSP_DUAL_ARCHIVE: types::MQIACF = types::MQIACF(1184);
        pub const MQIACF_SYSP_DUAL_BSDS: types::MQIACF = types::MQIACF(1185);
        pub const MQIACF_SYSP_MAX_CONNS: types::MQIACF = types::MQIACF(1186);
        pub const MQIACF_SYSP_MAX_CONNS_FORE: types::MQIACF = types::MQIACF(1187);
        pub const MQIACF_SYSP_MAX_CONNS_BACK: types::MQIACF = types::MQIACF(1188);
        pub const MQIACF_SYSP_EXIT_INTERVAL: types::MQIACF = types::MQIACF(1189);
        pub const MQIACF_SYSP_EXIT_TASKS: types::MQIACF = types::MQIACF(1190);
        pub const MQIACF_SYSP_CHKPOINT_COUNT: types::MQIACF = types::MQIACF(1191);
        pub const MQIACF_SYSP_OTMA_INTERVAL: types::MQIACF = types::MQIACF(1192);
        pub const MQIACF_SYSP_Q_INDEX_DEFER: types::MQIACF = types::MQIACF(1193);
        pub const MQIACF_SYSP_DB2_TASKS: types::MQIACF = types::MQIACF(1194);
        pub const MQIACF_SYSP_RESLEVEL_AUDIT: types::MQIACF = types::MQIACF(1195);
        pub const MQIACF_SYSP_ROUTING_CODE: types::MQIACF = types::MQIACF(1196);
        pub const MQIACF_SYSP_SMF_ACCOUNTING: types::MQIACF = types::MQIACF(1197);
        pub const MQIACF_SYSP_SMF_STATS: types::MQIACF = types::MQIACF(1198);
        pub const MQIACF_SYSP_SMF_INTERVAL: types::MQIACF = types::MQIACF(1199);
        pub const MQIACF_SYSP_TRACE_CLASS: types::MQIACF = types::MQIACF(1200);
        pub const MQIACF_SYSP_TRACE_SIZE: types::MQIACF = types::MQIACF(1201);
        pub const MQIACF_SYSP_WLM_INTERVAL: types::MQIACF = types::MQIACF(1202);
        pub const MQIACF_SYSP_ALLOC_UNIT: types::MQIACF = types::MQIACF(1203);
        pub const MQIACF_SYSP_ARCHIVE_RETAIN: types::MQIACF = types::MQIACF(1204);
        pub const MQIACF_SYSP_ARCHIVE_WTOR: types::MQIACF = types::MQIACF(1205);
        pub const MQIACF_SYSP_BLOCK_SIZE: types::MQIACF = types::MQIACF(1206);
        pub const MQIACF_SYSP_CATALOG: types::MQIACF = types::MQIACF(1207);
        pub const MQIACF_SYSP_COMPACT: types::MQIACF = types::MQIACF(1208);
        pub const MQIACF_SYSP_ALLOC_PRIMARY: types::MQIACF = types::MQIACF(1209);
        pub const MQIACF_SYSP_ALLOC_SECONDARY: types::MQIACF = types::MQIACF(1210);
        pub const MQIACF_SYSP_PROTECT: types::MQIACF = types::MQIACF(1211);
        pub const MQIACF_SYSP_QUIESCE_INTERVAL: types::MQIACF = types::MQIACF(1212);
        pub const MQIACF_SYSP_TIMESTAMP: types::MQIACF = types::MQIACF(1213);
        pub const MQIACF_SYSP_UNIT_ADDRESS: types::MQIACF = types::MQIACF(1214);
        pub const MQIACF_SYSP_UNIT_STATUS: types::MQIACF = types::MQIACF(1215);
        pub const MQIACF_SYSP_LOG_COPY: types::MQIACF = types::MQIACF(1216);
        pub const MQIACF_SYSP_LOG_USED: types::MQIACF = types::MQIACF(1217);
        pub const MQIACF_SYSP_LOG_SUSPEND: types::MQIACF = types::MQIACF(1218);
        pub const MQIACF_SYSP_OFFLOAD_STATUS: types::MQIACF = types::MQIACF(1219);
        pub const MQIACF_SYSP_TOTAL_LOGS: types::MQIACF = types::MQIACF(1220);
        pub const MQIACF_SYSP_FULL_LOGS: types::MQIACF = types::MQIACF(1221);
        pub const MQIACF_LISTENER_ATTRS: types::MQIACF = types::MQIACF(1222);
        pub const MQIACF_LISTENER_STATUS_ATTRS: types::MQIACF = types::MQIACF(1223);
        pub const MQIACF_SERVICE_ATTRS: types::MQIACF = types::MQIACF(1224);
        pub const MQIACF_SERVICE_STATUS_ATTRS: types::MQIACF = types::MQIACF(1225);
        pub const MQIACF_Q_TIME_INDICATOR: types::MQIACF = types::MQIACF(1226);
        pub const MQIACF_OLDEST_MSG_AGE: types::MQIACF = types::MQIACF(1227);
        pub const MQIACF_AUTH_OPTIONS: types::MQIACF = types::MQIACF(1228);
        pub const MQIACF_Q_MGR_STATUS_ATTRS: types::MQIACF = types::MQIACF(1229);
        pub const MQIACF_CONNECTION_COUNT: types::MQIACF = types::MQIACF(1230);
        pub const MQIACF_Q_MGR_FACILITY: types::MQIACF = types::MQIACF(1231);
        pub const MQIACF_CHINIT_STATUS: types::MQIACF = types::MQIACF(1232);
        pub const MQIACF_CMD_SERVER_STATUS: types::MQIACF = types::MQIACF(1233);
        pub const MQIACF_ROUTE_DETAIL: types::MQIACF = types::MQIACF(1234);
        pub const MQIACF_RECORDED_ACTIVITIES: types::MQIACF = types::MQIACF(1235);
        pub const MQIACF_MAX_ACTIVITIES: types::MQIACF = types::MQIACF(1236);
        pub const MQIACF_DISCONTINUITY_COUNT: types::MQIACF = types::MQIACF(1237);
        pub const MQIACF_ROUTE_ACCUMULATION: types::MQIACF = types::MQIACF(1238);
        pub const MQIACF_ROUTE_DELIVERY: types::MQIACF = types::MQIACF(1239);
        pub const MQIACF_OPERATION_TYPE: types::MQIACF = types::MQIACF(1240);
        pub const MQIACF_BACKOUT_COUNT: types::MQIACF = types::MQIACF(1241);
        pub const MQIACF_COMP_CODE: types::MQIACF = types::MQIACF(1242);
        pub const MQIACF_ENCODING: types::MQIACF = types::MQIACF(1243);
        pub const MQIACF_EXPIRY: types::MQIACF = types::MQIACF(1244);
        pub const MQIACF_FEEDBACK: types::MQIACF = types::MQIACF(1245);
        pub const MQIACF_MSG_FLAGS: types::MQIACF = types::MQIACF(1247);
        pub const MQIACF_MSG_LENGTH: types::MQIACF = types::MQIACF(1248);
        pub const MQIACF_MSG_TYPE: types::MQIACF = types::MQIACF(1249);
        pub const MQIACF_OFFSET: types::MQIACF = types::MQIACF(1250);
        pub const MQIACF_ORIGINAL_LENGTH: types::MQIACF = types::MQIACF(1251);
        pub const MQIACF_PERSISTENCE: types::MQIACF = types::MQIACF(1252);
        pub const MQIACF_PRIORITY: types::MQIACF = types::MQIACF(1253);
        pub const MQIACF_REASON_CODE: types::MQIACF = types::MQIACF(1254);
        pub const MQIACF_REPORT: types::MQIACF = types::MQIACF(1255);
        pub const MQIACF_VERSION: types::MQIACF = types::MQIACF(1256);
        pub const MQIACF_UNRECORDED_ACTIVITIES: types::MQIACF = types::MQIACF(1257);
        pub const MQIACF_MONITORING: types::MQIACF = types::MQIACF(1258);
        pub const MQIACF_ROUTE_FORWARDING: types::MQIACF = types::MQIACF(1259);
        pub const MQIACF_SERVICE_STATUS: types::MQIACF = types::MQIACF(1260);
        pub const MQIACF_Q_TYPES: types::MQIACF = types::MQIACF(1261);
        pub const MQIACF_USER_ID_SUPPORT: types::MQIACF = types::MQIACF(1262);
        pub const MQIACF_INTERFACE_VERSION: types::MQIACF = types::MQIACF(1263);
        pub const MQIACF_AUTH_SERVICE_ATTRS: types::MQIACF = types::MQIACF(1264);
        pub const MQIACF_USAGE_EXPAND_TYPE: types::MQIACF = types::MQIACF(1265);
        pub const MQIACF_SYSP_CLUSTER_CACHE: types::MQIACF = types::MQIACF(1266);
        pub const MQIACF_SYSP_DB2_BLOB_TASKS: types::MQIACF = types::MQIACF(1267);
        pub const MQIACF_SYSP_WLM_INT_UNITS: types::MQIACF = types::MQIACF(1268);
        pub const MQIACF_TOPIC_ATTRS: types::MQIACF = types::MQIACF(1269);
        pub const MQIACF_PUBSUB_PROPERTIES: types::MQIACF = types::MQIACF(1271);
        pub const MQIACF_DESTINATION_CLASS: types::MQIACF = types::MQIACF(1273);
        pub const MQIACF_DURABLE_SUBSCRIPTION: types::MQIACF = types::MQIACF(1274);
        pub const MQIACF_SUBSCRIPTION_SCOPE: types::MQIACF = types::MQIACF(1275);
        pub const MQIACF_VARIABLE_USER_ID: types::MQIACF = types::MQIACF(1277);
        pub const MQIACF_REQUEST_ONLY: types::MQIACF = types::MQIACF(1280);
        pub const MQIACF_PUB_PRIORITY: types::MQIACF = types::MQIACF(1283);
        pub const MQIACF_SUB_ATTRS: types::MQIACF = types::MQIACF(1287);
        pub const MQIACF_WILDCARD_SCHEMA: types::MQIACF = types::MQIACF(1288);
        pub const MQIACF_SUB_TYPE: types::MQIACF = types::MQIACF(1289);
        pub const MQIACF_MESSAGE_COUNT: types::MQIACF = types::MQIACF(1290);
        pub const MQIACF_Q_MGR_PUBSUB: types::MQIACF = types::MQIACF(1291);
        pub const MQIACF_Q_MGR_VERSION: types::MQIACF = types::MQIACF(1292);
        pub const MQIACF_SUB_STATUS_ATTRS: types::MQIACF = types::MQIACF(1294);
        pub const MQIACF_TOPIC_STATUS: types::MQIACF = types::MQIACF(1295);
        pub const MQIACF_TOPIC_SUB: types::MQIACF = types::MQIACF(1296);
        pub const MQIACF_TOPIC_PUB: types::MQIACF = types::MQIACF(1297);
        pub const MQIACF_RETAINED_PUBLICATION: types::MQIACF = types::MQIACF(1300);
        pub const MQIACF_TOPIC_STATUS_ATTRS: types::MQIACF = types::MQIACF(1301);
        pub const MQIACF_TOPIC_STATUS_TYPE: types::MQIACF = types::MQIACF(1302);
        pub const MQIACF_SUB_OPTIONS: types::MQIACF = types::MQIACF(1303);
        pub const MQIACF_PUBLISH_COUNT: types::MQIACF = types::MQIACF(1304);
        pub const MQIACF_CLEAR_TYPE: types::MQIACF = types::MQIACF(1305);
        pub const MQIACF_CLEAR_SCOPE: types::MQIACF = types::MQIACF(1306);
        pub const MQIACF_SUB_LEVEL: types::MQIACF = types::MQIACF(1307);
        pub const MQIACF_ASYNC_STATE: types::MQIACF = types::MQIACF(1308);
        pub const MQIACF_SUB_SUMMARY: types::MQIACF = types::MQIACF(1309);
        pub const MQIACF_OBSOLETE_MSGS: types::MQIACF = types::MQIACF(1310);
        pub const MQIACF_PUBSUB_STATUS: types::MQIACF = types::MQIACF(1311);
        pub const MQIACF_PS_STATUS_TYPE: types::MQIACF = types::MQIACF(1314);
        pub const MQIACF_PUBSUB_STATUS_ATTRS: types::MQIACF = types::MQIACF(1318);
        pub const MQIACF_SELECTOR_TYPE: types::MQIACF = types::MQIACF(1321);
        pub const MQIACF_LOG_COMPRESSION: types::MQIACF = types::MQIACF(1322);
        pub const MQIACF_GROUPUR_CHECK_ID: types::MQIACF = types::MQIACF(1323);
        pub const MQIACF_MULC_CAPTURE: types::MQIACF = types::MQIACF(1324);
        pub const MQIACF_PERMIT_STANDBY: types::MQIACF = types::MQIACF(1325);
        pub const MQIACF_OPERATION_MODE: types::MQIACF = types::MQIACF(1326);
        pub const MQIACF_COMM_INFO_ATTRS: types::MQIACF = types::MQIACF(1327);
        pub const MQIACF_CF_SMDS_BLOCK_SIZE: types::MQIACF = types::MQIACF(1328);
        pub const MQIACF_CF_SMDS_EXPAND: types::MQIACF = types::MQIACF(1329);
        pub const MQIACF_USAGE_FREE_BUFF: types::MQIACF = types::MQIACF(1330);
        pub const MQIACF_USAGE_FREE_BUFF_PERC: types::MQIACF = types::MQIACF(1331);
        pub const MQIACF_CF_STRUC_ACCESS: types::MQIACF = types::MQIACF(1332);
        pub const MQIACF_CF_STATUS_SMDS: types::MQIACF = types::MQIACF(1333);
        pub const MQIACF_SMDS_ATTRS: types::MQIACF = types::MQIACF(1334);
        pub const MQIACF_USAGE_SMDS: types::MQIACF = types::MQIACF(1335);
        pub const MQIACF_USAGE_BLOCK_SIZE: types::MQIACF = types::MQIACF(1336);
        pub const MQIACF_USAGE_DATA_BLOCKS: types::MQIACF = types::MQIACF(1337);
        pub const MQIACF_USAGE_EMPTY_BUFFERS: types::MQIACF = types::MQIACF(1338);
        pub const MQIACF_USAGE_INUSE_BUFFERS: types::MQIACF = types::MQIACF(1339);
        pub const MQIACF_USAGE_LOWEST_FREE: types::MQIACF = types::MQIACF(1340);
        pub const MQIACF_USAGE_OFFLOAD_MSGS: types::MQIACF = types::MQIACF(1341);
        pub const MQIACF_USAGE_READS_SAVED: types::MQIACF = types::MQIACF(1342);
        pub const MQIACF_USAGE_SAVED_BUFFERS: types::MQIACF = types::MQIACF(1343);
        pub const MQIACF_USAGE_TOTAL_BLOCKS: types::MQIACF = types::MQIACF(1344);
        pub const MQIACF_USAGE_USED_BLOCKS: types::MQIACF = types::MQIACF(1345);
        pub const MQIACF_USAGE_USED_RATE: types::MQIACF = types::MQIACF(1346);
        pub const MQIACF_USAGE_WAIT_RATE: types::MQIACF = types::MQIACF(1347);
        pub const MQIACF_SMDS_OPENMODE: types::MQIACF = types::MQIACF(1348);
        pub const MQIACF_SMDS_STATUS: types::MQIACF = types::MQIACF(1349);
        pub const MQIACF_SMDS_AVAIL: types::MQIACF = types::MQIACF(1350);
        pub const MQIACF_MCAST_REL_INDICATOR: types::MQIACF = types::MQIACF(1351);
        pub const MQIACF_CHLAUTH_TYPE: types::MQIACF = types::MQIACF(1352);
        pub const MQIACF_MQXR_DIAGNOSTICS_TYPE: types::MQIACF = types::MQIACF(1354);
        pub const MQIACF_CHLAUTH_ATTRS: types::MQIACF = types::MQIACF(1355);
        pub const MQIACF_OPERATION_ID: types::MQIACF = types::MQIACF(1356);
        pub const MQIACF_API_CALLER_TYPE: types::MQIACF = types::MQIACF(1357);
        pub const MQIACF_API_ENVIRONMENT: types::MQIACF = types::MQIACF(1358);
        pub const MQIACF_TRACE_DETAIL: types::MQIACF = types::MQIACF(1359);
        pub const MQIACF_HOBJ: types::MQIACF = types::MQIACF(1360);
        pub const MQIACF_CALL_TYPE: types::MQIACF = types::MQIACF(1361);
        pub const MQIACF_MQCB_OPERATION: types::MQIACF = types::MQIACF(1362);
        pub const MQIACF_MQCB_TYPE: types::MQIACF = types::MQIACF(1363);
        pub const MQIACF_MQCB_OPTIONS: types::MQIACF = types::MQIACF(1364);
        pub const MQIACF_CLOSE_OPTIONS: types::MQIACF = types::MQIACF(1365);
        pub const MQIACF_CTL_OPERATION: types::MQIACF = types::MQIACF(1366);
        pub const MQIACF_GET_OPTIONS: types::MQIACF = types::MQIACF(1367);
        pub const MQIACF_RECS_PRESENT: types::MQIACF = types::MQIACF(1368);
        pub const MQIACF_KNOWN_DEST_COUNT: types::MQIACF = types::MQIACF(1369);
        pub const MQIACF_UNKNOWN_DEST_COUNT: types::MQIACF = types::MQIACF(1370);
        pub const MQIACF_INVALID_DEST_COUNT: types::MQIACF = types::MQIACF(1371);
        pub const MQIACF_RESOLVED_TYPE: types::MQIACF = types::MQIACF(1372);
        pub const MQIACF_PUT_OPTIONS: types::MQIACF = types::MQIACF(1373);
        pub const MQIACF_BUFFER_LENGTH: types::MQIACF = types::MQIACF(1374);
        pub const MQIACF_TRACE_DATA_LENGTH: types::MQIACF = types::MQIACF(1375);
        pub const MQIACF_SMDS_EXPANDST: types::MQIACF = types::MQIACF(1376);
        pub const MQIACF_ITEM_COUNT: types::MQIACF = types::MQIACF(1378);
        pub const MQIACF_EXPIRY_TIME: types::MQIACF = types::MQIACF(1379);
        pub const MQIACF_CONNECT_TIME: types::MQIACF = types::MQIACF(1380);
        pub const MQIACF_DISCONNECT_TIME: types::MQIACF = types::MQIACF(1381);
        pub const MQIACF_HSUB: types::MQIACF = types::MQIACF(1382);
        pub const MQIACF_SUBRQ_OPTIONS: types::MQIACF = types::MQIACF(1383);
        pub const MQIACF_XA_RMID: types::MQIACF = types::MQIACF(1384);
        pub const MQIACF_XA_FLAGS: types::MQIACF = types::MQIACF(1385);
        pub const MQIACF_XA_RETCODE: types::MQIACF = types::MQIACF(1386);
        pub const MQIACF_XA_HANDLE: types::MQIACF = types::MQIACF(1387);
        pub const MQIACF_XA_RETVAL: types::MQIACF = types::MQIACF(1388);
        pub const MQIACF_STATUS_TYPE: types::MQIACF = types::MQIACF(1389);
        pub const MQIACF_XA_COUNT: types::MQIACF = types::MQIACF(1390);
        pub const MQIACF_SELECTOR_COUNT: types::MQIACF = types::MQIACF(1391);
        pub const MQIACF_SELECTORS: types::MQIACF = types::MQIACF(1392);
        pub const MQIACF_INTATTR_COUNT: types::MQIACF = types::MQIACF(1393);
        pub const MQIACF_INT_ATTRS: types::MQIACF = types::MQIACF(1394);
        pub const MQIACF_SUBRQ_ACTION: types::MQIACF = types::MQIACF(1395);
        pub const MQIACF_NUM_PUBS: types::MQIACF = types::MQIACF(1396);
        pub const MQIACF_POINTER_SIZE: types::MQIACF = types::MQIACF(1397);
        pub const MQIACF_REMOVE_AUTHREC: types::MQIACF = types::MQIACF(1398);
        pub const MQIACF_XR_ATTRS: types::MQIACF = types::MQIACF(1399);
        pub const MQIACF_APPL_FUNCTION_TYPE: types::MQIACF = types::MQIACF(1400);
        pub const MQIACF_AMQP_ATTRS: types::MQIACF = types::MQIACF(1401);
        pub const MQIACF_EXPORT_TYPE: types::MQIACF = types::MQIACF(1402);
        pub const MQIACF_EXPORT_ATTRS: types::MQIACF = types::MQIACF(1403);
        pub const MQIACF_SYSTEM_OBJECTS: types::MQIACF = types::MQIACF(1404);
        pub const MQIACF_CONNECTION_SWAP: types::MQIACF = types::MQIACF(1405);
        pub const MQIACF_AMQP_DIAGNOSTICS_TYPE: types::MQIACF = types::MQIACF(1406);
        pub const MQIACF_BUFFER_POOL_LOCATION: types::MQIACF = types::MQIACF(1408);
        pub const MQIACF_LDAP_CONNECTION_STATUS: types::MQIACF = types::MQIACF(1409);
        pub const MQIACF_SYSP_MAX_ACE_POOL: types::MQIACF = types::MQIACF(1410);
        pub const MQIACF_PAGECLAS: types::MQIACF = types::MQIACF(1411);
        pub const MQIACF_AUTH_REC_TYPE: types::MQIACF = types::MQIACF(1412);
        pub const MQIACF_SYSP_MAX_CONC_OFFLOADS: types::MQIACF = types::MQIACF(1413);
        pub const MQIACF_SYSP_ZHYPERWRITE: types::MQIACF = types::MQIACF(1414);
        pub const MQIACF_Q_MGR_STATUS_LOG: types::MQIACF = types::MQIACF(1415);
        pub const MQIACF_ARCHIVE_LOG_SIZE: types::MQIACF = types::MQIACF(1416);
        pub const MQIACF_MEDIA_LOG_SIZE: types::MQIACF = types::MQIACF(1417);
        pub const MQIACF_RESTART_LOG_SIZE: types::MQIACF = types::MQIACF(1418);
        pub const MQIACF_REUSABLE_LOG_SIZE: types::MQIACF = types::MQIACF(1419);
        pub const MQIACF_LOG_IN_USE: types::MQIACF = types::MQIACF(1420);
        pub const MQIACF_LOG_UTILIZATION: types::MQIACF = types::MQIACF(1421);
        pub const MQIACF_LOG_REDUCTION: types::MQIACF = types::MQIACF(1422);
        pub const MQIACF_IGNORE_STATE: types::MQIACF = types::MQIACF(1423);
        pub const MQIACF_MOVABLE_APPL_COUNT: types::MQIACF = types::MQIACF(1424);
        pub const MQIACF_APPL_INFO_ATTRS: types::MQIACF = types::MQIACF(1425);
        pub const MQIACF_APPL_MOVABLE: types::MQIACF = types::MQIACF(1426);
        pub const MQIACF_REMOTE_QMGR_ACTIVE: types::MQIACF = types::MQIACF(1427);
        pub const MQIACF_APPL_INFO_TYPE: types::MQIACF = types::MQIACF(1428);
        pub const MQIACF_APPL_INFO_APPL: types::MQIACF = types::MQIACF(1429);
        pub const MQIACF_APPL_INFO_QMGR: types::MQIACF = types::MQIACF(1430);
        pub const MQIACF_APPL_INFO_LOCAL: types::MQIACF = types::MQIACF(1431);
        pub const MQIACF_APPL_IMMOVABLE_COUNT: types::MQIACF = types::MQIACF(1432);
        pub const MQIACF_BALANCED: types::MQIACF = types::MQIACF(1433);
        pub const MQIACF_BALSTATE: types::MQIACF = types::MQIACF(1434);
        pub const MQIACF_APPL_IMMOVABLE_REASON: types::MQIACF = types::MQIACF(1435);
        pub const MQIACF_DS_ENCRYPTED: types::MQIACF = types::MQIACF(1436);
        pub const MQIACF_CUR_Q_FILE_SIZE: types::MQIACF = types::MQIACF(1437);
        pub const MQIACF_CUR_MAX_FILE_SIZE: types::MQIACF = types::MQIACF(1438);
        pub const MQIACF_BALANCING_TYPE: types::MQIACF = types::MQIACF(1439);
        pub const MQIACF_BALANCING_OPTIONS: types::MQIACF = types::MQIACF(1440);
        pub const MQIACF_BALANCING_TIMEOUT: types::MQIACF = types::MQIACF(1441);
        pub const MQIACF_SYSP_SMF_STAT_TIME_SECS: types::MQIACF = types::MQIACF(1442);
        pub const MQIACF_SYSP_SMF_ACCT_TIME_MINS: types::MQIACF = types::MQIACF(1443);
        pub const MQIACF_SYSP_SMF_ACCT_TIME_SECS: types::MQIACF = types::MQIACF(1444);
        pub const MQIACF_Q_MGR_STATUS_INFO_TYPE: types::MQIACF = types::MQIACF(1445);
        pub const MQIACF_Q_MGR_STATUS_INFO_Q_MGR: types::MQIACF = types::MQIACF(1446);
        pub const MQIACF_Q_MGR_STATUS_INFO_NHA: types::MQIACF = types::MQIACF(1447);
        pub const MQIACF_AUTO_CLUSTER_TYPE: types::MQIACF = types::MQIACF(1448);
        pub const MQIACF_DATA_FS_IN_USE: types::MQIACF = types::MQIACF(1449);
        pub const MQIACF_DATA_FS_SIZE: types::MQIACF = types::MQIACF(1450);
        pub const MQIACF_LOG_EXTENT_SIZE: types::MQIACF = types::MQIACF(1451);
        pub const MQIACF_LOG_FS_IN_USE: types::MQIACF = types::MQIACF(1452);
        pub const MQIACF_LOG_FS_SIZE: types::MQIACF = types::MQIACF(1453);
        pub const MQIACF_LOG_PRIMARIES: types::MQIACF = types::MQIACF(1454);
        pub const MQIACF_LOG_SECONDARIES: types::MQIACF = types::MQIACF(1455);
        pub const MQIACF_LOG_TYPE: types::MQIACF = types::MQIACF(1456);
        pub const MQIACF_NHA_INSTANCE_ACTV_CONNS: types::MQIACF = types::MQIACF(1457);
        pub const MQIACF_NHA_INSTANCE_BACKLOG: types::MQIACF = types::MQIACF(1458);
        pub const MQIACF_NHA_INSTANCE_IN_SYNC: types::MQIACF = types::MQIACF(1459);
        pub const MQIACF_NHA_INSTANCE_ROLE: types::MQIACF = types::MQIACF(1460);
        pub const MQIACF_NHA_IN_SYNC_INSTANCES: types::MQIACF = types::MQIACF(1461);
        pub const MQIACF_NHA_TOTAL_INSTANCES: types::MQIACF = types::MQIACF(1462);
        pub const MQIACF_Q_MGR_FS_ENCRYPTED: types::MQIACF = types::MQIACF(1463);
        pub const MQIACF_Q_MGR_FS_IN_USE: types::MQIACF = types::MQIACF(1464);
        pub const MQIACF_Q_MGR_FS_SIZE: types::MQIACF = types::MQIACF(1465);
        pub const MQIACF_SYSP_ZHYPERLINK: types::MQIACF = types::MQIACF(1466);
        pub const MQIACF_CHECKPOINT_COUNT: types::MQIACF = types::MQIACF(1468);
        pub const MQIACF_CHECKPOINT_OPERATIONS: types::MQIACF = types::MQIACF(1469);
        pub const MQIACF_CHECKPOINT_SIZE: types::MQIACF = types::MQIACF(1470);
        pub const MQIACF_NHA_GROUP_BACKLOG: types::MQIACF = types::MQIACF(1471);
        pub const MQIACF_NHA_GROUP_CONNECTED: types::MQIACF = types::MQIACF(1472);
        pub const MQIACF_NHA_GROUP_IN_SYNC: types::MQIACF = types::MQIACF(1473);
        pub const MQIACF_NHA_GROUP_ROLE: types::MQIACF = types::MQIACF(1474);
        pub const MQIACF_NHA_GROUP_STATUS: types::MQIACF = types::MQIACF(1475);
        pub const MQIACF_NHA_INSTANCE_STATUS: types::MQIACF = types::MQIACF(1476);
        pub const MQIACF_NHA_TYPE: types::MQIACF = types::MQIACF(1477);
        pub const MQIACF_EVENT_DUPLICATE_COUNT: types::MQIACF = types::MQIACF(1478);
        pub const MQIACF_FIRST: types::MQIACF = types::MQIACF(1001);
        pub const MQIACF_MODE: types::MQIACF = types::MQIACF(1008);
        pub const MQIACF_ERROR_IDENTIFIER: types::MQIACF = types::MQIACF(1013);
        pub const MQIACF_SYSP_SMF_STAT_TIME_MINS: types::MQIACF = types::MQIACF(1199);
        pub const MQIACF_LAST_USED: types::MQIACF = types::MQIACF(1478);
        pub const MQIACH_XMIT_PROTOCOL_TYPE: types::MQIACH = types::MQIACH(1501);
        pub const MQIACH_BATCH_SIZE: types::MQIACH = types::MQIACH(1502);
        pub const MQIACH_DISC_INTERVAL: types::MQIACH = types::MQIACH(1503);
        pub const MQIACH_SHORT_TIMER: types::MQIACH = types::MQIACH(1504);
        pub const MQIACH_SHORT_RETRY: types::MQIACH = types::MQIACH(1505);
        pub const MQIACH_LONG_TIMER: types::MQIACH = types::MQIACH(1506);
        pub const MQIACH_LONG_RETRY: types::MQIACH = types::MQIACH(1507);
        pub const MQIACH_PUT_AUTHORITY: types::MQIACH = types::MQIACH(1508);
        pub const MQIACH_SEQUENCE_NUMBER_WRAP: types::MQIACH = types::MQIACH(1509);
        pub const MQIACH_MAX_MSG_LENGTH: types::MQIACH = types::MQIACH(1510);
        pub const MQIACH_CHANNEL_TYPE: types::MQIACH = types::MQIACH(1511);
        pub const MQIACH_DATA_COUNT: types::MQIACH = types::MQIACH(1512);
        pub const MQIACH_NAME_COUNT: types::MQIACH = types::MQIACH(1513);
        pub const MQIACH_MSG_SEQUENCE_NUMBER: types::MQIACH = types::MQIACH(1514);
        pub const MQIACH_DATA_CONVERSION: types::MQIACH = types::MQIACH(1515);
        pub const MQIACH_IN_DOUBT: types::MQIACH = types::MQIACH(1516);
        pub const MQIACH_MCA_TYPE: types::MQIACH = types::MQIACH(1517);
        pub const MQIACH_SESSION_COUNT: types::MQIACH = types::MQIACH(1518);
        pub const MQIACH_ADAPTER: types::MQIACH = types::MQIACH(1519);
        pub const MQIACH_COMMAND_COUNT: types::MQIACH = types::MQIACH(1520);
        pub const MQIACH_SOCKET: types::MQIACH = types::MQIACH(1521);
        pub const MQIACH_PORT: types::MQIACH = types::MQIACH(1522);
        pub const MQIACH_CHANNEL_INSTANCE_TYPE: types::MQIACH = types::MQIACH(1523);
        pub const MQIACH_CHANNEL_INSTANCE_ATTRS: types::MQIACH = types::MQIACH(1524);
        pub const MQIACH_CHANNEL_ERROR_DATA: types::MQIACH = types::MQIACH(1525);
        pub const MQIACH_CHANNEL_TABLE: types::MQIACH = types::MQIACH(1526);
        pub const MQIACH_CHANNEL_STATUS: types::MQIACH = types::MQIACH(1527);
        pub const MQIACH_INDOUBT_STATUS: types::MQIACH = types::MQIACH(1528);
        pub const MQIACH_LAST_SEQ_NUMBER: types::MQIACH = types::MQIACH(1529);
        pub const MQIACH_CURRENT_MSGS: types::MQIACH = types::MQIACH(1531);
        pub const MQIACH_CURRENT_SEQ_NUMBER: types::MQIACH = types::MQIACH(1532);
        pub const MQIACH_SSL_RETURN_CODE: types::MQIACH = types::MQIACH(1533);
        pub const MQIACH_MSGS: types::MQIACH = types::MQIACH(1534);
        pub const MQIACH_BYTES_SENT: types::MQIACH = types::MQIACH(1535);
        pub const MQIACH_BYTES_RCVD: types::MQIACH = types::MQIACH(1536);
        pub const MQIACH_BATCHES: types::MQIACH = types::MQIACH(1537);
        pub const MQIACH_BUFFERS_SENT: types::MQIACH = types::MQIACH(1538);
        pub const MQIACH_BUFFERS_RCVD: types::MQIACH = types::MQIACH(1539);
        pub const MQIACH_LONG_RETRIES_LEFT: types::MQIACH = types::MQIACH(1540);
        pub const MQIACH_SHORT_RETRIES_LEFT: types::MQIACH = types::MQIACH(1541);
        pub const MQIACH_MCA_STATUS: types::MQIACH = types::MQIACH(1542);
        pub const MQIACH_STOP_REQUESTED: types::MQIACH = types::MQIACH(1543);
        pub const MQIACH_MR_COUNT: types::MQIACH = types::MQIACH(1544);
        pub const MQIACH_MR_INTERVAL: types::MQIACH = types::MQIACH(1545);
        pub const MQIACH_NPM_SPEED: types::MQIACH = types::MQIACH(1562);
        pub const MQIACH_HB_INTERVAL: types::MQIACH = types::MQIACH(1563);
        pub const MQIACH_BATCH_INTERVAL: types::MQIACH = types::MQIACH(1564);
        pub const MQIACH_NETWORK_PRIORITY: types::MQIACH = types::MQIACH(1565);
        pub const MQIACH_KEEP_ALIVE_INTERVAL: types::MQIACH = types::MQIACH(1566);
        pub const MQIACH_BATCH_HB: types::MQIACH = types::MQIACH(1567);
        pub const MQIACH_SSL_CLIENT_AUTH: types::MQIACH = types::MQIACH(1568);
        pub const MQIACH_ALLOC_RETRY: types::MQIACH = types::MQIACH(1570);
        pub const MQIACH_ALLOC_FAST_TIMER: types::MQIACH = types::MQIACH(1571);
        pub const MQIACH_ALLOC_SLOW_TIMER: types::MQIACH = types::MQIACH(1572);
        pub const MQIACH_DISC_RETRY: types::MQIACH = types::MQIACH(1573);
        pub const MQIACH_PORT_NUMBER: types::MQIACH = types::MQIACH(1574);
        pub const MQIACH_HDR_COMPRESSION: types::MQIACH = types::MQIACH(1575);
        pub const MQIACH_MSG_COMPRESSION: types::MQIACH = types::MQIACH(1576);
        pub const MQIACH_CLWL_CHANNEL_RANK: types::MQIACH = types::MQIACH(1577);
        pub const MQIACH_CLWL_CHANNEL_PRIORITY: types::MQIACH = types::MQIACH(1578);
        pub const MQIACH_CLWL_CHANNEL_WEIGHT: types::MQIACH = types::MQIACH(1579);
        pub const MQIACH_CHANNEL_DISP: types::MQIACH = types::MQIACH(1580);
        pub const MQIACH_INBOUND_DISP: types::MQIACH = types::MQIACH(1581);
        pub const MQIACH_CHANNEL_TYPES: types::MQIACH = types::MQIACH(1582);
        pub const MQIACH_ADAPS_STARTED: types::MQIACH = types::MQIACH(1583);
        pub const MQIACH_ADAPS_MAX: types::MQIACH = types::MQIACH(1584);
        pub const MQIACH_DISPS_STARTED: types::MQIACH = types::MQIACH(1585);
        pub const MQIACH_DISPS_MAX: types::MQIACH = types::MQIACH(1586);
        pub const MQIACH_SSLTASKS_STARTED: types::MQIACH = types::MQIACH(1587);
        pub const MQIACH_SSLTASKS_MAX: types::MQIACH = types::MQIACH(1588);
        pub const MQIACH_CURRENT_CHL: types::MQIACH = types::MQIACH(1589);
        pub const MQIACH_CURRENT_CHL_MAX: types::MQIACH = types::MQIACH(1590);
        pub const MQIACH_CURRENT_CHL_TCP: types::MQIACH = types::MQIACH(1591);
        pub const MQIACH_CURRENT_CHL_LU62: types::MQIACH = types::MQIACH(1592);
        pub const MQIACH_ACTIVE_CHL: types::MQIACH = types::MQIACH(1593);
        pub const MQIACH_ACTIVE_CHL_MAX: types::MQIACH = types::MQIACH(1594);
        pub const MQIACH_ACTIVE_CHL_PAUSED: types::MQIACH = types::MQIACH(1595);
        pub const MQIACH_ACTIVE_CHL_STARTED: types::MQIACH = types::MQIACH(1596);
        pub const MQIACH_ACTIVE_CHL_STOPPED: types::MQIACH = types::MQIACH(1597);
        pub const MQIACH_ACTIVE_CHL_RETRY: types::MQIACH = types::MQIACH(1598);
        pub const MQIACH_LISTENER_STATUS: types::MQIACH = types::MQIACH(1599);
        pub const MQIACH_SHARED_CHL_RESTART: types::MQIACH = types::MQIACH(1600);
        pub const MQIACH_LISTENER_CONTROL: types::MQIACH = types::MQIACH(1601);
        pub const MQIACH_BACKLOG: types::MQIACH = types::MQIACH(1602);
        pub const MQIACH_XMITQ_TIME_INDICATOR: types::MQIACH = types::MQIACH(1604);
        pub const MQIACH_NETWORK_TIME_INDICATOR: types::MQIACH = types::MQIACH(1605);
        pub const MQIACH_EXIT_TIME_INDICATOR: types::MQIACH = types::MQIACH(1606);
        pub const MQIACH_BATCH_SIZE_INDICATOR: types::MQIACH = types::MQIACH(1607);
        pub const MQIACH_XMITQ_MSGS_AVAILABLE: types::MQIACH = types::MQIACH(1608);
        pub const MQIACH_CHANNEL_SUBSTATE: types::MQIACH = types::MQIACH(1609);
        pub const MQIACH_SSL_KEY_RESETS: types::MQIACH = types::MQIACH(1610);
        pub const MQIACH_COMPRESSION_RATE: types::MQIACH = types::MQIACH(1611);
        pub const MQIACH_COMPRESSION_TIME: types::MQIACH = types::MQIACH(1612);
        pub const MQIACH_MAX_XMIT_SIZE: types::MQIACH = types::MQIACH(1613);
        pub const MQIACH_DEF_CHANNEL_DISP: types::MQIACH = types::MQIACH(1614);
        pub const MQIACH_SHARING_CONVERSATIONS: types::MQIACH = types::MQIACH(1615);
        pub const MQIACH_MAX_SHARING_CONVS: types::MQIACH = types::MQIACH(1616);
        pub const MQIACH_CURRENT_SHARING_CONVS: types::MQIACH = types::MQIACH(1617);
        pub const MQIACH_MAX_INSTANCES: types::MQIACH = types::MQIACH(1618);
        pub const MQIACH_MAX_INSTS_PER_CLIENT: types::MQIACH = types::MQIACH(1619);
        pub const MQIACH_CLIENT_CHANNEL_WEIGHT: types::MQIACH = types::MQIACH(1620);
        pub const MQIACH_CONNECTION_AFFINITY: types::MQIACH = types::MQIACH(1621);
        pub const MQIACH_AUTH_INFO_TYPES: types::MQIACH = types::MQIACH(1622);
        pub const MQIACH_RESET_REQUESTED: types::MQIACH = types::MQIACH(1623);
        pub const MQIACH_BATCH_DATA_LIMIT: types::MQIACH = types::MQIACH(1624);
        pub const MQIACH_MSG_HISTORY: types::MQIACH = types::MQIACH(1625);
        pub const MQIACH_MULTICAST_PROPERTIES: types::MQIACH = types::MQIACH(1626);
        pub const MQIACH_NEW_SUBSCRIBER_HISTORY: types::MQIACH = types::MQIACH(1627);
        pub const MQIACH_MC_HB_INTERVAL: types::MQIACH = types::MQIACH(1628);
        pub const MQIACH_USE_CLIENT_ID: types::MQIACH = types::MQIACH(1629);
        pub const MQIACH_MQTT_KEEP_ALIVE: types::MQIACH = types::MQIACH(1630);
        pub const MQIACH_IN_DOUBT_IN: types::MQIACH = types::MQIACH(1631);
        pub const MQIACH_IN_DOUBT_OUT: types::MQIACH = types::MQIACH(1632);
        pub const MQIACH_MSGS_SENT: types::MQIACH = types::MQIACH(1633);
        pub const MQIACH_MSGS_RCVD: types::MQIACH = types::MQIACH(1634);
        pub const MQIACH_PENDING_OUT: types::MQIACH = types::MQIACH(1635);
        pub const MQIACH_AVAILABLE_CIPHERSPECS: types::MQIACH = types::MQIACH(1636);
        pub const MQIACH_MATCH: types::MQIACH = types::MQIACH(1637);
        pub const MQIACH_USER_SOURCE: types::MQIACH = types::MQIACH(1638);
        pub const MQIACH_WARNING: types::MQIACH = types::MQIACH(1639);
        pub const MQIACH_DEF_RECONNECT: types::MQIACH = types::MQIACH(1640);
        pub const MQIACH_CHANNEL_SUMMARY_ATTRS: types::MQIACH = types::MQIACH(1642);
        pub const MQIACH_PROTOCOL: types::MQIACH = types::MQIACH(1643);
        pub const MQIACH_AMQP_KEEP_ALIVE: types::MQIACH = types::MQIACH(1644);
        pub const MQIACH_SECURITY_PROTOCOL: types::MQIACH = types::MQIACH(1645);
        pub const MQIACH_SPL_PROTECTION: types::MQIACH = types::MQIACH(1646);
        pub const MQIACH_FIRST: types::MQIACH = types::MQIACH(1501);
        pub const MQIACH_LAST_SEQUENCE_NUMBER: types::MQIACH = types::MQIACH(1529);
        pub const MQIACH_CURRENT_SEQUENCE_NUMBER: types::MQIACH = types::MQIACH(1532);
        pub const MQIACH_BYTES_RECEIVED: types::MQIACH = types::MQIACH(1536);
        pub const MQIACH_BUFFERS_RECEIVED: types::MQIACH = types::MQIACH(1539);
        pub const MQIACH_MSGS_RECEIVED: types::MQIACH = types::MQIACH(1634);
        pub const MQIACH_LAST_USED: types::MQIACH = types::MQIACH(1646);
        pub const MQIAMO64_AVG_Q_TIME: types::MQIAMO64 = types::MQIAMO64(703);
        pub const MQIAMO64_Q_TIME_AVG: types::MQIAMO64 = types::MQIAMO64(741);
        pub const MQIAMO64_Q_TIME_MAX: types::MQIAMO64 = types::MQIAMO64(742);
        pub const MQIAMO64_Q_TIME_MIN: types::MQIAMO64 = types::MQIAMO64(743);
        pub const MQIAMO64_BROWSE_BYTES: types::MQIAMO64 = types::MQIAMO64(745);
        pub const MQIAMO64_BYTES: types::MQIAMO64 = types::MQIAMO64(746);
        pub const MQIAMO64_GET_BYTES: types::MQIAMO64 = types::MQIAMO64(747);
        pub const MQIAMO64_PUT_BYTES: types::MQIAMO64 = types::MQIAMO64(748);
        pub const MQIAMO64_TOPIC_PUT_BYTES: types::MQIAMO64 = types::MQIAMO64(783);
        pub const MQIAMO64_PUBLISH_MSG_BYTES: types::MQIAMO64 = types::MQIAMO64(785);
        pub const MQIAMO64_HIGHRES_TIME: types::MQIAMO64 = types::MQIAMO64(838);
        pub const MQIAMO64_QMGR_OP_DURATION: types::MQIAMO64 = types::MQIAMO64(844);
        pub const MQIAMO64_MONITOR_INTERVAL: types::MQIAMO64 = types::MQIAMO64(845);
        pub const MQIAMO_AVG_BATCH_SIZE: types::MQIAMO = types::MQIAMO(702);
        pub const MQIAMO_AVG_Q_TIME: types::MQIAMO = types::MQIAMO(703);
        pub const MQIAMO_BACKOUTS: types::MQIAMO = types::MQIAMO(704);
        pub const MQIAMO_BROWSES: types::MQIAMO = types::MQIAMO(705);
        pub const MQIAMO_BROWSE_MAX_BYTES: types::MQIAMO = types::MQIAMO(706);
        pub const MQIAMO_BROWSE_MIN_BYTES: types::MQIAMO = types::MQIAMO(707);
        pub const MQIAMO_BROWSES_FAILED: types::MQIAMO = types::MQIAMO(708);
        pub const MQIAMO_CLOSES: types::MQIAMO = types::MQIAMO(709);
        pub const MQIAMO_COMMITS: types::MQIAMO = types::MQIAMO(710);
        pub const MQIAMO_COMMITS_FAILED: types::MQIAMO = types::MQIAMO(711);
        pub const MQIAMO_CONNS: types::MQIAMO = types::MQIAMO(712);
        pub const MQIAMO_CONNS_MAX: types::MQIAMO = types::MQIAMO(713);
        pub const MQIAMO_DISCS: types::MQIAMO = types::MQIAMO(714);
        pub const MQIAMO_DISCS_IMPLICIT: types::MQIAMO = types::MQIAMO(715);
        pub const MQIAMO_DISC_TYPE: types::MQIAMO = types::MQIAMO(716);
        pub const MQIAMO_EXIT_TIME_AVG: types::MQIAMO = types::MQIAMO(717);
        pub const MQIAMO_EXIT_TIME_MAX: types::MQIAMO = types::MQIAMO(718);
        pub const MQIAMO_EXIT_TIME_MIN: types::MQIAMO = types::MQIAMO(719);
        pub const MQIAMO_FULL_BATCHES: types::MQIAMO = types::MQIAMO(720);
        pub const MQIAMO_GENERATED_MSGS: types::MQIAMO = types::MQIAMO(721);
        pub const MQIAMO_GETS: types::MQIAMO = types::MQIAMO(722);
        pub const MQIAMO_GET_MAX_BYTES: types::MQIAMO = types::MQIAMO(723);
        pub const MQIAMO_GET_MIN_BYTES: types::MQIAMO = types::MQIAMO(724);
        pub const MQIAMO_GETS_FAILED: types::MQIAMO = types::MQIAMO(725);
        pub const MQIAMO_INCOMPLETE_BATCHES: types::MQIAMO = types::MQIAMO(726);
        pub const MQIAMO_INQS: types::MQIAMO = types::MQIAMO(727);
        pub const MQIAMO_MSGS: types::MQIAMO = types::MQIAMO(728);
        pub const MQIAMO_NET_TIME_AVG: types::MQIAMO = types::MQIAMO(729);
        pub const MQIAMO_NET_TIME_MAX: types::MQIAMO = types::MQIAMO(730);
        pub const MQIAMO_NET_TIME_MIN: types::MQIAMO = types::MQIAMO(731);
        pub const MQIAMO_OBJECT_COUNT: types::MQIAMO = types::MQIAMO(732);
        pub const MQIAMO_OPENS: types::MQIAMO = types::MQIAMO(733);
        pub const MQIAMO_PUT1S: types::MQIAMO = types::MQIAMO(734);
        pub const MQIAMO_PUTS: types::MQIAMO = types::MQIAMO(735);
        pub const MQIAMO_PUT_MAX_BYTES: types::MQIAMO = types::MQIAMO(736);
        pub const MQIAMO_PUT_MIN_BYTES: types::MQIAMO = types::MQIAMO(737);
        pub const MQIAMO_PUT_RETRIES: types::MQIAMO = types::MQIAMO(738);
        pub const MQIAMO_Q_MAX_DEPTH: types::MQIAMO = types::MQIAMO(739);
        pub const MQIAMO_Q_MIN_DEPTH: types::MQIAMO = types::MQIAMO(740);
        pub const MQIAMO_Q_TIME_AVG: types::MQIAMO = types::MQIAMO(741);
        pub const MQIAMO_Q_TIME_MAX: types::MQIAMO = types::MQIAMO(742);
        pub const MQIAMO_Q_TIME_MIN: types::MQIAMO = types::MQIAMO(743);
        pub const MQIAMO_SETS: types::MQIAMO = types::MQIAMO(744);
        pub const MQIAMO_CONNS_FAILED: types::MQIAMO = types::MQIAMO(749);
        pub const MQIAMO_OPENS_FAILED: types::MQIAMO = types::MQIAMO(751);
        pub const MQIAMO_INQS_FAILED: types::MQIAMO = types::MQIAMO(752);
        pub const MQIAMO_SETS_FAILED: types::MQIAMO = types::MQIAMO(753);
        pub const MQIAMO_PUTS_FAILED: types::MQIAMO = types::MQIAMO(754);
        pub const MQIAMO_PUT1S_FAILED: types::MQIAMO = types::MQIAMO(755);
        pub const MQIAMO_CLOSES_FAILED: types::MQIAMO = types::MQIAMO(757);
        pub const MQIAMO_MSGS_EXPIRED: types::MQIAMO = types::MQIAMO(758);
        pub const MQIAMO_MSGS_NOT_QUEUED: types::MQIAMO = types::MQIAMO(759);
        pub const MQIAMO_MSGS_PURGED: types::MQIAMO = types::MQIAMO(760);
        pub const MQIAMO_SUBS_DUR: types::MQIAMO = types::MQIAMO(764);
        pub const MQIAMO_SUBS_NDUR: types::MQIAMO = types::MQIAMO(765);
        pub const MQIAMO_SUBS_FAILED: types::MQIAMO = types::MQIAMO(766);
        pub const MQIAMO_SUBRQS: types::MQIAMO = types::MQIAMO(767);
        pub const MQIAMO_SUBRQS_FAILED: types::MQIAMO = types::MQIAMO(768);
        pub const MQIAMO_CBS: types::MQIAMO = types::MQIAMO(769);
        pub const MQIAMO_CBS_FAILED: types::MQIAMO = types::MQIAMO(770);
        pub const MQIAMO_CTLS: types::MQIAMO = types::MQIAMO(771);
        pub const MQIAMO_CTLS_FAILED: types::MQIAMO = types::MQIAMO(772);
        pub const MQIAMO_STATS: types::MQIAMO = types::MQIAMO(773);
        pub const MQIAMO_STATS_FAILED: types::MQIAMO = types::MQIAMO(774);
        pub const MQIAMO_SUB_DUR_HIGHWATER: types::MQIAMO = types::MQIAMO(775);
        pub const MQIAMO_SUB_DUR_LOWWATER: types::MQIAMO = types::MQIAMO(776);
        pub const MQIAMO_SUB_NDUR_HIGHWATER: types::MQIAMO = types::MQIAMO(777);
        pub const MQIAMO_SUB_NDUR_LOWWATER: types::MQIAMO = types::MQIAMO(778);
        pub const MQIAMO_TOPIC_PUTS: types::MQIAMO = types::MQIAMO(779);
        pub const MQIAMO_TOPIC_PUTS_FAILED: types::MQIAMO = types::MQIAMO(780);
        pub const MQIAMO_TOPIC_PUT1S: types::MQIAMO = types::MQIAMO(781);
        pub const MQIAMO_TOPIC_PUT1S_FAILED: types::MQIAMO = types::MQIAMO(782);
        pub const MQIAMO_PUBLISH_MSG_COUNT: types::MQIAMO = types::MQIAMO(784);
        pub const MQIAMO_UNSUBS_DUR: types::MQIAMO = types::MQIAMO(786);
        pub const MQIAMO_UNSUBS_NDUR: types::MQIAMO = types::MQIAMO(787);
        pub const MQIAMO_UNSUBS_FAILED: types::MQIAMO = types::MQIAMO(788);
        pub const MQIAMO_INTERVAL: types::MQIAMO = types::MQIAMO(789);
        pub const MQIAMO_MSGS_SENT: types::MQIAMO = types::MQIAMO(790);
        pub const MQIAMO_BYTES_SENT: types::MQIAMO = types::MQIAMO(791);
        pub const MQIAMO_REPAIR_BYTES: types::MQIAMO = types::MQIAMO(792);
        pub const MQIAMO_FEEDBACK_MODE: types::MQIAMO = types::MQIAMO(793);
        pub const MQIAMO_RELIABILITY_TYPE: types::MQIAMO = types::MQIAMO(794);
        pub const MQIAMO_LATE_JOIN_MARK: types::MQIAMO = types::MQIAMO(795);
        pub const MQIAMO_NACKS_RCVD: types::MQIAMO = types::MQIAMO(796);
        pub const MQIAMO_REPAIR_PKTS: types::MQIAMO = types::MQIAMO(797);
        pub const MQIAMO_HISTORY_PKTS: types::MQIAMO = types::MQIAMO(798);
        pub const MQIAMO_PENDING_PKTS: types::MQIAMO = types::MQIAMO(799);
        pub const MQIAMO_PKT_RATE: types::MQIAMO = types::MQIAMO(800);
        pub const MQIAMO_MCAST_XMIT_RATE: types::MQIAMO = types::MQIAMO(801);
        pub const MQIAMO_MCAST_BATCH_TIME: types::MQIAMO = types::MQIAMO(802);
        pub const MQIAMO_MCAST_HEARTBEAT: types::MQIAMO = types::MQIAMO(803);
        pub const MQIAMO_DEST_DATA_PORT: types::MQIAMO = types::MQIAMO(804);
        pub const MQIAMO_DEST_REPAIR_PORT: types::MQIAMO = types::MQIAMO(805);
        pub const MQIAMO_ACKS_RCVD: types::MQIAMO = types::MQIAMO(806);
        pub const MQIAMO_ACTIVE_ACKERS: types::MQIAMO = types::MQIAMO(807);
        pub const MQIAMO_PKTS_SENT: types::MQIAMO = types::MQIAMO(808);
        pub const MQIAMO_TOTAL_REPAIR_PKTS: types::MQIAMO = types::MQIAMO(809);
        pub const MQIAMO_TOTAL_PKTS_SENT: types::MQIAMO = types::MQIAMO(810);
        pub const MQIAMO_TOTAL_MSGS_SENT: types::MQIAMO = types::MQIAMO(811);
        pub const MQIAMO_TOTAL_BYTES_SENT: types::MQIAMO = types::MQIAMO(812);
        pub const MQIAMO_NUM_STREAMS: types::MQIAMO = types::MQIAMO(813);
        pub const MQIAMO_ACK_FEEDBACK: types::MQIAMO = types::MQIAMO(814);
        pub const MQIAMO_NACK_FEEDBACK: types::MQIAMO = types::MQIAMO(815);
        pub const MQIAMO_PKTS_LOST: types::MQIAMO = types::MQIAMO(816);
        pub const MQIAMO_MSGS_RCVD: types::MQIAMO = types::MQIAMO(817);
        pub const MQIAMO_MSG_BYTES_RCVD: types::MQIAMO = types::MQIAMO(818);
        pub const MQIAMO_MSGS_DELIVERED: types::MQIAMO = types::MQIAMO(819);
        pub const MQIAMO_PKTS_PROCESSED: types::MQIAMO = types::MQIAMO(820);
        pub const MQIAMO_PKTS_DELIVERED: types::MQIAMO = types::MQIAMO(821);
        pub const MQIAMO_PKTS_DROPPED: types::MQIAMO = types::MQIAMO(822);
        pub const MQIAMO_PKTS_DUPLICATED: types::MQIAMO = types::MQIAMO(823);
        pub const MQIAMO_NACKS_CREATED: types::MQIAMO = types::MQIAMO(824);
        pub const MQIAMO_NACK_PKTS_SENT: types::MQIAMO = types::MQIAMO(825);
        pub const MQIAMO_REPAIR_PKTS_RQSTD: types::MQIAMO = types::MQIAMO(826);
        pub const MQIAMO_REPAIR_PKTS_RCVD: types::MQIAMO = types::MQIAMO(827);
        pub const MQIAMO_PKTS_REPAIRED: types::MQIAMO = types::MQIAMO(828);
        pub const MQIAMO_TOTAL_MSGS_RCVD: types::MQIAMO = types::MQIAMO(829);
        pub const MQIAMO_TOTAL_MSG_BYTES_RCVD: types::MQIAMO = types::MQIAMO(830);
        pub const MQIAMO_TOTAL_REPAIR_PKTS_RCVD: types::MQIAMO = types::MQIAMO(831);
        pub const MQIAMO_TOTAL_REPAIR_PKTS_RQSTD: types::MQIAMO = types::MQIAMO(832);
        pub const MQIAMO_TOTAL_MSGS_PROCESSED: types::MQIAMO = types::MQIAMO(833);
        pub const MQIAMO_TOTAL_MSGS_SELECTED: types::MQIAMO = types::MQIAMO(834);
        pub const MQIAMO_TOTAL_MSGS_EXPIRED: types::MQIAMO = types::MQIAMO(835);
        pub const MQIAMO_TOTAL_MSGS_DELIVERED: types::MQIAMO = types::MQIAMO(836);
        pub const MQIAMO_TOTAL_MSGS_RETURNED: types::MQIAMO = types::MQIAMO(837);
        pub const MQIAMO_MONITOR_CLASS: types::MQIAMO = types::MQIAMO(839);
        pub const MQIAMO_MONITOR_TYPE: types::MQIAMO = types::MQIAMO(840);
        pub const MQIAMO_MONITOR_ELEMENT: types::MQIAMO = types::MQIAMO(841);
        pub const MQIAMO_MONITOR_DATATYPE: types::MQIAMO = types::MQIAMO(842);
        pub const MQIAMO_MONITOR_FLAGS: types::MQIAMO = types::MQIAMO(843);
        pub const MQIAMO_FIRST: types::MQIAMO = types::MQIAMO(701);
        pub const MQIAMO_LAST_USED: types::MQIAMO = types::MQIAMO(845);
        pub const MQIAMO_MONITOR_UNIT: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
            1,
        );
        pub const MQIAMO_MONITOR_DELTA: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
            2,
        );
        pub const MQIAMO_MONITOR_LSN: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
            3,
        );
        pub const MQIAMO_MONITOR_HUNDREDTHS: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
            100,
        );
        pub const MQIAMO_MONITOR_KB: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
            1024,
        );
        pub const MQIAMO_MONITOR_PERCENT: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
            10000,
        );
        pub const MQIAMO_MONITOR_MICROSEC: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
            1000000,
        );
        pub const MQIAMO_MONITOR_MB: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
            1048576,
        );
        pub const MQIAMO_MONITOR_GB: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
            100000000,
        );
        pub const MQIAMO_MONITOR_FLAGS_NONE: types::MQIAMO_MONITOR_FLAGS = types::MQIAMO_MONITOR_FLAGS(
            0,
        );
        pub const MQIAMO_MONITOR_FLAGS_OBJNAME: types::MQIAMO_MONITOR_FLAGS = types::MQIAMO_MONITOR_FLAGS(
            1,
        );
        pub const MQIDO_COMMIT: types::MQIDO = types::MQIDO(1);
        pub const MQIDO_BACKOUT: types::MQIDO = types::MQIDO(2);
        pub const MQIMMREASON_NONE: types::MQIMMREASON = types::MQIMMREASON(0);
        pub const MQIMMREASON_NOT_CLIENT: types::MQIMMREASON = types::MQIMMREASON(1);
        pub const MQIMMREASON_NOT_RECONNECTABLE: types::MQIMMREASON = types::MQIMMREASON(
            2,
        );
        pub const MQIMMREASON_MOVING: types::MQIMMREASON = types::MQIMMREASON(3);
        pub const MQIMMREASON_APPLNAME_CHANGED: types::MQIMMREASON = types::MQIMMREASON(
            4,
        );
        pub const MQIMMREASON_IN_TRANSACTION: types::MQIMMREASON = types::MQIMMREASON(5);
        pub const MQIMMREASON_AWAITS_REPLY: types::MQIMMREASON = types::MQIMMREASON(6);
        pub const MQIMMREASON_NO_REDIRECT: types::MQIMMREASON = types::MQIMMREASON(7);
        pub const MQINBD_Q_MGR: types::MQINBD = types::MQINBD(0);
        pub const MQINBD_GROUP: types::MQINBD = types::MQINBD(3);
        pub const MQIS_NO: types::MQIS = types::MQIS(0);
        pub const MQIS_YES: types::MQIS = types::MQIS(1);
        pub const MQLDAPC_INACTIVE: types::MQLDAPC = types::MQLDAPC(0);
        pub const MQLDAPC_CONNECTED: types::MQLDAPC = types::MQLDAPC(1);
        pub const MQLDAPC_ERROR: types::MQLDAPC = types::MQLDAPC(2);
        pub const MQLDAP_AUTHORMD_OS: types::MQLDAP_AUTHORMD = types::MQLDAP_AUTHORMD(0);
        pub const MQLDAP_AUTHORMD_SEARCHGRP: types::MQLDAP_AUTHORMD = types::MQLDAP_AUTHORMD(
            1,
        );
        pub const MQLDAP_AUTHORMD_SEARCHUSR: types::MQLDAP_AUTHORMD = types::MQLDAP_AUTHORMD(
            2,
        );
        pub const MQLDAP_AUTHORMD_SRCHGRPSN: types::MQLDAP_AUTHORMD = types::MQLDAP_AUTHORMD(
            3,
        );
        pub const MQLDAP_NESTGRP_NO: types::MQLDAP_NESTGRP = types::MQLDAP_NESTGRP(0);
        pub const MQLDAP_NESTGRP_YES: types::MQLDAP_NESTGRP = types::MQLDAP_NESTGRP(1);
        pub const MQLOGTYPE_CIRCULAR: types::MQLOGTYPE = types::MQLOGTYPE(0);
        pub const MQLOGTYPE_LINEAR: types::MQLOGTYPE = types::MQLOGTYPE(1);
        pub const MQLOGTYPE_REPLICATED: types::MQLOGTYPE = types::MQLOGTYPE(2);
        pub const MQLR_MAX: types::MQLR = types::MQLR(-2);
        pub const MQLR_AUTO: types::MQLR = types::MQLR(-1);
        pub const MQLR_ONE: types::MQLR = types::MQLR(1);
        pub const MQMATCH_GENERIC: types::MQMATCH = types::MQMATCH(0);
        pub const MQMATCH_RUNCHECK: types::MQMATCH = types::MQMATCH(1);
        pub const MQMATCH_EXACT: types::MQMATCH = types::MQMATCH(2);
        pub const MQMATCH_ALL: types::MQMATCH = types::MQMATCH(3);
        pub const MQMCAS_STOPPED: types::MQMCAS = types::MQMCAS(0);
        pub const MQMCAS_RUNNING: types::MQMCAS = types::MQMCAS(3);
        pub const MQMCP_COMPAT: types::MQMCP = types::MQMCP(-2);
        pub const MQMCP_ALL: types::MQMCP = types::MQMCP(-1);
        pub const MQMCP_NONE: types::MQMCP = types::MQMCP(0);
        pub const MQMCP_USER: types::MQMCP = types::MQMCP(1);
        pub const MQMCP_REPLY: types::MQMCP = types::MQMCP(2);
        pub const MQMLP_ENCRYPTION_ALG_NONE: types::MQMLP_ENCRYPTION = types::MQMLP_ENCRYPTION(
            0,
        );
        pub const MQMLP_ENCRYPTION_ALG_RC2: types::MQMLP_ENCRYPTION = types::MQMLP_ENCRYPTION(
            1,
        );
        pub const MQMLP_ENCRYPTION_ALG_DES: types::MQMLP_ENCRYPTION = types::MQMLP_ENCRYPTION(
            2,
        );
        pub const MQMLP_ENCRYPTION_ALG_3DES: types::MQMLP_ENCRYPTION = types::MQMLP_ENCRYPTION(
            3,
        );
        pub const MQMLP_ENCRYPTION_ALG_AES128: types::MQMLP_ENCRYPTION = types::MQMLP_ENCRYPTION(
            4,
        );
        pub const MQMLP_ENCRYPTION_ALG_AES256: types::MQMLP_ENCRYPTION = types::MQMLP_ENCRYPTION(
            5,
        );
        pub const MQMLP_SIGN_ALG_NONE: types::MQMLP_SIGN = types::MQMLP_SIGN(0);
        pub const MQMLP_SIGN_ALG_MD5: types::MQMLP_SIGN = types::MQMLP_SIGN(1);
        pub const MQMLP_SIGN_ALG_SHA1: types::MQMLP_SIGN = types::MQMLP_SIGN(2);
        pub const MQMLP_SIGN_ALG_SHA224: types::MQMLP_SIGN = types::MQMLP_SIGN(3);
        pub const MQMLP_SIGN_ALG_SHA256: types::MQMLP_SIGN = types::MQMLP_SIGN(4);
        pub const MQMLP_SIGN_ALG_SHA384: types::MQMLP_SIGN = types::MQMLP_SIGN(5);
        pub const MQMLP_SIGN_ALG_SHA512: types::MQMLP_SIGN = types::MQMLP_SIGN(6);
        pub const MQMLP_TOLERATE_UNPROTECTED_NO: types::MQMLP_TOLERATE = types::MQMLP_TOLERATE(
            0,
        );
        pub const MQMLP_TOLERATE_UNPROTECTED_YES: types::MQMLP_TOLERATE = types::MQMLP_TOLERATE(
            1,
        );
        pub const MQMODE_FORCE: types::MQMODE = types::MQMODE(0);
        pub const MQMODE_QUIESCE: types::MQMODE = types::MQMODE(1);
        pub const MQMODE_TERMINATE: types::MQMODE = types::MQMODE(2);
        pub const MQMULC_STANDARD: types::MQMULC = types::MQMULC(0);
        pub const MQMULC_REFINED: types::MQMULC = types::MQMULC(1);
        pub const MQNHABACKLOG_UNKNOWN: types::MQNHABACKLOG = types::MQNHABACKLOG(-1);
        pub const MQNHACONNACTV_NO: types::MQNHACONNACTV = types::MQNHACONNACTV(0);
        pub const MQNHACONNACTV_YES: types::MQNHACONNACTV = types::MQNHACONNACTV(1);
        pub const MQNHACONNGRP_NO: types::MQNHACONNGRP = types::MQNHACONNGRP(0);
        pub const MQNHACONNGRP_YES: types::MQNHACONNGRP = types::MQNHACONNGRP(1);
        pub const MQNHACONNGRP_SUSPENDED: types::MQNHACONNGRP = types::MQNHACONNGRP(2);
        pub const MQNHAGRPROLE_UNKNOWN: types::MQNHAGRPROLE = types::MQNHAGRPROLE(0);
        pub const MQNHAGRPROLE_NOT_CONFIGURED: types::MQNHAGRPROLE = types::MQNHAGRPROLE(
            1,
        );
        pub const MQNHAGRPROLE_LIVE: types::MQNHAGRPROLE = types::MQNHAGRPROLE(2);
        pub const MQNHAGRPROLE_RECOVERY: types::MQNHAGRPROLE = types::MQNHAGRPROLE(3);
        pub const MQNHAGRPROLE_PENDING_LIVE: types::MQNHAGRPROLE = types::MQNHAGRPROLE(
            4,
        );
        pub const MQNHAGRPROLE_PENDING_RECOVERY: types::MQNHAGRPROLE = types::MQNHAGRPROLE(
            5,
        );
        pub const MQNHAINSYNC_NO: types::MQNHAINSYNC = types::MQNHAINSYNC(0);
        pub const MQNHAINSYNC_YES: types::MQNHAINSYNC = types::MQNHAINSYNC(1);
        pub const MQNHAROLE_UNKNOWN: types::MQNHAROLE = types::MQNHAROLE(0);
        pub const MQNHAROLE_ACTIVE: types::MQNHAROLE = types::MQNHAROLE(1);
        pub const MQNHAROLE_REPLICA: types::MQNHAROLE = types::MQNHAROLE(2);
        pub const MQNHAROLE_LEADER: types::MQNHAROLE = types::MQNHAROLE(3);
        pub const MQNHASTATUS_UNKNOWN: types::MQNHASTATUS = types::MQNHASTATUS(0);
        pub const MQNHASTATUS_NORMAL: types::MQNHASTATUS = types::MQNHASTATUS(1);
        pub const MQNHASTATUS_CHECKING: types::MQNHASTATUS = types::MQNHASTATUS(2);
        pub const MQNHASTATUS_SYNCHRONIZING: types::MQNHASTATUS = types::MQNHASTATUS(3);
        pub const MQNHASTATUS_REBASING: types::MQNHASTATUS = types::MQNHASTATUS(4);
        pub const MQNHASTATUS_DISK_FULL: types::MQNHASTATUS = types::MQNHASTATUS(5);
        pub const MQNHASTATUS_DISCONNECTED: types::MQNHASTATUS = types::MQNHASTATUS(6);
        pub const MQNHASTATUS_PARTITIONED: types::MQNHASTATUS = types::MQNHASTATUS(7);
        pub const MQNHATYPE_ALL: types::MQNHATYPE = types::MQNHATYPE(-1);
        pub const MQNHATYPE_INSTANCE: types::MQNHATYPE = types::MQNHATYPE(0);
        pub const MQNHATYPE_GROUP: types::MQNHATYPE = types::MQNHATYPE(1);
        pub const MQNSH_ALL: types::MQNSH = types::MQNSH(-1);
        pub const MQNSH_NONE: types::MQNSH = types::MQNSH(0);
        pub const MQOPER_UNKNOWN: types::MQOPER = types::MQOPER(0);
        pub const MQOPER_BROWSE: types::MQOPER = types::MQOPER(1);
        pub const MQOPER_DISCARD: types::MQOPER = types::MQOPER(2);
        pub const MQOPER_GET: types::MQOPER = types::MQOPER(3);
        pub const MQOPER_PUT: types::MQOPER = types::MQOPER(4);
        pub const MQOPER_PUT_REPLY: types::MQOPER = types::MQOPER(5);
        pub const MQOPER_PUT_REPORT: types::MQOPER = types::MQOPER(6);
        pub const MQOPER_RECEIVE: types::MQOPER = types::MQOPER(7);
        pub const MQOPER_SEND: types::MQOPER = types::MQOPER(8);
        pub const MQOPER_TRANSFORM: types::MQOPER = types::MQOPER(9);
        pub const MQOPER_PUBLISH: types::MQOPER = types::MQOPER(10);
        pub const MQOPER_EXCLUDED_PUBLISH: types::MQOPER = types::MQOPER(11);
        pub const MQOPER_DISCARDED_PUBLISH: types::MQOPER = types::MQOPER(12);
        pub const MQOPER_SYSTEM_FIRST: types::MQOPER = types::MQOPER(0);
        pub const MQOPER_SYSTEM_LAST: types::MQOPER = types::MQOPER(65535);
        pub const MQOPER_APPL_FIRST: types::MQOPER = types::MQOPER(65536);
        pub const MQOPER_APPL_LAST: types::MQOPER = types::MQOPER(999999999);
        pub const MQOPMODE_COMPAT: types::MQOPMODE = types::MQOPMODE(0);
        pub const MQOPMODE_NEW_FUNCTION: types::MQOPMODE = types::MQOPMODE(1);
        pub const MQPAGECLAS_4KB: types::MQPAGECLAS = types::MQPAGECLAS(0);
        pub const MQPAGECLAS_FIXED4KB: types::MQPAGECLAS = types::MQPAGECLAS(1);
        pub const MQPO_NO: types::MQPO = types::MQPO(0);
        pub const MQPO_YES: types::MQPO = types::MQPO(1);
        pub const MQPSCT_NONE: types::MQPSCT = types::MQPSCT(-1);
        pub const MQPSST_ALL: types::MQPSST = types::MQPSST(0);
        pub const MQPSST_LOCAL: types::MQPSST = types::MQPSST(1);
        pub const MQPSST_PARENT: types::MQPSST = types::MQPSST(2);
        pub const MQPSST_CHILD: types::MQPSST = types::MQPSST(3);
        pub const MQPS_STATUS_INACTIVE: types::MQPS = types::MQPS(0);
        pub const MQPS_STATUS_STARTING: types::MQPS = types::MQPS(1);
        pub const MQPS_STATUS_STOPPING: types::MQPS = types::MQPS(2);
        pub const MQPS_STATUS_ACTIVE: types::MQPS = types::MQPS(3);
        pub const MQPS_STATUS_COMPAT: types::MQPS = types::MQPS(4);
        pub const MQPS_STATUS_ERROR: types::MQPS = types::MQPS(5);
        pub const MQPS_STATUS_REFUSED: types::MQPS = types::MQPS(6);
        pub const MQPUBO_NONE: types::MQPUBO = types::MQPUBO(0);
        pub const MQPUBO_CORREL_ID_AS_IDENTITY: types::MQPUBO = types::MQPUBO(1);
        pub const MQPUBO_RETAIN_PUBLICATION: types::MQPUBO = types::MQPUBO(2);
        pub const MQPUBO_OTHER_SUBSCRIBERS_ONLY: types::MQPUBO = types::MQPUBO(4);
        pub const MQPUBO_NO_REGISTRATION: types::MQPUBO = types::MQPUBO(8);
        pub const MQPUBO_IS_RETAINED_PUBLICATION: types::MQPUBO = types::MQPUBO(16);
        pub const MQQMDT_EXPLICIT_CLUSTER_SENDER: types::MQQMDT = types::MQQMDT(1);
        pub const MQQMDT_AUTO_CLUSTER_SENDER: types::MQQMDT = types::MQQMDT(2);
        pub const MQQMDT_CLUSTER_RECEIVER: types::MQQMDT = types::MQQMDT(3);
        pub const MQQMDT_AUTO_EXP_CLUSTER_SENDER: types::MQQMDT = types::MQQMDT(4);
        pub const MQQMFAC_IMS_BRIDGE: types::MQQMFAC = types::MQQMFAC(1);
        pub const MQQMFAC_DB2: types::MQQMFAC = types::MQQMFAC(2);
        pub const MQQMSTA_STARTING: types::MQQMSTA = types::MQQMSTA(1);
        pub const MQQMSTA_RUNNING: types::MQQMSTA = types::MQQMSTA(2);
        pub const MQQMSTA_QUIESCING: types::MQQMSTA = types::MQQMSTA(3);
        pub const MQQMSTA_STANDBY: types::MQQMSTA = types::MQQMSTA(4);
        pub const MQQMT_NORMAL: types::MQQMT = types::MQQMT(0);
        pub const MQQMT_REPOSITORY: types::MQQMT = types::MQQMT(1);
        pub const MQQO_NO: types::MQQO = types::MQQO(0);
        pub const MQQO_YES: types::MQQO = types::MQQO(1);
        pub const MQQSGS_UNKNOWN: types::MQQSGS = types::MQQSGS(0);
        pub const MQQSGS_CREATED: types::MQQSGS = types::MQQSGS(1);
        pub const MQQSGS_ACTIVE: types::MQQSGS = types::MQQSGS(2);
        pub const MQQSGS_INACTIVE: types::MQQSGS = types::MQQSGS(3);
        pub const MQQSGS_FAILED: types::MQQSGS = types::MQQSGS(4);
        pub const MQQSGS_PENDING: types::MQQSGS = types::MQQSGS(5);
        pub const MQQSIE_NONE: types::MQQSIE = types::MQQSIE(0);
        pub const MQQSIE_HIGH: types::MQQSIE = types::MQQSIE(1);
        pub const MQQSIE_OK: types::MQQSIE = types::MQQSIE(2);
        pub const MQQSOT_ALL: types::MQQSOT = types::MQQSOT(1);
        pub const MQQSOT_INPUT: types::MQQSOT = types::MQQSOT(2);
        pub const MQQSOT_OUTPUT: types::MQQSOT = types::MQQSOT(3);
        pub const MQQSO_NO: types::MQQSO = types::MQQSO(0);
        pub const MQQSO_YES: types::MQQSO = types::MQQSO(1);
        pub const MQQSO_EXCLUSIVE: types::MQQSO = types::MQQSO(2);
        pub const MQQSO_SHARED: types::MQQSO = types::MQQSO(1);
        pub const MQQSUM_NO: types::MQQSUM = types::MQQSUM(0);
        pub const MQQSUM_YES: types::MQQSUM = types::MQQSUM(1);
        pub const MQRAR_NO: types::MQRAR = types::MQRAR(0);
        pub const MQRAR_YES: types::MQRAR = types::MQRAR(1);
        pub const MQRCCF_CFH_TYPE_ERROR: types::MQRCCF = types::MQRCCF(3001);
        pub const MQRCCF_CFH_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3002);
        pub const MQRCCF_CFH_VERSION_ERROR: types::MQRCCF = types::MQRCCF(3003);
        pub const MQRCCF_CFH_MSG_SEQ_NUMBER_ERR: types::MQRCCF = types::MQRCCF(3004);
        pub const MQRCCF_CFH_CONTROL_ERROR: types::MQRCCF = types::MQRCCF(3005);
        pub const MQRCCF_CFH_PARM_COUNT_ERROR: types::MQRCCF = types::MQRCCF(3006);
        pub const MQRCCF_CFH_COMMAND_ERROR: types::MQRCCF = types::MQRCCF(3007);
        pub const MQRCCF_COMMAND_FAILED: types::MQRCCF = types::MQRCCF(3008);
        pub const MQRCCF_CFIN_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3009);
        pub const MQRCCF_CFST_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3010);
        pub const MQRCCF_CFST_STRING_LENGTH_ERR: types::MQRCCF = types::MQRCCF(3011);
        pub const MQRCCF_FORCE_VALUE_ERROR: types::MQRCCF = types::MQRCCF(3012);
        pub const MQRCCF_STRUCTURE_TYPE_ERROR: types::MQRCCF = types::MQRCCF(3013);
        pub const MQRCCF_CFIN_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(3014);
        pub const MQRCCF_CFST_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(3015);
        pub const MQRCCF_MSG_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3016);
        pub const MQRCCF_CFIN_DUPLICATE_PARM: types::MQRCCF = types::MQRCCF(3017);
        pub const MQRCCF_CFST_DUPLICATE_PARM: types::MQRCCF = types::MQRCCF(3018);
        pub const MQRCCF_PARM_COUNT_TOO_SMALL: types::MQRCCF = types::MQRCCF(3019);
        pub const MQRCCF_PARM_COUNT_TOO_BIG: types::MQRCCF = types::MQRCCF(3020);
        pub const MQRCCF_Q_ALREADY_IN_CELL: types::MQRCCF = types::MQRCCF(3021);
        pub const MQRCCF_Q_TYPE_ERROR: types::MQRCCF = types::MQRCCF(3022);
        pub const MQRCCF_MD_FORMAT_ERROR: types::MQRCCF = types::MQRCCF(3023);
        pub const MQRCCF_CFSL_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3024);
        pub const MQRCCF_REPLACE_VALUE_ERROR: types::MQRCCF = types::MQRCCF(3025);
        pub const MQRCCF_CFIL_DUPLICATE_VALUE: types::MQRCCF = types::MQRCCF(3026);
        pub const MQRCCF_CFIL_COUNT_ERROR: types::MQRCCF = types::MQRCCF(3027);
        pub const MQRCCF_CFIL_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3028);
        pub const MQRCCF_QUIESCE_VALUE_ERROR: types::MQRCCF = types::MQRCCF(3029);
        pub const MQRCCF_MSG_SEQ_NUMBER_ERROR: types::MQRCCF = types::MQRCCF(3030);
        pub const MQRCCF_PING_DATA_COUNT_ERROR: types::MQRCCF = types::MQRCCF(3031);
        pub const MQRCCF_PING_DATA_COMPARE_ERROR: types::MQRCCF = types::MQRCCF(3032);
        pub const MQRCCF_CFSL_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(3033);
        pub const MQRCCF_CHANNEL_TYPE_ERROR: types::MQRCCF = types::MQRCCF(3034);
        pub const MQRCCF_PARM_SEQUENCE_ERROR: types::MQRCCF = types::MQRCCF(3035);
        pub const MQRCCF_XMIT_PROTOCOL_TYPE_ERR: types::MQRCCF = types::MQRCCF(3036);
        pub const MQRCCF_BATCH_SIZE_ERROR: types::MQRCCF = types::MQRCCF(3037);
        pub const MQRCCF_DISC_INT_ERROR: types::MQRCCF = types::MQRCCF(3038);
        pub const MQRCCF_SHORT_RETRY_ERROR: types::MQRCCF = types::MQRCCF(3039);
        pub const MQRCCF_SHORT_TIMER_ERROR: types::MQRCCF = types::MQRCCF(3040);
        pub const MQRCCF_LONG_RETRY_ERROR: types::MQRCCF = types::MQRCCF(3041);
        pub const MQRCCF_LONG_TIMER_ERROR: types::MQRCCF = types::MQRCCF(3042);
        pub const MQRCCF_SEQ_NUMBER_WRAP_ERROR: types::MQRCCF = types::MQRCCF(3043);
        pub const MQRCCF_MAX_MSG_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3044);
        pub const MQRCCF_PUT_AUTH_ERROR: types::MQRCCF = types::MQRCCF(3045);
        pub const MQRCCF_PURGE_VALUE_ERROR: types::MQRCCF = types::MQRCCF(3046);
        pub const MQRCCF_CFIL_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(3047);
        pub const MQRCCF_MSG_TRUNCATED: types::MQRCCF = types::MQRCCF(3048);
        pub const MQRCCF_CCSID_ERROR: types::MQRCCF = types::MQRCCF(3049);
        pub const MQRCCF_ENCODING_ERROR: types::MQRCCF = types::MQRCCF(3050);
        pub const MQRCCF_QUEUES_VALUE_ERROR: types::MQRCCF = types::MQRCCF(3051);
        pub const MQRCCF_DATA_CONV_VALUE_ERROR: types::MQRCCF = types::MQRCCF(3052);
        pub const MQRCCF_INDOUBT_VALUE_ERROR: types::MQRCCF = types::MQRCCF(3053);
        pub const MQRCCF_ESCAPE_TYPE_ERROR: types::MQRCCF = types::MQRCCF(3054);
        pub const MQRCCF_REPOS_VALUE_ERROR: types::MQRCCF = types::MQRCCF(3055);
        pub const MQRCCF_CHANNEL_TABLE_ERROR: types::MQRCCF = types::MQRCCF(3062);
        pub const MQRCCF_MCA_TYPE_ERROR: types::MQRCCF = types::MQRCCF(3063);
        pub const MQRCCF_CHL_INST_TYPE_ERROR: types::MQRCCF = types::MQRCCF(3064);
        pub const MQRCCF_CHL_STATUS_NOT_FOUND: types::MQRCCF = types::MQRCCF(3065);
        pub const MQRCCF_CFSL_DUPLICATE_PARM: types::MQRCCF = types::MQRCCF(3066);
        pub const MQRCCF_CFSL_TOTAL_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3067);
        pub const MQRCCF_CFSL_COUNT_ERROR: types::MQRCCF = types::MQRCCF(3068);
        pub const MQRCCF_CFSL_STRING_LENGTH_ERR: types::MQRCCF = types::MQRCCF(3069);
        pub const MQRCCF_BROKER_DELETED: types::MQRCCF = types::MQRCCF(3070);
        pub const MQRCCF_STREAM_ERROR: types::MQRCCF = types::MQRCCF(3071);
        pub const MQRCCF_TOPIC_ERROR: types::MQRCCF = types::MQRCCF(3072);
        pub const MQRCCF_NOT_REGISTERED: types::MQRCCF = types::MQRCCF(3073);
        pub const MQRCCF_Q_MGR_NAME_ERROR: types::MQRCCF = types::MQRCCF(3074);
        pub const MQRCCF_INCORRECT_STREAM: types::MQRCCF = types::MQRCCF(3075);
        pub const MQRCCF_Q_NAME_ERROR: types::MQRCCF = types::MQRCCF(3076);
        pub const MQRCCF_NO_RETAINED_MSG: types::MQRCCF = types::MQRCCF(3077);
        pub const MQRCCF_DUPLICATE_IDENTITY: types::MQRCCF = types::MQRCCF(3078);
        pub const MQRCCF_INCORRECT_Q: types::MQRCCF = types::MQRCCF(3079);
        pub const MQRCCF_CORREL_ID_ERROR: types::MQRCCF = types::MQRCCF(3080);
        pub const MQRCCF_NOT_AUTHORIZED: types::MQRCCF = types::MQRCCF(3081);
        pub const MQRCCF_UNKNOWN_STREAM: types::MQRCCF = types::MQRCCF(3082);
        pub const MQRCCF_REG_OPTIONS_ERROR: types::MQRCCF = types::MQRCCF(3083);
        pub const MQRCCF_PUB_OPTIONS_ERROR: types::MQRCCF = types::MQRCCF(3084);
        pub const MQRCCF_UNKNOWN_BROKER: types::MQRCCF = types::MQRCCF(3085);
        pub const MQRCCF_Q_MGR_CCSID_ERROR: types::MQRCCF = types::MQRCCF(3086);
        pub const MQRCCF_DEL_OPTIONS_ERROR: types::MQRCCF = types::MQRCCF(3087);
        pub const MQRCCF_CLUSTER_NAME_CONFLICT: types::MQRCCF = types::MQRCCF(3088);
        pub const MQRCCF_REPOS_NAME_CONFLICT: types::MQRCCF = types::MQRCCF(3089);
        pub const MQRCCF_CLUSTER_Q_USAGE_ERROR: types::MQRCCF = types::MQRCCF(3090);
        pub const MQRCCF_ACTION_VALUE_ERROR: types::MQRCCF = types::MQRCCF(3091);
        pub const MQRCCF_COMMS_LIBRARY_ERROR: types::MQRCCF = types::MQRCCF(3092);
        pub const MQRCCF_NETBIOS_NAME_ERROR: types::MQRCCF = types::MQRCCF(3093);
        pub const MQRCCF_BROKER_COMMAND_FAILED: types::MQRCCF = types::MQRCCF(3094);
        pub const MQRCCF_CFST_CONFLICTING_PARM: types::MQRCCF = types::MQRCCF(3095);
        pub const MQRCCF_PATH_NOT_VALID: types::MQRCCF = types::MQRCCF(3096);
        pub const MQRCCF_PARM_SYNTAX_ERROR: types::MQRCCF = types::MQRCCF(3097);
        pub const MQRCCF_PWD_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3098);
        pub const MQRCCF_FILTER_ERROR: types::MQRCCF = types::MQRCCF(3150);
        pub const MQRCCF_WRONG_USER: types::MQRCCF = types::MQRCCF(3151);
        pub const MQRCCF_DUPLICATE_SUBSCRIPTION: types::MQRCCF = types::MQRCCF(3152);
        pub const MQRCCF_SUB_NAME_ERROR: types::MQRCCF = types::MQRCCF(3153);
        pub const MQRCCF_SUB_IDENTITY_ERROR: types::MQRCCF = types::MQRCCF(3154);
        pub const MQRCCF_SUBSCRIPTION_IN_USE: types::MQRCCF = types::MQRCCF(3155);
        pub const MQRCCF_SUBSCRIPTION_LOCKED: types::MQRCCF = types::MQRCCF(3156);
        pub const MQRCCF_ALREADY_JOINED: types::MQRCCF = types::MQRCCF(3157);
        pub const MQRCCF_OBJECT_IN_USE: types::MQRCCF = types::MQRCCF(3160);
        pub const MQRCCF_UNKNOWN_FILE_NAME: types::MQRCCF = types::MQRCCF(3161);
        pub const MQRCCF_FILE_NOT_AVAILABLE: types::MQRCCF = types::MQRCCF(3162);
        pub const MQRCCF_DISC_RETRY_ERROR: types::MQRCCF = types::MQRCCF(3163);
        pub const MQRCCF_ALLOC_RETRY_ERROR: types::MQRCCF = types::MQRCCF(3164);
        pub const MQRCCF_ALLOC_SLOW_TIMER_ERROR: types::MQRCCF = types::MQRCCF(3165);
        pub const MQRCCF_ALLOC_FAST_TIMER_ERROR: types::MQRCCF = types::MQRCCF(3166);
        pub const MQRCCF_PORT_NUMBER_ERROR: types::MQRCCF = types::MQRCCF(3167);
        pub const MQRCCF_CHL_SYSTEM_NOT_ACTIVE: types::MQRCCF = types::MQRCCF(3168);
        pub const MQRCCF_ENTITY_NAME_MISSING: types::MQRCCF = types::MQRCCF(3169);
        pub const MQRCCF_PROFILE_NAME_ERROR: types::MQRCCF = types::MQRCCF(3170);
        pub const MQRCCF_AUTH_VALUE_ERROR: types::MQRCCF = types::MQRCCF(3171);
        pub const MQRCCF_AUTH_VALUE_MISSING: types::MQRCCF = types::MQRCCF(3172);
        pub const MQRCCF_OBJECT_TYPE_MISSING: types::MQRCCF = types::MQRCCF(3173);
        pub const MQRCCF_CONNECTION_ID_ERROR: types::MQRCCF = types::MQRCCF(3174);
        pub const MQRCCF_LOG_TYPE_ERROR: types::MQRCCF = types::MQRCCF(3175);
        pub const MQRCCF_PROGRAM_NOT_AVAILABLE: types::MQRCCF = types::MQRCCF(3176);
        pub const MQRCCF_PROGRAM_AUTH_FAILED: types::MQRCCF = types::MQRCCF(3177);
        pub const MQRCCF_NONE_FOUND: types::MQRCCF = types::MQRCCF(3200);
        pub const MQRCCF_SECURITY_SWITCH_OFF: types::MQRCCF = types::MQRCCF(3201);
        pub const MQRCCF_SECURITY_REFRESH_FAILED: types::MQRCCF = types::MQRCCF(3202);
        pub const MQRCCF_PARM_CONFLICT: types::MQRCCF = types::MQRCCF(3203);
        pub const MQRCCF_COMMAND_INHIBITED: types::MQRCCF = types::MQRCCF(3204);
        pub const MQRCCF_OBJECT_BEING_DELETED: types::MQRCCF = types::MQRCCF(3205);
        pub const MQRCCF_STORAGE_CLASS_IN_USE: types::MQRCCF = types::MQRCCF(3207);
        pub const MQRCCF_OBJECT_NAME_RESTRICTED: types::MQRCCF = types::MQRCCF(3208);
        pub const MQRCCF_OBJECT_LIMIT_EXCEEDED: types::MQRCCF = types::MQRCCF(3209);
        pub const MQRCCF_OBJECT_OPEN_FORCE: types::MQRCCF = types::MQRCCF(3210);
        pub const MQRCCF_DISPOSITION_CONFLICT: types::MQRCCF = types::MQRCCF(3211);
        pub const MQRCCF_Q_MGR_NOT_IN_QSG: types::MQRCCF = types::MQRCCF(3212);
        pub const MQRCCF_ATTR_VALUE_FIXED: types::MQRCCF = types::MQRCCF(3213);
        pub const MQRCCF_NAMELIST_ERROR: types::MQRCCF = types::MQRCCF(3215);
        pub const MQRCCF_NO_CHANNEL_INITIATOR: types::MQRCCF = types::MQRCCF(3217);
        pub const MQRCCF_CHANNEL_INITIATOR_ERROR: types::MQRCCF = types::MQRCCF(3218);
        pub const MQRCCF_COMMAND_LEVEL_CONFLICT: types::MQRCCF = types::MQRCCF(3222);
        pub const MQRCCF_Q_ATTR_CONFLICT: types::MQRCCF = types::MQRCCF(3223);
        pub const MQRCCF_EVENTS_DISABLED: types::MQRCCF = types::MQRCCF(3224);
        pub const MQRCCF_COMMAND_SCOPE_ERROR: types::MQRCCF = types::MQRCCF(3225);
        pub const MQRCCF_COMMAND_REPLY_ERROR: types::MQRCCF = types::MQRCCF(3226);
        pub const MQRCCF_FUNCTION_RESTRICTED: types::MQRCCF = types::MQRCCF(3227);
        pub const MQRCCF_PARM_MISSING: types::MQRCCF = types::MQRCCF(3228);
        pub const MQRCCF_PARM_VALUE_ERROR: types::MQRCCF = types::MQRCCF(3229);
        pub const MQRCCF_COMMAND_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3230);
        pub const MQRCCF_COMMAND_ORIGIN_ERROR: types::MQRCCF = types::MQRCCF(3231);
        pub const MQRCCF_LISTENER_CONFLICT: types::MQRCCF = types::MQRCCF(3232);
        pub const MQRCCF_LISTENER_STARTED: types::MQRCCF = types::MQRCCF(3233);
        pub const MQRCCF_LISTENER_STOPPED: types::MQRCCF = types::MQRCCF(3234);
        pub const MQRCCF_CHANNEL_ERROR: types::MQRCCF = types::MQRCCF(3235);
        pub const MQRCCF_CF_STRUC_ERROR: types::MQRCCF = types::MQRCCF(3236);
        pub const MQRCCF_UNKNOWN_USER_ID: types::MQRCCF = types::MQRCCF(3237);
        pub const MQRCCF_UNEXPECTED_ERROR: types::MQRCCF = types::MQRCCF(3238);
        pub const MQRCCF_NO_XCF_PARTNER: types::MQRCCF = types::MQRCCF(3239);
        pub const MQRCCF_CFGR_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(3240);
        pub const MQRCCF_CFIF_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3241);
        pub const MQRCCF_CFIF_OPERATOR_ERROR: types::MQRCCF = types::MQRCCF(3242);
        pub const MQRCCF_CFIF_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(3243);
        pub const MQRCCF_CFSF_FILTER_VAL_LEN_ERR: types::MQRCCF = types::MQRCCF(3244);
        pub const MQRCCF_CFSF_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3245);
        pub const MQRCCF_CFSF_OPERATOR_ERROR: types::MQRCCF = types::MQRCCF(3246);
        pub const MQRCCF_CFSF_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(3247);
        pub const MQRCCF_TOO_MANY_FILTERS: types::MQRCCF = types::MQRCCF(3248);
        pub const MQRCCF_LISTENER_RUNNING: types::MQRCCF = types::MQRCCF(3249);
        pub const MQRCCF_LSTR_STATUS_NOT_FOUND: types::MQRCCF = types::MQRCCF(3250);
        pub const MQRCCF_SERVICE_RUNNING: types::MQRCCF = types::MQRCCF(3251);
        pub const MQRCCF_SERV_STATUS_NOT_FOUND: types::MQRCCF = types::MQRCCF(3252);
        pub const MQRCCF_SERVICE_STOPPED: types::MQRCCF = types::MQRCCF(3253);
        pub const MQRCCF_CFBS_DUPLICATE_PARM: types::MQRCCF = types::MQRCCF(3254);
        pub const MQRCCF_CFBS_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3255);
        pub const MQRCCF_CFBS_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(3256);
        pub const MQRCCF_CFBS_STRING_LENGTH_ERR: types::MQRCCF = types::MQRCCF(3257);
        pub const MQRCCF_CFGR_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3258);
        pub const MQRCCF_CFGR_PARM_COUNT_ERROR: types::MQRCCF = types::MQRCCF(3259);
        pub const MQRCCF_CONN_NOT_STOPPED: types::MQRCCF = types::MQRCCF(3260);
        pub const MQRCCF_SERVICE_REQUEST_PENDING: types::MQRCCF = types::MQRCCF(3261);
        pub const MQRCCF_NO_START_CMD: types::MQRCCF = types::MQRCCF(3262);
        pub const MQRCCF_NO_STOP_CMD: types::MQRCCF = types::MQRCCF(3263);
        pub const MQRCCF_CFBF_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(3264);
        pub const MQRCCF_CFBF_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(3265);
        pub const MQRCCF_CFBF_OPERATOR_ERROR: types::MQRCCF = types::MQRCCF(3266);
        pub const MQRCCF_CFBF_FILTER_VAL_LEN_ERR: types::MQRCCF = types::MQRCCF(3267);
        pub const MQRCCF_LISTENER_STILL_ACTIVE: types::MQRCCF = types::MQRCCF(3268);
        pub const MQRCCF_DEF_XMIT_Q_CLUS_ERROR: types::MQRCCF = types::MQRCCF(3269);
        pub const MQRCCF_TOPICSTR_ALREADY_EXISTS: types::MQRCCF = types::MQRCCF(3300);
        pub const MQRCCF_SHARING_CONVS_ERROR: types::MQRCCF = types::MQRCCF(3301);
        pub const MQRCCF_SHARING_CONVS_TYPE: types::MQRCCF = types::MQRCCF(3302);
        pub const MQRCCF_SECURITY_CASE_CONFLICT: types::MQRCCF = types::MQRCCF(3303);
        pub const MQRCCF_TOPIC_TYPE_ERROR: types::MQRCCF = types::MQRCCF(3305);
        pub const MQRCCF_MAX_INSTANCES_ERROR: types::MQRCCF = types::MQRCCF(3306);
        pub const MQRCCF_MAX_INSTS_PER_CLNT_ERR: types::MQRCCF = types::MQRCCF(3307);
        pub const MQRCCF_TOPIC_STRING_NOT_FOUND: types::MQRCCF = types::MQRCCF(3308);
        pub const MQRCCF_SUBSCRIPTION_POINT_ERR: types::MQRCCF = types::MQRCCF(3309);
        pub const MQRCCF_SUB_ALREADY_EXISTS: types::MQRCCF = types::MQRCCF(3311);
        pub const MQRCCF_UNKNOWN_OBJECT_NAME: types::MQRCCF = types::MQRCCF(3312);
        pub const MQRCCF_REMOTE_Q_NAME_ERROR: types::MQRCCF = types::MQRCCF(3313);
        pub const MQRCCF_DURABILITY_NOT_ALLOWED: types::MQRCCF = types::MQRCCF(3314);
        pub const MQRCCF_HOBJ_ERROR: types::MQRCCF = types::MQRCCF(3315);
        pub const MQRCCF_DEST_NAME_ERROR: types::MQRCCF = types::MQRCCF(3316);
        pub const MQRCCF_INVALID_DESTINATION: types::MQRCCF = types::MQRCCF(3317);
        pub const MQRCCF_PUBSUB_INHIBITED: types::MQRCCF = types::MQRCCF(3318);
        pub const MQRCCF_GROUPUR_CHECKS_FAILED: types::MQRCCF = types::MQRCCF(3319);
        pub const MQRCCF_COMM_INFO_TYPE_ERROR: types::MQRCCF = types::MQRCCF(3320);
        pub const MQRCCF_USE_CLIENT_ID_ERROR: types::MQRCCF = types::MQRCCF(3321);
        pub const MQRCCF_CLIENT_ID_NOT_FOUND: types::MQRCCF = types::MQRCCF(3322);
        pub const MQRCCF_CLIENT_ID_ERROR: types::MQRCCF = types::MQRCCF(3323);
        pub const MQRCCF_PORT_IN_USE: types::MQRCCF = types::MQRCCF(3324);
        pub const MQRCCF_SSL_ALT_PROVIDER_REQD: types::MQRCCF = types::MQRCCF(3325);
        pub const MQRCCF_CHLAUTH_TYPE_ERROR: types::MQRCCF = types::MQRCCF(3326);
        pub const MQRCCF_CHLAUTH_ACTION_ERROR: types::MQRCCF = types::MQRCCF(3327);
        pub const MQRCCF_POLICY_NOT_FOUND: types::MQRCCF = types::MQRCCF(3328);
        pub const MQRCCF_ENCRYPTION_ALG_ERROR: types::MQRCCF = types::MQRCCF(3329);
        pub const MQRCCF_SIGNATURE_ALG_ERROR: types::MQRCCF = types::MQRCCF(3330);
        pub const MQRCCF_TOLERATION_POL_ERROR: types::MQRCCF = types::MQRCCF(3331);
        pub const MQRCCF_POLICY_VERSION_ERROR: types::MQRCCF = types::MQRCCF(3332);
        pub const MQRCCF_RECIPIENT_DN_MISSING: types::MQRCCF = types::MQRCCF(3333);
        pub const MQRCCF_POLICY_NAME_MISSING: types::MQRCCF = types::MQRCCF(3334);
        pub const MQRCCF_CHLAUTH_USERSRC_ERROR: types::MQRCCF = types::MQRCCF(3335);
        pub const MQRCCF_WRONG_CHLAUTH_TYPE: types::MQRCCF = types::MQRCCF(3336);
        pub const MQRCCF_CHLAUTH_ALREADY_EXISTS: types::MQRCCF = types::MQRCCF(3337);
        pub const MQRCCF_CHLAUTH_NOT_FOUND: types::MQRCCF = types::MQRCCF(3338);
        pub const MQRCCF_WRONG_CHLAUTH_ACTION: types::MQRCCF = types::MQRCCF(3339);
        pub const MQRCCF_WRONG_CHLAUTH_USERSRC: types::MQRCCF = types::MQRCCF(3340);
        pub const MQRCCF_CHLAUTH_WARN_ERROR: types::MQRCCF = types::MQRCCF(3341);
        pub const MQRCCF_WRONG_CHLAUTH_MATCH: types::MQRCCF = types::MQRCCF(3342);
        pub const MQRCCF_IPADDR_RANGE_CONFLICT: types::MQRCCF = types::MQRCCF(3343);
        pub const MQRCCF_CHLAUTH_MAX_EXCEEDED: types::MQRCCF = types::MQRCCF(3344);
        pub const MQRCCF_ADDRESS_ERROR: types::MQRCCF = types::MQRCCF(3345);
        pub const MQRCCF_IPADDR_RANGE_ERROR: types::MQRCCF = types::MQRCCF(3346);
        pub const MQRCCF_PROFILE_NAME_MISSING: types::MQRCCF = types::MQRCCF(3347);
        pub const MQRCCF_CHLAUTH_CLNTUSER_ERROR: types::MQRCCF = types::MQRCCF(3348);
        pub const MQRCCF_CHLAUTH_NAME_ERROR: types::MQRCCF = types::MQRCCF(3349);
        pub const MQRCCF_CHLAUTH_RUNCHECK_ERROR: types::MQRCCF = types::MQRCCF(3350);
        pub const MQRCCF_CF_STRUC_ALREADY_FAILED: types::MQRCCF = types::MQRCCF(3351);
        pub const MQRCCF_CFCONLOS_CHECKS_FAILED: types::MQRCCF = types::MQRCCF(3352);
        pub const MQRCCF_SUITE_B_ERROR: types::MQRCCF = types::MQRCCF(3353);
        pub const MQRCCF_CHANNEL_NOT_STARTED: types::MQRCCF = types::MQRCCF(3354);
        pub const MQRCCF_CUSTOM_ERROR: types::MQRCCF = types::MQRCCF(3355);
        pub const MQRCCF_BACKLOG_OUT_OF_RANGE: types::MQRCCF = types::MQRCCF(3356);
        pub const MQRCCF_CHLAUTH_DISABLED: types::MQRCCF = types::MQRCCF(3357);
        pub const MQRCCF_SMDS_REQUIRES_DSGROUP: types::MQRCCF = types::MQRCCF(3358);
        pub const MQRCCF_PSCLUS_DISABLED_TOPDEF: types::MQRCCF = types::MQRCCF(3359);
        pub const MQRCCF_PSCLUS_TOPIC_EXISTS: types::MQRCCF = types::MQRCCF(3360);
        pub const MQRCCF_SSL_CIPHER_SUITE_ERROR: types::MQRCCF = types::MQRCCF(3361);
        pub const MQRCCF_SOCKET_ERROR: types::MQRCCF = types::MQRCCF(3362);
        pub const MQRCCF_CLUS_XMIT_Q_USAGE_ERROR: types::MQRCCF = types::MQRCCF(3363);
        pub const MQRCCF_CERT_VAL_POLICY_ERROR: types::MQRCCF = types::MQRCCF(3364);
        pub const MQRCCF_INVALID_PROTOCOL: types::MQRCCF = types::MQRCCF(3365);
        pub const MQRCCF_REVDNS_DISABLED: types::MQRCCF = types::MQRCCF(3366);
        pub const MQRCCF_CLROUTE_NOT_ALTERABLE: types::MQRCCF = types::MQRCCF(3367);
        pub const MQRCCF_CLUSTER_TOPIC_CONFLICT: types::MQRCCF = types::MQRCCF(3368);
        pub const MQRCCF_DEFCLXQ_MODEL_Q_ERROR: types::MQRCCF = types::MQRCCF(3369);
        pub const MQRCCF_CHLAUTH_CHKCLI_ERROR: types::MQRCCF = types::MQRCCF(3370);
        pub const MQRCCF_CERT_LABEL_NOT_ALLOWED: types::MQRCCF = types::MQRCCF(3371);
        pub const MQRCCF_Q_MGR_ATTR_CONFLICT: types::MQRCCF = types::MQRCCF(3372);
        pub const MQRCCF_ENTITY_TYPE_MISSING: types::MQRCCF = types::MQRCCF(3373);
        pub const MQRCCF_CLWL_EXIT_NAME_ERROR: types::MQRCCF = types::MQRCCF(3374);
        pub const MQRCCF_SERVICE_NAME_ERROR: types::MQRCCF = types::MQRCCF(3375);
        pub const MQRCCF_REMOTE_CHL_TYPE_ERROR: types::MQRCCF = types::MQRCCF(3376);
        pub const MQRCCF_TOPIC_RESTRICTED: types::MQRCCF = types::MQRCCF(3377);
        pub const MQRCCF_CURRENT_LOG_EXTENT: types::MQRCCF = types::MQRCCF(3378);
        pub const MQRCCF_LOG_EXTENT_NOT_FOUND: types::MQRCCF = types::MQRCCF(3379);
        pub const MQRCCF_LOG_NOT_REDUCED: types::MQRCCF = types::MQRCCF(3380);
        pub const MQRCCF_LOG_EXTENT_ERROR: types::MQRCCF = types::MQRCCF(3381);
        pub const MQRCCF_ACCESS_BLOCKED: types::MQRCCF = types::MQRCCF(3382);
        pub const MQRCCF_PS_REQUIRED_MQUC: types::MQRCCF = types::MQRCCF(3383);
        pub const MQRCCF_STREAMQ_DEST_NOT_SUPP: types::MQRCCF = types::MQRCCF(3384);
        pub const MQRCCF_STREAMQ_DEST_CONFLICT: types::MQRCCF = types::MQRCCF(3385);
        pub const MQRCCF_STREAMQ_NOT_SUPPORTED: types::MQRCCF = types::MQRCCF(3386);
        pub const MQRCCF_STREAMQ_CONFLICT: types::MQRCCF = types::MQRCCF(3387);
        pub const MQRCCF_INCOMPATIBLE_QM_IN_QSG: types::MQRCCF = types::MQRCCF(3389);
        pub const MQRCCF_ATTR_VALUE_ERROR_QSG_QM: types::MQRCCF = types::MQRCCF(3390);
        pub const MQRCCF_AUTHORIZED: types::MQRCCF = types::MQRCCF(3391);
        pub const MQRCCF_OBJECT_ALREADY_EXISTS: types::MQRCCF = types::MQRCCF(4001);
        pub const MQRCCF_OBJECT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4002);
        pub const MQRCCF_LIKE_OBJECT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4003);
        pub const MQRCCF_OBJECT_OPEN: types::MQRCCF = types::MQRCCF(4004);
        pub const MQRCCF_ATTR_VALUE_ERROR: types::MQRCCF = types::MQRCCF(4005);
        pub const MQRCCF_UNKNOWN_Q_MGR: types::MQRCCF = types::MQRCCF(4006);
        pub const MQRCCF_Q_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4007);
        pub const MQRCCF_OBJECT_NAME_ERROR: types::MQRCCF = types::MQRCCF(4008);
        pub const MQRCCF_ALLOCATE_FAILED: types::MQRCCF = types::MQRCCF(4009);
        pub const MQRCCF_HOST_NOT_AVAILABLE: types::MQRCCF = types::MQRCCF(4010);
        pub const MQRCCF_CONFIGURATION_ERROR: types::MQRCCF = types::MQRCCF(4011);
        pub const MQRCCF_CONNECTION_REFUSED: types::MQRCCF = types::MQRCCF(4012);
        pub const MQRCCF_ENTRY_ERROR: types::MQRCCF = types::MQRCCF(4013);
        pub const MQRCCF_SEND_FAILED: types::MQRCCF = types::MQRCCF(4014);
        pub const MQRCCF_RECEIVED_DATA_ERROR: types::MQRCCF = types::MQRCCF(4015);
        pub const MQRCCF_RECEIVE_FAILED: types::MQRCCF = types::MQRCCF(4016);
        pub const MQRCCF_CONNECTION_CLOSED: types::MQRCCF = types::MQRCCF(4017);
        pub const MQRCCF_NO_STORAGE: types::MQRCCF = types::MQRCCF(4018);
        pub const MQRCCF_NO_COMMS_MANAGER: types::MQRCCF = types::MQRCCF(4019);
        pub const MQRCCF_LISTENER_NOT_STARTED: types::MQRCCF = types::MQRCCF(4020);
        pub const MQRCCF_BIND_FAILED: types::MQRCCF = types::MQRCCF(4024);
        pub const MQRCCF_CHANNEL_INDOUBT: types::MQRCCF = types::MQRCCF(4025);
        pub const MQRCCF_MQCONN_FAILED: types::MQRCCF = types::MQRCCF(4026);
        pub const MQRCCF_MQOPEN_FAILED: types::MQRCCF = types::MQRCCF(4027);
        pub const MQRCCF_MQGET_FAILED: types::MQRCCF = types::MQRCCF(4028);
        pub const MQRCCF_MQPUT_FAILED: types::MQRCCF = types::MQRCCF(4029);
        pub const MQRCCF_PING_ERROR: types::MQRCCF = types::MQRCCF(4030);
        pub const MQRCCF_CHANNEL_IN_USE: types::MQRCCF = types::MQRCCF(4031);
        pub const MQRCCF_CHANNEL_NOT_FOUND: types::MQRCCF = types::MQRCCF(4032);
        pub const MQRCCF_UNKNOWN_REMOTE_CHANNEL: types::MQRCCF = types::MQRCCF(4033);
        pub const MQRCCF_REMOTE_QM_UNAVAILABLE: types::MQRCCF = types::MQRCCF(4034);
        pub const MQRCCF_REMOTE_QM_TERMINATING: types::MQRCCF = types::MQRCCF(4035);
        pub const MQRCCF_MQINQ_FAILED: types::MQRCCF = types::MQRCCF(4036);
        pub const MQRCCF_NOT_XMIT_Q: types::MQRCCF = types::MQRCCF(4037);
        pub const MQRCCF_CHANNEL_DISABLED: types::MQRCCF = types::MQRCCF(4038);
        pub const MQRCCF_USER_EXIT_NOT_AVAILABLE: types::MQRCCF = types::MQRCCF(4039);
        pub const MQRCCF_COMMIT_FAILED: types::MQRCCF = types::MQRCCF(4040);
        pub const MQRCCF_WRONG_CHANNEL_TYPE: types::MQRCCF = types::MQRCCF(4041);
        pub const MQRCCF_CHANNEL_ALREADY_EXISTS: types::MQRCCF = types::MQRCCF(4042);
        pub const MQRCCF_DATA_TOO_LARGE: types::MQRCCF = types::MQRCCF(4043);
        pub const MQRCCF_CHANNEL_NAME_ERROR: types::MQRCCF = types::MQRCCF(4044);
        pub const MQRCCF_XMIT_Q_NAME_ERROR: types::MQRCCF = types::MQRCCF(4045);
        pub const MQRCCF_MCA_NAME_ERROR: types::MQRCCF = types::MQRCCF(4047);
        pub const MQRCCF_SEND_EXIT_NAME_ERROR: types::MQRCCF = types::MQRCCF(4048);
        pub const MQRCCF_SEC_EXIT_NAME_ERROR: types::MQRCCF = types::MQRCCF(4049);
        pub const MQRCCF_MSG_EXIT_NAME_ERROR: types::MQRCCF = types::MQRCCF(4050);
        pub const MQRCCF_RCV_EXIT_NAME_ERROR: types::MQRCCF = types::MQRCCF(4051);
        pub const MQRCCF_XMIT_Q_NAME_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4052);
        pub const MQRCCF_MCA_NAME_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4053);
        pub const MQRCCF_DISC_INT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4054);
        pub const MQRCCF_SHORT_RETRY_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4055);
        pub const MQRCCF_SHORT_TIMER_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4056);
        pub const MQRCCF_LONG_RETRY_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4057);
        pub const MQRCCF_LONG_TIMER_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4058);
        pub const MQRCCF_PUT_AUTH_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4059);
        pub const MQRCCF_KEEP_ALIVE_INT_ERROR: types::MQRCCF = types::MQRCCF(4060);
        pub const MQRCCF_MISSING_CONN_NAME: types::MQRCCF = types::MQRCCF(4061);
        pub const MQRCCF_CONN_NAME_ERROR: types::MQRCCF = types::MQRCCF(4062);
        pub const MQRCCF_MQSET_FAILED: types::MQRCCF = types::MQRCCF(4063);
        pub const MQRCCF_CHANNEL_NOT_ACTIVE: types::MQRCCF = types::MQRCCF(4064);
        pub const MQRCCF_TERMINATED_BY_SEC_EXIT: types::MQRCCF = types::MQRCCF(4065);
        pub const MQRCCF_DYNAMIC_Q_SCOPE_ERROR: types::MQRCCF = types::MQRCCF(4067);
        pub const MQRCCF_CELL_DIR_NOT_AVAILABLE: types::MQRCCF = types::MQRCCF(4068);
        pub const MQRCCF_MR_COUNT_ERROR: types::MQRCCF = types::MQRCCF(4069);
        pub const MQRCCF_MR_COUNT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4070);
        pub const MQRCCF_MR_EXIT_NAME_ERROR: types::MQRCCF = types::MQRCCF(4071);
        pub const MQRCCF_MR_EXIT_NAME_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4072);
        pub const MQRCCF_MR_INTERVAL_ERROR: types::MQRCCF = types::MQRCCF(4073);
        pub const MQRCCF_MR_INTERVAL_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4074);
        pub const MQRCCF_NPM_SPEED_ERROR: types::MQRCCF = types::MQRCCF(4075);
        pub const MQRCCF_NPM_SPEED_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4076);
        pub const MQRCCF_HB_INTERVAL_ERROR: types::MQRCCF = types::MQRCCF(4077);
        pub const MQRCCF_HB_INTERVAL_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4078);
        pub const MQRCCF_CHAD_ERROR: types::MQRCCF = types::MQRCCF(4079);
        pub const MQRCCF_CHAD_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4080);
        pub const MQRCCF_CHAD_EVENT_ERROR: types::MQRCCF = types::MQRCCF(4081);
        pub const MQRCCF_CHAD_EVENT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4082);
        pub const MQRCCF_CHAD_EXIT_ERROR: types::MQRCCF = types::MQRCCF(4083);
        pub const MQRCCF_CHAD_EXIT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4084);
        pub const MQRCCF_SUPPRESSED_BY_EXIT: types::MQRCCF = types::MQRCCF(4085);
        pub const MQRCCF_BATCH_INT_ERROR: types::MQRCCF = types::MQRCCF(4086);
        pub const MQRCCF_BATCH_INT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4087);
        pub const MQRCCF_NET_PRIORITY_ERROR: types::MQRCCF = types::MQRCCF(4088);
        pub const MQRCCF_NET_PRIORITY_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4089);
        pub const MQRCCF_CHANNEL_CLOSED: types::MQRCCF = types::MQRCCF(4090);
        pub const MQRCCF_Q_STATUS_NOT_FOUND: types::MQRCCF = types::MQRCCF(4091);
        pub const MQRCCF_SSL_CIPHER_SPEC_ERROR: types::MQRCCF = types::MQRCCF(4092);
        pub const MQRCCF_SSL_PEER_NAME_ERROR: types::MQRCCF = types::MQRCCF(4093);
        pub const MQRCCF_SSL_CLIENT_AUTH_ERROR: types::MQRCCF = types::MQRCCF(4094);
        pub const MQRCCF_RETAINED_NOT_SUPPORTED: types::MQRCCF = types::MQRCCF(4095);
        pub const MQRCCF_KWD_VALUE_WRONG_TYPE: types::MQRCCF = types::MQRCCF(4096);
        pub const MQRCCF_APPL_STATUS_NOT_FOUND: types::MQRCCF = types::MQRCCF(4097);
        pub const MQRCCF_NHA_NOT_AVAILABLE: types::MQRCCF = types::MQRCCF(4098);
        pub const MQRCCF_Q_MGR_STATUS_NOT_FOUND: types::MQRCCF = types::MQRCCF(4099);
        pub const MQRCCF_MODE_VALUE_ERROR: types::MQRCCF = types::MQRCCF(3029);
        pub const MQRCCF_IPADDR_ERROR: types::MQRCCF = types::MQRCCF(3345);
        pub const MQRDNS_ENABLED: types::MQRDNS = types::MQRDNS(0);
        pub const MQRDNS_DISABLED: types::MQRDNS = types::MQRDNS(1);
        pub const MQREGO_NONE: types::MQREGO = types::MQREGO(0);
        pub const MQREGO_CORREL_ID_AS_IDENTITY: types::MQREGO = types::MQREGO(1);
        pub const MQREGO_ANONYMOUS: types::MQREGO = types::MQREGO(2);
        pub const MQREGO_LOCAL: types::MQREGO = types::MQREGO(4);
        pub const MQREGO_DIRECT_REQUESTS: types::MQREGO = types::MQREGO(8);
        pub const MQREGO_NEW_PUBLICATIONS_ONLY: types::MQREGO = types::MQREGO(16);
        pub const MQREGO_PUBLISH_ON_REQUEST_ONLY: types::MQREGO = types::MQREGO(32);
        pub const MQREGO_DEREGISTER_ALL: types::MQREGO = types::MQREGO(64);
        pub const MQREGO_INCLUDE_STREAM_NAME: types::MQREGO = types::MQREGO(128);
        pub const MQREGO_INFORM_IF_RETAINED: types::MQREGO = types::MQREGO(256);
        pub const MQREGO_DUPLICATES_OK: types::MQREGO = types::MQREGO(512);
        pub const MQREGO_NON_PERSISTENT: types::MQREGO = types::MQREGO(1024);
        pub const MQREGO_PERSISTENT: types::MQREGO = types::MQREGO(2048);
        pub const MQREGO_PERSISTENT_AS_PUBLISH: types::MQREGO = types::MQREGO(4096);
        pub const MQREGO_PERSISTENT_AS_Q: types::MQREGO = types::MQREGO(8192);
        pub const MQREGO_ADD_NAME: types::MQREGO = types::MQREGO(16384);
        pub const MQREGO_NO_ALTERATION: types::MQREGO = types::MQREGO(32768);
        pub const MQREGO_FULL_RESPONSE: types::MQREGO = types::MQREGO(65536);
        pub const MQREGO_JOIN_SHARED: types::MQREGO = types::MQREGO(131072);
        pub const MQREGO_JOIN_EXCLUSIVE: types::MQREGO = types::MQREGO(262144);
        pub const MQREGO_LEAVE_ONLY: types::MQREGO = types::MQREGO(524288);
        pub const MQREGO_VARIABLE_USER_ID: types::MQREGO = types::MQREGO(1048576);
        pub const MQREGO_LOCKED: types::MQREGO = types::MQREGO(2097152);
        pub const MQROUTE_DELIVER_REJ_UNSUP_MASK: types::MQROUTE = types::MQROUTE(
            -65536,
        );
        pub const MQROUTE_UNLIMITED_ACTIVITIES: types::MQROUTE = types::MQROUTE(0);
        pub const MQROUTE_DETAIL_LOW: types::MQROUTE = types::MQROUTE(2);
        pub const MQROUTE_DETAIL_MEDIUM: types::MQROUTE = types::MQROUTE(8);
        pub const MQROUTE_DETAIL_HIGH: types::MQROUTE = types::MQROUTE(32);
        pub const MQROUTE_FORWARD_ALL: types::MQROUTE = types::MQROUTE(256);
        pub const MQROUTE_FORWARD_IF_SUPPORTED: types::MQROUTE = types::MQROUTE(512);
        pub const MQROUTE_DELIVER_YES: types::MQROUTE = types::MQROUTE(4096);
        pub const MQROUTE_DELIVER_NO: types::MQROUTE = types::MQROUTE(8192);
        pub const MQROUTE_ACCUMULATE_NONE: types::MQROUTE = types::MQROUTE(65539);
        pub const MQROUTE_ACCUMULATE_IN_MSG: types::MQROUTE = types::MQROUTE(65540);
        pub const MQROUTE_ACCUMULATE_AND_REPLY: types::MQROUTE = types::MQROUTE(65541);
        pub const MQROUTE_FORWARD_REJ_UNSUP_MASK: types::MQROUTE = types::MQROUTE(
            -65536,
        );
        pub const MQRP_NO: types::MQRP = types::MQRP(0);
        pub const MQRP_YES: types::MQRP = types::MQRP(1);
        pub const MQRQ_CONN_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(1);
        pub const MQRQ_OPEN_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(2);
        pub const MQRQ_CLOSE_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(3);
        pub const MQRQ_CMD_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(4);
        pub const MQRQ_Q_MGR_STOPPING: types::MQRQ = types::MQRQ(5);
        pub const MQRQ_Q_MGR_QUIESCING: types::MQRQ = types::MQRQ(6);
        pub const MQRQ_CHANNEL_STOPPED_OK: types::MQRQ = types::MQRQ(7);
        pub const MQRQ_CHANNEL_STOPPED_ERROR: types::MQRQ = types::MQRQ(8);
        pub const MQRQ_CHANNEL_STOPPED_RETRY: types::MQRQ = types::MQRQ(9);
        pub const MQRQ_CHANNEL_STOPPED_DISABLED: types::MQRQ = types::MQRQ(10);
        pub const MQRQ_BRIDGE_STOPPED_OK: types::MQRQ = types::MQRQ(11);
        pub const MQRQ_BRIDGE_STOPPED_ERROR: types::MQRQ = types::MQRQ(12);
        pub const MQRQ_SSL_HANDSHAKE_ERROR: types::MQRQ = types::MQRQ(13);
        pub const MQRQ_SSL_CIPHER_SPEC_ERROR: types::MQRQ = types::MQRQ(14);
        pub const MQRQ_SSL_CLIENT_AUTH_ERROR: types::MQRQ = types::MQRQ(15);
        pub const MQRQ_SSL_PEER_NAME_ERROR: types::MQRQ = types::MQRQ(16);
        pub const MQRQ_SUB_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(17);
        pub const MQRQ_SUB_DEST_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(18);
        pub const MQRQ_SSL_UNKNOWN_REVOCATION: types::MQRQ = types::MQRQ(19);
        pub const MQRQ_SYS_CONN_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(20);
        pub const MQRQ_CHANNEL_BLOCKED_ADDRESS: types::MQRQ = types::MQRQ(21);
        pub const MQRQ_CHANNEL_BLOCKED_USERID: types::MQRQ = types::MQRQ(22);
        pub const MQRQ_CHANNEL_BLOCKED_NOACCESS: types::MQRQ = types::MQRQ(23);
        pub const MQRQ_MAX_ACTIVE_CHANNELS: types::MQRQ = types::MQRQ(24);
        pub const MQRQ_MAX_CHANNELS: types::MQRQ = types::MQRQ(25);
        pub const MQRQ_SVRCONN_INST_LIMIT: types::MQRQ = types::MQRQ(26);
        pub const MQRQ_CLIENT_INST_LIMIT: types::MQRQ = types::MQRQ(27);
        pub const MQRQ_CAF_NOT_INSTALLED: types::MQRQ = types::MQRQ(28);
        pub const MQRQ_CSP_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(29);
        pub const MQRQ_FAILOVER_PERMITTED: types::MQRQ = types::MQRQ(30);
        pub const MQRQ_FAILOVER_NOT_PERMITTED: types::MQRQ = types::MQRQ(31);
        pub const MQRQ_STANDBY_ACTIVATED: types::MQRQ = types::MQRQ(32);
        pub const MQRQ_REPLICA_ACTIVATED: types::MQRQ = types::MQRQ(33);
        pub const MQRQ_CONN_AUTHORIZED: types::MQRQ = types::MQRQ(65);
        pub const MQRQ_OPEN_AUTHORIZED: types::MQRQ = types::MQRQ(66);
        pub const MQRQ_SUB_AUTHORIZED: types::MQRQ = types::MQRQ(67);
        pub const MQRQ_SUB_DEST_AUTHORIZED: types::MQRQ = types::MQRQ(68);
        pub const MQRT_CONFIGURATION: types::MQRT = types::MQRT(1);
        pub const MQRT_EXPIRY: types::MQRT = types::MQRT(2);
        pub const MQRT_NSPROC: types::MQRT = types::MQRT(3);
        pub const MQRT_PROXYSUB: types::MQRT = types::MQRT(4);
        pub const MQRT_SUB_CONFIGURATION: types::MQRT = types::MQRT(5);
        pub const MQSECCOMM_NO: types::MQSECCOMM = types::MQSECCOMM(0);
        pub const MQSECCOMM_YES: types::MQSECCOMM = types::MQSECCOMM(1);
        pub const MQSECCOMM_ANON: types::MQSECCOMM = types::MQSECCOMM(2);
        pub const MQSECITEM_ALL: types::MQSECITEM = types::MQSECITEM(0);
        pub const MQSECITEM_MQADMIN: types::MQSECITEM = types::MQSECITEM(1);
        pub const MQSECITEM_MQNLIST: types::MQSECITEM = types::MQSECITEM(2);
        pub const MQSECITEM_MQPROC: types::MQSECITEM = types::MQSECITEM(3);
        pub const MQSECITEM_MQQUEUE: types::MQSECITEM = types::MQSECITEM(4);
        pub const MQSECITEM_MQCONN: types::MQSECITEM = types::MQSECITEM(5);
        pub const MQSECITEM_MQCMDS: types::MQSECITEM = types::MQSECITEM(6);
        pub const MQSECITEM_MXADMIN: types::MQSECITEM = types::MQSECITEM(7);
        pub const MQSECITEM_MXNLIST: types::MQSECITEM = types::MQSECITEM(8);
        pub const MQSECITEM_MXPROC: types::MQSECITEM = types::MQSECITEM(9);
        pub const MQSECITEM_MXQUEUE: types::MQSECITEM = types::MQSECITEM(10);
        pub const MQSECITEM_MXTOPIC: types::MQSECITEM = types::MQSECITEM(11);
        pub const MQSECSW_PROCESS: types::MQSECSW = types::MQSECSW(1);
        pub const MQSECSW_NAMELIST: types::MQSECSW = types::MQSECSW(2);
        pub const MQSECSW_Q: types::MQSECSW = types::MQSECSW(3);
        pub const MQSECSW_TOPIC: types::MQSECSW = types::MQSECSW(4);
        pub const MQSECSW_CONTEXT: types::MQSECSW = types::MQSECSW(6);
        pub const MQSECSW_ALTERNATE_USER: types::MQSECSW = types::MQSECSW(7);
        pub const MQSECSW_COMMAND: types::MQSECSW = types::MQSECSW(8);
        pub const MQSECSW_CONNECTION: types::MQSECSW = types::MQSECSW(9);
        pub const MQSECSW_SUBSYSTEM: types::MQSECSW = types::MQSECSW(10);
        pub const MQSECSW_COMMAND_RESOURCES: types::MQSECSW = types::MQSECSW(11);
        pub const MQSECSW_Q_MGR: types::MQSECSW = types::MQSECSW(15);
        pub const MQSECSW_QSG: types::MQSECSW = types::MQSECSW(16);
        pub const MQSECSW_OFF_FOUND: types::MQSECSW = types::MQSECSW(21);
        pub const MQSECSW_ON_FOUND: types::MQSECSW = types::MQSECSW(22);
        pub const MQSECSW_OFF_NOT_FOUND: types::MQSECSW = types::MQSECSW(23);
        pub const MQSECSW_ON_NOT_FOUND: types::MQSECSW = types::MQSECSW(24);
        pub const MQSECSW_OFF_ERROR: types::MQSECSW = types::MQSECSW(25);
        pub const MQSECSW_ON_OVERRIDDEN: types::MQSECSW = types::MQSECSW(26);
        pub const MQSECTYPE_AUTHSERV: types::MQSECTYPE = types::MQSECTYPE(1);
        pub const MQSECTYPE_SSL: types::MQSECTYPE = types::MQSECTYPE(2);
        pub const MQSECTYPE_CLASSES: types::MQSECTYPE = types::MQSECTYPE(3);
        pub const MQSECTYPE_CONNAUTH: types::MQSECTYPE = types::MQSECTYPE(4);
        pub const MQSELTYPE_NONE: types::MQSELTYPE = types::MQSELTYPE(0);
        pub const MQSELTYPE_STANDARD: types::MQSELTYPE = types::MQSELTYPE(1);
        pub const MQSELTYPE_EXTENDED: types::MQSELTYPE = types::MQSELTYPE(2);
        pub const MQSTDBY_NOT_PERMITTED: types::MQSTDBY = types::MQSTDBY(0);
        pub const MQSTDBY_PERMITTED: types::MQSTDBY = types::MQSTDBY(1);
        pub const MQSUBTYPE_USER: types::MQSUBTYPE = types::MQSUBTYPE(-2);
        pub const MQSUBTYPE_ALL: types::MQSUBTYPE = types::MQSUBTYPE(-1);
        pub const MQSUBTYPE_API: types::MQSUBTYPE = types::MQSUBTYPE(1);
        pub const MQSUBTYPE_ADMIN: types::MQSUBTYPE = types::MQSUBTYPE(2);
        pub const MQSUBTYPE_PROXY: types::MQSUBTYPE = types::MQSUBTYPE(3);
        pub const MQSUS_NO: types::MQSUS = types::MQSUS(0);
        pub const MQSUS_YES: types::MQSUS = types::MQSUS(1);
        pub const MQSYNCPOINT_YES: types::MQSYNCPOINT = types::MQSYNCPOINT(0);
        pub const MQSYNCPOINT_IFPER: types::MQSYNCPOINT = types::MQSYNCPOINT(1);
        pub const MQSYSOBJ_YES: types::MQSYSOBJ = types::MQSYSOBJ(0);
        pub const MQSYSOBJ_NO: types::MQSYSOBJ = types::MQSYSOBJ(1);
        pub const MQSYSP_NO: types::MQSYSP = types::MQSYSP(0);
        pub const MQSYSP_YES: types::MQSYSP = types::MQSYSP(1);
        pub const MQSYSP_EXTENDED: types::MQSYSP = types::MQSYSP(2);
        pub const MQSYSP_TYPE_INITIAL: types::MQSYSP = types::MQSYSP(10);
        pub const MQSYSP_TYPE_SET: types::MQSYSP = types::MQSYSP(11);
        pub const MQSYSP_TYPE_LOG_COPY: types::MQSYSP = types::MQSYSP(12);
        pub const MQSYSP_TYPE_LOG_STATUS: types::MQSYSP = types::MQSYSP(13);
        pub const MQSYSP_TYPE_ARCHIVE_TAPE: types::MQSYSP = types::MQSYSP(14);
        pub const MQSYSP_ALLOC_BLK: types::MQSYSP = types::MQSYSP(20);
        pub const MQSYSP_ALLOC_TRK: types::MQSYSP = types::MQSYSP(21);
        pub const MQSYSP_ALLOC_CYL: types::MQSYSP = types::MQSYSP(22);
        pub const MQSYSP_STATUS_BUSY: types::MQSYSP = types::MQSYSP(30);
        pub const MQSYSP_STATUS_PREMOUNT: types::MQSYSP = types::MQSYSP(31);
        pub const MQSYSP_STATUS_AVAILABLE: types::MQSYSP = types::MQSYSP(32);
        pub const MQSYSP_STATUS_UNKNOWN: types::MQSYSP = types::MQSYSP(33);
        pub const MQSYSP_STATUS_ALLOC_ARCHIVE: types::MQSYSP = types::MQSYSP(34);
        pub const MQSYSP_STATUS_COPYING_BSDS: types::MQSYSP = types::MQSYSP(35);
        pub const MQSYSP_STATUS_COPYING_LOG: types::MQSYSP = types::MQSYSP(36);
        pub const MQS_AVAIL_NORMAL: types::MQS_AVAIL = types::MQS_AVAIL(0);
        pub const MQS_AVAIL_ERROR: types::MQS_AVAIL = types::MQS_AVAIL(1);
        pub const MQS_AVAIL_STOPPED: types::MQS_AVAIL = types::MQS_AVAIL(2);
        pub const MQS_EXPANDST_NORMAL: types::MQS_EXPANDST = types::MQS_EXPANDST(0);
        pub const MQS_EXPANDST_FAILED: types::MQS_EXPANDST = types::MQS_EXPANDST(1);
        pub const MQS_EXPANDST_MAXIMUM: types::MQS_EXPANDST = types::MQS_EXPANDST(2);
        pub const MQS_OPENMODE_NONE: types::MQS_OPENMODE = types::MQS_OPENMODE(0);
        pub const MQS_OPENMODE_READONLY: types::MQS_OPENMODE = types::MQS_OPENMODE(1);
        pub const MQS_OPENMODE_UPDATE: types::MQS_OPENMODE = types::MQS_OPENMODE(2);
        pub const MQS_OPENMODE_RECOVERY: types::MQS_OPENMODE = types::MQS_OPENMODE(3);
        pub const MQS_STATUS_CLOSED: types::MQS_STATUS = types::MQS_STATUS(0);
        pub const MQS_STATUS_CLOSING: types::MQS_STATUS = types::MQS_STATUS(1);
        pub const MQS_STATUS_OPENING: types::MQS_STATUS = types::MQS_STATUS(2);
        pub const MQS_STATUS_OPEN: types::MQS_STATUS = types::MQS_STATUS(3);
        pub const MQS_STATUS_NOTENABLED: types::MQS_STATUS = types::MQS_STATUS(4);
        pub const MQS_STATUS_ALLOCFAIL: types::MQS_STATUS = types::MQS_STATUS(5);
        pub const MQS_STATUS_OPENFAIL: types::MQS_STATUS = types::MQS_STATUS(6);
        pub const MQS_STATUS_STGFAIL: types::MQS_STATUS = types::MQS_STATUS(7);
        pub const MQS_STATUS_DATAFAIL: types::MQS_STATUS = types::MQS_STATUS(8);
        pub const MQTIME_UNIT_MINS: types::MQTIME = types::MQTIME(0);
        pub const MQTIME_UNIT_SECS: types::MQTIME = types::MQTIME(1);
        pub const MQUCI_NO: types::MQUCI = types::MQUCI(0);
        pub const MQUCI_YES: types::MQUCI = types::MQUCI(1);
        pub const MQUIDSUPP_NO: types::MQUIDSUPP = types::MQUIDSUPP(0);
        pub const MQUIDSUPP_YES: types::MQUIDSUPP = types::MQUIDSUPP(1);
        pub const MQUNDELIVERED_NORMAL: types::MQUNDELIVERED = types::MQUNDELIVERED(0);
        pub const MQUNDELIVERED_SAFE: types::MQUNDELIVERED = types::MQUNDELIVERED(1);
        pub const MQUNDELIVERED_DISCARD: types::MQUNDELIVERED = types::MQUNDELIVERED(2);
        pub const MQUNDELIVERED_KEEP: types::MQUNDELIVERED = types::MQUNDELIVERED(3);
        pub const MQUOWST_NONE: types::MQUOWST = types::MQUOWST(0);
        pub const MQUOWST_ACTIVE: types::MQUOWST = types::MQUOWST(1);
        pub const MQUOWST_PREPARED: types::MQUOWST = types::MQUOWST(2);
        pub const MQUOWST_UNRESOLVED: types::MQUOWST = types::MQUOWST(3);
        pub const MQUOWT_Q_MGR: types::MQUOWT = types::MQUOWT(0);
        pub const MQUOWT_CICS: types::MQUOWT = types::MQUOWT(1);
        pub const MQUOWT_RRS: types::MQUOWT = types::MQUOWT(2);
        pub const MQUOWT_IMS: types::MQUOWT = types::MQUOWT(3);
        pub const MQUOWT_XA: types::MQUOWT = types::MQUOWT(4);
        pub const MQUSAGE_DS_OLDEST_ACTIVE_UOW: types::MQUSAGE_DS = types::MQUSAGE_DS(
            10,
        );
        pub const MQUSAGE_DS_OLDEST_PS_RECOVERY: types::MQUSAGE_DS = types::MQUSAGE_DS(
            11,
        );
        pub const MQUSAGE_DS_OLDEST_CF_RECOVERY: types::MQUSAGE_DS = types::MQUSAGE_DS(
            12,
        );
        pub const MQUSAGE_EXPAND_USER: types::MQUSAGE_EXPAND = types::MQUSAGE_EXPAND(1);
        pub const MQUSAGE_EXPAND_SYSTEM: types::MQUSAGE_EXPAND = types::MQUSAGE_EXPAND(
            2,
        );
        pub const MQUSAGE_EXPAND_NONE: types::MQUSAGE_EXPAND = types::MQUSAGE_EXPAND(3);
        pub const MQUSAGE_PS_AVAILABLE: types::MQUSAGE_PS = types::MQUSAGE_PS(0);
        pub const MQUSAGE_PS_DEFINED: types::MQUSAGE_PS = types::MQUSAGE_PS(1);
        pub const MQUSAGE_PS_OFFLINE: types::MQUSAGE_PS = types::MQUSAGE_PS(2);
        pub const MQUSAGE_PS_NOT_DEFINED: types::MQUSAGE_PS = types::MQUSAGE_PS(3);
        pub const MQUSAGE_PS_SUSPENDED: types::MQUSAGE_PS = types::MQUSAGE_PS(4);
        pub const MQUSAGE_SMDS_AVAILABLE: types::MQUSAGE_SMDS = types::MQUSAGE_SMDS(0);
        pub const MQUSAGE_SMDS_NO_DATA: types::MQUSAGE_SMDS = types::MQUSAGE_SMDS(1);
    }
    #[cfg(feature = "pcf")]
    pub use pcf::*;
}
