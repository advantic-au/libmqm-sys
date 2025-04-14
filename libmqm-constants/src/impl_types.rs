use libmqm_sys::lib as sys;

use super::value::impl_default_value;
use super::{bitflags::impl_bitflags, types, value::impl_value};

impl_bitflags!(types::MQMF);
impl_bitflags!(types::MQRO);
impl_bitflags!(types::MQREGO);
impl_bitflags!(types::MQCTLO);
impl_bitflags!(types::MQMO);
impl_bitflags!(types::MQPUBO);
impl_bitflags!(types::MQZAO);
impl_bitflags!(types::MQAUTHOPT);

impl_bitflags!(types::MQOO);
impl_bitflags!(types::MQCO);
impl_default_value!(types::MQCO, sys::MQCO_NONE);
impl_bitflags!(types::MQBO);
impl_default_value!(types::MQBO, sys::MQBO_NONE);
impl_bitflags!(types::MQSO);
impl_bitflags!(types::MQOP);

impl_value!(types::MQCBCT);
impl_value!(types::MQCBCF);
impl_value!(types::MQCS);
impl_value!(types::MQRD);
impl_value!(types::MQSR);

impl_bitflags!(types::MQSRO);
impl_default_value!(types::MQSRO, sys::MQSRO_NONE);
impl_value!(types::MQTYPE);
impl_default_value!(types::MQTYPE, sys::MQTYPE_AS_SET);
impl_bitflags!(types::MQENC);
impl_default_value!(types::MQENC, sys::MQENC_NATIVE);
impl_bitflags!(types::MQGMO);
impl_default_value!(types::MQGMO, sys::MQGMO_NONE);
impl_bitflags!(types::MQPMO);
impl_default_value!(types::MQPMO, sys::MQPMO_NONE);
impl_value!(types::MQSTAT);
impl_value!(types::MQCMHO);
impl_default_value!(types::MQCMHO, sys::MQCMHO_DEFAULT_VALIDATION);
impl_value!(types::MQSMPO);
impl_default_value!(types::MQSMPO, sys::MQSMPO_SET_FIRST);
impl_value!(types::MQDMPO);
impl_default_value!(types::MQDMPO, sys::MQDMPO_DEL_FIRST);
impl_value!(types::MQXA);
impl_bitflags!(types::MQCBDO);
impl_bitflags!(types::MQIMPO);
impl_bitflags!(types::MQMHBO);
impl_bitflags!(types::MQBMHO);
impl_default_value!(types::MQIMPO, sys::MQIMPO_NONE);
impl_value!(types::MQPD);
impl_bitflags!(types::MQCOPY);
impl_value!(types::MQRC);
impl_value!(types::MQCC);
impl_bitflags!(types::MQDCC);
impl_default_value!(types::MQDCC, sys::MQDCC_NONE);

impl_bitflags!(types::MQCNO);
impl_value!(types::MQXPT);

impl_value!(types::MQOT);

#[cfg(feature = "mqc_9_4_1_0")]
impl_value!(types::MQ_HTTPSCERTREV);
#[cfg(feature = "mqc_9_4_1_0")]
impl_value!(types::MQ_HTTPSCERTVAL);

impl types::MQRC {
    #[must_use]
    pub fn ibm_reference_url(&self, language: &str, version: Option<&str>) -> Option<String> {
        let name = self.mq_primary_name()?.to_lowercase().replace('_', "-");
        let version = version.unwrap_or("latest");
        let code = self.0;
        Some(format!(
            "https://www.ibm.com/docs/{language}/ibm-mq/{version}?topic=codes-{code}-{code:04x}-rc{code}-{name}"
        ))
    }
}

impl From<types::MQCA> for types::MQXA {
    fn from(value: types::MQCA) -> Self {
        Self(value.0)
    }
}

impl From<types::MQIA> for types::MQXA {
    fn from(value: types::MQIA) -> Self {
        Self(value.0)
    }
}

impl types::MQXA {
    #[inline]
    #[must_use]
    pub const fn is_text(&self) -> bool {
        let val = self.0;
        val >= sys::MQCA_FIRST && val <= sys::MQCA_LAST
    }

    #[inline]
    #[must_use]
    pub const fn is_int(&self) -> bool {
        let val = self.0;
        val >= sys::MQIA_FIRST && val <= sys::MQIA_LAST
    }
}
