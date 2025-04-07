#![allow(clippy::allow_attributes)] // reason = "Macro include 'allow' for generation purposes"

use std::borrow::Cow;

use super::lookup::{ConstLookup, ConstantItem};

macro_rules! impl_mask {
    ($name:path) => {
        #[allow(unused_imports)]
        use $crate::lookup::{ConstLookup as _, HasConstLookup as _, HasMqNames as _};

        impl std::str::FromStr for $name {
            type Err = <libmqm_sys::lib::MQLONG as std::str::FromStr>::Err;

            fn from_str(name: &str) -> Result<Self, Self::Err> {
                Ok(Self(
                    Self::const_lookup()
                        .by_name(name)
                        .map_or_else(|| std::str::FromStr::from_str(name), Ok)?,
                ))
            }
        }

        impl $name {
            pub fn masked_list(
                &self,
            ) -> (
                impl Iterator<Item = $crate::lookup::ConstantItem<'static>>,
                ::libmqm_sys::lib::MQLONG,
            ) {
                let &Self(val) = self;
                $crate::mask::masked_list(val, Self::const_lookup().all())
            }

            fn mask_str<'a>(
                list: impl Iterator<Item = $crate::lookup::ConstantItem<'a>>,
                residual: ::libmqm_sys::lib::MQLONG,
            ) -> Option<std::borrow::Cow<'a, str>> {
                $crate::mask::mask_str(Self::const_lookup(), list, residual)
            }
        }

        impl<Y: Into<::libmqm_sys::lib::MQLONG>> std::ops::BitOr<Y> for $name {
            type Output = Self;

            fn bitor(self, rhs: Y) -> Self::Output {
                Self(self.0 | rhs.into())
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl<Y: Into<::libmqm_sys::lib::MQLONG>> std::ops::BitOrAssign<Y> for $name {
            fn bitor_assign(&mut self, rhs: Y) {
                self.0 |= rhs.into();
            }
        }

        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl<Y: Into<::libmqm_sys::lib::MQLONG>> std::ops::BitAnd<Y> for $name {
            type Output = Self;

            fn bitand(self, rhs: Y) -> Self::Output {
                Self(self.0 & rhs.into())
            }
        }

        impl std::ops::BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl<Y: Into<::libmqm_sys::lib::MQLONG>> std::ops::BitAndAssign<Y> for $name {
            fn bitand_assign(&mut self, rhs: Y) {
                self.0 &= rhs.into();
            }
        }

        impl std::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        // Format of Display is 'CONSTANT_A|CONSTANT_B|(residual number))'
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let (list_iter, residual) = self.masked_list();
                match Self::mask_str(list_iter, residual) {
                    Some(mask_str) => f.write_str(&mask_str),
                    None => f.write_str(&format!("{:#X}", self.0)),
                }
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                $crate::mask::mask_debug(stringify!($i), self.0, Self::const_lookup(), f)
            }
        }
    };
}

pub(crate) use impl_mask;

pub fn mask_debug(
    type_name: &str,
    value: libmqm_sys::lib::MQLONG,
    lookup: &impl ConstLookup,
    f: &mut std::fmt::Formatter,
) -> std::fmt::Result {
    let (list, residual) = masked_list(value, lookup.all());
    if residual == value && residual != 0 {
        f.debug_tuple(type_name).field(&format_args!("{value:#X}")).finish()
    } else {
        match mask_str(lookup, list, residual) {
            Some(mask_str) => f
                .debug_tuple(type_name)
                .field(&format_args!("{mask_str} = {value:#X}"))
                .finish(),
            _ => f.debug_tuple(type_name).field(&format_args!("{value:#X}")).finish(),
        }
    }
}

pub fn masked_list<'a>(
    value: libmqm_sys::lib::MQLONG,
    source: impl Iterator<Item = ConstantItem<'a>>,
) -> (impl Iterator<Item = ConstantItem<'a>>, libmqm_sys::lib::MQLONG) {
    let mut mask_list = Vec::new();
    let residual = source
        .into_iter()
        .filter(|(value, name)| *value != 0 && !name.ends_with("_MASK"))
        .fold(value, |acc, item @ (val, ..)| {
            let masked = value & val;
            if masked == val {
                mask_list.push(item);
                acc & !masked
            } else {
                acc
            }
        });
    (mask_list.into_iter(), residual)
}

pub fn mask_str<'a>(
    lookup: &'a impl ConstLookup,
    list: impl Iterator<Item = ConstantItem<'a>>,
    residual: libmqm_sys::lib::MQLONG,
) -> Option<Cow<'a, str>> {
    let res_cow = (residual != 0).then(|| Cow::from(format!("{residual:#X}")));
    let list = list.map(|(.., name)| Cow::from(name)).chain(res_cow);
    list.reduce(|mut acc, name| {
        let acc_mut = acc.to_mut();
        acc_mut.push('|');
        acc_mut.push_str(&name);
        acc
    })
    .or_else(|| lookup.by_value(residual).next().map(Cow::from))
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod test {
    use crate::value::define_new_type;
    use ::libmqm_sys::lib as sys;

    use super::*;

    const ONEB: &[ConstantItem] = &[
        (0, "ZERO"),
        (0, "ZERO_ALIAS"),
        (0b1, "ONE"),
        (0b1, "ONEB"),
        (0b1, "ONE_MASK"),
        (0b10, "TWO"),
    ];
    define_new_type!(MaskOne, sys::MQLONG, ONEB);
    impl_mask!(MaskOne);
    const NO_ZERO: &[ConstantItem] = &[(1, "ONE")];
    define_new_type!(NoZero, sys::MQLONG, NO_ZERO);
    impl_mask!(NoZero);

    #[test]
    fn mask_type() {
        let mut one = MaskOne::from(1);
        let two = (one & MaskOne::from(2)) | 7;
        one |= MaskOne::from(2);
        one |= 2;

        let one_copy = one;
        assert_eq!(one, one_copy);
        assert_eq!(two, MaskOne::from(7));
    }

    #[test]
    fn mask_debug() {
        assert_eq!(format!("{:?}", MaskOne::from(1)), "MaskOne(ONE|ONEB = 0x1)");
        assert_eq!(format!("{:?}", MaskOne::from(0)), "MaskOne(ZERO = 0x0)");
        assert_eq!(format!("{:?}", MaskOne::from(0b101)), "MaskOne(ONE|ONEB|0x4 = 0x5)");
        assert_eq!(format!("{:?}", MaskOne::from(0b100)), "MaskOne(0x4)");
        assert_eq!(format!("{:?}", NoZero::from(0)), "NoZero(0x0)");
    }
}
