use crate::{
    lookup::{ConstLookup, ConstSource, ConstantItem, LinearSource},
    mapping,
};

use libmqm_sys::lib as sys;

use super::types;
use super::value::impl_default_value;

macro_rules! impl_partialcmp_value {
    ($new_type:path, [$($other_type:path),*]) => {
        $(
            impl_partialcmp_value!($new_type, $other_type);
        )*
    };
    ($new_type:path, $other_type:path) => {
        impl PartialEq<$other_type> for $new_type {
            fn eq(&self, other: &$other_type) -> bool {
                other.0 == self.0
            }
        }

        impl PartialOrd<$other_type> for $new_type {
            fn partial_cmp(&self, other: &$other_type) -> Option<std::cmp::Ordering> {
                Some(self.0.cmp(&other.0))
            }
        }
    };
}

// impl_value!(types::MQIND);
// impl_value!(types::MQQT);
// impl_value!(types::MQAT);
// impl_value!(types::MQCMD);
impl_default_value!(types::MQCMD, sys::MQCMD_NONE);
// impl_value!(types::MQCFOP);
// impl_value!(types::Selector);
impl_partialcmp_value!(
    types::Selector,
    [
        types::MQIA,
        types::MQCA,
        types::MQIACF,
        types::MQCACF,
        types::MQIACH,
        types::MQCACH,
        types::MQIASY,
        types::MQHA
    ]
);
impl_partialcmp_value!(types::MQIA, types::Selector);
impl_partialcmp_value!(types::MQCA, types::Selector);
impl_partialcmp_value!(types::MQIACF, types::Selector);
impl_partialcmp_value!(types::MQCACF, types::Selector);
impl_partialcmp_value!(types::MQIACH, types::Selector);
impl_partialcmp_value!(types::MQCACH, types::Selector);
impl_partialcmp_value!(types::MQIASY, types::Selector);
impl_partialcmp_value!(types::MQHA, types::Selector);

impl_default_value!(types::MQIND, sys::MQIND_NONE);
// impl_bitflags!(types::MQCBO);
// impl_value!(types::MQITEM);

/*

MQAI selector constant lookup is complex... thanks to this - no less than 8 different constant sets.
https://www.ibm.com/docs/en/ibm-mq/latest?topic=reference-mqai-selectors

It would be more efficient to generate one large set as part of the build process, but this will do for now.

*/

const FIRST_LAST_MAPSTR: LinearSource = ConstSource(
    &[
        (sys::MQBA_FIRST, "MQBA_FIRST"),
        (sys::MQBA_LAST, "MQBA_LAST"),
        (sys::MQGA_FIRST, "MQGA_FIRST"),
        (sys::MQGA_LAST, "MQGA_LAST"),
        (sys::MQOA_FIRST, "MQOA_FIRST"),
        (sys::MQOA_LAST, "MQOA_LAST"),
        (sys::MQUA_FIRST, "MQUA_FIRST"),
        (sys::MQUA_LAST, "MQUA_LAST"),
    ],
    &[],
);

impl ConstLookup for crate::mapping::SelectorLookup {
    fn by_value(&self, value: sys::MQLONG) -> impl Iterator<Item = &str> {
        mapping::MQIA_MAPSTR
            .by_value(value)
            .chain(mapping::MQCA_MAPSTR.by_value(value))
            .chain(mapping::MQIACF_MAPSTR.by_value(value))
            .chain(mapping::MQCACF_MAPSTR.by_value(value))
            .chain(mapping::MQIACH_MAPSTR.by_value(value))
            .chain(mapping::MQCACH_MAPSTR.by_value(value))
            .chain(mapping::MQIASY_MAPSTR.by_value(value))
            .chain(mapping::MQHA_MAPSTR.by_value(value))
            .chain(FIRST_LAST_MAPSTR.by_value(value))
    }

    fn by_name(&self, name: &str) -> Option<sys::MQLONG> {
        mapping::MQIA_MAPSTR
            .by_name(name)
            .or_else(|| mapping::MQCA_MAPSTR.by_name(name))
            .or_else(|| mapping::MQIACF_MAPSTR.by_name(name))
            .or_else(|| mapping::MQCACF_MAPSTR.by_name(name))
            .or_else(|| mapping::MQIACH_MAPSTR.by_name(name))
            .or_else(|| mapping::MQCACH_MAPSTR.by_name(name))
            .or_else(|| mapping::MQIASY_MAPSTR.by_name(name))
            .or_else(|| mapping::MQHA_MAPSTR.by_name(name))
            .or_else(|| FIRST_LAST_MAPSTR.by_name(name))
    }

    fn all(&self) -> impl Iterator<Item = ConstantItem> {
        mapping::MQIA_MAPSTR
            .all()
            .chain(mapping::MQCA_MAPSTR.all())
            .chain(mapping::MQIACF_MAPSTR.all())
            .chain(mapping::MQCACF_MAPSTR.all())
            .chain(mapping::MQIACH_MAPSTR.all())
            .chain(mapping::MQCACH_MAPSTR.all())
            .chain(mapping::MQIASY_MAPSTR.all())
            .chain(mapping::MQHA_MAPSTR.all())
            .chain(FIRST_LAST_MAPSTR.all())
    }
}
