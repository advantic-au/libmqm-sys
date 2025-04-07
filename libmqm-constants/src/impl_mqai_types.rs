use crate::{
    lookup::{ConstLookup, ConstantItem},
    mapping,
};

use libmqm_sys::lib as sys;

use super::value::impl_default_value;
use super::{mask::impl_mask, types, value::impl_value};

impl_value!(types::MQIND);
impl_value!(types::MQQT);
impl_value!(types::MQAT);
impl_value!(types::MQCMD);
impl_default_value!(types::MQCMD, sys::MQCMD_NONE);
impl_value!(types::MQCFOP);
impl_value!(types::MqaiSelector);
impl_default_value!(types::MQIND, sys::MQIND_NONE);
impl_mask!(types::MQCBO);
impl_value!(types::MQITEM);

/*

MQAI selector constant lookup is complex... thanks to this - no less than 8 different constant sets.
https://www.ibm.com/docs/en/ibm-mq/latest?topic=reference-mqai-selectors

It would be more efficient to generate one large set as part of the build process, but this will do for now.

*/

impl ConstLookup for crate::mapping::MqaiSelectorLookup {
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
    }
}
