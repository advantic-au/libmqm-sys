use libmqm_sys as mq;

use crate::constants;
use crate::lookup::{ConstLookup, ConstSource, ConstantItem, HasMqNames as _, LinearSource};
use crate::mapping;
use crate::types;

use super::value::{impl_default_value, impl_equivalent_type};

#[cfg(feature = "pcf")]
mod pcf {
    use crate::types;
    use crate::value::impl_equivalent_type;

    impl_equivalent_type!(types::MQIACF, types::Selector);
    impl_equivalent_type!(types::MQCACF, types::Selector);
    impl_equivalent_type!(types::MQIACH, types::Selector);
    impl_equivalent_type!(types::MQCACH, types::Selector);

    impl_equivalent_type!(types::Selector, [types::MQIACF, types::MQCACF, types::MQIACH, types::MQCACH]);
}

#[cfg(feature = "mqai")]
mod mqia {
    use crate::value::{impl_default_value, impl_equivalent_type};
    use crate::{constants, types};

    impl_default_value!(types::MQIND, constants::MQIND_NONE);

    impl_equivalent_type!(types::MQIASY, types::Selector);
    impl_equivalent_type!(types::MQHA, types::Selector);
    impl_equivalent_type!(types::Selector, [types::MQIASY, types::MQHA]);
}

impl_equivalent_type!(types::MQIA, types::Selector);
impl_equivalent_type!(types::MQCA, types::Selector);
impl_equivalent_type!(types::Selector, [types::MQIA, types::MQCA]);

impl_default_value!(types::MQCO, constants::MQCO_NONE);
impl_default_value!(types::MQBO, constants::MQBO_NONE);
impl_default_value!(types::MQTYPE, constants::MQTYPE_AS_SET);
impl_default_value!(types::MQENC, constants::MQENC_NATIVE);
impl_default_value!(types::MQGMO, constants::MQGMO_NONE);
impl_default_value!(types::MQPMO, constants::MQPMO_NONE);
impl_default_value!(types::MQCMHO, constants::MQCMHO_DEFAULT_VALIDATION);
impl_default_value!(types::MQSMPO, constants::MQSMPO_SET_FIRST);
impl_default_value!(types::MQDMPO, constants::MQDMPO_DEL_FIRST);
impl_default_value!(types::MQIMPO, constants::MQIMPO_NONE);
#[cfg(feature = "exits")]
impl_default_value!(types::MQDCC, constants::MQDCC_NONE);
#[cfg(feature = "pcf")]
impl_default_value!(types::MQCMD, constants::MQCMD_NONE);

const FIRST_LAST_MAPSTR: LinearSource = ConstSource(
    &[
        (mq::MQBA_FIRST, "MQBA_FIRST"),
        (mq::MQBA_LAST, "MQBA_LAST"),
        (mq::MQGA_FIRST, "MQGA_FIRST"),
        (mq::MQGA_LAST, "MQGA_LAST"),
        #[cfg(feature = "mqai")]
        (libmqm_sys::mqai::MQOA_FIRST, "MQOA_FIRST"),
        #[cfg(feature = "mqai")]
        (libmqm_sys::mqai::MQOA_LAST, "MQOA_LAST"),
        #[cfg(feature = "pcf")]
        (libmqm_sys::pcf::MQUA_FIRST, "MQUA_FIRST"),
        #[cfg(feature = "pcf")]
        (libmqm_sys::pcf::MQUA_LAST, "MQUA_LAST"),
    ],
    &[],
);

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

#[cfg(feature = "pcf")]
impl_equivalent_type!(types::MQRC, types::MQRCCF);

#[cfg(feature = "pcf")]
impl_equivalent_type!(types::MQRCCF, types::MQRC);

impl types::MQXA {
    #[inline]
    #[must_use]
    pub const fn is_text(&self) -> bool {
        self.0 >= constants::MQCA_FIRST.0 && self.0 <= constants::MQCA_LAST.0
    }

    #[inline]
    #[must_use]
    pub const fn is_int(&self) -> bool {
        self.0 >= constants::MQIA_FIRST.0 && self.0 <= constants::MQIA_LAST.0
    }
}

