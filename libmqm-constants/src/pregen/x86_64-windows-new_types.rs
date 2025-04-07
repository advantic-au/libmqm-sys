/* Generated with MQ client version 9.4.2.0 */
pub mod types {
    use ::libmqm_sys::lib as mqsys;
    use crate::value::define_new_type;
    define_new_type!(pub MQACTIVE, mqsys::MQLONG, crate ::mapping::MQACTIVE_CONST);
    define_new_type!(pub MQACTP, mqsys::MQLONG, crate ::mapping::MQACTP_CONST);
    define_new_type!(pub MQACTV, mqsys::MQLONG, crate ::mapping::MQACTV_CONST);
    define_new_type!(pub MQACT, mqsys::MQLONG, crate ::mapping::MQACT_CONST);
    define_new_type!(
        pub MQADOPT_CHECK, mqsys::MQLONG, crate ::mapping::MQADOPT_CHECK_CONST
    );
    define_new_type!(
        pub MQADOPT_TYPE, mqsys::MQLONG, crate ::mapping::MQADOPT_TYPE_CONST
    );
    define_new_type!(pub MQADPCTX, mqsys::MQLONG, crate ::mapping::MQADPCTX_CONST);
    define_new_type!(pub MQAIT, mqsys::MQLONG, crate ::mapping::MQAIT_CONST);
    define_new_type!(pub MQAPPL, mqsys::MQLONG, crate ::mapping::MQAPPL_CONST);
    define_new_type!(pub MQAS, mqsys::MQLONG, crate ::mapping::MQAS_CONST);
    define_new_type!(
        pub MQAT, mqsys::MQLONG, crate ::mapping::MQAT_CONST, "Put Application Types"
    );
    define_new_type!(
        pub MQAUTHENTICATE, mqsys::MQLONG, crate ::mapping::MQAUTHENTICATE_CONST
    );
    define_new_type!(pub MQAUTHOPT, mqsys::MQLONG, crate ::mapping::MQAUTHOPT_CONST);
    define_new_type!(pub MQAUTH, mqsys::MQLONG, crate ::mapping::MQAUTH_CONST);
    define_new_type!(pub MQAUTOCLUS, mqsys::MQLONG, crate ::mapping::MQAUTOCLUS_CONST);
    define_new_type!(pub MQAUTO, mqsys::MQLONG, crate ::mapping::MQAUTO_CONST);
    define_new_type!(pub MQBACF, mqsys::MQLONG, crate ::mapping::MQBACF_CONST);
    define_new_type!(pub MQBALANCED, mqsys::MQLONG, crate ::mapping::MQBALANCED_CONST);
    define_new_type!(pub MQBALSTATE, mqsys::MQLONG, crate ::mapping::MQBALSTATE_CONST);
    define_new_type!(pub MQBL, mqsys::MQLONG, crate ::mapping::MQBL_CONST);
    define_new_type!(
        pub MQBMHO, mqsys::MQLONG, crate ::mapping::MQBMHO_CONST,
        "Options mask to control the action of `MQBUFMH`"
    );
    define_new_type!(pub MQBND, mqsys::MQLONG, crate ::mapping::MQBND_CONST);
    define_new_type!(
        pub MQBNO_BALTYPE, mqsys::MQLONG, crate ::mapping::MQBNO_BALTYPE_CONST
    );
    define_new_type!(
        pub MQBNO_OPTIONS, mqsys::MQLONG, crate ::mapping::MQBNO_OPTIONS_CONST
    );
    define_new_type!(
        pub MQBNO_TIMEOUT, mqsys::MQLONG, crate ::mapping::MQBNO_TIMEOUT_CONST
    );
    define_new_type!(
        pub MQBO, mqsys::MQLONG, crate ::mapping::MQBO_CONST,
        "Options mask to control the action of `MQBEGIN`"
    );
    define_new_type!(
        pub MQBPLOCATION, mqsys::MQLONG, crate ::mapping::MQBPLOCATION_CONST
    );
    define_new_type!(pub MQBT, mqsys::MQLONG, crate ::mapping::MQBT_CONST);
    define_new_type!(pub MQCACF, mqsys::MQLONG, crate ::mapping::MQCACF_CONST);
    define_new_type!(pub MQCACH, mqsys::MQLONG, crate ::mapping::MQCACH_CONST);
    define_new_type!(pub MQCADSD, mqsys::MQLONG, crate ::mapping::MQCADSD_CONST);
    define_new_type!(pub MQCAFTY, mqsys::MQLONG, crate ::mapping::MQCAFTY_CONST);
    define_new_type!(pub MQCAMO, mqsys::MQLONG, crate ::mapping::MQCAMO_CONST);
    define_new_type!(pub MQCAP, mqsys::MQLONG, crate ::mapping::MQCAP_CONST);
    define_new_type!(pub MQCAUT, mqsys::MQLONG, crate ::mapping::MQCAUT_CONST);
    define_new_type!(pub MQCA, mqsys::MQLONG, crate ::mapping::MQCA_CONST);
    define_new_type!(
        pub MQCBCF, mqsys::MQLONG, crate ::mapping::MQCBCF_CONST,
        "Flags containing information about the callback consumer"
    );
    define_new_type!(
        pub MQCBCT, mqsys::MQLONG, crate ::mapping::MQCBCT_CONST,
        "Callback control and message delivery call types"
    );
    define_new_type!(
        pub MQCBDO, mqsys::MQLONG, crate ::mapping::MQCBDO_CONST,
        "Options mask to control the action of `MQCB`"
    );
    define_new_type!(pub MQCBD, mqsys::MQLONG, crate ::mapping::MQCBD_CONST);
    define_new_type!(
        pub MQCBO, mqsys::MQLONG, crate ::mapping::MQCBO_CONST,
        "Create-Bag options mask for `mqCreateBag`"
    );
    define_new_type!(pub MQCBT, mqsys::MQLONG, crate ::mapping::MQCBT_CONST);
    define_new_type!(pub MQCCSI, mqsys::MQLONG, crate ::mapping::MQCCSI_CONST);
    define_new_type!(pub MQCCT, mqsys::MQLONG, crate ::mapping::MQCCT_CONST);
    define_new_type!(
        pub MQCC, mqsys::MQLONG, crate ::mapping::MQCC_CONST,
        "Completion Code from an MQ function call"
    );
    define_new_type!(pub MQCDC, mqsys::MQLONG, crate ::mapping::MQCDC_CONST);
    define_new_type!(pub MQCEX, mqsys::MQLONG, crate ::mapping::MQCEX_CONST);
    define_new_type!(pub MQCFACCESS, mqsys::MQLONG, crate ::mapping::MQCFACCESS_CONST);
    define_new_type!(pub MQCFCONLOS, mqsys::MQLONG, crate ::mapping::MQCFCONLOS_CONST);
    define_new_type!(pub MQCFC, mqsys::MQLONG, crate ::mapping::MQCFC_CONST);
    define_new_type!(pub MQCFOFFLD, mqsys::MQLONG, crate ::mapping::MQCFOFFLD_CONST);
    define_new_type!(
        pub MQCFOP, mqsys::MQLONG, crate ::mapping::MQCFOP_CONST,
        "Command format Filter Operators"
    );
    define_new_type!(
        pub MQCFO_REFRESH, mqsys::MQLONG, crate ::mapping::MQCFO_REFRESH_CONST
    );
    define_new_type!(
        pub MQCFO_REMOVE, mqsys::MQLONG, crate ::mapping::MQCFO_REMOVE_CONST
    );
    define_new_type!(pub MQCFR, mqsys::MQLONG, crate ::mapping::MQCFR_CONST);
    define_new_type!(pub MQCFSTATUS, mqsys::MQLONG, crate ::mapping::MQCFSTATUS_CONST);
    define_new_type!(pub MQCFTYPE, mqsys::MQLONG, crate ::mapping::MQCFTYPE_CONST);
    define_new_type!(pub MQCFT, mqsys::MQLONG, crate ::mapping::MQCFT_CONST);
    define_new_type!(pub MQCF, mqsys::MQLONG, crate ::mapping::MQCF_CONST);
    define_new_type!(pub MQCGWI, mqsys::MQLONG, crate ::mapping::MQCGWI_CONST);
    define_new_type!(pub MQCHAD, mqsys::MQLONG, crate ::mapping::MQCHAD_CONST);
    define_new_type!(pub MQCHIDS, mqsys::MQLONG, crate ::mapping::MQCHIDS_CONST);
    define_new_type!(pub MQCHK, mqsys::MQLONG, crate ::mapping::MQCHK_CONST);
    define_new_type!(pub MQCHLA, mqsys::MQLONG, crate ::mapping::MQCHLA_CONST);
    define_new_type!(pub MQCHLD, mqsys::MQLONG, crate ::mapping::MQCHLD_CONST);
    define_new_type!(pub MQCHRR, mqsys::MQLONG, crate ::mapping::MQCHRR_CONST);
    define_new_type!(pub MQCHSH, mqsys::MQLONG, crate ::mapping::MQCHSH_CONST);
    define_new_type!(pub MQCHSR, mqsys::MQLONG, crate ::mapping::MQCHSR_CONST);
    define_new_type!(pub MQCHSSTATE, mqsys::MQLONG, crate ::mapping::MQCHSSTATE_CONST);
    define_new_type!(pub MQCHS, mqsys::MQLONG, crate ::mapping::MQCHS_CONST);
    define_new_type!(pub MQCHTAB, mqsys::MQLONG, crate ::mapping::MQCHTAB_CONST);
    define_new_type!(pub MQCHT, mqsys::MQLONG, crate ::mapping::MQCHT_CONST);
    define_new_type!(pub MQCIH, mqsys::MQLONG, crate ::mapping::MQCIH_CONST);
    define_new_type!(pub MQCIT, mqsys::MQLONG, crate ::mapping::MQCIT_CONST);
    define_new_type!(pub MQCLCT, mqsys::MQLONG, crate ::mapping::MQCLCT_CONST);
    define_new_type!(pub MQCLROUTE, mqsys::MQLONG, crate ::mapping::MQCLROUTE_CONST);
    define_new_type!(pub MQCLRS, mqsys::MQLONG, crate ::mapping::MQCLRS_CONST);
    define_new_type!(pub MQCLRT, mqsys::MQLONG, crate ::mapping::MQCLRT_CONST);
    define_new_type!(pub MQCLST, mqsys::MQLONG, crate ::mapping::MQCLST_CONST);
    define_new_type!(pub MQCLT, mqsys::MQLONG, crate ::mapping::MQCLT_CONST);
    define_new_type!(pub MQCLWL, mqsys::MQLONG, crate ::mapping::MQCLWL_CONST);
    define_new_type!(pub MQCLXQ, mqsys::MQLONG, crate ::mapping::MQCLXQ_CONST);
    define_new_type!(pub MQCMDI, mqsys::MQLONG, crate ::mapping::MQCMDI_CONST);
    define_new_type!(pub MQCMDL, mqsys::MQLONG, crate ::mapping::MQCMDL_CONST);
    define_new_type!(
        pub MQCMD, mqsys::MQLONG, crate ::mapping::MQCMD_CONST, "Command Codes"
    );
    define_new_type!(
        pub MQCMHO, mqsys::MQLONG, crate ::mapping::MQCMHO_CONST,
        "Create message handle options for `MQCRTMH`"
    );
    define_new_type!(
        pub MQCNO, mqsys::MQLONG, crate ::mapping::MQCNO_CONST,
        "Options mask that control the action of `MQCONNX`"
    );
    define_new_type!(pub MQCODL, mqsys::MQLONG, crate ::mapping::MQCODL_CONST);
    define_new_type!(pub MQCOMPRESS, mqsys::MQLONG, crate ::mapping::MQCOMPRESS_CONST);
    define_new_type!(
        pub MQCOPY, mqsys::MQLONG, crate ::mapping::MQCOPY_CONST,
        "Property copy options mask"
    );
    define_new_type!(
        pub MQCO, mqsys::MQLONG, crate ::mapping::MQCO_CONST,
        "Options mask to control the action of `MQCLOSE`"
    );
    define_new_type!(pub MQCQT, mqsys::MQLONG, crate ::mapping::MQCQT_CONST);
    define_new_type!(pub MQCRC, mqsys::MQLONG, crate ::mapping::MQCRC_CONST);
    define_new_type!(pub MQCSP, mqsys::MQLONG, crate ::mapping::MQCSP_CONST);
    define_new_type!(
        pub MQCSRV_CONVERT, mqsys::MQLONG, crate ::mapping::MQCSRV_CONVERT_CONST
    );
    define_new_type!(pub MQCSRV_DLQ, mqsys::MQLONG, crate ::mapping::MQCSRV_DLQ_CONST);
    define_new_type!(
        pub MQCS, mqsys::MQLONG, crate ::mapping::MQCS_CONST, "Callback consumer state"
    );
    define_new_type!(pub MQCTES, mqsys::MQLONG, crate ::mapping::MQCTES_CONST);
    define_new_type!(pub MQCTLO, mqsys::MQLONG, crate ::mapping::MQCTLO_CONST);
    define_new_type!(pub MQCUOWC, mqsys::MQLONG, crate ::mapping::MQCUOWC_CONST);
    define_new_type!(
        pub MQDCC, mqsys::MQLONG, crate ::mapping::MQDCC_CONST,
        "Options mask that control the action of `MQXCNVC`"
    );
    define_new_type!(pub MQDC, mqsys::MQLONG, crate ::mapping::MQDC_CONST);
    define_new_type!(pub MQDELO, mqsys::MQLONG, crate ::mapping::MQDELO_CONST);
    define_new_type!(pub MQDHF, mqsys::MQLONG, crate ::mapping::MQDHF_CONST);
    define_new_type!(
        pub MQDISCONNECT, mqsys::MQLONG, crate ::mapping::MQDISCONNECT_CONST
    );
    define_new_type!(pub MQDLV, mqsys::MQLONG, crate ::mapping::MQDLV_CONST);
    define_new_type!(pub MQDL, mqsys::MQLONG, crate ::mapping::MQDL_CONST);
    define_new_type!(pub MQDMHO, mqsys::MQLONG, crate ::mapping::MQDMHO_CONST);
    define_new_type!(
        pub MQDMPO, mqsys::MQLONG, crate ::mapping::MQDMPO_CONST,
        "Delete message property options"
    );
    define_new_type!(pub MQDNSWLM, mqsys::MQLONG, crate ::mapping::MQDNSWLM_CONST);
    define_new_type!(pub MQDOPT, mqsys::MQLONG, crate ::mapping::MQDOPT_CONST);
    define_new_type!(pub MQDSB, mqsys::MQLONG, crate ::mapping::MQDSB_CONST);
    define_new_type!(pub MQDSE, mqsys::MQLONG, crate ::mapping::MQDSE_CONST);
    define_new_type!(pub MQEC, mqsys::MQLONG, crate ::mapping::MQEC_CONST);
    define_new_type!(pub MQEI, mqsys::MQLONG, crate ::mapping::MQEI_CONST);
    define_new_type!(
        pub MQENC, mqsys::MQLONG, crate ::mapping::MQENC_CONST,
        "Mask describing data encoding"
    );
    define_new_type!(pub MQEPH, mqsys::MQLONG, crate ::mapping::MQEPH_CONST);
    define_new_type!(pub MQET, mqsys::MQLONG, crate ::mapping::MQET_CONST);
    define_new_type!(pub MQEVO, mqsys::MQLONG, crate ::mapping::MQEVO_CONST);
    define_new_type!(pub MQEVR, mqsys::MQLONG, crate ::mapping::MQEVR_CONST);
    define_new_type!(pub MQEXPI, mqsys::MQLONG, crate ::mapping::MQEXPI_CONST);
    define_new_type!(pub MQEXTATTRS, mqsys::MQLONG, crate ::mapping::MQEXTATTRS_CONST);
    define_new_type!(pub MQEXT, mqsys::MQLONG, crate ::mapping::MQEXT_CONST);
    define_new_type!(pub MQFB, mqsys::MQLONG, crate ::mapping::MQFB_CONST);
    define_new_type!(pub MQFC, mqsys::MQLONG, crate ::mapping::MQFC_CONST);
    define_new_type!(pub MQFSENC, mqsys::MQLONG, crate ::mapping::MQFSENC_CONST);
    define_new_type!(pub MQFS, mqsys::MQLONG, crate ::mapping::MQFS_CONST);
    define_new_type!(pub MQFUN, mqsys::MQLONG, crate ::mapping::MQFUN_CONST);
    define_new_type!(pub MQGACF, mqsys::MQLONG, crate ::mapping::MQGACF_CONST);
    define_new_type!(
        pub MQGMO, mqsys::MQLONG, crate ::mapping::MQGMO_CONST,
        "Options mask to control the action of `MQGET`"
    );
    define_new_type!(pub MQGUR, mqsys::MQLONG, crate ::mapping::MQGUR_CONST);
    define_new_type!(pub MQHA, mqsys::MQLONG, crate ::mapping::MQHA_CONST);
    define_new_type!(pub MQHB, mqsys::MQLONG, crate ::mapping::MQHB_CONST);
    define_new_type!(pub MQHC, mqsys::MQHCONN, crate ::mapping::MQHC_CONST);
    define_new_type!(pub MQHM, mqsys::MQHMSG, crate ::mapping::MQHM_CONST);
    define_new_type!(pub MQHO, mqsys::MQHOBJ, crate ::mapping::MQHO_CONST);
    define_new_type!(pub MQHSTATE, mqsys::MQLONG, crate ::mapping::MQHSTATE_CONST);
    define_new_type!(pub MQIACF, mqsys::MQLONG, crate ::mapping::MQIACF_CONST);
    define_new_type!(pub MQIACH, mqsys::MQLONG, crate ::mapping::MQIACH_CONST);
    define_new_type!(pub MQIAMO64, mqsys::MQLONG, crate ::mapping::MQIAMO64_CONST);
    define_new_type!(pub MQIAMO, mqsys::MQLONG, crate ::mapping::MQIAMO_CONST);
    define_new_type!(
        pub MQIAMO_MONITOR_DATATYPE, mqsys::MQLONG, crate
        ::mapping::MQIAMO_MONITOR_DATATYPE_CONST
    );
    define_new_type!(
        pub MQIAMO_MONITOR_FLAGS, mqsys::MQLONG, crate
        ::mapping::MQIAMO_MONITOR_FLAGS_CONST
    );
    define_new_type!(pub MQIASY, mqsys::MQLONG, crate ::mapping::MQIASY_CONST);
    define_new_type!(pub MQIAV, mqsys::MQLONG, crate ::mapping::MQIAV_CONST);
    define_new_type!(pub MQIA, mqsys::MQLONG, crate ::mapping::MQIA_CONST);
    define_new_type!(pub MQIDO, mqsys::MQLONG, crate ::mapping::MQIDO_CONST);
    define_new_type!(pub MQIEPF, mqsys::MQLONG, crate ::mapping::MQIEPF_CONST);
    define_new_type!(pub MQIGQPA, mqsys::MQLONG, crate ::mapping::MQIGQPA_CONST);
    define_new_type!(pub MQIGQ, mqsys::MQLONG, crate ::mapping::MQIGQ_CONST);
    define_new_type!(pub MQIIH, mqsys::MQLONG, crate ::mapping::MQIIH_CONST);
    define_new_type!(pub MQIMGRCOV, mqsys::MQLONG, crate ::mapping::MQIMGRCOV_CONST);
    define_new_type!(pub MQIMMREASON, mqsys::MQLONG, crate ::mapping::MQIMMREASON_CONST);
    define_new_type!(
        pub MQIMPO, mqsys::MQLONG, crate ::mapping::MQIMPO_CONST,
        "Options mask to control the action of `MQINQMP`"
    );
    define_new_type!(pub MQINBD, mqsys::MQLONG, crate ::mapping::MQINBD_CONST);
    define_new_type!(
        pub MQIND, mqsys::MQLONG, crate ::mapping::MQIND_CONST, "Special Index Values"
    );
    define_new_type!(pub MQIPADDR, mqsys::MQLONG, crate ::mapping::MQIPADDR_CONST);
    define_new_type!(pub MQIS, mqsys::MQLONG, crate ::mapping::MQIS_CONST);
    define_new_type!(
        pub MQITEM, mqsys::MQLONG, crate ::mapping::MQITEM_CONST,
        "Item Type for `mqInquireItemInfo`"
    );
    define_new_type!(pub MQIT, mqsys::MQLONG, crate ::mapping::MQIT_CONST);
    define_new_type!(pub MQKAI, mqsys::MQLONG, crate ::mapping::MQKAI_CONST);
    define_new_type!(pub MQKEY, mqsys::MQLONG, crate ::mapping::MQKEY_CONST);
    define_new_type!(pub MQLDAPC, mqsys::MQLONG, crate ::mapping::MQLDAPC_CONST);
    define_new_type!(
        pub MQLDAP_AUTHORMD, mqsys::MQLONG, crate ::mapping::MQLDAP_AUTHORMD_CONST
    );
    define_new_type!(
        pub MQLDAP_NESTGRP, mqsys::MQLONG, crate ::mapping::MQLDAP_NESTGRP_CONST
    );
    define_new_type!(pub MQLOGTYPE, mqsys::MQLONG, crate ::mapping::MQLOGTYPE_CONST);
    define_new_type!(pub MQLR, mqsys::MQLONG, crate ::mapping::MQLR_CONST);
    define_new_type!(pub MQMASTER, mqsys::MQLONG, crate ::mapping::MQMASTER_CONST);
    define_new_type!(pub MQMATCH, mqsys::MQLONG, crate ::mapping::MQMATCH_CONST);
    define_new_type!(pub MQMCAS, mqsys::MQLONG, crate ::mapping::MQMCAS_CONST);
    define_new_type!(pub MQMCAT, mqsys::MQLONG, crate ::mapping::MQMCAT_CONST);
    define_new_type!(pub MQMCB, mqsys::MQLONG, crate ::mapping::MQMCB_CONST);
    define_new_type!(pub MQMCEV, mqsys::MQLONG, crate ::mapping::MQMCEV_CONST);
    define_new_type!(pub MQMCP, mqsys::MQLONG, crate ::mapping::MQMCP_CONST);
    define_new_type!(pub MQMC, mqsys::MQLONG, crate ::mapping::MQMC_CONST);
    define_new_type!(pub MQMDEF, mqsys::MQLONG, crate ::mapping::MQMDEF_CONST);
    define_new_type!(pub MQMDS, mqsys::MQLONG, crate ::mapping::MQMDS_CONST);
    define_new_type!(
        pub MQMEDIMGINTVL, mqsys::MQLONG, crate ::mapping::MQMEDIMGINTVL_CONST
    );
    define_new_type!(
        pub MQMEDIMGLOGLN, mqsys::MQLONG, crate ::mapping::MQMEDIMGLOGLN_CONST
    );
    define_new_type!(
        pub MQMEDIMGSCHED, mqsys::MQLONG, crate ::mapping::MQMEDIMGSCHED_CONST
    );
    define_new_type!(pub MQMF, mqsys::MQLONG, crate ::mapping::MQMF_CONST);
    define_new_type!(
        pub MQMHBO, mqsys::MQLONG, crate ::mapping::MQMHBO_CONST,
        "Options mask to control the action of `MQMHBUF`"
    );
    define_new_type!(
        pub MQMLP_ENCRYPTION, mqsys::MQLONG, crate ::mapping::MQMLP_ENCRYPTION_CONST
    );
    define_new_type!(pub MQMLP_SIGN, mqsys::MQLONG, crate ::mapping::MQMLP_SIGN_CONST);
    define_new_type!(
        pub MQMLP_TOLERATE, mqsys::MQLONG, crate ::mapping::MQMLP_TOLERATE_CONST
    );
    define_new_type!(pub MQMMBI, mqsys::MQLONG, crate ::mapping::MQMMBI_CONST);
    define_new_type!(pub MQMODE, mqsys::MQLONG, crate ::mapping::MQMODE_CONST);
    define_new_type!(pub MQMON, mqsys::MQLONG, crate ::mapping::MQMON_CONST);
    define_new_type!(
        pub MQMON_AVAILABILITY, mqsys::MQLONG, crate ::mapping::MQMON_AVAILABILITY_CONST
    );
    define_new_type!(
        pub MQMON_OVERRIDE, mqsys::MQLONG, crate ::mapping::MQMON_OVERRIDE_CONST
    );
    define_new_type!(pub MQMO, mqsys::MQLONG, crate ::mapping::MQMO_CONST);
    define_new_type!(pub MQMT, mqsys::MQLONG, crate ::mapping::MQMT_CONST);
    define_new_type!(pub MQMULC, mqsys::MQLONG, crate ::mapping::MQMULC_CONST);
    define_new_type!(pub MQNC, mqsys::MQLONG, crate ::mapping::MQNC_CONST);
    define_new_type!(
        pub MQNHABACKLOG, mqsys::MQLONG, crate ::mapping::MQNHABACKLOG_CONST
    );
    define_new_type!(
        pub MQNHACONNACTV, mqsys::MQLONG, crate ::mapping::MQNHACONNACTV_CONST
    );
    define_new_type!(
        pub MQNHACONNGRP, mqsys::MQLONG, crate ::mapping::MQNHACONNGRP_CONST
    );
    define_new_type!(
        pub MQNHAGRPROLE, mqsys::MQLONG, crate ::mapping::MQNHAGRPROLE_CONST
    );
    define_new_type!(pub MQNHAINSYNC, mqsys::MQLONG, crate ::mapping::MQNHAINSYNC_CONST);
    define_new_type!(pub MQNHAROLE, mqsys::MQLONG, crate ::mapping::MQNHAROLE_CONST);
    define_new_type!(pub MQNHASTATUS, mqsys::MQLONG, crate ::mapping::MQNHASTATUS_CONST);
    define_new_type!(pub MQNHATYPE, mqsys::MQLONG, crate ::mapping::MQNHATYPE_CONST);
    define_new_type!(pub MQNPMS, mqsys::MQLONG, crate ::mapping::MQNPMS_CONST);
    define_new_type!(pub MQNPM, mqsys::MQLONG, crate ::mapping::MQNPM_CONST);
    define_new_type!(pub MQNSH, mqsys::MQLONG, crate ::mapping::MQNSH_CONST);
    define_new_type!(pub MQNT, mqsys::MQLONG, crate ::mapping::MQNT_CONST);
    define_new_type!(pub MQOL, mqsys::MQLONG, crate ::mapping::MQOL_CONST);
    define_new_type!(pub MQOM, mqsys::MQLONG, crate ::mapping::MQOM_CONST);
    define_new_type!(
        pub MQOO, mqsys::MQLONG, crate ::mapping::MQOO_CONST,
        "Options mask to control the action of `MQOPEN`"
    );
    define_new_type!(pub MQOPER, mqsys::MQLONG, crate ::mapping::MQOPER_CONST);
    define_new_type!(pub MQOPMODE, mqsys::MQLONG, crate ::mapping::MQOPMODE_CONST);
    define_new_type!(
        pub MQOP, mqsys::MQLONG, crate ::mapping::MQOP_CONST,
        "Operation codes for `MQCTL` and `MQCB`"
    );
    define_new_type!(
        pub MQOT, mqsys::MQLONG, crate ::mapping::MQOT_CONST,
        "Object Types and Extended Object Types"
    );
    define_new_type!(pub MQPAGECLAS, mqsys::MQLONG, crate ::mapping::MQPAGECLAS_CONST);
    define_new_type!(pub MQPA, mqsys::MQLONG, crate ::mapping::MQPA_CONST);
    define_new_type!(
        pub MQPD, mqsys::MQLONG, crate ::mapping::MQPD_CONST,
        "Property descriptor, support and context"
    );
    define_new_type!(pub MQPER, mqsys::MQLONG, crate ::mapping::MQPER_CONST);
    define_new_type!(pub MQPL, mqsys::MQLONG, crate ::mapping::MQPL_CONST);
    define_new_type!(
        pub MQPMO, mqsys::MQLONG, crate ::mapping::MQPMO_CONST,
        "Options mask to control the action of `MQPUT` and `MQPUT1`"
    );
    define_new_type!(pub MQPMRF, mqsys::MQLONG, crate ::mapping::MQPMRF_CONST);
    define_new_type!(pub MQPO, mqsys::MQLONG, crate ::mapping::MQPO_CONST);
    define_new_type!(pub MQPRI, mqsys::MQLONG, crate ::mapping::MQPRI_CONST);
    define_new_type!(pub MQPROP, mqsys::MQLONG, crate ::mapping::MQPROP_CONST);
    define_new_type!(pub MQPROTO, mqsys::MQLONG, crate ::mapping::MQPROTO_CONST);
    define_new_type!(pub MQPRT, mqsys::MQLONG, crate ::mapping::MQPRT_CONST);
    define_new_type!(pub MQPSCLUS, mqsys::MQLONG, crate ::mapping::MQPSCLUS_CONST);
    define_new_type!(pub MQPSCT, mqsys::MQLONG, crate ::mapping::MQPSCT_CONST);
    define_new_type!(pub MQPSM, mqsys::MQLONG, crate ::mapping::MQPSM_CONST);
    define_new_type!(pub MQPSPROP, mqsys::MQLONG, crate ::mapping::MQPSPROP_CONST);
    define_new_type!(pub MQPSST, mqsys::MQLONG, crate ::mapping::MQPSST_CONST);
    define_new_type!(pub MQPS, mqsys::MQLONG, crate ::mapping::MQPS_CONST);
    define_new_type!(pub MQPUBO, mqsys::MQLONG, crate ::mapping::MQPUBO_CONST);
    define_new_type!(
        pub MQQA_BACKOUT, mqsys::MQLONG, crate ::mapping::MQQA_BACKOUT_CONST
    );
    define_new_type!(pub MQQA_GET, mqsys::MQLONG, crate ::mapping::MQQA_GET_CONST);
    define_new_type!(pub MQQA_PUT, mqsys::MQLONG, crate ::mapping::MQQA_PUT_CONST);
    define_new_type!(
        pub MQQA_SHAREABLE, mqsys::MQLONG, crate ::mapping::MQQA_SHAREABLE_CONST
    );
    define_new_type!(pub MQQDT, mqsys::MQLONG, crate ::mapping::MQQDT_CONST);
    define_new_type!(pub MQQFS, mqsys::MQLONG, crate ::mapping::MQQFS_CONST);
    define_new_type!(pub MQQF, mqsys::MQLONG, crate ::mapping::MQQF_CONST);
    define_new_type!(pub MQQMDT, mqsys::MQLONG, crate ::mapping::MQQMDT_CONST);
    define_new_type!(pub MQQMFAC, mqsys::MQLONG, crate ::mapping::MQQMFAC_CONST);
    define_new_type!(pub MQQMF, mqsys::MQLONG, crate ::mapping::MQQMF_CONST);
    define_new_type!(pub MQQMOPT, mqsys::MQLONG, crate ::mapping::MQQMOPT_CONST);
    define_new_type!(pub MQQMSTA, mqsys::MQLONG, crate ::mapping::MQQMSTA_CONST);
    define_new_type!(pub MQQMT, mqsys::MQLONG, crate ::mapping::MQQMT_CONST);
    define_new_type!(pub MQQO, mqsys::MQLONG, crate ::mapping::MQQO_CONST);
    define_new_type!(pub MQQSGD, mqsys::MQLONG, crate ::mapping::MQQSGD_CONST);
    define_new_type!(pub MQQSGS, mqsys::MQLONG, crate ::mapping::MQQSGS_CONST);
    define_new_type!(pub MQQSIE, mqsys::MQLONG, crate ::mapping::MQQSIE_CONST);
    define_new_type!(pub MQQSOT, mqsys::MQLONG, crate ::mapping::MQQSOT_CONST);
    define_new_type!(pub MQQSO, mqsys::MQLONG, crate ::mapping::MQQSO_CONST);
    define_new_type!(pub MQQSUM, mqsys::MQLONG, crate ::mapping::MQQSUM_CONST);
    define_new_type!(
        pub MQQT, mqsys::MQLONG, crate ::mapping::MQQT_CONST,
        "Queue Types and Extended Queue Types"
    );
    define_new_type!(pub MQRAR, mqsys::MQLONG, crate ::mapping::MQRAR_CONST);
    define_new_type!(pub MQRCCF, mqsys::MQLONG, crate ::mapping::MQRCCF_CONST);
    define_new_type!(pub MQRCN, mqsys::MQLONG, crate ::mapping::MQRCN_CONST);
    define_new_type!(pub MQRCVTIME, mqsys::MQLONG, crate ::mapping::MQRCVTIME_CONST);
    define_new_type!(
        pub MQRC, mqsys::MQLONG, crate ::mapping::MQRC_CONST,
        "Reason Code from an MQ function call"
    );
    define_new_type!(pub MQRDNS, mqsys::MQLONG, crate ::mapping::MQRDNS_CONST);
    define_new_type!(
        pub MQRD, mqsys::MQLONG, crate ::mapping::MQRD_CONST, "Reconnect delay"
    );
    define_new_type!(pub MQREADA, mqsys::MQLONG, crate ::mapping::MQREADA_CONST);
    define_new_type!(pub MQRECAUTO, mqsys::MQLONG, crate ::mapping::MQRECAUTO_CONST);
    define_new_type!(pub MQRECORDING, mqsys::MQLONG, crate ::mapping::MQRECORDING_CONST);
    define_new_type!(pub MQREGO, mqsys::MQLONG, crate ::mapping::MQREGO_CONST);
    define_new_type!(pub MQREORG, mqsys::MQLONG, crate ::mapping::MQREORG_CONST);
    define_new_type!(pub MQRFH, mqsys::MQLONG, crate ::mapping::MQRFH_CONST);
    define_new_type!(pub MQRL, mqsys::MQLONG, crate ::mapping::MQRL_CONST);
    define_new_type!(pub MQRMHF, mqsys::MQLONG, crate ::mapping::MQRMHF_CONST);
    define_new_type!(pub MQROUTE, mqsys::MQLONG, crate ::mapping::MQROUTE_CONST);
    define_new_type!(pub MQRO, mqsys::MQLONG, crate ::mapping::MQRO_CONST);
    define_new_type!(pub MQRP, mqsys::MQLONG, crate ::mapping::MQRP_CONST);
    define_new_type!(pub MQRQ, mqsys::MQLONG, crate ::mapping::MQRQ_CONST);
    define_new_type!(pub MQRT, mqsys::MQLONG, crate ::mapping::MQRT_CONST);
    define_new_type!(pub MQRU, mqsys::MQLONG, crate ::mapping::MQRU_CONST);
    define_new_type!(pub MQSCA, mqsys::MQLONG, crate ::mapping::MQSCA_CONST);
    define_new_type!(pub MQSCOPE, mqsys::MQLONG, crate ::mapping::MQSCOPE_CONST);
    define_new_type!(pub MQSCO, mqsys::MQLONG, crate ::mapping::MQSCO_CONST);
    define_new_type!(pub MQSCYC, mqsys::MQLONG, crate ::mapping::MQSCYC_CONST);
    define_new_type!(pub MQSECCOMM, mqsys::MQLONG, crate ::mapping::MQSECCOMM_CONST);
    define_new_type!(pub MQSECITEM, mqsys::MQLONG, crate ::mapping::MQSECITEM_CONST);
    define_new_type!(pub MQSECPROT, mqsys::MQLONG, crate ::mapping::MQSECPROT_CONST);
    define_new_type!(pub MQSECSW, mqsys::MQLONG, crate ::mapping::MQSECSW_CONST);
    define_new_type!(pub MQSECTYPE, mqsys::MQLONG, crate ::mapping::MQSECTYPE_CONST);
    define_new_type!(pub MQSELTYPE, mqsys::MQLONG, crate ::mapping::MQSELTYPE_CONST);
    define_new_type!(pub MQSEL_ALL, mqsys::MQLONG, crate ::mapping::MQSEL_ALL_CONST);
    define_new_type!(pub MQSEL_ANY, mqsys::MQLONG, crate ::mapping::MQSEL_ANY_CONST);
    define_new_type!(
        pub MQSMPO, mqsys::MQLONG, crate ::mapping::MQSMPO_CONST,
        "Set message property options"
    );
    define_new_type!(
        pub MQSO, mqsys::MQLONG, crate ::mapping::MQSO_CONST,
        "Options mask to control the action of `MQSUB`"
    );
    define_new_type!(pub MQSPL, mqsys::MQLONG, crate ::mapping::MQSPL_CONST);
    define_new_type!(pub MQSP, mqsys::MQLONG, crate ::mapping::MQSP_CONST);
    define_new_type!(pub MQSQQM, mqsys::MQLONG, crate ::mapping::MQSQQM_CONST);
    define_new_type!(
        pub MQSRO, mqsys::MQLONG, crate ::mapping::MQSRO_CONST,
        "Options mask that control the action of `MQSUBRQ`"
    );
    define_new_type!(
        pub MQSR, mqsys::MQLONG, crate ::mapping::MQSR_CONST,
        "Value describing action for `MQSUBRQ`"
    );
    define_new_type!(pub MQSSL, mqsys::MQLONG, crate ::mapping::MQSSL_CONST);
    define_new_type!(
        pub MQSTAT, mqsys::MQLONG, crate ::mapping::MQSTAT_CONST,
        "Value describing the MQSTAT outcome"
    );
    define_new_type!(pub MQSTDBY, mqsys::MQLONG, crate ::mapping::MQSTDBY_CONST);
    define_new_type!(pub MQST, mqsys::MQLONG, crate ::mapping::MQST_CONST);
    define_new_type!(pub MQSUBTYPE, mqsys::MQLONG, crate ::mapping::MQSUBTYPE_CONST);
    define_new_type!(pub MQSUB, mqsys::MQLONG, crate ::mapping::MQSUB_CONST);
    define_new_type!(
        pub MQSUB_DURABILITY, mqsys::MQLONG, crate ::mapping::MQSUB_DURABILITY_CONST
    );
    define_new_type!(pub MQSUS, mqsys::MQLONG, crate ::mapping::MQSUS_CONST);
    define_new_type!(
        pub MQSVC_CONTROL, mqsys::MQLONG, crate ::mapping::MQSVC_CONTROL_CONST
    );
    define_new_type!(
        pub MQSVC_STATUS, mqsys::MQLONG, crate ::mapping::MQSVC_STATUS_CONST
    );
    define_new_type!(pub MQSVC_TYPE, mqsys::MQLONG, crate ::mapping::MQSVC_TYPE_CONST);
    define_new_type!(pub MQSYNCPOINT, mqsys::MQLONG, crate ::mapping::MQSYNCPOINT_CONST);
    define_new_type!(pub MQSYSOBJ, mqsys::MQLONG, crate ::mapping::MQSYSOBJ_CONST);
    define_new_type!(pub MQSYSP, mqsys::MQLONG, crate ::mapping::MQSYSP_CONST);
    define_new_type!(pub MQS_AVAIL, mqsys::MQLONG, crate ::mapping::MQS_AVAIL_CONST);
    define_new_type!(
        pub MQS_EXPANDST, mqsys::MQLONG, crate ::mapping::MQS_EXPANDST_CONST
    );
    define_new_type!(
        pub MQS_OPENMODE, mqsys::MQLONG, crate ::mapping::MQS_OPENMODE_CONST
    );
    define_new_type!(pub MQS_STATUS, mqsys::MQLONG, crate ::mapping::MQS_STATUS_CONST);
    define_new_type!(pub MQTA, mqsys::MQLONG, crate ::mapping::MQTA_CONST);
    define_new_type!(pub MQTA_PROXY, mqsys::MQLONG, crate ::mapping::MQTA_PROXY_CONST);
    define_new_type!(pub MQTA_PUB, mqsys::MQLONG, crate ::mapping::MQTA_PUB_CONST);
    define_new_type!(pub MQTA_SUB, mqsys::MQLONG, crate ::mapping::MQTA_SUB_CONST);
    define_new_type!(pub MQTCPKEEP, mqsys::MQLONG, crate ::mapping::MQTCPKEEP_CONST);
    define_new_type!(pub MQTCPSTACK, mqsys::MQLONG, crate ::mapping::MQTCPSTACK_CONST);
    define_new_type!(pub MQTC, mqsys::MQLONG, crate ::mapping::MQTC_CONST);
    define_new_type!(pub MQTIME, mqsys::MQLONG, crate ::mapping::MQTIME_CONST);
    define_new_type!(pub MQTOPT, mqsys::MQLONG, crate ::mapping::MQTOPT_CONST);
    define_new_type!(pub MQTRAXSTR, mqsys::MQLONG, crate ::mapping::MQTRAXSTR_CONST);
    define_new_type!(pub MQTRIGGER, mqsys::MQLONG, crate ::mapping::MQTRIGGER_CONST);
    define_new_type!(pub MQTSCOPE, mqsys::MQLONG, crate ::mapping::MQTSCOPE_CONST);
    define_new_type!(pub MQTT, mqsys::MQLONG, crate ::mapping::MQTT_CONST);
    define_new_type!(
        pub MQTYPE, mqsys::MQLONG, crate ::mapping::MQTYPE_CONST, "Property data types"
    );
    define_new_type!(pub MQUCI, mqsys::MQLONG, crate ::mapping::MQUCI_CONST);
    define_new_type!(pub MQUIDSUPP, mqsys::MQLONG, crate ::mapping::MQUIDSUPP_CONST);
    define_new_type!(
        pub MQUNDELIVERED, mqsys::MQLONG, crate ::mapping::MQUNDELIVERED_CONST
    );
    define_new_type!(pub MQUOWST, mqsys::MQLONG, crate ::mapping::MQUOWST_CONST);
    define_new_type!(pub MQUOWT, mqsys::MQLONG, crate ::mapping::MQUOWT_CONST);
    define_new_type!(pub MQUSAGE_DS, mqsys::MQLONG, crate ::mapping::MQUSAGE_DS_CONST);
    define_new_type!(
        pub MQUSAGE_EXPAND, mqsys::MQLONG, crate ::mapping::MQUSAGE_EXPAND_CONST
    );
    define_new_type!(pub MQUSAGE_PS, mqsys::MQLONG, crate ::mapping::MQUSAGE_PS_CONST);
    define_new_type!(
        pub MQUSAGE_SMDS, mqsys::MQLONG, crate ::mapping::MQUSAGE_SMDS_CONST
    );
    define_new_type!(pub MQUSEDLQ, mqsys::MQLONG, crate ::mapping::MQUSEDLQ_CONST);
    define_new_type!(pub MQUSRC, mqsys::MQLONG, crate ::mapping::MQUSRC_CONST);
    define_new_type!(pub MQUS, mqsys::MQLONG, crate ::mapping::MQUS_CONST);
    define_new_type!(pub MQVL, mqsys::MQLONG, crate ::mapping::MQVL_CONST);
    define_new_type!(pub MQVS, mqsys::MQLONG, crate ::mapping::MQVS_CONST);
    define_new_type!(pub MQVU, mqsys::MQLONG, crate ::mapping::MQVU_CONST);
    define_new_type!(pub MQWARN, mqsys::MQLONG, crate ::mapping::MQWARN_CONST);
    define_new_type!(pub MQWIH, mqsys::MQLONG, crate ::mapping::MQWIH_CONST);
    define_new_type!(pub MQWI, mqsys::MQLONG, crate ::mapping::MQWI_CONST);
    define_new_type!(pub MQWS, mqsys::MQLONG, crate ::mapping::MQWS_CONST);
    define_new_type!(pub MQWXP, mqsys::MQLONG, crate ::mapping::MQWXP_CONST);
    define_new_type!(pub MQXACT, mqsys::MQLONG, crate ::mapping::MQXACT_CONST);
    define_new_type!(pub MQXCC, mqsys::MQLONG, crate ::mapping::MQXCC_CONST);
    define_new_type!(pub MQXDR, mqsys::MQLONG, crate ::mapping::MQXDR_CONST);
    define_new_type!(pub MQXEPO, mqsys::MQLONG, crate ::mapping::MQXEPO_CONST);
    define_new_type!(pub MQXE, mqsys::MQLONG, crate ::mapping::MQXE_CONST);
    define_new_type!(pub MQXF, mqsys::MQLONG, crate ::mapping::MQXF_CONST);
    define_new_type!(pub MQXPT, mqsys::MQLONG, crate ::mapping::MQXPT_CONST);
    define_new_type!(pub MQXR2, mqsys::MQLONG, crate ::mapping::MQXR2_CONST);
    define_new_type!(pub MQXR, mqsys::MQLONG, crate ::mapping::MQXR_CONST);
    define_new_type!(pub MQXT, mqsys::MQLONG, crate ::mapping::MQXT_CONST);
    define_new_type!(pub MQZAET, mqsys::MQLONG, crate ::mapping::MQZAET_CONST);
    define_new_type!(pub MQZAO, mqsys::MQLONG, crate ::mapping::MQZAO_CONST);
    define_new_type!(pub MQZAT, mqsys::MQLONG, crate ::mapping::MQZAT_CONST);
    define_new_type!(pub MQZCI, mqsys::MQLONG, crate ::mapping::MQZCI_CONST);
    define_new_type!(pub MQZID, mqsys::MQLONG, crate ::mapping::MQZID_CONST);
    define_new_type!(
        pub MQZID_AUTHORITY, mqsys::MQLONG, crate ::mapping::MQZID_AUTHORITY_CONST
    );
    define_new_type!(pub MQZID_NAME, mqsys::MQLONG, crate ::mapping::MQZID_NAME_CONST);
    define_new_type!(
        pub MQZID_USERID, mqsys::MQLONG, crate ::mapping::MQZID_USERID_CONST
    );
    define_new_type!(pub MQZIO, mqsys::MQLONG, crate ::mapping::MQZIO_CONST);
    define_new_type!(pub MQZSE, mqsys::MQLONG, crate ::mapping::MQZSE_CONST);
    define_new_type!(pub MQZSL, mqsys::MQLONG, crate ::mapping::MQZSL_CONST);
    define_new_type!(pub MQZTO, mqsys::MQLONG, crate ::mapping::MQZTO_CONST);
    define_new_type!(pub MQ_CERT, mqsys::MQLONG, crate ::mapping::MQ_CERT_CONST);
    define_new_type!(
        pub MQ_HTTPSCERTREV, mqsys::MQLONG, crate ::mapping::MQ_HTTPSCERTREV_CONST,
        "Level of certificate revocation check that is required for HTTPS connections"
    );
    define_new_type!(
        pub MQ_HTTPSCERTVAL, mqsys::MQLONG, crate ::mapping::MQ_HTTPSCERTVAL_CONST,
        "Level of certificate validation that is required for HTTPS connections"
    );
    define_new_type!(pub MQ_MQTT, mqsys::MQLONG, crate ::mapping::MQ_MQTT_CONST);
    define_new_type!(pub MQ_SUITE, mqsys::MQLONG, crate ::mapping::MQ_SUITE_CONST);
}
pub mod constants {
    use crate::types;
    use ::libmqm_sys::lib as mqsys;
    pub const MQACTIVE_NO: types::MQACTIVE = types::MQACTIVE(mqsys::MQACTIVE_NO);
    pub const MQACTIVE_YES: types::MQACTIVE = types::MQACTIVE(mqsys::MQACTIVE_YES);
    pub const MQACTP_NEW: types::MQACTP = types::MQACTP(mqsys::MQACTP_NEW);
    pub const MQACTP_FORWARD: types::MQACTP = types::MQACTP(mqsys::MQACTP_FORWARD);
    pub const MQACTP_REPLY: types::MQACTP = types::MQACTP(mqsys::MQACTP_REPLY);
    pub const MQACTP_REPORT: types::MQACTP = types::MQACTP(mqsys::MQACTP_REPORT);
    pub const MQACTV_DETAIL_LOW: types::MQACTV = types::MQACTV(mqsys::MQACTV_DETAIL_LOW);
    pub const MQACTV_DETAIL_MEDIUM: types::MQACTV = types::MQACTV(
        mqsys::MQACTV_DETAIL_MEDIUM,
    );
    pub const MQACTV_DETAIL_HIGH: types::MQACTV = types::MQACTV(
        mqsys::MQACTV_DETAIL_HIGH,
    );
    pub const MQACT_FORCE_REMOVE: types::MQACT = types::MQACT(mqsys::MQACT_FORCE_REMOVE);
    pub const MQACT_ADVANCE_LOG: types::MQACT = types::MQACT(mqsys::MQACT_ADVANCE_LOG);
    pub const MQACT_COLLECT_STATISTICS: types::MQACT = types::MQACT(
        mqsys::MQACT_COLLECT_STATISTICS,
    );
    pub const MQACT_PUBSUB: types::MQACT = types::MQACT(mqsys::MQACT_PUBSUB);
    pub const MQACT_ADD: types::MQACT = types::MQACT(mqsys::MQACT_ADD);
    pub const MQACT_REPLACE: types::MQACT = types::MQACT(mqsys::MQACT_REPLACE);
    pub const MQACT_REMOVE: types::MQACT = types::MQACT(mqsys::MQACT_REMOVE);
    pub const MQACT_REMOVEALL: types::MQACT = types::MQACT(mqsys::MQACT_REMOVEALL);
    pub const MQACT_FAIL: types::MQACT = types::MQACT(mqsys::MQACT_FAIL);
    pub const MQACT_REDUCE_LOG: types::MQACT = types::MQACT(mqsys::MQACT_REDUCE_LOG);
    pub const MQACT_ARCHIVE_LOG: types::MQACT = types::MQACT(mqsys::MQACT_ARCHIVE_LOG);
    pub const MQADOPT_CHECK_NONE: types::MQADOPT_CHECK = types::MQADOPT_CHECK(
        mqsys::MQADOPT_CHECK_NONE,
    );
    pub const MQADOPT_CHECK_ALL: types::MQADOPT_CHECK = types::MQADOPT_CHECK(
        mqsys::MQADOPT_CHECK_ALL,
    );
    pub const MQADOPT_CHECK_Q_MGR_NAME: types::MQADOPT_CHECK = types::MQADOPT_CHECK(
        mqsys::MQADOPT_CHECK_Q_MGR_NAME,
    );
    pub const MQADOPT_CHECK_NET_ADDR: types::MQADOPT_CHECK = types::MQADOPT_CHECK(
        mqsys::MQADOPT_CHECK_NET_ADDR,
    );
    pub const MQADOPT_CHECK_CHANNEL_NAME: types::MQADOPT_CHECK = types::MQADOPT_CHECK(
        mqsys::MQADOPT_CHECK_CHANNEL_NAME,
    );
    pub const MQADOPT_TYPE_NO: types::MQADOPT_TYPE = types::MQADOPT_TYPE(
        mqsys::MQADOPT_TYPE_NO,
    );
    pub const MQADOPT_TYPE_ALL: types::MQADOPT_TYPE = types::MQADOPT_TYPE(
        mqsys::MQADOPT_TYPE_ALL,
    );
    pub const MQADOPT_TYPE_SVR: types::MQADOPT_TYPE = types::MQADOPT_TYPE(
        mqsys::MQADOPT_TYPE_SVR,
    );
    pub const MQADOPT_TYPE_SDR: types::MQADOPT_TYPE = types::MQADOPT_TYPE(
        mqsys::MQADOPT_TYPE_SDR,
    );
    pub const MQADOPT_TYPE_RCVR: types::MQADOPT_TYPE = types::MQADOPT_TYPE(
        mqsys::MQADOPT_TYPE_RCVR,
    );
    pub const MQADOPT_TYPE_CLUSRCVR: types::MQADOPT_TYPE = types::MQADOPT_TYPE(
        mqsys::MQADOPT_TYPE_CLUSRCVR,
    );
    pub const MQADPCTX_NO: types::MQADPCTX = types::MQADPCTX(mqsys::MQADPCTX_NO);
    pub const MQADPCTX_YES: types::MQADPCTX = types::MQADPCTX(mqsys::MQADPCTX_YES);
    pub const MQAIT_ALL: types::MQAIT = types::MQAIT(mqsys::MQAIT_ALL);
    pub const MQAIT_CRL_LDAP: types::MQAIT = types::MQAIT(mqsys::MQAIT_CRL_LDAP);
    pub const MQAIT_OCSP: types::MQAIT = types::MQAIT(mqsys::MQAIT_OCSP);
    pub const MQAIT_IDPW_OS: types::MQAIT = types::MQAIT(mqsys::MQAIT_IDPW_OS);
    pub const MQAIT_IDPW_LDAP: types::MQAIT = types::MQAIT(mqsys::MQAIT_IDPW_LDAP);
    pub const MQAPPL_IMMOVABLE: types::MQAPPL = types::MQAPPL(mqsys::MQAPPL_IMMOVABLE);
    pub const MQAPPL_MOVABLE: types::MQAPPL = types::MQAPPL(mqsys::MQAPPL_MOVABLE);
    pub const MQAS_NONE: types::MQAS = types::MQAS(mqsys::MQAS_NONE);
    pub const MQAS_STARTED: types::MQAS = types::MQAS(mqsys::MQAS_STARTED);
    pub const MQAS_START_WAIT: types::MQAS = types::MQAS(mqsys::MQAS_START_WAIT);
    pub const MQAS_STOPPED: types::MQAS = types::MQAS(mqsys::MQAS_STOPPED);
    pub const MQAS_SUSPENDED: types::MQAS = types::MQAS(mqsys::MQAS_SUSPENDED);
    pub const MQAS_SUSPENDED_TEMPORARY: types::MQAS = types::MQAS(
        mqsys::MQAS_SUSPENDED_TEMPORARY,
    );
    pub const MQAS_ACTIVE: types::MQAS = types::MQAS(mqsys::MQAS_ACTIVE);
    pub const MQAS_INACTIVE: types::MQAS = types::MQAS(mqsys::MQAS_INACTIVE);
    pub const MQAT_UNKNOWN: types::MQAT = types::MQAT(mqsys::MQAT_UNKNOWN);
    pub const MQAT_NO_CONTEXT: types::MQAT = types::MQAT(mqsys::MQAT_NO_CONTEXT);
    pub const MQAT_CICS: types::MQAT = types::MQAT(mqsys::MQAT_CICS);
    pub const MQAT_ZOS: types::MQAT = types::MQAT(mqsys::MQAT_ZOS);
    pub const MQAT_IMS: types::MQAT = types::MQAT(mqsys::MQAT_IMS);
    pub const MQAT_OS2: types::MQAT = types::MQAT(mqsys::MQAT_OS2);
    pub const MQAT_DOS: types::MQAT = types::MQAT(mqsys::MQAT_DOS);
    pub const MQAT_UNIX: types::MQAT = types::MQAT(mqsys::MQAT_UNIX);
    pub const MQAT_QMGR: types::MQAT = types::MQAT(mqsys::MQAT_QMGR);
    pub const MQAT_OS400: types::MQAT = types::MQAT(mqsys::MQAT_OS400);
    pub const MQAT_WINDOWS: types::MQAT = types::MQAT(mqsys::MQAT_WINDOWS);
    pub const MQAT_CICS_VSE: types::MQAT = types::MQAT(mqsys::MQAT_CICS_VSE);
    pub const MQAT_WINDOWS_NT: types::MQAT = types::MQAT(mqsys::MQAT_WINDOWS_NT);
    pub const MQAT_VMS: types::MQAT = types::MQAT(mqsys::MQAT_VMS);
    pub const MQAT_NSK: types::MQAT = types::MQAT(mqsys::MQAT_NSK);
    pub const MQAT_VOS: types::MQAT = types::MQAT(mqsys::MQAT_VOS);
    pub const MQAT_OPEN_TP1: types::MQAT = types::MQAT(mqsys::MQAT_OPEN_TP1);
    pub const MQAT_VM: types::MQAT = types::MQAT(mqsys::MQAT_VM);
    pub const MQAT_IMS_BRIDGE: types::MQAT = types::MQAT(mqsys::MQAT_IMS_BRIDGE);
    pub const MQAT_XCF: types::MQAT = types::MQAT(mqsys::MQAT_XCF);
    pub const MQAT_CICS_BRIDGE: types::MQAT = types::MQAT(mqsys::MQAT_CICS_BRIDGE);
    pub const MQAT_NOTES_AGENT: types::MQAT = types::MQAT(mqsys::MQAT_NOTES_AGENT);
    pub const MQAT_TPF: types::MQAT = types::MQAT(mqsys::MQAT_TPF);
    pub const MQAT_USER: types::MQAT = types::MQAT(mqsys::MQAT_USER);
    pub const MQAT_QMGR_PUBLISH: types::MQAT = types::MQAT(mqsys::MQAT_QMGR_PUBLISH);
    pub const MQAT_JAVA: types::MQAT = types::MQAT(mqsys::MQAT_JAVA);
    pub const MQAT_DQM: types::MQAT = types::MQAT(mqsys::MQAT_DQM);
    pub const MQAT_CHANNEL_INITIATOR: types::MQAT = types::MQAT(
        mqsys::MQAT_CHANNEL_INITIATOR,
    );
    pub const MQAT_WLM: types::MQAT = types::MQAT(mqsys::MQAT_WLM);
    pub const MQAT_BATCH: types::MQAT = types::MQAT(mqsys::MQAT_BATCH);
    pub const MQAT_RRS_BATCH: types::MQAT = types::MQAT(mqsys::MQAT_RRS_BATCH);
    pub const MQAT_SIB: types::MQAT = types::MQAT(mqsys::MQAT_SIB);
    pub const MQAT_SYSTEM_EXTENSION: types::MQAT = types::MQAT(
        mqsys::MQAT_SYSTEM_EXTENSION,
    );
    pub const MQAT_MCAST_PUBLISH: types::MQAT = types::MQAT(mqsys::MQAT_MCAST_PUBLISH);
    pub const MQAT_AMQP: types::MQAT = types::MQAT(mqsys::MQAT_AMQP);
    pub const MQAT_MVS: types::MQAT = types::MQAT(mqsys::MQAT_MVS);
    pub const MQAT_OS390: types::MQAT = types::MQAT(mqsys::MQAT_OS390);
    pub const MQAT_AIX: types::MQAT = types::MQAT(mqsys::MQAT_AIX);
    pub const MQAT_DEFAULT: types::MQAT = types::MQAT(mqsys::MQAT_DEFAULT);
    pub const MQAT_GUARDIAN: types::MQAT = types::MQAT(mqsys::MQAT_GUARDIAN);
    pub const MQAT_BROKER: types::MQAT = types::MQAT(mqsys::MQAT_BROKER);
    pub const MQAUTHENTICATE_OS: types::MQAUTHENTICATE = types::MQAUTHENTICATE(
        mqsys::MQAUTHENTICATE_OS,
    );
    pub const MQAUTHENTICATE_PAM: types::MQAUTHENTICATE = types::MQAUTHENTICATE(
        mqsys::MQAUTHENTICATE_PAM,
    );
    pub const MQAUTHOPT_ENTITY_EXPLICIT: types::MQAUTHOPT = types::MQAUTHOPT(
        mqsys::MQAUTHOPT_ENTITY_EXPLICIT,
    );
    pub const MQAUTHOPT_ENTITY_SET: types::MQAUTHOPT = types::MQAUTHOPT(
        mqsys::MQAUTHOPT_ENTITY_SET,
    );
    pub const MQAUTHOPT_NAME_EXPLICIT: types::MQAUTHOPT = types::MQAUTHOPT(
        mqsys::MQAUTHOPT_NAME_EXPLICIT,
    );
    pub const MQAUTHOPT_NAME_ALL_MATCHING: types::MQAUTHOPT = types::MQAUTHOPT(
        mqsys::MQAUTHOPT_NAME_ALL_MATCHING,
    );
    pub const MQAUTHOPT_NAME_AS_WILDCARD: types::MQAUTHOPT = types::MQAUTHOPT(
        mqsys::MQAUTHOPT_NAME_AS_WILDCARD,
    );
    pub const MQAUTHOPT_CUMULATIVE: types::MQAUTHOPT = types::MQAUTHOPT(
        mqsys::MQAUTHOPT_CUMULATIVE,
    );
    pub const MQAUTHOPT_EXCLUDE_TEMP: types::MQAUTHOPT = types::MQAUTHOPT(
        mqsys::MQAUTHOPT_EXCLUDE_TEMP,
    );
    pub const MQAUTH_ALL_MQI: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_ALL_MQI);
    pub const MQAUTH_ALL_ADMIN: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_ALL_ADMIN);
    pub const MQAUTH_ALL: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_ALL);
    pub const MQAUTH_NONE: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_NONE);
    pub const MQAUTH_ALT_USER_AUTHORITY: types::MQAUTH = types::MQAUTH(
        mqsys::MQAUTH_ALT_USER_AUTHORITY,
    );
    pub const MQAUTH_BROWSE: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_BROWSE);
    pub const MQAUTH_CHANGE: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_CHANGE);
    pub const MQAUTH_CLEAR: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_CLEAR);
    pub const MQAUTH_CONNECT: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_CONNECT);
    pub const MQAUTH_CREATE: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_CREATE);
    pub const MQAUTH_DELETE: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_DELETE);
    pub const MQAUTH_DISPLAY: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_DISPLAY);
    pub const MQAUTH_INPUT: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_INPUT);
    pub const MQAUTH_INQUIRE: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_INQUIRE);
    pub const MQAUTH_OUTPUT: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_OUTPUT);
    pub const MQAUTH_PASS_ALL_CONTEXT: types::MQAUTH = types::MQAUTH(
        mqsys::MQAUTH_PASS_ALL_CONTEXT,
    );
    pub const MQAUTH_PASS_IDENTITY_CONTEXT: types::MQAUTH = types::MQAUTH(
        mqsys::MQAUTH_PASS_IDENTITY_CONTEXT,
    );
    pub const MQAUTH_SET: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_SET);
    pub const MQAUTH_SET_ALL_CONTEXT: types::MQAUTH = types::MQAUTH(
        mqsys::MQAUTH_SET_ALL_CONTEXT,
    );
    pub const MQAUTH_SET_IDENTITY_CONTEXT: types::MQAUTH = types::MQAUTH(
        mqsys::MQAUTH_SET_IDENTITY_CONTEXT,
    );
    pub const MQAUTH_CONTROL: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_CONTROL);
    pub const MQAUTH_CONTROL_EXTENDED: types::MQAUTH = types::MQAUTH(
        mqsys::MQAUTH_CONTROL_EXTENDED,
    );
    pub const MQAUTH_PUBLISH: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_PUBLISH);
    pub const MQAUTH_SUBSCRIBE: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_SUBSCRIBE);
    pub const MQAUTH_RESUME: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_RESUME);
    pub const MQAUTH_SYSTEM: types::MQAUTH = types::MQAUTH(mqsys::MQAUTH_SYSTEM);
    pub const MQAUTOCLUS_TYPE_NONE: types::MQAUTOCLUS = types::MQAUTOCLUS(
        mqsys::MQAUTOCLUS_TYPE_NONE,
    );
    pub const MQAUTOCLUS_TYPE_UNIFORM: types::MQAUTOCLUS = types::MQAUTOCLUS(
        mqsys::MQAUTOCLUS_TYPE_UNIFORM,
    );
    pub const MQAUTO_START_NO: types::MQAUTO = types::MQAUTO(mqsys::MQAUTO_START_NO);
    pub const MQAUTO_START_YES: types::MQAUTO = types::MQAUTO(mqsys::MQAUTO_START_YES);
    pub const MQBACF_EVENT_ACCOUNTING_TOKEN: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_EVENT_ACCOUNTING_TOKEN,
    );
    pub const MQBACF_EVENT_SECURITY_ID: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_EVENT_SECURITY_ID,
    );
    pub const MQBACF_RESPONSE_SET: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_RESPONSE_SET,
    );
    pub const MQBACF_RESPONSE_ID: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_RESPONSE_ID,
    );
    pub const MQBACF_EXTERNAL_UOW_ID: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_EXTERNAL_UOW_ID,
    );
    pub const MQBACF_CONNECTION_ID: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_CONNECTION_ID,
    );
    pub const MQBACF_GENERIC_CONNECTION_ID: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_GENERIC_CONNECTION_ID,
    );
    pub const MQBACF_ORIGIN_UOW_ID: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_ORIGIN_UOW_ID,
    );
    pub const MQBACF_Q_MGR_UOW_ID: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_Q_MGR_UOW_ID,
    );
    pub const MQBACF_ACCOUNTING_TOKEN: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_ACCOUNTING_TOKEN,
    );
    pub const MQBACF_CORREL_ID: types::MQBACF = types::MQBACF(mqsys::MQBACF_CORREL_ID);
    pub const MQBACF_GROUP_ID: types::MQBACF = types::MQBACF(mqsys::MQBACF_GROUP_ID);
    pub const MQBACF_MSG_ID: types::MQBACF = types::MQBACF(mqsys::MQBACF_MSG_ID);
    pub const MQBACF_CF_LEID: types::MQBACF = types::MQBACF(mqsys::MQBACF_CF_LEID);
    pub const MQBACF_DESTINATION_CORREL_ID: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_DESTINATION_CORREL_ID,
    );
    pub const MQBACF_SUB_ID: types::MQBACF = types::MQBACF(mqsys::MQBACF_SUB_ID);
    pub const MQBACF_ALTERNATE_SECURITYID: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_ALTERNATE_SECURITYID,
    );
    pub const MQBACF_MESSAGE_DATA: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_MESSAGE_DATA,
    );
    pub const MQBACF_MQBO_STRUCT: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_MQBO_STRUCT,
    );
    pub const MQBACF_MQCB_FUNCTION: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_MQCB_FUNCTION,
    );
    pub const MQBACF_MQCBC_STRUCT: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_MQCBC_STRUCT,
    );
    pub const MQBACF_MQCBD_STRUCT: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_MQCBD_STRUCT,
    );
    pub const MQBACF_MQCD_STRUCT: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_MQCD_STRUCT,
    );
    pub const MQBACF_MQCNO_STRUCT: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_MQCNO_STRUCT,
    );
    pub const MQBACF_MQGMO_STRUCT: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_MQGMO_STRUCT,
    );
    pub const MQBACF_MQMD_STRUCT: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_MQMD_STRUCT,
    );
    pub const MQBACF_MQPMO_STRUCT: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_MQPMO_STRUCT,
    );
    pub const MQBACF_MQSD_STRUCT: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_MQSD_STRUCT,
    );
    pub const MQBACF_MQSTS_STRUCT: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_MQSTS_STRUCT,
    );
    pub const MQBACF_SUB_CORREL_ID: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_SUB_CORREL_ID,
    );
    pub const MQBACF_XA_XID: types::MQBACF = types::MQBACF(mqsys::MQBACF_XA_XID);
    pub const MQBACF_XQH_CORREL_ID: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_XQH_CORREL_ID,
    );
    pub const MQBACF_XQH_MSG_ID: types::MQBACF = types::MQBACF(mqsys::MQBACF_XQH_MSG_ID);
    pub const MQBACF_REQUEST_ID: types::MQBACF = types::MQBACF(mqsys::MQBACF_REQUEST_ID);
    pub const MQBACF_PROPERTIES_DATA: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_PROPERTIES_DATA,
    );
    pub const MQBACF_CONN_TAG: types::MQBACF = types::MQBACF(mqsys::MQBACF_CONN_TAG);
    pub const MQBACF_MQBNO_STRUCT: types::MQBACF = types::MQBACF(
        mqsys::MQBACF_MQBNO_STRUCT,
    );
    pub const MQBALANCED_NO: types::MQBALANCED = types::MQBALANCED(mqsys::MQBALANCED_NO);
    pub const MQBALANCED_YES: types::MQBALANCED = types::MQBALANCED(
        mqsys::MQBALANCED_YES,
    );
    pub const MQBALANCED_NOT_APPLICABLE: types::MQBALANCED = types::MQBALANCED(
        mqsys::MQBALANCED_NOT_APPLICABLE,
    );
    pub const MQBALANCED_UNKNOWN: types::MQBALANCED = types::MQBALANCED(
        mqsys::MQBALANCED_UNKNOWN,
    );
    pub const MQBALSTATE_NOT_APPLICABLE: types::MQBALSTATE = types::MQBALSTATE(
        mqsys::MQBALSTATE_NOT_APPLICABLE,
    );
    pub const MQBALSTATE_LOW: types::MQBALSTATE = types::MQBALSTATE(
        mqsys::MQBALSTATE_LOW,
    );
    pub const MQBALSTATE_OK: types::MQBALSTATE = types::MQBALSTATE(mqsys::MQBALSTATE_OK);
    pub const MQBALSTATE_HIGH: types::MQBALSTATE = types::MQBALSTATE(
        mqsys::MQBALSTATE_HIGH,
    );
    pub const MQBALSTATE_UNKNOWN: types::MQBALSTATE = types::MQBALSTATE(
        mqsys::MQBALSTATE_UNKNOWN,
    );
    pub const MQBL_NULL_TERMINATED: types::MQBL = types::MQBL(
        mqsys::MQBL_NULL_TERMINATED,
    );
    pub const MQBMHO_NONE: types::MQBMHO = types::MQBMHO(mqsys::MQBMHO_NONE);
    pub const MQBMHO_DELETE_PROPERTIES: types::MQBMHO = types::MQBMHO(
        mqsys::MQBMHO_DELETE_PROPERTIES,
    );
    pub const MQBND_BIND_ON_OPEN: types::MQBND = types::MQBND(mqsys::MQBND_BIND_ON_OPEN);
    pub const MQBND_BIND_NOT_FIXED: types::MQBND = types::MQBND(
        mqsys::MQBND_BIND_NOT_FIXED,
    );
    pub const MQBND_BIND_ON_GROUP: types::MQBND = types::MQBND(
        mqsys::MQBND_BIND_ON_GROUP,
    );
    pub const MQBNO_BALTYPE_SIMPLE: types::MQBNO_BALTYPE = types::MQBNO_BALTYPE(
        mqsys::MQBNO_BALTYPE_SIMPLE,
    );
    pub const MQBNO_BALTYPE_REQREP: types::MQBNO_BALTYPE = types::MQBNO_BALTYPE(
        mqsys::MQBNO_BALTYPE_REQREP,
    );
    pub const MQBNO_BALTYPE_RA_MANAGED: types::MQBNO_BALTYPE = types::MQBNO_BALTYPE(
        mqsys::MQBNO_BALTYPE_RA_MANAGED,
    );
    pub const MQBNO_OPTIONS_NONE: types::MQBNO_OPTIONS = types::MQBNO_OPTIONS(
        mqsys::MQBNO_OPTIONS_NONE,
    );
    pub const MQBNO_OPTIONS_IGNORE_TRANS: types::MQBNO_OPTIONS = types::MQBNO_OPTIONS(
        mqsys::MQBNO_OPTIONS_IGNORE_TRANS,
    );
    pub const MQBNO_TIMEOUT_NEVER: types::MQBNO_TIMEOUT = types::MQBNO_TIMEOUT(
        mqsys::MQBNO_TIMEOUT_NEVER,
    );
    pub const MQBNO_TIMEOUT_AS_DEFAULT: types::MQBNO_TIMEOUT = types::MQBNO_TIMEOUT(
        mqsys::MQBNO_TIMEOUT_AS_DEFAULT,
    );
    pub const MQBNO_TIMEOUT_IMMEDIATE: types::MQBNO_TIMEOUT = types::MQBNO_TIMEOUT(
        mqsys::MQBNO_TIMEOUT_IMMEDIATE,
    );
    pub const MQBO_NONE: types::MQBO = types::MQBO(mqsys::MQBO_NONE);
    pub const MQBPLOCATION_BELOW: types::MQBPLOCATION = types::MQBPLOCATION(
        mqsys::MQBPLOCATION_BELOW,
    );
    pub const MQBPLOCATION_ABOVE: types::MQBPLOCATION = types::MQBPLOCATION(
        mqsys::MQBPLOCATION_ABOVE,
    );
    pub const MQBPLOCATION_SWITCHING_ABOVE: types::MQBPLOCATION = types::MQBPLOCATION(
        mqsys::MQBPLOCATION_SWITCHING_ABOVE,
    );
    pub const MQBPLOCATION_SWITCHING_BELOW: types::MQBPLOCATION = types::MQBPLOCATION(
        mqsys::MQBPLOCATION_SWITCHING_BELOW,
    );
    pub const MQBT_OTMA: types::MQBT = types::MQBT(mqsys::MQBT_OTMA);
    pub const MQCACF_FROM_Q_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_FROM_Q_NAME,
    );
    pub const MQCACF_TO_Q_NAME: types::MQCACF = types::MQCACF(mqsys::MQCACF_TO_Q_NAME);
    pub const MQCACF_FROM_PROCESS_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_FROM_PROCESS_NAME,
    );
    pub const MQCACF_TO_PROCESS_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TO_PROCESS_NAME,
    );
    pub const MQCACF_FROM_NAMELIST_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_FROM_NAMELIST_NAME,
    );
    pub const MQCACF_TO_NAMELIST_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TO_NAMELIST_NAME,
    );
    pub const MQCACF_FROM_CHANNEL_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_FROM_CHANNEL_NAME,
    );
    pub const MQCACF_TO_CHANNEL_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TO_CHANNEL_NAME,
    );
    pub const MQCACF_FROM_AUTH_INFO_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_FROM_AUTH_INFO_NAME,
    );
    pub const MQCACF_TO_AUTH_INFO_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TO_AUTH_INFO_NAME,
    );
    pub const MQCACF_Q_NAMES: types::MQCACF = types::MQCACF(mqsys::MQCACF_Q_NAMES);
    pub const MQCACF_PROCESS_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_PROCESS_NAMES,
    );
    pub const MQCACF_NAMELIST_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NAMELIST_NAMES,
    );
    pub const MQCACF_ESCAPE_TEXT: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_ESCAPE_TEXT,
    );
    pub const MQCACF_LOCAL_Q_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_LOCAL_Q_NAMES,
    );
    pub const MQCACF_MODEL_Q_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_MODEL_Q_NAMES,
    );
    pub const MQCACF_ALIAS_Q_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_ALIAS_Q_NAMES,
    );
    pub const MQCACF_REMOTE_Q_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_REMOTE_Q_NAMES,
    );
    pub const MQCACF_SENDER_CHANNEL_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SENDER_CHANNEL_NAMES,
    );
    pub const MQCACF_SERVER_CHANNEL_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SERVER_CHANNEL_NAMES,
    );
    pub const MQCACF_REQUESTER_CHANNEL_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_REQUESTER_CHANNEL_NAMES,
    );
    pub const MQCACF_RECEIVER_CHANNEL_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_RECEIVER_CHANNEL_NAMES,
    );
    pub const MQCACF_OBJECT_Q_MGR_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_OBJECT_Q_MGR_NAME,
    );
    pub const MQCACF_APPL_NAME: types::MQCACF = types::MQCACF(mqsys::MQCACF_APPL_NAME);
    pub const MQCACF_USER_IDENTIFIER: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_USER_IDENTIFIER,
    );
    pub const MQCACF_AUX_ERROR_DATA_STR_1: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_AUX_ERROR_DATA_STR_1,
    );
    pub const MQCACF_AUX_ERROR_DATA_STR_2: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_AUX_ERROR_DATA_STR_2,
    );
    pub const MQCACF_AUX_ERROR_DATA_STR_3: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_AUX_ERROR_DATA_STR_3,
    );
    pub const MQCACF_BRIDGE_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_BRIDGE_NAME,
    );
    pub const MQCACF_STREAM_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_STREAM_NAME,
    );
    pub const MQCACF_TOPIC: types::MQCACF = types::MQCACF(mqsys::MQCACF_TOPIC);
    pub const MQCACF_PARENT_Q_MGR_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_PARENT_Q_MGR_NAME,
    );
    pub const MQCACF_CORREL_ID: types::MQCACF = types::MQCACF(mqsys::MQCACF_CORREL_ID);
    pub const MQCACF_PUBLISH_TIMESTAMP: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_PUBLISH_TIMESTAMP,
    );
    pub const MQCACF_STRING_DATA: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_STRING_DATA,
    );
    pub const MQCACF_SUPPORTED_STREAM_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SUPPORTED_STREAM_NAME,
    );
    pub const MQCACF_REG_TOPIC: types::MQCACF = types::MQCACF(mqsys::MQCACF_REG_TOPIC);
    pub const MQCACF_REG_TIME: types::MQCACF = types::MQCACF(mqsys::MQCACF_REG_TIME);
    pub const MQCACF_REG_USER_ID: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_REG_USER_ID,
    );
    pub const MQCACF_CHILD_Q_MGR_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CHILD_Q_MGR_NAME,
    );
    pub const MQCACF_REG_STREAM_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_REG_STREAM_NAME,
    );
    pub const MQCACF_REG_Q_MGR_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_REG_Q_MGR_NAME,
    );
    pub const MQCACF_REG_Q_NAME: types::MQCACF = types::MQCACF(mqsys::MQCACF_REG_Q_NAME);
    pub const MQCACF_REG_CORREL_ID: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_REG_CORREL_ID,
    );
    pub const MQCACF_EVENT_USER_ID: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_EVENT_USER_ID,
    );
    pub const MQCACF_OBJECT_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_OBJECT_NAME,
    );
    pub const MQCACF_EVENT_Q_MGR: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_EVENT_Q_MGR,
    );
    pub const MQCACF_AUTH_INFO_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_AUTH_INFO_NAMES,
    );
    pub const MQCACF_EVENT_APPL_IDENTITY: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_EVENT_APPL_IDENTITY,
    );
    pub const MQCACF_EVENT_APPL_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_EVENT_APPL_NAME,
    );
    pub const MQCACF_EVENT_APPL_ORIGIN: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_EVENT_APPL_ORIGIN,
    );
    pub const MQCACF_SUBSCRIPTION_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SUBSCRIPTION_NAME,
    );
    pub const MQCACF_REG_SUB_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_REG_SUB_NAME,
    );
    pub const MQCACF_SUBSCRIPTION_IDENTITY: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SUBSCRIPTION_IDENTITY,
    );
    pub const MQCACF_REG_SUB_IDENTITY: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_REG_SUB_IDENTITY,
    );
    pub const MQCACF_SUBSCRIPTION_USER_DATA: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SUBSCRIPTION_USER_DATA,
    );
    pub const MQCACF_REG_SUB_USER_DATA: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_REG_SUB_USER_DATA,
    );
    pub const MQCACF_APPL_TAG: types::MQCACF = types::MQCACF(mqsys::MQCACF_APPL_TAG);
    pub const MQCACF_DATA_SET_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_DATA_SET_NAME,
    );
    pub const MQCACF_UOW_START_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_UOW_START_DATE,
    );
    pub const MQCACF_UOW_START_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_UOW_START_TIME,
    );
    pub const MQCACF_UOW_LOG_START_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_UOW_LOG_START_DATE,
    );
    pub const MQCACF_UOW_LOG_START_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_UOW_LOG_START_TIME,
    );
    pub const MQCACF_UOW_LOG_EXTENT_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_UOW_LOG_EXTENT_NAME,
    );
    pub const MQCACF_PRINCIPAL_ENTITY_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_PRINCIPAL_ENTITY_NAMES,
    );
    pub const MQCACF_GROUP_ENTITY_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_GROUP_ENTITY_NAMES,
    );
    pub const MQCACF_AUTH_PROFILE_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_AUTH_PROFILE_NAME,
    );
    pub const MQCACF_ENTITY_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_ENTITY_NAME,
    );
    pub const MQCACF_SERVICE_COMPONENT: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SERVICE_COMPONENT,
    );
    pub const MQCACF_RESPONSE_Q_MGR_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_RESPONSE_Q_MGR_NAME,
    );
    pub const MQCACF_CURRENT_LOG_EXTENT_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CURRENT_LOG_EXTENT_NAME,
    );
    pub const MQCACF_RESTART_LOG_EXTENT_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_RESTART_LOG_EXTENT_NAME,
    );
    pub const MQCACF_MEDIA_LOG_EXTENT_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_MEDIA_LOG_EXTENT_NAME,
    );
    pub const MQCACF_LOG_PATH: types::MQCACF = types::MQCACF(mqsys::MQCACF_LOG_PATH);
    pub const MQCACF_COMMAND_MQSC: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_COMMAND_MQSC,
    );
    pub const MQCACF_Q_MGR_CPF: types::MQCACF = types::MQCACF(mqsys::MQCACF_Q_MGR_CPF);
    pub const MQCACF_USAGE_LOG_RBA: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_USAGE_LOG_RBA,
    );
    pub const MQCACF_USAGE_LOG_LRSN: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_USAGE_LOG_LRSN,
    );
    pub const MQCACF_COMMAND_SCOPE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_COMMAND_SCOPE,
    );
    pub const MQCACF_ASID: types::MQCACF = types::MQCACF(mqsys::MQCACF_ASID);
    pub const MQCACF_PSB_NAME: types::MQCACF = types::MQCACF(mqsys::MQCACF_PSB_NAME);
    pub const MQCACF_PST_ID: types::MQCACF = types::MQCACF(mqsys::MQCACF_PST_ID);
    pub const MQCACF_TASK_NUMBER: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TASK_NUMBER,
    );
    pub const MQCACF_TRANSACTION_ID: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TRANSACTION_ID,
    );
    pub const MQCACF_Q_MGR_UOW_ID: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_Q_MGR_UOW_ID,
    );
    pub const MQCACF_ORIGIN_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_ORIGIN_NAME,
    );
    pub const MQCACF_ENV_INFO: types::MQCACF = types::MQCACF(mqsys::MQCACF_ENV_INFO);
    pub const MQCACF_SECURITY_PROFILE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SECURITY_PROFILE,
    );
    pub const MQCACF_CONFIGURATION_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CONFIGURATION_DATE,
    );
    pub const MQCACF_CONFIGURATION_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CONFIGURATION_TIME,
    );
    pub const MQCACF_FROM_CF_STRUC_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_FROM_CF_STRUC_NAME,
    );
    pub const MQCACF_TO_CF_STRUC_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TO_CF_STRUC_NAME,
    );
    pub const MQCACF_CF_STRUC_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CF_STRUC_NAMES,
    );
    pub const MQCACF_FAIL_DATE: types::MQCACF = types::MQCACF(mqsys::MQCACF_FAIL_DATE);
    pub const MQCACF_FAIL_TIME: types::MQCACF = types::MQCACF(mqsys::MQCACF_FAIL_TIME);
    pub const MQCACF_BACKUP_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_BACKUP_DATE,
    );
    pub const MQCACF_BACKUP_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_BACKUP_TIME,
    );
    pub const MQCACF_SYSTEM_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSTEM_NAME,
    );
    pub const MQCACF_CF_STRUC_BACKUP_START: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CF_STRUC_BACKUP_START,
    );
    pub const MQCACF_CF_STRUC_BACKUP_END: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CF_STRUC_BACKUP_END,
    );
    pub const MQCACF_CF_STRUC_LOG_Q_MGRS: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CF_STRUC_LOG_Q_MGRS,
    );
    pub const MQCACF_FROM_STORAGE_CLASS: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_FROM_STORAGE_CLASS,
    );
    pub const MQCACF_TO_STORAGE_CLASS: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TO_STORAGE_CLASS,
    );
    pub const MQCACF_STORAGE_CLASS_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_STORAGE_CLASS_NAMES,
    );
    pub const MQCACF_DSG_NAME: types::MQCACF = types::MQCACF(mqsys::MQCACF_DSG_NAME);
    pub const MQCACF_DB2_NAME: types::MQCACF = types::MQCACF(mqsys::MQCACF_DB2_NAME);
    pub const MQCACF_SYSP_CMD_USER_ID: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_CMD_USER_ID,
    );
    pub const MQCACF_SYSP_OTMA_GROUP: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_OTMA_GROUP,
    );
    pub const MQCACF_SYSP_OTMA_MEMBER: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_OTMA_MEMBER,
    );
    pub const MQCACF_SYSP_OTMA_DRU_EXIT: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_OTMA_DRU_EXIT,
    );
    pub const MQCACF_SYSP_OTMA_TPIPE_PFX: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_OTMA_TPIPE_PFX,
    );
    pub const MQCACF_SYSP_ARCHIVE_PFX1: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_ARCHIVE_PFX1,
    );
    pub const MQCACF_SYSP_ARCHIVE_UNIT1: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_ARCHIVE_UNIT1,
    );
    pub const MQCACF_SYSP_LOG_CORREL_ID: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_LOG_CORREL_ID,
    );
    pub const MQCACF_SYSP_UNIT_VOLSER: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_UNIT_VOLSER,
    );
    pub const MQCACF_SYSP_Q_MGR_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_Q_MGR_TIME,
    );
    pub const MQCACF_SYSP_Q_MGR_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_Q_MGR_DATE,
    );
    pub const MQCACF_SYSP_Q_MGR_RBA: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_Q_MGR_RBA,
    );
    pub const MQCACF_SYSP_LOG_RBA: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_LOG_RBA,
    );
    pub const MQCACF_SYSP_SERVICE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_SERVICE,
    );
    pub const MQCACF_FROM_LISTENER_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_FROM_LISTENER_NAME,
    );
    pub const MQCACF_TO_LISTENER_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TO_LISTENER_NAME,
    );
    pub const MQCACF_FROM_SERVICE_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_FROM_SERVICE_NAME,
    );
    pub const MQCACF_TO_SERVICE_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TO_SERVICE_NAME,
    );
    pub const MQCACF_LAST_PUT_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_LAST_PUT_DATE,
    );
    pub const MQCACF_LAST_PUT_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_LAST_PUT_TIME,
    );
    pub const MQCACF_LAST_GET_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_LAST_GET_DATE,
    );
    pub const MQCACF_LAST_GET_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_LAST_GET_TIME,
    );
    pub const MQCACF_OPERATION_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_OPERATION_DATE,
    );
    pub const MQCACF_OPERATION_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_OPERATION_TIME,
    );
    pub const MQCACF_ACTIVITY_DESC: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_ACTIVITY_DESC,
    );
    pub const MQCACF_APPL_IDENTITY_DATA: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_APPL_IDENTITY_DATA,
    );
    pub const MQCACF_APPL_ORIGIN_DATA: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_APPL_ORIGIN_DATA,
    );
    pub const MQCACF_PUT_DATE: types::MQCACF = types::MQCACF(mqsys::MQCACF_PUT_DATE);
    pub const MQCACF_PUT_TIME: types::MQCACF = types::MQCACF(mqsys::MQCACF_PUT_TIME);
    pub const MQCACF_REPLY_TO_Q: types::MQCACF = types::MQCACF(mqsys::MQCACF_REPLY_TO_Q);
    pub const MQCACF_REPLY_TO_Q_MGR: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_REPLY_TO_Q_MGR,
    );
    pub const MQCACF_RESOLVED_Q_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_RESOLVED_Q_NAME,
    );
    pub const MQCACF_STRUC_ID: types::MQCACF = types::MQCACF(mqsys::MQCACF_STRUC_ID);
    pub const MQCACF_VALUE_NAME: types::MQCACF = types::MQCACF(mqsys::MQCACF_VALUE_NAME);
    pub const MQCACF_SERVICE_START_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SERVICE_START_DATE,
    );
    pub const MQCACF_SERVICE_START_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SERVICE_START_TIME,
    );
    pub const MQCACF_SYSP_OFFLINE_RBA: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_OFFLINE_RBA,
    );
    pub const MQCACF_SYSP_ARCHIVE_PFX2: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_ARCHIVE_PFX2,
    );
    pub const MQCACF_SYSP_ARCHIVE_UNIT2: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SYSP_ARCHIVE_UNIT2,
    );
    pub const MQCACF_TO_TOPIC_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TO_TOPIC_NAME,
    );
    pub const MQCACF_FROM_TOPIC_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_FROM_TOPIC_NAME,
    );
    pub const MQCACF_TOPIC_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TOPIC_NAMES,
    );
    pub const MQCACF_SUB_NAME: types::MQCACF = types::MQCACF(mqsys::MQCACF_SUB_NAME);
    pub const MQCACF_DESTINATION_Q_MGR: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_DESTINATION_Q_MGR,
    );
    pub const MQCACF_DESTINATION: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_DESTINATION,
    );
    pub const MQCACF_SUB_USER_ID: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SUB_USER_ID,
    );
    pub const MQCACF_SUB_USER_DATA: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SUB_USER_DATA,
    );
    pub const MQCACF_SUB_SELECTOR: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SUB_SELECTOR,
    );
    pub const MQCACF_LAST_PUB_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_LAST_PUB_DATE,
    );
    pub const MQCACF_LAST_PUB_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_LAST_PUB_TIME,
    );
    pub const MQCACF_FROM_SUB_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_FROM_SUB_NAME,
    );
    pub const MQCACF_TO_SUB_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TO_SUB_NAME,
    );
    pub const MQCACF_LAST_MSG_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_LAST_MSG_TIME,
    );
    pub const MQCACF_LAST_MSG_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_LAST_MSG_DATE,
    );
    pub const MQCACF_SUBSCRIPTION_POINT: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SUBSCRIPTION_POINT,
    );
    pub const MQCACF_FILTER: types::MQCACF = types::MQCACF(mqsys::MQCACF_FILTER);
    pub const MQCACF_NONE: types::MQCACF = types::MQCACF(mqsys::MQCACF_NONE);
    pub const MQCACF_ADMIN_TOPIC_NAMES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_ADMIN_TOPIC_NAMES,
    );
    pub const MQCACF_ROUTING_FINGER_PRINT: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_ROUTING_FINGER_PRINT,
    );
    pub const MQCACF_APPL_DESC: types::MQCACF = types::MQCACF(mqsys::MQCACF_APPL_DESC);
    pub const MQCACF_Q_MGR_START_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_Q_MGR_START_DATE,
    );
    pub const MQCACF_Q_MGR_START_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_Q_MGR_START_TIME,
    );
    pub const MQCACF_FROM_COMM_INFO_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_FROM_COMM_INFO_NAME,
    );
    pub const MQCACF_TO_COMM_INFO_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_TO_COMM_INFO_NAME,
    );
    pub const MQCACF_CF_OFFLOAD_SIZE1: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CF_OFFLOAD_SIZE1,
    );
    pub const MQCACF_CF_OFFLOAD_SIZE2: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CF_OFFLOAD_SIZE2,
    );
    pub const MQCACF_CF_OFFLOAD_SIZE3: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CF_OFFLOAD_SIZE3,
    );
    pub const MQCACF_CF_SMDS_GENERIC_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CF_SMDS_GENERIC_NAME,
    );
    pub const MQCACF_CF_SMDS: types::MQCACF = types::MQCACF(mqsys::MQCACF_CF_SMDS);
    pub const MQCACF_RECOVERY_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_RECOVERY_DATE,
    );
    pub const MQCACF_RECOVERY_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_RECOVERY_TIME,
    );
    pub const MQCACF_CF_SMDSCONN: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CF_SMDSCONN,
    );
    pub const MQCACF_CF_STRUC_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CF_STRUC_NAME,
    );
    pub const MQCACF_ALTERNATE_USERID: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_ALTERNATE_USERID,
    );
    pub const MQCACF_CHAR_ATTRS: types::MQCACF = types::MQCACF(mqsys::MQCACF_CHAR_ATTRS);
    pub const MQCACF_DYNAMIC_Q_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_DYNAMIC_Q_NAME,
    );
    pub const MQCACF_HOST_NAME: types::MQCACF = types::MQCACF(mqsys::MQCACF_HOST_NAME);
    pub const MQCACF_MQCB_NAME: types::MQCACF = types::MQCACF(mqsys::MQCACF_MQCB_NAME);
    pub const MQCACF_OBJECT_STRING: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_OBJECT_STRING,
    );
    pub const MQCACF_RESOLVED_LOCAL_Q_MGR: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_RESOLVED_LOCAL_Q_MGR,
    );
    pub const MQCACF_RESOLVED_LOCAL_Q_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_RESOLVED_LOCAL_Q_NAME,
    );
    pub const MQCACF_RESOLVED_OBJECT_STRING: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_RESOLVED_OBJECT_STRING,
    );
    pub const MQCACF_RESOLVED_Q_MGR: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_RESOLVED_Q_MGR,
    );
    pub const MQCACF_SELECTION_STRING: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_SELECTION_STRING,
    );
    pub const MQCACF_XA_INFO: types::MQCACF = types::MQCACF(mqsys::MQCACF_XA_INFO);
    pub const MQCACF_APPL_FUNCTION: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_APPL_FUNCTION,
    );
    pub const MQCACF_XQH_REMOTE_Q_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_XQH_REMOTE_Q_NAME,
    );
    pub const MQCACF_XQH_REMOTE_Q_MGR: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_XQH_REMOTE_Q_MGR,
    );
    pub const MQCACF_XQH_PUT_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_XQH_PUT_TIME,
    );
    pub const MQCACF_XQH_PUT_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_XQH_PUT_DATE,
    );
    pub const MQCACF_EXCL_OPERATOR_MESSAGES: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_EXCL_OPERATOR_MESSAGES,
    );
    pub const MQCACF_CSP_USER_IDENTIFIER: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_CSP_USER_IDENTIFIER,
    );
    pub const MQCACF_AMQP_CLIENT_ID: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_AMQP_CLIENT_ID,
    );
    pub const MQCACF_ARCHIVE_LOG_EXTENT_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_ARCHIVE_LOG_EXTENT_NAME,
    );
    pub const MQCACF_APPL_IMMOVABLE_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_APPL_IMMOVABLE_DATE,
    );
    pub const MQCACF_APPL_IMMOVABLE_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_APPL_IMMOVABLE_TIME,
    );
    pub const MQCACF_NHA_INSTANCE_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_INSTANCE_NAME,
    );
    pub const MQCACF_Q_MGR_DATA_PATH: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_Q_MGR_DATA_PATH,
    );
    pub const MQCACF_UNIFORM_CLUSTER_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_UNIFORM_CLUSTER_NAME,
    );
    pub const MQCACF_LOG_START_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_LOG_START_DATE,
    );
    pub const MQCACF_LOG_START_LSN: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_LOG_START_LSN,
    );
    pub const MQCACF_LOG_START_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_LOG_START_TIME,
    );
    pub const MQCACF_NHA_GROUP_INITIAL_DATE: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_GROUP_INITIAL_DATE,
    );
    pub const MQCACF_NHA_GROUP_INITIAL_LSN: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_GROUP_INITIAL_LSN,
    );
    pub const MQCACF_NHA_GROUP_INITIAL_TIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_GROUP_INITIAL_TIME,
    );
    pub const MQCACF_NHA_REPL_ADDRESS: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_REPL_ADDRESS,
    );
    pub const MQCACF_DISK_WRITTEN_LSN: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_DISK_WRITTEN_LSN,
    );
    pub const MQCACF_NHA_ACKNOWLEDGED_LSN: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_ACKNOWLEDGED_LSN,
    );
    pub const MQCACF_NHA_GROUP_ADDRESS: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_GROUP_ADDRESS,
    );
    pub const MQCACF_NHA_GROUP_SYNC_ISOTIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_GROUP_SYNC_ISOTIME,
    );
    pub const MQCACF_NHA_GROUP_INIT_ISOTIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_GROUP_INIT_ISOTIME,
    );
    pub const MQCACF_NHA_GROUP_LIVE_ISOTIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_GROUP_LIVE_ISOTIME,
    );
    pub const MQCACF_NHA_GROUP_LSN: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_GROUP_LSN,
    );
    pub const MQCACF_NHA_GROUP_NAME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_GROUP_NAME,
    );
    pub const MQCACF_NHA_GROUP_RECOV_LSN: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_GROUP_RECOV_LSN,
    );
    pub const MQCACF_NHA_GROUP_RECOV_ISOTIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_GROUP_RECOV_ISOTIME,
    );
    pub const MQCACF_NHA_SYNC_ISOTIME: types::MQCACF = types::MQCACF(
        mqsys::MQCACF_NHA_SYNC_ISOTIME,
    );
    pub const MQCACH_CHANNEL_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_CHANNEL_NAME,
    );
    pub const MQCACH_DESC: types::MQCACH = types::MQCACH(mqsys::MQCACH_DESC);
    pub const MQCACH_MODE_NAME: types::MQCACH = types::MQCACH(mqsys::MQCACH_MODE_NAME);
    pub const MQCACH_TP_NAME: types::MQCACH = types::MQCACH(mqsys::MQCACH_TP_NAME);
    pub const MQCACH_XMIT_Q_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_XMIT_Q_NAME,
    );
    pub const MQCACH_CONNECTION_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_CONNECTION_NAME,
    );
    pub const MQCACH_MCA_NAME: types::MQCACH = types::MQCACH(mqsys::MQCACH_MCA_NAME);
    pub const MQCACH_SEC_EXIT_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SEC_EXIT_NAME,
    );
    pub const MQCACH_MSG_EXIT_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_MSG_EXIT_NAME,
    );
    pub const MQCACH_SEND_EXIT_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SEND_EXIT_NAME,
    );
    pub const MQCACH_RCV_EXIT_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_RCV_EXIT_NAME,
    );
    pub const MQCACH_CHANNEL_NAMES: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_CHANNEL_NAMES,
    );
    pub const MQCACH_SEC_EXIT_USER_DATA: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SEC_EXIT_USER_DATA,
    );
    pub const MQCACH_MSG_EXIT_USER_DATA: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_MSG_EXIT_USER_DATA,
    );
    pub const MQCACH_SEND_EXIT_USER_DATA: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SEND_EXIT_USER_DATA,
    );
    pub const MQCACH_RCV_EXIT_USER_DATA: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_RCV_EXIT_USER_DATA,
    );
    pub const MQCACH_USER_ID: types::MQCACH = types::MQCACH(mqsys::MQCACH_USER_ID);
    pub const MQCACH_PASSWORD: types::MQCACH = types::MQCACH(mqsys::MQCACH_PASSWORD);
    pub const MQCACH_LOCAL_ADDRESS: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_LOCAL_ADDRESS,
    );
    pub const MQCACH_LOCAL_NAME: types::MQCACH = types::MQCACH(mqsys::MQCACH_LOCAL_NAME);
    pub const MQCACH_LAST_MSG_TIME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_LAST_MSG_TIME,
    );
    pub const MQCACH_LAST_MSG_DATE: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_LAST_MSG_DATE,
    );
    pub const MQCACH_MCA_USER_ID: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_MCA_USER_ID,
    );
    pub const MQCACH_CHANNEL_START_TIME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_CHANNEL_START_TIME,
    );
    pub const MQCACH_CHANNEL_START_DATE: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_CHANNEL_START_DATE,
    );
    pub const MQCACH_MCA_JOB_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_MCA_JOB_NAME,
    );
    pub const MQCACH_LAST_LUWID: types::MQCACH = types::MQCACH(mqsys::MQCACH_LAST_LUWID);
    pub const MQCACH_CURRENT_LUWID: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_CURRENT_LUWID,
    );
    pub const MQCACH_FORMAT_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_FORMAT_NAME,
    );
    pub const MQCACH_MR_EXIT_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_MR_EXIT_NAME,
    );
    pub const MQCACH_MR_EXIT_USER_DATA: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_MR_EXIT_USER_DATA,
    );
    pub const MQCACH_SSL_CIPHER_SPEC: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SSL_CIPHER_SPEC,
    );
    pub const MQCACH_SSL_PEER_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SSL_PEER_NAME,
    );
    pub const MQCACH_SSL_HANDSHAKE_STAGE: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SSL_HANDSHAKE_STAGE,
    );
    pub const MQCACH_SSL_SHORT_PEER_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SSL_SHORT_PEER_NAME,
    );
    pub const MQCACH_REMOTE_APPL_TAG: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_REMOTE_APPL_TAG,
    );
    pub const MQCACH_SSL_CERT_USER_ID: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SSL_CERT_USER_ID,
    );
    pub const MQCACH_SSL_CERT_ISSUER_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SSL_CERT_ISSUER_NAME,
    );
    pub const MQCACH_LU_NAME: types::MQCACH = types::MQCACH(mqsys::MQCACH_LU_NAME);
    pub const MQCACH_IP_ADDRESS: types::MQCACH = types::MQCACH(mqsys::MQCACH_IP_ADDRESS);
    pub const MQCACH_TCP_NAME: types::MQCACH = types::MQCACH(mqsys::MQCACH_TCP_NAME);
    pub const MQCACH_LISTENER_NAME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_LISTENER_NAME,
    );
    pub const MQCACH_LISTENER_DESC: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_LISTENER_DESC,
    );
    pub const MQCACH_LISTENER_START_DATE: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_LISTENER_START_DATE,
    );
    pub const MQCACH_LISTENER_START_TIME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_LISTENER_START_TIME,
    );
    pub const MQCACH_SSL_KEY_RESET_DATE: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SSL_KEY_RESET_DATE,
    );
    pub const MQCACH_SSL_KEY_RESET_TIME: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SSL_KEY_RESET_TIME,
    );
    pub const MQCACH_REMOTE_VERSION: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_REMOTE_VERSION,
    );
    pub const MQCACH_REMOTE_PRODUCT: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_REMOTE_PRODUCT,
    );
    pub const MQCACH_GROUP_ADDRESS: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_GROUP_ADDRESS,
    );
    pub const MQCACH_JAAS_CONFIG: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_JAAS_CONFIG,
    );
    pub const MQCACH_CLIENT_ID: types::MQCACH = types::MQCACH(mqsys::MQCACH_CLIENT_ID);
    pub const MQCACH_SSL_KEY_PASSPHRASE: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SSL_KEY_PASSPHRASE,
    );
    pub const MQCACH_CONNECTION_NAME_LIST: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_CONNECTION_NAME_LIST,
    );
    pub const MQCACH_CLIENT_USER_ID: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_CLIENT_USER_ID,
    );
    pub const MQCACH_MCA_USER_ID_LIST: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_MCA_USER_ID_LIST,
    );
    pub const MQCACH_SSL_CIPHER_SUITE: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_SSL_CIPHER_SUITE,
    );
    pub const MQCACH_WEBCONTENT_PATH: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_WEBCONTENT_PATH,
    );
    pub const MQCACH_TOPIC_ROOT: types::MQCACH = types::MQCACH(mqsys::MQCACH_TOPIC_ROOT);
    pub const MQCACH_TEMPORARY_MODEL_Q: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_TEMPORARY_MODEL_Q,
    );
    pub const MQCACH_TEMPORARY_Q_PREFIX: types::MQCACH = types::MQCACH(
        mqsys::MQCACH_TEMPORARY_Q_PREFIX,
    );
    pub const MQCADSD_NONE: types::MQCADSD = types::MQCADSD(mqsys::MQCADSD_NONE);
    pub const MQCADSD_SEND: types::MQCADSD = types::MQCADSD(mqsys::MQCADSD_SEND);
    pub const MQCADSD_RECV: types::MQCADSD = types::MQCADSD(mqsys::MQCADSD_RECV);
    pub const MQCADSD_MSGFORMAT: types::MQCADSD = types::MQCADSD(
        mqsys::MQCADSD_MSGFORMAT,
    );
    pub const MQCAFTY_NONE: types::MQCAFTY = types::MQCAFTY(mqsys::MQCAFTY_NONE);
    pub const MQCAFTY_PREFERRED: types::MQCAFTY = types::MQCAFTY(
        mqsys::MQCAFTY_PREFERRED,
    );
    pub const MQCAMO_CLOSE_DATE: types::MQCAMO = types::MQCAMO(mqsys::MQCAMO_CLOSE_DATE);
    pub const MQCAMO_CLOSE_TIME: types::MQCAMO = types::MQCAMO(mqsys::MQCAMO_CLOSE_TIME);
    pub const MQCAMO_CONN_DATE: types::MQCAMO = types::MQCAMO(mqsys::MQCAMO_CONN_DATE);
    pub const MQCAMO_CONN_TIME: types::MQCAMO = types::MQCAMO(mqsys::MQCAMO_CONN_TIME);
    pub const MQCAMO_DISC_DATE: types::MQCAMO = types::MQCAMO(mqsys::MQCAMO_DISC_DATE);
    pub const MQCAMO_DISC_TIME: types::MQCAMO = types::MQCAMO(mqsys::MQCAMO_DISC_TIME);
    pub const MQCAMO_END_DATE: types::MQCAMO = types::MQCAMO(mqsys::MQCAMO_END_DATE);
    pub const MQCAMO_END_TIME: types::MQCAMO = types::MQCAMO(mqsys::MQCAMO_END_TIME);
    pub const MQCAMO_OPEN_DATE: types::MQCAMO = types::MQCAMO(mqsys::MQCAMO_OPEN_DATE);
    pub const MQCAMO_OPEN_TIME: types::MQCAMO = types::MQCAMO(mqsys::MQCAMO_OPEN_TIME);
    pub const MQCAMO_START_DATE: types::MQCAMO = types::MQCAMO(mqsys::MQCAMO_START_DATE);
    pub const MQCAMO_START_TIME: types::MQCAMO = types::MQCAMO(mqsys::MQCAMO_START_TIME);
    pub const MQCAMO_MONITOR_CLASS: types::MQCAMO = types::MQCAMO(
        mqsys::MQCAMO_MONITOR_CLASS,
    );
    pub const MQCAMO_MONITOR_TYPE: types::MQCAMO = types::MQCAMO(
        mqsys::MQCAMO_MONITOR_TYPE,
    );
    pub const MQCAMO_MONITOR_DESC: types::MQCAMO = types::MQCAMO(
        mqsys::MQCAMO_MONITOR_DESC,
    );
    pub const MQCAP_NOT_SUPPORTED: types::MQCAP = types::MQCAP(
        mqsys::MQCAP_NOT_SUPPORTED,
    );
    pub const MQCAP_SUPPORTED: types::MQCAP = types::MQCAP(mqsys::MQCAP_SUPPORTED);
    pub const MQCAP_EXPIRED: types::MQCAP = types::MQCAP(mqsys::MQCAP_EXPIRED);
    pub const MQCAUT_ALL: types::MQCAUT = types::MQCAUT(mqsys::MQCAUT_ALL);
    pub const MQCAUT_BLOCKUSER: types::MQCAUT = types::MQCAUT(mqsys::MQCAUT_BLOCKUSER);
    pub const MQCAUT_BLOCKADDR: types::MQCAUT = types::MQCAUT(mqsys::MQCAUT_BLOCKADDR);
    pub const MQCAUT_SSLPEERMAP: types::MQCAUT = types::MQCAUT(mqsys::MQCAUT_SSLPEERMAP);
    pub const MQCAUT_ADDRESSMAP: types::MQCAUT = types::MQCAUT(mqsys::MQCAUT_ADDRESSMAP);
    pub const MQCAUT_USERMAP: types::MQCAUT = types::MQCAUT(mqsys::MQCAUT_USERMAP);
    pub const MQCAUT_QMGRMAP: types::MQCAUT = types::MQCAUT(mqsys::MQCAUT_QMGRMAP);
    pub const MQCA_APPL_ID: types::MQCA = types::MQCA(mqsys::MQCA_APPL_ID);
    pub const MQCA_BASE_OBJECT_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_BASE_OBJECT_NAME,
    );
    pub const MQCA_COMMAND_INPUT_Q_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_COMMAND_INPUT_Q_NAME,
    );
    pub const MQCA_CREATION_DATE: types::MQCA = types::MQCA(mqsys::MQCA_CREATION_DATE);
    pub const MQCA_CREATION_TIME: types::MQCA = types::MQCA(mqsys::MQCA_CREATION_TIME);
    pub const MQCA_DEAD_LETTER_Q_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_DEAD_LETTER_Q_NAME,
    );
    pub const MQCA_ENV_DATA: types::MQCA = types::MQCA(mqsys::MQCA_ENV_DATA);
    pub const MQCA_INITIATION_Q_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_INITIATION_Q_NAME,
    );
    pub const MQCA_NAMELIST_DESC: types::MQCA = types::MQCA(mqsys::MQCA_NAMELIST_DESC);
    pub const MQCA_NAMELIST_NAME: types::MQCA = types::MQCA(mqsys::MQCA_NAMELIST_NAME);
    pub const MQCA_PROCESS_DESC: types::MQCA = types::MQCA(mqsys::MQCA_PROCESS_DESC);
    pub const MQCA_PROCESS_NAME: types::MQCA = types::MQCA(mqsys::MQCA_PROCESS_NAME);
    pub const MQCA_Q_DESC: types::MQCA = types::MQCA(mqsys::MQCA_Q_DESC);
    pub const MQCA_Q_MGR_DESC: types::MQCA = types::MQCA(mqsys::MQCA_Q_MGR_DESC);
    pub const MQCA_Q_MGR_NAME: types::MQCA = types::MQCA(mqsys::MQCA_Q_MGR_NAME);
    pub const MQCA_Q_NAME: types::MQCA = types::MQCA(mqsys::MQCA_Q_NAME);
    pub const MQCA_REMOTE_Q_MGR_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_REMOTE_Q_MGR_NAME,
    );
    pub const MQCA_REMOTE_Q_NAME: types::MQCA = types::MQCA(mqsys::MQCA_REMOTE_Q_NAME);
    pub const MQCA_BACKOUT_REQ_Q_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_BACKOUT_REQ_Q_NAME,
    );
    pub const MQCA_NAMES: types::MQCA = types::MQCA(mqsys::MQCA_NAMES);
    pub const MQCA_USER_DATA: types::MQCA = types::MQCA(mqsys::MQCA_USER_DATA);
    pub const MQCA_STORAGE_CLASS: types::MQCA = types::MQCA(mqsys::MQCA_STORAGE_CLASS);
    pub const MQCA_TRIGGER_DATA: types::MQCA = types::MQCA(mqsys::MQCA_TRIGGER_DATA);
    pub const MQCA_XMIT_Q_NAME: types::MQCA = types::MQCA(mqsys::MQCA_XMIT_Q_NAME);
    pub const MQCA_DEF_XMIT_Q_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_DEF_XMIT_Q_NAME,
    );
    pub const MQCA_CHANNEL_AUTO_DEF_EXIT: types::MQCA = types::MQCA(
        mqsys::MQCA_CHANNEL_AUTO_DEF_EXIT,
    );
    pub const MQCA_ALTERATION_DATE: types::MQCA = types::MQCA(
        mqsys::MQCA_ALTERATION_DATE,
    );
    pub const MQCA_ALTERATION_TIME: types::MQCA = types::MQCA(
        mqsys::MQCA_ALTERATION_TIME,
    );
    pub const MQCA_CLUSTER_NAME: types::MQCA = types::MQCA(mqsys::MQCA_CLUSTER_NAME);
    pub const MQCA_CLUSTER_NAMELIST: types::MQCA = types::MQCA(
        mqsys::MQCA_CLUSTER_NAMELIST,
    );
    pub const MQCA_CLUSTER_Q_MGR_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_CLUSTER_Q_MGR_NAME,
    );
    pub const MQCA_Q_MGR_IDENTIFIER: types::MQCA = types::MQCA(
        mqsys::MQCA_Q_MGR_IDENTIFIER,
    );
    pub const MQCA_CLUSTER_WORKLOAD_EXIT: types::MQCA = types::MQCA(
        mqsys::MQCA_CLUSTER_WORKLOAD_EXIT,
    );
    pub const MQCA_CLUSTER_WORKLOAD_DATA: types::MQCA = types::MQCA(
        mqsys::MQCA_CLUSTER_WORKLOAD_DATA,
    );
    pub const MQCA_REPOSITORY_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_REPOSITORY_NAME,
    );
    pub const MQCA_REPOSITORY_NAMELIST: types::MQCA = types::MQCA(
        mqsys::MQCA_REPOSITORY_NAMELIST,
    );
    pub const MQCA_CLUSTER_DATE: types::MQCA = types::MQCA(mqsys::MQCA_CLUSTER_DATE);
    pub const MQCA_CLUSTER_TIME: types::MQCA = types::MQCA(mqsys::MQCA_CLUSTER_TIME);
    pub const MQCA_CF_STRUC_NAME: types::MQCA = types::MQCA(mqsys::MQCA_CF_STRUC_NAME);
    pub const MQCA_QSG_NAME: types::MQCA = types::MQCA(mqsys::MQCA_QSG_NAME);
    pub const MQCA_IGQ_USER_ID: types::MQCA = types::MQCA(mqsys::MQCA_IGQ_USER_ID);
    pub const MQCA_STORAGE_CLASS_DESC: types::MQCA = types::MQCA(
        mqsys::MQCA_STORAGE_CLASS_DESC,
    );
    pub const MQCA_XCF_GROUP_NAME: types::MQCA = types::MQCA(mqsys::MQCA_XCF_GROUP_NAME);
    pub const MQCA_XCF_MEMBER_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_XCF_MEMBER_NAME,
    );
    pub const MQCA_AUTH_INFO_NAME: types::MQCA = types::MQCA(mqsys::MQCA_AUTH_INFO_NAME);
    pub const MQCA_AUTH_INFO_DESC: types::MQCA = types::MQCA(mqsys::MQCA_AUTH_INFO_DESC);
    pub const MQCA_LDAP_USER_NAME: types::MQCA = types::MQCA(mqsys::MQCA_LDAP_USER_NAME);
    pub const MQCA_LDAP_PASSWORD: types::MQCA = types::MQCA(mqsys::MQCA_LDAP_PASSWORD);
    pub const MQCA_SSL_KEY_REPOSITORY: types::MQCA = types::MQCA(
        mqsys::MQCA_SSL_KEY_REPOSITORY,
    );
    pub const MQCA_SSL_CRL_NAMELIST: types::MQCA = types::MQCA(
        mqsys::MQCA_SSL_CRL_NAMELIST,
    );
    pub const MQCA_SSL_CRYPTO_HARDWARE: types::MQCA = types::MQCA(
        mqsys::MQCA_SSL_CRYPTO_HARDWARE,
    );
    pub const MQCA_CF_STRUC_DESC: types::MQCA = types::MQCA(mqsys::MQCA_CF_STRUC_DESC);
    pub const MQCA_AUTH_INFO_CONN_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_AUTH_INFO_CONN_NAME,
    );
    pub const MQCA_INITIAL_KEY: types::MQCA = types::MQCA(mqsys::MQCA_INITIAL_KEY);
    pub const MQCA_SSL_KEY_REPO_PASSWORD: types::MQCA = types::MQCA(
        mqsys::MQCA_SSL_KEY_REPO_PASSWORD,
    );
    pub const MQCA_CICS_FILE_NAME: types::MQCA = types::MQCA(mqsys::MQCA_CICS_FILE_NAME);
    pub const MQCA_TRIGGER_TRANS_ID: types::MQCA = types::MQCA(
        mqsys::MQCA_TRIGGER_TRANS_ID,
    );
    pub const MQCA_TRIGGER_PROGRAM_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_TRIGGER_PROGRAM_NAME,
    );
    pub const MQCA_TRIGGER_TERM_ID: types::MQCA = types::MQCA(
        mqsys::MQCA_TRIGGER_TERM_ID,
    );
    pub const MQCA_TRIGGER_CHANNEL_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_TRIGGER_CHANNEL_NAME,
    );
    pub const MQCA_SYSTEM_LOG_Q_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_SYSTEM_LOG_Q_NAME,
    );
    pub const MQCA_MONITOR_Q_NAME: types::MQCA = types::MQCA(mqsys::MQCA_MONITOR_Q_NAME);
    pub const MQCA_COMMAND_REPLY_Q_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_COMMAND_REPLY_Q_NAME,
    );
    pub const MQCA_BATCH_INTERFACE_ID: types::MQCA = types::MQCA(
        mqsys::MQCA_BATCH_INTERFACE_ID,
    );
    pub const MQCA_SSL_KEY_LIBRARY: types::MQCA = types::MQCA(
        mqsys::MQCA_SSL_KEY_LIBRARY,
    );
    pub const MQCA_SSL_KEY_MEMBER: types::MQCA = types::MQCA(mqsys::MQCA_SSL_KEY_MEMBER);
    pub const MQCA_DNS_GROUP: types::MQCA = types::MQCA(mqsys::MQCA_DNS_GROUP);
    pub const MQCA_LU_GROUP_NAME: types::MQCA = types::MQCA(mqsys::MQCA_LU_GROUP_NAME);
    pub const MQCA_LU_NAME: types::MQCA = types::MQCA(mqsys::MQCA_LU_NAME);
    pub const MQCA_LU62_ARM_SUFFIX: types::MQCA = types::MQCA(
        mqsys::MQCA_LU62_ARM_SUFFIX,
    );
    pub const MQCA_TCP_NAME: types::MQCA = types::MQCA(mqsys::MQCA_TCP_NAME);
    pub const MQCA_CHINIT_SERVICE_PARM: types::MQCA = types::MQCA(
        mqsys::MQCA_CHINIT_SERVICE_PARM,
    );
    pub const MQCA_SERVICE_NAME: types::MQCA = types::MQCA(mqsys::MQCA_SERVICE_NAME);
    pub const MQCA_SERVICE_DESC: types::MQCA = types::MQCA(mqsys::MQCA_SERVICE_DESC);
    pub const MQCA_SERVICE_START_COMMAND: types::MQCA = types::MQCA(
        mqsys::MQCA_SERVICE_START_COMMAND,
    );
    pub const MQCA_SERVICE_START_ARGS: types::MQCA = types::MQCA(
        mqsys::MQCA_SERVICE_START_ARGS,
    );
    pub const MQCA_SERVICE_STOP_COMMAND: types::MQCA = types::MQCA(
        mqsys::MQCA_SERVICE_STOP_COMMAND,
    );
    pub const MQCA_SERVICE_STOP_ARGS: types::MQCA = types::MQCA(
        mqsys::MQCA_SERVICE_STOP_ARGS,
    );
    pub const MQCA_STDOUT_DESTINATION: types::MQCA = types::MQCA(
        mqsys::MQCA_STDOUT_DESTINATION,
    );
    pub const MQCA_STDERR_DESTINATION: types::MQCA = types::MQCA(
        mqsys::MQCA_STDERR_DESTINATION,
    );
    pub const MQCA_TPIPE_NAME: types::MQCA = types::MQCA(mqsys::MQCA_TPIPE_NAME);
    pub const MQCA_PASS_TICKET_APPL: types::MQCA = types::MQCA(
        mqsys::MQCA_PASS_TICKET_APPL,
    );
    pub const MQCA_AUTO_REORG_START_TIME: types::MQCA = types::MQCA(
        mqsys::MQCA_AUTO_REORG_START_TIME,
    );
    pub const MQCA_AUTO_REORG_CATALOG: types::MQCA = types::MQCA(
        mqsys::MQCA_AUTO_REORG_CATALOG,
    );
    pub const MQCA_TOPIC_NAME: types::MQCA = types::MQCA(mqsys::MQCA_TOPIC_NAME);
    pub const MQCA_TOPIC_DESC: types::MQCA = types::MQCA(mqsys::MQCA_TOPIC_DESC);
    pub const MQCA_TOPIC_STRING: types::MQCA = types::MQCA(mqsys::MQCA_TOPIC_STRING);
    pub const MQCA_MODEL_DURABLE_Q: types::MQCA = types::MQCA(
        mqsys::MQCA_MODEL_DURABLE_Q,
    );
    pub const MQCA_MODEL_NON_DURABLE_Q: types::MQCA = types::MQCA(
        mqsys::MQCA_MODEL_NON_DURABLE_Q,
    );
    pub const MQCA_RESUME_DATE: types::MQCA = types::MQCA(mqsys::MQCA_RESUME_DATE);
    pub const MQCA_RESUME_TIME: types::MQCA = types::MQCA(mqsys::MQCA_RESUME_TIME);
    pub const MQCA_CHILD: types::MQCA = types::MQCA(mqsys::MQCA_CHILD);
    pub const MQCA_PARENT: types::MQCA = types::MQCA(mqsys::MQCA_PARENT);
    pub const MQCA_ADMIN_TOPIC_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_ADMIN_TOPIC_NAME,
    );
    pub const MQCA_TOPIC_STRING_FILTER: types::MQCA = types::MQCA(
        mqsys::MQCA_TOPIC_STRING_FILTER,
    );
    pub const MQCA_AUTH_INFO_OCSP_URL: types::MQCA = types::MQCA(
        mqsys::MQCA_AUTH_INFO_OCSP_URL,
    );
    pub const MQCA_COMM_INFO_NAME: types::MQCA = types::MQCA(mqsys::MQCA_COMM_INFO_NAME);
    pub const MQCA_COMM_INFO_DESC: types::MQCA = types::MQCA(mqsys::MQCA_COMM_INFO_DESC);
    pub const MQCA_POLICY_NAME: types::MQCA = types::MQCA(mqsys::MQCA_POLICY_NAME);
    pub const MQCA_SIGNER_DN: types::MQCA = types::MQCA(mqsys::MQCA_SIGNER_DN);
    pub const MQCA_RECIPIENT_DN: types::MQCA = types::MQCA(mqsys::MQCA_RECIPIENT_DN);
    pub const MQCA_INSTALLATION_DESC: types::MQCA = types::MQCA(
        mqsys::MQCA_INSTALLATION_DESC,
    );
    pub const MQCA_INSTALLATION_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_INSTALLATION_NAME,
    );
    pub const MQCA_INSTALLATION_PATH: types::MQCA = types::MQCA(
        mqsys::MQCA_INSTALLATION_PATH,
    );
    pub const MQCA_CHLAUTH_DESC: types::MQCA = types::MQCA(mqsys::MQCA_CHLAUTH_DESC);
    pub const MQCA_CUSTOM: types::MQCA = types::MQCA(mqsys::MQCA_CUSTOM);
    pub const MQCA_VERSION: types::MQCA = types::MQCA(mqsys::MQCA_VERSION);
    pub const MQCA_CERT_LABEL: types::MQCA = types::MQCA(mqsys::MQCA_CERT_LABEL);
    pub const MQCA_XR_VERSION: types::MQCA = types::MQCA(mqsys::MQCA_XR_VERSION);
    pub const MQCA_XR_SSL_CIPHER_SUITES: types::MQCA = types::MQCA(
        mqsys::MQCA_XR_SSL_CIPHER_SUITES,
    );
    pub const MQCA_CLUS_CHL_NAME: types::MQCA = types::MQCA(mqsys::MQCA_CLUS_CHL_NAME);
    pub const MQCA_CONN_AUTH: types::MQCA = types::MQCA(mqsys::MQCA_CONN_AUTH);
    pub const MQCA_LDAP_BASE_DN_USERS: types::MQCA = types::MQCA(
        mqsys::MQCA_LDAP_BASE_DN_USERS,
    );
    pub const MQCA_LDAP_SHORT_USER_FIELD: types::MQCA = types::MQCA(
        mqsys::MQCA_LDAP_SHORT_USER_FIELD,
    );
    pub const MQCA_LDAP_USER_OBJECT_CLASS: types::MQCA = types::MQCA(
        mqsys::MQCA_LDAP_USER_OBJECT_CLASS,
    );
    pub const MQCA_LDAP_USER_ATTR_FIELD: types::MQCA = types::MQCA(
        mqsys::MQCA_LDAP_USER_ATTR_FIELD,
    );
    pub const MQCA_SSL_CERT_ISSUER_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_SSL_CERT_ISSUER_NAME,
    );
    pub const MQCA_QSG_CERT_LABEL: types::MQCA = types::MQCA(mqsys::MQCA_QSG_CERT_LABEL);
    pub const MQCA_LDAP_BASE_DN_GROUPS: types::MQCA = types::MQCA(
        mqsys::MQCA_LDAP_BASE_DN_GROUPS,
    );
    pub const MQCA_LDAP_GROUP_OBJECT_CLASS: types::MQCA = types::MQCA(
        mqsys::MQCA_LDAP_GROUP_OBJECT_CLASS,
    );
    pub const MQCA_LDAP_GROUP_ATTR_FIELD: types::MQCA = types::MQCA(
        mqsys::MQCA_LDAP_GROUP_ATTR_FIELD,
    );
    pub const MQCA_LDAP_FIND_GROUP_FIELD: types::MQCA = types::MQCA(
        mqsys::MQCA_LDAP_FIND_GROUP_FIELD,
    );
    pub const MQCA_AMQP_VERSION: types::MQCA = types::MQCA(mqsys::MQCA_AMQP_VERSION);
    pub const MQCA_AMQP_SSL_CIPHER_SUITES: types::MQCA = types::MQCA(
        mqsys::MQCA_AMQP_SSL_CIPHER_SUITES,
    );
    pub const MQCA_STREAM_QUEUE_NAME: types::MQCA = types::MQCA(
        mqsys::MQCA_STREAM_QUEUE_NAME,
    );
    pub const MQCA_USER_LIST: types::MQCA = types::MQCA(mqsys::MQCA_USER_LIST);
    pub const MQCA_BASE_Q_NAME: types::MQCA = types::MQCA(mqsys::MQCA_BASE_Q_NAME);
    pub const MQCBCF_NONE: types::MQCBCF = types::MQCBCF(mqsys::MQCBCF_NONE);
    pub const MQCBCF_READA_BUFFER_EMPTY: types::MQCBCF = types::MQCBCF(
        mqsys::MQCBCF_READA_BUFFER_EMPTY,
    );
    pub const MQCBCT_START_CALL: types::MQCBCT = types::MQCBCT(mqsys::MQCBCT_START_CALL);
    pub const MQCBCT_STOP_CALL: types::MQCBCT = types::MQCBCT(mqsys::MQCBCT_STOP_CALL);
    pub const MQCBCT_REGISTER_CALL: types::MQCBCT = types::MQCBCT(
        mqsys::MQCBCT_REGISTER_CALL,
    );
    pub const MQCBCT_DEREGISTER_CALL: types::MQCBCT = types::MQCBCT(
        mqsys::MQCBCT_DEREGISTER_CALL,
    );
    pub const MQCBCT_EVENT_CALL: types::MQCBCT = types::MQCBCT(mqsys::MQCBCT_EVENT_CALL);
    pub const MQCBCT_MSG_REMOVED: types::MQCBCT = types::MQCBCT(
        mqsys::MQCBCT_MSG_REMOVED,
    );
    pub const MQCBCT_MSG_NOT_REMOVED: types::MQCBCT = types::MQCBCT(
        mqsys::MQCBCT_MSG_NOT_REMOVED,
    );
    pub const MQCBCT_MC_EVENT_CALL: types::MQCBCT = types::MQCBCT(
        mqsys::MQCBCT_MC_EVENT_CALL,
    );
    pub const MQCBDO_NONE: types::MQCBDO = types::MQCBDO(mqsys::MQCBDO_NONE);
    pub const MQCBDO_START_CALL: types::MQCBDO = types::MQCBDO(mqsys::MQCBDO_START_CALL);
    pub const MQCBDO_STOP_CALL: types::MQCBDO = types::MQCBDO(mqsys::MQCBDO_STOP_CALL);
    pub const MQCBDO_REGISTER_CALL: types::MQCBDO = types::MQCBDO(
        mqsys::MQCBDO_REGISTER_CALL,
    );
    pub const MQCBDO_DEREGISTER_CALL: types::MQCBDO = types::MQCBDO(
        mqsys::MQCBDO_DEREGISTER_CALL,
    );
    pub const MQCBDO_FAIL_IF_QUIESCING: types::MQCBDO = types::MQCBDO(
        mqsys::MQCBDO_FAIL_IF_QUIESCING,
    );
    pub const MQCBDO_EVENT_CALL: types::MQCBDO = types::MQCBDO(mqsys::MQCBDO_EVENT_CALL);
    pub const MQCBDO_MC_EVENT_CALL: types::MQCBDO = types::MQCBDO(
        mqsys::MQCBDO_MC_EVENT_CALL,
    );
    pub const MQCBD_FULL_MSG_LENGTH: types::MQCBD = types::MQCBD(
        mqsys::MQCBD_FULL_MSG_LENGTH,
    );
    pub const MQCBO_NONE: types::MQCBO = types::MQCBO(mqsys::MQCBO_NONE);
    pub const MQCBO_ADMIN_BAG: types::MQCBO = types::MQCBO(mqsys::MQCBO_ADMIN_BAG);
    pub const MQCBO_LIST_FORM_ALLOWED: types::MQCBO = types::MQCBO(
        mqsys::MQCBO_LIST_FORM_ALLOWED,
    );
    pub const MQCBO_REORDER_AS_REQUIRED: types::MQCBO = types::MQCBO(
        mqsys::MQCBO_REORDER_AS_REQUIRED,
    );
    pub const MQCBO_CHECK_SELECTORS: types::MQCBO = types::MQCBO(
        mqsys::MQCBO_CHECK_SELECTORS,
    );
    pub const MQCBO_COMMAND_BAG: types::MQCBO = types::MQCBO(mqsys::MQCBO_COMMAND_BAG);
    pub const MQCBO_SYSTEM_BAG: types::MQCBO = types::MQCBO(mqsys::MQCBO_SYSTEM_BAG);
    pub const MQCBO_GROUP_BAG: types::MQCBO = types::MQCBO(mqsys::MQCBO_GROUP_BAG);
    pub const MQCBO_DO_NOT_CHECK_SELECTORS: types::MQCBO = types::MQCBO(
        mqsys::MQCBO_DO_NOT_CHECK_SELECTORS,
    );
    pub const MQCBO_DO_NOT_REORDER: types::MQCBO = types::MQCBO(
        mqsys::MQCBO_DO_NOT_REORDER,
    );
    pub const MQCBO_LIST_FORM_INHIBITED: types::MQCBO = types::MQCBO(
        mqsys::MQCBO_LIST_FORM_INHIBITED,
    );
    pub const MQCBO_USER_BAG: types::MQCBO = types::MQCBO(mqsys::MQCBO_USER_BAG);
    pub const MQCBT_MESSAGE_CONSUMER: types::MQCBT = types::MQCBT(
        mqsys::MQCBT_MESSAGE_CONSUMER,
    );
    pub const MQCBT_EVENT_HANDLER: types::MQCBT = types::MQCBT(
        mqsys::MQCBT_EVENT_HANDLER,
    );
    pub const MQCCSI_AS_PUBLISHED: types::MQCCSI = types::MQCCSI(
        mqsys::MQCCSI_AS_PUBLISHED,
    );
    pub const MQCCSI_APPL: types::MQCCSI = types::MQCCSI(mqsys::MQCCSI_APPL);
    pub const MQCCSI_INHERIT: types::MQCCSI = types::MQCCSI(mqsys::MQCCSI_INHERIT);
    pub const MQCCSI_EMBEDDED: types::MQCCSI = types::MQCCSI(mqsys::MQCCSI_EMBEDDED);
    pub const MQCCSI_DEFAULT: types::MQCCSI = types::MQCCSI(mqsys::MQCCSI_DEFAULT);
    pub const MQCCSI_Q_MGR: types::MQCCSI = types::MQCCSI(mqsys::MQCCSI_Q_MGR);
    pub const MQCCSI_UNDEFINED: types::MQCCSI = types::MQCCSI(mqsys::MQCCSI_UNDEFINED);
    pub const MQCCT_NO: types::MQCCT = types::MQCCT(mqsys::MQCCT_NO);
    pub const MQCCT_YES: types::MQCCT = types::MQCCT(mqsys::MQCCT_YES);
    pub const MQCC_UNKNOWN: types::MQCC = types::MQCC(mqsys::MQCC_UNKNOWN);
    pub const MQCC_OK: types::MQCC = types::MQCC(mqsys::MQCC_OK);
    pub const MQCC_WARNING: types::MQCC = types::MQCC(mqsys::MQCC_WARNING);
    pub const MQCC_FAILED: types::MQCC = types::MQCC(mqsys::MQCC_FAILED);
    pub const MQCDC_NO_SENDER_CONVERSION: types::MQCDC = types::MQCDC(
        mqsys::MQCDC_NO_SENDER_CONVERSION,
    );
    pub const MQCDC_SENDER_CONVERSION: types::MQCDC = types::MQCDC(
        mqsys::MQCDC_SENDER_CONVERSION,
    );
    pub const MQCEX_AS_PARENT: types::MQCEX = types::MQCEX(mqsys::MQCEX_AS_PARENT);
    pub const MQCEX_NOLIMIT: types::MQCEX = types::MQCEX(mqsys::MQCEX_NOLIMIT);
    pub const MQCFACCESS_ENABLED: types::MQCFACCESS = types::MQCFACCESS(
        mqsys::MQCFACCESS_ENABLED,
    );
    pub const MQCFACCESS_SUSPENDED: types::MQCFACCESS = types::MQCFACCESS(
        mqsys::MQCFACCESS_SUSPENDED,
    );
    pub const MQCFACCESS_DISABLED: types::MQCFACCESS = types::MQCFACCESS(
        mqsys::MQCFACCESS_DISABLED,
    );
    pub const MQCFCONLOS_TERMINATE: types::MQCFCONLOS = types::MQCFCONLOS(
        mqsys::MQCFCONLOS_TERMINATE,
    );
    pub const MQCFCONLOS_TOLERATE: types::MQCFCONLOS = types::MQCFCONLOS(
        mqsys::MQCFCONLOS_TOLERATE,
    );
    pub const MQCFCONLOS_ASQMGR: types::MQCFCONLOS = types::MQCFCONLOS(
        mqsys::MQCFCONLOS_ASQMGR,
    );
    pub const MQCFC_NOT_LAST: types::MQCFC = types::MQCFC(mqsys::MQCFC_NOT_LAST);
    pub const MQCFC_LAST: types::MQCFC = types::MQCFC(mqsys::MQCFC_LAST);
    pub const MQCFOFFLD_NONE: types::MQCFOFFLD = types::MQCFOFFLD(mqsys::MQCFOFFLD_NONE);
    pub const MQCFOFFLD_SMDS: types::MQCFOFFLD = types::MQCFOFFLD(mqsys::MQCFOFFLD_SMDS);
    pub const MQCFOFFLD_DB2: types::MQCFOFFLD = types::MQCFOFFLD(mqsys::MQCFOFFLD_DB2);
    pub const MQCFOFFLD_BOTH: types::MQCFOFFLD = types::MQCFOFFLD(mqsys::MQCFOFFLD_BOTH);
    pub const MQCFOP_LESS: types::MQCFOP = types::MQCFOP(mqsys::MQCFOP_LESS);
    pub const MQCFOP_EQUAL: types::MQCFOP = types::MQCFOP(mqsys::MQCFOP_EQUAL);
    pub const MQCFOP_NOT_GREATER: types::MQCFOP = types::MQCFOP(
        mqsys::MQCFOP_NOT_GREATER,
    );
    pub const MQCFOP_GREATER: types::MQCFOP = types::MQCFOP(mqsys::MQCFOP_GREATER);
    pub const MQCFOP_NOT_EQUAL: types::MQCFOP = types::MQCFOP(mqsys::MQCFOP_NOT_EQUAL);
    pub const MQCFOP_NOT_LESS: types::MQCFOP = types::MQCFOP(mqsys::MQCFOP_NOT_LESS);
    pub const MQCFOP_CONTAINS: types::MQCFOP = types::MQCFOP(mqsys::MQCFOP_CONTAINS);
    pub const MQCFOP_EXCLUDES: types::MQCFOP = types::MQCFOP(mqsys::MQCFOP_EXCLUDES);
    pub const MQCFOP_LIKE: types::MQCFOP = types::MQCFOP(mqsys::MQCFOP_LIKE);
    pub const MQCFOP_NOT_LIKE: types::MQCFOP = types::MQCFOP(mqsys::MQCFOP_NOT_LIKE);
    pub const MQCFOP_CONTAINS_GEN: types::MQCFOP = types::MQCFOP(
        mqsys::MQCFOP_CONTAINS_GEN,
    );
    pub const MQCFOP_EXCLUDES_GEN: types::MQCFOP = types::MQCFOP(
        mqsys::MQCFOP_EXCLUDES_GEN,
    );
    pub const MQCFO_REFRESH_REPOSITORY_NO: types::MQCFO_REFRESH = types::MQCFO_REFRESH(
        mqsys::MQCFO_REFRESH_REPOSITORY_NO,
    );
    pub const MQCFO_REFRESH_REPOSITORY_YES: types::MQCFO_REFRESH = types::MQCFO_REFRESH(
        mqsys::MQCFO_REFRESH_REPOSITORY_YES,
    );
    pub const MQCFO_REMOVE_QUEUES_NO: types::MQCFO_REMOVE = types::MQCFO_REMOVE(
        mqsys::MQCFO_REMOVE_QUEUES_NO,
    );
    pub const MQCFO_REMOVE_QUEUES_YES: types::MQCFO_REMOVE = types::MQCFO_REMOVE(
        mqsys::MQCFO_REMOVE_QUEUES_YES,
    );
    pub const MQCFR_NO: types::MQCFR = types::MQCFR(mqsys::MQCFR_NO);
    pub const MQCFR_YES: types::MQCFR = types::MQCFR(mqsys::MQCFR_YES);
    pub const MQCFSTATUS_NOT_FOUND: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_NOT_FOUND,
    );
    pub const MQCFSTATUS_ACTIVE: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_ACTIVE,
    );
    pub const MQCFSTATUS_IN_RECOVER: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_IN_RECOVER,
    );
    pub const MQCFSTATUS_IN_BACKUP: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_IN_BACKUP,
    );
    pub const MQCFSTATUS_FAILED: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_FAILED,
    );
    pub const MQCFSTATUS_NONE: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_NONE,
    );
    pub const MQCFSTATUS_UNKNOWN: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_UNKNOWN,
    );
    pub const MQCFSTATUS_RECOVERED: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_RECOVERED,
    );
    pub const MQCFSTATUS_EMPTY: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_EMPTY,
    );
    pub const MQCFSTATUS_NEW: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_NEW,
    );
    pub const MQCFSTATUS_ADMIN_INCOMPLETE: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_ADMIN_INCOMPLETE,
    );
    pub const MQCFSTATUS_NEVER_USED: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_NEVER_USED,
    );
    pub const MQCFSTATUS_NO_BACKUP: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_NO_BACKUP,
    );
    pub const MQCFSTATUS_NOT_FAILED: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_NOT_FAILED,
    );
    pub const MQCFSTATUS_NOT_RECOVERABLE: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_NOT_RECOVERABLE,
    );
    pub const MQCFSTATUS_XES_ERROR: types::MQCFSTATUS = types::MQCFSTATUS(
        mqsys::MQCFSTATUS_XES_ERROR,
    );
    pub const MQCFTYPE_APPL: types::MQCFTYPE = types::MQCFTYPE(mqsys::MQCFTYPE_APPL);
    pub const MQCFTYPE_ADMIN: types::MQCFTYPE = types::MQCFTYPE(mqsys::MQCFTYPE_ADMIN);
    pub const MQCFT_NONE: types::MQCFT = types::MQCFT(mqsys::MQCFT_NONE);
    pub const MQCFT_COMMAND: types::MQCFT = types::MQCFT(mqsys::MQCFT_COMMAND);
    pub const MQCFT_RESPONSE: types::MQCFT = types::MQCFT(mqsys::MQCFT_RESPONSE);
    pub const MQCFT_INTEGER: types::MQCFT = types::MQCFT(mqsys::MQCFT_INTEGER);
    pub const MQCFT_STRING: types::MQCFT = types::MQCFT(mqsys::MQCFT_STRING);
    pub const MQCFT_INTEGER_LIST: types::MQCFT = types::MQCFT(mqsys::MQCFT_INTEGER_LIST);
    pub const MQCFT_STRING_LIST: types::MQCFT = types::MQCFT(mqsys::MQCFT_STRING_LIST);
    pub const MQCFT_EVENT: types::MQCFT = types::MQCFT(mqsys::MQCFT_EVENT);
    pub const MQCFT_USER: types::MQCFT = types::MQCFT(mqsys::MQCFT_USER);
    pub const MQCFT_BYTE_STRING: types::MQCFT = types::MQCFT(mqsys::MQCFT_BYTE_STRING);
    pub const MQCFT_TRACE_ROUTE: types::MQCFT = types::MQCFT(mqsys::MQCFT_TRACE_ROUTE);
    pub const MQCFT_REPORT: types::MQCFT = types::MQCFT(mqsys::MQCFT_REPORT);
    pub const MQCFT_INTEGER_FILTER: types::MQCFT = types::MQCFT(
        mqsys::MQCFT_INTEGER_FILTER,
    );
    pub const MQCFT_STRING_FILTER: types::MQCFT = types::MQCFT(
        mqsys::MQCFT_STRING_FILTER,
    );
    pub const MQCFT_BYTE_STRING_FILTER: types::MQCFT = types::MQCFT(
        mqsys::MQCFT_BYTE_STRING_FILTER,
    );
    pub const MQCFT_COMMAND_XR: types::MQCFT = types::MQCFT(mqsys::MQCFT_COMMAND_XR);
    pub const MQCFT_XR_MSG: types::MQCFT = types::MQCFT(mqsys::MQCFT_XR_MSG);
    pub const MQCFT_XR_ITEM: types::MQCFT = types::MQCFT(mqsys::MQCFT_XR_ITEM);
    pub const MQCFT_XR_SUMMARY: types::MQCFT = types::MQCFT(mqsys::MQCFT_XR_SUMMARY);
    pub const MQCFT_GROUP: types::MQCFT = types::MQCFT(mqsys::MQCFT_GROUP);
    pub const MQCFT_STATISTICS: types::MQCFT = types::MQCFT(mqsys::MQCFT_STATISTICS);
    pub const MQCFT_ACCOUNTING: types::MQCFT = types::MQCFT(mqsys::MQCFT_ACCOUNTING);
    pub const MQCFT_INTEGER64: types::MQCFT = types::MQCFT(mqsys::MQCFT_INTEGER64);
    pub const MQCFT_INTEGER64_LIST: types::MQCFT = types::MQCFT(
        mqsys::MQCFT_INTEGER64_LIST,
    );
    pub const MQCFT_APP_ACTIVITY: types::MQCFT = types::MQCFT(mqsys::MQCFT_APP_ACTIVITY);
    pub const MQCFT_STATUS: types::MQCFT = types::MQCFT(mqsys::MQCFT_STATUS);
    pub const MQCF_NONE: types::MQCF = types::MQCF(mqsys::MQCF_NONE);
    pub const MQCF_DIST_LISTS: types::MQCF = types::MQCF(mqsys::MQCF_DIST_LISTS);
    pub const MQCGWI_DEFAULT: types::MQCGWI = types::MQCGWI(mqsys::MQCGWI_DEFAULT);
    pub const MQCHAD_DISABLED: types::MQCHAD = types::MQCHAD(mqsys::MQCHAD_DISABLED);
    pub const MQCHAD_ENABLED: types::MQCHAD = types::MQCHAD(mqsys::MQCHAD_ENABLED);
    pub const MQCHIDS_NOT_INDOUBT: types::MQCHIDS = types::MQCHIDS(
        mqsys::MQCHIDS_NOT_INDOUBT,
    );
    pub const MQCHIDS_INDOUBT: types::MQCHIDS = types::MQCHIDS(mqsys::MQCHIDS_INDOUBT);
    pub const MQCHK_OPTIONAL: types::MQCHK = types::MQCHK(mqsys::MQCHK_OPTIONAL);
    pub const MQCHK_NONE: types::MQCHK = types::MQCHK(mqsys::MQCHK_NONE);
    pub const MQCHK_REQUIRED_ADMIN: types::MQCHK = types::MQCHK(
        mqsys::MQCHK_REQUIRED_ADMIN,
    );
    pub const MQCHK_REQUIRED: types::MQCHK = types::MQCHK(mqsys::MQCHK_REQUIRED);
    pub const MQCHK_AS_Q_MGR: types::MQCHK = types::MQCHK(mqsys::MQCHK_AS_Q_MGR);
    pub const MQCHLA_DISABLED: types::MQCHLA = types::MQCHLA(mqsys::MQCHLA_DISABLED);
    pub const MQCHLA_ENABLED: types::MQCHLA = types::MQCHLA(mqsys::MQCHLA_ENABLED);
    pub const MQCHLD_ALL: types::MQCHLD = types::MQCHLD(mqsys::MQCHLD_ALL);
    pub const MQCHLD_DEFAULT: types::MQCHLD = types::MQCHLD(mqsys::MQCHLD_DEFAULT);
    pub const MQCHLD_SHARED: types::MQCHLD = types::MQCHLD(mqsys::MQCHLD_SHARED);
    pub const MQCHLD_PRIVATE: types::MQCHLD = types::MQCHLD(mqsys::MQCHLD_PRIVATE);
    pub const MQCHLD_FIXSHARED: types::MQCHLD = types::MQCHLD(mqsys::MQCHLD_FIXSHARED);
    pub const MQCHRR_RESET_NOT_REQUESTED: types::MQCHRR = types::MQCHRR(
        mqsys::MQCHRR_RESET_NOT_REQUESTED,
    );
    pub const MQCHSH_RESTART_NO: types::MQCHSH = types::MQCHSH(mqsys::MQCHSH_RESTART_NO);
    pub const MQCHSH_RESTART_YES: types::MQCHSH = types::MQCHSH(
        mqsys::MQCHSH_RESTART_YES,
    );
    pub const MQCHSR_STOP_NOT_REQUESTED: types::MQCHSR = types::MQCHSR(
        mqsys::MQCHSR_STOP_NOT_REQUESTED,
    );
    pub const MQCHSR_STOP_REQUESTED: types::MQCHSR = types::MQCHSR(
        mqsys::MQCHSR_STOP_REQUESTED,
    );
    pub const MQCHSSTATE_OTHER: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_OTHER,
    );
    pub const MQCHSSTATE_END_OF_BATCH: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_END_OF_BATCH,
    );
    pub const MQCHSSTATE_SENDING: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_SENDING,
    );
    pub const MQCHSSTATE_RECEIVING: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_RECEIVING,
    );
    pub const MQCHSSTATE_SERIALIZING: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_SERIALIZING,
    );
    pub const MQCHSSTATE_RESYNCHING: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_RESYNCHING,
    );
    pub const MQCHSSTATE_HEARTBEATING: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_HEARTBEATING,
    );
    pub const MQCHSSTATE_IN_SCYEXIT: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_IN_SCYEXIT,
    );
    pub const MQCHSSTATE_IN_RCVEXIT: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_IN_RCVEXIT,
    );
    pub const MQCHSSTATE_IN_SENDEXIT: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_IN_SENDEXIT,
    );
    pub const MQCHSSTATE_IN_MSGEXIT: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_IN_MSGEXIT,
    );
    pub const MQCHSSTATE_IN_MREXIT: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_IN_MREXIT,
    );
    pub const MQCHSSTATE_IN_CHADEXIT: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_IN_CHADEXIT,
    );
    pub const MQCHSSTATE_NET_CONNECTING: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_NET_CONNECTING,
    );
    pub const MQCHSSTATE_SSL_HANDSHAKING: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_SSL_HANDSHAKING,
    );
    pub const MQCHSSTATE_NAME_SERVER: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_NAME_SERVER,
    );
    pub const MQCHSSTATE_IN_MQPUT: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_IN_MQPUT,
    );
    pub const MQCHSSTATE_IN_MQGET: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_IN_MQGET,
    );
    pub const MQCHSSTATE_IN_MQI_CALL: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_IN_MQI_CALL,
    );
    pub const MQCHSSTATE_COMPRESSING: types::MQCHSSTATE = types::MQCHSSTATE(
        mqsys::MQCHSSTATE_COMPRESSING,
    );
    pub const MQCHS_INACTIVE: types::MQCHS = types::MQCHS(mqsys::MQCHS_INACTIVE);
    pub const MQCHS_BINDING: types::MQCHS = types::MQCHS(mqsys::MQCHS_BINDING);
    pub const MQCHS_STARTING: types::MQCHS = types::MQCHS(mqsys::MQCHS_STARTING);
    pub const MQCHS_RUNNING: types::MQCHS = types::MQCHS(mqsys::MQCHS_RUNNING);
    pub const MQCHS_STOPPING: types::MQCHS = types::MQCHS(mqsys::MQCHS_STOPPING);
    pub const MQCHS_RETRYING: types::MQCHS = types::MQCHS(mqsys::MQCHS_RETRYING);
    pub const MQCHS_STOPPED: types::MQCHS = types::MQCHS(mqsys::MQCHS_STOPPED);
    pub const MQCHS_REQUESTING: types::MQCHS = types::MQCHS(mqsys::MQCHS_REQUESTING);
    pub const MQCHS_PAUSED: types::MQCHS = types::MQCHS(mqsys::MQCHS_PAUSED);
    pub const MQCHS_DISCONNECTED: types::MQCHS = types::MQCHS(mqsys::MQCHS_DISCONNECTED);
    pub const MQCHS_INITIALIZING: types::MQCHS = types::MQCHS(mqsys::MQCHS_INITIALIZING);
    pub const MQCHS_SWITCHING: types::MQCHS = types::MQCHS(mqsys::MQCHS_SWITCHING);
    pub const MQCHTAB_Q_MGR: types::MQCHTAB = types::MQCHTAB(mqsys::MQCHTAB_Q_MGR);
    pub const MQCHTAB_CLNTCONN: types::MQCHTAB = types::MQCHTAB(mqsys::MQCHTAB_CLNTCONN);
    pub const MQCHT_SENDER: types::MQCHT = types::MQCHT(mqsys::MQCHT_SENDER);
    pub const MQCHT_SERVER: types::MQCHT = types::MQCHT(mqsys::MQCHT_SERVER);
    pub const MQCHT_RECEIVER: types::MQCHT = types::MQCHT(mqsys::MQCHT_RECEIVER);
    pub const MQCHT_REQUESTER: types::MQCHT = types::MQCHT(mqsys::MQCHT_REQUESTER);
    pub const MQCHT_ALL: types::MQCHT = types::MQCHT(mqsys::MQCHT_ALL);
    pub const MQCHT_CLNTCONN: types::MQCHT = types::MQCHT(mqsys::MQCHT_CLNTCONN);
    pub const MQCHT_SVRCONN: types::MQCHT = types::MQCHT(mqsys::MQCHT_SVRCONN);
    pub const MQCHT_CLUSRCVR: types::MQCHT = types::MQCHT(mqsys::MQCHT_CLUSRCVR);
    pub const MQCHT_CLUSSDR: types::MQCHT = types::MQCHT(mqsys::MQCHT_CLUSSDR);
    pub const MQCHT_MQTT: types::MQCHT = types::MQCHT(mqsys::MQCHT_MQTT);
    pub const MQCHT_AMQP: types::MQCHT = types::MQCHT(mqsys::MQCHT_AMQP);
    pub const MQCIH_NONE: types::MQCIH = types::MQCIH(mqsys::MQCIH_NONE);
    pub const MQCIH_PASS_EXPIRATION: types::MQCIH = types::MQCIH(
        mqsys::MQCIH_PASS_EXPIRATION,
    );
    pub const MQCIH_REPLY_WITHOUT_NULLS: types::MQCIH = types::MQCIH(
        mqsys::MQCIH_REPLY_WITHOUT_NULLS,
    );
    pub const MQCIH_SYNC_ON_RETURN: types::MQCIH = types::MQCIH(
        mqsys::MQCIH_SYNC_ON_RETURN,
    );
    pub const MQCIH_NO_SYNC_ON_RETURN: types::MQCIH = types::MQCIH(
        mqsys::MQCIH_NO_SYNC_ON_RETURN,
    );
    pub const MQCIH_REPLY_WITH_NULLS: types::MQCIH = types::MQCIH(
        mqsys::MQCIH_REPLY_WITH_NULLS,
    );
    pub const MQCIH_UNLIMITED_EXPIRATION: types::MQCIH = types::MQCIH(
        mqsys::MQCIH_UNLIMITED_EXPIRATION,
    );
    pub const MQCIT_MULTICAST: types::MQCIT = types::MQCIT(mqsys::MQCIT_MULTICAST);
    pub const MQCLCT_STATIC: types::MQCLCT = types::MQCLCT(mqsys::MQCLCT_STATIC);
    pub const MQCLCT_DYNAMIC: types::MQCLCT = types::MQCLCT(mqsys::MQCLCT_DYNAMIC);
    pub const MQCLROUTE_DIRECT: types::MQCLROUTE = types::MQCLROUTE(
        mqsys::MQCLROUTE_DIRECT,
    );
    pub const MQCLROUTE_TOPIC_HOST: types::MQCLROUTE = types::MQCLROUTE(
        mqsys::MQCLROUTE_TOPIC_HOST,
    );
    pub const MQCLROUTE_NONE: types::MQCLROUTE = types::MQCLROUTE(mqsys::MQCLROUTE_NONE);
    pub const MQCLRS_LOCAL: types::MQCLRS = types::MQCLRS(mqsys::MQCLRS_LOCAL);
    pub const MQCLRS_GLOBAL: types::MQCLRS = types::MQCLRS(mqsys::MQCLRS_GLOBAL);
    pub const MQCLRT_RETAINED: types::MQCLRT = types::MQCLRT(mqsys::MQCLRT_RETAINED);
    pub const MQCLST_ACTIVE: types::MQCLST = types::MQCLST(mqsys::MQCLST_ACTIVE);
    pub const MQCLST_PENDING: types::MQCLST = types::MQCLST(mqsys::MQCLST_PENDING);
    pub const MQCLST_INVALID: types::MQCLST = types::MQCLST(mqsys::MQCLST_INVALID);
    pub const MQCLST_ERROR: types::MQCLST = types::MQCLST(mqsys::MQCLST_ERROR);
    pub const MQCLT_PROGRAM: types::MQCLT = types::MQCLT(mqsys::MQCLT_PROGRAM);
    pub const MQCLT_TRANSACTION: types::MQCLT = types::MQCLT(mqsys::MQCLT_TRANSACTION);
    pub const MQCLWL_USEQ_AS_Q_MGR: types::MQCLWL = types::MQCLWL(
        mqsys::MQCLWL_USEQ_AS_Q_MGR,
    );
    pub const MQCLWL_USEQ_LOCAL: types::MQCLWL = types::MQCLWL(mqsys::MQCLWL_USEQ_LOCAL);
    pub const MQCLWL_USEQ_ANY: types::MQCLWL = types::MQCLWL(mqsys::MQCLWL_USEQ_ANY);
    pub const MQCLXQ_SCTQ: types::MQCLXQ = types::MQCLXQ(mqsys::MQCLXQ_SCTQ);
    pub const MQCLXQ_CHANNEL: types::MQCLXQ = types::MQCLXQ(mqsys::MQCLXQ_CHANNEL);
    pub const MQCMDI_CMDSCOPE_ACCEPTED: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_CMDSCOPE_ACCEPTED,
    );
    pub const MQCMDI_CMDSCOPE_GENERATED: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_CMDSCOPE_GENERATED,
    );
    pub const MQCMDI_CMDSCOPE_COMPLETED: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_CMDSCOPE_COMPLETED,
    );
    pub const MQCMDI_QSG_DISP_COMPLETED: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_QSG_DISP_COMPLETED,
    );
    pub const MQCMDI_COMMAND_ACCEPTED: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_COMMAND_ACCEPTED,
    );
    pub const MQCMDI_CLUSTER_REQUEST_QUEUED: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_CLUSTER_REQUEST_QUEUED,
    );
    pub const MQCMDI_CHANNEL_INIT_STARTED: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_CHANNEL_INIT_STARTED,
    );
    pub const MQCMDI_RECOVER_STARTED: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_RECOVER_STARTED,
    );
    pub const MQCMDI_BACKUP_STARTED: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_BACKUP_STARTED,
    );
    pub const MQCMDI_RECOVER_COMPLETED: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_RECOVER_COMPLETED,
    );
    pub const MQCMDI_SEC_TIMER_ZERO: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_SEC_TIMER_ZERO,
    );
    pub const MQCMDI_REFRESH_CONFIGURATION: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_REFRESH_CONFIGURATION,
    );
    pub const MQCMDI_SEC_SIGNOFF_ERROR: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_SEC_SIGNOFF_ERROR,
    );
    pub const MQCMDI_IMS_BRIDGE_SUSPENDED: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_IMS_BRIDGE_SUSPENDED,
    );
    pub const MQCMDI_DB2_SUSPENDED: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_DB2_SUSPENDED,
    );
    pub const MQCMDI_DB2_OBSOLETE_MSGS: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_DB2_OBSOLETE_MSGS,
    );
    pub const MQCMDI_SEC_UPPERCASE: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_SEC_UPPERCASE,
    );
    pub const MQCMDI_SEC_MIXEDCASE: types::MQCMDI = types::MQCMDI(
        mqsys::MQCMDI_SEC_MIXEDCASE,
    );
    pub const MQCMDL_LEVEL_1: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_1);
    pub const MQCMDL_LEVEL_101: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_101);
    pub const MQCMDL_LEVEL_110: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_110);
    pub const MQCMDL_LEVEL_114: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_114);
    pub const MQCMDL_LEVEL_120: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_120);
    pub const MQCMDL_LEVEL_200: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_200);
    pub const MQCMDL_LEVEL_201: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_201);
    pub const MQCMDL_LEVEL_210: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_210);
    pub const MQCMDL_LEVEL_211: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_211);
    pub const MQCMDL_LEVEL_220: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_220);
    pub const MQCMDL_LEVEL_221: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_221);
    pub const MQCMDL_LEVEL_230: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_230);
    pub const MQCMDL_LEVEL_320: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_320);
    pub const MQCMDL_LEVEL_420: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_420);
    pub const MQCMDL_LEVEL_500: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_500);
    pub const MQCMDL_LEVEL_510: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_510);
    pub const MQCMDL_LEVEL_520: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_520);
    pub const MQCMDL_LEVEL_530: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_530);
    pub const MQCMDL_LEVEL_531: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_531);
    pub const MQCMDL_LEVEL_600: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_600);
    pub const MQCMDL_LEVEL_700: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_700);
    pub const MQCMDL_LEVEL_701: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_701);
    pub const MQCMDL_LEVEL_710: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_710);
    pub const MQCMDL_LEVEL_711: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_711);
    pub const MQCMDL_LEVEL_750: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_750);
    pub const MQCMDL_LEVEL_800: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_800);
    pub const MQCMDL_LEVEL_801: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_801);
    pub const MQCMDL_LEVEL_802: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_802);
    pub const MQCMDL_LEVEL_900: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_900);
    pub const MQCMDL_LEVEL_901: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_901);
    pub const MQCMDL_LEVEL_902: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_902);
    pub const MQCMDL_LEVEL_903: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_903);
    pub const MQCMDL_LEVEL_904: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_904);
    pub const MQCMDL_LEVEL_905: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_905);
    pub const MQCMDL_LEVEL_910: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_910);
    pub const MQCMDL_LEVEL_911: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_911);
    pub const MQCMDL_LEVEL_912: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_912);
    pub const MQCMDL_LEVEL_913: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_913);
    pub const MQCMDL_LEVEL_914: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_914);
    pub const MQCMDL_LEVEL_915: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_915);
    pub const MQCMDL_LEVEL_920: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_920);
    pub const MQCMDL_LEVEL_921: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_921);
    pub const MQCMDL_LEVEL_922: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_922);
    pub const MQCMDL_LEVEL_923: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_923);
    pub const MQCMDL_LEVEL_924: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_924);
    pub const MQCMDL_LEVEL_925: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_925);
    pub const MQCMDL_LEVEL_930: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_930);
    pub const MQCMDL_LEVEL_931: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_931);
    pub const MQCMDL_LEVEL_932: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_932);
    pub const MQCMDL_LEVEL_933: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_933);
    pub const MQCMDL_LEVEL_934: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_934);
    pub const MQCMDL_LEVEL_935: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_935);
    pub const MQCMDL_LEVEL_940: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_940);
    pub const MQCMDL_LEVEL_941: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_941);
    pub const MQCMDL_LEVEL_942: types::MQCMDL = types::MQCMDL(mqsys::MQCMDL_LEVEL_942);
    pub const MQCMDL_CURRENT_LEVEL: types::MQCMDL = types::MQCMDL(
        mqsys::MQCMDL_CURRENT_LEVEL,
    );
    pub const MQCMD_NONE: types::MQCMD = types::MQCMD(mqsys::MQCMD_NONE);
    pub const MQCMD_CHANGE_Q_MGR: types::MQCMD = types::MQCMD(mqsys::MQCMD_CHANGE_Q_MGR);
    pub const MQCMD_INQUIRE_Q_MGR: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_Q_MGR,
    );
    pub const MQCMD_CHANGE_PROCESS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_PROCESS,
    );
    pub const MQCMD_COPY_PROCESS: types::MQCMD = types::MQCMD(mqsys::MQCMD_COPY_PROCESS);
    pub const MQCMD_CREATE_PROCESS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CREATE_PROCESS,
    );
    pub const MQCMD_DELETE_PROCESS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_PROCESS,
    );
    pub const MQCMD_INQUIRE_PROCESS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_PROCESS,
    );
    pub const MQCMD_CHANGE_Q: types::MQCMD = types::MQCMD(mqsys::MQCMD_CHANGE_Q);
    pub const MQCMD_CLEAR_Q: types::MQCMD = types::MQCMD(mqsys::MQCMD_CLEAR_Q);
    pub const MQCMD_COPY_Q: types::MQCMD = types::MQCMD(mqsys::MQCMD_COPY_Q);
    pub const MQCMD_CREATE_Q: types::MQCMD = types::MQCMD(mqsys::MQCMD_CREATE_Q);
    pub const MQCMD_DELETE_Q: types::MQCMD = types::MQCMD(mqsys::MQCMD_DELETE_Q);
    pub const MQCMD_INQUIRE_Q: types::MQCMD = types::MQCMD(mqsys::MQCMD_INQUIRE_Q);
    pub const MQCMD_REFRESH_Q_MGR: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_REFRESH_Q_MGR,
    );
    pub const MQCMD_RESET_Q_STATS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_RESET_Q_STATS,
    );
    pub const MQCMD_INQUIRE_Q_NAMES: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_Q_NAMES,
    );
    pub const MQCMD_INQUIRE_PROCESS_NAMES: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_PROCESS_NAMES,
    );
    pub const MQCMD_INQUIRE_CHANNEL_NAMES: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_CHANNEL_NAMES,
    );
    pub const MQCMD_CHANGE_CHANNEL: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_CHANNEL,
    );
    pub const MQCMD_COPY_CHANNEL: types::MQCMD = types::MQCMD(mqsys::MQCMD_COPY_CHANNEL);
    pub const MQCMD_CREATE_CHANNEL: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CREATE_CHANNEL,
    );
    pub const MQCMD_DELETE_CHANNEL: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_CHANNEL,
    );
    pub const MQCMD_INQUIRE_CHANNEL: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_CHANNEL,
    );
    pub const MQCMD_PING_CHANNEL: types::MQCMD = types::MQCMD(mqsys::MQCMD_PING_CHANNEL);
    pub const MQCMD_RESET_CHANNEL: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_RESET_CHANNEL,
    );
    pub const MQCMD_START_CHANNEL: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_START_CHANNEL,
    );
    pub const MQCMD_STOP_CHANNEL: types::MQCMD = types::MQCMD(mqsys::MQCMD_STOP_CHANNEL);
    pub const MQCMD_START_CHANNEL_INIT: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_START_CHANNEL_INIT,
    );
    pub const MQCMD_START_CHANNEL_LISTENER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_START_CHANNEL_LISTENER,
    );
    pub const MQCMD_CHANGE_NAMELIST: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_NAMELIST,
    );
    pub const MQCMD_COPY_NAMELIST: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_COPY_NAMELIST,
    );
    pub const MQCMD_CREATE_NAMELIST: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CREATE_NAMELIST,
    );
    pub const MQCMD_DELETE_NAMELIST: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_NAMELIST,
    );
    pub const MQCMD_INQUIRE_NAMELIST: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_NAMELIST,
    );
    pub const MQCMD_INQUIRE_NAMELIST_NAMES: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_NAMELIST_NAMES,
    );
    pub const MQCMD_ESCAPE: types::MQCMD = types::MQCMD(mqsys::MQCMD_ESCAPE);
    pub const MQCMD_RESOLVE_CHANNEL: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_RESOLVE_CHANNEL,
    );
    pub const MQCMD_PING_Q_MGR: types::MQCMD = types::MQCMD(mqsys::MQCMD_PING_Q_MGR);
    pub const MQCMD_INQUIRE_Q_STATUS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_Q_STATUS,
    );
    pub const MQCMD_INQUIRE_CHANNEL_STATUS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_CHANNEL_STATUS,
    );
    pub const MQCMD_CONFIG_EVENT: types::MQCMD = types::MQCMD(mqsys::MQCMD_CONFIG_EVENT);
    pub const MQCMD_Q_MGR_EVENT: types::MQCMD = types::MQCMD(mqsys::MQCMD_Q_MGR_EVENT);
    pub const MQCMD_PERFM_EVENT: types::MQCMD = types::MQCMD(mqsys::MQCMD_PERFM_EVENT);
    pub const MQCMD_CHANNEL_EVENT: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANNEL_EVENT,
    );
    pub const MQCMD_DELETE_PUBLICATION: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_PUBLICATION,
    );
    pub const MQCMD_DEREGISTER_PUBLISHER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DEREGISTER_PUBLISHER,
    );
    pub const MQCMD_DEREGISTER_SUBSCRIBER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DEREGISTER_SUBSCRIBER,
    );
    pub const MQCMD_PUBLISH: types::MQCMD = types::MQCMD(mqsys::MQCMD_PUBLISH);
    pub const MQCMD_REGISTER_PUBLISHER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_REGISTER_PUBLISHER,
    );
    pub const MQCMD_REGISTER_SUBSCRIBER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_REGISTER_SUBSCRIBER,
    );
    pub const MQCMD_REQUEST_UPDATE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_REQUEST_UPDATE,
    );
    pub const MQCMD_BROKER_INTERNAL: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_BROKER_INTERNAL,
    );
    pub const MQCMD_ACTIVITY_MSG: types::MQCMD = types::MQCMD(mqsys::MQCMD_ACTIVITY_MSG);
    pub const MQCMD_INQUIRE_CLUSTER_Q_MGR: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_CLUSTER_Q_MGR,
    );
    pub const MQCMD_RESUME_Q_MGR_CLUSTER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_RESUME_Q_MGR_CLUSTER,
    );
    pub const MQCMD_SUSPEND_Q_MGR_CLUSTER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_SUSPEND_Q_MGR_CLUSTER,
    );
    pub const MQCMD_REFRESH_CLUSTER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_REFRESH_CLUSTER,
    );
    pub const MQCMD_RESET_CLUSTER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_RESET_CLUSTER,
    );
    pub const MQCMD_TRACE_ROUTE: types::MQCMD = types::MQCMD(mqsys::MQCMD_TRACE_ROUTE);
    pub const MQCMD_REFRESH_SECURITY: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_REFRESH_SECURITY,
    );
    pub const MQCMD_CHANGE_AUTH_INFO: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_AUTH_INFO,
    );
    pub const MQCMD_COPY_AUTH_INFO: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_COPY_AUTH_INFO,
    );
    pub const MQCMD_CREATE_AUTH_INFO: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CREATE_AUTH_INFO,
    );
    pub const MQCMD_DELETE_AUTH_INFO: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_AUTH_INFO,
    );
    pub const MQCMD_INQUIRE_AUTH_INFO: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_AUTH_INFO,
    );
    pub const MQCMD_INQUIRE_AUTH_INFO_NAMES: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_AUTH_INFO_NAMES,
    );
    pub const MQCMD_INQUIRE_CONNECTION: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_CONNECTION,
    );
    pub const MQCMD_STOP_CONNECTION: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_STOP_CONNECTION,
    );
    pub const MQCMD_INQUIRE_AUTH_RECS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_AUTH_RECS,
    );
    pub const MQCMD_INQUIRE_ENTITY_AUTH: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_ENTITY_AUTH,
    );
    pub const MQCMD_DELETE_AUTH_REC: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_AUTH_REC,
    );
    pub const MQCMD_SET_AUTH_REC: types::MQCMD = types::MQCMD(mqsys::MQCMD_SET_AUTH_REC);
    pub const MQCMD_LOGGER_EVENT: types::MQCMD = types::MQCMD(mqsys::MQCMD_LOGGER_EVENT);
    pub const MQCMD_RESET_Q_MGR: types::MQCMD = types::MQCMD(mqsys::MQCMD_RESET_Q_MGR);
    pub const MQCMD_CHANGE_LISTENER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_LISTENER,
    );
    pub const MQCMD_COPY_LISTENER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_COPY_LISTENER,
    );
    pub const MQCMD_CREATE_LISTENER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CREATE_LISTENER,
    );
    pub const MQCMD_DELETE_LISTENER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_LISTENER,
    );
    pub const MQCMD_INQUIRE_LISTENER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_LISTENER,
    );
    pub const MQCMD_INQUIRE_LISTENER_STATUS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_LISTENER_STATUS,
    );
    pub const MQCMD_COMMAND_EVENT: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_COMMAND_EVENT,
    );
    pub const MQCMD_CHANGE_SECURITY: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_SECURITY,
    );
    pub const MQCMD_CHANGE_CF_STRUC: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_CF_STRUC,
    );
    pub const MQCMD_CHANGE_STG_CLASS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_STG_CLASS,
    );
    pub const MQCMD_CHANGE_TRACE: types::MQCMD = types::MQCMD(mqsys::MQCMD_CHANGE_TRACE);
    pub const MQCMD_ARCHIVE_LOG: types::MQCMD = types::MQCMD(mqsys::MQCMD_ARCHIVE_LOG);
    pub const MQCMD_BACKUP_CF_STRUC: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_BACKUP_CF_STRUC,
    );
    pub const MQCMD_CREATE_BUFFER_POOL: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CREATE_BUFFER_POOL,
    );
    pub const MQCMD_CREATE_PAGE_SET: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CREATE_PAGE_SET,
    );
    pub const MQCMD_CREATE_CF_STRUC: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CREATE_CF_STRUC,
    );
    pub const MQCMD_CREATE_STG_CLASS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CREATE_STG_CLASS,
    );
    pub const MQCMD_COPY_CF_STRUC: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_COPY_CF_STRUC,
    );
    pub const MQCMD_COPY_STG_CLASS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_COPY_STG_CLASS,
    );
    pub const MQCMD_DELETE_CF_STRUC: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_CF_STRUC,
    );
    pub const MQCMD_DELETE_STG_CLASS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_STG_CLASS,
    );
    pub const MQCMD_INQUIRE_ARCHIVE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_ARCHIVE,
    );
    pub const MQCMD_INQUIRE_CF_STRUC: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_CF_STRUC,
    );
    pub const MQCMD_INQUIRE_CF_STRUC_STATUS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_CF_STRUC_STATUS,
    );
    pub const MQCMD_INQUIRE_CMD_SERVER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_CMD_SERVER,
    );
    pub const MQCMD_INQUIRE_CHANNEL_INIT: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_CHANNEL_INIT,
    );
    pub const MQCMD_INQUIRE_QSG: types::MQCMD = types::MQCMD(mqsys::MQCMD_INQUIRE_QSG);
    pub const MQCMD_INQUIRE_LOG: types::MQCMD = types::MQCMD(mqsys::MQCMD_INQUIRE_LOG);
    pub const MQCMD_INQUIRE_SECURITY: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_SECURITY,
    );
    pub const MQCMD_INQUIRE_STG_CLASS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_STG_CLASS,
    );
    pub const MQCMD_INQUIRE_SYSTEM: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_SYSTEM,
    );
    pub const MQCMD_INQUIRE_THREAD: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_THREAD,
    );
    pub const MQCMD_INQUIRE_TRACE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_TRACE,
    );
    pub const MQCMD_INQUIRE_USAGE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_USAGE,
    );
    pub const MQCMD_MOVE_Q: types::MQCMD = types::MQCMD(mqsys::MQCMD_MOVE_Q);
    pub const MQCMD_RECOVER_BSDS: types::MQCMD = types::MQCMD(mqsys::MQCMD_RECOVER_BSDS);
    pub const MQCMD_RECOVER_CF_STRUC: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_RECOVER_CF_STRUC,
    );
    pub const MQCMD_RESET_TPIPE: types::MQCMD = types::MQCMD(mqsys::MQCMD_RESET_TPIPE);
    pub const MQCMD_RESOLVE_INDOUBT: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_RESOLVE_INDOUBT,
    );
    pub const MQCMD_RESUME_Q_MGR: types::MQCMD = types::MQCMD(mqsys::MQCMD_RESUME_Q_MGR);
    pub const MQCMD_REVERIFY_SECURITY: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_REVERIFY_SECURITY,
    );
    pub const MQCMD_SET_ARCHIVE: types::MQCMD = types::MQCMD(mqsys::MQCMD_SET_ARCHIVE);
    pub const MQCMD_SET_LOG: types::MQCMD = types::MQCMD(mqsys::MQCMD_SET_LOG);
    pub const MQCMD_SET_SYSTEM: types::MQCMD = types::MQCMD(mqsys::MQCMD_SET_SYSTEM);
    pub const MQCMD_START_CMD_SERVER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_START_CMD_SERVER,
    );
    pub const MQCMD_START_Q_MGR: types::MQCMD = types::MQCMD(mqsys::MQCMD_START_Q_MGR);
    pub const MQCMD_START_TRACE: types::MQCMD = types::MQCMD(mqsys::MQCMD_START_TRACE);
    pub const MQCMD_STOP_CHANNEL_INIT: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_STOP_CHANNEL_INIT,
    );
    pub const MQCMD_STOP_CHANNEL_LISTENER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_STOP_CHANNEL_LISTENER,
    );
    pub const MQCMD_STOP_CMD_SERVER: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_STOP_CMD_SERVER,
    );
    pub const MQCMD_STOP_Q_MGR: types::MQCMD = types::MQCMD(mqsys::MQCMD_STOP_Q_MGR);
    pub const MQCMD_STOP_TRACE: types::MQCMD = types::MQCMD(mqsys::MQCMD_STOP_TRACE);
    pub const MQCMD_SUSPEND_Q_MGR: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_SUSPEND_Q_MGR,
    );
    pub const MQCMD_INQUIRE_CF_STRUC_NAMES: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_CF_STRUC_NAMES,
    );
    pub const MQCMD_INQUIRE_STG_CLASS_NAMES: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_STG_CLASS_NAMES,
    );
    pub const MQCMD_CHANGE_SERVICE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_SERVICE,
    );
    pub const MQCMD_COPY_SERVICE: types::MQCMD = types::MQCMD(mqsys::MQCMD_COPY_SERVICE);
    pub const MQCMD_CREATE_SERVICE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CREATE_SERVICE,
    );
    pub const MQCMD_DELETE_SERVICE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_SERVICE,
    );
    pub const MQCMD_INQUIRE_SERVICE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_SERVICE,
    );
    pub const MQCMD_INQUIRE_SERVICE_STATUS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_SERVICE_STATUS,
    );
    pub const MQCMD_START_SERVICE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_START_SERVICE,
    );
    pub const MQCMD_STOP_SERVICE: types::MQCMD = types::MQCMD(mqsys::MQCMD_STOP_SERVICE);
    pub const MQCMD_DELETE_BUFFER_POOL: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_BUFFER_POOL,
    );
    pub const MQCMD_DELETE_PAGE_SET: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_PAGE_SET,
    );
    pub const MQCMD_CHANGE_BUFFER_POOL: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_BUFFER_POOL,
    );
    pub const MQCMD_CHANGE_PAGE_SET: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_PAGE_SET,
    );
    pub const MQCMD_INQUIRE_Q_MGR_STATUS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_Q_MGR_STATUS,
    );
    pub const MQCMD_CREATE_LOG: types::MQCMD = types::MQCMD(mqsys::MQCMD_CREATE_LOG);
    pub const MQCMD_STATISTICS_MQI: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_STATISTICS_MQI,
    );
    pub const MQCMD_STATISTICS_Q: types::MQCMD = types::MQCMD(mqsys::MQCMD_STATISTICS_Q);
    pub const MQCMD_STATISTICS_CHANNEL: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_STATISTICS_CHANNEL,
    );
    pub const MQCMD_ACCOUNTING_MQI: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_ACCOUNTING_MQI,
    );
    pub const MQCMD_ACCOUNTING_Q: types::MQCMD = types::MQCMD(mqsys::MQCMD_ACCOUNTING_Q);
    pub const MQCMD_INQUIRE_AUTH_SERVICE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_AUTH_SERVICE,
    );
    pub const MQCMD_CHANGE_TOPIC: types::MQCMD = types::MQCMD(mqsys::MQCMD_CHANGE_TOPIC);
    pub const MQCMD_COPY_TOPIC: types::MQCMD = types::MQCMD(mqsys::MQCMD_COPY_TOPIC);
    pub const MQCMD_CREATE_TOPIC: types::MQCMD = types::MQCMD(mqsys::MQCMD_CREATE_TOPIC);
    pub const MQCMD_DELETE_TOPIC: types::MQCMD = types::MQCMD(mqsys::MQCMD_DELETE_TOPIC);
    pub const MQCMD_INQUIRE_TOPIC: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_TOPIC,
    );
    pub const MQCMD_INQUIRE_TOPIC_NAMES: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_TOPIC_NAMES,
    );
    pub const MQCMD_INQUIRE_SUBSCRIPTION: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_SUBSCRIPTION,
    );
    pub const MQCMD_CREATE_SUBSCRIPTION: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CREATE_SUBSCRIPTION,
    );
    pub const MQCMD_CHANGE_SUBSCRIPTION: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_SUBSCRIPTION,
    );
    pub const MQCMD_DELETE_SUBSCRIPTION: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_SUBSCRIPTION,
    );
    pub const MQCMD_COPY_SUBSCRIPTION: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_COPY_SUBSCRIPTION,
    );
    pub const MQCMD_INQUIRE_SUB_STATUS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_SUB_STATUS,
    );
    pub const MQCMD_INQUIRE_TOPIC_STATUS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_TOPIC_STATUS,
    );
    pub const MQCMD_CLEAR_TOPIC_STRING: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CLEAR_TOPIC_STRING,
    );
    pub const MQCMD_INQUIRE_PUBSUB_STATUS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_PUBSUB_STATUS,
    );
    pub const MQCMD_INQUIRE_SMDS: types::MQCMD = types::MQCMD(mqsys::MQCMD_INQUIRE_SMDS);
    pub const MQCMD_CHANGE_SMDS: types::MQCMD = types::MQCMD(mqsys::MQCMD_CHANGE_SMDS);
    pub const MQCMD_RESET_SMDS: types::MQCMD = types::MQCMD(mqsys::MQCMD_RESET_SMDS);
    pub const MQCMD_CREATE_COMM_INFO: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CREATE_COMM_INFO,
    );
    pub const MQCMD_INQUIRE_COMM_INFO: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_COMM_INFO,
    );
    pub const MQCMD_CHANGE_COMM_INFO: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_COMM_INFO,
    );
    pub const MQCMD_COPY_COMM_INFO: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_COPY_COMM_INFO,
    );
    pub const MQCMD_DELETE_COMM_INFO: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_COMM_INFO,
    );
    pub const MQCMD_PURGE_CHANNEL: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_PURGE_CHANNEL,
    );
    pub const MQCMD_MQXR_DIAGNOSTICS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_MQXR_DIAGNOSTICS,
    );
    pub const MQCMD_START_SMDSCONN: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_START_SMDSCONN,
    );
    pub const MQCMD_STOP_SMDSCONN: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_STOP_SMDSCONN,
    );
    pub const MQCMD_INQUIRE_SMDSCONN: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_SMDSCONN,
    );
    pub const MQCMD_INQUIRE_MQXR_STATUS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_MQXR_STATUS,
    );
    pub const MQCMD_START_CLIENT_TRACE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_START_CLIENT_TRACE,
    );
    pub const MQCMD_STOP_CLIENT_TRACE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_STOP_CLIENT_TRACE,
    );
    pub const MQCMD_SET_CHLAUTH_REC: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_SET_CHLAUTH_REC,
    );
    pub const MQCMD_INQUIRE_CHLAUTH_RECS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_CHLAUTH_RECS,
    );
    pub const MQCMD_INQUIRE_PROT_POLICY: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_PROT_POLICY,
    );
    pub const MQCMD_CREATE_PROT_POLICY: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CREATE_PROT_POLICY,
    );
    pub const MQCMD_DELETE_PROT_POLICY: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_DELETE_PROT_POLICY,
    );
    pub const MQCMD_CHANGE_PROT_POLICY: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_CHANGE_PROT_POLICY,
    );
    pub const MQCMD_ACTIVITY_TRACE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_ACTIVITY_TRACE,
    );
    pub const MQCMD_RESET_CF_STRUC: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_RESET_CF_STRUC,
    );
    pub const MQCMD_INQUIRE_XR_CAPABILITY: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_XR_CAPABILITY,
    );
    pub const MQCMD_INQUIRE_AMQP_CAPABILITY: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_AMQP_CAPABILITY,
    );
    pub const MQCMD_AMQP_DIAGNOSTICS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_AMQP_DIAGNOSTICS,
    );
    pub const MQCMD_INTER_Q_MGR_STATUS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INTER_Q_MGR_STATUS,
    );
    pub const MQCMD_INTER_Q_MGR_BALANCE: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INTER_Q_MGR_BALANCE,
    );
    pub const MQCMD_INQUIRE_APPL_STATUS: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_INQUIRE_APPL_STATUS,
    );
    pub const MQCMD_SET_PROT_POLICY: types::MQCMD = types::MQCMD(
        mqsys::MQCMD_SET_PROT_POLICY,
    );
    pub const MQCMHO_NONE: types::MQCMHO = types::MQCMHO(mqsys::MQCMHO_NONE);
    pub const MQCMHO_NO_VALIDATION: types::MQCMHO = types::MQCMHO(
        mqsys::MQCMHO_NO_VALIDATION,
    );
    pub const MQCMHO_VALIDATE: types::MQCMHO = types::MQCMHO(mqsys::MQCMHO_VALIDATE);
    pub const MQCMHO_DEFAULT_VALIDATION: types::MQCMHO = types::MQCMHO(
        mqsys::MQCMHO_DEFAULT_VALIDATION,
    );
    pub const MQCNO_NONE: types::MQCNO = types::MQCNO(mqsys::MQCNO_NONE);
    pub const MQCNO_FASTPATH_BINDING: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_FASTPATH_BINDING,
    );
    pub const MQCNO_SERIALIZE_CONN_TAG_Q_MGR: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_SERIALIZE_CONN_TAG_Q_MGR,
    );
    pub const MQCNO_SERIALIZE_CONN_TAG_QSG: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_SERIALIZE_CONN_TAG_QSG,
    );
    pub const MQCNO_RESTRICT_CONN_TAG_Q_MGR: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_RESTRICT_CONN_TAG_Q_MGR,
    );
    pub const MQCNO_RESTRICT_CONN_TAG_QSG: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_RESTRICT_CONN_TAG_QSG,
    );
    pub const MQCNO_HANDLE_SHARE_NONE: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_HANDLE_SHARE_NONE,
    );
    pub const MQCNO_HANDLE_SHARE_BLOCK: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_HANDLE_SHARE_BLOCK,
    );
    pub const MQCNO_HANDLE_SHARE_NO_BLOCK: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_HANDLE_SHARE_NO_BLOCK,
    );
    pub const MQCNO_SHARED_BINDING: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_SHARED_BINDING,
    );
    pub const MQCNO_ISOLATED_BINDING: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_ISOLATED_BINDING,
    );
    pub const MQCNO_LOCAL_BINDING: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_LOCAL_BINDING,
    );
    pub const MQCNO_CLIENT_BINDING: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_CLIENT_BINDING,
    );
    pub const MQCNO_ACCOUNTING_MQI_ENABLED: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_ACCOUNTING_MQI_ENABLED,
    );
    pub const MQCNO_ACCOUNTING_MQI_DISABLED: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_ACCOUNTING_MQI_DISABLED,
    );
    pub const MQCNO_ACCOUNTING_Q_ENABLED: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_ACCOUNTING_Q_ENABLED,
    );
    pub const MQCNO_ACCOUNTING_Q_DISABLED: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_ACCOUNTING_Q_DISABLED,
    );
    pub const MQCNO_NO_CONV_SHARING: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_NO_CONV_SHARING,
    );
    pub const MQCNO_ALL_CONVS_SHARE: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_ALL_CONVS_SHARE,
    );
    pub const MQCNO_CD_FOR_OUTPUT_ONLY: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_CD_FOR_OUTPUT_ONLY,
    );
    pub const MQCNO_USE_CD_SELECTION: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_USE_CD_SELECTION,
    );
    pub const MQCNO_GENERATE_CONN_TAG: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_GENERATE_CONN_TAG,
    );
    pub const MQCNO_RECONNECT: types::MQCNO = types::MQCNO(mqsys::MQCNO_RECONNECT);
    pub const MQCNO_RECONNECT_DISABLED: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_RECONNECT_DISABLED,
    );
    pub const MQCNO_RECONNECT_Q_MGR: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_RECONNECT_Q_MGR,
    );
    pub const MQCNO_ACTIVITY_TRACE_ENABLED: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_ACTIVITY_TRACE_ENABLED,
    );
    pub const MQCNO_ACTIVITY_TRACE_DISABLED: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_ACTIVITY_TRACE_DISABLED,
    );
    pub const MQCNO_RECONNECT_AS_DEF: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_RECONNECT_AS_DEF,
    );
    pub const MQCNO_STANDARD_BINDING: types::MQCNO = types::MQCNO(
        mqsys::MQCNO_STANDARD_BINDING,
    );
    pub const MQCODL_AS_INPUT: types::MQCODL = types::MQCODL(mqsys::MQCODL_AS_INPUT);
    pub const MQCOMPRESS_NOT_AVAILABLE: types::MQCOMPRESS = types::MQCOMPRESS(
        mqsys::MQCOMPRESS_NOT_AVAILABLE,
    );
    pub const MQCOMPRESS_NONE: types::MQCOMPRESS = types::MQCOMPRESS(
        mqsys::MQCOMPRESS_NONE,
    );
    pub const MQCOMPRESS_RLE: types::MQCOMPRESS = types::MQCOMPRESS(
        mqsys::MQCOMPRESS_RLE,
    );
    pub const MQCOMPRESS_ZLIBFAST: types::MQCOMPRESS = types::MQCOMPRESS(
        mqsys::MQCOMPRESS_ZLIBFAST,
    );
    pub const MQCOMPRESS_ZLIBHIGH: types::MQCOMPRESS = types::MQCOMPRESS(
        mqsys::MQCOMPRESS_ZLIBHIGH,
    );
    pub const MQCOMPRESS_SYSTEM: types::MQCOMPRESS = types::MQCOMPRESS(
        mqsys::MQCOMPRESS_SYSTEM,
    );
    pub const MQCOMPRESS_LZ4FAST: types::MQCOMPRESS = types::MQCOMPRESS(
        mqsys::MQCOMPRESS_LZ4FAST,
    );
    pub const MQCOMPRESS_LZ4HIGH: types::MQCOMPRESS = types::MQCOMPRESS(
        mqsys::MQCOMPRESS_LZ4HIGH,
    );
    pub const MQCOMPRESS_ANY: types::MQCOMPRESS = types::MQCOMPRESS(
        mqsys::MQCOMPRESS_ANY,
    );
    pub const MQCOPY_NONE: types::MQCOPY = types::MQCOPY(mqsys::MQCOPY_NONE);
    pub const MQCOPY_ALL: types::MQCOPY = types::MQCOPY(mqsys::MQCOPY_ALL);
    pub const MQCOPY_FORWARD: types::MQCOPY = types::MQCOPY(mqsys::MQCOPY_FORWARD);
    pub const MQCOPY_PUBLISH: types::MQCOPY = types::MQCOPY(mqsys::MQCOPY_PUBLISH);
    pub const MQCOPY_REPLY: types::MQCOPY = types::MQCOPY(mqsys::MQCOPY_REPLY);
    pub const MQCOPY_REPORT: types::MQCOPY = types::MQCOPY(mqsys::MQCOPY_REPORT);
    pub const MQCOPY_DEFAULT: types::MQCOPY = types::MQCOPY(mqsys::MQCOPY_DEFAULT);
    pub const MQCO_NONE: types::MQCO = types::MQCO(mqsys::MQCO_NONE);
    pub const MQCO_DELETE: types::MQCO = types::MQCO(mqsys::MQCO_DELETE);
    pub const MQCO_DELETE_PURGE: types::MQCO = types::MQCO(mqsys::MQCO_DELETE_PURGE);
    pub const MQCO_KEEP_SUB: types::MQCO = types::MQCO(mqsys::MQCO_KEEP_SUB);
    pub const MQCO_REMOVE_SUB: types::MQCO = types::MQCO(mqsys::MQCO_REMOVE_SUB);
    pub const MQCO_QUIESCE: types::MQCO = types::MQCO(mqsys::MQCO_QUIESCE);
    pub const MQCO_IMMEDIATE: types::MQCO = types::MQCO(mqsys::MQCO_IMMEDIATE);
    pub const MQCQT_LOCAL_Q: types::MQCQT = types::MQCQT(mqsys::MQCQT_LOCAL_Q);
    pub const MQCQT_ALIAS_Q: types::MQCQT = types::MQCQT(mqsys::MQCQT_ALIAS_Q);
    pub const MQCQT_REMOTE_Q: types::MQCQT = types::MQCQT(mqsys::MQCQT_REMOTE_Q);
    pub const MQCQT_Q_MGR_ALIAS: types::MQCQT = types::MQCQT(mqsys::MQCQT_Q_MGR_ALIAS);
    pub const MQCRC_OK: types::MQCRC = types::MQCRC(mqsys::MQCRC_OK);
    pub const MQCRC_CICS_EXEC_ERROR: types::MQCRC = types::MQCRC(
        mqsys::MQCRC_CICS_EXEC_ERROR,
    );
    pub const MQCRC_MQ_API_ERROR: types::MQCRC = types::MQCRC(mqsys::MQCRC_MQ_API_ERROR);
    pub const MQCRC_BRIDGE_ERROR: types::MQCRC = types::MQCRC(mqsys::MQCRC_BRIDGE_ERROR);
    pub const MQCRC_BRIDGE_ABEND: types::MQCRC = types::MQCRC(mqsys::MQCRC_BRIDGE_ABEND);
    pub const MQCRC_APPLICATION_ABEND: types::MQCRC = types::MQCRC(
        mqsys::MQCRC_APPLICATION_ABEND,
    );
    pub const MQCRC_SECURITY_ERROR: types::MQCRC = types::MQCRC(
        mqsys::MQCRC_SECURITY_ERROR,
    );
    pub const MQCRC_PROGRAM_NOT_AVAILABLE: types::MQCRC = types::MQCRC(
        mqsys::MQCRC_PROGRAM_NOT_AVAILABLE,
    );
    pub const MQCRC_BRIDGE_TIMEOUT: types::MQCRC = types::MQCRC(
        mqsys::MQCRC_BRIDGE_TIMEOUT,
    );
    pub const MQCRC_TRANSID_NOT_AVAILABLE: types::MQCRC = types::MQCRC(
        mqsys::MQCRC_TRANSID_NOT_AVAILABLE,
    );
    pub const MQCSP_AUTH_NONE: types::MQCSP = types::MQCSP(mqsys::MQCSP_AUTH_NONE);
    pub const MQCSP_AUTH_USER_ID_AND_PWD: types::MQCSP = types::MQCSP(
        mqsys::MQCSP_AUTH_USER_ID_AND_PWD,
    );
    pub const MQCSP_AUTH_ID_TOKEN: types::MQCSP = types::MQCSP(
        mqsys::MQCSP_AUTH_ID_TOKEN,
    );
    pub const MQCSRV_CONVERT_NO: types::MQCSRV_CONVERT = types::MQCSRV_CONVERT(
        mqsys::MQCSRV_CONVERT_NO,
    );
    pub const MQCSRV_CONVERT_YES: types::MQCSRV_CONVERT = types::MQCSRV_CONVERT(
        mqsys::MQCSRV_CONVERT_YES,
    );
    pub const MQCSRV_DLQ_NO: types::MQCSRV_DLQ = types::MQCSRV_DLQ(mqsys::MQCSRV_DLQ_NO);
    pub const MQCSRV_DLQ_YES: types::MQCSRV_DLQ = types::MQCSRV_DLQ(
        mqsys::MQCSRV_DLQ_YES,
    );
    pub const MQCS_NONE: types::MQCS = types::MQCS(mqsys::MQCS_NONE);
    pub const MQCS_SUSPENDED_TEMPORARY: types::MQCS = types::MQCS(
        mqsys::MQCS_SUSPENDED_TEMPORARY,
    );
    pub const MQCS_SUSPENDED_USER_ACTION: types::MQCS = types::MQCS(
        mqsys::MQCS_SUSPENDED_USER_ACTION,
    );
    pub const MQCS_SUSPENDED: types::MQCS = types::MQCS(mqsys::MQCS_SUSPENDED);
    pub const MQCS_STOPPED: types::MQCS = types::MQCS(mqsys::MQCS_STOPPED);
    pub const MQCTES_NOSYNC: types::MQCTES = types::MQCTES(mqsys::MQCTES_NOSYNC);
    pub const MQCTES_COMMIT: types::MQCTES = types::MQCTES(mqsys::MQCTES_COMMIT);
    pub const MQCTES_BACKOUT: types::MQCTES = types::MQCTES(mqsys::MQCTES_BACKOUT);
    pub const MQCTES_ENDTASK: types::MQCTES = types::MQCTES(mqsys::MQCTES_ENDTASK);
    pub const MQCTLO_NONE: types::MQCTLO = types::MQCTLO(mqsys::MQCTLO_NONE);
    pub const MQCTLO_THREAD_AFFINITY: types::MQCTLO = types::MQCTLO(
        mqsys::MQCTLO_THREAD_AFFINITY,
    );
    pub const MQCTLO_FAIL_IF_QUIESCING: types::MQCTLO = types::MQCTLO(
        mqsys::MQCTLO_FAIL_IF_QUIESCING,
    );
    pub const MQCUOWC_MIDDLE: types::MQCUOWC = types::MQCUOWC(mqsys::MQCUOWC_MIDDLE);
    pub const MQCUOWC_COMMIT: types::MQCUOWC = types::MQCUOWC(mqsys::MQCUOWC_COMMIT);
    pub const MQCUOWC_ONLY: types::MQCUOWC = types::MQCUOWC(mqsys::MQCUOWC_ONLY);
    pub const MQCUOWC_BACKOUT: types::MQCUOWC = types::MQCUOWC(mqsys::MQCUOWC_BACKOUT);
    pub const MQCUOWC_CONTINUE: types::MQCUOWC = types::MQCUOWC(mqsys::MQCUOWC_CONTINUE);
    pub const MQDCC_NONE: types::MQDCC = types::MQDCC(mqsys::MQDCC_NONE);
    pub const MQDCC_DEFAULT_CONVERSION: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_DEFAULT_CONVERSION,
    );
    pub const MQDCC_FILL_TARGET_BUFFER: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_FILL_TARGET_BUFFER,
    );
    pub const MQDCC_INT_DEFAULT_CONVERSION: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_INT_DEFAULT_CONVERSION,
    );
    pub const MQDCC_SOURCE_ENC_NORMAL: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_SOURCE_ENC_NORMAL,
    );
    pub const MQDCC_SOURCE_ENC_REVERSED: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_SOURCE_ENC_REVERSED,
    );
    pub const MQDCC_SOURCE_ENC_MASK: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_SOURCE_ENC_MASK,
    );
    pub const MQDCC_TARGET_ENC_NORMAL: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_TARGET_ENC_NORMAL,
    );
    pub const MQDCC_TARGET_ENC_REVERSED: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_TARGET_ENC_REVERSED,
    );
    pub const MQDCC_TARGET_ENC_MASK: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_TARGET_ENC_MASK,
    );
    pub const MQDCC_SOURCE_ENC_UNDEFINED: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_SOURCE_ENC_UNDEFINED,
    );
    pub const MQDCC_TARGET_ENC_UNDEFINED: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_TARGET_ENC_UNDEFINED,
    );
    pub const MQDCC_SOURCE_ENC_FACTOR: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_SOURCE_ENC_FACTOR,
    );
    pub const MQDCC_SOURCE_ENC_NATIVE: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_SOURCE_ENC_NATIVE,
    );
    pub const MQDCC_TARGET_ENC_FACTOR: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_TARGET_ENC_FACTOR,
    );
    pub const MQDCC_TARGET_ENC_NATIVE: types::MQDCC = types::MQDCC(
        mqsys::MQDCC_TARGET_ENC_NATIVE,
    );
    pub const MQDC_MANAGED: types::MQDC = types::MQDC(mqsys::MQDC_MANAGED);
    pub const MQDC_PROVIDED: types::MQDC = types::MQDC(mqsys::MQDC_PROVIDED);
    pub const MQDELO_NONE: types::MQDELO = types::MQDELO(mqsys::MQDELO_NONE);
    pub const MQDELO_LOCAL: types::MQDELO = types::MQDELO(mqsys::MQDELO_LOCAL);
    pub const MQDHF_NONE: types::MQDHF = types::MQDHF(mqsys::MQDHF_NONE);
    pub const MQDHF_NEW_MSG_IDS: types::MQDHF = types::MQDHF(mqsys::MQDHF_NEW_MSG_IDS);
    pub const MQDISCONNECT_NORMAL: types::MQDISCONNECT = types::MQDISCONNECT(
        mqsys::MQDISCONNECT_NORMAL,
    );
    pub const MQDISCONNECT_IMPLICIT: types::MQDISCONNECT = types::MQDISCONNECT(
        mqsys::MQDISCONNECT_IMPLICIT,
    );
    pub const MQDISCONNECT_Q_MGR: types::MQDISCONNECT = types::MQDISCONNECT(
        mqsys::MQDISCONNECT_Q_MGR,
    );
    pub const MQDLV_AS_PARENT: types::MQDLV = types::MQDLV(mqsys::MQDLV_AS_PARENT);
    pub const MQDLV_ALL: types::MQDLV = types::MQDLV(mqsys::MQDLV_ALL);
    pub const MQDLV_ALL_DUR: types::MQDLV = types::MQDLV(mqsys::MQDLV_ALL_DUR);
    pub const MQDLV_ALL_AVAIL: types::MQDLV = types::MQDLV(mqsys::MQDLV_ALL_AVAIL);
    pub const MQDL_NOT_SUPPORTED: types::MQDL = types::MQDL(mqsys::MQDL_NOT_SUPPORTED);
    pub const MQDL_SUPPORTED: types::MQDL = types::MQDL(mqsys::MQDL_SUPPORTED);
    pub const MQDMHO_NONE: types::MQDMHO = types::MQDMHO(mqsys::MQDMHO_NONE);
    pub const MQDMPO_NONE: types::MQDMPO = types::MQDMPO(mqsys::MQDMPO_NONE);
    pub const MQDMPO_DEL_PROP_UNDER_CURSOR: types::MQDMPO = types::MQDMPO(
        mqsys::MQDMPO_DEL_PROP_UNDER_CURSOR,
    );
    pub const MQDNSWLM_NO: types::MQDNSWLM = types::MQDNSWLM(mqsys::MQDNSWLM_NO);
    pub const MQDNSWLM_YES: types::MQDNSWLM = types::MQDNSWLM(mqsys::MQDNSWLM_YES);
    pub const MQDOPT_RESOLVED: types::MQDOPT = types::MQDOPT(mqsys::MQDOPT_RESOLVED);
    pub const MQDOPT_DEFINED: types::MQDOPT = types::MQDOPT(mqsys::MQDOPT_DEFINED);
    pub const MQDSB_DEFAULT: types::MQDSB = types::MQDSB(mqsys::MQDSB_DEFAULT);
    pub const MQDSB_8K: types::MQDSB = types::MQDSB(mqsys::MQDSB_8K);
    pub const MQDSB_16K: types::MQDSB = types::MQDSB(mqsys::MQDSB_16K);
    pub const MQDSB_32K: types::MQDSB = types::MQDSB(mqsys::MQDSB_32K);
    pub const MQDSB_64K: types::MQDSB = types::MQDSB(mqsys::MQDSB_64K);
    pub const MQDSB_128K: types::MQDSB = types::MQDSB(mqsys::MQDSB_128K);
    pub const MQDSB_256K: types::MQDSB = types::MQDSB(mqsys::MQDSB_256K);
    pub const MQDSB_512K: types::MQDSB = types::MQDSB(mqsys::MQDSB_512K);
    pub const MQDSB_1M: types::MQDSB = types::MQDSB(mqsys::MQDSB_1M);
    pub const MQDSB_1024K: types::MQDSB = types::MQDSB(mqsys::MQDSB_1024K);
    pub const MQDSE_DEFAULT: types::MQDSE = types::MQDSE(mqsys::MQDSE_DEFAULT);
    pub const MQDSE_YES: types::MQDSE = types::MQDSE(mqsys::MQDSE_YES);
    pub const MQDSE_NO: types::MQDSE = types::MQDSE(mqsys::MQDSE_NO);
    pub const MQEC_MSG_ARRIVED: types::MQEC = types::MQEC(mqsys::MQEC_MSG_ARRIVED);
    pub const MQEC_WAIT_INTERVAL_EXPIRED: types::MQEC = types::MQEC(
        mqsys::MQEC_WAIT_INTERVAL_EXPIRED,
    );
    pub const MQEC_WAIT_CANCELED: types::MQEC = types::MQEC(mqsys::MQEC_WAIT_CANCELED);
    pub const MQEC_Q_MGR_QUIESCING: types::MQEC = types::MQEC(
        mqsys::MQEC_Q_MGR_QUIESCING,
    );
    pub const MQEC_CONNECTION_QUIESCING: types::MQEC = types::MQEC(
        mqsys::MQEC_CONNECTION_QUIESCING,
    );
    pub const MQEI_UNLIMITED: types::MQEI = types::MQEI(mqsys::MQEI_UNLIMITED);
    pub const MQENC_RESERVED_MASK: types::MQENC = types::MQENC(
        mqsys::MQENC_RESERVED_MASK,
    );
    pub const MQENC_AS_PUBLISHED: types::MQENC = types::MQENC(mqsys::MQENC_AS_PUBLISHED);
    pub const MQENC_INTEGER_NORMAL: types::MQENC = types::MQENC(
        mqsys::MQENC_INTEGER_NORMAL,
    );
    pub const MQENC_INTEGER_REVERSED: types::MQENC = types::MQENC(
        mqsys::MQENC_INTEGER_REVERSED,
    );
    pub const MQENC_INTEGER_MASK: types::MQENC = types::MQENC(mqsys::MQENC_INTEGER_MASK);
    pub const MQENC_DECIMAL_NORMAL: types::MQENC = types::MQENC(
        mqsys::MQENC_DECIMAL_NORMAL,
    );
    pub const MQENC_DECIMAL_REVERSED: types::MQENC = types::MQENC(
        mqsys::MQENC_DECIMAL_REVERSED,
    );
    pub const MQENC_DECIMAL_MASK: types::MQENC = types::MQENC(mqsys::MQENC_DECIMAL_MASK);
    pub const MQENC_FLOAT_IEEE_NORMAL: types::MQENC = types::MQENC(
        mqsys::MQENC_FLOAT_IEEE_NORMAL,
    );
    pub const MQENC_NORMAL: types::MQENC = types::MQENC(mqsys::MQENC_NORMAL);
    pub const MQENC_FLOAT_IEEE_REVERSED: types::MQENC = types::MQENC(
        mqsys::MQENC_FLOAT_IEEE_REVERSED,
    );
    pub const MQENC_REVERSED: types::MQENC = types::MQENC(mqsys::MQENC_REVERSED);
    pub const MQENC_FLOAT_S390: types::MQENC = types::MQENC(mqsys::MQENC_FLOAT_S390);
    pub const MQENC_S390: types::MQENC = types::MQENC(mqsys::MQENC_S390);
    pub const MQENC_FLOAT_TNS: types::MQENC = types::MQENC(mqsys::MQENC_FLOAT_TNS);
    pub const MQENC_TNS: types::MQENC = types::MQENC(mqsys::MQENC_TNS);
    pub const MQENC_FLOAT_MASK: types::MQENC = types::MQENC(mqsys::MQENC_FLOAT_MASK);
    pub const MQENC_DECIMAL_UNDEFINED: types::MQENC = types::MQENC(
        mqsys::MQENC_DECIMAL_UNDEFINED,
    );
    pub const MQENC_FLOAT_UNDEFINED: types::MQENC = types::MQENC(
        mqsys::MQENC_FLOAT_UNDEFINED,
    );
    pub const MQENC_INTEGER_UNDEFINED: types::MQENC = types::MQENC(
        mqsys::MQENC_INTEGER_UNDEFINED,
    );
    pub const MQENC_NATIVE: types::MQENC = types::MQENC(mqsys::MQENC_NATIVE);
    pub const MQEPH_NONE: types::MQEPH = types::MQEPH(mqsys::MQEPH_NONE);
    pub const MQEPH_CCSID_EMBEDDED: types::MQEPH = types::MQEPH(
        mqsys::MQEPH_CCSID_EMBEDDED,
    );
    pub const MQET_MQSC: types::MQET = types::MQET(mqsys::MQET_MQSC);
    pub const MQEVO_OTHER: types::MQEVO = types::MQEVO(mqsys::MQEVO_OTHER);
    pub const MQEVO_CONSOLE: types::MQEVO = types::MQEVO(mqsys::MQEVO_CONSOLE);
    pub const MQEVO_INIT: types::MQEVO = types::MQEVO(mqsys::MQEVO_INIT);
    pub const MQEVO_MSG: types::MQEVO = types::MQEVO(mqsys::MQEVO_MSG);
    pub const MQEVO_MQSET: types::MQEVO = types::MQEVO(mqsys::MQEVO_MQSET);
    pub const MQEVO_INTERNAL: types::MQEVO = types::MQEVO(mqsys::MQEVO_INTERNAL);
    pub const MQEVO_MQSUB: types::MQEVO = types::MQEVO(mqsys::MQEVO_MQSUB);
    pub const MQEVO_CTLMSG: types::MQEVO = types::MQEVO(mqsys::MQEVO_CTLMSG);
    pub const MQEVO_REST: types::MQEVO = types::MQEVO(mqsys::MQEVO_REST);
    pub const MQEVR_DISABLED: types::MQEVR = types::MQEVR(mqsys::MQEVR_DISABLED);
    pub const MQEVR_ENABLED: types::MQEVR = types::MQEVR(mqsys::MQEVR_ENABLED);
    pub const MQEVR_EXCEPTION: types::MQEVR = types::MQEVR(mqsys::MQEVR_EXCEPTION);
    pub const MQEVR_NO_DISPLAY: types::MQEVR = types::MQEVR(mqsys::MQEVR_NO_DISPLAY);
    pub const MQEVR_API_ONLY: types::MQEVR = types::MQEVR(mqsys::MQEVR_API_ONLY);
    pub const MQEVR_ADMIN_ONLY: types::MQEVR = types::MQEVR(mqsys::MQEVR_ADMIN_ONLY);
    pub const MQEVR_USER_ONLY: types::MQEVR = types::MQEVR(mqsys::MQEVR_USER_ONLY);
    pub const MQEXPI_OFF: types::MQEXPI = types::MQEXPI(mqsys::MQEXPI_OFF);
    pub const MQEXTATTRS_ALL: types::MQEXTATTRS = types::MQEXTATTRS(
        mqsys::MQEXTATTRS_ALL,
    );
    pub const MQEXTATTRS_NONDEF: types::MQEXTATTRS = types::MQEXTATTRS(
        mqsys::MQEXTATTRS_NONDEF,
    );
    pub const MQEXT_ALL: types::MQEXT = types::MQEXT(mqsys::MQEXT_ALL);
    pub const MQEXT_OBJECT: types::MQEXT = types::MQEXT(mqsys::MQEXT_OBJECT);
    pub const MQEXT_AUTHORITY: types::MQEXT = types::MQEXT(mqsys::MQEXT_AUTHORITY);
    pub const MQFB_NONE: types::MQFB = types::MQFB(mqsys::MQFB_NONE);
    pub const MQFB_QUIT: types::MQFB = types::MQFB(mqsys::MQFB_QUIT);
    pub const MQFB_EXPIRATION: types::MQFB = types::MQFB(mqsys::MQFB_EXPIRATION);
    pub const MQFB_COA: types::MQFB = types::MQFB(mqsys::MQFB_COA);
    pub const MQFB_COD: types::MQFB = types::MQFB(mqsys::MQFB_COD);
    pub const MQFB_CHANNEL_COMPLETED: types::MQFB = types::MQFB(
        mqsys::MQFB_CHANNEL_COMPLETED,
    );
    pub const MQFB_CHANNEL_FAIL_RETRY: types::MQFB = types::MQFB(
        mqsys::MQFB_CHANNEL_FAIL_RETRY,
    );
    pub const MQFB_CHANNEL_FAIL: types::MQFB = types::MQFB(mqsys::MQFB_CHANNEL_FAIL);
    pub const MQFB_APPL_CANNOT_BE_STARTED: types::MQFB = types::MQFB(
        mqsys::MQFB_APPL_CANNOT_BE_STARTED,
    );
    pub const MQFB_TM_ERROR: types::MQFB = types::MQFB(mqsys::MQFB_TM_ERROR);
    pub const MQFB_APPL_TYPE_ERROR: types::MQFB = types::MQFB(
        mqsys::MQFB_APPL_TYPE_ERROR,
    );
    pub const MQFB_STOPPED_BY_MSG_EXIT: types::MQFB = types::MQFB(
        mqsys::MQFB_STOPPED_BY_MSG_EXIT,
    );
    pub const MQFB_ACTIVITY: types::MQFB = types::MQFB(mqsys::MQFB_ACTIVITY);
    pub const MQFB_XMIT_Q_MSG_ERROR: types::MQFB = types::MQFB(
        mqsys::MQFB_XMIT_Q_MSG_ERROR,
    );
    pub const MQFB_PAN: types::MQFB = types::MQFB(mqsys::MQFB_PAN);
    pub const MQFB_NAN: types::MQFB = types::MQFB(mqsys::MQFB_NAN);
    pub const MQFB_STOPPED_BY_CHAD_EXIT: types::MQFB = types::MQFB(
        mqsys::MQFB_STOPPED_BY_CHAD_EXIT,
    );
    pub const MQFB_STOPPED_BY_PUBSUB_EXIT: types::MQFB = types::MQFB(
        mqsys::MQFB_STOPPED_BY_PUBSUB_EXIT,
    );
    pub const MQFB_NOT_A_REPOSITORY_MSG: types::MQFB = types::MQFB(
        mqsys::MQFB_NOT_A_REPOSITORY_MSG,
    );
    pub const MQFB_BIND_OPEN_CLUSRCVR_DEL: types::MQFB = types::MQFB(
        mqsys::MQFB_BIND_OPEN_CLUSRCVR_DEL,
    );
    pub const MQFB_MAX_ACTIVITIES: types::MQFB = types::MQFB(mqsys::MQFB_MAX_ACTIVITIES);
    pub const MQFB_NOT_FORWARDED: types::MQFB = types::MQFB(mqsys::MQFB_NOT_FORWARDED);
    pub const MQFB_NOT_DELIVERED: types::MQFB = types::MQFB(mqsys::MQFB_NOT_DELIVERED);
    pub const MQFB_UNSUPPORTED_FORWARDING: types::MQFB = types::MQFB(
        mqsys::MQFB_UNSUPPORTED_FORWARDING,
    );
    pub const MQFB_UNSUPPORTED_DELIVERY: types::MQFB = types::MQFB(
        mqsys::MQFB_UNSUPPORTED_DELIVERY,
    );
    pub const MQFB_DATA_LENGTH_ZERO: types::MQFB = types::MQFB(
        mqsys::MQFB_DATA_LENGTH_ZERO,
    );
    pub const MQFB_DATA_LENGTH_NEGATIVE: types::MQFB = types::MQFB(
        mqsys::MQFB_DATA_LENGTH_NEGATIVE,
    );
    pub const MQFB_DATA_LENGTH_TOO_BIG: types::MQFB = types::MQFB(
        mqsys::MQFB_DATA_LENGTH_TOO_BIG,
    );
    pub const MQFB_BUFFER_OVERFLOW: types::MQFB = types::MQFB(
        mqsys::MQFB_BUFFER_OVERFLOW,
    );
    pub const MQFB_LENGTH_OFF_BY_ONE: types::MQFB = types::MQFB(
        mqsys::MQFB_LENGTH_OFF_BY_ONE,
    );
    pub const MQFB_IIH_ERROR: types::MQFB = types::MQFB(mqsys::MQFB_IIH_ERROR);
    pub const MQFB_NOT_AUTHORIZED_FOR_IMS: types::MQFB = types::MQFB(
        mqsys::MQFB_NOT_AUTHORIZED_FOR_IMS,
    );
    pub const MQFB_DATA_LENGTH_TOO_SHORT: types::MQFB = types::MQFB(
        mqsys::MQFB_DATA_LENGTH_TOO_SHORT,
    );
    pub const MQFB_IMS_ERROR: types::MQFB = types::MQFB(mqsys::MQFB_IMS_ERROR);
    pub const MQFB_CICS_INTERNAL_ERROR: types::MQFB = types::MQFB(
        mqsys::MQFB_CICS_INTERNAL_ERROR,
    );
    pub const MQFB_CICS_NOT_AUTHORIZED: types::MQFB = types::MQFB(
        mqsys::MQFB_CICS_NOT_AUTHORIZED,
    );
    pub const MQFB_CICS_BRIDGE_FAILURE: types::MQFB = types::MQFB(
        mqsys::MQFB_CICS_BRIDGE_FAILURE,
    );
    pub const MQFB_CICS_CORREL_ID_ERROR: types::MQFB = types::MQFB(
        mqsys::MQFB_CICS_CORREL_ID_ERROR,
    );
    pub const MQFB_CICS_CCSID_ERROR: types::MQFB = types::MQFB(
        mqsys::MQFB_CICS_CCSID_ERROR,
    );
    pub const MQFB_CICS_ENCODING_ERROR: types::MQFB = types::MQFB(
        mqsys::MQFB_CICS_ENCODING_ERROR,
    );
    pub const MQFB_CICS_CIH_ERROR: types::MQFB = types::MQFB(mqsys::MQFB_CICS_CIH_ERROR);
    pub const MQFB_CICS_UOW_ERROR: types::MQFB = types::MQFB(mqsys::MQFB_CICS_UOW_ERROR);
    pub const MQFB_CICS_COMMAREA_ERROR: types::MQFB = types::MQFB(
        mqsys::MQFB_CICS_COMMAREA_ERROR,
    );
    pub const MQFB_CICS_APPL_NOT_STARTED: types::MQFB = types::MQFB(
        mqsys::MQFB_CICS_APPL_NOT_STARTED,
    );
    pub const MQFB_CICS_APPL_ABENDED: types::MQFB = types::MQFB(
        mqsys::MQFB_CICS_APPL_ABENDED,
    );
    pub const MQFB_CICS_DLQ_ERROR: types::MQFB = types::MQFB(mqsys::MQFB_CICS_DLQ_ERROR);
    pub const MQFB_CICS_UOW_BACKED_OUT: types::MQFB = types::MQFB(
        mqsys::MQFB_CICS_UOW_BACKED_OUT,
    );
    pub const MQFB_PUBLICATIONS_ON_REQUEST: types::MQFB = types::MQFB(
        mqsys::MQFB_PUBLICATIONS_ON_REQUEST,
    );
    pub const MQFB_SUBSCRIBER_IS_PUBLISHER: types::MQFB = types::MQFB(
        mqsys::MQFB_SUBSCRIBER_IS_PUBLISHER,
    );
    pub const MQFB_MSG_SCOPE_MISMATCH: types::MQFB = types::MQFB(
        mqsys::MQFB_MSG_SCOPE_MISMATCH,
    );
    pub const MQFB_SELECTOR_MISMATCH: types::MQFB = types::MQFB(
        mqsys::MQFB_SELECTOR_MISMATCH,
    );
    pub const MQFB_NOT_A_GROUPUR_MSG: types::MQFB = types::MQFB(
        mqsys::MQFB_NOT_A_GROUPUR_MSG,
    );
    pub const MQFC_NO: types::MQFC = types::MQFC(mqsys::MQFC_NO);
    pub const MQFC_YES: types::MQFC = types::MQFC(mqsys::MQFC_YES);
    pub const MQFSENC_NO: types::MQFSENC = types::MQFSENC(mqsys::MQFSENC_NO);
    pub const MQFSENC_YES: types::MQFSENC = types::MQFSENC(mqsys::MQFSENC_YES);
    pub const MQFSENC_UNKNOWN: types::MQFSENC = types::MQFSENC(mqsys::MQFSENC_UNKNOWN);
    pub const MQFS_SHARED: types::MQFS = types::MQFS(mqsys::MQFS_SHARED);
    pub const MQFUN_TYPE_UNKNOWN: types::MQFUN = types::MQFUN(mqsys::MQFUN_TYPE_UNKNOWN);
    pub const MQFUN_TYPE_JVM: types::MQFUN = types::MQFUN(mqsys::MQFUN_TYPE_JVM);
    pub const MQFUN_TYPE_PROGRAM: types::MQFUN = types::MQFUN(mqsys::MQFUN_TYPE_PROGRAM);
    pub const MQFUN_TYPE_PROCEDURE: types::MQFUN = types::MQFUN(
        mqsys::MQFUN_TYPE_PROCEDURE,
    );
    pub const MQFUN_TYPE_USERDEF: types::MQFUN = types::MQFUN(mqsys::MQFUN_TYPE_USERDEF);
    pub const MQFUN_TYPE_COMMAND: types::MQFUN = types::MQFUN(mqsys::MQFUN_TYPE_COMMAND);
    pub const MQGACF_COMMAND_CONTEXT: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_COMMAND_CONTEXT,
    );
    pub const MQGACF_COMMAND_DATA: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_COMMAND_DATA,
    );
    pub const MQGACF_TRACE_ROUTE: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_TRACE_ROUTE,
    );
    pub const MQGACF_OPERATION: types::MQGACF = types::MQGACF(mqsys::MQGACF_OPERATION);
    pub const MQGACF_ACTIVITY: types::MQGACF = types::MQGACF(mqsys::MQGACF_ACTIVITY);
    pub const MQGACF_EMBEDDED_MQMD: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_EMBEDDED_MQMD,
    );
    pub const MQGACF_MESSAGE: types::MQGACF = types::MQGACF(mqsys::MQGACF_MESSAGE);
    pub const MQGACF_MQMD: types::MQGACF = types::MQGACF(mqsys::MQGACF_MQMD);
    pub const MQGACF_VALUE_NAMING: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_VALUE_NAMING,
    );
    pub const MQGACF_Q_ACCOUNTING_DATA: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_Q_ACCOUNTING_DATA,
    );
    pub const MQGACF_Q_STATISTICS_DATA: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_Q_STATISTICS_DATA,
    );
    pub const MQGACF_CHL_STATISTICS_DATA: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_CHL_STATISTICS_DATA,
    );
    pub const MQGACF_ACTIVITY_TRACE: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_ACTIVITY_TRACE,
    );
    pub const MQGACF_APP_DIST_LIST: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_APP_DIST_LIST,
    );
    pub const MQGACF_MONITOR_CLASS: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_MONITOR_CLASS,
    );
    pub const MQGACF_MONITOR_TYPE: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_MONITOR_TYPE,
    );
    pub const MQGACF_MONITOR_ELEMENT: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_MONITOR_ELEMENT,
    );
    pub const MQGACF_APPL_STATUS: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_APPL_STATUS,
    );
    pub const MQGACF_CHANGED_APPLS: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_CHANGED_APPLS,
    );
    pub const MQGACF_ALL_APPLS: types::MQGACF = types::MQGACF(mqsys::MQGACF_ALL_APPLS);
    pub const MQGACF_APPL_BALANCE: types::MQGACF = types::MQGACF(
        mqsys::MQGACF_APPL_BALANCE,
    );
    pub const MQGMO_NONE: types::MQGMO = types::MQGMO(mqsys::MQGMO_NONE);
    pub const MQGMO_WAIT: types::MQGMO = types::MQGMO(mqsys::MQGMO_WAIT);
    pub const MQGMO_SYNCPOINT: types::MQGMO = types::MQGMO(mqsys::MQGMO_SYNCPOINT);
    pub const MQGMO_NO_SYNCPOINT: types::MQGMO = types::MQGMO(mqsys::MQGMO_NO_SYNCPOINT);
    pub const MQGMO_SET_SIGNAL: types::MQGMO = types::MQGMO(mqsys::MQGMO_SET_SIGNAL);
    pub const MQGMO_BROWSE_FIRST: types::MQGMO = types::MQGMO(mqsys::MQGMO_BROWSE_FIRST);
    pub const MQGMO_BROWSE_NEXT: types::MQGMO = types::MQGMO(mqsys::MQGMO_BROWSE_NEXT);
    pub const MQGMO_ACCEPT_TRUNCATED_MSG: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_ACCEPT_TRUNCATED_MSG,
    );
    pub const MQGMO_MARK_SKIP_BACKOUT: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_MARK_SKIP_BACKOUT,
    );
    pub const MQGMO_MSG_UNDER_CURSOR: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_MSG_UNDER_CURSOR,
    );
    pub const MQGMO_LOCK: types::MQGMO = types::MQGMO(mqsys::MQGMO_LOCK);
    pub const MQGMO_UNLOCK: types::MQGMO = types::MQGMO(mqsys::MQGMO_UNLOCK);
    pub const MQGMO_BROWSE_MSG_UNDER_CURSOR: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_BROWSE_MSG_UNDER_CURSOR,
    );
    pub const MQGMO_SYNCPOINT_IF_PERSISTENT: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_SYNCPOINT_IF_PERSISTENT,
    );
    pub const MQGMO_FAIL_IF_QUIESCING: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_FAIL_IF_QUIESCING,
    );
    pub const MQGMO_CONVERT: types::MQGMO = types::MQGMO(mqsys::MQGMO_CONVERT);
    pub const MQGMO_LOGICAL_ORDER: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_LOGICAL_ORDER,
    );
    pub const MQGMO_COMPLETE_MSG: types::MQGMO = types::MQGMO(mqsys::MQGMO_COMPLETE_MSG);
    pub const MQGMO_ALL_MSGS_AVAILABLE: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_ALL_MSGS_AVAILABLE,
    );
    pub const MQGMO_ALL_SEGMENTS_AVAILABLE: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_ALL_SEGMENTS_AVAILABLE,
    );
    pub const MQGMO_MARK_BROWSE_HANDLE: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_MARK_BROWSE_HANDLE,
    );
    pub const MQGMO_MARK_BROWSE_CO_OP: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_MARK_BROWSE_CO_OP,
    );
    pub const MQGMO_UNMARK_BROWSE_CO_OP: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_UNMARK_BROWSE_CO_OP,
    );
    pub const MQGMO_UNMARK_BROWSE_HANDLE: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_UNMARK_BROWSE_HANDLE,
    );
    pub const MQGMO_UNMARKED_BROWSE_MSG: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_UNMARKED_BROWSE_MSG,
    );
    pub const MQGMO_BROWSE_HANDLE: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_BROWSE_HANDLE,
    );
    pub const MQGMO_BROWSE_CO_OP: types::MQGMO = types::MQGMO(mqsys::MQGMO_BROWSE_CO_OP);
    pub const MQGMO_PROPERTIES_FORCE_MQRFH2: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_PROPERTIES_FORCE_MQRFH2,
    );
    pub const MQGMO_NO_PROPERTIES: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_NO_PROPERTIES,
    );
    pub const MQGMO_PROPERTIES_IN_HANDLE: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_PROPERTIES_IN_HANDLE,
    );
    pub const MQGMO_PROPERTIES_COMPATIBILITY: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_PROPERTIES_COMPATIBILITY,
    );
    pub const MQGMO_NO_WAIT: types::MQGMO = types::MQGMO(mqsys::MQGMO_NO_WAIT);
    pub const MQGMO_PROPERTIES_AS_Q_DEF: types::MQGMO = types::MQGMO(
        mqsys::MQGMO_PROPERTIES_AS_Q_DEF,
    );
    pub const MQGUR_DISABLED: types::MQGUR = types::MQGUR(mqsys::MQGUR_DISABLED);
    pub const MQGUR_ENABLED: types::MQGUR = types::MQGUR(mqsys::MQGUR_ENABLED);
    pub const MQHA_BAG_HANDLE: types::MQHA = types::MQHA(mqsys::MQHA_BAG_HANDLE);
    pub const MQHB_NONE: types::MQHB = types::MQHB(mqsys::MQHB_NONE);
    pub const MQHB_UNUSABLE_HBAG: types::MQHB = types::MQHB(mqsys::MQHB_UNUSABLE_HBAG);
    pub const MQHC_UNASSOCIATED_HCONN: types::MQHC = types::MQHC(
        mqsys::MQHC_UNASSOCIATED_HCONN,
    );
    pub const MQHC_UNUSABLE_HCONN: types::MQHC = types::MQHC(mqsys::MQHC_UNUSABLE_HCONN);
    pub const MQHC_DEF_HCONN: types::MQHC = types::MQHC(mqsys::MQHC_DEF_HCONN);
    pub const MQHM_UNUSABLE_HMSG: types::MQHM = types::MQHM(mqsys::MQHM_UNUSABLE_HMSG);
    pub const MQHM_NONE: types::MQHM = types::MQHM(mqsys::MQHM_NONE);
    pub const MQHO_UNUSABLE_HOBJ: types::MQHO = types::MQHO(mqsys::MQHO_UNUSABLE_HOBJ);
    pub const MQHO_NONE: types::MQHO = types::MQHO(mqsys::MQHO_NONE);
    pub const MQHSTATE_INACTIVE: types::MQHSTATE = types::MQHSTATE(
        mqsys::MQHSTATE_INACTIVE,
    );
    pub const MQHSTATE_ACTIVE: types::MQHSTATE = types::MQHSTATE(mqsys::MQHSTATE_ACTIVE);
    pub const MQIACF_Q_MGR_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_ATTRS,
    );
    pub const MQIACF_Q_ATTRS: types::MQIACF = types::MQIACF(mqsys::MQIACF_Q_ATTRS);
    pub const MQIACF_PROCESS_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_PROCESS_ATTRS,
    );
    pub const MQIACF_NAMELIST_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_NAMELIST_ATTRS,
    );
    pub const MQIACF_FORCE: types::MQIACF = types::MQIACF(mqsys::MQIACF_FORCE);
    pub const MQIACF_REPLACE: types::MQIACF = types::MQIACF(mqsys::MQIACF_REPLACE);
    pub const MQIACF_PURGE: types::MQIACF = types::MQIACF(mqsys::MQIACF_PURGE);
    pub const MQIACF_QUIESCE: types::MQIACF = types::MQIACF(mqsys::MQIACF_QUIESCE);
    pub const MQIACF_ALL: types::MQIACF = types::MQIACF(mqsys::MQIACF_ALL);
    pub const MQIACF_EVENT_APPL_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_EVENT_APPL_TYPE,
    );
    pub const MQIACF_EVENT_ORIGIN: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_EVENT_ORIGIN,
    );
    pub const MQIACF_PARAMETER_ID: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_PARAMETER_ID,
    );
    pub const MQIACF_ERROR_ID: types::MQIACF = types::MQIACF(mqsys::MQIACF_ERROR_ID);
    pub const MQIACF_SELECTOR: types::MQIACF = types::MQIACF(mqsys::MQIACF_SELECTOR);
    pub const MQIACF_CHANNEL_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CHANNEL_ATTRS,
    );
    pub const MQIACF_OBJECT_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_OBJECT_TYPE,
    );
    pub const MQIACF_ESCAPE_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_ESCAPE_TYPE,
    );
    pub const MQIACF_ERROR_OFFSET: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_ERROR_OFFSET,
    );
    pub const MQIACF_AUTH_INFO_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_AUTH_INFO_ATTRS,
    );
    pub const MQIACF_REASON_QUALIFIER: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_REASON_QUALIFIER,
    );
    pub const MQIACF_COMMAND: types::MQIACF = types::MQIACF(mqsys::MQIACF_COMMAND);
    pub const MQIACF_OPEN_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_OPEN_OPTIONS,
    );
    pub const MQIACF_OPEN_TYPE: types::MQIACF = types::MQIACF(mqsys::MQIACF_OPEN_TYPE);
    pub const MQIACF_PROCESS_ID: types::MQIACF = types::MQIACF(mqsys::MQIACF_PROCESS_ID);
    pub const MQIACF_THREAD_ID: types::MQIACF = types::MQIACF(mqsys::MQIACF_THREAD_ID);
    pub const MQIACF_Q_STATUS_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_STATUS_ATTRS,
    );
    pub const MQIACF_UNCOMMITTED_MSGS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_UNCOMMITTED_MSGS,
    );
    pub const MQIACF_HANDLE_STATE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_HANDLE_STATE,
    );
    pub const MQIACF_AUX_ERROR_DATA_INT_1: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_AUX_ERROR_DATA_INT_1,
    );
    pub const MQIACF_AUX_ERROR_DATA_INT_2: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_AUX_ERROR_DATA_INT_2,
    );
    pub const MQIACF_CONV_REASON_CODE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CONV_REASON_CODE,
    );
    pub const MQIACF_BRIDGE_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_BRIDGE_TYPE,
    );
    pub const MQIACF_INQUIRY: types::MQIACF = types::MQIACF(mqsys::MQIACF_INQUIRY);
    pub const MQIACF_WAIT_INTERVAL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_WAIT_INTERVAL,
    );
    pub const MQIACF_OPTIONS: types::MQIACF = types::MQIACF(mqsys::MQIACF_OPTIONS);
    pub const MQIACF_BROKER_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_BROKER_OPTIONS,
    );
    pub const MQIACF_REFRESH_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_REFRESH_TYPE,
    );
    pub const MQIACF_SEQUENCE_NUMBER: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SEQUENCE_NUMBER,
    );
    pub const MQIACF_INTEGER_DATA: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_INTEGER_DATA,
    );
    pub const MQIACF_REGISTRATION_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_REGISTRATION_OPTIONS,
    );
    pub const MQIACF_PUBLICATION_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_PUBLICATION_OPTIONS,
    );
    pub const MQIACF_CLUSTER_INFO: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CLUSTER_INFO,
    );
    pub const MQIACF_Q_MGR_DEFINITION_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_DEFINITION_TYPE,
    );
    pub const MQIACF_Q_MGR_TYPE: types::MQIACF = types::MQIACF(mqsys::MQIACF_Q_MGR_TYPE);
    pub const MQIACF_ACTION: types::MQIACF = types::MQIACF(mqsys::MQIACF_ACTION);
    pub const MQIACF_SUSPEND: types::MQIACF = types::MQIACF(mqsys::MQIACF_SUSPEND);
    pub const MQIACF_BROKER_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_BROKER_COUNT,
    );
    pub const MQIACF_APPL_COUNT: types::MQIACF = types::MQIACF(mqsys::MQIACF_APPL_COUNT);
    pub const MQIACF_ANONYMOUS_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_ANONYMOUS_COUNT,
    );
    pub const MQIACF_REG_REG_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_REG_REG_OPTIONS,
    );
    pub const MQIACF_DELETE_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_DELETE_OPTIONS,
    );
    pub const MQIACF_CLUSTER_Q_MGR_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CLUSTER_Q_MGR_ATTRS,
    );
    pub const MQIACF_REFRESH_INTERVAL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_REFRESH_INTERVAL,
    );
    pub const MQIACF_REFRESH_REPOSITORY: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_REFRESH_REPOSITORY,
    );
    pub const MQIACF_REMOVE_QUEUES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_REMOVE_QUEUES,
    );
    pub const MQIACF_OPEN_INPUT_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_OPEN_INPUT_TYPE,
    );
    pub const MQIACF_OPEN_OUTPUT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_OPEN_OUTPUT,
    );
    pub const MQIACF_OPEN_SET: types::MQIACF = types::MQIACF(mqsys::MQIACF_OPEN_SET);
    pub const MQIACF_OPEN_INQUIRE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_OPEN_INQUIRE,
    );
    pub const MQIACF_OPEN_BROWSE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_OPEN_BROWSE,
    );
    pub const MQIACF_Q_STATUS_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_STATUS_TYPE,
    );
    pub const MQIACF_Q_HANDLE: types::MQIACF = types::MQIACF(mqsys::MQIACF_Q_HANDLE);
    pub const MQIACF_Q_STATUS: types::MQIACF = types::MQIACF(mqsys::MQIACF_Q_STATUS);
    pub const MQIACF_SECURITY_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SECURITY_TYPE,
    );
    pub const MQIACF_CONNECTION_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CONNECTION_ATTRS,
    );
    pub const MQIACF_CONNECT_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CONNECT_OPTIONS,
    );
    pub const MQIACF_CONN_INFO_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CONN_INFO_TYPE,
    );
    pub const MQIACF_CONN_INFO_CONN: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CONN_INFO_CONN,
    );
    pub const MQIACF_CONN_INFO_HANDLE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CONN_INFO_HANDLE,
    );
    pub const MQIACF_CONN_INFO_ALL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CONN_INFO_ALL,
    );
    pub const MQIACF_AUTH_PROFILE_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_AUTH_PROFILE_ATTRS,
    );
    pub const MQIACF_AUTHORIZATION_LIST: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_AUTHORIZATION_LIST,
    );
    pub const MQIACF_AUTH_ADD_AUTHS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_AUTH_ADD_AUTHS,
    );
    pub const MQIACF_AUTH_REMOVE_AUTHS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_AUTH_REMOVE_AUTHS,
    );
    pub const MQIACF_ENTITY_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_ENTITY_TYPE,
    );
    pub const MQIACF_COMMAND_INFO: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_COMMAND_INFO,
    );
    pub const MQIACF_CMDSCOPE_Q_MGR_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CMDSCOPE_Q_MGR_COUNT,
    );
    pub const MQIACF_Q_MGR_SYSTEM: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_SYSTEM,
    );
    pub const MQIACF_Q_MGR_EVENT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_EVENT,
    );
    pub const MQIACF_Q_MGR_DQM: types::MQIACF = types::MQIACF(mqsys::MQIACF_Q_MGR_DQM);
    pub const MQIACF_Q_MGR_CLUSTER: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_CLUSTER,
    );
    pub const MQIACF_QSG_DISPS: types::MQIACF = types::MQIACF(mqsys::MQIACF_QSG_DISPS);
    pub const MQIACF_UOW_STATE: types::MQIACF = types::MQIACF(mqsys::MQIACF_UOW_STATE);
    pub const MQIACF_SECURITY_ITEM: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SECURITY_ITEM,
    );
    pub const MQIACF_CF_STRUC_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STRUC_STATUS,
    );
    pub const MQIACF_UOW_TYPE: types::MQIACF = types::MQIACF(mqsys::MQIACF_UOW_TYPE);
    pub const MQIACF_CF_STRUC_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STRUC_ATTRS,
    );
    pub const MQIACF_EXCLUDE_INTERVAL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_EXCLUDE_INTERVAL,
    );
    pub const MQIACF_CF_STATUS_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STATUS_TYPE,
    );
    pub const MQIACF_CF_STATUS_SUMMARY: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STATUS_SUMMARY,
    );
    pub const MQIACF_CF_STATUS_CONNECT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STATUS_CONNECT,
    );
    pub const MQIACF_CF_STATUS_BACKUP: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STATUS_BACKUP,
    );
    pub const MQIACF_CF_STRUC_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STRUC_TYPE,
    );
    pub const MQIACF_CF_STRUC_SIZE_MAX: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STRUC_SIZE_MAX,
    );
    pub const MQIACF_CF_STRUC_SIZE_USED: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STRUC_SIZE_USED,
    );
    pub const MQIACF_CF_STRUC_ENTRIES_MAX: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STRUC_ENTRIES_MAX,
    );
    pub const MQIACF_CF_STRUC_ENTRIES_USED: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STRUC_ENTRIES_USED,
    );
    pub const MQIACF_CF_STRUC_BACKUP_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STRUC_BACKUP_SIZE,
    );
    pub const MQIACF_MOVE_TYPE: types::MQIACF = types::MQIACF(mqsys::MQIACF_MOVE_TYPE);
    pub const MQIACF_MOVE_TYPE_MOVE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_MOVE_TYPE_MOVE,
    );
    pub const MQIACF_MOVE_TYPE_ADD: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_MOVE_TYPE_ADD,
    );
    pub const MQIACF_Q_MGR_NUMBER: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_NUMBER,
    );
    pub const MQIACF_Q_MGR_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_STATUS,
    );
    pub const MQIACF_DB2_CONN_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_DB2_CONN_STATUS,
    );
    pub const MQIACF_SECURITY_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SECURITY_ATTRS,
    );
    pub const MQIACF_SECURITY_TIMEOUT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SECURITY_TIMEOUT,
    );
    pub const MQIACF_SECURITY_INTERVAL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SECURITY_INTERVAL,
    );
    pub const MQIACF_SECURITY_SWITCH: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SECURITY_SWITCH,
    );
    pub const MQIACF_SECURITY_SETTING: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SECURITY_SETTING,
    );
    pub const MQIACF_STORAGE_CLASS_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_STORAGE_CLASS_ATTRS,
    );
    pub const MQIACF_USAGE_TYPE: types::MQIACF = types::MQIACF(mqsys::MQIACF_USAGE_TYPE);
    pub const MQIACF_BUFFER_POOL_ID: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_BUFFER_POOL_ID,
    );
    pub const MQIACF_USAGE_TOTAL_PAGES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_TOTAL_PAGES,
    );
    pub const MQIACF_USAGE_UNUSED_PAGES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_UNUSED_PAGES,
    );
    pub const MQIACF_USAGE_PERSIST_PAGES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_PERSIST_PAGES,
    );
    pub const MQIACF_USAGE_NONPERSIST_PAGES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_NONPERSIST_PAGES,
    );
    pub const MQIACF_USAGE_RESTART_EXTENTS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_RESTART_EXTENTS,
    );
    pub const MQIACF_USAGE_EXPAND_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_EXPAND_COUNT,
    );
    pub const MQIACF_PAGESET_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_PAGESET_STATUS,
    );
    pub const MQIACF_USAGE_TOTAL_BUFFERS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_TOTAL_BUFFERS,
    );
    pub const MQIACF_USAGE_DATA_SET_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_DATA_SET_TYPE,
    );
    pub const MQIACF_USAGE_PAGESET: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_PAGESET,
    );
    pub const MQIACF_USAGE_DATA_SET: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_DATA_SET,
    );
    pub const MQIACF_USAGE_BUFFER_POOL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_BUFFER_POOL,
    );
    pub const MQIACF_MOVE_COUNT: types::MQIACF = types::MQIACF(mqsys::MQIACF_MOVE_COUNT);
    pub const MQIACF_EXPIRY_Q_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_EXPIRY_Q_COUNT,
    );
    pub const MQIACF_CONFIGURATION_OBJECTS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CONFIGURATION_OBJECTS,
    );
    pub const MQIACF_CONFIGURATION_EVENTS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CONFIGURATION_EVENTS,
    );
    pub const MQIACF_SYSP_TYPE: types::MQIACF = types::MQIACF(mqsys::MQIACF_SYSP_TYPE);
    pub const MQIACF_SYSP_DEALLOC_INTERVAL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_DEALLOC_INTERVAL,
    );
    pub const MQIACF_SYSP_MAX_ARCHIVE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_MAX_ARCHIVE,
    );
    pub const MQIACF_SYSP_MAX_READ_TAPES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_MAX_READ_TAPES,
    );
    pub const MQIACF_SYSP_IN_BUFFER_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_IN_BUFFER_SIZE,
    );
    pub const MQIACF_SYSP_OUT_BUFFER_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_OUT_BUFFER_SIZE,
    );
    pub const MQIACF_SYSP_OUT_BUFFER_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_OUT_BUFFER_COUNT,
    );
    pub const MQIACF_SYSP_ARCHIVE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_ARCHIVE,
    );
    pub const MQIACF_SYSP_DUAL_ACTIVE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_DUAL_ACTIVE,
    );
    pub const MQIACF_SYSP_DUAL_ARCHIVE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_DUAL_ARCHIVE,
    );
    pub const MQIACF_SYSP_DUAL_BSDS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_DUAL_BSDS,
    );
    pub const MQIACF_SYSP_MAX_CONNS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_MAX_CONNS,
    );
    pub const MQIACF_SYSP_MAX_CONNS_FORE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_MAX_CONNS_FORE,
    );
    pub const MQIACF_SYSP_MAX_CONNS_BACK: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_MAX_CONNS_BACK,
    );
    pub const MQIACF_SYSP_EXIT_INTERVAL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_EXIT_INTERVAL,
    );
    pub const MQIACF_SYSP_EXIT_TASKS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_EXIT_TASKS,
    );
    pub const MQIACF_SYSP_CHKPOINT_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_CHKPOINT_COUNT,
    );
    pub const MQIACF_SYSP_OTMA_INTERVAL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_OTMA_INTERVAL,
    );
    pub const MQIACF_SYSP_Q_INDEX_DEFER: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_Q_INDEX_DEFER,
    );
    pub const MQIACF_SYSP_DB2_TASKS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_DB2_TASKS,
    );
    pub const MQIACF_SYSP_RESLEVEL_AUDIT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_RESLEVEL_AUDIT,
    );
    pub const MQIACF_SYSP_ROUTING_CODE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_ROUTING_CODE,
    );
    pub const MQIACF_SYSP_SMF_ACCOUNTING: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_SMF_ACCOUNTING,
    );
    pub const MQIACF_SYSP_SMF_STATS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_SMF_STATS,
    );
    pub const MQIACF_SYSP_SMF_INTERVAL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_SMF_INTERVAL,
    );
    pub const MQIACF_SYSP_TRACE_CLASS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_TRACE_CLASS,
    );
    pub const MQIACF_SYSP_TRACE_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_TRACE_SIZE,
    );
    pub const MQIACF_SYSP_WLM_INTERVAL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_WLM_INTERVAL,
    );
    pub const MQIACF_SYSP_ALLOC_UNIT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_ALLOC_UNIT,
    );
    pub const MQIACF_SYSP_ARCHIVE_RETAIN: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_ARCHIVE_RETAIN,
    );
    pub const MQIACF_SYSP_ARCHIVE_WTOR: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_ARCHIVE_WTOR,
    );
    pub const MQIACF_SYSP_BLOCK_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_BLOCK_SIZE,
    );
    pub const MQIACF_SYSP_CATALOG: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_CATALOG,
    );
    pub const MQIACF_SYSP_COMPACT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_COMPACT,
    );
    pub const MQIACF_SYSP_ALLOC_PRIMARY: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_ALLOC_PRIMARY,
    );
    pub const MQIACF_SYSP_ALLOC_SECONDARY: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_ALLOC_SECONDARY,
    );
    pub const MQIACF_SYSP_PROTECT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_PROTECT,
    );
    pub const MQIACF_SYSP_QUIESCE_INTERVAL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_QUIESCE_INTERVAL,
    );
    pub const MQIACF_SYSP_TIMESTAMP: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_TIMESTAMP,
    );
    pub const MQIACF_SYSP_UNIT_ADDRESS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_UNIT_ADDRESS,
    );
    pub const MQIACF_SYSP_UNIT_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_UNIT_STATUS,
    );
    pub const MQIACF_SYSP_LOG_COPY: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_LOG_COPY,
    );
    pub const MQIACF_SYSP_LOG_USED: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_LOG_USED,
    );
    pub const MQIACF_SYSP_LOG_SUSPEND: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_LOG_SUSPEND,
    );
    pub const MQIACF_SYSP_OFFLOAD_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_OFFLOAD_STATUS,
    );
    pub const MQIACF_SYSP_TOTAL_LOGS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_TOTAL_LOGS,
    );
    pub const MQIACF_SYSP_FULL_LOGS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_FULL_LOGS,
    );
    pub const MQIACF_LISTENER_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_LISTENER_ATTRS,
    );
    pub const MQIACF_LISTENER_STATUS_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_LISTENER_STATUS_ATTRS,
    );
    pub const MQIACF_SERVICE_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SERVICE_ATTRS,
    );
    pub const MQIACF_SERVICE_STATUS_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SERVICE_STATUS_ATTRS,
    );
    pub const MQIACF_Q_TIME_INDICATOR: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_TIME_INDICATOR,
    );
    pub const MQIACF_OLDEST_MSG_AGE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_OLDEST_MSG_AGE,
    );
    pub const MQIACF_AUTH_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_AUTH_OPTIONS,
    );
    pub const MQIACF_Q_MGR_STATUS_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_STATUS_ATTRS,
    );
    pub const MQIACF_CONNECTION_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CONNECTION_COUNT,
    );
    pub const MQIACF_Q_MGR_FACILITY: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_FACILITY,
    );
    pub const MQIACF_CHINIT_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CHINIT_STATUS,
    );
    pub const MQIACF_CMD_SERVER_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CMD_SERVER_STATUS,
    );
    pub const MQIACF_ROUTE_DETAIL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_ROUTE_DETAIL,
    );
    pub const MQIACF_RECORDED_ACTIVITIES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_RECORDED_ACTIVITIES,
    );
    pub const MQIACF_MAX_ACTIVITIES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_MAX_ACTIVITIES,
    );
    pub const MQIACF_DISCONTINUITY_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_DISCONTINUITY_COUNT,
    );
    pub const MQIACF_ROUTE_ACCUMULATION: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_ROUTE_ACCUMULATION,
    );
    pub const MQIACF_ROUTE_DELIVERY: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_ROUTE_DELIVERY,
    );
    pub const MQIACF_OPERATION_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_OPERATION_TYPE,
    );
    pub const MQIACF_BACKOUT_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_BACKOUT_COUNT,
    );
    pub const MQIACF_COMP_CODE: types::MQIACF = types::MQIACF(mqsys::MQIACF_COMP_CODE);
    pub const MQIACF_ENCODING: types::MQIACF = types::MQIACF(mqsys::MQIACF_ENCODING);
    pub const MQIACF_EXPIRY: types::MQIACF = types::MQIACF(mqsys::MQIACF_EXPIRY);
    pub const MQIACF_FEEDBACK: types::MQIACF = types::MQIACF(mqsys::MQIACF_FEEDBACK);
    pub const MQIACF_MSG_FLAGS: types::MQIACF = types::MQIACF(mqsys::MQIACF_MSG_FLAGS);
    pub const MQIACF_MSG_LENGTH: types::MQIACF = types::MQIACF(mqsys::MQIACF_MSG_LENGTH);
    pub const MQIACF_MSG_TYPE: types::MQIACF = types::MQIACF(mqsys::MQIACF_MSG_TYPE);
    pub const MQIACF_OFFSET: types::MQIACF = types::MQIACF(mqsys::MQIACF_OFFSET);
    pub const MQIACF_ORIGINAL_LENGTH: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_ORIGINAL_LENGTH,
    );
    pub const MQIACF_PERSISTENCE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_PERSISTENCE,
    );
    pub const MQIACF_PRIORITY: types::MQIACF = types::MQIACF(mqsys::MQIACF_PRIORITY);
    pub const MQIACF_REASON_CODE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_REASON_CODE,
    );
    pub const MQIACF_REPORT: types::MQIACF = types::MQIACF(mqsys::MQIACF_REPORT);
    pub const MQIACF_VERSION: types::MQIACF = types::MQIACF(mqsys::MQIACF_VERSION);
    pub const MQIACF_UNRECORDED_ACTIVITIES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_UNRECORDED_ACTIVITIES,
    );
    pub const MQIACF_MONITORING: types::MQIACF = types::MQIACF(mqsys::MQIACF_MONITORING);
    pub const MQIACF_ROUTE_FORWARDING: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_ROUTE_FORWARDING,
    );
    pub const MQIACF_SERVICE_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SERVICE_STATUS,
    );
    pub const MQIACF_Q_TYPES: types::MQIACF = types::MQIACF(mqsys::MQIACF_Q_TYPES);
    pub const MQIACF_USER_ID_SUPPORT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USER_ID_SUPPORT,
    );
    pub const MQIACF_INTERFACE_VERSION: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_INTERFACE_VERSION,
    );
    pub const MQIACF_AUTH_SERVICE_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_AUTH_SERVICE_ATTRS,
    );
    pub const MQIACF_USAGE_EXPAND_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_EXPAND_TYPE,
    );
    pub const MQIACF_SYSP_CLUSTER_CACHE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_CLUSTER_CACHE,
    );
    pub const MQIACF_SYSP_DB2_BLOB_TASKS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_DB2_BLOB_TASKS,
    );
    pub const MQIACF_SYSP_WLM_INT_UNITS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_WLM_INT_UNITS,
    );
    pub const MQIACF_TOPIC_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_TOPIC_ATTRS,
    );
    pub const MQIACF_PUBSUB_PROPERTIES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_PUBSUB_PROPERTIES,
    );
    pub const MQIACF_DESTINATION_CLASS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_DESTINATION_CLASS,
    );
    pub const MQIACF_DURABLE_SUBSCRIPTION: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_DURABLE_SUBSCRIPTION,
    );
    pub const MQIACF_SUBSCRIPTION_SCOPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SUBSCRIPTION_SCOPE,
    );
    pub const MQIACF_VARIABLE_USER_ID: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_VARIABLE_USER_ID,
    );
    pub const MQIACF_REQUEST_ONLY: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_REQUEST_ONLY,
    );
    pub const MQIACF_PUB_PRIORITY: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_PUB_PRIORITY,
    );
    pub const MQIACF_SUB_ATTRS: types::MQIACF = types::MQIACF(mqsys::MQIACF_SUB_ATTRS);
    pub const MQIACF_WILDCARD_SCHEMA: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_WILDCARD_SCHEMA,
    );
    pub const MQIACF_SUB_TYPE: types::MQIACF = types::MQIACF(mqsys::MQIACF_SUB_TYPE);
    pub const MQIACF_MESSAGE_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_MESSAGE_COUNT,
    );
    pub const MQIACF_Q_MGR_PUBSUB: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_PUBSUB,
    );
    pub const MQIACF_Q_MGR_VERSION: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_VERSION,
    );
    pub const MQIACF_SUB_STATUS_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SUB_STATUS_ATTRS,
    );
    pub const MQIACF_TOPIC_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_TOPIC_STATUS,
    );
    pub const MQIACF_TOPIC_SUB: types::MQIACF = types::MQIACF(mqsys::MQIACF_TOPIC_SUB);
    pub const MQIACF_TOPIC_PUB: types::MQIACF = types::MQIACF(mqsys::MQIACF_TOPIC_PUB);
    pub const MQIACF_RETAINED_PUBLICATION: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_RETAINED_PUBLICATION,
    );
    pub const MQIACF_TOPIC_STATUS_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_TOPIC_STATUS_ATTRS,
    );
    pub const MQIACF_TOPIC_STATUS_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_TOPIC_STATUS_TYPE,
    );
    pub const MQIACF_SUB_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SUB_OPTIONS,
    );
    pub const MQIACF_PUBLISH_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_PUBLISH_COUNT,
    );
    pub const MQIACF_CLEAR_TYPE: types::MQIACF = types::MQIACF(mqsys::MQIACF_CLEAR_TYPE);
    pub const MQIACF_CLEAR_SCOPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CLEAR_SCOPE,
    );
    pub const MQIACF_SUB_LEVEL: types::MQIACF = types::MQIACF(mqsys::MQIACF_SUB_LEVEL);
    pub const MQIACF_ASYNC_STATE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_ASYNC_STATE,
    );
    pub const MQIACF_SUB_SUMMARY: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SUB_SUMMARY,
    );
    pub const MQIACF_OBSOLETE_MSGS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_OBSOLETE_MSGS,
    );
    pub const MQIACF_PUBSUB_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_PUBSUB_STATUS,
    );
    pub const MQIACF_PS_STATUS_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_PS_STATUS_TYPE,
    );
    pub const MQIACF_PUBSUB_STATUS_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_PUBSUB_STATUS_ATTRS,
    );
    pub const MQIACF_SELECTOR_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SELECTOR_TYPE,
    );
    pub const MQIACF_LOG_COMPRESSION: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_LOG_COMPRESSION,
    );
    pub const MQIACF_GROUPUR_CHECK_ID: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_GROUPUR_CHECK_ID,
    );
    pub const MQIACF_MULC_CAPTURE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_MULC_CAPTURE,
    );
    pub const MQIACF_PERMIT_STANDBY: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_PERMIT_STANDBY,
    );
    pub const MQIACF_OPERATION_MODE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_OPERATION_MODE,
    );
    pub const MQIACF_COMM_INFO_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_COMM_INFO_ATTRS,
    );
    pub const MQIACF_CF_SMDS_BLOCK_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_SMDS_BLOCK_SIZE,
    );
    pub const MQIACF_CF_SMDS_EXPAND: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_SMDS_EXPAND,
    );
    pub const MQIACF_USAGE_FREE_BUFF: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_FREE_BUFF,
    );
    pub const MQIACF_USAGE_FREE_BUFF_PERC: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_FREE_BUFF_PERC,
    );
    pub const MQIACF_CF_STRUC_ACCESS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STRUC_ACCESS,
    );
    pub const MQIACF_CF_STATUS_SMDS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CF_STATUS_SMDS,
    );
    pub const MQIACF_SMDS_ATTRS: types::MQIACF = types::MQIACF(mqsys::MQIACF_SMDS_ATTRS);
    pub const MQIACF_USAGE_SMDS: types::MQIACF = types::MQIACF(mqsys::MQIACF_USAGE_SMDS);
    pub const MQIACF_USAGE_BLOCK_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_BLOCK_SIZE,
    );
    pub const MQIACF_USAGE_DATA_BLOCKS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_DATA_BLOCKS,
    );
    pub const MQIACF_USAGE_EMPTY_BUFFERS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_EMPTY_BUFFERS,
    );
    pub const MQIACF_USAGE_INUSE_BUFFERS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_INUSE_BUFFERS,
    );
    pub const MQIACF_USAGE_LOWEST_FREE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_LOWEST_FREE,
    );
    pub const MQIACF_USAGE_OFFLOAD_MSGS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_OFFLOAD_MSGS,
    );
    pub const MQIACF_USAGE_READS_SAVED: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_READS_SAVED,
    );
    pub const MQIACF_USAGE_SAVED_BUFFERS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_SAVED_BUFFERS,
    );
    pub const MQIACF_USAGE_TOTAL_BLOCKS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_TOTAL_BLOCKS,
    );
    pub const MQIACF_USAGE_USED_BLOCKS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_USED_BLOCKS,
    );
    pub const MQIACF_USAGE_USED_RATE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_USED_RATE,
    );
    pub const MQIACF_USAGE_WAIT_RATE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_USAGE_WAIT_RATE,
    );
    pub const MQIACF_SMDS_OPENMODE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SMDS_OPENMODE,
    );
    pub const MQIACF_SMDS_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SMDS_STATUS,
    );
    pub const MQIACF_SMDS_AVAIL: types::MQIACF = types::MQIACF(mqsys::MQIACF_SMDS_AVAIL);
    pub const MQIACF_MCAST_REL_INDICATOR: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_MCAST_REL_INDICATOR,
    );
    pub const MQIACF_CHLAUTH_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CHLAUTH_TYPE,
    );
    pub const MQIACF_MQXR_DIAGNOSTICS_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_MQXR_DIAGNOSTICS_TYPE,
    );
    pub const MQIACF_CHLAUTH_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CHLAUTH_ATTRS,
    );
    pub const MQIACF_OPERATION_ID: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_OPERATION_ID,
    );
    pub const MQIACF_API_CALLER_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_API_CALLER_TYPE,
    );
    pub const MQIACF_API_ENVIRONMENT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_API_ENVIRONMENT,
    );
    pub const MQIACF_TRACE_DETAIL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_TRACE_DETAIL,
    );
    pub const MQIACF_HOBJ: types::MQIACF = types::MQIACF(mqsys::MQIACF_HOBJ);
    pub const MQIACF_CALL_TYPE: types::MQIACF = types::MQIACF(mqsys::MQIACF_CALL_TYPE);
    pub const MQIACF_MQCB_OPERATION: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_MQCB_OPERATION,
    );
    pub const MQIACF_MQCB_TYPE: types::MQIACF = types::MQIACF(mqsys::MQIACF_MQCB_TYPE);
    pub const MQIACF_MQCB_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_MQCB_OPTIONS,
    );
    pub const MQIACF_CLOSE_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CLOSE_OPTIONS,
    );
    pub const MQIACF_CTL_OPERATION: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CTL_OPERATION,
    );
    pub const MQIACF_GET_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_GET_OPTIONS,
    );
    pub const MQIACF_RECS_PRESENT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_RECS_PRESENT,
    );
    pub const MQIACF_KNOWN_DEST_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_KNOWN_DEST_COUNT,
    );
    pub const MQIACF_UNKNOWN_DEST_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_UNKNOWN_DEST_COUNT,
    );
    pub const MQIACF_INVALID_DEST_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_INVALID_DEST_COUNT,
    );
    pub const MQIACF_RESOLVED_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_RESOLVED_TYPE,
    );
    pub const MQIACF_PUT_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_PUT_OPTIONS,
    );
    pub const MQIACF_BUFFER_LENGTH: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_BUFFER_LENGTH,
    );
    pub const MQIACF_TRACE_DATA_LENGTH: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_TRACE_DATA_LENGTH,
    );
    pub const MQIACF_SMDS_EXPANDST: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SMDS_EXPANDST,
    );
    pub const MQIACF_ITEM_COUNT: types::MQIACF = types::MQIACF(mqsys::MQIACF_ITEM_COUNT);
    pub const MQIACF_EXPIRY_TIME: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_EXPIRY_TIME,
    );
    pub const MQIACF_CONNECT_TIME: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CONNECT_TIME,
    );
    pub const MQIACF_DISCONNECT_TIME: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_DISCONNECT_TIME,
    );
    pub const MQIACF_HSUB: types::MQIACF = types::MQIACF(mqsys::MQIACF_HSUB);
    pub const MQIACF_SUBRQ_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SUBRQ_OPTIONS,
    );
    pub const MQIACF_XA_RMID: types::MQIACF = types::MQIACF(mqsys::MQIACF_XA_RMID);
    pub const MQIACF_XA_FLAGS: types::MQIACF = types::MQIACF(mqsys::MQIACF_XA_FLAGS);
    pub const MQIACF_XA_RETCODE: types::MQIACF = types::MQIACF(mqsys::MQIACF_XA_RETCODE);
    pub const MQIACF_XA_HANDLE: types::MQIACF = types::MQIACF(mqsys::MQIACF_XA_HANDLE);
    pub const MQIACF_XA_RETVAL: types::MQIACF = types::MQIACF(mqsys::MQIACF_XA_RETVAL);
    pub const MQIACF_STATUS_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_STATUS_TYPE,
    );
    pub const MQIACF_XA_COUNT: types::MQIACF = types::MQIACF(mqsys::MQIACF_XA_COUNT);
    pub const MQIACF_SELECTOR_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SELECTOR_COUNT,
    );
    pub const MQIACF_SELECTORS: types::MQIACF = types::MQIACF(mqsys::MQIACF_SELECTORS);
    pub const MQIACF_INTATTR_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_INTATTR_COUNT,
    );
    pub const MQIACF_INT_ATTRS: types::MQIACF = types::MQIACF(mqsys::MQIACF_INT_ATTRS);
    pub const MQIACF_SUBRQ_ACTION: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SUBRQ_ACTION,
    );
    pub const MQIACF_NUM_PUBS: types::MQIACF = types::MQIACF(mqsys::MQIACF_NUM_PUBS);
    pub const MQIACF_POINTER_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_POINTER_SIZE,
    );
    pub const MQIACF_REMOVE_AUTHREC: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_REMOVE_AUTHREC,
    );
    pub const MQIACF_XR_ATTRS: types::MQIACF = types::MQIACF(mqsys::MQIACF_XR_ATTRS);
    pub const MQIACF_APPL_FUNCTION_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_APPL_FUNCTION_TYPE,
    );
    pub const MQIACF_AMQP_ATTRS: types::MQIACF = types::MQIACF(mqsys::MQIACF_AMQP_ATTRS);
    pub const MQIACF_EXPORT_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_EXPORT_TYPE,
    );
    pub const MQIACF_EXPORT_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_EXPORT_ATTRS,
    );
    pub const MQIACF_SYSTEM_OBJECTS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSTEM_OBJECTS,
    );
    pub const MQIACF_CONNECTION_SWAP: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CONNECTION_SWAP,
    );
    pub const MQIACF_AMQP_DIAGNOSTICS_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_AMQP_DIAGNOSTICS_TYPE,
    );
    pub const MQIACF_BUFFER_POOL_LOCATION: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_BUFFER_POOL_LOCATION,
    );
    pub const MQIACF_LDAP_CONNECTION_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_LDAP_CONNECTION_STATUS,
    );
    pub const MQIACF_SYSP_MAX_ACE_POOL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_MAX_ACE_POOL,
    );
    pub const MQIACF_PAGECLAS: types::MQIACF = types::MQIACF(mqsys::MQIACF_PAGECLAS);
    pub const MQIACF_AUTH_REC_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_AUTH_REC_TYPE,
    );
    pub const MQIACF_SYSP_MAX_CONC_OFFLOADS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_MAX_CONC_OFFLOADS,
    );
    pub const MQIACF_SYSP_ZHYPERWRITE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_ZHYPERWRITE,
    );
    pub const MQIACF_Q_MGR_STATUS_LOG: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_STATUS_LOG,
    );
    pub const MQIACF_ARCHIVE_LOG_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_ARCHIVE_LOG_SIZE,
    );
    pub const MQIACF_MEDIA_LOG_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_MEDIA_LOG_SIZE,
    );
    pub const MQIACF_RESTART_LOG_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_RESTART_LOG_SIZE,
    );
    pub const MQIACF_REUSABLE_LOG_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_REUSABLE_LOG_SIZE,
    );
    pub const MQIACF_LOG_IN_USE: types::MQIACF = types::MQIACF(mqsys::MQIACF_LOG_IN_USE);
    pub const MQIACF_LOG_UTILIZATION: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_LOG_UTILIZATION,
    );
    pub const MQIACF_LOG_REDUCTION: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_LOG_REDUCTION,
    );
    pub const MQIACF_IGNORE_STATE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_IGNORE_STATE,
    );
    pub const MQIACF_MOVABLE_APPL_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_MOVABLE_APPL_COUNT,
    );
    pub const MQIACF_APPL_INFO_ATTRS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_APPL_INFO_ATTRS,
    );
    pub const MQIACF_APPL_MOVABLE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_APPL_MOVABLE,
    );
    pub const MQIACF_REMOTE_QMGR_ACTIVE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_REMOTE_QMGR_ACTIVE,
    );
    pub const MQIACF_APPL_INFO_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_APPL_INFO_TYPE,
    );
    pub const MQIACF_APPL_INFO_APPL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_APPL_INFO_APPL,
    );
    pub const MQIACF_APPL_INFO_QMGR: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_APPL_INFO_QMGR,
    );
    pub const MQIACF_APPL_INFO_LOCAL: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_APPL_INFO_LOCAL,
    );
    pub const MQIACF_APPL_IMMOVABLE_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_APPL_IMMOVABLE_COUNT,
    );
    pub const MQIACF_BALANCED: types::MQIACF = types::MQIACF(mqsys::MQIACF_BALANCED);
    pub const MQIACF_BALSTATE: types::MQIACF = types::MQIACF(mqsys::MQIACF_BALSTATE);
    pub const MQIACF_APPL_IMMOVABLE_REASON: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_APPL_IMMOVABLE_REASON,
    );
    pub const MQIACF_DS_ENCRYPTED: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_DS_ENCRYPTED,
    );
    pub const MQIACF_CUR_Q_FILE_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CUR_Q_FILE_SIZE,
    );
    pub const MQIACF_CUR_MAX_FILE_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CUR_MAX_FILE_SIZE,
    );
    pub const MQIACF_BALANCING_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_BALANCING_TYPE,
    );
    pub const MQIACF_BALANCING_OPTIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_BALANCING_OPTIONS,
    );
    pub const MQIACF_BALANCING_TIMEOUT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_BALANCING_TIMEOUT,
    );
    pub const MQIACF_SYSP_SMF_STAT_TIME_SECS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_SMF_STAT_TIME_SECS,
    );
    pub const MQIACF_SYSP_SMF_ACCT_TIME_MINS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_SMF_ACCT_TIME_MINS,
    );
    pub const MQIACF_SYSP_SMF_ACCT_TIME_SECS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_SMF_ACCT_TIME_SECS,
    );
    pub const MQIACF_Q_MGR_STATUS_INFO_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_STATUS_INFO_TYPE,
    );
    pub const MQIACF_Q_MGR_STATUS_INFO_Q_MGR: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_STATUS_INFO_Q_MGR,
    );
    pub const MQIACF_Q_MGR_STATUS_INFO_NHA: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_STATUS_INFO_NHA,
    );
    pub const MQIACF_AUTO_CLUSTER_TYPE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_AUTO_CLUSTER_TYPE,
    );
    pub const MQIACF_DATA_FS_IN_USE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_DATA_FS_IN_USE,
    );
    pub const MQIACF_DATA_FS_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_DATA_FS_SIZE,
    );
    pub const MQIACF_LOG_EXTENT_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_LOG_EXTENT_SIZE,
    );
    pub const MQIACF_LOG_FS_IN_USE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_LOG_FS_IN_USE,
    );
    pub const MQIACF_LOG_FS_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_LOG_FS_SIZE,
    );
    pub const MQIACF_LOG_PRIMARIES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_LOG_PRIMARIES,
    );
    pub const MQIACF_LOG_SECONDARIES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_LOG_SECONDARIES,
    );
    pub const MQIACF_LOG_TYPE: types::MQIACF = types::MQIACF(mqsys::MQIACF_LOG_TYPE);
    pub const MQIACF_NHA_INSTANCE_ACTV_CONNS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_NHA_INSTANCE_ACTV_CONNS,
    );
    pub const MQIACF_NHA_INSTANCE_BACKLOG: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_NHA_INSTANCE_BACKLOG,
    );
    pub const MQIACF_NHA_INSTANCE_IN_SYNC: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_NHA_INSTANCE_IN_SYNC,
    );
    pub const MQIACF_NHA_INSTANCE_ROLE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_NHA_INSTANCE_ROLE,
    );
    pub const MQIACF_NHA_IN_SYNC_INSTANCES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_NHA_IN_SYNC_INSTANCES,
    );
    pub const MQIACF_NHA_TOTAL_INSTANCES: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_NHA_TOTAL_INSTANCES,
    );
    pub const MQIACF_Q_MGR_FS_ENCRYPTED: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_FS_ENCRYPTED,
    );
    pub const MQIACF_Q_MGR_FS_IN_USE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_FS_IN_USE,
    );
    pub const MQIACF_Q_MGR_FS_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_Q_MGR_FS_SIZE,
    );
    pub const MQIACF_SYSP_ZHYPERLINK: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_ZHYPERLINK,
    );
    pub const MQIACF_CHECKPOINT_COUNT: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CHECKPOINT_COUNT,
    );
    pub const MQIACF_CHECKPOINT_OPERATIONS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CHECKPOINT_OPERATIONS,
    );
    pub const MQIACF_CHECKPOINT_SIZE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_CHECKPOINT_SIZE,
    );
    pub const MQIACF_NHA_GROUP_BACKLOG: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_NHA_GROUP_BACKLOG,
    );
    pub const MQIACF_NHA_GROUP_CONNECTED: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_NHA_GROUP_CONNECTED,
    );
    pub const MQIACF_NHA_GROUP_IN_SYNC: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_NHA_GROUP_IN_SYNC,
    );
    pub const MQIACF_NHA_GROUP_ROLE: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_NHA_GROUP_ROLE,
    );
    pub const MQIACF_NHA_GROUP_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_NHA_GROUP_STATUS,
    );
    pub const MQIACF_NHA_INSTANCE_STATUS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_NHA_INSTANCE_STATUS,
    );
    pub const MQIACF_NHA_TYPE: types::MQIACF = types::MQIACF(mqsys::MQIACF_NHA_TYPE);
    pub const MQIACF_MODE: types::MQIACF = types::MQIACF(mqsys::MQIACF_MODE);
    pub const MQIACF_ERROR_IDENTIFIER: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_ERROR_IDENTIFIER,
    );
    pub const MQIACF_SYSP_SMF_STAT_TIME_MINS: types::MQIACF = types::MQIACF(
        mqsys::MQIACF_SYSP_SMF_STAT_TIME_MINS,
    );
    pub const MQIACH_XMIT_PROTOCOL_TYPE: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_XMIT_PROTOCOL_TYPE,
    );
    pub const MQIACH_BATCH_SIZE: types::MQIACH = types::MQIACH(mqsys::MQIACH_BATCH_SIZE);
    pub const MQIACH_DISC_INTERVAL: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_DISC_INTERVAL,
    );
    pub const MQIACH_SHORT_TIMER: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SHORT_TIMER,
    );
    pub const MQIACH_SHORT_RETRY: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SHORT_RETRY,
    );
    pub const MQIACH_LONG_TIMER: types::MQIACH = types::MQIACH(mqsys::MQIACH_LONG_TIMER);
    pub const MQIACH_LONG_RETRY: types::MQIACH = types::MQIACH(mqsys::MQIACH_LONG_RETRY);
    pub const MQIACH_PUT_AUTHORITY: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_PUT_AUTHORITY,
    );
    pub const MQIACH_SEQUENCE_NUMBER_WRAP: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SEQUENCE_NUMBER_WRAP,
    );
    pub const MQIACH_MAX_MSG_LENGTH: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_MAX_MSG_LENGTH,
    );
    pub const MQIACH_CHANNEL_TYPE: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CHANNEL_TYPE,
    );
    pub const MQIACH_DATA_COUNT: types::MQIACH = types::MQIACH(mqsys::MQIACH_DATA_COUNT);
    pub const MQIACH_NAME_COUNT: types::MQIACH = types::MQIACH(mqsys::MQIACH_NAME_COUNT);
    pub const MQIACH_MSG_SEQUENCE_NUMBER: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_MSG_SEQUENCE_NUMBER,
    );
    pub const MQIACH_DATA_CONVERSION: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_DATA_CONVERSION,
    );
    pub const MQIACH_IN_DOUBT: types::MQIACH = types::MQIACH(mqsys::MQIACH_IN_DOUBT);
    pub const MQIACH_MCA_TYPE: types::MQIACH = types::MQIACH(mqsys::MQIACH_MCA_TYPE);
    pub const MQIACH_SESSION_COUNT: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SESSION_COUNT,
    );
    pub const MQIACH_ADAPTER: types::MQIACH = types::MQIACH(mqsys::MQIACH_ADAPTER);
    pub const MQIACH_COMMAND_COUNT: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_COMMAND_COUNT,
    );
    pub const MQIACH_SOCKET: types::MQIACH = types::MQIACH(mqsys::MQIACH_SOCKET);
    pub const MQIACH_PORT: types::MQIACH = types::MQIACH(mqsys::MQIACH_PORT);
    pub const MQIACH_CHANNEL_INSTANCE_TYPE: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CHANNEL_INSTANCE_TYPE,
    );
    pub const MQIACH_CHANNEL_INSTANCE_ATTRS: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CHANNEL_INSTANCE_ATTRS,
    );
    pub const MQIACH_CHANNEL_ERROR_DATA: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CHANNEL_ERROR_DATA,
    );
    pub const MQIACH_CHANNEL_TABLE: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CHANNEL_TABLE,
    );
    pub const MQIACH_CHANNEL_STATUS: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CHANNEL_STATUS,
    );
    pub const MQIACH_INDOUBT_STATUS: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_INDOUBT_STATUS,
    );
    pub const MQIACH_LAST_SEQ_NUMBER: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_LAST_SEQ_NUMBER,
    );
    pub const MQIACH_CURRENT_MSGS: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CURRENT_MSGS,
    );
    pub const MQIACH_CURRENT_SEQ_NUMBER: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CURRENT_SEQ_NUMBER,
    );
    pub const MQIACH_SSL_RETURN_CODE: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SSL_RETURN_CODE,
    );
    pub const MQIACH_MSGS: types::MQIACH = types::MQIACH(mqsys::MQIACH_MSGS);
    pub const MQIACH_BYTES_SENT: types::MQIACH = types::MQIACH(mqsys::MQIACH_BYTES_SENT);
    pub const MQIACH_BYTES_RCVD: types::MQIACH = types::MQIACH(mqsys::MQIACH_BYTES_RCVD);
    pub const MQIACH_BATCHES: types::MQIACH = types::MQIACH(mqsys::MQIACH_BATCHES);
    pub const MQIACH_BUFFERS_SENT: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_BUFFERS_SENT,
    );
    pub const MQIACH_BUFFERS_RCVD: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_BUFFERS_RCVD,
    );
    pub const MQIACH_LONG_RETRIES_LEFT: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_LONG_RETRIES_LEFT,
    );
    pub const MQIACH_SHORT_RETRIES_LEFT: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SHORT_RETRIES_LEFT,
    );
    pub const MQIACH_MCA_STATUS: types::MQIACH = types::MQIACH(mqsys::MQIACH_MCA_STATUS);
    pub const MQIACH_STOP_REQUESTED: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_STOP_REQUESTED,
    );
    pub const MQIACH_MR_COUNT: types::MQIACH = types::MQIACH(mqsys::MQIACH_MR_COUNT);
    pub const MQIACH_MR_INTERVAL: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_MR_INTERVAL,
    );
    pub const MQIACH_NPM_SPEED: types::MQIACH = types::MQIACH(mqsys::MQIACH_NPM_SPEED);
    pub const MQIACH_HB_INTERVAL: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_HB_INTERVAL,
    );
    pub const MQIACH_BATCH_INTERVAL: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_BATCH_INTERVAL,
    );
    pub const MQIACH_NETWORK_PRIORITY: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_NETWORK_PRIORITY,
    );
    pub const MQIACH_KEEP_ALIVE_INTERVAL: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_KEEP_ALIVE_INTERVAL,
    );
    pub const MQIACH_BATCH_HB: types::MQIACH = types::MQIACH(mqsys::MQIACH_BATCH_HB);
    pub const MQIACH_SSL_CLIENT_AUTH: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SSL_CLIENT_AUTH,
    );
    pub const MQIACH_ALLOC_RETRY: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_ALLOC_RETRY,
    );
    pub const MQIACH_ALLOC_FAST_TIMER: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_ALLOC_FAST_TIMER,
    );
    pub const MQIACH_ALLOC_SLOW_TIMER: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_ALLOC_SLOW_TIMER,
    );
    pub const MQIACH_DISC_RETRY: types::MQIACH = types::MQIACH(mqsys::MQIACH_DISC_RETRY);
    pub const MQIACH_PORT_NUMBER: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_PORT_NUMBER,
    );
    pub const MQIACH_HDR_COMPRESSION: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_HDR_COMPRESSION,
    );
    pub const MQIACH_MSG_COMPRESSION: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_MSG_COMPRESSION,
    );
    pub const MQIACH_CLWL_CHANNEL_RANK: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CLWL_CHANNEL_RANK,
    );
    pub const MQIACH_CLWL_CHANNEL_PRIORITY: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CLWL_CHANNEL_PRIORITY,
    );
    pub const MQIACH_CLWL_CHANNEL_WEIGHT: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CLWL_CHANNEL_WEIGHT,
    );
    pub const MQIACH_CHANNEL_DISP: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CHANNEL_DISP,
    );
    pub const MQIACH_INBOUND_DISP: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_INBOUND_DISP,
    );
    pub const MQIACH_CHANNEL_TYPES: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CHANNEL_TYPES,
    );
    pub const MQIACH_ADAPS_STARTED: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_ADAPS_STARTED,
    );
    pub const MQIACH_ADAPS_MAX: types::MQIACH = types::MQIACH(mqsys::MQIACH_ADAPS_MAX);
    pub const MQIACH_DISPS_STARTED: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_DISPS_STARTED,
    );
    pub const MQIACH_DISPS_MAX: types::MQIACH = types::MQIACH(mqsys::MQIACH_DISPS_MAX);
    pub const MQIACH_SSLTASKS_STARTED: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SSLTASKS_STARTED,
    );
    pub const MQIACH_SSLTASKS_MAX: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SSLTASKS_MAX,
    );
    pub const MQIACH_CURRENT_CHL: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CURRENT_CHL,
    );
    pub const MQIACH_CURRENT_CHL_MAX: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CURRENT_CHL_MAX,
    );
    pub const MQIACH_CURRENT_CHL_TCP: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CURRENT_CHL_TCP,
    );
    pub const MQIACH_CURRENT_CHL_LU62: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CURRENT_CHL_LU62,
    );
    pub const MQIACH_ACTIVE_CHL: types::MQIACH = types::MQIACH(mqsys::MQIACH_ACTIVE_CHL);
    pub const MQIACH_ACTIVE_CHL_MAX: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_ACTIVE_CHL_MAX,
    );
    pub const MQIACH_ACTIVE_CHL_PAUSED: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_ACTIVE_CHL_PAUSED,
    );
    pub const MQIACH_ACTIVE_CHL_STARTED: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_ACTIVE_CHL_STARTED,
    );
    pub const MQIACH_ACTIVE_CHL_STOPPED: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_ACTIVE_CHL_STOPPED,
    );
    pub const MQIACH_ACTIVE_CHL_RETRY: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_ACTIVE_CHL_RETRY,
    );
    pub const MQIACH_LISTENER_STATUS: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_LISTENER_STATUS,
    );
    pub const MQIACH_SHARED_CHL_RESTART: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SHARED_CHL_RESTART,
    );
    pub const MQIACH_LISTENER_CONTROL: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_LISTENER_CONTROL,
    );
    pub const MQIACH_BACKLOG: types::MQIACH = types::MQIACH(mqsys::MQIACH_BACKLOG);
    pub const MQIACH_XMITQ_TIME_INDICATOR: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_XMITQ_TIME_INDICATOR,
    );
    pub const MQIACH_NETWORK_TIME_INDICATOR: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_NETWORK_TIME_INDICATOR,
    );
    pub const MQIACH_EXIT_TIME_INDICATOR: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_EXIT_TIME_INDICATOR,
    );
    pub const MQIACH_BATCH_SIZE_INDICATOR: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_BATCH_SIZE_INDICATOR,
    );
    pub const MQIACH_XMITQ_MSGS_AVAILABLE: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_XMITQ_MSGS_AVAILABLE,
    );
    pub const MQIACH_CHANNEL_SUBSTATE: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CHANNEL_SUBSTATE,
    );
    pub const MQIACH_SSL_KEY_RESETS: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SSL_KEY_RESETS,
    );
    pub const MQIACH_COMPRESSION_RATE: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_COMPRESSION_RATE,
    );
    pub const MQIACH_COMPRESSION_TIME: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_COMPRESSION_TIME,
    );
    pub const MQIACH_MAX_XMIT_SIZE: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_MAX_XMIT_SIZE,
    );
    pub const MQIACH_DEF_CHANNEL_DISP: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_DEF_CHANNEL_DISP,
    );
    pub const MQIACH_SHARING_CONVERSATIONS: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SHARING_CONVERSATIONS,
    );
    pub const MQIACH_MAX_SHARING_CONVS: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_MAX_SHARING_CONVS,
    );
    pub const MQIACH_CURRENT_SHARING_CONVS: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CURRENT_SHARING_CONVS,
    );
    pub const MQIACH_MAX_INSTANCES: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_MAX_INSTANCES,
    );
    pub const MQIACH_MAX_INSTS_PER_CLIENT: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_MAX_INSTS_PER_CLIENT,
    );
    pub const MQIACH_CLIENT_CHANNEL_WEIGHT: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CLIENT_CHANNEL_WEIGHT,
    );
    pub const MQIACH_CONNECTION_AFFINITY: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CONNECTION_AFFINITY,
    );
    pub const MQIACH_AUTH_INFO_TYPES: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_AUTH_INFO_TYPES,
    );
    pub const MQIACH_RESET_REQUESTED: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_RESET_REQUESTED,
    );
    pub const MQIACH_BATCH_DATA_LIMIT: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_BATCH_DATA_LIMIT,
    );
    pub const MQIACH_MSG_HISTORY: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_MSG_HISTORY,
    );
    pub const MQIACH_MULTICAST_PROPERTIES: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_MULTICAST_PROPERTIES,
    );
    pub const MQIACH_NEW_SUBSCRIBER_HISTORY: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_NEW_SUBSCRIBER_HISTORY,
    );
    pub const MQIACH_MC_HB_INTERVAL: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_MC_HB_INTERVAL,
    );
    pub const MQIACH_USE_CLIENT_ID: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_USE_CLIENT_ID,
    );
    pub const MQIACH_MQTT_KEEP_ALIVE: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_MQTT_KEEP_ALIVE,
    );
    pub const MQIACH_IN_DOUBT_IN: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_IN_DOUBT_IN,
    );
    pub const MQIACH_IN_DOUBT_OUT: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_IN_DOUBT_OUT,
    );
    pub const MQIACH_MSGS_SENT: types::MQIACH = types::MQIACH(mqsys::MQIACH_MSGS_SENT);
    pub const MQIACH_MSGS_RCVD: types::MQIACH = types::MQIACH(mqsys::MQIACH_MSGS_RCVD);
    pub const MQIACH_PENDING_OUT: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_PENDING_OUT,
    );
    pub const MQIACH_AVAILABLE_CIPHERSPECS: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_AVAILABLE_CIPHERSPECS,
    );
    pub const MQIACH_MATCH: types::MQIACH = types::MQIACH(mqsys::MQIACH_MATCH);
    pub const MQIACH_USER_SOURCE: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_USER_SOURCE,
    );
    pub const MQIACH_WARNING: types::MQIACH = types::MQIACH(mqsys::MQIACH_WARNING);
    pub const MQIACH_DEF_RECONNECT: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_DEF_RECONNECT,
    );
    pub const MQIACH_CHANNEL_SUMMARY_ATTRS: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CHANNEL_SUMMARY_ATTRS,
    );
    pub const MQIACH_PROTOCOL: types::MQIACH = types::MQIACH(mqsys::MQIACH_PROTOCOL);
    pub const MQIACH_AMQP_KEEP_ALIVE: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_AMQP_KEEP_ALIVE,
    );
    pub const MQIACH_SECURITY_PROTOCOL: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SECURITY_PROTOCOL,
    );
    pub const MQIACH_SPL_PROTECTION: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_SPL_PROTECTION,
    );
    pub const MQIACH_LAST_SEQUENCE_NUMBER: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_LAST_SEQUENCE_NUMBER,
    );
    pub const MQIACH_CURRENT_SEQUENCE_NUMBER: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_CURRENT_SEQUENCE_NUMBER,
    );
    pub const MQIACH_BYTES_RECEIVED: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_BYTES_RECEIVED,
    );
    pub const MQIACH_BUFFERS_RECEIVED: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_BUFFERS_RECEIVED,
    );
    pub const MQIACH_MSGS_RECEIVED: types::MQIACH = types::MQIACH(
        mqsys::MQIACH_MSGS_RECEIVED,
    );
    pub const MQIAMO64_AVG_Q_TIME: types::MQIAMO64 = types::MQIAMO64(
        mqsys::MQIAMO64_AVG_Q_TIME,
    );
    pub const MQIAMO64_Q_TIME_AVG: types::MQIAMO64 = types::MQIAMO64(
        mqsys::MQIAMO64_Q_TIME_AVG,
    );
    pub const MQIAMO64_Q_TIME_MAX: types::MQIAMO64 = types::MQIAMO64(
        mqsys::MQIAMO64_Q_TIME_MAX,
    );
    pub const MQIAMO64_Q_TIME_MIN: types::MQIAMO64 = types::MQIAMO64(
        mqsys::MQIAMO64_Q_TIME_MIN,
    );
    pub const MQIAMO64_BROWSE_BYTES: types::MQIAMO64 = types::MQIAMO64(
        mqsys::MQIAMO64_BROWSE_BYTES,
    );
    pub const MQIAMO64_BYTES: types::MQIAMO64 = types::MQIAMO64(mqsys::MQIAMO64_BYTES);
    pub const MQIAMO64_GET_BYTES: types::MQIAMO64 = types::MQIAMO64(
        mqsys::MQIAMO64_GET_BYTES,
    );
    pub const MQIAMO64_PUT_BYTES: types::MQIAMO64 = types::MQIAMO64(
        mqsys::MQIAMO64_PUT_BYTES,
    );
    pub const MQIAMO64_TOPIC_PUT_BYTES: types::MQIAMO64 = types::MQIAMO64(
        mqsys::MQIAMO64_TOPIC_PUT_BYTES,
    );
    pub const MQIAMO64_PUBLISH_MSG_BYTES: types::MQIAMO64 = types::MQIAMO64(
        mqsys::MQIAMO64_PUBLISH_MSG_BYTES,
    );
    pub const MQIAMO64_HIGHRES_TIME: types::MQIAMO64 = types::MQIAMO64(
        mqsys::MQIAMO64_HIGHRES_TIME,
    );
    pub const MQIAMO64_QMGR_OP_DURATION: types::MQIAMO64 = types::MQIAMO64(
        mqsys::MQIAMO64_QMGR_OP_DURATION,
    );
    pub const MQIAMO64_MONITOR_INTERVAL: types::MQIAMO64 = types::MQIAMO64(
        mqsys::MQIAMO64_MONITOR_INTERVAL,
    );
    pub const MQIAMO_AVG_BATCH_SIZE: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_AVG_BATCH_SIZE,
    );
    pub const MQIAMO_AVG_Q_TIME: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_AVG_Q_TIME);
    pub const MQIAMO_BACKOUTS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_BACKOUTS);
    pub const MQIAMO_BROWSES: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_BROWSES);
    pub const MQIAMO_BROWSE_MAX_BYTES: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_BROWSE_MAX_BYTES,
    );
    pub const MQIAMO_BROWSE_MIN_BYTES: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_BROWSE_MIN_BYTES,
    );
    pub const MQIAMO_BROWSES_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_BROWSES_FAILED,
    );
    pub const MQIAMO_CLOSES: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_CLOSES);
    pub const MQIAMO_COMMITS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_COMMITS);
    pub const MQIAMO_COMMITS_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_COMMITS_FAILED,
    );
    pub const MQIAMO_CONNS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_CONNS);
    pub const MQIAMO_CONNS_MAX: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_CONNS_MAX);
    pub const MQIAMO_DISCS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_DISCS);
    pub const MQIAMO_DISCS_IMPLICIT: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_DISCS_IMPLICIT,
    );
    pub const MQIAMO_DISC_TYPE: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_DISC_TYPE);
    pub const MQIAMO_EXIT_TIME_AVG: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_EXIT_TIME_AVG,
    );
    pub const MQIAMO_EXIT_TIME_MAX: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_EXIT_TIME_MAX,
    );
    pub const MQIAMO_EXIT_TIME_MIN: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_EXIT_TIME_MIN,
    );
    pub const MQIAMO_FULL_BATCHES: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_FULL_BATCHES,
    );
    pub const MQIAMO_GENERATED_MSGS: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_GENERATED_MSGS,
    );
    pub const MQIAMO_GETS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_GETS);
    pub const MQIAMO_GET_MAX_BYTES: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_GET_MAX_BYTES,
    );
    pub const MQIAMO_GET_MIN_BYTES: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_GET_MIN_BYTES,
    );
    pub const MQIAMO_GETS_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_GETS_FAILED,
    );
    pub const MQIAMO_INCOMPLETE_BATCHES: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_INCOMPLETE_BATCHES,
    );
    pub const MQIAMO_INQS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_INQS);
    pub const MQIAMO_MSGS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_MSGS);
    pub const MQIAMO_NET_TIME_AVG: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_NET_TIME_AVG,
    );
    pub const MQIAMO_NET_TIME_MAX: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_NET_TIME_MAX,
    );
    pub const MQIAMO_NET_TIME_MIN: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_NET_TIME_MIN,
    );
    pub const MQIAMO_OBJECT_COUNT: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_OBJECT_COUNT,
    );
    pub const MQIAMO_OPENS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_OPENS);
    pub const MQIAMO_PUT1S: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_PUT1S);
    pub const MQIAMO_PUTS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_PUTS);
    pub const MQIAMO_PUT_MAX_BYTES: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_PUT_MAX_BYTES,
    );
    pub const MQIAMO_PUT_MIN_BYTES: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_PUT_MIN_BYTES,
    );
    pub const MQIAMO_PUT_RETRIES: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_PUT_RETRIES,
    );
    pub const MQIAMO_Q_MAX_DEPTH: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_Q_MAX_DEPTH,
    );
    pub const MQIAMO_Q_MIN_DEPTH: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_Q_MIN_DEPTH,
    );
    pub const MQIAMO_Q_TIME_AVG: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_Q_TIME_AVG);
    pub const MQIAMO_Q_TIME_MAX: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_Q_TIME_MAX);
    pub const MQIAMO_Q_TIME_MIN: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_Q_TIME_MIN);
    pub const MQIAMO_SETS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_SETS);
    pub const MQIAMO_CONNS_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_CONNS_FAILED,
    );
    pub const MQIAMO_OPENS_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_OPENS_FAILED,
    );
    pub const MQIAMO_INQS_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_INQS_FAILED,
    );
    pub const MQIAMO_SETS_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_SETS_FAILED,
    );
    pub const MQIAMO_PUTS_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_PUTS_FAILED,
    );
    pub const MQIAMO_PUT1S_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_PUT1S_FAILED,
    );
    pub const MQIAMO_CLOSES_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_CLOSES_FAILED,
    );
    pub const MQIAMO_MSGS_EXPIRED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_MSGS_EXPIRED,
    );
    pub const MQIAMO_MSGS_NOT_QUEUED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_MSGS_NOT_QUEUED,
    );
    pub const MQIAMO_MSGS_PURGED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_MSGS_PURGED,
    );
    pub const MQIAMO_SUBS_DUR: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_SUBS_DUR);
    pub const MQIAMO_SUBS_NDUR: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_SUBS_NDUR);
    pub const MQIAMO_SUBS_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_SUBS_FAILED,
    );
    pub const MQIAMO_SUBRQS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_SUBRQS);
    pub const MQIAMO_SUBRQS_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_SUBRQS_FAILED,
    );
    pub const MQIAMO_CBS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_CBS);
    pub const MQIAMO_CBS_FAILED: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_CBS_FAILED);
    pub const MQIAMO_CTLS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_CTLS);
    pub const MQIAMO_CTLS_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_CTLS_FAILED,
    );
    pub const MQIAMO_STATS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_STATS);
    pub const MQIAMO_STATS_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_STATS_FAILED,
    );
    pub const MQIAMO_SUB_DUR_HIGHWATER: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_SUB_DUR_HIGHWATER,
    );
    pub const MQIAMO_SUB_DUR_LOWWATER: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_SUB_DUR_LOWWATER,
    );
    pub const MQIAMO_SUB_NDUR_HIGHWATER: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_SUB_NDUR_HIGHWATER,
    );
    pub const MQIAMO_SUB_NDUR_LOWWATER: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_SUB_NDUR_LOWWATER,
    );
    pub const MQIAMO_TOPIC_PUTS: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_TOPIC_PUTS);
    pub const MQIAMO_TOPIC_PUTS_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOPIC_PUTS_FAILED,
    );
    pub const MQIAMO_TOPIC_PUT1S: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOPIC_PUT1S,
    );
    pub const MQIAMO_TOPIC_PUT1S_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOPIC_PUT1S_FAILED,
    );
    pub const MQIAMO_PUBLISH_MSG_COUNT: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_PUBLISH_MSG_COUNT,
    );
    pub const MQIAMO_UNSUBS_DUR: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_UNSUBS_DUR);
    pub const MQIAMO_UNSUBS_NDUR: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_UNSUBS_NDUR,
    );
    pub const MQIAMO_UNSUBS_FAILED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_UNSUBS_FAILED,
    );
    pub const MQIAMO_INTERVAL: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_INTERVAL);
    pub const MQIAMO_MSGS_SENT: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_MSGS_SENT);
    pub const MQIAMO_BYTES_SENT: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_BYTES_SENT);
    pub const MQIAMO_REPAIR_BYTES: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_REPAIR_BYTES,
    );
    pub const MQIAMO_FEEDBACK_MODE: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_FEEDBACK_MODE,
    );
    pub const MQIAMO_RELIABILITY_TYPE: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_RELIABILITY_TYPE,
    );
    pub const MQIAMO_LATE_JOIN_MARK: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_LATE_JOIN_MARK,
    );
    pub const MQIAMO_NACKS_RCVD: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_NACKS_RCVD);
    pub const MQIAMO_REPAIR_PKTS: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_REPAIR_PKTS,
    );
    pub const MQIAMO_HISTORY_PKTS: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_HISTORY_PKTS,
    );
    pub const MQIAMO_PENDING_PKTS: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_PENDING_PKTS,
    );
    pub const MQIAMO_PKT_RATE: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_PKT_RATE);
    pub const MQIAMO_MCAST_XMIT_RATE: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_MCAST_XMIT_RATE,
    );
    pub const MQIAMO_MCAST_BATCH_TIME: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_MCAST_BATCH_TIME,
    );
    pub const MQIAMO_MCAST_HEARTBEAT: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_MCAST_HEARTBEAT,
    );
    pub const MQIAMO_DEST_DATA_PORT: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_DEST_DATA_PORT,
    );
    pub const MQIAMO_DEST_REPAIR_PORT: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_DEST_REPAIR_PORT,
    );
    pub const MQIAMO_ACKS_RCVD: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_ACKS_RCVD);
    pub const MQIAMO_ACTIVE_ACKERS: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_ACTIVE_ACKERS,
    );
    pub const MQIAMO_PKTS_SENT: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_PKTS_SENT);
    pub const MQIAMO_TOTAL_REPAIR_PKTS: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOTAL_REPAIR_PKTS,
    );
    pub const MQIAMO_TOTAL_PKTS_SENT: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOTAL_PKTS_SENT,
    );
    pub const MQIAMO_TOTAL_MSGS_SENT: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOTAL_MSGS_SENT,
    );
    pub const MQIAMO_TOTAL_BYTES_SENT: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOTAL_BYTES_SENT,
    );
    pub const MQIAMO_NUM_STREAMS: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_NUM_STREAMS,
    );
    pub const MQIAMO_ACK_FEEDBACK: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_ACK_FEEDBACK,
    );
    pub const MQIAMO_NACK_FEEDBACK: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_NACK_FEEDBACK,
    );
    pub const MQIAMO_PKTS_LOST: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_PKTS_LOST);
    pub const MQIAMO_MSGS_RCVD: types::MQIAMO = types::MQIAMO(mqsys::MQIAMO_MSGS_RCVD);
    pub const MQIAMO_MSG_BYTES_RCVD: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_MSG_BYTES_RCVD,
    );
    pub const MQIAMO_MSGS_DELIVERED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_MSGS_DELIVERED,
    );
    pub const MQIAMO_PKTS_PROCESSED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_PKTS_PROCESSED,
    );
    pub const MQIAMO_PKTS_DELIVERED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_PKTS_DELIVERED,
    );
    pub const MQIAMO_PKTS_DROPPED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_PKTS_DROPPED,
    );
    pub const MQIAMO_PKTS_DUPLICATED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_PKTS_DUPLICATED,
    );
    pub const MQIAMO_NACKS_CREATED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_NACKS_CREATED,
    );
    pub const MQIAMO_NACK_PKTS_SENT: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_NACK_PKTS_SENT,
    );
    pub const MQIAMO_REPAIR_PKTS_RQSTD: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_REPAIR_PKTS_RQSTD,
    );
    pub const MQIAMO_REPAIR_PKTS_RCVD: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_REPAIR_PKTS_RCVD,
    );
    pub const MQIAMO_PKTS_REPAIRED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_PKTS_REPAIRED,
    );
    pub const MQIAMO_TOTAL_MSGS_RCVD: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOTAL_MSGS_RCVD,
    );
    pub const MQIAMO_TOTAL_MSG_BYTES_RCVD: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOTAL_MSG_BYTES_RCVD,
    );
    pub const MQIAMO_TOTAL_REPAIR_PKTS_RCVD: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOTAL_REPAIR_PKTS_RCVD,
    );
    pub const MQIAMO_TOTAL_REPAIR_PKTS_RQSTD: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOTAL_REPAIR_PKTS_RQSTD,
    );
    pub const MQIAMO_TOTAL_MSGS_PROCESSED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOTAL_MSGS_PROCESSED,
    );
    pub const MQIAMO_TOTAL_MSGS_SELECTED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOTAL_MSGS_SELECTED,
    );
    pub const MQIAMO_TOTAL_MSGS_EXPIRED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOTAL_MSGS_EXPIRED,
    );
    pub const MQIAMO_TOTAL_MSGS_DELIVERED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOTAL_MSGS_DELIVERED,
    );
    pub const MQIAMO_TOTAL_MSGS_RETURNED: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_TOTAL_MSGS_RETURNED,
    );
    pub const MQIAMO_MONITOR_CLASS: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_MONITOR_CLASS,
    );
    pub const MQIAMO_MONITOR_TYPE: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_MONITOR_TYPE,
    );
    pub const MQIAMO_MONITOR_ELEMENT: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_MONITOR_ELEMENT,
    );
    pub const MQIAMO_MONITOR_DATATYPE: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_MONITOR_DATATYPE,
    );
    pub const MQIAMO_MONITOR_FLAGS: types::MQIAMO = types::MQIAMO(
        mqsys::MQIAMO_MONITOR_FLAGS,
    );
    pub const MQIAMO_MONITOR_UNIT: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
        mqsys::MQIAMO_MONITOR_UNIT,
    );
    pub const MQIAMO_MONITOR_DELTA: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
        mqsys::MQIAMO_MONITOR_DELTA,
    );
    pub const MQIAMO_MONITOR_LSN: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
        mqsys::MQIAMO_MONITOR_LSN,
    );
    pub const MQIAMO_MONITOR_HUNDREDTHS: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
        mqsys::MQIAMO_MONITOR_HUNDREDTHS,
    );
    pub const MQIAMO_MONITOR_KB: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
        mqsys::MQIAMO_MONITOR_KB,
    );
    pub const MQIAMO_MONITOR_PERCENT: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
        mqsys::MQIAMO_MONITOR_PERCENT,
    );
    pub const MQIAMO_MONITOR_MICROSEC: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
        mqsys::MQIAMO_MONITOR_MICROSEC,
    );
    pub const MQIAMO_MONITOR_MB: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
        mqsys::MQIAMO_MONITOR_MB,
    );
    pub const MQIAMO_MONITOR_GB: types::MQIAMO_MONITOR_DATATYPE = types::MQIAMO_MONITOR_DATATYPE(
        mqsys::MQIAMO_MONITOR_GB,
    );
    pub const MQIAMO_MONITOR_FLAGS_NONE: types::MQIAMO_MONITOR_FLAGS = types::MQIAMO_MONITOR_FLAGS(
        mqsys::MQIAMO_MONITOR_FLAGS_NONE,
    );
    pub const MQIAMO_MONITOR_FLAGS_OBJNAME: types::MQIAMO_MONITOR_FLAGS = types::MQIAMO_MONITOR_FLAGS(
        mqsys::MQIAMO_MONITOR_FLAGS_OBJNAME,
    );
    pub const MQIASY_VERSION: types::MQIASY = types::MQIASY(mqsys::MQIASY_VERSION);
    pub const MQIASY_BAG_OPTIONS: types::MQIASY = types::MQIASY(
        mqsys::MQIASY_BAG_OPTIONS,
    );
    pub const MQIASY_REASON: types::MQIASY = types::MQIASY(mqsys::MQIASY_REASON);
    pub const MQIASY_COMP_CODE: types::MQIASY = types::MQIASY(mqsys::MQIASY_COMP_CODE);
    pub const MQIASY_CONTROL: types::MQIASY = types::MQIASY(mqsys::MQIASY_CONTROL);
    pub const MQIASY_MSG_SEQ_NUMBER: types::MQIASY = types::MQIASY(
        mqsys::MQIASY_MSG_SEQ_NUMBER,
    );
    pub const MQIASY_COMMAND: types::MQIASY = types::MQIASY(mqsys::MQIASY_COMMAND);
    pub const MQIASY_TYPE: types::MQIASY = types::MQIASY(mqsys::MQIASY_TYPE);
    pub const MQIASY_CODED_CHAR_SET_ID: types::MQIASY = types::MQIASY(
        mqsys::MQIASY_CODED_CHAR_SET_ID,
    );
    pub const MQIAV_UNDEFINED: types::MQIAV = types::MQIAV(mqsys::MQIAV_UNDEFINED);
    pub const MQIAV_NOT_APPLICABLE: types::MQIAV = types::MQIAV(
        mqsys::MQIAV_NOT_APPLICABLE,
    );
    pub const MQIA_APPL_TYPE: types::MQIA = types::MQIA(mqsys::MQIA_APPL_TYPE);
    pub const MQIA_CODED_CHAR_SET_ID: types::MQIA = types::MQIA(
        mqsys::MQIA_CODED_CHAR_SET_ID,
    );
    pub const MQIA_CURRENT_Q_DEPTH: types::MQIA = types::MQIA(
        mqsys::MQIA_CURRENT_Q_DEPTH,
    );
    pub const MQIA_DEF_INPUT_OPEN_OPTION: types::MQIA = types::MQIA(
        mqsys::MQIA_DEF_INPUT_OPEN_OPTION,
    );
    pub const MQIA_DEF_PERSISTENCE: types::MQIA = types::MQIA(
        mqsys::MQIA_DEF_PERSISTENCE,
    );
    pub const MQIA_DEF_PRIORITY: types::MQIA = types::MQIA(mqsys::MQIA_DEF_PRIORITY);
    pub const MQIA_DEFINITION_TYPE: types::MQIA = types::MQIA(
        mqsys::MQIA_DEFINITION_TYPE,
    );
    pub const MQIA_HARDEN_GET_BACKOUT: types::MQIA = types::MQIA(
        mqsys::MQIA_HARDEN_GET_BACKOUT,
    );
    pub const MQIA_INHIBIT_GET: types::MQIA = types::MQIA(mqsys::MQIA_INHIBIT_GET);
    pub const MQIA_INHIBIT_PUT: types::MQIA = types::MQIA(mqsys::MQIA_INHIBIT_PUT);
    pub const MQIA_MAX_HANDLES: types::MQIA = types::MQIA(mqsys::MQIA_MAX_HANDLES);
    pub const MQIA_USAGE: types::MQIA = types::MQIA(mqsys::MQIA_USAGE);
    pub const MQIA_MAX_MSG_LENGTH: types::MQIA = types::MQIA(mqsys::MQIA_MAX_MSG_LENGTH);
    pub const MQIA_MAX_PRIORITY: types::MQIA = types::MQIA(mqsys::MQIA_MAX_PRIORITY);
    pub const MQIA_MAX_Q_DEPTH: types::MQIA = types::MQIA(mqsys::MQIA_MAX_Q_DEPTH);
    pub const MQIA_MSG_DELIVERY_SEQUENCE: types::MQIA = types::MQIA(
        mqsys::MQIA_MSG_DELIVERY_SEQUENCE,
    );
    pub const MQIA_OPEN_INPUT_COUNT: types::MQIA = types::MQIA(
        mqsys::MQIA_OPEN_INPUT_COUNT,
    );
    pub const MQIA_OPEN_OUTPUT_COUNT: types::MQIA = types::MQIA(
        mqsys::MQIA_OPEN_OUTPUT_COUNT,
    );
    pub const MQIA_NAME_COUNT: types::MQIA = types::MQIA(mqsys::MQIA_NAME_COUNT);
    pub const MQIA_Q_TYPE: types::MQIA = types::MQIA(mqsys::MQIA_Q_TYPE);
    pub const MQIA_RETENTION_INTERVAL: types::MQIA = types::MQIA(
        mqsys::MQIA_RETENTION_INTERVAL,
    );
    pub const MQIA_BACKOUT_THRESHOLD: types::MQIA = types::MQIA(
        mqsys::MQIA_BACKOUT_THRESHOLD,
    );
    pub const MQIA_SHAREABILITY: types::MQIA = types::MQIA(mqsys::MQIA_SHAREABILITY);
    pub const MQIA_TRIGGER_CONTROL: types::MQIA = types::MQIA(
        mqsys::MQIA_TRIGGER_CONTROL,
    );
    pub const MQIA_TRIGGER_INTERVAL: types::MQIA = types::MQIA(
        mqsys::MQIA_TRIGGER_INTERVAL,
    );
    pub const MQIA_TRIGGER_MSG_PRIORITY: types::MQIA = types::MQIA(
        mqsys::MQIA_TRIGGER_MSG_PRIORITY,
    );
    pub const MQIA_CPI_LEVEL: types::MQIA = types::MQIA(mqsys::MQIA_CPI_LEVEL);
    pub const MQIA_TRIGGER_TYPE: types::MQIA = types::MQIA(mqsys::MQIA_TRIGGER_TYPE);
    pub const MQIA_TRIGGER_DEPTH: types::MQIA = types::MQIA(mqsys::MQIA_TRIGGER_DEPTH);
    pub const MQIA_SYNCPOINT: types::MQIA = types::MQIA(mqsys::MQIA_SYNCPOINT);
    pub const MQIA_COMMAND_LEVEL: types::MQIA = types::MQIA(mqsys::MQIA_COMMAND_LEVEL);
    pub const MQIA_PLATFORM: types::MQIA = types::MQIA(mqsys::MQIA_PLATFORM);
    pub const MQIA_MAX_UNCOMMITTED_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_MAX_UNCOMMITTED_MSGS,
    );
    pub const MQIA_DIST_LISTS: types::MQIA = types::MQIA(mqsys::MQIA_DIST_LISTS);
    pub const MQIA_TIME_SINCE_RESET: types::MQIA = types::MQIA(
        mqsys::MQIA_TIME_SINCE_RESET,
    );
    pub const MQIA_HIGH_Q_DEPTH: types::MQIA = types::MQIA(mqsys::MQIA_HIGH_Q_DEPTH);
    pub const MQIA_MSG_ENQ_COUNT: types::MQIA = types::MQIA(mqsys::MQIA_MSG_ENQ_COUNT);
    pub const MQIA_MSG_DEQ_COUNT: types::MQIA = types::MQIA(mqsys::MQIA_MSG_DEQ_COUNT);
    pub const MQIA_EXPIRY_INTERVAL: types::MQIA = types::MQIA(
        mqsys::MQIA_EXPIRY_INTERVAL,
    );
    pub const MQIA_Q_DEPTH_HIGH_LIMIT: types::MQIA = types::MQIA(
        mqsys::MQIA_Q_DEPTH_HIGH_LIMIT,
    );
    pub const MQIA_Q_DEPTH_LOW_LIMIT: types::MQIA = types::MQIA(
        mqsys::MQIA_Q_DEPTH_LOW_LIMIT,
    );
    pub const MQIA_Q_DEPTH_MAX_EVENT: types::MQIA = types::MQIA(
        mqsys::MQIA_Q_DEPTH_MAX_EVENT,
    );
    pub const MQIA_Q_DEPTH_HIGH_EVENT: types::MQIA = types::MQIA(
        mqsys::MQIA_Q_DEPTH_HIGH_EVENT,
    );
    pub const MQIA_Q_DEPTH_LOW_EVENT: types::MQIA = types::MQIA(
        mqsys::MQIA_Q_DEPTH_LOW_EVENT,
    );
    pub const MQIA_SCOPE: types::MQIA = types::MQIA(mqsys::MQIA_SCOPE);
    pub const MQIA_Q_SERVICE_INTERVAL_EVENT: types::MQIA = types::MQIA(
        mqsys::MQIA_Q_SERVICE_INTERVAL_EVENT,
    );
    pub const MQIA_AUTHORITY_EVENT: types::MQIA = types::MQIA(
        mqsys::MQIA_AUTHORITY_EVENT,
    );
    pub const MQIA_INHIBIT_EVENT: types::MQIA = types::MQIA(mqsys::MQIA_INHIBIT_EVENT);
    pub const MQIA_LOCAL_EVENT: types::MQIA = types::MQIA(mqsys::MQIA_LOCAL_EVENT);
    pub const MQIA_REMOTE_EVENT: types::MQIA = types::MQIA(mqsys::MQIA_REMOTE_EVENT);
    pub const MQIA_CONFIGURATION_EVENT: types::MQIA = types::MQIA(
        mqsys::MQIA_CONFIGURATION_EVENT,
    );
    pub const MQIA_START_STOP_EVENT: types::MQIA = types::MQIA(
        mqsys::MQIA_START_STOP_EVENT,
    );
    pub const MQIA_PERFORMANCE_EVENT: types::MQIA = types::MQIA(
        mqsys::MQIA_PERFORMANCE_EVENT,
    );
    pub const MQIA_Q_SERVICE_INTERVAL: types::MQIA = types::MQIA(
        mqsys::MQIA_Q_SERVICE_INTERVAL,
    );
    pub const MQIA_CHANNEL_AUTO_DEF: types::MQIA = types::MQIA(
        mqsys::MQIA_CHANNEL_AUTO_DEF,
    );
    pub const MQIA_CHANNEL_AUTO_DEF_EVENT: types::MQIA = types::MQIA(
        mqsys::MQIA_CHANNEL_AUTO_DEF_EVENT,
    );
    pub const MQIA_INDEX_TYPE: types::MQIA = types::MQIA(mqsys::MQIA_INDEX_TYPE);
    pub const MQIA_CLUSTER_WORKLOAD_LENGTH: types::MQIA = types::MQIA(
        mqsys::MQIA_CLUSTER_WORKLOAD_LENGTH,
    );
    pub const MQIA_CLUSTER_Q_TYPE: types::MQIA = types::MQIA(mqsys::MQIA_CLUSTER_Q_TYPE);
    pub const MQIA_ARCHIVE: types::MQIA = types::MQIA(mqsys::MQIA_ARCHIVE);
    pub const MQIA_DEF_BIND: types::MQIA = types::MQIA(mqsys::MQIA_DEF_BIND);
    pub const MQIA_PAGESET_ID: types::MQIA = types::MQIA(mqsys::MQIA_PAGESET_ID);
    pub const MQIA_QSG_DISP: types::MQIA = types::MQIA(mqsys::MQIA_QSG_DISP);
    pub const MQIA_INTRA_GROUP_QUEUING: types::MQIA = types::MQIA(
        mqsys::MQIA_INTRA_GROUP_QUEUING,
    );
    pub const MQIA_IGQ_PUT_AUTHORITY: types::MQIA = types::MQIA(
        mqsys::MQIA_IGQ_PUT_AUTHORITY,
    );
    pub const MQIA_AUTH_INFO_TYPE: types::MQIA = types::MQIA(mqsys::MQIA_AUTH_INFO_TYPE);
    pub const MQIA_MSG_MARK_BROWSE_INTERVAL: types::MQIA = types::MQIA(
        mqsys::MQIA_MSG_MARK_BROWSE_INTERVAL,
    );
    pub const MQIA_SSL_TASKS: types::MQIA = types::MQIA(mqsys::MQIA_SSL_TASKS);
    pub const MQIA_CF_LEVEL: types::MQIA = types::MQIA(mqsys::MQIA_CF_LEVEL);
    pub const MQIA_CF_RECOVER: types::MQIA = types::MQIA(mqsys::MQIA_CF_RECOVER);
    pub const MQIA_NAMELIST_TYPE: types::MQIA = types::MQIA(mqsys::MQIA_NAMELIST_TYPE);
    pub const MQIA_CHANNEL_EVENT: types::MQIA = types::MQIA(mqsys::MQIA_CHANNEL_EVENT);
    pub const MQIA_BRIDGE_EVENT: types::MQIA = types::MQIA(mqsys::MQIA_BRIDGE_EVENT);
    pub const MQIA_SSL_EVENT: types::MQIA = types::MQIA(mqsys::MQIA_SSL_EVENT);
    pub const MQIA_SSL_RESET_COUNT: types::MQIA = types::MQIA(
        mqsys::MQIA_SSL_RESET_COUNT,
    );
    pub const MQIA_SHARED_Q_Q_MGR_NAME: types::MQIA = types::MQIA(
        mqsys::MQIA_SHARED_Q_Q_MGR_NAME,
    );
    pub const MQIA_NPM_CLASS: types::MQIA = types::MQIA(mqsys::MQIA_NPM_CLASS);
    pub const MQIA_MAX_OPEN_Q: types::MQIA = types::MQIA(mqsys::MQIA_MAX_OPEN_Q);
    pub const MQIA_MONITOR_INTERVAL: types::MQIA = types::MQIA(
        mqsys::MQIA_MONITOR_INTERVAL,
    );
    pub const MQIA_Q_USERS: types::MQIA = types::MQIA(mqsys::MQIA_Q_USERS);
    pub const MQIA_MAX_GLOBAL_LOCKS: types::MQIA = types::MQIA(
        mqsys::MQIA_MAX_GLOBAL_LOCKS,
    );
    pub const MQIA_MAX_LOCAL_LOCKS: types::MQIA = types::MQIA(
        mqsys::MQIA_MAX_LOCAL_LOCKS,
    );
    pub const MQIA_LISTENER_PORT_NUMBER: types::MQIA = types::MQIA(
        mqsys::MQIA_LISTENER_PORT_NUMBER,
    );
    pub const MQIA_BATCH_INTERFACE_AUTO: types::MQIA = types::MQIA(
        mqsys::MQIA_BATCH_INTERFACE_AUTO,
    );
    pub const MQIA_CMD_SERVER_AUTO: types::MQIA = types::MQIA(
        mqsys::MQIA_CMD_SERVER_AUTO,
    );
    pub const MQIA_CMD_SERVER_CONVERT_MSG: types::MQIA = types::MQIA(
        mqsys::MQIA_CMD_SERVER_CONVERT_MSG,
    );
    pub const MQIA_CMD_SERVER_DLQ_MSG: types::MQIA = types::MQIA(
        mqsys::MQIA_CMD_SERVER_DLQ_MSG,
    );
    pub const MQIA_MAX_Q_TRIGGERS: types::MQIA = types::MQIA(mqsys::MQIA_MAX_Q_TRIGGERS);
    pub const MQIA_TRIGGER_RESTART: types::MQIA = types::MQIA(
        mqsys::MQIA_TRIGGER_RESTART,
    );
    pub const MQIA_SSL_FIPS_REQUIRED: types::MQIA = types::MQIA(
        mqsys::MQIA_SSL_FIPS_REQUIRED,
    );
    pub const MQIA_IP_ADDRESS_VERSION: types::MQIA = types::MQIA(
        mqsys::MQIA_IP_ADDRESS_VERSION,
    );
    pub const MQIA_LOGGER_EVENT: types::MQIA = types::MQIA(mqsys::MQIA_LOGGER_EVENT);
    pub const MQIA_CLWL_Q_RANK: types::MQIA = types::MQIA(mqsys::MQIA_CLWL_Q_RANK);
    pub const MQIA_CLWL_Q_PRIORITY: types::MQIA = types::MQIA(
        mqsys::MQIA_CLWL_Q_PRIORITY,
    );
    pub const MQIA_CLWL_MRU_CHANNELS: types::MQIA = types::MQIA(
        mqsys::MQIA_CLWL_MRU_CHANNELS,
    );
    pub const MQIA_CLWL_USEQ: types::MQIA = types::MQIA(mqsys::MQIA_CLWL_USEQ);
    pub const MQIA_COMMAND_EVENT: types::MQIA = types::MQIA(mqsys::MQIA_COMMAND_EVENT);
    pub const MQIA_ACTIVE_CHANNELS: types::MQIA = types::MQIA(
        mqsys::MQIA_ACTIVE_CHANNELS,
    );
    pub const MQIA_CHINIT_ADAPTERS: types::MQIA = types::MQIA(
        mqsys::MQIA_CHINIT_ADAPTERS,
    );
    pub const MQIA_ADOPTNEWMCA_CHECK: types::MQIA = types::MQIA(
        mqsys::MQIA_ADOPTNEWMCA_CHECK,
    );
    pub const MQIA_ADOPTNEWMCA_TYPE: types::MQIA = types::MQIA(
        mqsys::MQIA_ADOPTNEWMCA_TYPE,
    );
    pub const MQIA_ADOPTNEWMCA_INTERVAL: types::MQIA = types::MQIA(
        mqsys::MQIA_ADOPTNEWMCA_INTERVAL,
    );
    pub const MQIA_CHINIT_DISPATCHERS: types::MQIA = types::MQIA(
        mqsys::MQIA_CHINIT_DISPATCHERS,
    );
    pub const MQIA_DNS_WLM: types::MQIA = types::MQIA(mqsys::MQIA_DNS_WLM);
    pub const MQIA_LISTENER_TIMER: types::MQIA = types::MQIA(mqsys::MQIA_LISTENER_TIMER);
    pub const MQIA_LU62_CHANNELS: types::MQIA = types::MQIA(mqsys::MQIA_LU62_CHANNELS);
    pub const MQIA_MAX_CHANNELS: types::MQIA = types::MQIA(mqsys::MQIA_MAX_CHANNELS);
    pub const MQIA_OUTBOUND_PORT_MIN: types::MQIA = types::MQIA(
        mqsys::MQIA_OUTBOUND_PORT_MIN,
    );
    pub const MQIA_RECEIVE_TIMEOUT: types::MQIA = types::MQIA(
        mqsys::MQIA_RECEIVE_TIMEOUT,
    );
    pub const MQIA_RECEIVE_TIMEOUT_TYPE: types::MQIA = types::MQIA(
        mqsys::MQIA_RECEIVE_TIMEOUT_TYPE,
    );
    pub const MQIA_RECEIVE_TIMEOUT_MIN: types::MQIA = types::MQIA(
        mqsys::MQIA_RECEIVE_TIMEOUT_MIN,
    );
    pub const MQIA_TCP_CHANNELS: types::MQIA = types::MQIA(mqsys::MQIA_TCP_CHANNELS);
    pub const MQIA_TCP_KEEP_ALIVE: types::MQIA = types::MQIA(mqsys::MQIA_TCP_KEEP_ALIVE);
    pub const MQIA_TCP_STACK_TYPE: types::MQIA = types::MQIA(mqsys::MQIA_TCP_STACK_TYPE);
    pub const MQIA_CHINIT_TRACE_AUTO_START: types::MQIA = types::MQIA(
        mqsys::MQIA_CHINIT_TRACE_AUTO_START,
    );
    pub const MQIA_CHINIT_TRACE_TABLE_SIZE: types::MQIA = types::MQIA(
        mqsys::MQIA_CHINIT_TRACE_TABLE_SIZE,
    );
    pub const MQIA_CHINIT_CONTROL: types::MQIA = types::MQIA(mqsys::MQIA_CHINIT_CONTROL);
    pub const MQIA_CMD_SERVER_CONTROL: types::MQIA = types::MQIA(
        mqsys::MQIA_CMD_SERVER_CONTROL,
    );
    pub const MQIA_SERVICE_TYPE: types::MQIA = types::MQIA(mqsys::MQIA_SERVICE_TYPE);
    pub const MQIA_MONITORING_CHANNEL: types::MQIA = types::MQIA(
        mqsys::MQIA_MONITORING_CHANNEL,
    );
    pub const MQIA_MONITORING_Q: types::MQIA = types::MQIA(mqsys::MQIA_MONITORING_Q);
    pub const MQIA_MONITORING_AUTO_CLUSSDR: types::MQIA = types::MQIA(
        mqsys::MQIA_MONITORING_AUTO_CLUSSDR,
    );
    pub const MQIA_STATISTICS_MQI: types::MQIA = types::MQIA(mqsys::MQIA_STATISTICS_MQI);
    pub const MQIA_STATISTICS_Q: types::MQIA = types::MQIA(mqsys::MQIA_STATISTICS_Q);
    pub const MQIA_STATISTICS_CHANNEL: types::MQIA = types::MQIA(
        mqsys::MQIA_STATISTICS_CHANNEL,
    );
    pub const MQIA_STATISTICS_AUTO_CLUSSDR: types::MQIA = types::MQIA(
        mqsys::MQIA_STATISTICS_AUTO_CLUSSDR,
    );
    pub const MQIA_STATISTICS_INTERVAL: types::MQIA = types::MQIA(
        mqsys::MQIA_STATISTICS_INTERVAL,
    );
    pub const MQIA_ACCOUNTING_MQI: types::MQIA = types::MQIA(mqsys::MQIA_ACCOUNTING_MQI);
    pub const MQIA_ACCOUNTING_Q: types::MQIA = types::MQIA(mqsys::MQIA_ACCOUNTING_Q);
    pub const MQIA_ACCOUNTING_INTERVAL: types::MQIA = types::MQIA(
        mqsys::MQIA_ACCOUNTING_INTERVAL,
    );
    pub const MQIA_ACCOUNTING_CONN_OVERRIDE: types::MQIA = types::MQIA(
        mqsys::MQIA_ACCOUNTING_CONN_OVERRIDE,
    );
    pub const MQIA_TRACE_ROUTE_RECORDING: types::MQIA = types::MQIA(
        mqsys::MQIA_TRACE_ROUTE_RECORDING,
    );
    pub const MQIA_ACTIVITY_RECORDING: types::MQIA = types::MQIA(
        mqsys::MQIA_ACTIVITY_RECORDING,
    );
    pub const MQIA_SERVICE_CONTROL: types::MQIA = types::MQIA(
        mqsys::MQIA_SERVICE_CONTROL,
    );
    pub const MQIA_OUTBOUND_PORT_MAX: types::MQIA = types::MQIA(
        mqsys::MQIA_OUTBOUND_PORT_MAX,
    );
    pub const MQIA_SECURITY_CASE: types::MQIA = types::MQIA(mqsys::MQIA_SECURITY_CASE);
    pub const MQIA_QMOPT_CSMT_ON_ERROR: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_CSMT_ON_ERROR,
    );
    pub const MQIA_QMOPT_CONS_INFO_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_CONS_INFO_MSGS,
    );
    pub const MQIA_QMOPT_CONS_WARNING_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_CONS_WARNING_MSGS,
    );
    pub const MQIA_QMOPT_CONS_ERROR_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_CONS_ERROR_MSGS,
    );
    pub const MQIA_QMOPT_CONS_CRITICAL_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_CONS_CRITICAL_MSGS,
    );
    pub const MQIA_QMOPT_CONS_COMMS_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_CONS_COMMS_MSGS,
    );
    pub const MQIA_QMOPT_CONS_REORG_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_CONS_REORG_MSGS,
    );
    pub const MQIA_QMOPT_CONS_SYSTEM_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_CONS_SYSTEM_MSGS,
    );
    pub const MQIA_QMOPT_LOG_INFO_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_LOG_INFO_MSGS,
    );
    pub const MQIA_QMOPT_LOG_WARNING_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_LOG_WARNING_MSGS,
    );
    pub const MQIA_QMOPT_LOG_ERROR_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_LOG_ERROR_MSGS,
    );
    pub const MQIA_QMOPT_LOG_CRITICAL_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_LOG_CRITICAL_MSGS,
    );
    pub const MQIA_QMOPT_LOG_COMMS_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_LOG_COMMS_MSGS,
    );
    pub const MQIA_QMOPT_LOG_REORG_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_LOG_REORG_MSGS,
    );
    pub const MQIA_QMOPT_LOG_SYSTEM_MSGS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_LOG_SYSTEM_MSGS,
    );
    pub const MQIA_QMOPT_TRACE_MQI_CALLS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_TRACE_MQI_CALLS,
    );
    pub const MQIA_QMOPT_TRACE_COMMS: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_TRACE_COMMS,
    );
    pub const MQIA_QMOPT_TRACE_REORG: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_TRACE_REORG,
    );
    pub const MQIA_QMOPT_TRACE_CONVERSION: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_TRACE_CONVERSION,
    );
    pub const MQIA_QMOPT_TRACE_SYSTEM: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_TRACE_SYSTEM,
    );
    pub const MQIA_QMOPT_INTERNAL_DUMP: types::MQIA = types::MQIA(
        mqsys::MQIA_QMOPT_INTERNAL_DUMP,
    );
    pub const MQIA_MAX_RECOVERY_TASKS: types::MQIA = types::MQIA(
        mqsys::MQIA_MAX_RECOVERY_TASKS,
    );
    pub const MQIA_MAX_CLIENTS: types::MQIA = types::MQIA(mqsys::MQIA_MAX_CLIENTS);
    pub const MQIA_AUTO_REORGANIZATION: types::MQIA = types::MQIA(
        mqsys::MQIA_AUTO_REORGANIZATION,
    );
    pub const MQIA_AUTO_REORG_INTERVAL: types::MQIA = types::MQIA(
        mqsys::MQIA_AUTO_REORG_INTERVAL,
    );
    pub const MQIA_DURABLE_SUB: types::MQIA = types::MQIA(mqsys::MQIA_DURABLE_SUB);
    pub const MQIA_MULTICAST: types::MQIA = types::MQIA(mqsys::MQIA_MULTICAST);
    pub const MQIA_INHIBIT_PUB: types::MQIA = types::MQIA(mqsys::MQIA_INHIBIT_PUB);
    pub const MQIA_INHIBIT_SUB: types::MQIA = types::MQIA(mqsys::MQIA_INHIBIT_SUB);
    pub const MQIA_TREE_LIFE_TIME: types::MQIA = types::MQIA(mqsys::MQIA_TREE_LIFE_TIME);
    pub const MQIA_DEF_PUT_RESPONSE_TYPE: types::MQIA = types::MQIA(
        mqsys::MQIA_DEF_PUT_RESPONSE_TYPE,
    );
    pub const MQIA_TOPIC_DEF_PERSISTENCE: types::MQIA = types::MQIA(
        mqsys::MQIA_TOPIC_DEF_PERSISTENCE,
    );
    pub const MQIA_MASTER_ADMIN: types::MQIA = types::MQIA(mqsys::MQIA_MASTER_ADMIN);
    pub const MQIA_PUBSUB_MODE: types::MQIA = types::MQIA(mqsys::MQIA_PUBSUB_MODE);
    pub const MQIA_DEF_READ_AHEAD: types::MQIA = types::MQIA(mqsys::MQIA_DEF_READ_AHEAD);
    pub const MQIA_READ_AHEAD: types::MQIA = types::MQIA(mqsys::MQIA_READ_AHEAD);
    pub const MQIA_PROPERTY_CONTROL: types::MQIA = types::MQIA(
        mqsys::MQIA_PROPERTY_CONTROL,
    );
    pub const MQIA_MAX_PROPERTIES_LENGTH: types::MQIA = types::MQIA(
        mqsys::MQIA_MAX_PROPERTIES_LENGTH,
    );
    pub const MQIA_BASE_TYPE: types::MQIA = types::MQIA(mqsys::MQIA_BASE_TYPE);
    pub const MQIA_PM_DELIVERY: types::MQIA = types::MQIA(mqsys::MQIA_PM_DELIVERY);
    pub const MQIA_NPM_DELIVERY: types::MQIA = types::MQIA(mqsys::MQIA_NPM_DELIVERY);
    pub const MQIA_PROXY_SUB: types::MQIA = types::MQIA(mqsys::MQIA_PROXY_SUB);
    pub const MQIA_PUBSUB_NP_MSG: types::MQIA = types::MQIA(mqsys::MQIA_PUBSUB_NP_MSG);
    pub const MQIA_SUB_COUNT: types::MQIA = types::MQIA(mqsys::MQIA_SUB_COUNT);
    pub const MQIA_PUBSUB_NP_RESP: types::MQIA = types::MQIA(mqsys::MQIA_PUBSUB_NP_RESP);
    pub const MQIA_PUBSUB_MAXMSG_RETRY_COUNT: types::MQIA = types::MQIA(
        mqsys::MQIA_PUBSUB_MAXMSG_RETRY_COUNT,
    );
    pub const MQIA_PUBSUB_SYNC_PT: types::MQIA = types::MQIA(mqsys::MQIA_PUBSUB_SYNC_PT);
    pub const MQIA_TOPIC_TYPE: types::MQIA = types::MQIA(mqsys::MQIA_TOPIC_TYPE);
    pub const MQIA_PUB_COUNT: types::MQIA = types::MQIA(mqsys::MQIA_PUB_COUNT);
    pub const MQIA_WILDCARD_OPERATION: types::MQIA = types::MQIA(
        mqsys::MQIA_WILDCARD_OPERATION,
    );
    pub const MQIA_SUB_SCOPE: types::MQIA = types::MQIA(mqsys::MQIA_SUB_SCOPE);
    pub const MQIA_PUB_SCOPE: types::MQIA = types::MQIA(mqsys::MQIA_PUB_SCOPE);
    pub const MQIA_GROUP_UR: types::MQIA = types::MQIA(mqsys::MQIA_GROUP_UR);
    pub const MQIA_UR_DISP: types::MQIA = types::MQIA(mqsys::MQIA_UR_DISP);
    pub const MQIA_COMM_INFO_TYPE: types::MQIA = types::MQIA(mqsys::MQIA_COMM_INFO_TYPE);
    pub const MQIA_CF_OFFLOAD: types::MQIA = types::MQIA(mqsys::MQIA_CF_OFFLOAD);
    pub const MQIA_CF_OFFLOAD_THRESHOLD1: types::MQIA = types::MQIA(
        mqsys::MQIA_CF_OFFLOAD_THRESHOLD1,
    );
    pub const MQIA_CF_OFFLOAD_THRESHOLD2: types::MQIA = types::MQIA(
        mqsys::MQIA_CF_OFFLOAD_THRESHOLD2,
    );
    pub const MQIA_CF_OFFLOAD_THRESHOLD3: types::MQIA = types::MQIA(
        mqsys::MQIA_CF_OFFLOAD_THRESHOLD3,
    );
    pub const MQIA_CF_SMDS_BUFFERS: types::MQIA = types::MQIA(
        mqsys::MQIA_CF_SMDS_BUFFERS,
    );
    pub const MQIA_CF_OFFLDUSE: types::MQIA = types::MQIA(mqsys::MQIA_CF_OFFLDUSE);
    pub const MQIA_MAX_RESPONSES: types::MQIA = types::MQIA(mqsys::MQIA_MAX_RESPONSES);
    pub const MQIA_RESPONSE_RESTART_POINT: types::MQIA = types::MQIA(
        mqsys::MQIA_RESPONSE_RESTART_POINT,
    );
    pub const MQIA_COMM_EVENT: types::MQIA = types::MQIA(mqsys::MQIA_COMM_EVENT);
    pub const MQIA_MCAST_BRIDGE: types::MQIA = types::MQIA(mqsys::MQIA_MCAST_BRIDGE);
    pub const MQIA_USE_DEAD_LETTER_Q: types::MQIA = types::MQIA(
        mqsys::MQIA_USE_DEAD_LETTER_Q,
    );
    pub const MQIA_TOLERATE_UNPROTECTED: types::MQIA = types::MQIA(
        mqsys::MQIA_TOLERATE_UNPROTECTED,
    );
    pub const MQIA_SIGNATURE_ALGORITHM: types::MQIA = types::MQIA(
        mqsys::MQIA_SIGNATURE_ALGORITHM,
    );
    pub const MQIA_ENCRYPTION_ALGORITHM: types::MQIA = types::MQIA(
        mqsys::MQIA_ENCRYPTION_ALGORITHM,
    );
    pub const MQIA_POLICY_VERSION: types::MQIA = types::MQIA(mqsys::MQIA_POLICY_VERSION);
    pub const MQIA_ACTIVITY_CONN_OVERRIDE: types::MQIA = types::MQIA(
        mqsys::MQIA_ACTIVITY_CONN_OVERRIDE,
    );
    pub const MQIA_ACTIVITY_TRACE: types::MQIA = types::MQIA(mqsys::MQIA_ACTIVITY_TRACE);
    pub const MQIA_SUB_CONFIGURATION_EVENT: types::MQIA = types::MQIA(
        mqsys::MQIA_SUB_CONFIGURATION_EVENT,
    );
    pub const MQIA_XR_CAPABILITY: types::MQIA = types::MQIA(mqsys::MQIA_XR_CAPABILITY);
    pub const MQIA_CF_RECAUTO: types::MQIA = types::MQIA(mqsys::MQIA_CF_RECAUTO);
    pub const MQIA_QMGR_CFCONLOS: types::MQIA = types::MQIA(mqsys::MQIA_QMGR_CFCONLOS);
    pub const MQIA_CF_CFCONLOS: types::MQIA = types::MQIA(mqsys::MQIA_CF_CFCONLOS);
    pub const MQIA_SUITE_B_STRENGTH: types::MQIA = types::MQIA(
        mqsys::MQIA_SUITE_B_STRENGTH,
    );
    pub const MQIA_CHLAUTH_RECORDS: types::MQIA = types::MQIA(
        mqsys::MQIA_CHLAUTH_RECORDS,
    );
    pub const MQIA_PUBSUB_CLUSTER: types::MQIA = types::MQIA(mqsys::MQIA_PUBSUB_CLUSTER);
    pub const MQIA_DEF_CLUSTER_XMIT_Q_TYPE: types::MQIA = types::MQIA(
        mqsys::MQIA_DEF_CLUSTER_XMIT_Q_TYPE,
    );
    pub const MQIA_PROT_POLICY_CAPABILITY: types::MQIA = types::MQIA(
        mqsys::MQIA_PROT_POLICY_CAPABILITY,
    );
    pub const MQIA_CERT_VAL_POLICY: types::MQIA = types::MQIA(
        mqsys::MQIA_CERT_VAL_POLICY,
    );
    pub const MQIA_TOPIC_NODE_COUNT: types::MQIA = types::MQIA(
        mqsys::MQIA_TOPIC_NODE_COUNT,
    );
    pub const MQIA_REVERSE_DNS_LOOKUP: types::MQIA = types::MQIA(
        mqsys::MQIA_REVERSE_DNS_LOOKUP,
    );
    pub const MQIA_CLUSTER_PUB_ROUTE: types::MQIA = types::MQIA(
        mqsys::MQIA_CLUSTER_PUB_ROUTE,
    );
    pub const MQIA_CLUSTER_OBJECT_STATE: types::MQIA = types::MQIA(
        mqsys::MQIA_CLUSTER_OBJECT_STATE,
    );
    pub const MQIA_CHECK_LOCAL_BINDING: types::MQIA = types::MQIA(
        mqsys::MQIA_CHECK_LOCAL_BINDING,
    );
    pub const MQIA_CHECK_CLIENT_BINDING: types::MQIA = types::MQIA(
        mqsys::MQIA_CHECK_CLIENT_BINDING,
    );
    pub const MQIA_AUTHENTICATION_FAIL_DELAY: types::MQIA = types::MQIA(
        mqsys::MQIA_AUTHENTICATION_FAIL_DELAY,
    );
    pub const MQIA_ADOPT_CONTEXT: types::MQIA = types::MQIA(mqsys::MQIA_ADOPT_CONTEXT);
    pub const MQIA_LDAP_SECURE_COMM: types::MQIA = types::MQIA(
        mqsys::MQIA_LDAP_SECURE_COMM,
    );
    pub const MQIA_DISPLAY_TYPE: types::MQIA = types::MQIA(mqsys::MQIA_DISPLAY_TYPE);
    pub const MQIA_LDAP_AUTHORMD: types::MQIA = types::MQIA(mqsys::MQIA_LDAP_AUTHORMD);
    pub const MQIA_LDAP_NESTGRP: types::MQIA = types::MQIA(mqsys::MQIA_LDAP_NESTGRP);
    pub const MQIA_AMQP_CAPABILITY: types::MQIA = types::MQIA(
        mqsys::MQIA_AMQP_CAPABILITY,
    );
    pub const MQIA_AUTHENTICATION_METHOD: types::MQIA = types::MQIA(
        mqsys::MQIA_AUTHENTICATION_METHOD,
    );
    pub const MQIA_KEY_REUSE_COUNT: types::MQIA = types::MQIA(
        mqsys::MQIA_KEY_REUSE_COUNT,
    );
    pub const MQIA_MEDIA_IMAGE_SCHEDULING: types::MQIA = types::MQIA(
        mqsys::MQIA_MEDIA_IMAGE_SCHEDULING,
    );
    pub const MQIA_MEDIA_IMAGE_INTERVAL: types::MQIA = types::MQIA(
        mqsys::MQIA_MEDIA_IMAGE_INTERVAL,
    );
    pub const MQIA_MEDIA_IMAGE_LOG_LENGTH: types::MQIA = types::MQIA(
        mqsys::MQIA_MEDIA_IMAGE_LOG_LENGTH,
    );
    pub const MQIA_MEDIA_IMAGE_RECOVER_OBJ: types::MQIA = types::MQIA(
        mqsys::MQIA_MEDIA_IMAGE_RECOVER_OBJ,
    );
    pub const MQIA_MEDIA_IMAGE_RECOVER_Q: types::MQIA = types::MQIA(
        mqsys::MQIA_MEDIA_IMAGE_RECOVER_Q,
    );
    pub const MQIA_ADVANCED_CAPABILITY: types::MQIA = types::MQIA(
        mqsys::MQIA_ADVANCED_CAPABILITY,
    );
    pub const MQIA_MAX_Q_FILE_SIZE: types::MQIA = types::MQIA(
        mqsys::MQIA_MAX_Q_FILE_SIZE,
    );
    pub const MQIA_STREAM_QUEUE_QOS: types::MQIA = types::MQIA(
        mqsys::MQIA_STREAM_QUEUE_QOS,
    );
    pub const MQIA_CAP_EXPIRY: types::MQIA = types::MQIA(mqsys::MQIA_CAP_EXPIRY);
    pub const MQIA_USER_LIST: types::MQIA = types::MQIA(mqsys::MQIA_USER_LIST);
    pub const MQIDO_COMMIT: types::MQIDO = types::MQIDO(mqsys::MQIDO_COMMIT);
    pub const MQIDO_BACKOUT: types::MQIDO = types::MQIDO(mqsys::MQIDO_BACKOUT);
    pub const MQIEPF_NONE: types::MQIEPF = types::MQIEPF(mqsys::MQIEPF_NONE);
    pub const MQIEPF_THREADED_LIBRARY: types::MQIEPF = types::MQIEPF(
        mqsys::MQIEPF_THREADED_LIBRARY,
    );
    pub const MQIEPF_LOCAL_LIBRARY: types::MQIEPF = types::MQIEPF(
        mqsys::MQIEPF_LOCAL_LIBRARY,
    );
    pub const MQIEPF_CLIENT_LIBRARY: types::MQIEPF = types::MQIEPF(
        mqsys::MQIEPF_CLIENT_LIBRARY,
    );
    pub const MQIEPF_NON_THREADED_LIBRARY: types::MQIEPF = types::MQIEPF(
        mqsys::MQIEPF_NON_THREADED_LIBRARY,
    );
    pub const MQIGQPA_DEFAULT: types::MQIGQPA = types::MQIGQPA(mqsys::MQIGQPA_DEFAULT);
    pub const MQIGQPA_CONTEXT: types::MQIGQPA = types::MQIGQPA(mqsys::MQIGQPA_CONTEXT);
    pub const MQIGQPA_ONLY_IGQ: types::MQIGQPA = types::MQIGQPA(mqsys::MQIGQPA_ONLY_IGQ);
    pub const MQIGQPA_ALTERNATE_OR_IGQ: types::MQIGQPA = types::MQIGQPA(
        mqsys::MQIGQPA_ALTERNATE_OR_IGQ,
    );
    pub const MQIGQ_DISABLED: types::MQIGQ = types::MQIGQ(mqsys::MQIGQ_DISABLED);
    pub const MQIGQ_ENABLED: types::MQIGQ = types::MQIGQ(mqsys::MQIGQ_ENABLED);
    pub const MQIIH_NONE: types::MQIIH = types::MQIIH(mqsys::MQIIH_NONE);
    pub const MQIIH_PASS_EXPIRATION: types::MQIIH = types::MQIIH(
        mqsys::MQIIH_PASS_EXPIRATION,
    );
    pub const MQIIH_REPLY_FORMAT_NONE: types::MQIIH = types::MQIIH(
        mqsys::MQIIH_REPLY_FORMAT_NONE,
    );
    pub const MQIIH_IGNORE_PURG: types::MQIIH = types::MQIIH(mqsys::MQIIH_IGNORE_PURG);
    pub const MQIIH_CM0_REQUEST_RESPONSE: types::MQIIH = types::MQIIH(
        mqsys::MQIIH_CM0_REQUEST_RESPONSE,
    );
    pub const MQIIH_UNLIMITED_EXPIRATION: types::MQIIH = types::MQIIH(
        mqsys::MQIIH_UNLIMITED_EXPIRATION,
    );
    pub const MQIMGRCOV_NO: types::MQIMGRCOV = types::MQIMGRCOV(mqsys::MQIMGRCOV_NO);
    pub const MQIMGRCOV_YES: types::MQIMGRCOV = types::MQIMGRCOV(mqsys::MQIMGRCOV_YES);
    pub const MQIMGRCOV_AS_Q_MGR: types::MQIMGRCOV = types::MQIMGRCOV(
        mqsys::MQIMGRCOV_AS_Q_MGR,
    );
    pub const MQIMMREASON_NONE: types::MQIMMREASON = types::MQIMMREASON(
        mqsys::MQIMMREASON_NONE,
    );
    pub const MQIMMREASON_NOT_CLIENT: types::MQIMMREASON = types::MQIMMREASON(
        mqsys::MQIMMREASON_NOT_CLIENT,
    );
    pub const MQIMMREASON_NOT_RECONNECTABLE: types::MQIMMREASON = types::MQIMMREASON(
        mqsys::MQIMMREASON_NOT_RECONNECTABLE,
    );
    pub const MQIMMREASON_MOVING: types::MQIMMREASON = types::MQIMMREASON(
        mqsys::MQIMMREASON_MOVING,
    );
    pub const MQIMMREASON_APPLNAME_CHANGED: types::MQIMMREASON = types::MQIMMREASON(
        mqsys::MQIMMREASON_APPLNAME_CHANGED,
    );
    pub const MQIMMREASON_IN_TRANSACTION: types::MQIMMREASON = types::MQIMMREASON(
        mqsys::MQIMMREASON_IN_TRANSACTION,
    );
    pub const MQIMMREASON_AWAITS_REPLY: types::MQIMMREASON = types::MQIMMREASON(
        mqsys::MQIMMREASON_AWAITS_REPLY,
    );
    pub const MQIMMREASON_NO_REDIRECT: types::MQIMMREASON = types::MQIMMREASON(
        mqsys::MQIMMREASON_NO_REDIRECT,
    );
    pub const MQIMPO_NONE: types::MQIMPO = types::MQIMPO(mqsys::MQIMPO_NONE);
    pub const MQIMPO_CONVERT_TYPE: types::MQIMPO = types::MQIMPO(
        mqsys::MQIMPO_CONVERT_TYPE,
    );
    pub const MQIMPO_QUERY_LENGTH: types::MQIMPO = types::MQIMPO(
        mqsys::MQIMPO_QUERY_LENGTH,
    );
    pub const MQIMPO_INQ_NEXT: types::MQIMPO = types::MQIMPO(mqsys::MQIMPO_INQ_NEXT);
    pub const MQIMPO_INQ_PROP_UNDER_CURSOR: types::MQIMPO = types::MQIMPO(
        mqsys::MQIMPO_INQ_PROP_UNDER_CURSOR,
    );
    pub const MQIMPO_CONVERT_VALUE: types::MQIMPO = types::MQIMPO(
        mqsys::MQIMPO_CONVERT_VALUE,
    );
    pub const MQINBD_Q_MGR: types::MQINBD = types::MQINBD(mqsys::MQINBD_Q_MGR);
    pub const MQINBD_GROUP: types::MQINBD = types::MQINBD(mqsys::MQINBD_GROUP);
    pub const MQIND_ALL: types::MQIND = types::MQIND(mqsys::MQIND_ALL);
    pub const MQIND_NONE: types::MQIND = types::MQIND(mqsys::MQIND_NONE);
    pub const MQIPADDR_IPV4: types::MQIPADDR = types::MQIPADDR(mqsys::MQIPADDR_IPV4);
    pub const MQIPADDR_IPV6: types::MQIPADDR = types::MQIPADDR(mqsys::MQIPADDR_IPV6);
    pub const MQIS_NO: types::MQIS = types::MQIS(mqsys::MQIS_NO);
    pub const MQIS_YES: types::MQIS = types::MQIS(mqsys::MQIS_YES);
    pub const MQITEM_INTEGER: types::MQITEM = types::MQITEM(mqsys::MQITEM_INTEGER);
    pub const MQITEM_STRING: types::MQITEM = types::MQITEM(mqsys::MQITEM_STRING);
    pub const MQITEM_BAG: types::MQITEM = types::MQITEM(mqsys::MQITEM_BAG);
    pub const MQITEM_BYTE_STRING: types::MQITEM = types::MQITEM(
        mqsys::MQITEM_BYTE_STRING,
    );
    pub const MQITEM_INTEGER_FILTER: types::MQITEM = types::MQITEM(
        mqsys::MQITEM_INTEGER_FILTER,
    );
    pub const MQITEM_STRING_FILTER: types::MQITEM = types::MQITEM(
        mqsys::MQITEM_STRING_FILTER,
    );
    pub const MQITEM_INTEGER64: types::MQITEM = types::MQITEM(mqsys::MQITEM_INTEGER64);
    pub const MQITEM_BYTE_STRING_FILTER: types::MQITEM = types::MQITEM(
        mqsys::MQITEM_BYTE_STRING_FILTER,
    );
    pub const MQIT_NONE: types::MQIT = types::MQIT(mqsys::MQIT_NONE);
    pub const MQIT_MSG_ID: types::MQIT = types::MQIT(mqsys::MQIT_MSG_ID);
    pub const MQIT_CORREL_ID: types::MQIT = types::MQIT(mqsys::MQIT_CORREL_ID);
    pub const MQIT_MSG_TOKEN: types::MQIT = types::MQIT(mqsys::MQIT_MSG_TOKEN);
    pub const MQIT_GROUP_ID: types::MQIT = types::MQIT(mqsys::MQIT_GROUP_ID);
    pub const MQIT_INTEGER: types::MQIT = types::MQIT(mqsys::MQIT_INTEGER);
    pub const MQIT_STRING: types::MQIT = types::MQIT(mqsys::MQIT_STRING);
    pub const MQIT_BAG: types::MQIT = types::MQIT(mqsys::MQIT_BAG);
    pub const MQKAI_AUTO: types::MQKAI = types::MQKAI(mqsys::MQKAI_AUTO);
    pub const MQKEY_REUSE_UNLIMITED: types::MQKEY = types::MQKEY(
        mqsys::MQKEY_REUSE_UNLIMITED,
    );
    pub const MQKEY_REUSE_DISABLED: types::MQKEY = types::MQKEY(
        mqsys::MQKEY_REUSE_DISABLED,
    );
    pub const MQLDAPC_INACTIVE: types::MQLDAPC = types::MQLDAPC(mqsys::MQLDAPC_INACTIVE);
    pub const MQLDAPC_CONNECTED: types::MQLDAPC = types::MQLDAPC(
        mqsys::MQLDAPC_CONNECTED,
    );
    pub const MQLDAPC_ERROR: types::MQLDAPC = types::MQLDAPC(mqsys::MQLDAPC_ERROR);
    pub const MQLDAP_AUTHORMD_OS: types::MQLDAP_AUTHORMD = types::MQLDAP_AUTHORMD(
        mqsys::MQLDAP_AUTHORMD_OS,
    );
    pub const MQLDAP_AUTHORMD_SEARCHGRP: types::MQLDAP_AUTHORMD = types::MQLDAP_AUTHORMD(
        mqsys::MQLDAP_AUTHORMD_SEARCHGRP,
    );
    pub const MQLDAP_AUTHORMD_SEARCHUSR: types::MQLDAP_AUTHORMD = types::MQLDAP_AUTHORMD(
        mqsys::MQLDAP_AUTHORMD_SEARCHUSR,
    );
    pub const MQLDAP_AUTHORMD_SRCHGRPSN: types::MQLDAP_AUTHORMD = types::MQLDAP_AUTHORMD(
        mqsys::MQLDAP_AUTHORMD_SRCHGRPSN,
    );
    pub const MQLDAP_NESTGRP_NO: types::MQLDAP_NESTGRP = types::MQLDAP_NESTGRP(
        mqsys::MQLDAP_NESTGRP_NO,
    );
    pub const MQLDAP_NESTGRP_YES: types::MQLDAP_NESTGRP = types::MQLDAP_NESTGRP(
        mqsys::MQLDAP_NESTGRP_YES,
    );
    pub const MQLOGTYPE_CIRCULAR: types::MQLOGTYPE = types::MQLOGTYPE(
        mqsys::MQLOGTYPE_CIRCULAR,
    );
    pub const MQLOGTYPE_LINEAR: types::MQLOGTYPE = types::MQLOGTYPE(
        mqsys::MQLOGTYPE_LINEAR,
    );
    pub const MQLOGTYPE_REPLICATED: types::MQLOGTYPE = types::MQLOGTYPE(
        mqsys::MQLOGTYPE_REPLICATED,
    );
    pub const MQLR_MAX: types::MQLR = types::MQLR(mqsys::MQLR_MAX);
    pub const MQLR_AUTO: types::MQLR = types::MQLR(mqsys::MQLR_AUTO);
    pub const MQLR_ONE: types::MQLR = types::MQLR(mqsys::MQLR_ONE);
    pub const MQMASTER_NO: types::MQMASTER = types::MQMASTER(mqsys::MQMASTER_NO);
    pub const MQMASTER_YES: types::MQMASTER = types::MQMASTER(mqsys::MQMASTER_YES);
    pub const MQMATCH_GENERIC: types::MQMATCH = types::MQMATCH(mqsys::MQMATCH_GENERIC);
    pub const MQMATCH_RUNCHECK: types::MQMATCH = types::MQMATCH(mqsys::MQMATCH_RUNCHECK);
    pub const MQMATCH_EXACT: types::MQMATCH = types::MQMATCH(mqsys::MQMATCH_EXACT);
    pub const MQMATCH_ALL: types::MQMATCH = types::MQMATCH(mqsys::MQMATCH_ALL);
    pub const MQMCAS_STOPPED: types::MQMCAS = types::MQMCAS(mqsys::MQMCAS_STOPPED);
    pub const MQMCAS_RUNNING: types::MQMCAS = types::MQMCAS(mqsys::MQMCAS_RUNNING);
    pub const MQMCAT_PROCESS: types::MQMCAT = types::MQMCAT(mqsys::MQMCAT_PROCESS);
    pub const MQMCAT_THREAD: types::MQMCAT = types::MQMCAT(mqsys::MQMCAT_THREAD);
    pub const MQMCB_DISABLED: types::MQMCB = types::MQMCB(mqsys::MQMCB_DISABLED);
    pub const MQMCB_ENABLED: types::MQMCB = types::MQMCB(mqsys::MQMCB_ENABLED);
    pub const MQMCEV_PACKET_LOSS: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_PACKET_LOSS,
    );
    pub const MQMCEV_HEARTBEAT_TIMEOUT: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_HEARTBEAT_TIMEOUT,
    );
    pub const MQMCEV_VERSION_CONFLICT: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_VERSION_CONFLICT,
    );
    pub const MQMCEV_RELIABILITY: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_RELIABILITY,
    );
    pub const MQMCEV_CLOSED_TRANS: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_CLOSED_TRANS,
    );
    pub const MQMCEV_STREAM_ERROR: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_STREAM_ERROR,
    );
    pub const MQMCEV_NEW_SOURCE: types::MQMCEV = types::MQMCEV(mqsys::MQMCEV_NEW_SOURCE);
    pub const MQMCEV_RECEIVE_QUEUE_TRIMMED: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_RECEIVE_QUEUE_TRIMMED,
    );
    pub const MQMCEV_PACKET_LOSS_NACK_EXPIRE: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_PACKET_LOSS_NACK_EXPIRE,
    );
    pub const MQMCEV_ACK_RETRIES_EXCEEDED: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_ACK_RETRIES_EXCEEDED,
    );
    pub const MQMCEV_STREAM_SUSPEND_NACK: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_STREAM_SUSPEND_NACK,
    );
    pub const MQMCEV_STREAM_RESUME_NACK: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_STREAM_RESUME_NACK,
    );
    pub const MQMCEV_STREAM_EXPELLED: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_STREAM_EXPELLED,
    );
    pub const MQMCEV_FIRST_MESSAGE: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_FIRST_MESSAGE,
    );
    pub const MQMCEV_LATE_JOIN_FAILURE: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_LATE_JOIN_FAILURE,
    );
    pub const MQMCEV_MESSAGE_LOSS: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_MESSAGE_LOSS,
    );
    pub const MQMCEV_SEND_PACKET_FAILURE: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_SEND_PACKET_FAILURE,
    );
    pub const MQMCEV_REPAIR_DELAY: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_REPAIR_DELAY,
    );
    pub const MQMCEV_MEMORY_ALERT_ON: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_MEMORY_ALERT_ON,
    );
    pub const MQMCEV_MEMORY_ALERT_OFF: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_MEMORY_ALERT_OFF,
    );
    pub const MQMCEV_NACK_ALERT_ON: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_NACK_ALERT_ON,
    );
    pub const MQMCEV_NACK_ALERT_OFF: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_NACK_ALERT_OFF,
    );
    pub const MQMCEV_REPAIR_ALERT_ON: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_REPAIR_ALERT_ON,
    );
    pub const MQMCEV_REPAIR_ALERT_OFF: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_REPAIR_ALERT_OFF,
    );
    pub const MQMCEV_RELIABILITY_CHANGED: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_RELIABILITY_CHANGED,
    );
    pub const MQMCEV_SHM_DEST_UNUSABLE: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_SHM_DEST_UNUSABLE,
    );
    pub const MQMCEV_SHM_PORT_UNUSABLE: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_SHM_PORT_UNUSABLE,
    );
    pub const MQMCEV_CCT_GETTIME_FAILED: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_CCT_GETTIME_FAILED,
    );
    pub const MQMCEV_DEST_INTERFACE_FAILURE: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_DEST_INTERFACE_FAILURE,
    );
    pub const MQMCEV_DEST_INTERFACE_FAILOVER: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_DEST_INTERFACE_FAILOVER,
    );
    pub const MQMCEV_PORT_INTERFACE_FAILURE: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_PORT_INTERFACE_FAILURE,
    );
    pub const MQMCEV_PORT_INTERFACE_FAILOVER: types::MQMCEV = types::MQMCEV(
        mqsys::MQMCEV_PORT_INTERFACE_FAILOVER,
    );
    pub const MQMCP_COMPAT: types::MQMCP = types::MQMCP(mqsys::MQMCP_COMPAT);
    pub const MQMCP_ALL: types::MQMCP = types::MQMCP(mqsys::MQMCP_ALL);
    pub const MQMCP_NONE: types::MQMCP = types::MQMCP(mqsys::MQMCP_NONE);
    pub const MQMCP_USER: types::MQMCP = types::MQMCP(mqsys::MQMCP_USER);
    pub const MQMCP_REPLY: types::MQMCP = types::MQMCP(mqsys::MQMCP_REPLY);
    pub const MQMC_AS_PARENT: types::MQMC = types::MQMC(mqsys::MQMC_AS_PARENT);
    pub const MQMC_ENABLED: types::MQMC = types::MQMC(mqsys::MQMC_ENABLED);
    pub const MQMC_DISABLED: types::MQMC = types::MQMC(mqsys::MQMC_DISABLED);
    pub const MQMC_ONLY: types::MQMC = types::MQMC(mqsys::MQMC_ONLY);
    pub const MQMDEF_NONE: types::MQMDEF = types::MQMDEF(mqsys::MQMDEF_NONE);
    pub const MQMDS_PRIORITY: types::MQMDS = types::MQMDS(mqsys::MQMDS_PRIORITY);
    pub const MQMDS_FIFO: types::MQMDS = types::MQMDS(mqsys::MQMDS_FIFO);
    pub const MQMEDIMGINTVL_OFF: types::MQMEDIMGINTVL = types::MQMEDIMGINTVL(
        mqsys::MQMEDIMGINTVL_OFF,
    );
    pub const MQMEDIMGLOGLN_OFF: types::MQMEDIMGLOGLN = types::MQMEDIMGLOGLN(
        mqsys::MQMEDIMGLOGLN_OFF,
    );
    pub const MQMEDIMGSCHED_MANUAL: types::MQMEDIMGSCHED = types::MQMEDIMGSCHED(
        mqsys::MQMEDIMGSCHED_MANUAL,
    );
    pub const MQMEDIMGSCHED_AUTO: types::MQMEDIMGSCHED = types::MQMEDIMGSCHED(
        mqsys::MQMEDIMGSCHED_AUTO,
    );
    pub const MQMF_ACCEPT_UNSUP_MASK: types::MQMF = types::MQMF(
        mqsys::MQMF_ACCEPT_UNSUP_MASK,
    );
    pub const MQMF_NONE: types::MQMF = types::MQMF(mqsys::MQMF_NONE);
    pub const MQMF_SEGMENTATION_ALLOWED: types::MQMF = types::MQMF(
        mqsys::MQMF_SEGMENTATION_ALLOWED,
    );
    pub const MQMF_SEGMENT: types::MQMF = types::MQMF(mqsys::MQMF_SEGMENT);
    pub const MQMF_LAST_SEGMENT: types::MQMF = types::MQMF(mqsys::MQMF_LAST_SEGMENT);
    pub const MQMF_MSG_IN_GROUP: types::MQMF = types::MQMF(mqsys::MQMF_MSG_IN_GROUP);
    pub const MQMF_LAST_MSG_IN_GROUP: types::MQMF = types::MQMF(
        mqsys::MQMF_LAST_MSG_IN_GROUP,
    );
    pub const MQMF_REJECT_UNSUP_MASK: types::MQMF = types::MQMF(
        mqsys::MQMF_REJECT_UNSUP_MASK,
    );
    pub const MQMF_ACCEPT_UNSUP_IF_XMIT_MASK: types::MQMF = types::MQMF(
        mqsys::MQMF_ACCEPT_UNSUP_IF_XMIT_MASK,
    );
    pub const MQMF_SEGMENTATION_INHIBITED: types::MQMF = types::MQMF(
        mqsys::MQMF_SEGMENTATION_INHIBITED,
    );
    pub const MQMHBO_NONE: types::MQMHBO = types::MQMHBO(mqsys::MQMHBO_NONE);
    pub const MQMHBO_PROPERTIES_IN_MQRFH2: types::MQMHBO = types::MQMHBO(
        mqsys::MQMHBO_PROPERTIES_IN_MQRFH2,
    );
    pub const MQMHBO_DELETE_PROPERTIES: types::MQMHBO = types::MQMHBO(
        mqsys::MQMHBO_DELETE_PROPERTIES,
    );
    pub const MQMLP_ENCRYPTION_ALG_NONE: types::MQMLP_ENCRYPTION = types::MQMLP_ENCRYPTION(
        mqsys::MQMLP_ENCRYPTION_ALG_NONE,
    );
    pub const MQMLP_ENCRYPTION_ALG_RC2: types::MQMLP_ENCRYPTION = types::MQMLP_ENCRYPTION(
        mqsys::MQMLP_ENCRYPTION_ALG_RC2,
    );
    pub const MQMLP_ENCRYPTION_ALG_DES: types::MQMLP_ENCRYPTION = types::MQMLP_ENCRYPTION(
        mqsys::MQMLP_ENCRYPTION_ALG_DES,
    );
    pub const MQMLP_ENCRYPTION_ALG_3DES: types::MQMLP_ENCRYPTION = types::MQMLP_ENCRYPTION(
        mqsys::MQMLP_ENCRYPTION_ALG_3DES,
    );
    pub const MQMLP_ENCRYPTION_ALG_AES128: types::MQMLP_ENCRYPTION = types::MQMLP_ENCRYPTION(
        mqsys::MQMLP_ENCRYPTION_ALG_AES128,
    );
    pub const MQMLP_ENCRYPTION_ALG_AES256: types::MQMLP_ENCRYPTION = types::MQMLP_ENCRYPTION(
        mqsys::MQMLP_ENCRYPTION_ALG_AES256,
    );
    pub const MQMLP_SIGN_ALG_NONE: types::MQMLP_SIGN = types::MQMLP_SIGN(
        mqsys::MQMLP_SIGN_ALG_NONE,
    );
    pub const MQMLP_SIGN_ALG_MD5: types::MQMLP_SIGN = types::MQMLP_SIGN(
        mqsys::MQMLP_SIGN_ALG_MD5,
    );
    pub const MQMLP_SIGN_ALG_SHA1: types::MQMLP_SIGN = types::MQMLP_SIGN(
        mqsys::MQMLP_SIGN_ALG_SHA1,
    );
    pub const MQMLP_SIGN_ALG_SHA224: types::MQMLP_SIGN = types::MQMLP_SIGN(
        mqsys::MQMLP_SIGN_ALG_SHA224,
    );
    pub const MQMLP_SIGN_ALG_SHA256: types::MQMLP_SIGN = types::MQMLP_SIGN(
        mqsys::MQMLP_SIGN_ALG_SHA256,
    );
    pub const MQMLP_SIGN_ALG_SHA384: types::MQMLP_SIGN = types::MQMLP_SIGN(
        mqsys::MQMLP_SIGN_ALG_SHA384,
    );
    pub const MQMLP_SIGN_ALG_SHA512: types::MQMLP_SIGN = types::MQMLP_SIGN(
        mqsys::MQMLP_SIGN_ALG_SHA512,
    );
    pub const MQMLP_TOLERATE_UNPROTECTED_NO: types::MQMLP_TOLERATE = types::MQMLP_TOLERATE(
        mqsys::MQMLP_TOLERATE_UNPROTECTED_NO,
    );
    pub const MQMLP_TOLERATE_UNPROTECTED_YES: types::MQMLP_TOLERATE = types::MQMLP_TOLERATE(
        mqsys::MQMLP_TOLERATE_UNPROTECTED_YES,
    );
    pub const MQMMBI_UNLIMITED: types::MQMMBI = types::MQMMBI(mqsys::MQMMBI_UNLIMITED);
    pub const MQMODE_FORCE: types::MQMODE = types::MQMODE(mqsys::MQMODE_FORCE);
    pub const MQMODE_QUIESCE: types::MQMODE = types::MQMODE(mqsys::MQMODE_QUIESCE);
    pub const MQMODE_TERMINATE: types::MQMODE = types::MQMODE(mqsys::MQMODE_TERMINATE);
    pub const MQMON_Q_MGR: types::MQMON = types::MQMON(mqsys::MQMON_Q_MGR);
    pub const MQMON_NONE: types::MQMON = types::MQMON(mqsys::MQMON_NONE);
    pub const MQMON_OFF: types::MQMON = types::MQMON(mqsys::MQMON_OFF);
    pub const MQMON_ON: types::MQMON = types::MQMON(mqsys::MQMON_ON);
    pub const MQMON_LOW: types::MQMON = types::MQMON(mqsys::MQMON_LOW);
    pub const MQMON_MEDIUM: types::MQMON = types::MQMON(mqsys::MQMON_MEDIUM);
    pub const MQMON_HIGH: types::MQMON = types::MQMON(mqsys::MQMON_HIGH);
    pub const MQMON_NOT_AVAILABLE: types::MQMON_AVAILABILITY = types::MQMON_AVAILABILITY(
        mqsys::MQMON_NOT_AVAILABLE,
    );
    pub const MQMON_DISABLED: types::MQMON_OVERRIDE = types::MQMON_OVERRIDE(
        mqsys::MQMON_DISABLED,
    );
    pub const MQMON_ENABLED: types::MQMON_OVERRIDE = types::MQMON_OVERRIDE(
        mqsys::MQMON_ENABLED,
    );
    pub const MQMO_NONE: types::MQMO = types::MQMO(mqsys::MQMO_NONE);
    pub const MQMO_MATCH_MSG_ID: types::MQMO = types::MQMO(mqsys::MQMO_MATCH_MSG_ID);
    pub const MQMO_MATCH_CORREL_ID: types::MQMO = types::MQMO(
        mqsys::MQMO_MATCH_CORREL_ID,
    );
    pub const MQMO_MATCH_GROUP_ID: types::MQMO = types::MQMO(mqsys::MQMO_MATCH_GROUP_ID);
    pub const MQMO_MATCH_MSG_SEQ_NUMBER: types::MQMO = types::MQMO(
        mqsys::MQMO_MATCH_MSG_SEQ_NUMBER,
    );
    pub const MQMO_MATCH_OFFSET: types::MQMO = types::MQMO(mqsys::MQMO_MATCH_OFFSET);
    pub const MQMO_MATCH_MSG_TOKEN: types::MQMO = types::MQMO(
        mqsys::MQMO_MATCH_MSG_TOKEN,
    );
    pub const MQMT_REQUEST: types::MQMT = types::MQMT(mqsys::MQMT_REQUEST);
    pub const MQMT_REPLY: types::MQMT = types::MQMT(mqsys::MQMT_REPLY);
    pub const MQMT_REPORT: types::MQMT = types::MQMT(mqsys::MQMT_REPORT);
    pub const MQMT_DATAGRAM: types::MQMT = types::MQMT(mqsys::MQMT_DATAGRAM);
    pub const MQMT_MQE_FIELDS_FROM_MQE: types::MQMT = types::MQMT(
        mqsys::MQMT_MQE_FIELDS_FROM_MQE,
    );
    pub const MQMT_MQE_FIELDS: types::MQMT = types::MQMT(mqsys::MQMT_MQE_FIELDS);
    pub const MQMULC_STANDARD: types::MQMULC = types::MQMULC(mqsys::MQMULC_STANDARD);
    pub const MQMULC_REFINED: types::MQMULC = types::MQMULC(mqsys::MQMULC_REFINED);
    pub const MQNC_MAX_NAMELIST_NAME_COUNT: types::MQNC = types::MQNC(
        mqsys::MQNC_MAX_NAMELIST_NAME_COUNT,
    );
    pub const MQNHABACKLOG_UNKNOWN: types::MQNHABACKLOG = types::MQNHABACKLOG(
        mqsys::MQNHABACKLOG_UNKNOWN,
    );
    pub const MQNHACONNACTV_NO: types::MQNHACONNACTV = types::MQNHACONNACTV(
        mqsys::MQNHACONNACTV_NO,
    );
    pub const MQNHACONNACTV_YES: types::MQNHACONNACTV = types::MQNHACONNACTV(
        mqsys::MQNHACONNACTV_YES,
    );
    pub const MQNHACONNGRP_NO: types::MQNHACONNGRP = types::MQNHACONNGRP(
        mqsys::MQNHACONNGRP_NO,
    );
    pub const MQNHACONNGRP_YES: types::MQNHACONNGRP = types::MQNHACONNGRP(
        mqsys::MQNHACONNGRP_YES,
    );
    pub const MQNHACONNGRP_SUSPENDED: types::MQNHACONNGRP = types::MQNHACONNGRP(
        mqsys::MQNHACONNGRP_SUSPENDED,
    );
    pub const MQNHAGRPROLE_UNKNOWN: types::MQNHAGRPROLE = types::MQNHAGRPROLE(
        mqsys::MQNHAGRPROLE_UNKNOWN,
    );
    pub const MQNHAGRPROLE_NOT_CONFIGURED: types::MQNHAGRPROLE = types::MQNHAGRPROLE(
        mqsys::MQNHAGRPROLE_NOT_CONFIGURED,
    );
    pub const MQNHAGRPROLE_LIVE: types::MQNHAGRPROLE = types::MQNHAGRPROLE(
        mqsys::MQNHAGRPROLE_LIVE,
    );
    pub const MQNHAGRPROLE_RECOVERY: types::MQNHAGRPROLE = types::MQNHAGRPROLE(
        mqsys::MQNHAGRPROLE_RECOVERY,
    );
    pub const MQNHAGRPROLE_PENDING_LIVE: types::MQNHAGRPROLE = types::MQNHAGRPROLE(
        mqsys::MQNHAGRPROLE_PENDING_LIVE,
    );
    pub const MQNHAGRPROLE_PENDING_RECOVERY: types::MQNHAGRPROLE = types::MQNHAGRPROLE(
        mqsys::MQNHAGRPROLE_PENDING_RECOVERY,
    );
    pub const MQNHAINSYNC_NO: types::MQNHAINSYNC = types::MQNHAINSYNC(
        mqsys::MQNHAINSYNC_NO,
    );
    pub const MQNHAINSYNC_YES: types::MQNHAINSYNC = types::MQNHAINSYNC(
        mqsys::MQNHAINSYNC_YES,
    );
    pub const MQNHAROLE_UNKNOWN: types::MQNHAROLE = types::MQNHAROLE(
        mqsys::MQNHAROLE_UNKNOWN,
    );
    pub const MQNHAROLE_ACTIVE: types::MQNHAROLE = types::MQNHAROLE(
        mqsys::MQNHAROLE_ACTIVE,
    );
    pub const MQNHAROLE_REPLICA: types::MQNHAROLE = types::MQNHAROLE(
        mqsys::MQNHAROLE_REPLICA,
    );
    pub const MQNHAROLE_LEADER: types::MQNHAROLE = types::MQNHAROLE(
        mqsys::MQNHAROLE_LEADER,
    );
    pub const MQNHASTATUS_UNKNOWN: types::MQNHASTATUS = types::MQNHASTATUS(
        mqsys::MQNHASTATUS_UNKNOWN,
    );
    pub const MQNHASTATUS_NORMAL: types::MQNHASTATUS = types::MQNHASTATUS(
        mqsys::MQNHASTATUS_NORMAL,
    );
    pub const MQNHASTATUS_CHECKING: types::MQNHASTATUS = types::MQNHASTATUS(
        mqsys::MQNHASTATUS_CHECKING,
    );
    pub const MQNHASTATUS_SYNCHRONIZING: types::MQNHASTATUS = types::MQNHASTATUS(
        mqsys::MQNHASTATUS_SYNCHRONIZING,
    );
    pub const MQNHASTATUS_REBASING: types::MQNHASTATUS = types::MQNHASTATUS(
        mqsys::MQNHASTATUS_REBASING,
    );
    pub const MQNHASTATUS_DISK_FULL: types::MQNHASTATUS = types::MQNHASTATUS(
        mqsys::MQNHASTATUS_DISK_FULL,
    );
    pub const MQNHASTATUS_DISCONNECTED: types::MQNHASTATUS = types::MQNHASTATUS(
        mqsys::MQNHASTATUS_DISCONNECTED,
    );
    pub const MQNHASTATUS_PARTITIONED: types::MQNHASTATUS = types::MQNHASTATUS(
        mqsys::MQNHASTATUS_PARTITIONED,
    );
    pub const MQNHATYPE_ALL: types::MQNHATYPE = types::MQNHATYPE(mqsys::MQNHATYPE_ALL);
    pub const MQNHATYPE_INSTANCE: types::MQNHATYPE = types::MQNHATYPE(
        mqsys::MQNHATYPE_INSTANCE,
    );
    pub const MQNHATYPE_GROUP: types::MQNHATYPE = types::MQNHATYPE(
        mqsys::MQNHATYPE_GROUP,
    );
    pub const MQNPMS_NORMAL: types::MQNPMS = types::MQNPMS(mqsys::MQNPMS_NORMAL);
    pub const MQNPMS_FAST: types::MQNPMS = types::MQNPMS(mqsys::MQNPMS_FAST);
    pub const MQNPM_CLASS_NORMAL: types::MQNPM = types::MQNPM(mqsys::MQNPM_CLASS_NORMAL);
    pub const MQNPM_CLASS_HIGH: types::MQNPM = types::MQNPM(mqsys::MQNPM_CLASS_HIGH);
    pub const MQNSH_ALL: types::MQNSH = types::MQNSH(mqsys::MQNSH_ALL);
    pub const MQNSH_NONE: types::MQNSH = types::MQNSH(mqsys::MQNSH_NONE);
    pub const MQNT_NONE: types::MQNT = types::MQNT(mqsys::MQNT_NONE);
    pub const MQNT_Q: types::MQNT = types::MQNT(mqsys::MQNT_Q);
    pub const MQNT_CLUSTER: types::MQNT = types::MQNT(mqsys::MQNT_CLUSTER);
    pub const MQNT_AUTH_INFO: types::MQNT = types::MQNT(mqsys::MQNT_AUTH_INFO);
    pub const MQNT_ALL: types::MQNT = types::MQNT(mqsys::MQNT_ALL);
    pub const MQOL_UNDEFINED: types::MQOL = types::MQOL(mqsys::MQOL_UNDEFINED);
    pub const MQOM_NO: types::MQOM = types::MQOM(mqsys::MQOM_NO);
    pub const MQOM_YES: types::MQOM = types::MQOM(mqsys::MQOM_YES);
    pub const MQOO_READ_AHEAD_AS_Q_DEF: types::MQOO = types::MQOO(
        mqsys::MQOO_READ_AHEAD_AS_Q_DEF,
    );
    pub const MQOO_INPUT_AS_Q_DEF: types::MQOO = types::MQOO(mqsys::MQOO_INPUT_AS_Q_DEF);
    pub const MQOO_INPUT_SHARED: types::MQOO = types::MQOO(mqsys::MQOO_INPUT_SHARED);
    pub const MQOO_INPUT_EXCLUSIVE: types::MQOO = types::MQOO(
        mqsys::MQOO_INPUT_EXCLUSIVE,
    );
    pub const MQOO_BROWSE: types::MQOO = types::MQOO(mqsys::MQOO_BROWSE);
    pub const MQOO_OUTPUT: types::MQOO = types::MQOO(mqsys::MQOO_OUTPUT);
    pub const MQOO_INQUIRE: types::MQOO = types::MQOO(mqsys::MQOO_INQUIRE);
    pub const MQOO_SET: types::MQOO = types::MQOO(mqsys::MQOO_SET);
    pub const MQOO_SAVE_ALL_CONTEXT: types::MQOO = types::MQOO(
        mqsys::MQOO_SAVE_ALL_CONTEXT,
    );
    pub const MQOO_PASS_IDENTITY_CONTEXT: types::MQOO = types::MQOO(
        mqsys::MQOO_PASS_IDENTITY_CONTEXT,
    );
    pub const MQOO_PASS_ALL_CONTEXT: types::MQOO = types::MQOO(
        mqsys::MQOO_PASS_ALL_CONTEXT,
    );
    pub const MQOO_SET_IDENTITY_CONTEXT: types::MQOO = types::MQOO(
        mqsys::MQOO_SET_IDENTITY_CONTEXT,
    );
    pub const MQOO_SET_ALL_CONTEXT: types::MQOO = types::MQOO(
        mqsys::MQOO_SET_ALL_CONTEXT,
    );
    pub const MQOO_ALTERNATE_USER_AUTHORITY: types::MQOO = types::MQOO(
        mqsys::MQOO_ALTERNATE_USER_AUTHORITY,
    );
    pub const MQOO_FAIL_IF_QUIESCING: types::MQOO = types::MQOO(
        mqsys::MQOO_FAIL_IF_QUIESCING,
    );
    pub const MQOO_BIND_ON_OPEN: types::MQOO = types::MQOO(mqsys::MQOO_BIND_ON_OPEN);
    pub const MQOO_BIND_NOT_FIXED: types::MQOO = types::MQOO(mqsys::MQOO_BIND_NOT_FIXED);
    pub const MQOO_RESOLVE_NAMES: types::MQOO = types::MQOO(mqsys::MQOO_RESOLVE_NAMES);
    pub const MQOO_CO_OP: types::MQOO = types::MQOO(mqsys::MQOO_CO_OP);
    pub const MQOO_RESOLVE_LOCAL_Q: types::MQOO = types::MQOO(
        mqsys::MQOO_RESOLVE_LOCAL_Q,
    );
    pub const MQOO_NO_READ_AHEAD: types::MQOO = types::MQOO(mqsys::MQOO_NO_READ_AHEAD);
    pub const MQOO_READ_AHEAD: types::MQOO = types::MQOO(mqsys::MQOO_READ_AHEAD);
    pub const MQOO_NO_MULTICAST: types::MQOO = types::MQOO(mqsys::MQOO_NO_MULTICAST);
    pub const MQOO_BIND_ON_GROUP: types::MQOO = types::MQOO(mqsys::MQOO_BIND_ON_GROUP);
    pub const MQOO_BIND_AS_Q_DEF: types::MQOO = types::MQOO(mqsys::MQOO_BIND_AS_Q_DEF);
    pub const MQOO_RESOLVE_LOCAL_TOPIC: types::MQOO = types::MQOO(
        mqsys::MQOO_RESOLVE_LOCAL_TOPIC,
    );
    pub const MQOPER_UNKNOWN: types::MQOPER = types::MQOPER(mqsys::MQOPER_UNKNOWN);
    pub const MQOPER_BROWSE: types::MQOPER = types::MQOPER(mqsys::MQOPER_BROWSE);
    pub const MQOPER_DISCARD: types::MQOPER = types::MQOPER(mqsys::MQOPER_DISCARD);
    pub const MQOPER_GET: types::MQOPER = types::MQOPER(mqsys::MQOPER_GET);
    pub const MQOPER_PUT: types::MQOPER = types::MQOPER(mqsys::MQOPER_PUT);
    pub const MQOPER_PUT_REPLY: types::MQOPER = types::MQOPER(mqsys::MQOPER_PUT_REPLY);
    pub const MQOPER_PUT_REPORT: types::MQOPER = types::MQOPER(mqsys::MQOPER_PUT_REPORT);
    pub const MQOPER_RECEIVE: types::MQOPER = types::MQOPER(mqsys::MQOPER_RECEIVE);
    pub const MQOPER_SEND: types::MQOPER = types::MQOPER(mqsys::MQOPER_SEND);
    pub const MQOPER_TRANSFORM: types::MQOPER = types::MQOPER(mqsys::MQOPER_TRANSFORM);
    pub const MQOPER_PUBLISH: types::MQOPER = types::MQOPER(mqsys::MQOPER_PUBLISH);
    pub const MQOPER_EXCLUDED_PUBLISH: types::MQOPER = types::MQOPER(
        mqsys::MQOPER_EXCLUDED_PUBLISH,
    );
    pub const MQOPER_DISCARDED_PUBLISH: types::MQOPER = types::MQOPER(
        mqsys::MQOPER_DISCARDED_PUBLISH,
    );
    pub const MQOPMODE_COMPAT: types::MQOPMODE = types::MQOPMODE(mqsys::MQOPMODE_COMPAT);
    pub const MQOPMODE_NEW_FUNCTION: types::MQOPMODE = types::MQOPMODE(
        mqsys::MQOPMODE_NEW_FUNCTION,
    );
    pub const MQOP_START: types::MQOP = types::MQOP(mqsys::MQOP_START);
    pub const MQOP_START_WAIT: types::MQOP = types::MQOP(mqsys::MQOP_START_WAIT);
    pub const MQOP_STOP: types::MQOP = types::MQOP(mqsys::MQOP_STOP);
    pub const MQOP_REGISTER: types::MQOP = types::MQOP(mqsys::MQOP_REGISTER);
    pub const MQOP_DEREGISTER: types::MQOP = types::MQOP(mqsys::MQOP_DEREGISTER);
    pub const MQOP_SUSPEND: types::MQOP = types::MQOP(mqsys::MQOP_SUSPEND);
    pub const MQOP_RESUME: types::MQOP = types::MQOP(mqsys::MQOP_RESUME);
    pub const MQOT_NONE: types::MQOT = types::MQOT(mqsys::MQOT_NONE);
    pub const MQOT_Q: types::MQOT = types::MQOT(mqsys::MQOT_Q);
    pub const MQOT_NAMELIST: types::MQOT = types::MQOT(mqsys::MQOT_NAMELIST);
    pub const MQOT_PROCESS: types::MQOT = types::MQOT(mqsys::MQOT_PROCESS);
    pub const MQOT_STORAGE_CLASS: types::MQOT = types::MQOT(mqsys::MQOT_STORAGE_CLASS);
    pub const MQOT_Q_MGR: types::MQOT = types::MQOT(mqsys::MQOT_Q_MGR);
    pub const MQOT_CHANNEL: types::MQOT = types::MQOT(mqsys::MQOT_CHANNEL);
    pub const MQOT_AUTH_INFO: types::MQOT = types::MQOT(mqsys::MQOT_AUTH_INFO);
    pub const MQOT_TOPIC: types::MQOT = types::MQOT(mqsys::MQOT_TOPIC);
    pub const MQOT_COMM_INFO: types::MQOT = types::MQOT(mqsys::MQOT_COMM_INFO);
    pub const MQOT_CF_STRUC: types::MQOT = types::MQOT(mqsys::MQOT_CF_STRUC);
    pub const MQOT_LISTENER: types::MQOT = types::MQOT(mqsys::MQOT_LISTENER);
    pub const MQOT_SERVICE: types::MQOT = types::MQOT(mqsys::MQOT_SERVICE);
    pub const MQOT_RESERVED_1: types::MQOT = types::MQOT(mqsys::MQOT_RESERVED_1);
    pub const MQOT_ALL: types::MQOT = types::MQOT(mqsys::MQOT_ALL);
    pub const MQOT_ALIAS_Q: types::MQOT = types::MQOT(mqsys::MQOT_ALIAS_Q);
    pub const MQOT_MODEL_Q: types::MQOT = types::MQOT(mqsys::MQOT_MODEL_Q);
    pub const MQOT_LOCAL_Q: types::MQOT = types::MQOT(mqsys::MQOT_LOCAL_Q);
    pub const MQOT_REMOTE_Q: types::MQOT = types::MQOT(mqsys::MQOT_REMOTE_Q);
    pub const MQOT_SENDER_CHANNEL: types::MQOT = types::MQOT(mqsys::MQOT_SENDER_CHANNEL);
    pub const MQOT_SERVER_CHANNEL: types::MQOT = types::MQOT(mqsys::MQOT_SERVER_CHANNEL);
    pub const MQOT_REQUESTER_CHANNEL: types::MQOT = types::MQOT(
        mqsys::MQOT_REQUESTER_CHANNEL,
    );
    pub const MQOT_RECEIVER_CHANNEL: types::MQOT = types::MQOT(
        mqsys::MQOT_RECEIVER_CHANNEL,
    );
    pub const MQOT_CURRENT_CHANNEL: types::MQOT = types::MQOT(
        mqsys::MQOT_CURRENT_CHANNEL,
    );
    pub const MQOT_SAVED_CHANNEL: types::MQOT = types::MQOT(mqsys::MQOT_SAVED_CHANNEL);
    pub const MQOT_SVRCONN_CHANNEL: types::MQOT = types::MQOT(
        mqsys::MQOT_SVRCONN_CHANNEL,
    );
    pub const MQOT_CLNTCONN_CHANNEL: types::MQOT = types::MQOT(
        mqsys::MQOT_CLNTCONN_CHANNEL,
    );
    pub const MQOT_SHORT_CHANNEL: types::MQOT = types::MQOT(mqsys::MQOT_SHORT_CHANNEL);
    pub const MQOT_CHLAUTH: types::MQOT = types::MQOT(mqsys::MQOT_CHLAUTH);
    pub const MQOT_REMOTE_Q_MGR_NAME: types::MQOT = types::MQOT(
        mqsys::MQOT_REMOTE_Q_MGR_NAME,
    );
    pub const MQOT_PROT_POLICY: types::MQOT = types::MQOT(mqsys::MQOT_PROT_POLICY);
    pub const MQOT_TT_CHANNEL: types::MQOT = types::MQOT(mqsys::MQOT_TT_CHANNEL);
    pub const MQOT_AMQP_CHANNEL: types::MQOT = types::MQOT(mqsys::MQOT_AMQP_CHANNEL);
    pub const MQOT_AUTH_REC: types::MQOT = types::MQOT(mqsys::MQOT_AUTH_REC);
    pub const MQPAGECLAS_4KB: types::MQPAGECLAS = types::MQPAGECLAS(
        mqsys::MQPAGECLAS_4KB,
    );
    pub const MQPAGECLAS_FIXED4KB: types::MQPAGECLAS = types::MQPAGECLAS(
        mqsys::MQPAGECLAS_FIXED4KB,
    );
    pub const MQPA_DEFAULT: types::MQPA = types::MQPA(mqsys::MQPA_DEFAULT);
    pub const MQPA_CONTEXT: types::MQPA = types::MQPA(mqsys::MQPA_CONTEXT);
    pub const MQPA_ONLY_MCA: types::MQPA = types::MQPA(mqsys::MQPA_ONLY_MCA);
    pub const MQPA_ALTERNATE_OR_MCA: types::MQPA = types::MQPA(
        mqsys::MQPA_ALTERNATE_OR_MCA,
    );
    pub const MQPD_REJECT_UNSUP_MASK: types::MQPD = types::MQPD(
        mqsys::MQPD_REJECT_UNSUP_MASK,
    );
    pub const MQPD_NONE: types::MQPD = types::MQPD(mqsys::MQPD_NONE);
    pub const MQPD_SUPPORT_OPTIONAL: types::MQPD = types::MQPD(
        mqsys::MQPD_SUPPORT_OPTIONAL,
    );
    pub const MQPD_ACCEPT_UNSUP_MASK: types::MQPD = types::MQPD(
        mqsys::MQPD_ACCEPT_UNSUP_MASK,
    );
    pub const MQPD_SUPPORT_REQUIRED_IF_LOCAL: types::MQPD = types::MQPD(
        mqsys::MQPD_SUPPORT_REQUIRED_IF_LOCAL,
    );
    pub const MQPD_ACCEPT_UNSUP_IF_XMIT_MASK: types::MQPD = types::MQPD(
        mqsys::MQPD_ACCEPT_UNSUP_IF_XMIT_MASK,
    );
    pub const MQPD_SUPPORT_REQUIRED: types::MQPD = types::MQPD(
        mqsys::MQPD_SUPPORT_REQUIRED,
    );
    pub const MQPD_NO_CONTEXT: types::MQPD = types::MQPD(mqsys::MQPD_NO_CONTEXT);
    pub const MQPD_USER_CONTEXT: types::MQPD = types::MQPD(mqsys::MQPD_USER_CONTEXT);
    pub const MQPER_PERSISTENCE_AS_PARENT: types::MQPER = types::MQPER(
        mqsys::MQPER_PERSISTENCE_AS_PARENT,
    );
    pub const MQPER_NOT_PERSISTENT: types::MQPER = types::MQPER(
        mqsys::MQPER_NOT_PERSISTENT,
    );
    pub const MQPER_PERSISTENT: types::MQPER = types::MQPER(mqsys::MQPER_PERSISTENT);
    pub const MQPER_PERSISTENCE_AS_Q_DEF: types::MQPER = types::MQPER(
        mqsys::MQPER_PERSISTENCE_AS_Q_DEF,
    );
    pub const MQPER_PERSISTENCE_AS_TOPIC_DEF: types::MQPER = types::MQPER(
        mqsys::MQPER_PERSISTENCE_AS_TOPIC_DEF,
    );
    pub const MQPL_ZOS: types::MQPL = types::MQPL(mqsys::MQPL_ZOS);
    pub const MQPL_OS2: types::MQPL = types::MQPL(mqsys::MQPL_OS2);
    pub const MQPL_UNIX: types::MQPL = types::MQPL(mqsys::MQPL_UNIX);
    pub const MQPL_OS400: types::MQPL = types::MQPL(mqsys::MQPL_OS400);
    pub const MQPL_WINDOWS: types::MQPL = types::MQPL(mqsys::MQPL_WINDOWS);
    pub const MQPL_WINDOWS_NT: types::MQPL = types::MQPL(mqsys::MQPL_WINDOWS_NT);
    pub const MQPL_VMS: types::MQPL = types::MQPL(mqsys::MQPL_VMS);
    pub const MQPL_NSK: types::MQPL = types::MQPL(mqsys::MQPL_NSK);
    pub const MQPL_OPEN_TP1: types::MQPL = types::MQPL(mqsys::MQPL_OPEN_TP1);
    pub const MQPL_VM: types::MQPL = types::MQPL(mqsys::MQPL_VM);
    pub const MQPL_TPF: types::MQPL = types::MQPL(mqsys::MQPL_TPF);
    pub const MQPL_VSE: types::MQPL = types::MQPL(mqsys::MQPL_VSE);
    pub const MQPL_APPLIANCE: types::MQPL = types::MQPL(mqsys::MQPL_APPLIANCE);
    pub const MQPL_MVS: types::MQPL = types::MQPL(mqsys::MQPL_MVS);
    pub const MQPL_OS390: types::MQPL = types::MQPL(mqsys::MQPL_OS390);
    pub const MQPL_AIX: types::MQPL = types::MQPL(mqsys::MQPL_AIX);
    pub const MQPL_NSS: types::MQPL = types::MQPL(mqsys::MQPL_NSS);
    pub const MQPMO_NONE: types::MQPMO = types::MQPMO(mqsys::MQPMO_NONE);
    pub const MQPMO_SYNCPOINT: types::MQPMO = types::MQPMO(mqsys::MQPMO_SYNCPOINT);
    pub const MQPMO_NO_SYNCPOINT: types::MQPMO = types::MQPMO(mqsys::MQPMO_NO_SYNCPOINT);
    pub const MQPMO_DEFAULT_CONTEXT: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_DEFAULT_CONTEXT,
    );
    pub const MQPMO_NEW_MSG_ID: types::MQPMO = types::MQPMO(mqsys::MQPMO_NEW_MSG_ID);
    pub const MQPMO_NEW_CORREL_ID: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_NEW_CORREL_ID,
    );
    pub const MQPMO_PASS_IDENTITY_CONTEXT: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_PASS_IDENTITY_CONTEXT,
    );
    pub const MQPMO_PASS_ALL_CONTEXT: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_PASS_ALL_CONTEXT,
    );
    pub const MQPMO_SET_IDENTITY_CONTEXT: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_SET_IDENTITY_CONTEXT,
    );
    pub const MQPMO_SET_ALL_CONTEXT: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_SET_ALL_CONTEXT,
    );
    pub const MQPMO_ALTERNATE_USER_AUTHORITY: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_ALTERNATE_USER_AUTHORITY,
    );
    pub const MQPMO_FAIL_IF_QUIESCING: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_FAIL_IF_QUIESCING,
    );
    pub const MQPMO_NO_CONTEXT: types::MQPMO = types::MQPMO(mqsys::MQPMO_NO_CONTEXT);
    pub const MQPMO_LOGICAL_ORDER: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_LOGICAL_ORDER,
    );
    pub const MQPMO_ASYNC_RESPONSE: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_ASYNC_RESPONSE,
    );
    pub const MQPMO_SYNC_RESPONSE: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_SYNC_RESPONSE,
    );
    pub const MQPMO_RESOLVE_LOCAL_Q: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_RESOLVE_LOCAL_Q,
    );
    pub const MQPMO_WARN_IF_NO_SUBS_MATCHED: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_WARN_IF_NO_SUBS_MATCHED,
    );
    pub const MQPMO_RETAIN: types::MQPMO = types::MQPMO(mqsys::MQPMO_RETAIN);
    pub const MQPMO_MD_FOR_OUTPUT_ONLY: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_MD_FOR_OUTPUT_ONLY,
    );
    pub const MQPMO_SCOPE_QMGR: types::MQPMO = types::MQPMO(mqsys::MQPMO_SCOPE_QMGR);
    pub const MQPMO_SUPPRESS_REPLYTO: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_SUPPRESS_REPLYTO,
    );
    pub const MQPMO_NOT_OWN_SUBS: types::MQPMO = types::MQPMO(mqsys::MQPMO_NOT_OWN_SUBS);
    pub const MQPMO_RESPONSE_AS_Q_DEF: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_RESPONSE_AS_Q_DEF,
    );
    pub const MQPMO_RESPONSE_AS_TOPIC_DEF: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_RESPONSE_AS_TOPIC_DEF,
    );
    pub const MQPMO_PUB_OPTIONS_MASK: types::MQPMO = types::MQPMO(
        mqsys::MQPMO_PUB_OPTIONS_MASK,
    );
    pub const MQPMRF_NONE: types::MQPMRF = types::MQPMRF(mqsys::MQPMRF_NONE);
    pub const MQPMRF_MSG_ID: types::MQPMRF = types::MQPMRF(mqsys::MQPMRF_MSG_ID);
    pub const MQPMRF_CORREL_ID: types::MQPMRF = types::MQPMRF(mqsys::MQPMRF_CORREL_ID);
    pub const MQPMRF_GROUP_ID: types::MQPMRF = types::MQPMRF(mqsys::MQPMRF_GROUP_ID);
    pub const MQPMRF_FEEDBACK: types::MQPMRF = types::MQPMRF(mqsys::MQPMRF_FEEDBACK);
    pub const MQPMRF_ACCOUNTING_TOKEN: types::MQPMRF = types::MQPMRF(
        mqsys::MQPMRF_ACCOUNTING_TOKEN,
    );
    pub const MQPO_NO: types::MQPO = types::MQPO(mqsys::MQPO_NO);
    pub const MQPO_YES: types::MQPO = types::MQPO(mqsys::MQPO_YES);
    pub const MQPRI_PRIORITY_AS_PUBLISHED: types::MQPRI = types::MQPRI(
        mqsys::MQPRI_PRIORITY_AS_PUBLISHED,
    );
    pub const MQPRI_PRIORITY_AS_PARENT: types::MQPRI = types::MQPRI(
        mqsys::MQPRI_PRIORITY_AS_PARENT,
    );
    pub const MQPRI_PRIORITY_AS_Q_DEF: types::MQPRI = types::MQPRI(
        mqsys::MQPRI_PRIORITY_AS_Q_DEF,
    );
    pub const MQPRI_PRIORITY_AS_TOPIC_DEF: types::MQPRI = types::MQPRI(
        mqsys::MQPRI_PRIORITY_AS_TOPIC_DEF,
    );
    pub const MQPROP_UNRESTRICTED_LENGTH: types::MQPROP = types::MQPROP(
        mqsys::MQPROP_UNRESTRICTED_LENGTH,
    );
    pub const MQPROP_COMPATIBILITY: types::MQPROP = types::MQPROP(
        mqsys::MQPROP_COMPATIBILITY,
    );
    pub const MQPROP_NONE: types::MQPROP = types::MQPROP(mqsys::MQPROP_NONE);
    pub const MQPROP_ALL: types::MQPROP = types::MQPROP(mqsys::MQPROP_ALL);
    pub const MQPROP_FORCE_MQRFH2: types::MQPROP = types::MQPROP(
        mqsys::MQPROP_FORCE_MQRFH2,
    );
    pub const MQPROP_V6COMPAT: types::MQPROP = types::MQPROP(mqsys::MQPROP_V6COMPAT);
    pub const MQPROTO_MQTTV3: types::MQPROTO = types::MQPROTO(mqsys::MQPROTO_MQTTV3);
    pub const MQPROTO_HTTP: types::MQPROTO = types::MQPROTO(mqsys::MQPROTO_HTTP);
    pub const MQPROTO_AMQP: types::MQPROTO = types::MQPROTO(mqsys::MQPROTO_AMQP);
    pub const MQPROTO_MQTTV311: types::MQPROTO = types::MQPROTO(mqsys::MQPROTO_MQTTV311);
    pub const MQPRT_RESPONSE_AS_PARENT: types::MQPRT = types::MQPRT(
        mqsys::MQPRT_RESPONSE_AS_PARENT,
    );
    pub const MQPRT_SYNC_RESPONSE: types::MQPRT = types::MQPRT(
        mqsys::MQPRT_SYNC_RESPONSE,
    );
    pub const MQPRT_ASYNC_RESPONSE: types::MQPRT = types::MQPRT(
        mqsys::MQPRT_ASYNC_RESPONSE,
    );
    pub const MQPSCLUS_DISABLED: types::MQPSCLUS = types::MQPSCLUS(
        mqsys::MQPSCLUS_DISABLED,
    );
    pub const MQPSCLUS_ENABLED: types::MQPSCLUS = types::MQPSCLUS(
        mqsys::MQPSCLUS_ENABLED,
    );
    pub const MQPSCT_NONE: types::MQPSCT = types::MQPSCT(mqsys::MQPSCT_NONE);
    pub const MQPSM_DISABLED: types::MQPSM = types::MQPSM(mqsys::MQPSM_DISABLED);
    pub const MQPSM_COMPAT: types::MQPSM = types::MQPSM(mqsys::MQPSM_COMPAT);
    pub const MQPSM_ENABLED: types::MQPSM = types::MQPSM(mqsys::MQPSM_ENABLED);
    pub const MQPSPROP_NONE: types::MQPSPROP = types::MQPSPROP(mqsys::MQPSPROP_NONE);
    pub const MQPSPROP_COMPAT: types::MQPSPROP = types::MQPSPROP(mqsys::MQPSPROP_COMPAT);
    pub const MQPSPROP_RFH2: types::MQPSPROP = types::MQPSPROP(mqsys::MQPSPROP_RFH2);
    pub const MQPSPROP_MSGPROP: types::MQPSPROP = types::MQPSPROP(
        mqsys::MQPSPROP_MSGPROP,
    );
    pub const MQPSST_ALL: types::MQPSST = types::MQPSST(mqsys::MQPSST_ALL);
    pub const MQPSST_LOCAL: types::MQPSST = types::MQPSST(mqsys::MQPSST_LOCAL);
    pub const MQPSST_PARENT: types::MQPSST = types::MQPSST(mqsys::MQPSST_PARENT);
    pub const MQPSST_CHILD: types::MQPSST = types::MQPSST(mqsys::MQPSST_CHILD);
    pub const MQPS_STATUS_INACTIVE: types::MQPS = types::MQPS(
        mqsys::MQPS_STATUS_INACTIVE,
    );
    pub const MQPS_STATUS_STARTING: types::MQPS = types::MQPS(
        mqsys::MQPS_STATUS_STARTING,
    );
    pub const MQPS_STATUS_STOPPING: types::MQPS = types::MQPS(
        mqsys::MQPS_STATUS_STOPPING,
    );
    pub const MQPS_STATUS_ACTIVE: types::MQPS = types::MQPS(mqsys::MQPS_STATUS_ACTIVE);
    pub const MQPS_STATUS_COMPAT: types::MQPS = types::MQPS(mqsys::MQPS_STATUS_COMPAT);
    pub const MQPS_STATUS_ERROR: types::MQPS = types::MQPS(mqsys::MQPS_STATUS_ERROR);
    pub const MQPS_STATUS_REFUSED: types::MQPS = types::MQPS(mqsys::MQPS_STATUS_REFUSED);
    pub const MQPUBO_NONE: types::MQPUBO = types::MQPUBO(mqsys::MQPUBO_NONE);
    pub const MQPUBO_CORREL_ID_AS_IDENTITY: types::MQPUBO = types::MQPUBO(
        mqsys::MQPUBO_CORREL_ID_AS_IDENTITY,
    );
    pub const MQPUBO_RETAIN_PUBLICATION: types::MQPUBO = types::MQPUBO(
        mqsys::MQPUBO_RETAIN_PUBLICATION,
    );
    pub const MQPUBO_OTHER_SUBSCRIBERS_ONLY: types::MQPUBO = types::MQPUBO(
        mqsys::MQPUBO_OTHER_SUBSCRIBERS_ONLY,
    );
    pub const MQPUBO_NO_REGISTRATION: types::MQPUBO = types::MQPUBO(
        mqsys::MQPUBO_NO_REGISTRATION,
    );
    pub const MQPUBO_IS_RETAINED_PUBLICATION: types::MQPUBO = types::MQPUBO(
        mqsys::MQPUBO_IS_RETAINED_PUBLICATION,
    );
    pub const MQQA_BACKOUT_NOT_HARDENED: types::MQQA_BACKOUT = types::MQQA_BACKOUT(
        mqsys::MQQA_BACKOUT_NOT_HARDENED,
    );
    pub const MQQA_BACKOUT_HARDENED: types::MQQA_BACKOUT = types::MQQA_BACKOUT(
        mqsys::MQQA_BACKOUT_HARDENED,
    );
    pub const MQQA_GET_ALLOWED: types::MQQA_GET = types::MQQA_GET(
        mqsys::MQQA_GET_ALLOWED,
    );
    pub const MQQA_GET_INHIBITED: types::MQQA_GET = types::MQQA_GET(
        mqsys::MQQA_GET_INHIBITED,
    );
    pub const MQQA_PUT_ALLOWED: types::MQQA_PUT = types::MQQA_PUT(
        mqsys::MQQA_PUT_ALLOWED,
    );
    pub const MQQA_PUT_INHIBITED: types::MQQA_PUT = types::MQQA_PUT(
        mqsys::MQQA_PUT_INHIBITED,
    );
    pub const MQQA_NOT_SHAREABLE: types::MQQA_SHAREABLE = types::MQQA_SHAREABLE(
        mqsys::MQQA_NOT_SHAREABLE,
    );
    pub const MQQA_SHAREABLE: types::MQQA_SHAREABLE = types::MQQA_SHAREABLE(
        mqsys::MQQA_SHAREABLE,
    );
    pub const MQQDT_PREDEFINED: types::MQQDT = types::MQQDT(mqsys::MQQDT_PREDEFINED);
    pub const MQQDT_PERMANENT_DYNAMIC: types::MQQDT = types::MQQDT(
        mqsys::MQQDT_PERMANENT_DYNAMIC,
    );
    pub const MQQDT_TEMPORARY_DYNAMIC: types::MQQDT = types::MQQDT(
        mqsys::MQQDT_TEMPORARY_DYNAMIC,
    );
    pub const MQQDT_SHARED_DYNAMIC: types::MQQDT = types::MQQDT(
        mqsys::MQQDT_SHARED_DYNAMIC,
    );
    pub const MQQFS_DEFAULT: types::MQQFS = types::MQQFS(mqsys::MQQFS_DEFAULT);
    pub const MQQF_LOCAL_Q: types::MQQF = types::MQQF(mqsys::MQQF_LOCAL_Q);
    pub const MQQF_CLWL_USEQ_ANY: types::MQQF = types::MQQF(mqsys::MQQF_CLWL_USEQ_ANY);
    pub const MQQF_CLWL_USEQ_LOCAL: types::MQQF = types::MQQF(
        mqsys::MQQF_CLWL_USEQ_LOCAL,
    );
    pub const MQQMDT_EXPLICIT_CLUSTER_SENDER: types::MQQMDT = types::MQQMDT(
        mqsys::MQQMDT_EXPLICIT_CLUSTER_SENDER,
    );
    pub const MQQMDT_AUTO_CLUSTER_SENDER: types::MQQMDT = types::MQQMDT(
        mqsys::MQQMDT_AUTO_CLUSTER_SENDER,
    );
    pub const MQQMDT_CLUSTER_RECEIVER: types::MQQMDT = types::MQQMDT(
        mqsys::MQQMDT_CLUSTER_RECEIVER,
    );
    pub const MQQMDT_AUTO_EXP_CLUSTER_SENDER: types::MQQMDT = types::MQQMDT(
        mqsys::MQQMDT_AUTO_EXP_CLUSTER_SENDER,
    );
    pub const MQQMFAC_IMS_BRIDGE: types::MQQMFAC = types::MQQMFAC(
        mqsys::MQQMFAC_IMS_BRIDGE,
    );
    pub const MQQMFAC_DB2: types::MQQMFAC = types::MQQMFAC(mqsys::MQQMFAC_DB2);
    pub const MQQMF_REPOSITORY_Q_MGR: types::MQQMF = types::MQQMF(
        mqsys::MQQMF_REPOSITORY_Q_MGR,
    );
    pub const MQQMF_CLUSSDR_USER_DEFINED: types::MQQMF = types::MQQMF(
        mqsys::MQQMF_CLUSSDR_USER_DEFINED,
    );
    pub const MQQMF_CLUSSDR_AUTO_DEFINED: types::MQQMF = types::MQQMF(
        mqsys::MQQMF_CLUSSDR_AUTO_DEFINED,
    );
    pub const MQQMF_AVAILABLE: types::MQQMF = types::MQQMF(mqsys::MQQMF_AVAILABLE);
    pub const MQQMOPT_DISABLED: types::MQQMOPT = types::MQQMOPT(mqsys::MQQMOPT_DISABLED);
    pub const MQQMOPT_ENABLED: types::MQQMOPT = types::MQQMOPT(mqsys::MQQMOPT_ENABLED);
    pub const MQQMOPT_REPLY: types::MQQMOPT = types::MQQMOPT(mqsys::MQQMOPT_REPLY);
    pub const MQQMSTA_STARTING: types::MQQMSTA = types::MQQMSTA(mqsys::MQQMSTA_STARTING);
    pub const MQQMSTA_RUNNING: types::MQQMSTA = types::MQQMSTA(mqsys::MQQMSTA_RUNNING);
    pub const MQQMSTA_QUIESCING: types::MQQMSTA = types::MQQMSTA(
        mqsys::MQQMSTA_QUIESCING,
    );
    pub const MQQMSTA_STANDBY: types::MQQMSTA = types::MQQMSTA(mqsys::MQQMSTA_STANDBY);
    pub const MQQMT_NORMAL: types::MQQMT = types::MQQMT(mqsys::MQQMT_NORMAL);
    pub const MQQMT_REPOSITORY: types::MQQMT = types::MQQMT(mqsys::MQQMT_REPOSITORY);
    pub const MQQO_NO: types::MQQO = types::MQQO(mqsys::MQQO_NO);
    pub const MQQO_YES: types::MQQO = types::MQQO(mqsys::MQQO_YES);
    pub const MQQSGD_ALL: types::MQQSGD = types::MQQSGD(mqsys::MQQSGD_ALL);
    pub const MQQSGD_Q_MGR: types::MQQSGD = types::MQQSGD(mqsys::MQQSGD_Q_MGR);
    pub const MQQSGD_COPY: types::MQQSGD = types::MQQSGD(mqsys::MQQSGD_COPY);
    pub const MQQSGD_SHARED: types::MQQSGD = types::MQQSGD(mqsys::MQQSGD_SHARED);
    pub const MQQSGD_GROUP: types::MQQSGD = types::MQQSGD(mqsys::MQQSGD_GROUP);
    pub const MQQSGD_PRIVATE: types::MQQSGD = types::MQQSGD(mqsys::MQQSGD_PRIVATE);
    pub const MQQSGD_LIVE: types::MQQSGD = types::MQQSGD(mqsys::MQQSGD_LIVE);
    pub const MQQSGS_UNKNOWN: types::MQQSGS = types::MQQSGS(mqsys::MQQSGS_UNKNOWN);
    pub const MQQSGS_CREATED: types::MQQSGS = types::MQQSGS(mqsys::MQQSGS_CREATED);
    pub const MQQSGS_ACTIVE: types::MQQSGS = types::MQQSGS(mqsys::MQQSGS_ACTIVE);
    pub const MQQSGS_INACTIVE: types::MQQSGS = types::MQQSGS(mqsys::MQQSGS_INACTIVE);
    pub const MQQSGS_FAILED: types::MQQSGS = types::MQQSGS(mqsys::MQQSGS_FAILED);
    pub const MQQSGS_PENDING: types::MQQSGS = types::MQQSGS(mqsys::MQQSGS_PENDING);
    pub const MQQSIE_NONE: types::MQQSIE = types::MQQSIE(mqsys::MQQSIE_NONE);
    pub const MQQSIE_HIGH: types::MQQSIE = types::MQQSIE(mqsys::MQQSIE_HIGH);
    pub const MQQSIE_OK: types::MQQSIE = types::MQQSIE(mqsys::MQQSIE_OK);
    pub const MQQSOT_ALL: types::MQQSOT = types::MQQSOT(mqsys::MQQSOT_ALL);
    pub const MQQSOT_INPUT: types::MQQSOT = types::MQQSOT(mqsys::MQQSOT_INPUT);
    pub const MQQSOT_OUTPUT: types::MQQSOT = types::MQQSOT(mqsys::MQQSOT_OUTPUT);
    pub const MQQSO_NO: types::MQQSO = types::MQQSO(mqsys::MQQSO_NO);
    pub const MQQSO_YES: types::MQQSO = types::MQQSO(mqsys::MQQSO_YES);
    pub const MQQSO_EXCLUSIVE: types::MQQSO = types::MQQSO(mqsys::MQQSO_EXCLUSIVE);
    pub const MQQSO_SHARED: types::MQQSO = types::MQQSO(mqsys::MQQSO_SHARED);
    pub const MQQSUM_NO: types::MQQSUM = types::MQQSUM(mqsys::MQQSUM_NO);
    pub const MQQSUM_YES: types::MQQSUM = types::MQQSUM(mqsys::MQQSUM_YES);
    pub const MQQT_LOCAL: types::MQQT = types::MQQT(mqsys::MQQT_LOCAL);
    pub const MQQT_MODEL: types::MQQT = types::MQQT(mqsys::MQQT_MODEL);
    pub const MQQT_ALIAS: types::MQQT = types::MQQT(mqsys::MQQT_ALIAS);
    pub const MQQT_REMOTE: types::MQQT = types::MQQT(mqsys::MQQT_REMOTE);
    pub const MQQT_CLUSTER: types::MQQT = types::MQQT(mqsys::MQQT_CLUSTER);
    pub const MQQT_ALL: types::MQQT = types::MQQT(mqsys::MQQT_ALL);
    pub const MQRAR_NO: types::MQRAR = types::MQRAR(mqsys::MQRAR_NO);
    pub const MQRAR_YES: types::MQRAR = types::MQRAR(mqsys::MQRAR_YES);
    pub const MQRCCF_CFH_TYPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFH_TYPE_ERROR,
    );
    pub const MQRCCF_CFH_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFH_LENGTH_ERROR,
    );
    pub const MQRCCF_CFH_VERSION_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFH_VERSION_ERROR,
    );
    pub const MQRCCF_CFH_MSG_SEQ_NUMBER_ERR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFH_MSG_SEQ_NUMBER_ERR,
    );
    pub const MQRCCF_CFH_CONTROL_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFH_CONTROL_ERROR,
    );
    pub const MQRCCF_CFH_PARM_COUNT_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFH_PARM_COUNT_ERROR,
    );
    pub const MQRCCF_CFH_COMMAND_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFH_COMMAND_ERROR,
    );
    pub const MQRCCF_COMMAND_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_COMMAND_FAILED,
    );
    pub const MQRCCF_CFIN_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFIN_LENGTH_ERROR,
    );
    pub const MQRCCF_CFST_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFST_LENGTH_ERROR,
    );
    pub const MQRCCF_CFST_STRING_LENGTH_ERR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFST_STRING_LENGTH_ERR,
    );
    pub const MQRCCF_FORCE_VALUE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_FORCE_VALUE_ERROR,
    );
    pub const MQRCCF_STRUCTURE_TYPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_STRUCTURE_TYPE_ERROR,
    );
    pub const MQRCCF_CFIN_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFIN_PARM_ID_ERROR,
    );
    pub const MQRCCF_CFST_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFST_PARM_ID_ERROR,
    );
    pub const MQRCCF_MSG_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MSG_LENGTH_ERROR,
    );
    pub const MQRCCF_CFIN_DUPLICATE_PARM: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFIN_DUPLICATE_PARM,
    );
    pub const MQRCCF_CFST_DUPLICATE_PARM: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFST_DUPLICATE_PARM,
    );
    pub const MQRCCF_PARM_COUNT_TOO_SMALL: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PARM_COUNT_TOO_SMALL,
    );
    pub const MQRCCF_PARM_COUNT_TOO_BIG: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PARM_COUNT_TOO_BIG,
    );
    pub const MQRCCF_Q_ALREADY_IN_CELL: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_Q_ALREADY_IN_CELL,
    );
    pub const MQRCCF_Q_TYPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_Q_TYPE_ERROR,
    );
    pub const MQRCCF_MD_FORMAT_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MD_FORMAT_ERROR,
    );
    pub const MQRCCF_CFSL_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFSL_LENGTH_ERROR,
    );
    pub const MQRCCF_REPLACE_VALUE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_REPLACE_VALUE_ERROR,
    );
    pub const MQRCCF_CFIL_DUPLICATE_VALUE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFIL_DUPLICATE_VALUE,
    );
    pub const MQRCCF_CFIL_COUNT_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFIL_COUNT_ERROR,
    );
    pub const MQRCCF_CFIL_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFIL_LENGTH_ERROR,
    );
    pub const MQRCCF_QUIESCE_VALUE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_QUIESCE_VALUE_ERROR,
    );
    pub const MQRCCF_MSG_SEQ_NUMBER_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MSG_SEQ_NUMBER_ERROR,
    );
    pub const MQRCCF_PING_DATA_COUNT_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PING_DATA_COUNT_ERROR,
    );
    pub const MQRCCF_PING_DATA_COMPARE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PING_DATA_COMPARE_ERROR,
    );
    pub const MQRCCF_CFSL_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFSL_PARM_ID_ERROR,
    );
    pub const MQRCCF_CHANNEL_TYPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHANNEL_TYPE_ERROR,
    );
    pub const MQRCCF_PARM_SEQUENCE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PARM_SEQUENCE_ERROR,
    );
    pub const MQRCCF_XMIT_PROTOCOL_TYPE_ERR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_XMIT_PROTOCOL_TYPE_ERR,
    );
    pub const MQRCCF_BATCH_SIZE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_BATCH_SIZE_ERROR,
    );
    pub const MQRCCF_DISC_INT_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DISC_INT_ERROR,
    );
    pub const MQRCCF_SHORT_RETRY_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SHORT_RETRY_ERROR,
    );
    pub const MQRCCF_SHORT_TIMER_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SHORT_TIMER_ERROR,
    );
    pub const MQRCCF_LONG_RETRY_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LONG_RETRY_ERROR,
    );
    pub const MQRCCF_LONG_TIMER_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LONG_TIMER_ERROR,
    );
    pub const MQRCCF_SEQ_NUMBER_WRAP_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SEQ_NUMBER_WRAP_ERROR,
    );
    pub const MQRCCF_MAX_MSG_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MAX_MSG_LENGTH_ERROR,
    );
    pub const MQRCCF_PUT_AUTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PUT_AUTH_ERROR,
    );
    pub const MQRCCF_PURGE_VALUE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PURGE_VALUE_ERROR,
    );
    pub const MQRCCF_CFIL_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFIL_PARM_ID_ERROR,
    );
    pub const MQRCCF_MSG_TRUNCATED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MSG_TRUNCATED,
    );
    pub const MQRCCF_CCSID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CCSID_ERROR,
    );
    pub const MQRCCF_ENCODING_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ENCODING_ERROR,
    );
    pub const MQRCCF_QUEUES_VALUE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_QUEUES_VALUE_ERROR,
    );
    pub const MQRCCF_DATA_CONV_VALUE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DATA_CONV_VALUE_ERROR,
    );
    pub const MQRCCF_INDOUBT_VALUE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_INDOUBT_VALUE_ERROR,
    );
    pub const MQRCCF_ESCAPE_TYPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ESCAPE_TYPE_ERROR,
    );
    pub const MQRCCF_REPOS_VALUE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_REPOS_VALUE_ERROR,
    );
    pub const MQRCCF_CHANNEL_TABLE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHANNEL_TABLE_ERROR,
    );
    pub const MQRCCF_MCA_TYPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MCA_TYPE_ERROR,
    );
    pub const MQRCCF_CHL_INST_TYPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHL_INST_TYPE_ERROR,
    );
    pub const MQRCCF_CHL_STATUS_NOT_FOUND: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHL_STATUS_NOT_FOUND,
    );
    pub const MQRCCF_CFSL_DUPLICATE_PARM: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFSL_DUPLICATE_PARM,
    );
    pub const MQRCCF_CFSL_TOTAL_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFSL_TOTAL_LENGTH_ERROR,
    );
    pub const MQRCCF_CFSL_COUNT_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFSL_COUNT_ERROR,
    );
    pub const MQRCCF_CFSL_STRING_LENGTH_ERR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFSL_STRING_LENGTH_ERR,
    );
    pub const MQRCCF_BROKER_DELETED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_BROKER_DELETED,
    );
    pub const MQRCCF_STREAM_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_STREAM_ERROR,
    );
    pub const MQRCCF_TOPIC_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_TOPIC_ERROR,
    );
    pub const MQRCCF_NOT_REGISTERED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NOT_REGISTERED,
    );
    pub const MQRCCF_Q_MGR_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_Q_MGR_NAME_ERROR,
    );
    pub const MQRCCF_INCORRECT_STREAM: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_INCORRECT_STREAM,
    );
    pub const MQRCCF_Q_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_Q_NAME_ERROR,
    );
    pub const MQRCCF_NO_RETAINED_MSG: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NO_RETAINED_MSG,
    );
    pub const MQRCCF_DUPLICATE_IDENTITY: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DUPLICATE_IDENTITY,
    );
    pub const MQRCCF_INCORRECT_Q: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_INCORRECT_Q,
    );
    pub const MQRCCF_CORREL_ID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CORREL_ID_ERROR,
    );
    pub const MQRCCF_NOT_AUTHORIZED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NOT_AUTHORIZED,
    );
    pub const MQRCCF_UNKNOWN_STREAM: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_UNKNOWN_STREAM,
    );
    pub const MQRCCF_REG_OPTIONS_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_REG_OPTIONS_ERROR,
    );
    pub const MQRCCF_PUB_OPTIONS_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PUB_OPTIONS_ERROR,
    );
    pub const MQRCCF_UNKNOWN_BROKER: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_UNKNOWN_BROKER,
    );
    pub const MQRCCF_Q_MGR_CCSID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_Q_MGR_CCSID_ERROR,
    );
    pub const MQRCCF_DEL_OPTIONS_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DEL_OPTIONS_ERROR,
    );
    pub const MQRCCF_CLUSTER_NAME_CONFLICT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CLUSTER_NAME_CONFLICT,
    );
    pub const MQRCCF_REPOS_NAME_CONFLICT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_REPOS_NAME_CONFLICT,
    );
    pub const MQRCCF_CLUSTER_Q_USAGE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CLUSTER_Q_USAGE_ERROR,
    );
    pub const MQRCCF_ACTION_VALUE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ACTION_VALUE_ERROR,
    );
    pub const MQRCCF_COMMS_LIBRARY_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_COMMS_LIBRARY_ERROR,
    );
    pub const MQRCCF_NETBIOS_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NETBIOS_NAME_ERROR,
    );
    pub const MQRCCF_BROKER_COMMAND_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_BROKER_COMMAND_FAILED,
    );
    pub const MQRCCF_CFST_CONFLICTING_PARM: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFST_CONFLICTING_PARM,
    );
    pub const MQRCCF_PATH_NOT_VALID: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PATH_NOT_VALID,
    );
    pub const MQRCCF_PARM_SYNTAX_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PARM_SYNTAX_ERROR,
    );
    pub const MQRCCF_PWD_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PWD_LENGTH_ERROR,
    );
    pub const MQRCCF_FILTER_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_FILTER_ERROR,
    );
    pub const MQRCCF_WRONG_USER: types::MQRCCF = types::MQRCCF(mqsys::MQRCCF_WRONG_USER);
    pub const MQRCCF_DUPLICATE_SUBSCRIPTION: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DUPLICATE_SUBSCRIPTION,
    );
    pub const MQRCCF_SUB_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SUB_NAME_ERROR,
    );
    pub const MQRCCF_SUB_IDENTITY_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SUB_IDENTITY_ERROR,
    );
    pub const MQRCCF_SUBSCRIPTION_IN_USE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SUBSCRIPTION_IN_USE,
    );
    pub const MQRCCF_SUBSCRIPTION_LOCKED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SUBSCRIPTION_LOCKED,
    );
    pub const MQRCCF_ALREADY_JOINED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ALREADY_JOINED,
    );
    pub const MQRCCF_OBJECT_IN_USE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_OBJECT_IN_USE,
    );
    pub const MQRCCF_UNKNOWN_FILE_NAME: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_UNKNOWN_FILE_NAME,
    );
    pub const MQRCCF_FILE_NOT_AVAILABLE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_FILE_NOT_AVAILABLE,
    );
    pub const MQRCCF_DISC_RETRY_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DISC_RETRY_ERROR,
    );
    pub const MQRCCF_ALLOC_RETRY_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ALLOC_RETRY_ERROR,
    );
    pub const MQRCCF_ALLOC_SLOW_TIMER_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ALLOC_SLOW_TIMER_ERROR,
    );
    pub const MQRCCF_ALLOC_FAST_TIMER_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ALLOC_FAST_TIMER_ERROR,
    );
    pub const MQRCCF_PORT_NUMBER_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PORT_NUMBER_ERROR,
    );
    pub const MQRCCF_CHL_SYSTEM_NOT_ACTIVE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHL_SYSTEM_NOT_ACTIVE,
    );
    pub const MQRCCF_ENTITY_NAME_MISSING: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ENTITY_NAME_MISSING,
    );
    pub const MQRCCF_PROFILE_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PROFILE_NAME_ERROR,
    );
    pub const MQRCCF_AUTH_VALUE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_AUTH_VALUE_ERROR,
    );
    pub const MQRCCF_AUTH_VALUE_MISSING: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_AUTH_VALUE_MISSING,
    );
    pub const MQRCCF_OBJECT_TYPE_MISSING: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_OBJECT_TYPE_MISSING,
    );
    pub const MQRCCF_CONNECTION_ID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CONNECTION_ID_ERROR,
    );
    pub const MQRCCF_LOG_TYPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LOG_TYPE_ERROR,
    );
    pub const MQRCCF_PROGRAM_NOT_AVAILABLE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PROGRAM_NOT_AVAILABLE,
    );
    pub const MQRCCF_PROGRAM_AUTH_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PROGRAM_AUTH_FAILED,
    );
    pub const MQRCCF_NONE_FOUND: types::MQRCCF = types::MQRCCF(mqsys::MQRCCF_NONE_FOUND);
    pub const MQRCCF_SECURITY_SWITCH_OFF: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SECURITY_SWITCH_OFF,
    );
    pub const MQRCCF_SECURITY_REFRESH_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SECURITY_REFRESH_FAILED,
    );
    pub const MQRCCF_PARM_CONFLICT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PARM_CONFLICT,
    );
    pub const MQRCCF_COMMAND_INHIBITED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_COMMAND_INHIBITED,
    );
    pub const MQRCCF_OBJECT_BEING_DELETED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_OBJECT_BEING_DELETED,
    );
    pub const MQRCCF_STORAGE_CLASS_IN_USE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_STORAGE_CLASS_IN_USE,
    );
    pub const MQRCCF_OBJECT_NAME_RESTRICTED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_OBJECT_NAME_RESTRICTED,
    );
    pub const MQRCCF_OBJECT_LIMIT_EXCEEDED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_OBJECT_LIMIT_EXCEEDED,
    );
    pub const MQRCCF_OBJECT_OPEN_FORCE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_OBJECT_OPEN_FORCE,
    );
    pub const MQRCCF_DISPOSITION_CONFLICT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DISPOSITION_CONFLICT,
    );
    pub const MQRCCF_Q_MGR_NOT_IN_QSG: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_Q_MGR_NOT_IN_QSG,
    );
    pub const MQRCCF_ATTR_VALUE_FIXED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ATTR_VALUE_FIXED,
    );
    pub const MQRCCF_NAMELIST_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NAMELIST_ERROR,
    );
    pub const MQRCCF_NO_CHANNEL_INITIATOR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NO_CHANNEL_INITIATOR,
    );
    pub const MQRCCF_CHANNEL_INITIATOR_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHANNEL_INITIATOR_ERROR,
    );
    pub const MQRCCF_COMMAND_LEVEL_CONFLICT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_COMMAND_LEVEL_CONFLICT,
    );
    pub const MQRCCF_Q_ATTR_CONFLICT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_Q_ATTR_CONFLICT,
    );
    pub const MQRCCF_EVENTS_DISABLED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_EVENTS_DISABLED,
    );
    pub const MQRCCF_COMMAND_SCOPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_COMMAND_SCOPE_ERROR,
    );
    pub const MQRCCF_COMMAND_REPLY_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_COMMAND_REPLY_ERROR,
    );
    pub const MQRCCF_FUNCTION_RESTRICTED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_FUNCTION_RESTRICTED,
    );
    pub const MQRCCF_PARM_MISSING: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PARM_MISSING,
    );
    pub const MQRCCF_PARM_VALUE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PARM_VALUE_ERROR,
    );
    pub const MQRCCF_COMMAND_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_COMMAND_LENGTH_ERROR,
    );
    pub const MQRCCF_COMMAND_ORIGIN_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_COMMAND_ORIGIN_ERROR,
    );
    pub const MQRCCF_LISTENER_CONFLICT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LISTENER_CONFLICT,
    );
    pub const MQRCCF_LISTENER_STARTED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LISTENER_STARTED,
    );
    pub const MQRCCF_LISTENER_STOPPED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LISTENER_STOPPED,
    );
    pub const MQRCCF_CHANNEL_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHANNEL_ERROR,
    );
    pub const MQRCCF_CF_STRUC_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CF_STRUC_ERROR,
    );
    pub const MQRCCF_UNKNOWN_USER_ID: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_UNKNOWN_USER_ID,
    );
    pub const MQRCCF_UNEXPECTED_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_UNEXPECTED_ERROR,
    );
    pub const MQRCCF_NO_XCF_PARTNER: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NO_XCF_PARTNER,
    );
    pub const MQRCCF_CFGR_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFGR_PARM_ID_ERROR,
    );
    pub const MQRCCF_CFIF_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFIF_LENGTH_ERROR,
    );
    pub const MQRCCF_CFIF_OPERATOR_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFIF_OPERATOR_ERROR,
    );
    pub const MQRCCF_CFIF_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFIF_PARM_ID_ERROR,
    );
    pub const MQRCCF_CFSF_FILTER_VAL_LEN_ERR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFSF_FILTER_VAL_LEN_ERR,
    );
    pub const MQRCCF_CFSF_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFSF_LENGTH_ERROR,
    );
    pub const MQRCCF_CFSF_OPERATOR_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFSF_OPERATOR_ERROR,
    );
    pub const MQRCCF_CFSF_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFSF_PARM_ID_ERROR,
    );
    pub const MQRCCF_TOO_MANY_FILTERS: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_TOO_MANY_FILTERS,
    );
    pub const MQRCCF_LISTENER_RUNNING: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LISTENER_RUNNING,
    );
    pub const MQRCCF_LSTR_STATUS_NOT_FOUND: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LSTR_STATUS_NOT_FOUND,
    );
    pub const MQRCCF_SERVICE_RUNNING: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SERVICE_RUNNING,
    );
    pub const MQRCCF_SERV_STATUS_NOT_FOUND: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SERV_STATUS_NOT_FOUND,
    );
    pub const MQRCCF_SERVICE_STOPPED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SERVICE_STOPPED,
    );
    pub const MQRCCF_CFBS_DUPLICATE_PARM: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFBS_DUPLICATE_PARM,
    );
    pub const MQRCCF_CFBS_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFBS_LENGTH_ERROR,
    );
    pub const MQRCCF_CFBS_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFBS_PARM_ID_ERROR,
    );
    pub const MQRCCF_CFBS_STRING_LENGTH_ERR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFBS_STRING_LENGTH_ERR,
    );
    pub const MQRCCF_CFGR_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFGR_LENGTH_ERROR,
    );
    pub const MQRCCF_CFGR_PARM_COUNT_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFGR_PARM_COUNT_ERROR,
    );
    pub const MQRCCF_CONN_NOT_STOPPED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CONN_NOT_STOPPED,
    );
    pub const MQRCCF_SERVICE_REQUEST_PENDING: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SERVICE_REQUEST_PENDING,
    );
    pub const MQRCCF_NO_START_CMD: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NO_START_CMD,
    );
    pub const MQRCCF_NO_STOP_CMD: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NO_STOP_CMD,
    );
    pub const MQRCCF_CFBF_LENGTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFBF_LENGTH_ERROR,
    );
    pub const MQRCCF_CFBF_PARM_ID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFBF_PARM_ID_ERROR,
    );
    pub const MQRCCF_CFBF_OPERATOR_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFBF_OPERATOR_ERROR,
    );
    pub const MQRCCF_CFBF_FILTER_VAL_LEN_ERR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFBF_FILTER_VAL_LEN_ERR,
    );
    pub const MQRCCF_LISTENER_STILL_ACTIVE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LISTENER_STILL_ACTIVE,
    );
    pub const MQRCCF_DEF_XMIT_Q_CLUS_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DEF_XMIT_Q_CLUS_ERROR,
    );
    pub const MQRCCF_TOPICSTR_ALREADY_EXISTS: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_TOPICSTR_ALREADY_EXISTS,
    );
    pub const MQRCCF_SHARING_CONVS_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SHARING_CONVS_ERROR,
    );
    pub const MQRCCF_SHARING_CONVS_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SHARING_CONVS_TYPE,
    );
    pub const MQRCCF_SECURITY_CASE_CONFLICT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SECURITY_CASE_CONFLICT,
    );
    pub const MQRCCF_TOPIC_TYPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_TOPIC_TYPE_ERROR,
    );
    pub const MQRCCF_MAX_INSTANCES_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MAX_INSTANCES_ERROR,
    );
    pub const MQRCCF_MAX_INSTS_PER_CLNT_ERR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MAX_INSTS_PER_CLNT_ERR,
    );
    pub const MQRCCF_TOPIC_STRING_NOT_FOUND: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_TOPIC_STRING_NOT_FOUND,
    );
    pub const MQRCCF_SUBSCRIPTION_POINT_ERR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SUBSCRIPTION_POINT_ERR,
    );
    pub const MQRCCF_SUB_ALREADY_EXISTS: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SUB_ALREADY_EXISTS,
    );
    pub const MQRCCF_UNKNOWN_OBJECT_NAME: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_UNKNOWN_OBJECT_NAME,
    );
    pub const MQRCCF_REMOTE_Q_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_REMOTE_Q_NAME_ERROR,
    );
    pub const MQRCCF_DURABILITY_NOT_ALLOWED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DURABILITY_NOT_ALLOWED,
    );
    pub const MQRCCF_HOBJ_ERROR: types::MQRCCF = types::MQRCCF(mqsys::MQRCCF_HOBJ_ERROR);
    pub const MQRCCF_DEST_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DEST_NAME_ERROR,
    );
    pub const MQRCCF_INVALID_DESTINATION: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_INVALID_DESTINATION,
    );
    pub const MQRCCF_PUBSUB_INHIBITED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PUBSUB_INHIBITED,
    );
    pub const MQRCCF_GROUPUR_CHECKS_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_GROUPUR_CHECKS_FAILED,
    );
    pub const MQRCCF_COMM_INFO_TYPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_COMM_INFO_TYPE_ERROR,
    );
    pub const MQRCCF_USE_CLIENT_ID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_USE_CLIENT_ID_ERROR,
    );
    pub const MQRCCF_CLIENT_ID_NOT_FOUND: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CLIENT_ID_NOT_FOUND,
    );
    pub const MQRCCF_CLIENT_ID_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CLIENT_ID_ERROR,
    );
    pub const MQRCCF_PORT_IN_USE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PORT_IN_USE,
    );
    pub const MQRCCF_SSL_ALT_PROVIDER_REQD: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SSL_ALT_PROVIDER_REQD,
    );
    pub const MQRCCF_CHLAUTH_TYPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHLAUTH_TYPE_ERROR,
    );
    pub const MQRCCF_CHLAUTH_ACTION_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHLAUTH_ACTION_ERROR,
    );
    pub const MQRCCF_POLICY_NOT_FOUND: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_POLICY_NOT_FOUND,
    );
    pub const MQRCCF_ENCRYPTION_ALG_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ENCRYPTION_ALG_ERROR,
    );
    pub const MQRCCF_SIGNATURE_ALG_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SIGNATURE_ALG_ERROR,
    );
    pub const MQRCCF_TOLERATION_POL_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_TOLERATION_POL_ERROR,
    );
    pub const MQRCCF_POLICY_VERSION_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_POLICY_VERSION_ERROR,
    );
    pub const MQRCCF_RECIPIENT_DN_MISSING: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_RECIPIENT_DN_MISSING,
    );
    pub const MQRCCF_POLICY_NAME_MISSING: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_POLICY_NAME_MISSING,
    );
    pub const MQRCCF_CHLAUTH_USERSRC_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHLAUTH_USERSRC_ERROR,
    );
    pub const MQRCCF_WRONG_CHLAUTH_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_WRONG_CHLAUTH_TYPE,
    );
    pub const MQRCCF_CHLAUTH_ALREADY_EXISTS: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHLAUTH_ALREADY_EXISTS,
    );
    pub const MQRCCF_CHLAUTH_NOT_FOUND: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHLAUTH_NOT_FOUND,
    );
    pub const MQRCCF_WRONG_CHLAUTH_ACTION: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_WRONG_CHLAUTH_ACTION,
    );
    pub const MQRCCF_WRONG_CHLAUTH_USERSRC: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_WRONG_CHLAUTH_USERSRC,
    );
    pub const MQRCCF_CHLAUTH_WARN_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHLAUTH_WARN_ERROR,
    );
    pub const MQRCCF_WRONG_CHLAUTH_MATCH: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_WRONG_CHLAUTH_MATCH,
    );
    pub const MQRCCF_IPADDR_RANGE_CONFLICT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_IPADDR_RANGE_CONFLICT,
    );
    pub const MQRCCF_CHLAUTH_MAX_EXCEEDED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHLAUTH_MAX_EXCEEDED,
    );
    pub const MQRCCF_ADDRESS_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ADDRESS_ERROR,
    );
    pub const MQRCCF_IPADDR_RANGE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_IPADDR_RANGE_ERROR,
    );
    pub const MQRCCF_PROFILE_NAME_MISSING: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PROFILE_NAME_MISSING,
    );
    pub const MQRCCF_CHLAUTH_CLNTUSER_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHLAUTH_CLNTUSER_ERROR,
    );
    pub const MQRCCF_CHLAUTH_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHLAUTH_NAME_ERROR,
    );
    pub const MQRCCF_CHLAUTH_RUNCHECK_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHLAUTH_RUNCHECK_ERROR,
    );
    pub const MQRCCF_CF_STRUC_ALREADY_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CF_STRUC_ALREADY_FAILED,
    );
    pub const MQRCCF_CFCONLOS_CHECKS_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CFCONLOS_CHECKS_FAILED,
    );
    pub const MQRCCF_SUITE_B_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SUITE_B_ERROR,
    );
    pub const MQRCCF_CHANNEL_NOT_STARTED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHANNEL_NOT_STARTED,
    );
    pub const MQRCCF_CUSTOM_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CUSTOM_ERROR,
    );
    pub const MQRCCF_BACKLOG_OUT_OF_RANGE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_BACKLOG_OUT_OF_RANGE,
    );
    pub const MQRCCF_CHLAUTH_DISABLED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHLAUTH_DISABLED,
    );
    pub const MQRCCF_SMDS_REQUIRES_DSGROUP: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SMDS_REQUIRES_DSGROUP,
    );
    pub const MQRCCF_PSCLUS_DISABLED_TOPDEF: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PSCLUS_DISABLED_TOPDEF,
    );
    pub const MQRCCF_PSCLUS_TOPIC_EXISTS: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PSCLUS_TOPIC_EXISTS,
    );
    pub const MQRCCF_SSL_CIPHER_SUITE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SSL_CIPHER_SUITE_ERROR,
    );
    pub const MQRCCF_SOCKET_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SOCKET_ERROR,
    );
    pub const MQRCCF_CLUS_XMIT_Q_USAGE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CLUS_XMIT_Q_USAGE_ERROR,
    );
    pub const MQRCCF_CERT_VAL_POLICY_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CERT_VAL_POLICY_ERROR,
    );
    pub const MQRCCF_INVALID_PROTOCOL: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_INVALID_PROTOCOL,
    );
    pub const MQRCCF_REVDNS_DISABLED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_REVDNS_DISABLED,
    );
    pub const MQRCCF_CLROUTE_NOT_ALTERABLE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CLROUTE_NOT_ALTERABLE,
    );
    pub const MQRCCF_CLUSTER_TOPIC_CONFLICT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CLUSTER_TOPIC_CONFLICT,
    );
    pub const MQRCCF_DEFCLXQ_MODEL_Q_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DEFCLXQ_MODEL_Q_ERROR,
    );
    pub const MQRCCF_CHLAUTH_CHKCLI_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHLAUTH_CHKCLI_ERROR,
    );
    pub const MQRCCF_CERT_LABEL_NOT_ALLOWED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CERT_LABEL_NOT_ALLOWED,
    );
    pub const MQRCCF_Q_MGR_ATTR_CONFLICT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_Q_MGR_ATTR_CONFLICT,
    );
    pub const MQRCCF_ENTITY_TYPE_MISSING: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ENTITY_TYPE_MISSING,
    );
    pub const MQRCCF_CLWL_EXIT_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CLWL_EXIT_NAME_ERROR,
    );
    pub const MQRCCF_SERVICE_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SERVICE_NAME_ERROR,
    );
    pub const MQRCCF_REMOTE_CHL_TYPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_REMOTE_CHL_TYPE_ERROR,
    );
    pub const MQRCCF_TOPIC_RESTRICTED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_TOPIC_RESTRICTED,
    );
    pub const MQRCCF_CURRENT_LOG_EXTENT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CURRENT_LOG_EXTENT,
    );
    pub const MQRCCF_LOG_EXTENT_NOT_FOUND: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LOG_EXTENT_NOT_FOUND,
    );
    pub const MQRCCF_LOG_NOT_REDUCED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LOG_NOT_REDUCED,
    );
    pub const MQRCCF_LOG_EXTENT_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LOG_EXTENT_ERROR,
    );
    pub const MQRCCF_ACCESS_BLOCKED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ACCESS_BLOCKED,
    );
    pub const MQRCCF_PS_REQUIRED_MQUC: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PS_REQUIRED_MQUC,
    );
    pub const MQRCCF_STREAMQ_DEST_NOT_SUPP: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_STREAMQ_DEST_NOT_SUPP,
    );
    pub const MQRCCF_STREAMQ_DEST_CONFLICT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_STREAMQ_DEST_CONFLICT,
    );
    pub const MQRCCF_STREAMQ_NOT_SUPPORTED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_STREAMQ_NOT_SUPPORTED,
    );
    pub const MQRCCF_STREAMQ_CONFLICT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_STREAMQ_CONFLICT,
    );
    pub const MQRCCF_INCOMPATIBLE_QM_IN_QSG: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_INCOMPATIBLE_QM_IN_QSG,
    );
    pub const MQRCCF_OBJECT_ALREADY_EXISTS: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_OBJECT_ALREADY_EXISTS,
    );
    pub const MQRCCF_OBJECT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_OBJECT_WRONG_TYPE,
    );
    pub const MQRCCF_LIKE_OBJECT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LIKE_OBJECT_WRONG_TYPE,
    );
    pub const MQRCCF_OBJECT_OPEN: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_OBJECT_OPEN,
    );
    pub const MQRCCF_ATTR_VALUE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ATTR_VALUE_ERROR,
    );
    pub const MQRCCF_UNKNOWN_Q_MGR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_UNKNOWN_Q_MGR,
    );
    pub const MQRCCF_Q_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_Q_WRONG_TYPE,
    );
    pub const MQRCCF_OBJECT_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_OBJECT_NAME_ERROR,
    );
    pub const MQRCCF_ALLOCATE_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ALLOCATE_FAILED,
    );
    pub const MQRCCF_HOST_NOT_AVAILABLE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_HOST_NOT_AVAILABLE,
    );
    pub const MQRCCF_CONFIGURATION_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CONFIGURATION_ERROR,
    );
    pub const MQRCCF_CONNECTION_REFUSED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CONNECTION_REFUSED,
    );
    pub const MQRCCF_ENTRY_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_ENTRY_ERROR,
    );
    pub const MQRCCF_SEND_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SEND_FAILED,
    );
    pub const MQRCCF_RECEIVED_DATA_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_RECEIVED_DATA_ERROR,
    );
    pub const MQRCCF_RECEIVE_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_RECEIVE_FAILED,
    );
    pub const MQRCCF_CONNECTION_CLOSED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CONNECTION_CLOSED,
    );
    pub const MQRCCF_NO_STORAGE: types::MQRCCF = types::MQRCCF(mqsys::MQRCCF_NO_STORAGE);
    pub const MQRCCF_NO_COMMS_MANAGER: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NO_COMMS_MANAGER,
    );
    pub const MQRCCF_LISTENER_NOT_STARTED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LISTENER_NOT_STARTED,
    );
    pub const MQRCCF_BIND_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_BIND_FAILED,
    );
    pub const MQRCCF_CHANNEL_INDOUBT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHANNEL_INDOUBT,
    );
    pub const MQRCCF_MQCONN_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MQCONN_FAILED,
    );
    pub const MQRCCF_MQOPEN_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MQOPEN_FAILED,
    );
    pub const MQRCCF_MQGET_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MQGET_FAILED,
    );
    pub const MQRCCF_MQPUT_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MQPUT_FAILED,
    );
    pub const MQRCCF_PING_ERROR: types::MQRCCF = types::MQRCCF(mqsys::MQRCCF_PING_ERROR);
    pub const MQRCCF_CHANNEL_IN_USE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHANNEL_IN_USE,
    );
    pub const MQRCCF_CHANNEL_NOT_FOUND: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHANNEL_NOT_FOUND,
    );
    pub const MQRCCF_UNKNOWN_REMOTE_CHANNEL: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_UNKNOWN_REMOTE_CHANNEL,
    );
    pub const MQRCCF_REMOTE_QM_UNAVAILABLE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_REMOTE_QM_UNAVAILABLE,
    );
    pub const MQRCCF_REMOTE_QM_TERMINATING: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_REMOTE_QM_TERMINATING,
    );
    pub const MQRCCF_MQINQ_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MQINQ_FAILED,
    );
    pub const MQRCCF_NOT_XMIT_Q: types::MQRCCF = types::MQRCCF(mqsys::MQRCCF_NOT_XMIT_Q);
    pub const MQRCCF_CHANNEL_DISABLED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHANNEL_DISABLED,
    );
    pub const MQRCCF_USER_EXIT_NOT_AVAILABLE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_USER_EXIT_NOT_AVAILABLE,
    );
    pub const MQRCCF_COMMIT_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_COMMIT_FAILED,
    );
    pub const MQRCCF_WRONG_CHANNEL_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_WRONG_CHANNEL_TYPE,
    );
    pub const MQRCCF_CHANNEL_ALREADY_EXISTS: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHANNEL_ALREADY_EXISTS,
    );
    pub const MQRCCF_DATA_TOO_LARGE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DATA_TOO_LARGE,
    );
    pub const MQRCCF_CHANNEL_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHANNEL_NAME_ERROR,
    );
    pub const MQRCCF_XMIT_Q_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_XMIT_Q_NAME_ERROR,
    );
    pub const MQRCCF_MCA_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MCA_NAME_ERROR,
    );
    pub const MQRCCF_SEND_EXIT_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SEND_EXIT_NAME_ERROR,
    );
    pub const MQRCCF_SEC_EXIT_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SEC_EXIT_NAME_ERROR,
    );
    pub const MQRCCF_MSG_EXIT_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MSG_EXIT_NAME_ERROR,
    );
    pub const MQRCCF_RCV_EXIT_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_RCV_EXIT_NAME_ERROR,
    );
    pub const MQRCCF_XMIT_Q_NAME_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_XMIT_Q_NAME_WRONG_TYPE,
    );
    pub const MQRCCF_MCA_NAME_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MCA_NAME_WRONG_TYPE,
    );
    pub const MQRCCF_DISC_INT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DISC_INT_WRONG_TYPE,
    );
    pub const MQRCCF_SHORT_RETRY_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SHORT_RETRY_WRONG_TYPE,
    );
    pub const MQRCCF_SHORT_TIMER_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SHORT_TIMER_WRONG_TYPE,
    );
    pub const MQRCCF_LONG_RETRY_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LONG_RETRY_WRONG_TYPE,
    );
    pub const MQRCCF_LONG_TIMER_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_LONG_TIMER_WRONG_TYPE,
    );
    pub const MQRCCF_PUT_AUTH_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_PUT_AUTH_WRONG_TYPE,
    );
    pub const MQRCCF_KEEP_ALIVE_INT_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_KEEP_ALIVE_INT_ERROR,
    );
    pub const MQRCCF_MISSING_CONN_NAME: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MISSING_CONN_NAME,
    );
    pub const MQRCCF_CONN_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CONN_NAME_ERROR,
    );
    pub const MQRCCF_MQSET_FAILED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MQSET_FAILED,
    );
    pub const MQRCCF_CHANNEL_NOT_ACTIVE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHANNEL_NOT_ACTIVE,
    );
    pub const MQRCCF_TERMINATED_BY_SEC_EXIT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_TERMINATED_BY_SEC_EXIT,
    );
    pub const MQRCCF_DYNAMIC_Q_SCOPE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_DYNAMIC_Q_SCOPE_ERROR,
    );
    pub const MQRCCF_CELL_DIR_NOT_AVAILABLE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CELL_DIR_NOT_AVAILABLE,
    );
    pub const MQRCCF_MR_COUNT_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MR_COUNT_ERROR,
    );
    pub const MQRCCF_MR_COUNT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MR_COUNT_WRONG_TYPE,
    );
    pub const MQRCCF_MR_EXIT_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MR_EXIT_NAME_ERROR,
    );
    pub const MQRCCF_MR_EXIT_NAME_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MR_EXIT_NAME_WRONG_TYPE,
    );
    pub const MQRCCF_MR_INTERVAL_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MR_INTERVAL_ERROR,
    );
    pub const MQRCCF_MR_INTERVAL_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MR_INTERVAL_WRONG_TYPE,
    );
    pub const MQRCCF_NPM_SPEED_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NPM_SPEED_ERROR,
    );
    pub const MQRCCF_NPM_SPEED_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NPM_SPEED_WRONG_TYPE,
    );
    pub const MQRCCF_HB_INTERVAL_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_HB_INTERVAL_ERROR,
    );
    pub const MQRCCF_HB_INTERVAL_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_HB_INTERVAL_WRONG_TYPE,
    );
    pub const MQRCCF_CHAD_ERROR: types::MQRCCF = types::MQRCCF(mqsys::MQRCCF_CHAD_ERROR);
    pub const MQRCCF_CHAD_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHAD_WRONG_TYPE,
    );
    pub const MQRCCF_CHAD_EVENT_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHAD_EVENT_ERROR,
    );
    pub const MQRCCF_CHAD_EVENT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHAD_EVENT_WRONG_TYPE,
    );
    pub const MQRCCF_CHAD_EXIT_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHAD_EXIT_ERROR,
    );
    pub const MQRCCF_CHAD_EXIT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHAD_EXIT_WRONG_TYPE,
    );
    pub const MQRCCF_SUPPRESSED_BY_EXIT: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SUPPRESSED_BY_EXIT,
    );
    pub const MQRCCF_BATCH_INT_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_BATCH_INT_ERROR,
    );
    pub const MQRCCF_BATCH_INT_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_BATCH_INT_WRONG_TYPE,
    );
    pub const MQRCCF_NET_PRIORITY_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NET_PRIORITY_ERROR,
    );
    pub const MQRCCF_NET_PRIORITY_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NET_PRIORITY_WRONG_TYPE,
    );
    pub const MQRCCF_CHANNEL_CLOSED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_CHANNEL_CLOSED,
    );
    pub const MQRCCF_Q_STATUS_NOT_FOUND: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_Q_STATUS_NOT_FOUND,
    );
    pub const MQRCCF_SSL_CIPHER_SPEC_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SSL_CIPHER_SPEC_ERROR,
    );
    pub const MQRCCF_SSL_PEER_NAME_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SSL_PEER_NAME_ERROR,
    );
    pub const MQRCCF_SSL_CLIENT_AUTH_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_SSL_CLIENT_AUTH_ERROR,
    );
    pub const MQRCCF_RETAINED_NOT_SUPPORTED: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_RETAINED_NOT_SUPPORTED,
    );
    pub const MQRCCF_KWD_VALUE_WRONG_TYPE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_KWD_VALUE_WRONG_TYPE,
    );
    pub const MQRCCF_APPL_STATUS_NOT_FOUND: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_APPL_STATUS_NOT_FOUND,
    );
    pub const MQRCCF_NHA_NOT_AVAILABLE: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_NHA_NOT_AVAILABLE,
    );
    pub const MQRCCF_Q_MGR_STATUS_NOT_FOUND: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_Q_MGR_STATUS_NOT_FOUND,
    );
    pub const MQRCCF_MODE_VALUE_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_MODE_VALUE_ERROR,
    );
    pub const MQRCCF_IPADDR_ERROR: types::MQRCCF = types::MQRCCF(
        mqsys::MQRCCF_IPADDR_ERROR,
    );
    pub const MQRCN_NO: types::MQRCN = types::MQRCN(mqsys::MQRCN_NO);
    pub const MQRCN_YES: types::MQRCN = types::MQRCN(mqsys::MQRCN_YES);
    pub const MQRCN_Q_MGR: types::MQRCN = types::MQRCN(mqsys::MQRCN_Q_MGR);
    pub const MQRCN_DISABLED: types::MQRCN = types::MQRCN(mqsys::MQRCN_DISABLED);
    pub const MQRCVTIME_MULTIPLY: types::MQRCVTIME = types::MQRCVTIME(
        mqsys::MQRCVTIME_MULTIPLY,
    );
    pub const MQRCVTIME_ADD: types::MQRCVTIME = types::MQRCVTIME(mqsys::MQRCVTIME_ADD);
    pub const MQRCVTIME_EQUAL: types::MQRCVTIME = types::MQRCVTIME(
        mqsys::MQRCVTIME_EQUAL,
    );
    pub const MQRC_NONE: types::MQRC = types::MQRC(mqsys::MQRC_NONE);
    pub const MQRC_ALIAS_BASE_Q_TYPE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ALIAS_BASE_Q_TYPE_ERROR,
    );
    pub const MQRC_ALREADY_CONNECTED: types::MQRC = types::MQRC(
        mqsys::MQRC_ALREADY_CONNECTED,
    );
    pub const MQRC_BACKED_OUT: types::MQRC = types::MQRC(mqsys::MQRC_BACKED_OUT);
    pub const MQRC_BUFFER_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_BUFFER_ERROR);
    pub const MQRC_BUFFER_LENGTH_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_BUFFER_LENGTH_ERROR,
    );
    pub const MQRC_CHAR_ATTR_LENGTH_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CHAR_ATTR_LENGTH_ERROR,
    );
    pub const MQRC_CHAR_ATTRS_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CHAR_ATTRS_ERROR,
    );
    pub const MQRC_CHAR_ATTRS_TOO_SHORT: types::MQRC = types::MQRC(
        mqsys::MQRC_CHAR_ATTRS_TOO_SHORT,
    );
    pub const MQRC_CONNECTION_BROKEN: types::MQRC = types::MQRC(
        mqsys::MQRC_CONNECTION_BROKEN,
    );
    pub const MQRC_DATA_LENGTH_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_DATA_LENGTH_ERROR,
    );
    pub const MQRC_DYNAMIC_Q_NAME_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_DYNAMIC_Q_NAME_ERROR,
    );
    pub const MQRC_ENVIRONMENT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ENVIRONMENT_ERROR,
    );
    pub const MQRC_EXPIRY_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_EXPIRY_ERROR);
    pub const MQRC_FEEDBACK_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_FEEDBACK_ERROR);
    pub const MQRC_GET_INHIBITED: types::MQRC = types::MQRC(mqsys::MQRC_GET_INHIBITED);
    pub const MQRC_HANDLE_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_HANDLE_NOT_AVAILABLE,
    );
    pub const MQRC_HCONN_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_HCONN_ERROR);
    pub const MQRC_HOBJ_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_HOBJ_ERROR);
    pub const MQRC_INHIBIT_VALUE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_INHIBIT_VALUE_ERROR,
    );
    pub const MQRC_INT_ATTR_COUNT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_INT_ATTR_COUNT_ERROR,
    );
    pub const MQRC_INT_ATTR_COUNT_TOO_SMALL: types::MQRC = types::MQRC(
        mqsys::MQRC_INT_ATTR_COUNT_TOO_SMALL,
    );
    pub const MQRC_INT_ATTRS_ARRAY_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_INT_ATTRS_ARRAY_ERROR,
    );
    pub const MQRC_SYNCPOINT_LIMIT_REACHED: types::MQRC = types::MQRC(
        mqsys::MQRC_SYNCPOINT_LIMIT_REACHED,
    );
    pub const MQRC_MAX_CONNS_LIMIT_REACHED: types::MQRC = types::MQRC(
        mqsys::MQRC_MAX_CONNS_LIMIT_REACHED,
    );
    pub const MQRC_MD_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_MD_ERROR);
    pub const MQRC_MISSING_REPLY_TO_Q: types::MQRC = types::MQRC(
        mqsys::MQRC_MISSING_REPLY_TO_Q,
    );
    pub const MQRC_MSG_TYPE_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_MSG_TYPE_ERROR);
    pub const MQRC_MSG_TOO_BIG_FOR_Q: types::MQRC = types::MQRC(
        mqsys::MQRC_MSG_TOO_BIG_FOR_Q,
    );
    pub const MQRC_MSG_TOO_BIG_FOR_Q_MGR: types::MQRC = types::MQRC(
        mqsys::MQRC_MSG_TOO_BIG_FOR_Q_MGR,
    );
    pub const MQRC_NO_MSG_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_NO_MSG_AVAILABLE,
    );
    pub const MQRC_NO_MSG_UNDER_CURSOR: types::MQRC = types::MQRC(
        mqsys::MQRC_NO_MSG_UNDER_CURSOR,
    );
    pub const MQRC_NOT_AUTHORIZED: types::MQRC = types::MQRC(mqsys::MQRC_NOT_AUTHORIZED);
    pub const MQRC_NOT_OPEN_FOR_BROWSE: types::MQRC = types::MQRC(
        mqsys::MQRC_NOT_OPEN_FOR_BROWSE,
    );
    pub const MQRC_NOT_OPEN_FOR_INPUT: types::MQRC = types::MQRC(
        mqsys::MQRC_NOT_OPEN_FOR_INPUT,
    );
    pub const MQRC_NOT_OPEN_FOR_INQUIRE: types::MQRC = types::MQRC(
        mqsys::MQRC_NOT_OPEN_FOR_INQUIRE,
    );
    pub const MQRC_NOT_OPEN_FOR_OUTPUT: types::MQRC = types::MQRC(
        mqsys::MQRC_NOT_OPEN_FOR_OUTPUT,
    );
    pub const MQRC_NOT_OPEN_FOR_SET: types::MQRC = types::MQRC(
        mqsys::MQRC_NOT_OPEN_FOR_SET,
    );
    pub const MQRC_OBJECT_CHANGED: types::MQRC = types::MQRC(mqsys::MQRC_OBJECT_CHANGED);
    pub const MQRC_OBJECT_IN_USE: types::MQRC = types::MQRC(mqsys::MQRC_OBJECT_IN_USE);
    pub const MQRC_OBJECT_TYPE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_OBJECT_TYPE_ERROR,
    );
    pub const MQRC_OD_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_OD_ERROR);
    pub const MQRC_OPTION_NOT_VALID_FOR_TYPE: types::MQRC = types::MQRC(
        mqsys::MQRC_OPTION_NOT_VALID_FOR_TYPE,
    );
    pub const MQRC_OPTIONS_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_OPTIONS_ERROR);
    pub const MQRC_PERSISTENCE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_PERSISTENCE_ERROR,
    );
    pub const MQRC_PERSISTENT_NOT_ALLOWED: types::MQRC = types::MQRC(
        mqsys::MQRC_PERSISTENT_NOT_ALLOWED,
    );
    pub const MQRC_PRIORITY_EXCEEDS_MAXIMUM: types::MQRC = types::MQRC(
        mqsys::MQRC_PRIORITY_EXCEEDS_MAXIMUM,
    );
    pub const MQRC_PRIORITY_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_PRIORITY_ERROR);
    pub const MQRC_PUT_INHIBITED: types::MQRC = types::MQRC(mqsys::MQRC_PUT_INHIBITED);
    pub const MQRC_Q_DELETED: types::MQRC = types::MQRC(mqsys::MQRC_Q_DELETED);
    pub const MQRC_Q_FULL: types::MQRC = types::MQRC(mqsys::MQRC_Q_FULL);
    pub const MQRC_Q_NOT_EMPTY: types::MQRC = types::MQRC(mqsys::MQRC_Q_NOT_EMPTY);
    pub const MQRC_Q_SPACE_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_Q_SPACE_NOT_AVAILABLE,
    );
    pub const MQRC_Q_TYPE_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_Q_TYPE_ERROR);
    pub const MQRC_Q_MGR_NAME_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_Q_MGR_NAME_ERROR,
    );
    pub const MQRC_Q_MGR_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_Q_MGR_NOT_AVAILABLE,
    );
    pub const MQRC_REPORT_OPTIONS_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_REPORT_OPTIONS_ERROR,
    );
    pub const MQRC_SECOND_MARK_NOT_ALLOWED: types::MQRC = types::MQRC(
        mqsys::MQRC_SECOND_MARK_NOT_ALLOWED,
    );
    pub const MQRC_SECURITY_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_SECURITY_ERROR);
    pub const MQRC_TOKEN_TIMESTAMP_NOT_VALID: types::MQRC = types::MQRC(
        mqsys::MQRC_TOKEN_TIMESTAMP_NOT_VALID,
    );
    pub const MQRC_SELECTOR_COUNT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTOR_COUNT_ERROR,
    );
    pub const MQRC_SELECTOR_LIMIT_EXCEEDED: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTOR_LIMIT_EXCEEDED,
    );
    pub const MQRC_SELECTOR_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_SELECTOR_ERROR);
    pub const MQRC_SELECTOR_NOT_FOR_TYPE: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTOR_NOT_FOR_TYPE,
    );
    pub const MQRC_SIGNAL_OUTSTANDING: types::MQRC = types::MQRC(
        mqsys::MQRC_SIGNAL_OUTSTANDING,
    );
    pub const MQRC_SIGNAL_REQUEST_ACCEPTED: types::MQRC = types::MQRC(
        mqsys::MQRC_SIGNAL_REQUEST_ACCEPTED,
    );
    pub const MQRC_STORAGE_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_STORAGE_NOT_AVAILABLE,
    );
    pub const MQRC_SYNCPOINT_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_SYNCPOINT_NOT_AVAILABLE,
    );
    pub const MQRC_TRIGGER_CONTROL_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_TRIGGER_CONTROL_ERROR,
    );
    pub const MQRC_TRIGGER_DEPTH_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_TRIGGER_DEPTH_ERROR,
    );
    pub const MQRC_TRIGGER_MSG_PRIORITY_ERR: types::MQRC = types::MQRC(
        mqsys::MQRC_TRIGGER_MSG_PRIORITY_ERR,
    );
    pub const MQRC_TRIGGER_TYPE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_TRIGGER_TYPE_ERROR,
    );
    pub const MQRC_TRUNCATED_MSG_ACCEPTED: types::MQRC = types::MQRC(
        mqsys::MQRC_TRUNCATED_MSG_ACCEPTED,
    );
    pub const MQRC_TRUNCATED_MSG_FAILED: types::MQRC = types::MQRC(
        mqsys::MQRC_TRUNCATED_MSG_FAILED,
    );
    pub const MQRC_UNKNOWN_ALIAS_BASE_Q: types::MQRC = types::MQRC(
        mqsys::MQRC_UNKNOWN_ALIAS_BASE_Q,
    );
    pub const MQRC_UNKNOWN_OBJECT_NAME: types::MQRC = types::MQRC(
        mqsys::MQRC_UNKNOWN_OBJECT_NAME,
    );
    pub const MQRC_UNKNOWN_OBJECT_Q_MGR: types::MQRC = types::MQRC(
        mqsys::MQRC_UNKNOWN_OBJECT_Q_MGR,
    );
    pub const MQRC_UNKNOWN_REMOTE_Q_MGR: types::MQRC = types::MQRC(
        mqsys::MQRC_UNKNOWN_REMOTE_Q_MGR,
    );
    pub const MQRC_WAIT_INTERVAL_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_WAIT_INTERVAL_ERROR,
    );
    pub const MQRC_XMIT_Q_TYPE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_XMIT_Q_TYPE_ERROR,
    );
    pub const MQRC_XMIT_Q_USAGE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_XMIT_Q_USAGE_ERROR,
    );
    pub const MQRC_NOT_OPEN_FOR_PASS_ALL: types::MQRC = types::MQRC(
        mqsys::MQRC_NOT_OPEN_FOR_PASS_ALL,
    );
    pub const MQRC_NOT_OPEN_FOR_PASS_IDENT: types::MQRC = types::MQRC(
        mqsys::MQRC_NOT_OPEN_FOR_PASS_IDENT,
    );
    pub const MQRC_NOT_OPEN_FOR_SET_ALL: types::MQRC = types::MQRC(
        mqsys::MQRC_NOT_OPEN_FOR_SET_ALL,
    );
    pub const MQRC_NOT_OPEN_FOR_SET_IDENT: types::MQRC = types::MQRC(
        mqsys::MQRC_NOT_OPEN_FOR_SET_IDENT,
    );
    pub const MQRC_CONTEXT_HANDLE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CONTEXT_HANDLE_ERROR,
    );
    pub const MQRC_CONTEXT_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_CONTEXT_NOT_AVAILABLE,
    );
    pub const MQRC_SIGNAL1_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_SIGNAL1_ERROR);
    pub const MQRC_OBJECT_ALREADY_EXISTS: types::MQRC = types::MQRC(
        mqsys::MQRC_OBJECT_ALREADY_EXISTS,
    );
    pub const MQRC_OBJECT_DAMAGED: types::MQRC = types::MQRC(mqsys::MQRC_OBJECT_DAMAGED);
    pub const MQRC_RESOURCE_PROBLEM: types::MQRC = types::MQRC(
        mqsys::MQRC_RESOURCE_PROBLEM,
    );
    pub const MQRC_ANOTHER_Q_MGR_CONNECTED: types::MQRC = types::MQRC(
        mqsys::MQRC_ANOTHER_Q_MGR_CONNECTED,
    );
    pub const MQRC_UNKNOWN_REPORT_OPTION: types::MQRC = types::MQRC(
        mqsys::MQRC_UNKNOWN_REPORT_OPTION,
    );
    pub const MQRC_STORAGE_CLASS_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_STORAGE_CLASS_ERROR,
    );
    pub const MQRC_COD_NOT_VALID_FOR_XCF_Q: types::MQRC = types::MQRC(
        mqsys::MQRC_COD_NOT_VALID_FOR_XCF_Q,
    );
    pub const MQRC_XWAIT_CANCELED: types::MQRC = types::MQRC(mqsys::MQRC_XWAIT_CANCELED);
    pub const MQRC_XWAIT_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_XWAIT_ERROR);
    pub const MQRC_SUPPRESSED_BY_EXIT: types::MQRC = types::MQRC(
        mqsys::MQRC_SUPPRESSED_BY_EXIT,
    );
    pub const MQRC_FORMAT_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_FORMAT_ERROR);
    pub const MQRC_SOURCE_CCSID_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SOURCE_CCSID_ERROR,
    );
    pub const MQRC_SOURCE_INTEGER_ENC_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SOURCE_INTEGER_ENC_ERROR,
    );
    pub const MQRC_SOURCE_DECIMAL_ENC_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SOURCE_DECIMAL_ENC_ERROR,
    );
    pub const MQRC_SOURCE_FLOAT_ENC_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SOURCE_FLOAT_ENC_ERROR,
    );
    pub const MQRC_TARGET_CCSID_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_TARGET_CCSID_ERROR,
    );
    pub const MQRC_TARGET_INTEGER_ENC_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_TARGET_INTEGER_ENC_ERROR,
    );
    pub const MQRC_TARGET_DECIMAL_ENC_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_TARGET_DECIMAL_ENC_ERROR,
    );
    pub const MQRC_TARGET_FLOAT_ENC_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_TARGET_FLOAT_ENC_ERROR,
    );
    pub const MQRC_NOT_CONVERTED: types::MQRC = types::MQRC(mqsys::MQRC_NOT_CONVERTED);
    pub const MQRC_CONVERTED_MSG_TOO_BIG: types::MQRC = types::MQRC(
        mqsys::MQRC_CONVERTED_MSG_TOO_BIG,
    );
    pub const MQRC_NO_EXTERNAL_PARTICIPANTS: types::MQRC = types::MQRC(
        mqsys::MQRC_NO_EXTERNAL_PARTICIPANTS,
    );
    pub const MQRC_PARTICIPANT_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_PARTICIPANT_NOT_AVAILABLE,
    );
    pub const MQRC_OUTCOME_MIXED: types::MQRC = types::MQRC(mqsys::MQRC_OUTCOME_MIXED);
    pub const MQRC_OUTCOME_PENDING: types::MQRC = types::MQRC(
        mqsys::MQRC_OUTCOME_PENDING,
    );
    pub const MQRC_BRIDGE_STARTED: types::MQRC = types::MQRC(mqsys::MQRC_BRIDGE_STARTED);
    pub const MQRC_BRIDGE_STOPPED: types::MQRC = types::MQRC(mqsys::MQRC_BRIDGE_STOPPED);
    pub const MQRC_ADAPTER_STORAGE_SHORTAGE: types::MQRC = types::MQRC(
        mqsys::MQRC_ADAPTER_STORAGE_SHORTAGE,
    );
    pub const MQRC_UOW_IN_PROGRESS: types::MQRC = types::MQRC(
        mqsys::MQRC_UOW_IN_PROGRESS,
    );
    pub const MQRC_ADAPTER_CONN_LOAD_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ADAPTER_CONN_LOAD_ERROR,
    );
    pub const MQRC_ADAPTER_SERV_LOAD_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ADAPTER_SERV_LOAD_ERROR,
    );
    pub const MQRC_ADAPTER_DEFS_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ADAPTER_DEFS_ERROR,
    );
    pub const MQRC_ADAPTER_DEFS_LOAD_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ADAPTER_DEFS_LOAD_ERROR,
    );
    pub const MQRC_ADAPTER_CONV_LOAD_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ADAPTER_CONV_LOAD_ERROR,
    );
    pub const MQRC_BO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_BO_ERROR);
    pub const MQRC_DH_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_DH_ERROR);
    pub const MQRC_MULTIPLE_REASONS: types::MQRC = types::MQRC(
        mqsys::MQRC_MULTIPLE_REASONS,
    );
    pub const MQRC_OPEN_FAILED: types::MQRC = types::MQRC(mqsys::MQRC_OPEN_FAILED);
    pub const MQRC_ADAPTER_DISC_LOAD_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ADAPTER_DISC_LOAD_ERROR,
    );
    pub const MQRC_CNO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CNO_ERROR);
    pub const MQRC_CICS_WAIT_FAILED: types::MQRC = types::MQRC(
        mqsys::MQRC_CICS_WAIT_FAILED,
    );
    pub const MQRC_DLH_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_DLH_ERROR);
    pub const MQRC_HEADER_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_HEADER_ERROR);
    pub const MQRC_SOURCE_LENGTH_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SOURCE_LENGTH_ERROR,
    );
    pub const MQRC_TARGET_LENGTH_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_TARGET_LENGTH_ERROR,
    );
    pub const MQRC_SOURCE_BUFFER_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SOURCE_BUFFER_ERROR,
    );
    pub const MQRC_TARGET_BUFFER_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_TARGET_BUFFER_ERROR,
    );
    pub const MQRC_INCOMPLETE_TRANSACTION: types::MQRC = types::MQRC(
        mqsys::MQRC_INCOMPLETE_TRANSACTION,
    );
    pub const MQRC_IIH_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_IIH_ERROR);
    pub const MQRC_PCF_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_PCF_ERROR);
    pub const MQRC_DBCS_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_DBCS_ERROR);
    pub const MQRC_OBJECT_NAME_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_OBJECT_NAME_ERROR,
    );
    pub const MQRC_OBJECT_Q_MGR_NAME_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_OBJECT_Q_MGR_NAME_ERROR,
    );
    pub const MQRC_RECS_PRESENT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_RECS_PRESENT_ERROR,
    );
    pub const MQRC_OBJECT_RECORDS_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_OBJECT_RECORDS_ERROR,
    );
    pub const MQRC_RESPONSE_RECORDS_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_RESPONSE_RECORDS_ERROR,
    );
    pub const MQRC_ASID_MISMATCH: types::MQRC = types::MQRC(mqsys::MQRC_ASID_MISMATCH);
    pub const MQRC_PMO_RECORD_FLAGS_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_PMO_RECORD_FLAGS_ERROR,
    );
    pub const MQRC_PUT_MSG_RECORDS_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_PUT_MSG_RECORDS_ERROR,
    );
    pub const MQRC_CONN_ID_IN_USE: types::MQRC = types::MQRC(mqsys::MQRC_CONN_ID_IN_USE);
    pub const MQRC_Q_MGR_QUIESCING: types::MQRC = types::MQRC(
        mqsys::MQRC_Q_MGR_QUIESCING,
    );
    pub const MQRC_Q_MGR_STOPPING: types::MQRC = types::MQRC(mqsys::MQRC_Q_MGR_STOPPING);
    pub const MQRC_DUPLICATE_RECOV_COORD: types::MQRC = types::MQRC(
        mqsys::MQRC_DUPLICATE_RECOV_COORD,
    );
    pub const MQRC_PMO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_PMO_ERROR);
    pub const MQRC_API_EXIT_NOT_FOUND: types::MQRC = types::MQRC(
        mqsys::MQRC_API_EXIT_NOT_FOUND,
    );
    pub const MQRC_API_EXIT_LOAD_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_API_EXIT_LOAD_ERROR,
    );
    pub const MQRC_REMOTE_Q_NAME_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_REMOTE_Q_NAME_ERROR,
    );
    pub const MQRC_INCONSISTENT_PERSISTENCE: types::MQRC = types::MQRC(
        mqsys::MQRC_INCONSISTENT_PERSISTENCE,
    );
    pub const MQRC_GMO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_GMO_ERROR);
    pub const MQRC_CICS_BRIDGE_RESTRICTION: types::MQRC = types::MQRC(
        mqsys::MQRC_CICS_BRIDGE_RESTRICTION,
    );
    pub const MQRC_STOPPED_BY_CLUSTER_EXIT: types::MQRC = types::MQRC(
        mqsys::MQRC_STOPPED_BY_CLUSTER_EXIT,
    );
    pub const MQRC_CLUSTER_RESOLUTION_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CLUSTER_RESOLUTION_ERROR,
    );
    pub const MQRC_CONVERTED_STRING_TOO_BIG: types::MQRC = types::MQRC(
        mqsys::MQRC_CONVERTED_STRING_TOO_BIG,
    );
    pub const MQRC_TMC_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_TMC_ERROR);
    pub const MQRC_STORAGE_MEDIUM_FULL: types::MQRC = types::MQRC(
        mqsys::MQRC_STORAGE_MEDIUM_FULL,
    );
    pub const MQRC_PAGESET_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_PAGESET_ERROR);
    pub const MQRC_NAME_NOT_VALID_FOR_TYPE: types::MQRC = types::MQRC(
        mqsys::MQRC_NAME_NOT_VALID_FOR_TYPE,
    );
    pub const MQRC_UNEXPECTED_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_UNEXPECTED_ERROR,
    );
    pub const MQRC_UNKNOWN_XMIT_Q: types::MQRC = types::MQRC(mqsys::MQRC_UNKNOWN_XMIT_Q);
    pub const MQRC_UNKNOWN_DEF_XMIT_Q: types::MQRC = types::MQRC(
        mqsys::MQRC_UNKNOWN_DEF_XMIT_Q,
    );
    pub const MQRC_DEF_XMIT_Q_TYPE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_DEF_XMIT_Q_TYPE_ERROR,
    );
    pub const MQRC_DEF_XMIT_Q_USAGE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_DEF_XMIT_Q_USAGE_ERROR,
    );
    pub const MQRC_MSG_MARKED_BROWSE_CO_OP: types::MQRC = types::MQRC(
        mqsys::MQRC_MSG_MARKED_BROWSE_CO_OP,
    );
    pub const MQRC_NAME_IN_USE: types::MQRC = types::MQRC(mqsys::MQRC_NAME_IN_USE);
    pub const MQRC_CONNECTION_QUIESCING: types::MQRC = types::MQRC(
        mqsys::MQRC_CONNECTION_QUIESCING,
    );
    pub const MQRC_CONNECTION_STOPPING: types::MQRC = types::MQRC(
        mqsys::MQRC_CONNECTION_STOPPING,
    );
    pub const MQRC_ADAPTER_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_ADAPTER_NOT_AVAILABLE,
    );
    pub const MQRC_MSG_ID_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_MSG_ID_ERROR);
    pub const MQRC_CORREL_ID_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CORREL_ID_ERROR,
    );
    pub const MQRC_FILE_SYSTEM_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_FILE_SYSTEM_ERROR,
    );
    pub const MQRC_NO_MSG_LOCKED: types::MQRC = types::MQRC(mqsys::MQRC_NO_MSG_LOCKED);
    pub const MQRC_SOAP_DOTNET_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SOAP_DOTNET_ERROR,
    );
    pub const MQRC_SOAP_AXIS_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SOAP_AXIS_ERROR,
    );
    pub const MQRC_SOAP_URL_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_SOAP_URL_ERROR);
    pub const MQRC_FILE_NOT_AUDITED: types::MQRC = types::MQRC(
        mqsys::MQRC_FILE_NOT_AUDITED,
    );
    pub const MQRC_CONNECTION_NOT_AUTHORIZED: types::MQRC = types::MQRC(
        mqsys::MQRC_CONNECTION_NOT_AUTHORIZED,
    );
    pub const MQRC_MSG_TOO_BIG_FOR_CHANNEL: types::MQRC = types::MQRC(
        mqsys::MQRC_MSG_TOO_BIG_FOR_CHANNEL,
    );
    pub const MQRC_CALL_IN_PROGRESS: types::MQRC = types::MQRC(
        mqsys::MQRC_CALL_IN_PROGRESS,
    );
    pub const MQRC_RMH_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_RMH_ERROR);
    pub const MQRC_Q_MGR_ACTIVE: types::MQRC = types::MQRC(mqsys::MQRC_Q_MGR_ACTIVE);
    pub const MQRC_Q_MGR_NOT_ACTIVE: types::MQRC = types::MQRC(
        mqsys::MQRC_Q_MGR_NOT_ACTIVE,
    );
    pub const MQRC_Q_DEPTH_HIGH: types::MQRC = types::MQRC(mqsys::MQRC_Q_DEPTH_HIGH);
    pub const MQRC_Q_DEPTH_LOW: types::MQRC = types::MQRC(mqsys::MQRC_Q_DEPTH_LOW);
    pub const MQRC_Q_SERVICE_INTERVAL_HIGH: types::MQRC = types::MQRC(
        mqsys::MQRC_Q_SERVICE_INTERVAL_HIGH,
    );
    pub const MQRC_Q_SERVICE_INTERVAL_OK: types::MQRC = types::MQRC(
        mqsys::MQRC_Q_SERVICE_INTERVAL_OK,
    );
    pub const MQRC_RFH_HEADER_FIELD_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_RFH_HEADER_FIELD_ERROR,
    );
    pub const MQRC_RAS_PROPERTY_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_RAS_PROPERTY_ERROR,
    );
    pub const MQRC_UNIT_OF_WORK_NOT_STARTED: types::MQRC = types::MQRC(
        mqsys::MQRC_UNIT_OF_WORK_NOT_STARTED,
    );
    pub const MQRC_CHANNEL_AUTO_DEF_OK: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_AUTO_DEF_OK,
    );
    pub const MQRC_CHANNEL_AUTO_DEF_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_AUTO_DEF_ERROR,
    );
    pub const MQRC_CFH_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CFH_ERROR);
    pub const MQRC_CFIL_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CFIL_ERROR);
    pub const MQRC_CFIN_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CFIN_ERROR);
    pub const MQRC_CFSL_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CFSL_ERROR);
    pub const MQRC_CFST_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CFST_ERROR);
    pub const MQRC_INCOMPLETE_GROUP: types::MQRC = types::MQRC(
        mqsys::MQRC_INCOMPLETE_GROUP,
    );
    pub const MQRC_INCOMPLETE_MSG: types::MQRC = types::MQRC(mqsys::MQRC_INCOMPLETE_MSG);
    pub const MQRC_INCONSISTENT_CCSIDS: types::MQRC = types::MQRC(
        mqsys::MQRC_INCONSISTENT_CCSIDS,
    );
    pub const MQRC_INCONSISTENT_ENCODINGS: types::MQRC = types::MQRC(
        mqsys::MQRC_INCONSISTENT_ENCODINGS,
    );
    pub const MQRC_INCONSISTENT_UOW: types::MQRC = types::MQRC(
        mqsys::MQRC_INCONSISTENT_UOW,
    );
    pub const MQRC_INVALID_MSG_UNDER_CURSOR: types::MQRC = types::MQRC(
        mqsys::MQRC_INVALID_MSG_UNDER_CURSOR,
    );
    pub const MQRC_MATCH_OPTIONS_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_MATCH_OPTIONS_ERROR,
    );
    pub const MQRC_MDE_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_MDE_ERROR);
    pub const MQRC_MSG_FLAGS_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_MSG_FLAGS_ERROR,
    );
    pub const MQRC_MSG_SEQ_NUMBER_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_MSG_SEQ_NUMBER_ERROR,
    );
    pub const MQRC_OFFSET_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_OFFSET_ERROR);
    pub const MQRC_ORIGINAL_LENGTH_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ORIGINAL_LENGTH_ERROR,
    );
    pub const MQRC_SEGMENT_LENGTH_ZERO: types::MQRC = types::MQRC(
        mqsys::MQRC_SEGMENT_LENGTH_ZERO,
    );
    pub const MQRC_UOW_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_UOW_NOT_AVAILABLE,
    );
    pub const MQRC_WRONG_GMO_VERSION: types::MQRC = types::MQRC(
        mqsys::MQRC_WRONG_GMO_VERSION,
    );
    pub const MQRC_WRONG_MD_VERSION: types::MQRC = types::MQRC(
        mqsys::MQRC_WRONG_MD_VERSION,
    );
    pub const MQRC_GROUP_ID_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_GROUP_ID_ERROR);
    pub const MQRC_INCONSISTENT_BROWSE: types::MQRC = types::MQRC(
        mqsys::MQRC_INCONSISTENT_BROWSE,
    );
    pub const MQRC_XQH_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_XQH_ERROR);
    pub const MQRC_SRC_ENV_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_SRC_ENV_ERROR);
    pub const MQRC_SRC_NAME_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_SRC_NAME_ERROR);
    pub const MQRC_DEST_ENV_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_DEST_ENV_ERROR);
    pub const MQRC_DEST_NAME_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_DEST_NAME_ERROR,
    );
    pub const MQRC_TM_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_TM_ERROR);
    pub const MQRC_CLUSTER_EXIT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CLUSTER_EXIT_ERROR,
    );
    pub const MQRC_CLUSTER_EXIT_LOAD_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CLUSTER_EXIT_LOAD_ERROR,
    );
    pub const MQRC_CLUSTER_PUT_INHIBITED: types::MQRC = types::MQRC(
        mqsys::MQRC_CLUSTER_PUT_INHIBITED,
    );
    pub const MQRC_CLUSTER_RESOURCE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CLUSTER_RESOURCE_ERROR,
    );
    pub const MQRC_NO_DESTINATIONS_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_NO_DESTINATIONS_AVAILABLE,
    );
    pub const MQRC_CONN_TAG_IN_USE: types::MQRC = types::MQRC(
        mqsys::MQRC_CONN_TAG_IN_USE,
    );
    pub const MQRC_PARTIALLY_CONVERTED: types::MQRC = types::MQRC(
        mqsys::MQRC_PARTIALLY_CONVERTED,
    );
    pub const MQRC_CONNECTION_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CONNECTION_ERROR,
    );
    pub const MQRC_OPTION_ENVIRONMENT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_OPTION_ENVIRONMENT_ERROR,
    );
    pub const MQRC_CD_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CD_ERROR);
    pub const MQRC_CLIENT_CONN_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CLIENT_CONN_ERROR,
    );
    pub const MQRC_CHANNEL_STOPPED_BY_USER: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_STOPPED_BY_USER,
    );
    pub const MQRC_HCONFIG_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_HCONFIG_ERROR);
    pub const MQRC_FUNCTION_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_FUNCTION_ERROR);
    pub const MQRC_CHANNEL_STARTED: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_STARTED,
    );
    pub const MQRC_CHANNEL_STOPPED: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_STOPPED,
    );
    pub const MQRC_CHANNEL_CONV_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_CONV_ERROR,
    );
    pub const MQRC_SERVICE_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_SERVICE_NOT_AVAILABLE,
    );
    pub const MQRC_INITIALIZATION_FAILED: types::MQRC = types::MQRC(
        mqsys::MQRC_INITIALIZATION_FAILED,
    );
    pub const MQRC_TERMINATION_FAILED: types::MQRC = types::MQRC(
        mqsys::MQRC_TERMINATION_FAILED,
    );
    pub const MQRC_UNKNOWN_Q_NAME: types::MQRC = types::MQRC(mqsys::MQRC_UNKNOWN_Q_NAME);
    pub const MQRC_SERVICE_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_SERVICE_ERROR);
    pub const MQRC_Q_ALREADY_EXISTS: types::MQRC = types::MQRC(
        mqsys::MQRC_Q_ALREADY_EXISTS,
    );
    pub const MQRC_USER_ID_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_USER_ID_NOT_AVAILABLE,
    );
    pub const MQRC_UNKNOWN_ENTITY: types::MQRC = types::MQRC(mqsys::MQRC_UNKNOWN_ENTITY);
    pub const MQRC_UNKNOWN_AUTH_ENTITY: types::MQRC = types::MQRC(
        mqsys::MQRC_UNKNOWN_AUTH_ENTITY,
    );
    pub const MQRC_UNKNOWN_REF_OBJECT: types::MQRC = types::MQRC(
        mqsys::MQRC_UNKNOWN_REF_OBJECT,
    );
    pub const MQRC_CHANNEL_ACTIVATED: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_ACTIVATED,
    );
    pub const MQRC_CHANNEL_NOT_ACTIVATED: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_NOT_ACTIVATED,
    );
    pub const MQRC_UOW_CANCELED: types::MQRC = types::MQRC(mqsys::MQRC_UOW_CANCELED);
    pub const MQRC_FUNCTION_NOT_SUPPORTED: types::MQRC = types::MQRC(
        mqsys::MQRC_FUNCTION_NOT_SUPPORTED,
    );
    pub const MQRC_SELECTOR_TYPE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTOR_TYPE_ERROR,
    );
    pub const MQRC_COMMAND_TYPE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_COMMAND_TYPE_ERROR,
    );
    pub const MQRC_MULTIPLE_INSTANCE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_MULTIPLE_INSTANCE_ERROR,
    );
    pub const MQRC_SYSTEM_ITEM_NOT_ALTERABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_SYSTEM_ITEM_NOT_ALTERABLE,
    );
    pub const MQRC_BAG_CONVERSION_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_BAG_CONVERSION_ERROR,
    );
    pub const MQRC_SELECTOR_OUT_OF_RANGE: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTOR_OUT_OF_RANGE,
    );
    pub const MQRC_SELECTOR_NOT_UNIQUE: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTOR_NOT_UNIQUE,
    );
    pub const MQRC_INDEX_NOT_PRESENT: types::MQRC = types::MQRC(
        mqsys::MQRC_INDEX_NOT_PRESENT,
    );
    pub const MQRC_STRING_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_STRING_ERROR);
    pub const MQRC_ENCODING_NOT_SUPPORTED: types::MQRC = types::MQRC(
        mqsys::MQRC_ENCODING_NOT_SUPPORTED,
    );
    pub const MQRC_SELECTOR_NOT_PRESENT: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTOR_NOT_PRESENT,
    );
    pub const MQRC_OUT_SELECTOR_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_OUT_SELECTOR_ERROR,
    );
    pub const MQRC_STRING_TRUNCATED: types::MQRC = types::MQRC(
        mqsys::MQRC_STRING_TRUNCATED,
    );
    pub const MQRC_SELECTOR_WRONG_TYPE: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTOR_WRONG_TYPE,
    );
    pub const MQRC_INCONSISTENT_ITEM_TYPE: types::MQRC = types::MQRC(
        mqsys::MQRC_INCONSISTENT_ITEM_TYPE,
    );
    pub const MQRC_INDEX_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_INDEX_ERROR);
    pub const MQRC_SYSTEM_BAG_NOT_ALTERABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_SYSTEM_BAG_NOT_ALTERABLE,
    );
    pub const MQRC_ITEM_COUNT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ITEM_COUNT_ERROR,
    );
    pub const MQRC_FORMAT_NOT_SUPPORTED: types::MQRC = types::MQRC(
        mqsys::MQRC_FORMAT_NOT_SUPPORTED,
    );
    pub const MQRC_SELECTOR_NOT_SUPPORTED: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTOR_NOT_SUPPORTED,
    );
    pub const MQRC_ITEM_VALUE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ITEM_VALUE_ERROR,
    );
    pub const MQRC_HBAG_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_HBAG_ERROR);
    pub const MQRC_PARAMETER_MISSING: types::MQRC = types::MQRC(
        mqsys::MQRC_PARAMETER_MISSING,
    );
    pub const MQRC_CMD_SERVER_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_CMD_SERVER_NOT_AVAILABLE,
    );
    pub const MQRC_STRING_LENGTH_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_STRING_LENGTH_ERROR,
    );
    pub const MQRC_INQUIRY_COMMAND_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_INQUIRY_COMMAND_ERROR,
    );
    pub const MQRC_NESTED_BAG_NOT_SUPPORTED: types::MQRC = types::MQRC(
        mqsys::MQRC_NESTED_BAG_NOT_SUPPORTED,
    );
    pub const MQRC_BAG_WRONG_TYPE: types::MQRC = types::MQRC(mqsys::MQRC_BAG_WRONG_TYPE);
    pub const MQRC_ITEM_TYPE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ITEM_TYPE_ERROR,
    );
    pub const MQRC_SYSTEM_BAG_NOT_DELETABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_SYSTEM_BAG_NOT_DELETABLE,
    );
    pub const MQRC_SYSTEM_ITEM_NOT_DELETABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_SYSTEM_ITEM_NOT_DELETABLE,
    );
    pub const MQRC_CODED_CHAR_SET_ID_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CODED_CHAR_SET_ID_ERROR,
    );
    pub const MQRC_MSG_TOKEN_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_MSG_TOKEN_ERROR,
    );
    pub const MQRC_MISSING_WIH: types::MQRC = types::MQRC(mqsys::MQRC_MISSING_WIH);
    pub const MQRC_WIH_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_WIH_ERROR);
    pub const MQRC_RFH_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_RFH_ERROR);
    pub const MQRC_RFH_STRING_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_RFH_STRING_ERROR,
    );
    pub const MQRC_RFH_COMMAND_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_RFH_COMMAND_ERROR,
    );
    pub const MQRC_RFH_PARM_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_RFH_PARM_ERROR);
    pub const MQRC_RFH_DUPLICATE_PARM: types::MQRC = types::MQRC(
        mqsys::MQRC_RFH_DUPLICATE_PARM,
    );
    pub const MQRC_RFH_PARM_MISSING: types::MQRC = types::MQRC(
        mqsys::MQRC_RFH_PARM_MISSING,
    );
    pub const MQRC_CHAR_CONVERSION_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CHAR_CONVERSION_ERROR,
    );
    pub const MQRC_UCS2_CONVERSION_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_UCS2_CONVERSION_ERROR,
    );
    pub const MQRC_DB2_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_DB2_NOT_AVAILABLE,
    );
    pub const MQRC_OBJECT_NOT_UNIQUE: types::MQRC = types::MQRC(
        mqsys::MQRC_OBJECT_NOT_UNIQUE,
    );
    pub const MQRC_CONN_TAG_NOT_RELEASED: types::MQRC = types::MQRC(
        mqsys::MQRC_CONN_TAG_NOT_RELEASED,
    );
    pub const MQRC_CF_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_CF_NOT_AVAILABLE,
    );
    pub const MQRC_CF_STRUC_IN_USE: types::MQRC = types::MQRC(
        mqsys::MQRC_CF_STRUC_IN_USE,
    );
    pub const MQRC_CF_STRUC_LIST_HDR_IN_USE: types::MQRC = types::MQRC(
        mqsys::MQRC_CF_STRUC_LIST_HDR_IN_USE,
    );
    pub const MQRC_CF_STRUC_AUTH_FAILED: types::MQRC = types::MQRC(
        mqsys::MQRC_CF_STRUC_AUTH_FAILED,
    );
    pub const MQRC_CF_STRUC_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CF_STRUC_ERROR);
    pub const MQRC_CONN_TAG_NOT_USABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_CONN_TAG_NOT_USABLE,
    );
    pub const MQRC_GLOBAL_UOW_CONFLICT: types::MQRC = types::MQRC(
        mqsys::MQRC_GLOBAL_UOW_CONFLICT,
    );
    pub const MQRC_LOCAL_UOW_CONFLICT: types::MQRC = types::MQRC(
        mqsys::MQRC_LOCAL_UOW_CONFLICT,
    );
    pub const MQRC_HANDLE_IN_USE_FOR_UOW: types::MQRC = types::MQRC(
        mqsys::MQRC_HANDLE_IN_USE_FOR_UOW,
    );
    pub const MQRC_UOW_ENLISTMENT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_UOW_ENLISTMENT_ERROR,
    );
    pub const MQRC_UOW_MIX_NOT_SUPPORTED: types::MQRC = types::MQRC(
        mqsys::MQRC_UOW_MIX_NOT_SUPPORTED,
    );
    pub const MQRC_WXP_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_WXP_ERROR);
    pub const MQRC_CURRENT_RECORD_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CURRENT_RECORD_ERROR,
    );
    pub const MQRC_NEXT_OFFSET_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_NEXT_OFFSET_ERROR,
    );
    pub const MQRC_NO_RECORD_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_NO_RECORD_AVAILABLE,
    );
    pub const MQRC_OBJECT_LEVEL_INCOMPATIBLE: types::MQRC = types::MQRC(
        mqsys::MQRC_OBJECT_LEVEL_INCOMPATIBLE,
    );
    pub const MQRC_NEXT_RECORD_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_NEXT_RECORD_ERROR,
    );
    pub const MQRC_BACKOUT_THRESHOLD_REACHED: types::MQRC = types::MQRC(
        mqsys::MQRC_BACKOUT_THRESHOLD_REACHED,
    );
    pub const MQRC_MSG_NOT_MATCHED: types::MQRC = types::MQRC(
        mqsys::MQRC_MSG_NOT_MATCHED,
    );
    pub const MQRC_JMS_FORMAT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_JMS_FORMAT_ERROR,
    );
    pub const MQRC_SEGMENTS_NOT_SUPPORTED: types::MQRC = types::MQRC(
        mqsys::MQRC_SEGMENTS_NOT_SUPPORTED,
    );
    pub const MQRC_WRONG_CF_LEVEL: types::MQRC = types::MQRC(mqsys::MQRC_WRONG_CF_LEVEL);
    pub const MQRC_CONFIG_CREATE_OBJECT: types::MQRC = types::MQRC(
        mqsys::MQRC_CONFIG_CREATE_OBJECT,
    );
    pub const MQRC_CONFIG_CHANGE_OBJECT: types::MQRC = types::MQRC(
        mqsys::MQRC_CONFIG_CHANGE_OBJECT,
    );
    pub const MQRC_CONFIG_DELETE_OBJECT: types::MQRC = types::MQRC(
        mqsys::MQRC_CONFIG_DELETE_OBJECT,
    );
    pub const MQRC_CONFIG_REFRESH_OBJECT: types::MQRC = types::MQRC(
        mqsys::MQRC_CONFIG_REFRESH_OBJECT,
    );
    pub const MQRC_CHANNEL_SSL_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_SSL_ERROR,
    );
    pub const MQRC_PARTICIPANT_NOT_DEFINED: types::MQRC = types::MQRC(
        mqsys::MQRC_PARTICIPANT_NOT_DEFINED,
    );
    pub const MQRC_CF_STRUC_FAILED: types::MQRC = types::MQRC(
        mqsys::MQRC_CF_STRUC_FAILED,
    );
    pub const MQRC_API_EXIT_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_API_EXIT_ERROR);
    pub const MQRC_API_EXIT_INIT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_API_EXIT_INIT_ERROR,
    );
    pub const MQRC_API_EXIT_TERM_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_API_EXIT_TERM_ERROR,
    );
    pub const MQRC_EXIT_REASON_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_EXIT_REASON_ERROR,
    );
    pub const MQRC_RESERVED_VALUE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_RESERVED_VALUE_ERROR,
    );
    pub const MQRC_NO_DATA_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_NO_DATA_AVAILABLE,
    );
    pub const MQRC_SCO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_SCO_ERROR);
    pub const MQRC_KEY_REPOSITORY_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_KEY_REPOSITORY_ERROR,
    );
    pub const MQRC_CRYPTO_HARDWARE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CRYPTO_HARDWARE_ERROR,
    );
    pub const MQRC_AUTH_INFO_REC_COUNT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_AUTH_INFO_REC_COUNT_ERROR,
    );
    pub const MQRC_AUTH_INFO_REC_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_AUTH_INFO_REC_ERROR,
    );
    pub const MQRC_AIR_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_AIR_ERROR);
    pub const MQRC_AUTH_INFO_TYPE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_AUTH_INFO_TYPE_ERROR,
    );
    pub const MQRC_AUTH_INFO_CONN_NAME_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_AUTH_INFO_CONN_NAME_ERROR,
    );
    pub const MQRC_LDAP_USER_NAME_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_LDAP_USER_NAME_ERROR,
    );
    pub const MQRC_LDAP_USER_NAME_LENGTH_ERR: types::MQRC = types::MQRC(
        mqsys::MQRC_LDAP_USER_NAME_LENGTH_ERR,
    );
    pub const MQRC_LDAP_PASSWORD_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_LDAP_PASSWORD_ERROR,
    );
    pub const MQRC_SSL_ALREADY_INITIALIZED: types::MQRC = types::MQRC(
        mqsys::MQRC_SSL_ALREADY_INITIALIZED,
    );
    pub const MQRC_SSL_CONFIG_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SSL_CONFIG_ERROR,
    );
    pub const MQRC_SSL_INITIALIZATION_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SSL_INITIALIZATION_ERROR,
    );
    pub const MQRC_Q_INDEX_TYPE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_Q_INDEX_TYPE_ERROR,
    );
    pub const MQRC_CFBS_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CFBS_ERROR);
    pub const MQRC_SSL_NOT_ALLOWED: types::MQRC = types::MQRC(
        mqsys::MQRC_SSL_NOT_ALLOWED,
    );
    pub const MQRC_JSSE_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_JSSE_ERROR);
    pub const MQRC_SSL_PEER_NAME_MISMATCH: types::MQRC = types::MQRC(
        mqsys::MQRC_SSL_PEER_NAME_MISMATCH,
    );
    pub const MQRC_SSL_PEER_NAME_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SSL_PEER_NAME_ERROR,
    );
    pub const MQRC_UNSUPPORTED_CIPHER_SUITE: types::MQRC = types::MQRC(
        mqsys::MQRC_UNSUPPORTED_CIPHER_SUITE,
    );
    pub const MQRC_SSL_CERTIFICATE_REVOKED: types::MQRC = types::MQRC(
        mqsys::MQRC_SSL_CERTIFICATE_REVOKED,
    );
    pub const MQRC_SSL_CERT_STORE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SSL_CERT_STORE_ERROR,
    );
    pub const MQRC_CLIENT_EXIT_LOAD_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CLIENT_EXIT_LOAD_ERROR,
    );
    pub const MQRC_CLIENT_EXIT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CLIENT_EXIT_ERROR,
    );
    pub const MQRC_UOW_COMMITTED: types::MQRC = types::MQRC(mqsys::MQRC_UOW_COMMITTED);
    pub const MQRC_SSL_KEY_RESET_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SSL_KEY_RESET_ERROR,
    );
    pub const MQRC_UNKNOWN_COMPONENT_NAME: types::MQRC = types::MQRC(
        mqsys::MQRC_UNKNOWN_COMPONENT_NAME,
    );
    pub const MQRC_LOGGER_STATUS: types::MQRC = types::MQRC(mqsys::MQRC_LOGGER_STATUS);
    pub const MQRC_COMMAND_MQSC: types::MQRC = types::MQRC(mqsys::MQRC_COMMAND_MQSC);
    pub const MQRC_COMMAND_PCF: types::MQRC = types::MQRC(mqsys::MQRC_COMMAND_PCF);
    pub const MQRC_CFIF_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CFIF_ERROR);
    pub const MQRC_CFSF_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CFSF_ERROR);
    pub const MQRC_CFGR_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CFGR_ERROR);
    pub const MQRC_MSG_NOT_ALLOWED_IN_GROUP: types::MQRC = types::MQRC(
        mqsys::MQRC_MSG_NOT_ALLOWED_IN_GROUP,
    );
    pub const MQRC_FILTER_OPERATOR_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_FILTER_OPERATOR_ERROR,
    );
    pub const MQRC_NESTED_SELECTOR_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_NESTED_SELECTOR_ERROR,
    );
    pub const MQRC_EPH_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_EPH_ERROR);
    pub const MQRC_RFH_FORMAT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_RFH_FORMAT_ERROR,
    );
    pub const MQRC_CFBF_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CFBF_ERROR);
    pub const MQRC_CLIENT_CHANNEL_CONFLICT: types::MQRC = types::MQRC(
        mqsys::MQRC_CLIENT_CHANNEL_CONFLICT,
    );
    pub const MQRC_SD_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_SD_ERROR);
    pub const MQRC_TOPIC_STRING_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_TOPIC_STRING_ERROR,
    );
    pub const MQRC_STS_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_STS_ERROR);
    pub const MQRC_NO_SUBSCRIPTION: types::MQRC = types::MQRC(
        mqsys::MQRC_NO_SUBSCRIPTION,
    );
    pub const MQRC_SUBSCRIPTION_IN_USE: types::MQRC = types::MQRC(
        mqsys::MQRC_SUBSCRIPTION_IN_USE,
    );
    pub const MQRC_STAT_TYPE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_STAT_TYPE_ERROR,
    );
    pub const MQRC_SUB_USER_DATA_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SUB_USER_DATA_ERROR,
    );
    pub const MQRC_SUB_ALREADY_EXISTS: types::MQRC = types::MQRC(
        mqsys::MQRC_SUB_ALREADY_EXISTS,
    );
    pub const MQRC_IDENTITY_MISMATCH: types::MQRC = types::MQRC(
        mqsys::MQRC_IDENTITY_MISMATCH,
    );
    pub const MQRC_ALTER_SUB_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ALTER_SUB_ERROR,
    );
    pub const MQRC_DURABILITY_NOT_ALLOWED: types::MQRC = types::MQRC(
        mqsys::MQRC_DURABILITY_NOT_ALLOWED,
    );
    pub const MQRC_NO_RETAINED_MSG: types::MQRC = types::MQRC(
        mqsys::MQRC_NO_RETAINED_MSG,
    );
    pub const MQRC_SRO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_SRO_ERROR);
    pub const MQRC_SUB_NAME_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_SUB_NAME_ERROR);
    pub const MQRC_OBJECT_STRING_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_OBJECT_STRING_ERROR,
    );
    pub const MQRC_PROPERTY_NAME_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_PROPERTY_NAME_ERROR,
    );
    pub const MQRC_SEGMENTATION_NOT_ALLOWED: types::MQRC = types::MQRC(
        mqsys::MQRC_SEGMENTATION_NOT_ALLOWED,
    );
    pub const MQRC_CBD_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CBD_ERROR);
    pub const MQRC_CTLO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CTLO_ERROR);
    pub const MQRC_NO_CALLBACKS_ACTIVE: types::MQRC = types::MQRC(
        mqsys::MQRC_NO_CALLBACKS_ACTIVE,
    );
    pub const MQRC_CALLBACK_NOT_REGISTERED: types::MQRC = types::MQRC(
        mqsys::MQRC_CALLBACK_NOT_REGISTERED,
    );
    pub const MQRC_OPTIONS_CHANGED: types::MQRC = types::MQRC(
        mqsys::MQRC_OPTIONS_CHANGED,
    );
    pub const MQRC_READ_AHEAD_MSGS: types::MQRC = types::MQRC(
        mqsys::MQRC_READ_AHEAD_MSGS,
    );
    pub const MQRC_SELECTOR_SYNTAX_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTOR_SYNTAX_ERROR,
    );
    pub const MQRC_HMSG_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_HMSG_ERROR);
    pub const MQRC_CMHO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CMHO_ERROR);
    pub const MQRC_DMHO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_DMHO_ERROR);
    pub const MQRC_SMPO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_SMPO_ERROR);
    pub const MQRC_IMPO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_IMPO_ERROR);
    pub const MQRC_PROPERTY_NAME_TOO_BIG: types::MQRC = types::MQRC(
        mqsys::MQRC_PROPERTY_NAME_TOO_BIG,
    );
    pub const MQRC_PROP_VALUE_NOT_CONVERTED: types::MQRC = types::MQRC(
        mqsys::MQRC_PROP_VALUE_NOT_CONVERTED,
    );
    pub const MQRC_PROP_TYPE_NOT_SUPPORTED: types::MQRC = types::MQRC(
        mqsys::MQRC_PROP_TYPE_NOT_SUPPORTED,
    );
    pub const MQRC_PROPERTY_VALUE_TOO_BIG: types::MQRC = types::MQRC(
        mqsys::MQRC_PROPERTY_VALUE_TOO_BIG,
    );
    pub const MQRC_PROP_CONV_NOT_SUPPORTED: types::MQRC = types::MQRC(
        mqsys::MQRC_PROP_CONV_NOT_SUPPORTED,
    );
    pub const MQRC_PROPERTY_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_PROPERTY_NOT_AVAILABLE,
    );
    pub const MQRC_PROP_NUMBER_FORMAT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_PROP_NUMBER_FORMAT_ERROR,
    );
    pub const MQRC_PROPERTY_TYPE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_PROPERTY_TYPE_ERROR,
    );
    pub const MQRC_PROPERTIES_TOO_BIG: types::MQRC = types::MQRC(
        mqsys::MQRC_PROPERTIES_TOO_BIG,
    );
    pub const MQRC_PUT_NOT_RETAINED: types::MQRC = types::MQRC(
        mqsys::MQRC_PUT_NOT_RETAINED,
    );
    pub const MQRC_ALIAS_TARGTYPE_CHANGED: types::MQRC = types::MQRC(
        mqsys::MQRC_ALIAS_TARGTYPE_CHANGED,
    );
    pub const MQRC_DMPO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_DMPO_ERROR);
    pub const MQRC_PD_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_PD_ERROR);
    pub const MQRC_CALLBACK_TYPE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CALLBACK_TYPE_ERROR,
    );
    pub const MQRC_CBD_OPTIONS_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CBD_OPTIONS_ERROR,
    );
    pub const MQRC_MAX_MSG_LENGTH_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_MAX_MSG_LENGTH_ERROR,
    );
    pub const MQRC_CALLBACK_ROUTINE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CALLBACK_ROUTINE_ERROR,
    );
    pub const MQRC_CALLBACK_LINK_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CALLBACK_LINK_ERROR,
    );
    pub const MQRC_OPERATION_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_OPERATION_ERROR,
    );
    pub const MQRC_BMHO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_BMHO_ERROR);
    pub const MQRC_UNSUPPORTED_PROPERTY: types::MQRC = types::MQRC(
        mqsys::MQRC_UNSUPPORTED_PROPERTY,
    );
    pub const MQRC_MSG_LENGTH_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_MSG_LENGTH_ERROR,
    );
    pub const MQRC_PROP_NAME_NOT_CONVERTED: types::MQRC = types::MQRC(
        mqsys::MQRC_PROP_NAME_NOT_CONVERTED,
    );
    pub const MQRC_GET_ENABLED: types::MQRC = types::MQRC(mqsys::MQRC_GET_ENABLED);
    pub const MQRC_MODULE_NOT_FOUND: types::MQRC = types::MQRC(
        mqsys::MQRC_MODULE_NOT_FOUND,
    );
    pub const MQRC_MODULE_INVALID: types::MQRC = types::MQRC(mqsys::MQRC_MODULE_INVALID);
    pub const MQRC_MODULE_ENTRY_NOT_FOUND: types::MQRC = types::MQRC(
        mqsys::MQRC_MODULE_ENTRY_NOT_FOUND,
    );
    pub const MQRC_MIXED_CONTENT_NOT_ALLOWED: types::MQRC = types::MQRC(
        mqsys::MQRC_MIXED_CONTENT_NOT_ALLOWED,
    );
    pub const MQRC_MSG_HANDLE_IN_USE: types::MQRC = types::MQRC(
        mqsys::MQRC_MSG_HANDLE_IN_USE,
    );
    pub const MQRC_HCONN_ASYNC_ACTIVE: types::MQRC = types::MQRC(
        mqsys::MQRC_HCONN_ASYNC_ACTIVE,
    );
    pub const MQRC_MHBO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_MHBO_ERROR);
    pub const MQRC_PUBLICATION_FAILURE: types::MQRC = types::MQRC(
        mqsys::MQRC_PUBLICATION_FAILURE,
    );
    pub const MQRC_SUB_INHIBITED: types::MQRC = types::MQRC(mqsys::MQRC_SUB_INHIBITED);
    pub const MQRC_SELECTOR_ALWAYS_FALSE: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTOR_ALWAYS_FALSE,
    );
    pub const MQRC_XEPO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_XEPO_ERROR);
    pub const MQRC_DURABILITY_NOT_ALTERABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_DURABILITY_NOT_ALTERABLE,
    );
    pub const MQRC_TOPIC_NOT_ALTERABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_TOPIC_NOT_ALTERABLE,
    );
    pub const MQRC_SUBLEVEL_NOT_ALTERABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_SUBLEVEL_NOT_ALTERABLE,
    );
    pub const MQRC_PROPERTY_NAME_LENGTH_ERR: types::MQRC = types::MQRC(
        mqsys::MQRC_PROPERTY_NAME_LENGTH_ERR,
    );
    pub const MQRC_DUPLICATE_GROUP_SUB: types::MQRC = types::MQRC(
        mqsys::MQRC_DUPLICATE_GROUP_SUB,
    );
    pub const MQRC_GROUPING_NOT_ALTERABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_GROUPING_NOT_ALTERABLE,
    );
    pub const MQRC_SELECTOR_INVALID_FOR_TYPE: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTOR_INVALID_FOR_TYPE,
    );
    pub const MQRC_HOBJ_QUIESCED: types::MQRC = types::MQRC(mqsys::MQRC_HOBJ_QUIESCED);
    pub const MQRC_HOBJ_QUIESCED_NO_MSGS: types::MQRC = types::MQRC(
        mqsys::MQRC_HOBJ_QUIESCED_NO_MSGS,
    );
    pub const MQRC_SELECTION_STRING_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTION_STRING_ERROR,
    );
    pub const MQRC_RES_OBJECT_STRING_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_RES_OBJECT_STRING_ERROR,
    );
    pub const MQRC_CONNECTION_SUSPENDED: types::MQRC = types::MQRC(
        mqsys::MQRC_CONNECTION_SUSPENDED,
    );
    pub const MQRC_INVALID_DESTINATION: types::MQRC = types::MQRC(
        mqsys::MQRC_INVALID_DESTINATION,
    );
    pub const MQRC_INVALID_SUBSCRIPTION: types::MQRC = types::MQRC(
        mqsys::MQRC_INVALID_SUBSCRIPTION,
    );
    pub const MQRC_SELECTOR_NOT_ALTERABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTOR_NOT_ALTERABLE,
    );
    pub const MQRC_RETAINED_MSG_Q_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_RETAINED_MSG_Q_ERROR,
    );
    pub const MQRC_RETAINED_NOT_DELIVERED: types::MQRC = types::MQRC(
        mqsys::MQRC_RETAINED_NOT_DELIVERED,
    );
    pub const MQRC_RFH_RESTRICTED_FORMAT_ERR: types::MQRC = types::MQRC(
        mqsys::MQRC_RFH_RESTRICTED_FORMAT_ERR,
    );
    pub const MQRC_CONNECTION_STOPPED: types::MQRC = types::MQRC(
        mqsys::MQRC_CONNECTION_STOPPED,
    );
    pub const MQRC_ASYNC_UOW_CONFLICT: types::MQRC = types::MQRC(
        mqsys::MQRC_ASYNC_UOW_CONFLICT,
    );
    pub const MQRC_ASYNC_XA_CONFLICT: types::MQRC = types::MQRC(
        mqsys::MQRC_ASYNC_XA_CONFLICT,
    );
    pub const MQRC_PUBSUB_INHIBITED: types::MQRC = types::MQRC(
        mqsys::MQRC_PUBSUB_INHIBITED,
    );
    pub const MQRC_MSG_HANDLE_COPY_FAILURE: types::MQRC = types::MQRC(
        mqsys::MQRC_MSG_HANDLE_COPY_FAILURE,
    );
    pub const MQRC_DEST_CLASS_NOT_ALTERABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_DEST_CLASS_NOT_ALTERABLE,
    );
    pub const MQRC_OPERATION_NOT_ALLOWED: types::MQRC = types::MQRC(
        mqsys::MQRC_OPERATION_NOT_ALLOWED,
    );
    pub const MQRC_ACTION_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_ACTION_ERROR);
    pub const MQRC_CHANNEL_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_NOT_AVAILABLE,
    );
    pub const MQRC_HOST_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_HOST_NOT_AVAILABLE,
    );
    pub const MQRC_CHANNEL_CONFIG_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_CONFIG_ERROR,
    );
    pub const MQRC_UNKNOWN_CHANNEL_NAME: types::MQRC = types::MQRC(
        mqsys::MQRC_UNKNOWN_CHANNEL_NAME,
    );
    pub const MQRC_LOOPING_PUBLICATION: types::MQRC = types::MQRC(
        mqsys::MQRC_LOOPING_PUBLICATION,
    );
    pub const MQRC_ALREADY_JOINED: types::MQRC = types::MQRC(mqsys::MQRC_ALREADY_JOINED);
    pub const MQRC_STANDBY_Q_MGR: types::MQRC = types::MQRC(mqsys::MQRC_STANDBY_Q_MGR);
    pub const MQRC_RECONNECTING: types::MQRC = types::MQRC(mqsys::MQRC_RECONNECTING);
    pub const MQRC_RECONNECTED: types::MQRC = types::MQRC(mqsys::MQRC_RECONNECTED);
    pub const MQRC_RECONNECT_QMID_MISMATCH: types::MQRC = types::MQRC(
        mqsys::MQRC_RECONNECT_QMID_MISMATCH,
    );
    pub const MQRC_RECONNECT_INCOMPATIBLE: types::MQRC = types::MQRC(
        mqsys::MQRC_RECONNECT_INCOMPATIBLE,
    );
    pub const MQRC_RECONNECT_FAILED: types::MQRC = types::MQRC(
        mqsys::MQRC_RECONNECT_FAILED,
    );
    pub const MQRC_CALL_INTERRUPTED: types::MQRC = types::MQRC(
        mqsys::MQRC_CALL_INTERRUPTED,
    );
    pub const MQRC_NO_SUBS_MATCHED: types::MQRC = types::MQRC(
        mqsys::MQRC_NO_SUBS_MATCHED,
    );
    pub const MQRC_SELECTION_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_SELECTION_NOT_AVAILABLE,
    );
    pub const MQRC_CHANNEL_SSL_WARNING: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_SSL_WARNING,
    );
    pub const MQRC_OCSP_URL_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_OCSP_URL_ERROR);
    pub const MQRC_CONTENT_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CONTENT_ERROR);
    pub const MQRC_RECONNECT_Q_MGR_REQD: types::MQRC = types::MQRC(
        mqsys::MQRC_RECONNECT_Q_MGR_REQD,
    );
    pub const MQRC_RECONNECT_TIMED_OUT: types::MQRC = types::MQRC(
        mqsys::MQRC_RECONNECT_TIMED_OUT,
    );
    pub const MQRC_PUBLISH_EXIT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_PUBLISH_EXIT_ERROR,
    );
    pub const MQRC_COMMINFO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_COMMINFO_ERROR);
    pub const MQRC_DEF_SYNCPOINT_INHIBITED: types::MQRC = types::MQRC(
        mqsys::MQRC_DEF_SYNCPOINT_INHIBITED,
    );
    pub const MQRC_MULTICAST_ONLY: types::MQRC = types::MQRC(mqsys::MQRC_MULTICAST_ONLY);
    pub const MQRC_DATA_SET_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_DATA_SET_NOT_AVAILABLE,
    );
    pub const MQRC_GROUPING_NOT_ALLOWED: types::MQRC = types::MQRC(
        mqsys::MQRC_GROUPING_NOT_ALLOWED,
    );
    pub const MQRC_GROUP_ADDRESS_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_GROUP_ADDRESS_ERROR,
    );
    pub const MQRC_MULTICAST_CONFIG_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_MULTICAST_CONFIG_ERROR,
    );
    pub const MQRC_MULTICAST_INTERFACE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_MULTICAST_INTERFACE_ERROR,
    );
    pub const MQRC_MULTICAST_SEND_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_MULTICAST_SEND_ERROR,
    );
    pub const MQRC_MULTICAST_INTERNAL_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_MULTICAST_INTERNAL_ERROR,
    );
    pub const MQRC_CONNECTION_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_CONNECTION_NOT_AVAILABLE,
    );
    pub const MQRC_SYNCPOINT_NOT_ALLOWED: types::MQRC = types::MQRC(
        mqsys::MQRC_SYNCPOINT_NOT_ALLOWED,
    );
    pub const MQRC_SSL_ALT_PROVIDER_REQUIRED: types::MQRC = types::MQRC(
        mqsys::MQRC_SSL_ALT_PROVIDER_REQUIRED,
    );
    pub const MQRC_MCAST_PUB_STATUS: types::MQRC = types::MQRC(
        mqsys::MQRC_MCAST_PUB_STATUS,
    );
    pub const MQRC_MCAST_SUB_STATUS: types::MQRC = types::MQRC(
        mqsys::MQRC_MCAST_SUB_STATUS,
    );
    pub const MQRC_PRECONN_EXIT_LOAD_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_PRECONN_EXIT_LOAD_ERROR,
    );
    pub const MQRC_PRECONN_EXIT_NOT_FOUND: types::MQRC = types::MQRC(
        mqsys::MQRC_PRECONN_EXIT_NOT_FOUND,
    );
    pub const MQRC_PRECONN_EXIT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_PRECONN_EXIT_ERROR,
    );
    pub const MQRC_CD_ARRAY_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CD_ARRAY_ERROR);
    pub const MQRC_CHANNEL_BLOCKED: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_BLOCKED,
    );
    pub const MQRC_CHANNEL_BLOCKED_WARNING: types::MQRC = types::MQRC(
        mqsys::MQRC_CHANNEL_BLOCKED_WARNING,
    );
    pub const MQRC_SUBSCRIPTION_CREATE: types::MQRC = types::MQRC(
        mqsys::MQRC_SUBSCRIPTION_CREATE,
    );
    pub const MQRC_SUBSCRIPTION_DELETE: types::MQRC = types::MQRC(
        mqsys::MQRC_SUBSCRIPTION_DELETE,
    );
    pub const MQRC_SUBSCRIPTION_CHANGE: types::MQRC = types::MQRC(
        mqsys::MQRC_SUBSCRIPTION_CHANGE,
    );
    pub const MQRC_SUBSCRIPTION_REFRESH: types::MQRC = types::MQRC(
        mqsys::MQRC_SUBSCRIPTION_REFRESH,
    );
    pub const MQRC_INSTALLATION_MISMATCH: types::MQRC = types::MQRC(
        mqsys::MQRC_INSTALLATION_MISMATCH,
    );
    pub const MQRC_NOT_PRIVILEGED: types::MQRC = types::MQRC(mqsys::MQRC_NOT_PRIVILEGED);
    pub const MQRC_PROPERTIES_DISABLED: types::MQRC = types::MQRC(
        mqsys::MQRC_PROPERTIES_DISABLED,
    );
    pub const MQRC_HMSG_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_HMSG_NOT_AVAILABLE,
    );
    pub const MQRC_EXIT_PROPS_NOT_SUPPORTED: types::MQRC = types::MQRC(
        mqsys::MQRC_EXIT_PROPS_NOT_SUPPORTED,
    );
    pub const MQRC_INSTALLATION_MISSING: types::MQRC = types::MQRC(
        mqsys::MQRC_INSTALLATION_MISSING,
    );
    pub const MQRC_FASTPATH_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_FASTPATH_NOT_AVAILABLE,
    );
    pub const MQRC_CIPHER_SPEC_NOT_SUITE_B: types::MQRC = types::MQRC(
        mqsys::MQRC_CIPHER_SPEC_NOT_SUITE_B,
    );
    pub const MQRC_SUITE_B_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_SUITE_B_ERROR);
    pub const MQRC_CERT_VAL_POLICY_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CERT_VAL_POLICY_ERROR,
    );
    pub const MQRC_PASSWORD_PROTECTION_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_PASSWORD_PROTECTION_ERROR,
    );
    pub const MQRC_CSP_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CSP_ERROR);
    pub const MQRC_CERT_LABEL_NOT_ALLOWED: types::MQRC = types::MQRC(
        mqsys::MQRC_CERT_LABEL_NOT_ALLOWED,
    );
    pub const MQRC_ADMIN_TOPIC_STRING_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_ADMIN_TOPIC_STRING_ERROR,
    );
    pub const MQRC_AMQP_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_AMQP_NOT_AVAILABLE,
    );
    pub const MQRC_CCDT_URL_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_CCDT_URL_ERROR);
    pub const MQRC_Q_MGR_RECONNECT_REQUESTED: types::MQRC = types::MQRC(
        mqsys::MQRC_Q_MGR_RECONNECT_REQUESTED,
    );
    pub const MQRC_BNO_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_BNO_ERROR);
    pub const MQRC_OUTBOUND_SNI_NOT_VALID: types::MQRC = types::MQRC(
        mqsys::MQRC_OUTBOUND_SNI_NOT_VALID,
    );
    pub const MQRC_HTTPS_KEYSTORE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_HTTPS_KEYSTORE_ERROR,
    );
    pub const MQRC_REOPEN_EXCL_INPUT_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_REOPEN_EXCL_INPUT_ERROR,
    );
    pub const MQRC_REOPEN_INQUIRE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_REOPEN_INQUIRE_ERROR,
    );
    pub const MQRC_REOPEN_SAVED_CONTEXT_ERR: types::MQRC = types::MQRC(
        mqsys::MQRC_REOPEN_SAVED_CONTEXT_ERR,
    );
    pub const MQRC_REOPEN_TEMPORARY_Q_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_REOPEN_TEMPORARY_Q_ERROR,
    );
    pub const MQRC_ATTRIBUTE_LOCKED: types::MQRC = types::MQRC(
        mqsys::MQRC_ATTRIBUTE_LOCKED,
    );
    pub const MQRC_CURSOR_NOT_VALID: types::MQRC = types::MQRC(
        mqsys::MQRC_CURSOR_NOT_VALID,
    );
    pub const MQRC_ENCODING_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_ENCODING_ERROR);
    pub const MQRC_STRUC_ID_ERROR: types::MQRC = types::MQRC(mqsys::MQRC_STRUC_ID_ERROR);
    pub const MQRC_NULL_POINTER: types::MQRC = types::MQRC(mqsys::MQRC_NULL_POINTER);
    pub const MQRC_NO_CONNECTION_REFERENCE: types::MQRC = types::MQRC(
        mqsys::MQRC_NO_CONNECTION_REFERENCE,
    );
    pub const MQRC_NO_BUFFER: types::MQRC = types::MQRC(mqsys::MQRC_NO_BUFFER);
    pub const MQRC_BINARY_DATA_LENGTH_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_BINARY_DATA_LENGTH_ERROR,
    );
    pub const MQRC_BUFFER_NOT_AUTOMATIC: types::MQRC = types::MQRC(
        mqsys::MQRC_BUFFER_NOT_AUTOMATIC,
    );
    pub const MQRC_INSUFFICIENT_BUFFER: types::MQRC = types::MQRC(
        mqsys::MQRC_INSUFFICIENT_BUFFER,
    );
    pub const MQRC_INSUFFICIENT_DATA: types::MQRC = types::MQRC(
        mqsys::MQRC_INSUFFICIENT_DATA,
    );
    pub const MQRC_DATA_TRUNCATED: types::MQRC = types::MQRC(mqsys::MQRC_DATA_TRUNCATED);
    pub const MQRC_ZERO_LENGTH: types::MQRC = types::MQRC(mqsys::MQRC_ZERO_LENGTH);
    pub const MQRC_NEGATIVE_LENGTH: types::MQRC = types::MQRC(
        mqsys::MQRC_NEGATIVE_LENGTH,
    );
    pub const MQRC_NEGATIVE_OFFSET: types::MQRC = types::MQRC(
        mqsys::MQRC_NEGATIVE_OFFSET,
    );
    pub const MQRC_INCONSISTENT_FORMAT: types::MQRC = types::MQRC(
        mqsys::MQRC_INCONSISTENT_FORMAT,
    );
    pub const MQRC_INCONSISTENT_OBJECT_STATE: types::MQRC = types::MQRC(
        mqsys::MQRC_INCONSISTENT_OBJECT_STATE,
    );
    pub const MQRC_CONTEXT_OBJECT_NOT_VALID: types::MQRC = types::MQRC(
        mqsys::MQRC_CONTEXT_OBJECT_NOT_VALID,
    );
    pub const MQRC_CONTEXT_OPEN_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_CONTEXT_OPEN_ERROR,
    );
    pub const MQRC_STRUC_LENGTH_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_STRUC_LENGTH_ERROR,
    );
    pub const MQRC_NOT_CONNECTED: types::MQRC = types::MQRC(mqsys::MQRC_NOT_CONNECTED);
    pub const MQRC_NOT_OPEN: types::MQRC = types::MQRC(mqsys::MQRC_NOT_OPEN);
    pub const MQRC_DISTRIBUTION_LIST_EMPTY: types::MQRC = types::MQRC(
        mqsys::MQRC_DISTRIBUTION_LIST_EMPTY,
    );
    pub const MQRC_INCONSISTENT_OPEN_OPTIONS: types::MQRC = types::MQRC(
        mqsys::MQRC_INCONSISTENT_OPEN_OPTIONS,
    );
    pub const MQRC_WRONG_VERSION: types::MQRC = types::MQRC(mqsys::MQRC_WRONG_VERSION);
    pub const MQRC_REFERENCE_ERROR: types::MQRC = types::MQRC(
        mqsys::MQRC_REFERENCE_ERROR,
    );
    pub const MQRC_XR_NOT_AVAILABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_XR_NOT_AVAILABLE,
    );
    pub const MQRC_SUB_JOIN_NOT_ALTERABLE: types::MQRC = types::MQRC(
        mqsys::MQRC_SUB_JOIN_NOT_ALTERABLE,
    );
    pub const MQRDNS_ENABLED: types::MQRDNS = types::MQRDNS(mqsys::MQRDNS_ENABLED);
    pub const MQRDNS_DISABLED: types::MQRDNS = types::MQRDNS(mqsys::MQRDNS_DISABLED);
    pub const MQRD_NO_RECONNECT: types::MQRD = types::MQRD(mqsys::MQRD_NO_RECONNECT);
    pub const MQRD_NO_DELAY: types::MQRD = types::MQRD(mqsys::MQRD_NO_DELAY);
    pub const MQREADA_NO: types::MQREADA = types::MQREADA(mqsys::MQREADA_NO);
    pub const MQREADA_YES: types::MQREADA = types::MQREADA(mqsys::MQREADA_YES);
    pub const MQREADA_DISABLED: types::MQREADA = types::MQREADA(mqsys::MQREADA_DISABLED);
    pub const MQREADA_INHIBITED: types::MQREADA = types::MQREADA(
        mqsys::MQREADA_INHIBITED,
    );
    pub const MQREADA_BACKLOG: types::MQREADA = types::MQREADA(mqsys::MQREADA_BACKLOG);
    pub const MQRECAUTO_NO: types::MQRECAUTO = types::MQRECAUTO(mqsys::MQRECAUTO_NO);
    pub const MQRECAUTO_YES: types::MQRECAUTO = types::MQRECAUTO(mqsys::MQRECAUTO_YES);
    pub const MQRECORDING_DISABLED: types::MQRECORDING = types::MQRECORDING(
        mqsys::MQRECORDING_DISABLED,
    );
    pub const MQRECORDING_Q: types::MQRECORDING = types::MQRECORDING(
        mqsys::MQRECORDING_Q,
    );
    pub const MQRECORDING_MSG: types::MQRECORDING = types::MQRECORDING(
        mqsys::MQRECORDING_MSG,
    );
    pub const MQREGO_NONE: types::MQREGO = types::MQREGO(mqsys::MQREGO_NONE);
    pub const MQREGO_CORREL_ID_AS_IDENTITY: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_CORREL_ID_AS_IDENTITY,
    );
    pub const MQREGO_ANONYMOUS: types::MQREGO = types::MQREGO(mqsys::MQREGO_ANONYMOUS);
    pub const MQREGO_LOCAL: types::MQREGO = types::MQREGO(mqsys::MQREGO_LOCAL);
    pub const MQREGO_DIRECT_REQUESTS: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_DIRECT_REQUESTS,
    );
    pub const MQREGO_NEW_PUBLICATIONS_ONLY: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_NEW_PUBLICATIONS_ONLY,
    );
    pub const MQREGO_PUBLISH_ON_REQUEST_ONLY: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_PUBLISH_ON_REQUEST_ONLY,
    );
    pub const MQREGO_DEREGISTER_ALL: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_DEREGISTER_ALL,
    );
    pub const MQREGO_INCLUDE_STREAM_NAME: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_INCLUDE_STREAM_NAME,
    );
    pub const MQREGO_INFORM_IF_RETAINED: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_INFORM_IF_RETAINED,
    );
    pub const MQREGO_DUPLICATES_OK: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_DUPLICATES_OK,
    );
    pub const MQREGO_NON_PERSISTENT: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_NON_PERSISTENT,
    );
    pub const MQREGO_PERSISTENT: types::MQREGO = types::MQREGO(mqsys::MQREGO_PERSISTENT);
    pub const MQREGO_PERSISTENT_AS_PUBLISH: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_PERSISTENT_AS_PUBLISH,
    );
    pub const MQREGO_PERSISTENT_AS_Q: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_PERSISTENT_AS_Q,
    );
    pub const MQREGO_ADD_NAME: types::MQREGO = types::MQREGO(mqsys::MQREGO_ADD_NAME);
    pub const MQREGO_NO_ALTERATION: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_NO_ALTERATION,
    );
    pub const MQREGO_FULL_RESPONSE: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_FULL_RESPONSE,
    );
    pub const MQREGO_JOIN_SHARED: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_JOIN_SHARED,
    );
    pub const MQREGO_JOIN_EXCLUSIVE: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_JOIN_EXCLUSIVE,
    );
    pub const MQREGO_LEAVE_ONLY: types::MQREGO = types::MQREGO(mqsys::MQREGO_LEAVE_ONLY);
    pub const MQREGO_VARIABLE_USER_ID: types::MQREGO = types::MQREGO(
        mqsys::MQREGO_VARIABLE_USER_ID,
    );
    pub const MQREGO_LOCKED: types::MQREGO = types::MQREGO(mqsys::MQREGO_LOCKED);
    pub const MQREORG_DISABLED: types::MQREORG = types::MQREORG(mqsys::MQREORG_DISABLED);
    pub const MQREORG_ENABLED: types::MQREORG = types::MQREORG(mqsys::MQREORG_ENABLED);
    pub const MQRFH_FLAGS_RESTRICTED_MASK: types::MQRFH = types::MQRFH(
        mqsys::MQRFH_FLAGS_RESTRICTED_MASK,
    );
    pub const MQRFH_NONE: types::MQRFH = types::MQRFH(mqsys::MQRFH_NONE);
    pub const MQRFH_NO_FLAGS: types::MQRFH = types::MQRFH(mqsys::MQRFH_NO_FLAGS);
    pub const MQRL_UNDEFINED: types::MQRL = types::MQRL(mqsys::MQRL_UNDEFINED);
    pub const MQRMHF_NOT_LAST: types::MQRMHF = types::MQRMHF(mqsys::MQRMHF_NOT_LAST);
    pub const MQRMHF_LAST: types::MQRMHF = types::MQRMHF(mqsys::MQRMHF_LAST);
    pub const MQROUTE_DELIVER_REJ_UNSUP_MASK: types::MQROUTE = types::MQROUTE(
        mqsys::MQROUTE_DELIVER_REJ_UNSUP_MASK,
    );
    pub const MQROUTE_UNLIMITED_ACTIVITIES: types::MQROUTE = types::MQROUTE(
        mqsys::MQROUTE_UNLIMITED_ACTIVITIES,
    );
    pub const MQROUTE_DETAIL_LOW: types::MQROUTE = types::MQROUTE(
        mqsys::MQROUTE_DETAIL_LOW,
    );
    pub const MQROUTE_DETAIL_MEDIUM: types::MQROUTE = types::MQROUTE(
        mqsys::MQROUTE_DETAIL_MEDIUM,
    );
    pub const MQROUTE_DETAIL_HIGH: types::MQROUTE = types::MQROUTE(
        mqsys::MQROUTE_DETAIL_HIGH,
    );
    pub const MQROUTE_FORWARD_ALL: types::MQROUTE = types::MQROUTE(
        mqsys::MQROUTE_FORWARD_ALL,
    );
    pub const MQROUTE_FORWARD_IF_SUPPORTED: types::MQROUTE = types::MQROUTE(
        mqsys::MQROUTE_FORWARD_IF_SUPPORTED,
    );
    pub const MQROUTE_DELIVER_YES: types::MQROUTE = types::MQROUTE(
        mqsys::MQROUTE_DELIVER_YES,
    );
    pub const MQROUTE_DELIVER_NO: types::MQROUTE = types::MQROUTE(
        mqsys::MQROUTE_DELIVER_NO,
    );
    pub const MQROUTE_ACCUMULATE_NONE: types::MQROUTE = types::MQROUTE(
        mqsys::MQROUTE_ACCUMULATE_NONE,
    );
    pub const MQROUTE_ACCUMULATE_IN_MSG: types::MQROUTE = types::MQROUTE(
        mqsys::MQROUTE_ACCUMULATE_IN_MSG,
    );
    pub const MQROUTE_ACCUMULATE_AND_REPLY: types::MQROUTE = types::MQROUTE(
        mqsys::MQROUTE_ACCUMULATE_AND_REPLY,
    );
    pub const MQROUTE_FORWARD_REJ_UNSUP_MASK: types::MQROUTE = types::MQROUTE(
        mqsys::MQROUTE_FORWARD_REJ_UNSUP_MASK,
    );
    pub const MQRO_ACCEPT_UNSUP_MASK: types::MQRO = types::MQRO(
        mqsys::MQRO_ACCEPT_UNSUP_MASK,
    );
    pub const MQRO_NONE: types::MQRO = types::MQRO(mqsys::MQRO_NONE);
    pub const MQRO_PAN: types::MQRO = types::MQRO(mqsys::MQRO_PAN);
    pub const MQRO_NAN: types::MQRO = types::MQRO(mqsys::MQRO_NAN);
    pub const MQRO_ACTIVITY: types::MQRO = types::MQRO(mqsys::MQRO_ACTIVITY);
    pub const MQRO_PASS_CORREL_ID: types::MQRO = types::MQRO(mqsys::MQRO_PASS_CORREL_ID);
    pub const MQRO_PASS_MSG_ID: types::MQRO = types::MQRO(mqsys::MQRO_PASS_MSG_ID);
    pub const MQRO_COA: types::MQRO = types::MQRO(mqsys::MQRO_COA);
    pub const MQRO_COA_WITH_DATA: types::MQRO = types::MQRO(mqsys::MQRO_COA_WITH_DATA);
    pub const MQRO_COA_WITH_FULL_DATA: types::MQRO = types::MQRO(
        mqsys::MQRO_COA_WITH_FULL_DATA,
    );
    pub const MQRO_COD: types::MQRO = types::MQRO(mqsys::MQRO_COD);
    pub const MQRO_COD_WITH_DATA: types::MQRO = types::MQRO(mqsys::MQRO_COD_WITH_DATA);
    pub const MQRO_COD_WITH_FULL_DATA: types::MQRO = types::MQRO(
        mqsys::MQRO_COD_WITH_FULL_DATA,
    );
    pub const MQRO_PASS_DISCARD_AND_EXPIRY: types::MQRO = types::MQRO(
        mqsys::MQRO_PASS_DISCARD_AND_EXPIRY,
    );
    pub const MQRO_ACCEPT_UNSUP_IF_XMIT_MASK: types::MQRO = types::MQRO(
        mqsys::MQRO_ACCEPT_UNSUP_IF_XMIT_MASK,
    );
    pub const MQRO_EXPIRATION: types::MQRO = types::MQRO(mqsys::MQRO_EXPIRATION);
    pub const MQRO_EXPIRATION_WITH_DATA: types::MQRO = types::MQRO(
        mqsys::MQRO_EXPIRATION_WITH_DATA,
    );
    pub const MQRO_EXPIRATION_WITH_FULL_DATA: types::MQRO = types::MQRO(
        mqsys::MQRO_EXPIRATION_WITH_FULL_DATA,
    );
    pub const MQRO_EXCEPTION: types::MQRO = types::MQRO(mqsys::MQRO_EXCEPTION);
    pub const MQRO_EXCEPTION_WITH_DATA: types::MQRO = types::MQRO(
        mqsys::MQRO_EXCEPTION_WITH_DATA,
    );
    pub const MQRO_EXCEPTION_WITH_FULL_DATA: types::MQRO = types::MQRO(
        mqsys::MQRO_EXCEPTION_WITH_FULL_DATA,
    );
    pub const MQRO_DISCARD_MSG: types::MQRO = types::MQRO(mqsys::MQRO_DISCARD_MSG);
    pub const MQRO_REJECT_UNSUP_MASK: types::MQRO = types::MQRO(
        mqsys::MQRO_REJECT_UNSUP_MASK,
    );
    pub const MQRO_COPY_MSG_ID_TO_CORREL_ID: types::MQRO = types::MQRO(
        mqsys::MQRO_COPY_MSG_ID_TO_CORREL_ID,
    );
    pub const MQRO_DEAD_LETTER_Q: types::MQRO = types::MQRO(mqsys::MQRO_DEAD_LETTER_Q);
    pub const MQRO_NEW_MSG_ID: types::MQRO = types::MQRO(mqsys::MQRO_NEW_MSG_ID);
    pub const MQRP_NO: types::MQRP = types::MQRP(mqsys::MQRP_NO);
    pub const MQRP_YES: types::MQRP = types::MQRP(mqsys::MQRP_YES);
    pub const MQRQ_CONN_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_CONN_NOT_AUTHORIZED,
    );
    pub const MQRQ_OPEN_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_OPEN_NOT_AUTHORIZED,
    );
    pub const MQRQ_CLOSE_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_CLOSE_NOT_AUTHORIZED,
    );
    pub const MQRQ_CMD_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_CMD_NOT_AUTHORIZED,
    );
    pub const MQRQ_Q_MGR_STOPPING: types::MQRQ = types::MQRQ(mqsys::MQRQ_Q_MGR_STOPPING);
    pub const MQRQ_Q_MGR_QUIESCING: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_Q_MGR_QUIESCING,
    );
    pub const MQRQ_CHANNEL_STOPPED_OK: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_CHANNEL_STOPPED_OK,
    );
    pub const MQRQ_CHANNEL_STOPPED_ERROR: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_CHANNEL_STOPPED_ERROR,
    );
    pub const MQRQ_CHANNEL_STOPPED_RETRY: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_CHANNEL_STOPPED_RETRY,
    );
    pub const MQRQ_CHANNEL_STOPPED_DISABLED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_CHANNEL_STOPPED_DISABLED,
    );
    pub const MQRQ_BRIDGE_STOPPED_OK: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_BRIDGE_STOPPED_OK,
    );
    pub const MQRQ_BRIDGE_STOPPED_ERROR: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_BRIDGE_STOPPED_ERROR,
    );
    pub const MQRQ_SSL_HANDSHAKE_ERROR: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_SSL_HANDSHAKE_ERROR,
    );
    pub const MQRQ_SSL_CIPHER_SPEC_ERROR: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_SSL_CIPHER_SPEC_ERROR,
    );
    pub const MQRQ_SSL_CLIENT_AUTH_ERROR: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_SSL_CLIENT_AUTH_ERROR,
    );
    pub const MQRQ_SSL_PEER_NAME_ERROR: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_SSL_PEER_NAME_ERROR,
    );
    pub const MQRQ_SUB_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_SUB_NOT_AUTHORIZED,
    );
    pub const MQRQ_SUB_DEST_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_SUB_DEST_NOT_AUTHORIZED,
    );
    pub const MQRQ_SSL_UNKNOWN_REVOCATION: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_SSL_UNKNOWN_REVOCATION,
    );
    pub const MQRQ_SYS_CONN_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_SYS_CONN_NOT_AUTHORIZED,
    );
    pub const MQRQ_CHANNEL_BLOCKED_ADDRESS: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_CHANNEL_BLOCKED_ADDRESS,
    );
    pub const MQRQ_CHANNEL_BLOCKED_USERID: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_CHANNEL_BLOCKED_USERID,
    );
    pub const MQRQ_CHANNEL_BLOCKED_NOACCESS: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_CHANNEL_BLOCKED_NOACCESS,
    );
    pub const MQRQ_MAX_ACTIVE_CHANNELS: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_MAX_ACTIVE_CHANNELS,
    );
    pub const MQRQ_MAX_CHANNELS: types::MQRQ = types::MQRQ(mqsys::MQRQ_MAX_CHANNELS);
    pub const MQRQ_SVRCONN_INST_LIMIT: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_SVRCONN_INST_LIMIT,
    );
    pub const MQRQ_CLIENT_INST_LIMIT: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_CLIENT_INST_LIMIT,
    );
    pub const MQRQ_CAF_NOT_INSTALLED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_CAF_NOT_INSTALLED,
    );
    pub const MQRQ_CSP_NOT_AUTHORIZED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_CSP_NOT_AUTHORIZED,
    );
    pub const MQRQ_FAILOVER_PERMITTED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_FAILOVER_PERMITTED,
    );
    pub const MQRQ_FAILOVER_NOT_PERMITTED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_FAILOVER_NOT_PERMITTED,
    );
    pub const MQRQ_STANDBY_ACTIVATED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_STANDBY_ACTIVATED,
    );
    pub const MQRQ_REPLICA_ACTIVATED: types::MQRQ = types::MQRQ(
        mqsys::MQRQ_REPLICA_ACTIVATED,
    );
    pub const MQRT_CONFIGURATION: types::MQRT = types::MQRT(mqsys::MQRT_CONFIGURATION);
    pub const MQRT_EXPIRY: types::MQRT = types::MQRT(mqsys::MQRT_EXPIRY);
    pub const MQRT_NSPROC: types::MQRT = types::MQRT(mqsys::MQRT_NSPROC);
    pub const MQRT_PROXYSUB: types::MQRT = types::MQRT(mqsys::MQRT_PROXYSUB);
    pub const MQRT_SUB_CONFIGURATION: types::MQRT = types::MQRT(
        mqsys::MQRT_SUB_CONFIGURATION,
    );
    pub const MQRU_PUBLISH_ON_REQUEST: types::MQRU = types::MQRU(
        mqsys::MQRU_PUBLISH_ON_REQUEST,
    );
    pub const MQRU_PUBLISH_ALL: types::MQRU = types::MQRU(mqsys::MQRU_PUBLISH_ALL);
    pub const MQSCA_REQUIRED: types::MQSCA = types::MQSCA(mqsys::MQSCA_REQUIRED);
    pub const MQSCA_OPTIONAL: types::MQSCA = types::MQSCA(mqsys::MQSCA_OPTIONAL);
    pub const MQSCA_NEVER_REQUIRED: types::MQSCA = types::MQSCA(
        mqsys::MQSCA_NEVER_REQUIRED,
    );
    pub const MQSCOPE_ALL: types::MQSCOPE = types::MQSCOPE(mqsys::MQSCOPE_ALL);
    pub const MQSCOPE_AS_PARENT: types::MQSCOPE = types::MQSCOPE(
        mqsys::MQSCOPE_AS_PARENT,
    );
    pub const MQSCOPE_QMGR: types::MQSCOPE = types::MQSCOPE(mqsys::MQSCOPE_QMGR);
    pub const MQSCO_Q_MGR: types::MQSCO = types::MQSCO(mqsys::MQSCO_Q_MGR);
    pub const MQSCO_CELL: types::MQSCO = types::MQSCO(mqsys::MQSCO_CELL);
    pub const MQSCO_RESET_COUNT_DEFAULT: types::MQSCO = types::MQSCO(
        mqsys::MQSCO_RESET_COUNT_DEFAULT,
    );
    pub const MQSCYC_UPPER: types::MQSCYC = types::MQSCYC(mqsys::MQSCYC_UPPER);
    pub const MQSCYC_MIXED: types::MQSCYC = types::MQSCYC(mqsys::MQSCYC_MIXED);
    pub const MQSECCOMM_NO: types::MQSECCOMM = types::MQSECCOMM(mqsys::MQSECCOMM_NO);
    pub const MQSECCOMM_YES: types::MQSECCOMM = types::MQSECCOMM(mqsys::MQSECCOMM_YES);
    pub const MQSECCOMM_ANON: types::MQSECCOMM = types::MQSECCOMM(mqsys::MQSECCOMM_ANON);
    pub const MQSECITEM_ALL: types::MQSECITEM = types::MQSECITEM(mqsys::MQSECITEM_ALL);
    pub const MQSECITEM_MQADMIN: types::MQSECITEM = types::MQSECITEM(
        mqsys::MQSECITEM_MQADMIN,
    );
    pub const MQSECITEM_MQNLIST: types::MQSECITEM = types::MQSECITEM(
        mqsys::MQSECITEM_MQNLIST,
    );
    pub const MQSECITEM_MQPROC: types::MQSECITEM = types::MQSECITEM(
        mqsys::MQSECITEM_MQPROC,
    );
    pub const MQSECITEM_MQQUEUE: types::MQSECITEM = types::MQSECITEM(
        mqsys::MQSECITEM_MQQUEUE,
    );
    pub const MQSECITEM_MQCONN: types::MQSECITEM = types::MQSECITEM(
        mqsys::MQSECITEM_MQCONN,
    );
    pub const MQSECITEM_MQCMDS: types::MQSECITEM = types::MQSECITEM(
        mqsys::MQSECITEM_MQCMDS,
    );
    pub const MQSECITEM_MXADMIN: types::MQSECITEM = types::MQSECITEM(
        mqsys::MQSECITEM_MXADMIN,
    );
    pub const MQSECITEM_MXNLIST: types::MQSECITEM = types::MQSECITEM(
        mqsys::MQSECITEM_MXNLIST,
    );
    pub const MQSECITEM_MXPROC: types::MQSECITEM = types::MQSECITEM(
        mqsys::MQSECITEM_MXPROC,
    );
    pub const MQSECITEM_MXQUEUE: types::MQSECITEM = types::MQSECITEM(
        mqsys::MQSECITEM_MXQUEUE,
    );
    pub const MQSECITEM_MXTOPIC: types::MQSECITEM = types::MQSECITEM(
        mqsys::MQSECITEM_MXTOPIC,
    );
    pub const MQSECPROT_NONE: types::MQSECPROT = types::MQSECPROT(mqsys::MQSECPROT_NONE);
    pub const MQSECPROT_SSLV30: types::MQSECPROT = types::MQSECPROT(
        mqsys::MQSECPROT_SSLV30,
    );
    pub const MQSECPROT_TLSV10: types::MQSECPROT = types::MQSECPROT(
        mqsys::MQSECPROT_TLSV10,
    );
    pub const MQSECPROT_TLSV12: types::MQSECPROT = types::MQSECPROT(
        mqsys::MQSECPROT_TLSV12,
    );
    pub const MQSECPROT_TLSV13: types::MQSECPROT = types::MQSECPROT(
        mqsys::MQSECPROT_TLSV13,
    );
    pub const MQSECSW_PROCESS: types::MQSECSW = types::MQSECSW(mqsys::MQSECSW_PROCESS);
    pub const MQSECSW_NAMELIST: types::MQSECSW = types::MQSECSW(mqsys::MQSECSW_NAMELIST);
    pub const MQSECSW_Q: types::MQSECSW = types::MQSECSW(mqsys::MQSECSW_Q);
    pub const MQSECSW_TOPIC: types::MQSECSW = types::MQSECSW(mqsys::MQSECSW_TOPIC);
    pub const MQSECSW_CONTEXT: types::MQSECSW = types::MQSECSW(mqsys::MQSECSW_CONTEXT);
    pub const MQSECSW_ALTERNATE_USER: types::MQSECSW = types::MQSECSW(
        mqsys::MQSECSW_ALTERNATE_USER,
    );
    pub const MQSECSW_COMMAND: types::MQSECSW = types::MQSECSW(mqsys::MQSECSW_COMMAND);
    pub const MQSECSW_CONNECTION: types::MQSECSW = types::MQSECSW(
        mqsys::MQSECSW_CONNECTION,
    );
    pub const MQSECSW_SUBSYSTEM: types::MQSECSW = types::MQSECSW(
        mqsys::MQSECSW_SUBSYSTEM,
    );
    pub const MQSECSW_COMMAND_RESOURCES: types::MQSECSW = types::MQSECSW(
        mqsys::MQSECSW_COMMAND_RESOURCES,
    );
    pub const MQSECSW_Q_MGR: types::MQSECSW = types::MQSECSW(mqsys::MQSECSW_Q_MGR);
    pub const MQSECSW_QSG: types::MQSECSW = types::MQSECSW(mqsys::MQSECSW_QSG);
    pub const MQSECSW_OFF_FOUND: types::MQSECSW = types::MQSECSW(
        mqsys::MQSECSW_OFF_FOUND,
    );
    pub const MQSECSW_ON_FOUND: types::MQSECSW = types::MQSECSW(mqsys::MQSECSW_ON_FOUND);
    pub const MQSECSW_OFF_NOT_FOUND: types::MQSECSW = types::MQSECSW(
        mqsys::MQSECSW_OFF_NOT_FOUND,
    );
    pub const MQSECSW_ON_NOT_FOUND: types::MQSECSW = types::MQSECSW(
        mqsys::MQSECSW_ON_NOT_FOUND,
    );
    pub const MQSECSW_OFF_ERROR: types::MQSECSW = types::MQSECSW(
        mqsys::MQSECSW_OFF_ERROR,
    );
    pub const MQSECSW_ON_OVERRIDDEN: types::MQSECSW = types::MQSECSW(
        mqsys::MQSECSW_ON_OVERRIDDEN,
    );
    pub const MQSECTYPE_AUTHSERV: types::MQSECTYPE = types::MQSECTYPE(
        mqsys::MQSECTYPE_AUTHSERV,
    );
    pub const MQSECTYPE_SSL: types::MQSECTYPE = types::MQSECTYPE(mqsys::MQSECTYPE_SSL);
    pub const MQSECTYPE_CLASSES: types::MQSECTYPE = types::MQSECTYPE(
        mqsys::MQSECTYPE_CLASSES,
    );
    pub const MQSECTYPE_CONNAUTH: types::MQSECTYPE = types::MQSECTYPE(
        mqsys::MQSECTYPE_CONNAUTH,
    );
    pub const MQSELTYPE_NONE: types::MQSELTYPE = types::MQSELTYPE(mqsys::MQSELTYPE_NONE);
    pub const MQSELTYPE_STANDARD: types::MQSELTYPE = types::MQSELTYPE(
        mqsys::MQSELTYPE_STANDARD,
    );
    pub const MQSELTYPE_EXTENDED: types::MQSELTYPE = types::MQSELTYPE(
        mqsys::MQSELTYPE_EXTENDED,
    );
    pub const MQSEL_ALL_SYSTEM_SELECTORS: types::MQSEL_ALL = types::MQSEL_ALL(
        mqsys::MQSEL_ALL_SYSTEM_SELECTORS,
    );
    pub const MQSEL_ALL_USER_SELECTORS: types::MQSEL_ALL = types::MQSEL_ALL(
        mqsys::MQSEL_ALL_USER_SELECTORS,
    );
    pub const MQSEL_ALL_SELECTORS: types::MQSEL_ALL = types::MQSEL_ALL(
        mqsys::MQSEL_ALL_SELECTORS,
    );
    pub const MQSEL_ANY_SYSTEM_SELECTOR: types::MQSEL_ANY = types::MQSEL_ANY(
        mqsys::MQSEL_ANY_SYSTEM_SELECTOR,
    );
    pub const MQSEL_ANY_USER_SELECTOR: types::MQSEL_ANY = types::MQSEL_ANY(
        mqsys::MQSEL_ANY_USER_SELECTOR,
    );
    pub const MQSEL_ANY_SELECTOR: types::MQSEL_ANY = types::MQSEL_ANY(
        mqsys::MQSEL_ANY_SELECTOR,
    );
    pub const MQSMPO_NONE: types::MQSMPO = types::MQSMPO(mqsys::MQSMPO_NONE);
    pub const MQSMPO_SET_PROP_UNDER_CURSOR: types::MQSMPO = types::MQSMPO(
        mqsys::MQSMPO_SET_PROP_UNDER_CURSOR,
    );
    pub const MQSMPO_SET_PROP_AFTER_CURSOR: types::MQSMPO = types::MQSMPO(
        mqsys::MQSMPO_SET_PROP_AFTER_CURSOR,
    );
    pub const MQSMPO_APPEND_PROPERTY: types::MQSMPO = types::MQSMPO(
        mqsys::MQSMPO_APPEND_PROPERTY,
    );
    pub const MQSMPO_SET_PROP_BEFORE_CURSOR: types::MQSMPO = types::MQSMPO(
        mqsys::MQSMPO_SET_PROP_BEFORE_CURSOR,
    );
    pub const MQSO_NONE: types::MQSO = types::MQSO(mqsys::MQSO_NONE);
    pub const MQSO_ALTER: types::MQSO = types::MQSO(mqsys::MQSO_ALTER);
    pub const MQSO_CREATE: types::MQSO = types::MQSO(mqsys::MQSO_CREATE);
    pub const MQSO_RESUME: types::MQSO = types::MQSO(mqsys::MQSO_RESUME);
    pub const MQSO_DURABLE: types::MQSO = types::MQSO(mqsys::MQSO_DURABLE);
    pub const MQSO_GROUP_SUB: types::MQSO = types::MQSO(mqsys::MQSO_GROUP_SUB);
    pub const MQSO_MANAGED: types::MQSO = types::MQSO(mqsys::MQSO_MANAGED);
    pub const MQSO_SET_IDENTITY_CONTEXT: types::MQSO = types::MQSO(
        mqsys::MQSO_SET_IDENTITY_CONTEXT,
    );
    pub const MQSO_NO_MULTICAST: types::MQSO = types::MQSO(mqsys::MQSO_NO_MULTICAST);
    pub const MQSO_FIXED_USERID: types::MQSO = types::MQSO(mqsys::MQSO_FIXED_USERID);
    pub const MQSO_ANY_USERID: types::MQSO = types::MQSO(mqsys::MQSO_ANY_USERID);
    pub const MQSO_PUBLICATIONS_ON_REQUEST: types::MQSO = types::MQSO(
        mqsys::MQSO_PUBLICATIONS_ON_REQUEST,
    );
    pub const MQSO_NEW_PUBLICATIONS_ONLY: types::MQSO = types::MQSO(
        mqsys::MQSO_NEW_PUBLICATIONS_ONLY,
    );
    pub const MQSO_FAIL_IF_QUIESCING: types::MQSO = types::MQSO(
        mqsys::MQSO_FAIL_IF_QUIESCING,
    );
    pub const MQSO_ALTERNATE_USER_AUTHORITY: types::MQSO = types::MQSO(
        mqsys::MQSO_ALTERNATE_USER_AUTHORITY,
    );
    pub const MQSO_WILDCARD_CHAR: types::MQSO = types::MQSO(mqsys::MQSO_WILDCARD_CHAR);
    pub const MQSO_WILDCARD_TOPIC: types::MQSO = types::MQSO(mqsys::MQSO_WILDCARD_TOPIC);
    pub const MQSO_SET_CORREL_ID: types::MQSO = types::MQSO(mqsys::MQSO_SET_CORREL_ID);
    pub const MQSO_SCOPE_QMGR: types::MQSO = types::MQSO(mqsys::MQSO_SCOPE_QMGR);
    pub const MQSO_NO_READ_AHEAD: types::MQSO = types::MQSO(mqsys::MQSO_NO_READ_AHEAD);
    pub const MQSO_READ_AHEAD: types::MQSO = types::MQSO(mqsys::MQSO_READ_AHEAD);
    pub const MQSO_NON_DURABLE: types::MQSO = types::MQSO(mqsys::MQSO_NON_DURABLE);
    pub const MQSO_READ_AHEAD_AS_Q_DEF: types::MQSO = types::MQSO(
        mqsys::MQSO_READ_AHEAD_AS_Q_DEF,
    );
    pub const MQSPL_PASSTHRU: types::MQSPL = types::MQSPL(mqsys::MQSPL_PASSTHRU);
    pub const MQSPL_REMOVE: types::MQSPL = types::MQSPL(mqsys::MQSPL_REMOVE);
    pub const MQSPL_AS_POLICY: types::MQSPL = types::MQSPL(mqsys::MQSPL_AS_POLICY);
    pub const MQSP_NOT_AVAILABLE: types::MQSP = types::MQSP(mqsys::MQSP_NOT_AVAILABLE);
    pub const MQSP_AVAILABLE: types::MQSP = types::MQSP(mqsys::MQSP_AVAILABLE);
    pub const MQSQQM_USE: types::MQSQQM = types::MQSQQM(mqsys::MQSQQM_USE);
    pub const MQSQQM_IGNORE: types::MQSQQM = types::MQSQQM(mqsys::MQSQQM_IGNORE);
    pub const MQSRO_NONE: types::MQSRO = types::MQSRO(mqsys::MQSRO_NONE);
    pub const MQSRO_FAIL_IF_QUIESCING: types::MQSRO = types::MQSRO(
        mqsys::MQSRO_FAIL_IF_QUIESCING,
    );
    pub const MQSR_ACTION_PUBLICATION: types::MQSR = types::MQSR(
        mqsys::MQSR_ACTION_PUBLICATION,
    );
    pub const MQSSL_FIPS_NO: types::MQSSL = types::MQSSL(mqsys::MQSSL_FIPS_NO);
    pub const MQSSL_FIPS_YES: types::MQSSL = types::MQSSL(mqsys::MQSSL_FIPS_YES);
    pub const MQSTAT_TYPE_ASYNC_ERROR: types::MQSTAT = types::MQSTAT(
        mqsys::MQSTAT_TYPE_ASYNC_ERROR,
    );
    pub const MQSTAT_TYPE_RECONNECTION: types::MQSTAT = types::MQSTAT(
        mqsys::MQSTAT_TYPE_RECONNECTION,
    );
    pub const MQSTAT_TYPE_RECONNECTION_ERROR: types::MQSTAT = types::MQSTAT(
        mqsys::MQSTAT_TYPE_RECONNECTION_ERROR,
    );
    pub const MQSTDBY_NOT_PERMITTED: types::MQSTDBY = types::MQSTDBY(
        mqsys::MQSTDBY_NOT_PERMITTED,
    );
    pub const MQSTDBY_PERMITTED: types::MQSTDBY = types::MQSTDBY(
        mqsys::MQSTDBY_PERMITTED,
    );
    pub const MQST_BEST_EFFORT: types::MQST = types::MQST(mqsys::MQST_BEST_EFFORT);
    pub const MQST_MUST_DUP: types::MQST = types::MQST(mqsys::MQST_MUST_DUP);
    pub const MQSUBTYPE_USER: types::MQSUBTYPE = types::MQSUBTYPE(mqsys::MQSUBTYPE_USER);
    pub const MQSUBTYPE_ALL: types::MQSUBTYPE = types::MQSUBTYPE(mqsys::MQSUBTYPE_ALL);
    pub const MQSUBTYPE_API: types::MQSUBTYPE = types::MQSUBTYPE(mqsys::MQSUBTYPE_API);
    pub const MQSUBTYPE_ADMIN: types::MQSUBTYPE = types::MQSUBTYPE(
        mqsys::MQSUBTYPE_ADMIN,
    );
    pub const MQSUBTYPE_PROXY: types::MQSUBTYPE = types::MQSUBTYPE(
        mqsys::MQSUBTYPE_PROXY,
    );
    pub const MQSUB_DURABLE_AS_PARENT: types::MQSUB = types::MQSUB(
        mqsys::MQSUB_DURABLE_AS_PARENT,
    );
    pub const MQSUB_DURABLE_ALLOWED: types::MQSUB = types::MQSUB(
        mqsys::MQSUB_DURABLE_ALLOWED,
    );
    pub const MQSUB_DURABLE_INHIBITED: types::MQSUB = types::MQSUB(
        mqsys::MQSUB_DURABLE_INHIBITED,
    );
    pub const MQSUB_DURABLE_ALL: types::MQSUB_DURABILITY = types::MQSUB_DURABILITY(
        mqsys::MQSUB_DURABLE_ALL,
    );
    pub const MQSUB_DURABLE_YES: types::MQSUB_DURABILITY = types::MQSUB_DURABILITY(
        mqsys::MQSUB_DURABLE_YES,
    );
    pub const MQSUB_DURABLE_NO: types::MQSUB_DURABILITY = types::MQSUB_DURABILITY(
        mqsys::MQSUB_DURABLE_NO,
    );
    pub const MQSUS_NO: types::MQSUS = types::MQSUS(mqsys::MQSUS_NO);
    pub const MQSUS_YES: types::MQSUS = types::MQSUS(mqsys::MQSUS_YES);
    pub const MQSVC_CONTROL_Q_MGR: types::MQSVC_CONTROL = types::MQSVC_CONTROL(
        mqsys::MQSVC_CONTROL_Q_MGR,
    );
    pub const MQSVC_CONTROL_Q_MGR_START: types::MQSVC_CONTROL = types::MQSVC_CONTROL(
        mqsys::MQSVC_CONTROL_Q_MGR_START,
    );
    pub const MQSVC_CONTROL_MANUAL: types::MQSVC_CONTROL = types::MQSVC_CONTROL(
        mqsys::MQSVC_CONTROL_MANUAL,
    );
    pub const MQSVC_STATUS_STOPPED: types::MQSVC_STATUS = types::MQSVC_STATUS(
        mqsys::MQSVC_STATUS_STOPPED,
    );
    pub const MQSVC_STATUS_STARTING: types::MQSVC_STATUS = types::MQSVC_STATUS(
        mqsys::MQSVC_STATUS_STARTING,
    );
    pub const MQSVC_STATUS_RUNNING: types::MQSVC_STATUS = types::MQSVC_STATUS(
        mqsys::MQSVC_STATUS_RUNNING,
    );
    pub const MQSVC_STATUS_STOPPING: types::MQSVC_STATUS = types::MQSVC_STATUS(
        mqsys::MQSVC_STATUS_STOPPING,
    );
    pub const MQSVC_STATUS_RETRYING: types::MQSVC_STATUS = types::MQSVC_STATUS(
        mqsys::MQSVC_STATUS_RETRYING,
    );
    pub const MQSVC_TYPE_COMMAND: types::MQSVC_TYPE = types::MQSVC_TYPE(
        mqsys::MQSVC_TYPE_COMMAND,
    );
    pub const MQSVC_TYPE_SERVER: types::MQSVC_TYPE = types::MQSVC_TYPE(
        mqsys::MQSVC_TYPE_SERVER,
    );
    pub const MQSYNCPOINT_YES: types::MQSYNCPOINT = types::MQSYNCPOINT(
        mqsys::MQSYNCPOINT_YES,
    );
    pub const MQSYNCPOINT_IFPER: types::MQSYNCPOINT = types::MQSYNCPOINT(
        mqsys::MQSYNCPOINT_IFPER,
    );
    pub const MQSYSOBJ_YES: types::MQSYSOBJ = types::MQSYSOBJ(mqsys::MQSYSOBJ_YES);
    pub const MQSYSOBJ_NO: types::MQSYSOBJ = types::MQSYSOBJ(mqsys::MQSYSOBJ_NO);
    pub const MQSYSP_NO: types::MQSYSP = types::MQSYSP(mqsys::MQSYSP_NO);
    pub const MQSYSP_YES: types::MQSYSP = types::MQSYSP(mqsys::MQSYSP_YES);
    pub const MQSYSP_EXTENDED: types::MQSYSP = types::MQSYSP(mqsys::MQSYSP_EXTENDED);
    pub const MQSYSP_TYPE_INITIAL: types::MQSYSP = types::MQSYSP(
        mqsys::MQSYSP_TYPE_INITIAL,
    );
    pub const MQSYSP_TYPE_SET: types::MQSYSP = types::MQSYSP(mqsys::MQSYSP_TYPE_SET);
    pub const MQSYSP_TYPE_LOG_COPY: types::MQSYSP = types::MQSYSP(
        mqsys::MQSYSP_TYPE_LOG_COPY,
    );
    pub const MQSYSP_TYPE_LOG_STATUS: types::MQSYSP = types::MQSYSP(
        mqsys::MQSYSP_TYPE_LOG_STATUS,
    );
    pub const MQSYSP_TYPE_ARCHIVE_TAPE: types::MQSYSP = types::MQSYSP(
        mqsys::MQSYSP_TYPE_ARCHIVE_TAPE,
    );
    pub const MQSYSP_ALLOC_BLK: types::MQSYSP = types::MQSYSP(mqsys::MQSYSP_ALLOC_BLK);
    pub const MQSYSP_ALLOC_TRK: types::MQSYSP = types::MQSYSP(mqsys::MQSYSP_ALLOC_TRK);
    pub const MQSYSP_ALLOC_CYL: types::MQSYSP = types::MQSYSP(mqsys::MQSYSP_ALLOC_CYL);
    pub const MQSYSP_STATUS_BUSY: types::MQSYSP = types::MQSYSP(
        mqsys::MQSYSP_STATUS_BUSY,
    );
    pub const MQSYSP_STATUS_PREMOUNT: types::MQSYSP = types::MQSYSP(
        mqsys::MQSYSP_STATUS_PREMOUNT,
    );
    pub const MQSYSP_STATUS_AVAILABLE: types::MQSYSP = types::MQSYSP(
        mqsys::MQSYSP_STATUS_AVAILABLE,
    );
    pub const MQSYSP_STATUS_UNKNOWN: types::MQSYSP = types::MQSYSP(
        mqsys::MQSYSP_STATUS_UNKNOWN,
    );
    pub const MQSYSP_STATUS_ALLOC_ARCHIVE: types::MQSYSP = types::MQSYSP(
        mqsys::MQSYSP_STATUS_ALLOC_ARCHIVE,
    );
    pub const MQSYSP_STATUS_COPYING_BSDS: types::MQSYSP = types::MQSYSP(
        mqsys::MQSYSP_STATUS_COPYING_BSDS,
    );
    pub const MQSYSP_STATUS_COPYING_LOG: types::MQSYSP = types::MQSYSP(
        mqsys::MQSYSP_STATUS_COPYING_LOG,
    );
    pub const MQS_AVAIL_NORMAL: types::MQS_AVAIL = types::MQS_AVAIL(
        mqsys::MQS_AVAIL_NORMAL,
    );
    pub const MQS_AVAIL_ERROR: types::MQS_AVAIL = types::MQS_AVAIL(
        mqsys::MQS_AVAIL_ERROR,
    );
    pub const MQS_AVAIL_STOPPED: types::MQS_AVAIL = types::MQS_AVAIL(
        mqsys::MQS_AVAIL_STOPPED,
    );
    pub const MQS_EXPANDST_NORMAL: types::MQS_EXPANDST = types::MQS_EXPANDST(
        mqsys::MQS_EXPANDST_NORMAL,
    );
    pub const MQS_EXPANDST_FAILED: types::MQS_EXPANDST = types::MQS_EXPANDST(
        mqsys::MQS_EXPANDST_FAILED,
    );
    pub const MQS_EXPANDST_MAXIMUM: types::MQS_EXPANDST = types::MQS_EXPANDST(
        mqsys::MQS_EXPANDST_MAXIMUM,
    );
    pub const MQS_OPENMODE_NONE: types::MQS_OPENMODE = types::MQS_OPENMODE(
        mqsys::MQS_OPENMODE_NONE,
    );
    pub const MQS_OPENMODE_READONLY: types::MQS_OPENMODE = types::MQS_OPENMODE(
        mqsys::MQS_OPENMODE_READONLY,
    );
    pub const MQS_OPENMODE_UPDATE: types::MQS_OPENMODE = types::MQS_OPENMODE(
        mqsys::MQS_OPENMODE_UPDATE,
    );
    pub const MQS_OPENMODE_RECOVERY: types::MQS_OPENMODE = types::MQS_OPENMODE(
        mqsys::MQS_OPENMODE_RECOVERY,
    );
    pub const MQS_STATUS_CLOSED: types::MQS_STATUS = types::MQS_STATUS(
        mqsys::MQS_STATUS_CLOSED,
    );
    pub const MQS_STATUS_CLOSING: types::MQS_STATUS = types::MQS_STATUS(
        mqsys::MQS_STATUS_CLOSING,
    );
    pub const MQS_STATUS_OPENING: types::MQS_STATUS = types::MQS_STATUS(
        mqsys::MQS_STATUS_OPENING,
    );
    pub const MQS_STATUS_OPEN: types::MQS_STATUS = types::MQS_STATUS(
        mqsys::MQS_STATUS_OPEN,
    );
    pub const MQS_STATUS_NOTENABLED: types::MQS_STATUS = types::MQS_STATUS(
        mqsys::MQS_STATUS_NOTENABLED,
    );
    pub const MQS_STATUS_ALLOCFAIL: types::MQS_STATUS = types::MQS_STATUS(
        mqsys::MQS_STATUS_ALLOCFAIL,
    );
    pub const MQS_STATUS_OPENFAIL: types::MQS_STATUS = types::MQS_STATUS(
        mqsys::MQS_STATUS_OPENFAIL,
    );
    pub const MQS_STATUS_STGFAIL: types::MQS_STATUS = types::MQS_STATUS(
        mqsys::MQS_STATUS_STGFAIL,
    );
    pub const MQS_STATUS_DATAFAIL: types::MQS_STATUS = types::MQS_STATUS(
        mqsys::MQS_STATUS_DATAFAIL,
    );
    pub const MQTA_BLOCK: types::MQTA = types::MQTA(mqsys::MQTA_BLOCK);
    pub const MQTA_PASSTHRU: types::MQTA = types::MQTA(mqsys::MQTA_PASSTHRU);
    pub const MQTA_PROXY_SUB_FORCE: types::MQTA_PROXY = types::MQTA_PROXY(
        mqsys::MQTA_PROXY_SUB_FORCE,
    );
    pub const MQTA_PROXY_SUB_FIRSTUSE: types::MQTA_PROXY = types::MQTA_PROXY(
        mqsys::MQTA_PROXY_SUB_FIRSTUSE,
    );
    pub const MQTA_PUB_AS_PARENT: types::MQTA_PUB = types::MQTA_PUB(
        mqsys::MQTA_PUB_AS_PARENT,
    );
    pub const MQTA_PUB_INHIBITED: types::MQTA_PUB = types::MQTA_PUB(
        mqsys::MQTA_PUB_INHIBITED,
    );
    pub const MQTA_PUB_ALLOWED: types::MQTA_PUB = types::MQTA_PUB(
        mqsys::MQTA_PUB_ALLOWED,
    );
    pub const MQTA_SUB_AS_PARENT: types::MQTA_SUB = types::MQTA_SUB(
        mqsys::MQTA_SUB_AS_PARENT,
    );
    pub const MQTA_SUB_INHIBITED: types::MQTA_SUB = types::MQTA_SUB(
        mqsys::MQTA_SUB_INHIBITED,
    );
    pub const MQTA_SUB_ALLOWED: types::MQTA_SUB = types::MQTA_SUB(
        mqsys::MQTA_SUB_ALLOWED,
    );
    pub const MQTCPKEEP_NO: types::MQTCPKEEP = types::MQTCPKEEP(mqsys::MQTCPKEEP_NO);
    pub const MQTCPKEEP_YES: types::MQTCPKEEP = types::MQTCPKEEP(mqsys::MQTCPKEEP_YES);
    pub const MQTCPSTACK_SINGLE: types::MQTCPSTACK = types::MQTCPSTACK(
        mqsys::MQTCPSTACK_SINGLE,
    );
    pub const MQTCPSTACK_MULTIPLE: types::MQTCPSTACK = types::MQTCPSTACK(
        mqsys::MQTCPSTACK_MULTIPLE,
    );
    pub const MQTC_OFF: types::MQTC = types::MQTC(mqsys::MQTC_OFF);
    pub const MQTC_ON: types::MQTC = types::MQTC(mqsys::MQTC_ON);
    pub const MQTIME_UNIT_MINS: types::MQTIME = types::MQTIME(mqsys::MQTIME_UNIT_MINS);
    pub const MQTIME_UNIT_SECS: types::MQTIME = types::MQTIME(mqsys::MQTIME_UNIT_SECS);
    pub const MQTOPT_LOCAL: types::MQTOPT = types::MQTOPT(mqsys::MQTOPT_LOCAL);
    pub const MQTOPT_CLUSTER: types::MQTOPT = types::MQTOPT(mqsys::MQTOPT_CLUSTER);
    pub const MQTOPT_ALL: types::MQTOPT = types::MQTOPT(mqsys::MQTOPT_ALL);
    pub const MQTRAXSTR_NO: types::MQTRAXSTR = types::MQTRAXSTR(mqsys::MQTRAXSTR_NO);
    pub const MQTRAXSTR_YES: types::MQTRAXSTR = types::MQTRAXSTR(mqsys::MQTRAXSTR_YES);
    pub const MQTRIGGER_RESTART_NO: types::MQTRIGGER = types::MQTRIGGER(
        mqsys::MQTRIGGER_RESTART_NO,
    );
    pub const MQTRIGGER_RESTART_YES: types::MQTRIGGER = types::MQTRIGGER(
        mqsys::MQTRIGGER_RESTART_YES,
    );
    pub const MQTSCOPE_QMGR: types::MQTSCOPE = types::MQTSCOPE(mqsys::MQTSCOPE_QMGR);
    pub const MQTSCOPE_ALL: types::MQTSCOPE = types::MQTSCOPE(mqsys::MQTSCOPE_ALL);
    pub const MQTT_NONE: types::MQTT = types::MQTT(mqsys::MQTT_NONE);
    pub const MQTT_FIRST: types::MQTT = types::MQTT(mqsys::MQTT_FIRST);
    pub const MQTT_EVERY: types::MQTT = types::MQTT(mqsys::MQTT_EVERY);
    pub const MQTT_DEPTH: types::MQTT = types::MQTT(mqsys::MQTT_DEPTH);
    pub const MQTYPE_AS_SET: types::MQTYPE = types::MQTYPE(mqsys::MQTYPE_AS_SET);
    pub const MQTYPE_NULL: types::MQTYPE = types::MQTYPE(mqsys::MQTYPE_NULL);
    pub const MQTYPE_BOOLEAN: types::MQTYPE = types::MQTYPE(mqsys::MQTYPE_BOOLEAN);
    pub const MQTYPE_BYTE_STRING: types::MQTYPE = types::MQTYPE(
        mqsys::MQTYPE_BYTE_STRING,
    );
    pub const MQTYPE_INT8: types::MQTYPE = types::MQTYPE(mqsys::MQTYPE_INT8);
    pub const MQTYPE_INT16: types::MQTYPE = types::MQTYPE(mqsys::MQTYPE_INT16);
    pub const MQTYPE_INT32: types::MQTYPE = types::MQTYPE(mqsys::MQTYPE_INT32);
    pub const MQTYPE_INT64: types::MQTYPE = types::MQTYPE(mqsys::MQTYPE_INT64);
    pub const MQTYPE_FLOAT32: types::MQTYPE = types::MQTYPE(mqsys::MQTYPE_FLOAT32);
    pub const MQTYPE_FLOAT64: types::MQTYPE = types::MQTYPE(mqsys::MQTYPE_FLOAT64);
    pub const MQTYPE_STRING: types::MQTYPE = types::MQTYPE(mqsys::MQTYPE_STRING);
    pub const MQTYPE_LONG: types::MQTYPE = types::MQTYPE(mqsys::MQTYPE_LONG);
    pub const MQUCI_NO: types::MQUCI = types::MQUCI(mqsys::MQUCI_NO);
    pub const MQUCI_YES: types::MQUCI = types::MQUCI(mqsys::MQUCI_YES);
    pub const MQUIDSUPP_NO: types::MQUIDSUPP = types::MQUIDSUPP(mqsys::MQUIDSUPP_NO);
    pub const MQUIDSUPP_YES: types::MQUIDSUPP = types::MQUIDSUPP(mqsys::MQUIDSUPP_YES);
    pub const MQUNDELIVERED_NORMAL: types::MQUNDELIVERED = types::MQUNDELIVERED(
        mqsys::MQUNDELIVERED_NORMAL,
    );
    pub const MQUNDELIVERED_SAFE: types::MQUNDELIVERED = types::MQUNDELIVERED(
        mqsys::MQUNDELIVERED_SAFE,
    );
    pub const MQUNDELIVERED_DISCARD: types::MQUNDELIVERED = types::MQUNDELIVERED(
        mqsys::MQUNDELIVERED_DISCARD,
    );
    pub const MQUNDELIVERED_KEEP: types::MQUNDELIVERED = types::MQUNDELIVERED(
        mqsys::MQUNDELIVERED_KEEP,
    );
    pub const MQUOWST_NONE: types::MQUOWST = types::MQUOWST(mqsys::MQUOWST_NONE);
    pub const MQUOWST_ACTIVE: types::MQUOWST = types::MQUOWST(mqsys::MQUOWST_ACTIVE);
    pub const MQUOWST_PREPARED: types::MQUOWST = types::MQUOWST(mqsys::MQUOWST_PREPARED);
    pub const MQUOWST_UNRESOLVED: types::MQUOWST = types::MQUOWST(
        mqsys::MQUOWST_UNRESOLVED,
    );
    pub const MQUOWT_Q_MGR: types::MQUOWT = types::MQUOWT(mqsys::MQUOWT_Q_MGR);
    pub const MQUOWT_CICS: types::MQUOWT = types::MQUOWT(mqsys::MQUOWT_CICS);
    pub const MQUOWT_RRS: types::MQUOWT = types::MQUOWT(mqsys::MQUOWT_RRS);
    pub const MQUOWT_IMS: types::MQUOWT = types::MQUOWT(mqsys::MQUOWT_IMS);
    pub const MQUOWT_XA: types::MQUOWT = types::MQUOWT(mqsys::MQUOWT_XA);
    pub const MQUSAGE_DS_OLDEST_ACTIVE_UOW: types::MQUSAGE_DS = types::MQUSAGE_DS(
        mqsys::MQUSAGE_DS_OLDEST_ACTIVE_UOW,
    );
    pub const MQUSAGE_DS_OLDEST_PS_RECOVERY: types::MQUSAGE_DS = types::MQUSAGE_DS(
        mqsys::MQUSAGE_DS_OLDEST_PS_RECOVERY,
    );
    pub const MQUSAGE_DS_OLDEST_CF_RECOVERY: types::MQUSAGE_DS = types::MQUSAGE_DS(
        mqsys::MQUSAGE_DS_OLDEST_CF_RECOVERY,
    );
    pub const MQUSAGE_EXPAND_USER: types::MQUSAGE_EXPAND = types::MQUSAGE_EXPAND(
        mqsys::MQUSAGE_EXPAND_USER,
    );
    pub const MQUSAGE_EXPAND_SYSTEM: types::MQUSAGE_EXPAND = types::MQUSAGE_EXPAND(
        mqsys::MQUSAGE_EXPAND_SYSTEM,
    );
    pub const MQUSAGE_EXPAND_NONE: types::MQUSAGE_EXPAND = types::MQUSAGE_EXPAND(
        mqsys::MQUSAGE_EXPAND_NONE,
    );
    pub const MQUSAGE_PS_AVAILABLE: types::MQUSAGE_PS = types::MQUSAGE_PS(
        mqsys::MQUSAGE_PS_AVAILABLE,
    );
    pub const MQUSAGE_PS_DEFINED: types::MQUSAGE_PS = types::MQUSAGE_PS(
        mqsys::MQUSAGE_PS_DEFINED,
    );
    pub const MQUSAGE_PS_OFFLINE: types::MQUSAGE_PS = types::MQUSAGE_PS(
        mqsys::MQUSAGE_PS_OFFLINE,
    );
    pub const MQUSAGE_PS_NOT_DEFINED: types::MQUSAGE_PS = types::MQUSAGE_PS(
        mqsys::MQUSAGE_PS_NOT_DEFINED,
    );
    pub const MQUSAGE_PS_SUSPENDED: types::MQUSAGE_PS = types::MQUSAGE_PS(
        mqsys::MQUSAGE_PS_SUSPENDED,
    );
    pub const MQUSAGE_SMDS_AVAILABLE: types::MQUSAGE_SMDS = types::MQUSAGE_SMDS(
        mqsys::MQUSAGE_SMDS_AVAILABLE,
    );
    pub const MQUSAGE_SMDS_NO_DATA: types::MQUSAGE_SMDS = types::MQUSAGE_SMDS(
        mqsys::MQUSAGE_SMDS_NO_DATA,
    );
    pub const MQUSEDLQ_AS_PARENT: types::MQUSEDLQ = types::MQUSEDLQ(
        mqsys::MQUSEDLQ_AS_PARENT,
    );
    pub const MQUSEDLQ_NO: types::MQUSEDLQ = types::MQUSEDLQ(mqsys::MQUSEDLQ_NO);
    pub const MQUSEDLQ_YES: types::MQUSEDLQ = types::MQUSEDLQ(mqsys::MQUSEDLQ_YES);
    pub const MQUSRC_MAP: types::MQUSRC = types::MQUSRC(mqsys::MQUSRC_MAP);
    pub const MQUSRC_NOACCESS: types::MQUSRC = types::MQUSRC(mqsys::MQUSRC_NOACCESS);
    pub const MQUSRC_CHANNEL: types::MQUSRC = types::MQUSRC(mqsys::MQUSRC_CHANNEL);
    pub const MQUS_NORMAL: types::MQUS = types::MQUS(mqsys::MQUS_NORMAL);
    pub const MQUS_TRANSMISSION: types::MQUS = types::MQUS(mqsys::MQUS_TRANSMISSION);
    pub const MQVL_NULL_TERMINATED: types::MQVL = types::MQVL(
        mqsys::MQVL_NULL_TERMINATED,
    );
    pub const MQVL_EMPTY_STRING: types::MQVL = types::MQVL(mqsys::MQVL_EMPTY_STRING);
    pub const MQVS_NULL_TERMINATED: types::MQVS = types::MQVS(
        mqsys::MQVS_NULL_TERMINATED,
    );
    pub const MQVU_FIXED_USER: types::MQVU = types::MQVU(mqsys::MQVU_FIXED_USER);
    pub const MQVU_ANY_USER: types::MQVU = types::MQVU(mqsys::MQVU_ANY_USER);
    pub const MQWARN_NO: types::MQWARN = types::MQWARN(mqsys::MQWARN_NO);
    pub const MQWARN_YES: types::MQWARN = types::MQWARN(mqsys::MQWARN_YES);
    pub const MQWIH_NONE: types::MQWIH = types::MQWIH(mqsys::MQWIH_NONE);
    pub const MQWI_UNLIMITED: types::MQWI = types::MQWI(mqsys::MQWI_UNLIMITED);
    pub const MQWS_DEFAULT: types::MQWS = types::MQWS(mqsys::MQWS_DEFAULT);
    pub const MQWS_CHAR: types::MQWS = types::MQWS(mqsys::MQWS_CHAR);
    pub const MQWS_TOPIC: types::MQWS = types::MQWS(mqsys::MQWS_TOPIC);
    pub const MQWXP_PUT_BY_CLUSTER_CHL: types::MQWXP = types::MQWXP(
        mqsys::MQWXP_PUT_BY_CLUSTER_CHL,
    );
    pub const MQXACT_EXTERNAL: types::MQXACT = types::MQXACT(mqsys::MQXACT_EXTERNAL);
    pub const MQXACT_INTERNAL: types::MQXACT = types::MQXACT(mqsys::MQXACT_INTERNAL);
    pub const MQXCC_FAILED: types::MQXCC = types::MQXCC(mqsys::MQXCC_FAILED);
    pub const MQXCC_REQUEST_ACK: types::MQXCC = types::MQXCC(mqsys::MQXCC_REQUEST_ACK);
    pub const MQXCC_CLOSE_CHANNEL: types::MQXCC = types::MQXCC(
        mqsys::MQXCC_CLOSE_CHANNEL,
    );
    pub const MQXCC_SUPPRESS_EXIT: types::MQXCC = types::MQXCC(
        mqsys::MQXCC_SUPPRESS_EXIT,
    );
    pub const MQXCC_SEND_SEC_MSG: types::MQXCC = types::MQXCC(mqsys::MQXCC_SEND_SEC_MSG);
    pub const MQXCC_SEND_AND_REQUEST_SEC_MSG: types::MQXCC = types::MQXCC(
        mqsys::MQXCC_SEND_AND_REQUEST_SEC_MSG,
    );
    pub const MQXCC_SKIP_FUNCTION: types::MQXCC = types::MQXCC(
        mqsys::MQXCC_SKIP_FUNCTION,
    );
    pub const MQXCC_SUPPRESS_FUNCTION: types::MQXCC = types::MQXCC(
        mqsys::MQXCC_SUPPRESS_FUNCTION,
    );
    pub const MQXCC_OK: types::MQXCC = types::MQXCC(mqsys::MQXCC_OK);
    pub const MQXDR_OK: types::MQXDR = types::MQXDR(mqsys::MQXDR_OK);
    pub const MQXDR_CONVERSION_FAILED: types::MQXDR = types::MQXDR(
        mqsys::MQXDR_CONVERSION_FAILED,
    );
    pub const MQXEPO_NONE: types::MQXEPO = types::MQXEPO(mqsys::MQXEPO_NONE);
    pub const MQXE_OTHER: types::MQXE = types::MQXE(mqsys::MQXE_OTHER);
    pub const MQXE_MCA: types::MQXE = types::MQXE(mqsys::MQXE_MCA);
    pub const MQXE_MCA_SVRCONN: types::MQXE = types::MQXE(mqsys::MQXE_MCA_SVRCONN);
    pub const MQXE_COMMAND_SERVER: types::MQXE = types::MQXE(mqsys::MQXE_COMMAND_SERVER);
    pub const MQXE_MQSC: types::MQXE = types::MQXE(mqsys::MQXE_MQSC);
    pub const MQXE_MCA_CLNTCONN: types::MQXE = types::MQXE(mqsys::MQXE_MCA_CLNTCONN);
    pub const MQXF_INIT: types::MQXF = types::MQXF(mqsys::MQXF_INIT);
    pub const MQXF_TERM: types::MQXF = types::MQXF(mqsys::MQXF_TERM);
    pub const MQXF_CONN: types::MQXF = types::MQXF(mqsys::MQXF_CONN);
    pub const MQXF_CONNX: types::MQXF = types::MQXF(mqsys::MQXF_CONNX);
    pub const MQXF_DISC: types::MQXF = types::MQXF(mqsys::MQXF_DISC);
    pub const MQXF_OPEN: types::MQXF = types::MQXF(mqsys::MQXF_OPEN);
    pub const MQXF_CLOSE: types::MQXF = types::MQXF(mqsys::MQXF_CLOSE);
    pub const MQXF_PUT1: types::MQXF = types::MQXF(mqsys::MQXF_PUT1);
    pub const MQXF_PUT: types::MQXF = types::MQXF(mqsys::MQXF_PUT);
    pub const MQXF_GET: types::MQXF = types::MQXF(mqsys::MQXF_GET);
    pub const MQXF_DATA_CONV_ON_GET: types::MQXF = types::MQXF(
        mqsys::MQXF_DATA_CONV_ON_GET,
    );
    pub const MQXF_INQ: types::MQXF = types::MQXF(mqsys::MQXF_INQ);
    pub const MQXF_SET: types::MQXF = types::MQXF(mqsys::MQXF_SET);
    pub const MQXF_BEGIN: types::MQXF = types::MQXF(mqsys::MQXF_BEGIN);
    pub const MQXF_CMIT: types::MQXF = types::MQXF(mqsys::MQXF_CMIT);
    pub const MQXF_BACK: types::MQXF = types::MQXF(mqsys::MQXF_BACK);
    pub const MQXF_STAT: types::MQXF = types::MQXF(mqsys::MQXF_STAT);
    pub const MQXF_CB: types::MQXF = types::MQXF(mqsys::MQXF_CB);
    pub const MQXF_CTL: types::MQXF = types::MQXF(mqsys::MQXF_CTL);
    pub const MQXF_CALLBACK: types::MQXF = types::MQXF(mqsys::MQXF_CALLBACK);
    pub const MQXF_SUB: types::MQXF = types::MQXF(mqsys::MQXF_SUB);
    pub const MQXF_SUBRQ: types::MQXF = types::MQXF(mqsys::MQXF_SUBRQ);
    pub const MQXF_XACLOSE: types::MQXF = types::MQXF(mqsys::MQXF_XACLOSE);
    pub const MQXF_XACOMMIT: types::MQXF = types::MQXF(mqsys::MQXF_XACOMMIT);
    pub const MQXF_XACOMPLETE: types::MQXF = types::MQXF(mqsys::MQXF_XACOMPLETE);
    pub const MQXF_XAEND: types::MQXF = types::MQXF(mqsys::MQXF_XAEND);
    pub const MQXF_XAFORGET: types::MQXF = types::MQXF(mqsys::MQXF_XAFORGET);
    pub const MQXF_XAOPEN: types::MQXF = types::MQXF(mqsys::MQXF_XAOPEN);
    pub const MQXF_XAPREPARE: types::MQXF = types::MQXF(mqsys::MQXF_XAPREPARE);
    pub const MQXF_XARECOVER: types::MQXF = types::MQXF(mqsys::MQXF_XARECOVER);
    pub const MQXF_XAROLLBACK: types::MQXF = types::MQXF(mqsys::MQXF_XAROLLBACK);
    pub const MQXF_XASTART: types::MQXF = types::MQXF(mqsys::MQXF_XASTART);
    pub const MQXF_AXREG: types::MQXF = types::MQXF(mqsys::MQXF_AXREG);
    pub const MQXF_AXUNREG: types::MQXF = types::MQXF(mqsys::MQXF_AXUNREG);
    pub const MQXPT_ALL: types::MQXPT = types::MQXPT(mqsys::MQXPT_ALL);
    pub const MQXPT_LOCAL: types::MQXPT = types::MQXPT(mqsys::MQXPT_LOCAL);
    pub const MQXPT_LU62: types::MQXPT = types::MQXPT(mqsys::MQXPT_LU62);
    pub const MQXPT_TCP: types::MQXPT = types::MQXPT(mqsys::MQXPT_TCP);
    pub const MQXPT_NETBIOS: types::MQXPT = types::MQXPT(mqsys::MQXPT_NETBIOS);
    pub const MQXPT_SPX: types::MQXPT = types::MQXPT(mqsys::MQXPT_SPX);
    pub const MQXPT_DECNET: types::MQXPT = types::MQXPT(mqsys::MQXPT_DECNET);
    pub const MQXPT_UDP: types::MQXPT = types::MQXPT(mqsys::MQXPT_UDP);
    pub const MQXR2_DEFAULT_CONTINUATION: types::MQXR2 = types::MQXR2(
        mqsys::MQXR2_DEFAULT_CONTINUATION,
    );
    pub const MQXR2_PUT_WITH_DEF_USERID: types::MQXR2 = types::MQXR2(
        mqsys::MQXR2_PUT_WITH_DEF_USERID,
    );
    pub const MQXR2_PUT_WITH_MSG_USERID: types::MQXR2 = types::MQXR2(
        mqsys::MQXR2_PUT_WITH_MSG_USERID,
    );
    pub const MQXR2_USE_EXIT_BUFFER: types::MQXR2 = types::MQXR2(
        mqsys::MQXR2_USE_EXIT_BUFFER,
    );
    pub const MQXR2_CONTINUE_CHAIN: types::MQXR2 = types::MQXR2(
        mqsys::MQXR2_CONTINUE_CHAIN,
    );
    pub const MQXR2_SUPPRESS_CHAIN: types::MQXR2 = types::MQXR2(
        mqsys::MQXR2_SUPPRESS_CHAIN,
    );
    pub const MQXR2_DYNAMIC_CACHE: types::MQXR2 = types::MQXR2(
        mqsys::MQXR2_DYNAMIC_CACHE,
    );
    pub const MQXR2_PUT_WITH_DEF_ACTION: types::MQXR2 = types::MQXR2(
        mqsys::MQXR2_PUT_WITH_DEF_ACTION,
    );
    pub const MQXR2_STATIC_CACHE: types::MQXR2 = types::MQXR2(mqsys::MQXR2_STATIC_CACHE);
    pub const MQXR2_USE_AGENT_BUFFER: types::MQXR2 = types::MQXR2(
        mqsys::MQXR2_USE_AGENT_BUFFER,
    );
    pub const MQXR_BEFORE: types::MQXR = types::MQXR(mqsys::MQXR_BEFORE);
    pub const MQXR_AFTER: types::MQXR = types::MQXR(mqsys::MQXR_AFTER);
    pub const MQXR_CONNECTION: types::MQXR = types::MQXR(mqsys::MQXR_CONNECTION);
    pub const MQXR_BEFORE_CONVERT: types::MQXR = types::MQXR(mqsys::MQXR_BEFORE_CONVERT);
    pub const MQXR_INIT: types::MQXR = types::MQXR(mqsys::MQXR_INIT);
    pub const MQXR_TERM: types::MQXR = types::MQXR(mqsys::MQXR_TERM);
    pub const MQXR_MSG: types::MQXR = types::MQXR(mqsys::MQXR_MSG);
    pub const MQXR_XMIT: types::MQXR = types::MQXR(mqsys::MQXR_XMIT);
    pub const MQXR_SEC_MSG: types::MQXR = types::MQXR(mqsys::MQXR_SEC_MSG);
    pub const MQXR_INIT_SEC: types::MQXR = types::MQXR(mqsys::MQXR_INIT_SEC);
    pub const MQXR_RETRY: types::MQXR = types::MQXR(mqsys::MQXR_RETRY);
    pub const MQXR_AUTO_CLUSSDR: types::MQXR = types::MQXR(mqsys::MQXR_AUTO_CLUSSDR);
    pub const MQXR_AUTO_RECEIVER: types::MQXR = types::MQXR(mqsys::MQXR_AUTO_RECEIVER);
    pub const MQXR_CLWL_OPEN: types::MQXR = types::MQXR(mqsys::MQXR_CLWL_OPEN);
    pub const MQXR_CLWL_PUT: types::MQXR = types::MQXR(mqsys::MQXR_CLWL_PUT);
    pub const MQXR_CLWL_MOVE: types::MQXR = types::MQXR(mqsys::MQXR_CLWL_MOVE);
    pub const MQXR_CLWL_REPOS: types::MQXR = types::MQXR(mqsys::MQXR_CLWL_REPOS);
    pub const MQXR_CLWL_REPOS_MOVE: types::MQXR = types::MQXR(
        mqsys::MQXR_CLWL_REPOS_MOVE,
    );
    pub const MQXR_END_BATCH: types::MQXR = types::MQXR(mqsys::MQXR_END_BATCH);
    pub const MQXR_ACK_RECEIVED: types::MQXR = types::MQXR(mqsys::MQXR_ACK_RECEIVED);
    pub const MQXR_AUTO_SVRCONN: types::MQXR = types::MQXR(mqsys::MQXR_AUTO_SVRCONN);
    pub const MQXR_AUTO_CLUSRCVR: types::MQXR = types::MQXR(mqsys::MQXR_AUTO_CLUSRCVR);
    pub const MQXR_SEC_PARMS: types::MQXR = types::MQXR(mqsys::MQXR_SEC_PARMS);
    pub const MQXR_PUBLICATION: types::MQXR = types::MQXR(mqsys::MQXR_PUBLICATION);
    pub const MQXR_PRECONNECT: types::MQXR = types::MQXR(mqsys::MQXR_PRECONNECT);
    pub const MQXT_API_CROSSING_EXIT: types::MQXT = types::MQXT(
        mqsys::MQXT_API_CROSSING_EXIT,
    );
    pub const MQXT_API_EXIT: types::MQXT = types::MQXT(mqsys::MQXT_API_EXIT);
    pub const MQXT_CHANNEL_SEC_EXIT: types::MQXT = types::MQXT(
        mqsys::MQXT_CHANNEL_SEC_EXIT,
    );
    pub const MQXT_CHANNEL_MSG_EXIT: types::MQXT = types::MQXT(
        mqsys::MQXT_CHANNEL_MSG_EXIT,
    );
    pub const MQXT_CHANNEL_SEND_EXIT: types::MQXT = types::MQXT(
        mqsys::MQXT_CHANNEL_SEND_EXIT,
    );
    pub const MQXT_CHANNEL_RCV_EXIT: types::MQXT = types::MQXT(
        mqsys::MQXT_CHANNEL_RCV_EXIT,
    );
    pub const MQXT_CHANNEL_MSG_RETRY_EXIT: types::MQXT = types::MQXT(
        mqsys::MQXT_CHANNEL_MSG_RETRY_EXIT,
    );
    pub const MQXT_CHANNEL_AUTO_DEF_EXIT: types::MQXT = types::MQXT(
        mqsys::MQXT_CHANNEL_AUTO_DEF_EXIT,
    );
    pub const MQXT_CLUSTER_WORKLOAD_EXIT: types::MQXT = types::MQXT(
        mqsys::MQXT_CLUSTER_WORKLOAD_EXIT,
    );
    pub const MQXT_PUBSUB_ROUTING_EXIT: types::MQXT = types::MQXT(
        mqsys::MQXT_PUBSUB_ROUTING_EXIT,
    );
    pub const MQXT_PUBLISH_EXIT: types::MQXT = types::MQXT(mqsys::MQXT_PUBLISH_EXIT);
    pub const MQXT_PRECONNECT_EXIT: types::MQXT = types::MQXT(
        mqsys::MQXT_PRECONNECT_EXIT,
    );
    pub const MQZAET_NONE: types::MQZAET = types::MQZAET(mqsys::MQZAET_NONE);
    pub const MQZAET_PRINCIPAL: types::MQZAET = types::MQZAET(mqsys::MQZAET_PRINCIPAL);
    pub const MQZAET_GROUP: types::MQZAET = types::MQZAET(mqsys::MQZAET_GROUP);
    pub const MQZAET_UNKNOWN: types::MQZAET = types::MQZAET(mqsys::MQZAET_UNKNOWN);
    pub const MQZAO_NONE: types::MQZAO = types::MQZAO(mqsys::MQZAO_NONE);
    pub const MQZAO_CONNECT: types::MQZAO = types::MQZAO(mqsys::MQZAO_CONNECT);
    pub const MQZAO_BROWSE: types::MQZAO = types::MQZAO(mqsys::MQZAO_BROWSE);
    pub const MQZAO_INPUT: types::MQZAO = types::MQZAO(mqsys::MQZAO_INPUT);
    pub const MQZAO_OUTPUT: types::MQZAO = types::MQZAO(mqsys::MQZAO_OUTPUT);
    pub const MQZAO_INQUIRE: types::MQZAO = types::MQZAO(mqsys::MQZAO_INQUIRE);
    pub const MQZAO_SET: types::MQZAO = types::MQZAO(mqsys::MQZAO_SET);
    pub const MQZAO_PASS_IDENTITY_CONTEXT: types::MQZAO = types::MQZAO(
        mqsys::MQZAO_PASS_IDENTITY_CONTEXT,
    );
    pub const MQZAO_PASS_ALL_CONTEXT: types::MQZAO = types::MQZAO(
        mqsys::MQZAO_PASS_ALL_CONTEXT,
    );
    pub const MQZAO_SET_IDENTITY_CONTEXT: types::MQZAO = types::MQZAO(
        mqsys::MQZAO_SET_IDENTITY_CONTEXT,
    );
    pub const MQZAO_SET_ALL_CONTEXT: types::MQZAO = types::MQZAO(
        mqsys::MQZAO_SET_ALL_CONTEXT,
    );
    pub const MQZAO_ALTERNATE_USER_AUTHORITY: types::MQZAO = types::MQZAO(
        mqsys::MQZAO_ALTERNATE_USER_AUTHORITY,
    );
    pub const MQZAO_PUBLISH: types::MQZAO = types::MQZAO(mqsys::MQZAO_PUBLISH);
    pub const MQZAO_SUBSCRIBE: types::MQZAO = types::MQZAO(mqsys::MQZAO_SUBSCRIBE);
    pub const MQZAO_RESUME: types::MQZAO = types::MQZAO(mqsys::MQZAO_RESUME);
    pub const MQZAO_ALL_MQI: types::MQZAO = types::MQZAO(mqsys::MQZAO_ALL_MQI);
    pub const MQZAO_CREATE: types::MQZAO = types::MQZAO(mqsys::MQZAO_CREATE);
    pub const MQZAO_DELETE: types::MQZAO = types::MQZAO(mqsys::MQZAO_DELETE);
    pub const MQZAO_DISPLAY: types::MQZAO = types::MQZAO(mqsys::MQZAO_DISPLAY);
    pub const MQZAO_CHANGE: types::MQZAO = types::MQZAO(mqsys::MQZAO_CHANGE);
    pub const MQZAO_CLEAR: types::MQZAO = types::MQZAO(mqsys::MQZAO_CLEAR);
    pub const MQZAO_CONTROL: types::MQZAO = types::MQZAO(mqsys::MQZAO_CONTROL);
    pub const MQZAO_CONTROL_EXTENDED: types::MQZAO = types::MQZAO(
        mqsys::MQZAO_CONTROL_EXTENDED,
    );
    pub const MQZAO_AUTHORIZE: types::MQZAO = types::MQZAO(mqsys::MQZAO_AUTHORIZE);
    pub const MQZAO_ALL_ADMIN: types::MQZAO = types::MQZAO(mqsys::MQZAO_ALL_ADMIN);
    pub const MQZAO_REMOVE: types::MQZAO = types::MQZAO(mqsys::MQZAO_REMOVE);
    pub const MQZAO_SYSTEM: types::MQZAO = types::MQZAO(mqsys::MQZAO_SYSTEM);
    pub const MQZAO_ALL: types::MQZAO = types::MQZAO(mqsys::MQZAO_ALL);
    pub const MQZAO_CREATE_ONLY: types::MQZAO = types::MQZAO(mqsys::MQZAO_CREATE_ONLY);
    pub const MQZAT_INITIAL_CONTEXT: types::MQZAT = types::MQZAT(
        mqsys::MQZAT_INITIAL_CONTEXT,
    );
    pub const MQZAT_CHANGE_CONTEXT: types::MQZAT = types::MQZAT(
        mqsys::MQZAT_CHANGE_CONTEXT,
    );
    pub const MQZCI_CONTINUE: types::MQZCI = types::MQZCI(mqsys::MQZCI_CONTINUE);
    pub const MQZCI_STOP: types::MQZCI = types::MQZCI(mqsys::MQZCI_STOP);
    pub const MQZCI_DEFAULT: types::MQZCI = types::MQZCI(mqsys::MQZCI_DEFAULT);
    pub const MQZID_INIT: types::MQZID = types::MQZID(mqsys::MQZID_INIT);
    pub const MQZID_TERM: types::MQZID = types::MQZID(mqsys::MQZID_TERM);
    pub const MQZID_INIT_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_INIT_AUTHORITY,
    );
    pub const MQZID_TERM_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_TERM_AUTHORITY,
    );
    pub const MQZID_CHECK_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_CHECK_AUTHORITY,
    );
    pub const MQZID_COPY_ALL_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_COPY_ALL_AUTHORITY,
    );
    pub const MQZID_DELETE_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_DELETE_AUTHORITY,
    );
    pub const MQZID_SET_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_SET_AUTHORITY,
    );
    pub const MQZID_GET_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_GET_AUTHORITY,
    );
    pub const MQZID_GET_EXPLICIT_AUTHORITY: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_GET_EXPLICIT_AUTHORITY,
    );
    pub const MQZID_REFRESH_CACHE: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_REFRESH_CACHE,
    );
    pub const MQZID_ENUMERATE_AUTHORITY_DATA: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_ENUMERATE_AUTHORITY_DATA,
    );
    pub const MQZID_AUTHENTICATE_USER: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_AUTHENTICATE_USER,
    );
    pub const MQZID_FREE_USER: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_FREE_USER,
    );
    pub const MQZID_INQUIRE: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_INQUIRE,
    );
    pub const MQZID_CHECK_PRIVILEGED: types::MQZID_AUTHORITY = types::MQZID_AUTHORITY(
        mqsys::MQZID_CHECK_PRIVILEGED,
    );
    pub const MQZID_INIT_NAME: types::MQZID_NAME = types::MQZID_NAME(
        mqsys::MQZID_INIT_NAME,
    );
    pub const MQZID_TERM_NAME: types::MQZID_NAME = types::MQZID_NAME(
        mqsys::MQZID_TERM_NAME,
    );
    pub const MQZID_LOOKUP_NAME: types::MQZID_NAME = types::MQZID_NAME(
        mqsys::MQZID_LOOKUP_NAME,
    );
    pub const MQZID_INSERT_NAME: types::MQZID_NAME = types::MQZID_NAME(
        mqsys::MQZID_INSERT_NAME,
    );
    pub const MQZID_DELETE_NAME: types::MQZID_NAME = types::MQZID_NAME(
        mqsys::MQZID_DELETE_NAME,
    );
    pub const MQZID_INIT_USERID: types::MQZID_USERID = types::MQZID_USERID(
        mqsys::MQZID_INIT_USERID,
    );
    pub const MQZID_TERM_USERID: types::MQZID_USERID = types::MQZID_USERID(
        mqsys::MQZID_TERM_USERID,
    );
    pub const MQZID_FIND_USERID: types::MQZID_USERID = types::MQZID_USERID(
        mqsys::MQZID_FIND_USERID,
    );
    pub const MQZIO_PRIMARY: types::MQZIO = types::MQZIO(mqsys::MQZIO_PRIMARY);
    pub const MQZIO_SECONDARY: types::MQZIO = types::MQZIO(mqsys::MQZIO_SECONDARY);
    pub const MQZSE_CONTINUE: types::MQZSE = types::MQZSE(mqsys::MQZSE_CONTINUE);
    pub const MQZSE_START: types::MQZSE = types::MQZSE(mqsys::MQZSE_START);
    pub const MQZSL_NOT_RETURNED: types::MQZSL = types::MQZSL(mqsys::MQZSL_NOT_RETURNED);
    pub const MQZSL_RETURNED: types::MQZSL = types::MQZSL(mqsys::MQZSL_RETURNED);
    pub const MQZTO_PRIMARY: types::MQZTO = types::MQZTO(mqsys::MQZTO_PRIMARY);
    pub const MQZTO_SECONDARY: types::MQZTO = types::MQZTO(mqsys::MQZTO_SECONDARY);
    pub const MQ_CERT_VAL_POLICY_ANY: types::MQ_CERT = types::MQ_CERT(
        mqsys::MQ_CERT_VAL_POLICY_ANY,
    );
    pub const MQ_CERT_VAL_POLICY_RFC5280: types::MQ_CERT = types::MQ_CERT(
        mqsys::MQ_CERT_VAL_POLICY_RFC5280,
    );
    pub const MQ_CERT_VAL_POLICY_NONE: types::MQ_CERT = types::MQ_CERT(
        mqsys::MQ_CERT_VAL_POLICY_NONE,
    );
    pub const MQ_CERT_VAL_POLICY_DEFAULT: types::MQ_CERT = types::MQ_CERT(
        mqsys::MQ_CERT_VAL_POLICY_DEFAULT,
    );
    pub const MQ_HTTPSCERTREV_DEFAULT: types::MQ_HTTPSCERTREV = types::MQ_HTTPSCERTREV(
        mqsys::MQ_HTTPSCERTREV_DEFAULT,
    );
    pub const MQ_HTTPSCERTREV_REQUIRED: types::MQ_HTTPSCERTREV = types::MQ_HTTPSCERTREV(
        mqsys::MQ_HTTPSCERTREV_REQUIRED,
    );
    pub const MQ_HTTPSCERTREV_DISABLED: types::MQ_HTTPSCERTREV = types::MQ_HTTPSCERTREV(
        mqsys::MQ_HTTPSCERTREV_DISABLED,
    );
    pub const MQ_HTTPSCERTREV_OPTIONAL: types::MQ_HTTPSCERTREV = types::MQ_HTTPSCERTREV(
        mqsys::MQ_HTTPSCERTREV_OPTIONAL,
    );
    pub const MQ_HTTPSCERTVAL_DEFAULT: types::MQ_HTTPSCERTVAL = types::MQ_HTTPSCERTVAL(
        mqsys::MQ_HTTPSCERTVAL_DEFAULT,
    );
    pub const MQ_HTTPSCERTVAL_ANY: types::MQ_HTTPSCERTVAL = types::MQ_HTTPSCERTVAL(
        mqsys::MQ_HTTPSCERTVAL_ANY,
    );
    pub const MQ_HTTPSCERTVAL_NONE: types::MQ_HTTPSCERTVAL = types::MQ_HTTPSCERTVAL(
        mqsys::MQ_HTTPSCERTVAL_NONE,
    );
    pub const MQ_HTTPSCERTVAL_HOSTNAMECN: types::MQ_HTTPSCERTVAL = types::MQ_HTTPSCERTVAL(
        mqsys::MQ_HTTPSCERTVAL_HOSTNAMECN,
    );
    pub const MQ_MQTT_MAX_KEEP_ALIVE: types::MQ_MQTT = types::MQ_MQTT(
        mqsys::MQ_MQTT_MAX_KEEP_ALIVE,
    );
    pub const MQ_SUITE_B_NOT_AVAILABLE: types::MQ_SUITE = types::MQ_SUITE(
        mqsys::MQ_SUITE_B_NOT_AVAILABLE,
    );
    pub const MQ_SUITE_B_NONE: types::MQ_SUITE = types::MQ_SUITE(mqsys::MQ_SUITE_B_NONE);
    pub const MQ_SUITE_B_128_BIT: types::MQ_SUITE = types::MQ_SUITE(
        mqsys::MQ_SUITE_B_128_BIT,
    );
    pub const MQ_SUITE_B_192_BIT: types::MQ_SUITE = types::MQ_SUITE(
        mqsys::MQ_SUITE_B_192_BIT,
    );
    pub const MQ_SUITE_B_SIZE: types::MQ_SUITE = types::MQ_SUITE(mqsys::MQ_SUITE_B_SIZE);
}
