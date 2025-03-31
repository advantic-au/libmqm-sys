use libmqm_sys::lib as sys;

use super::{mapping, mask::define_mask, value::define_value, value::impl_default_value};

define_mask!(pub MQOO, mapping::MQOO_CONST, "Options mask to control the action of `MQOPEN`");
define_mask!(pub MQCO, mapping::MQCO_CONST, "Options mask to control the action of `MQCLOSE`");
impl_default_value!(MQCO, sys::MQCO_NONE);
define_mask!(pub MQBO, mapping::MQBO_CONST, "Options mask to control the action of `MQBEGIN`");
impl_default_value!(MQBO, sys::MQBO_NONE);
define_mask!(pub MQSO, mapping::MQSO_CONST, "Options mask to control the action of `MQSUB`");
define_mask!(pub MQOP, mapping::MQOP_CONST, "Operation codes for `MQCTL` and `MQCB`");
define_value!(pub MQCBCT, mapping::MQCBCT_CONST, "Callback control and message delivery call types");
define_value!(pub MQCBF, mapping::MQCBCF_CONST, "Flags containing information about the callback consumer");
define_value!(pub MQCS, mapping::MQCS_CONST, "Callback consumer state");
define_value!(pub MQRD, mapping::MQRD_CONST, "Reconnect delay");
define_value!(pub MQSR, mapping::MQSR_CONST, "Value describing action for `MQSUBRQ`");
define_mask!(pub MQSRO, mapping::MQSRO_CONST, "Options mask that control the action of MQSUBRQ");
impl_default_value!(MQSRO, sys::MQSRO_NONE);
define_value!(pub MQTYPE, mapping::MQTYPE_CONST, "Property data types");
impl_default_value!(MQTYPE, sys::MQTYPE_AS_SET);
define_mask!(pub MQENC, mapping::MQENC_CONST, "Mask describing data encoding");
impl_default_value!(MQENC, sys::MQENC_NATIVE);
define_mask!(pub MQGMO, mapping::MQGMO_CONST, "Options mask to control the action of `MQGET`");
impl_default_value!(MQGMO, sys::MQGMO_NONE);
define_mask!(pub MQPMO, mapping::MQPMO_CONST, "Options mask to control the action of `MQPUT` and `MQPUT1`");
impl_default_value!(MQPMO, sys::MQPMO_NONE);
define_value!(pub MQSTAT, mapping::MQSTAT_CONST, "Value describing the MQSTAT outcome");
define_value!(pub MQCMHO, mapping::MQCMHO_CONST, "Create message handle options for `MQCRTMH`");
impl_default_value!(MQCMHO, sys::MQCMHO_DEFAULT_VALIDATION);
define_value!(pub MQSMPO, mapping::MQSMPO_CONST, "Set message property options");
impl_default_value!(MQSMPO, sys::MQSMPO_SET_FIRST);
define_value!(pub MQDMPO, mapping::MQDMPO_CONST, "Delete message property options");
impl_default_value!(MQDMPO, sys::MQDMPO_DEL_FIRST);
define_value!(pub MQXA, mapping::MQXA_FULL_CONST, "Integer and Character attribute selectors");
define_mask!(pub MQCBDO, mapping::MQCBDO_CONST, "Options mask to control the action of `MQCB`");
define_mask!(pub MQIMPO, mapping::MQIMPO_CONST, "Options mask to control the action of `MQINQMP`");
define_mask!(pub MQMHBO, mapping::MQMHBO_CONST, "Options mask to control the action of `MQMHBUF`");
define_mask!(pub MQBMHO, mapping::MQBMHO_CONST, "Options mask to control the action of `MQBUFMH`");
impl_default_value!(MQIMPO, sys::MQIMPO_NONE);
define_value!(pub MQPD, mapping::MQPD_CONST, "Property descriptor, support and context");
define_mask!(pub MQCOPY, mapping::MQCOPY_CONST, "Property copy options mask");
define_value!(pub MQRC, mapping::MQRC_FULL_CONST, "Reason Code from an MQ function call");
define_value!(pub MQCC, mapping::MQCC_CONST, "Completion Code from an MQ function call");
define_mask!(pub MQDCC, mapping::MQDCC_CONST, "Options mask that control the action of `MQXCNVC`");
impl_default_value!(MQDCC, sys::MQDCC_NONE);

define_mask!(pub MQCNO, mapping::MQCNO_CONST, "Options mask that control the action of `MQCONNX`");
define_value!(pub MQXPT, mapping::MQXPT_CONST, "Transport Types");

define_value!(pub MQOT, mapping::MQOT_CONST, "Object Types and Extended Object Types");

#[cfg(feature = "mqc_9_4_1_0")]
define_value!(pub MQ_HTTPSCERTREV, mapping::MQ_HTTPSCERTREV_CONST, "Level of certificate revocation check that is required for HTTPS connections");
#[cfg(feature = "mqc_9_4_1_0")]
define_value!(pub MQ_HTTPSCERTVAL, mapping::MQ_HTTPSCERTVAL_CONST, "Level of certificate validation that is required for HTTPS connections");

impl MQRC {
    #[must_use]
    pub fn ibm_reference_url(&self, language: &str, version: Option<&str>) -> Option<String> {
        let name = self.mq_primary_name()?.to_lowercase().replace('_', "-");
        let version = version.unwrap_or("latest");
        let code = self.value();
        Some(format!(
            "https://www.ibm.com/docs/{language}/ibm-mq/{version}?topic=codes-{code}-{code:04x}-rc{code}-{name}"
        ))
    }
}

impl MQXA {
    #[inline]
    #[must_use]
    pub const fn is_text(&self) -> bool {
        let val = self.value();
        val >= sys::MQCA_FIRST && val <= sys::MQCA_LAST
    }

    #[inline]
    #[must_use]
    pub const fn is_int(&self) -> bool {
        let val = self.value();
        val >= sys::MQIA_FIRST && val <= sys::MQIA_LAST
    }
}