impl_equivalent_type!(types::MQXA, [types::MQCA, types::MQIA]);
impl_equivalent_type!(types::MQCA, types::MQXA);
impl_equivalent_type!(types::MQIA, types::MQXA);

/*

MQAI selector constant lookup is complex... thanks to this - no less than 8 different constant sets.
https://www.ibm.com/docs/en/ibm-mq/latest?topic=reference-mqai-selectors

It would be more efficient to generate one large set as part of the build process, but this will do for now.

*/

impl ConstLookup for crate::mapping::SelectorLookup {
    fn by_value(&self, value: mq::MQLONG) -> impl Iterator<Item = &str> {
        let mapping = mapping::MQIA_MAPSTR
            .by_value(value)
            .chain(mapping::MQCA_MAPSTR.by_value(value));

        #[cfg(feature = "pcf")]
        let mapping = mapping
            .chain(mapping::MQIACF_MAPSTR.by_value(value))
            .chain(mapping::MQCACF_MAPSTR.by_value(value))
            .chain(mapping::MQIACH_MAPSTR.by_value(value))
            .chain(mapping::MQCACH_MAPSTR.by_value(value));

        #[cfg(feature = "mqai")]
        let mapping = mapping
            .chain(mapping::MQIASY_MAPSTR.by_value(value))
            .chain(mapping::MQHA_MAPSTR.by_value(value));

        mapping.chain(FIRST_LAST_MAPSTR.by_value(value))
    }

    fn by_name(&self, name: &str) -> Option<mq::MQLONG> {
        let mapping = mapping::MQIA_MAPSTR
            .by_name(name)
            .or_else(|| mapping::MQCA_MAPSTR.by_name(name));

        #[cfg(feature = "pcf")]
        let mapping = mapping
            .or_else(|| mapping::MQIACF_MAPSTR.by_name(name))
            .or_else(|| mapping::MQCACF_MAPSTR.by_name(name))
            .or_else(|| mapping::MQIACH_MAPSTR.by_name(name))
            .or_else(|| mapping::MQCACH_MAPSTR.by_name(name));

        #[cfg(feature = "mqai")]
        let mapping = mapping
            .or_else(|| mapping::MQIASY_MAPSTR.by_name(name))
            .or_else(|| mapping::MQHA_MAPSTR.by_name(name));

        mapping.or_else(|| FIRST_LAST_MAPSTR.by_name(name))
    }

    fn all(&self) -> impl Iterator<Item = ConstantItem<'_>> {
        let mapping = mapping::MQIA_MAPSTR.all().chain(mapping::MQCA_MAPSTR.all());

        #[cfg(feature = "pcf")]
        let mapping = mapping
            .chain(mapping::MQIACF_MAPSTR.all())
            .chain(mapping::MQCACF_MAPSTR.all())
            .chain(mapping::MQIACH_MAPSTR.all())
            .chain(mapping::MQCACH_MAPSTR.all());

        #[cfg(feature = "mqai")]
        let mapping = mapping.chain(mapping::MQIASY_MAPSTR.all()).chain(mapping::MQHA_MAPSTR.all());

        mapping.chain(FIRST_LAST_MAPSTR.all())
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use crate::constants;
    use crate::types;

    #[test]
    fn reason_code_display() {
        assert_eq!(constants::MQRC_Q_MGR_ACTIVE.to_string(), "MQRC_Q_MGR_ACTIVE");
        assert_eq!(constants::MQRC_NONE.to_string(), "MQRC_NONE");
        assert_eq!(types::MQRC(-1).to_string(), "-1");
    }

    #[test]
    fn ibm_reference_url() {
        assert_eq!(
            constants::MQRC_Q_ALREADY_EXISTS.ibm_reference_url("en", None),
            Some("https://www.ibm.com/docs/en/ibm-mq/latest?topic=codes-2290-08f2-rc2290-mqrc-q-already-exists".to_owned())
        );

        #[cfg(feature = "pcf")]
        assert_eq!(
            types::MQRC::from(constants::MQRCCF_CFH_TYPE_ERROR).ibm_reference_url("en", None),
            Some("https://www.ibm.com/docs/en/ibm-mq/latest?topic=codes-3001-0bb9-rc3001-mqrccf-cfh-type-error".to_owned())
        );
    }
}
