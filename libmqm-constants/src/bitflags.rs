#![allow(clippy::allow_attributes)] // reason = "Macro include 'allow' for generation purposes"

use std::borrow::Cow;

use super::lookup::{ConstLookup, ConstantItem};

macro_rules! impl_bitflags {
    ($name:path, $type:path) => {
        $crate::bitflags::impl_bitflags!($name);
    };
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
            pub fn bitflags_list(
                &self,
            ) -> (
                impl Iterator<Item = $crate::lookup::ConstantItem<'static>>,
                ::libmqm_sys::lib::MQLONG,
            ) {
                let &Self(val) = self;
                $crate::bitflags::bitflags_list(val, Self::const_lookup().all())
            }

            #[must_use]
            #[allow(dead_code)]
            pub const fn contains(self, other: Self) -> bool {
                self.0 & other.0 == other.0
            }

            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }

            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }

            #[must_use]
            #[allow(dead_code)]
            pub const fn intersects(self, other: Self) -> bool {
                self.0 & other.0 != 0
            }

            #[must_use]
            pub const fn complement(self) -> Self {
                Self(!self.0)
            }

            pub const fn insert(&mut self, other: Self) {
                self.0 |= other.0;
            }

            pub const fn remove(&mut self, other: Self) {
                self.0 &= !other.0;
            }

            pub const fn toggle(&mut self, other: Self) {
                self.0 ^= other.0;
            }

            fn bitflags_str<'a>(
                list: impl Iterator<Item = $crate::lookup::ConstantItem<'a>>,
                residual: ::libmqm_sys::lib::MQLONG,
            ) -> Option<std::borrow::Cow<'a, str>> {
                $crate::bitflags::bitflags_str(Self::const_lookup(), list, residual)
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                self.union(rhs)
            }
        }

        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.insert(rhs);
            }
        }

        impl std::ops::BitXor for $name {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitXorAssign for $name {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.toggle(rhs);
            }
        }

        impl std::ops::BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                self.intersection(rhs)
            }
        }

        impl std::ops::Not for $name {
            type Output = Self;

            fn not(self) -> Self::Output {
                self.complement()
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                self.intersection(rhs.complement())
            }
        }

        impl std::ops::SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                self.remove(rhs);
            }
        }

        impl FromIterator<Self> for $name {
            fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
                iter.into_iter().fold(Self(0), Self::union )
            }
        }        

        // Format of Display is 'CONSTANT_A|CONSTANT_B|(residual number))'
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let (list_iter, residual) = self.bitflags_list();
                match Self::bitflags_str(list_iter, residual) {
                    Some(bitflags_str) => f.write_str(&bitflags_str),
                    None => f.write_str(&format!("{:#X}", self.0)),
                }
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                $crate::bitflags::bitflags_debug(stringify!($name), self.0, Self::const_lookup(), f)
            }
        }

        impl Extend<Self> for $name {
            fn extend<T: IntoIterator<Item = Self>>(&mut self, iter: T) {
                for flags in iter {
                    self.insert(flags);
                }
            }
        }
    };
}

pub(crate) use impl_bitflags;

pub fn bitflags_debug(
    type_name: &str,
    value: libmqm_sys::lib::MQLONG,
    lookup: &impl ConstLookup,
    f: &mut std::fmt::Formatter,
) -> std::fmt::Result {
    let (list, residual) = bitflags_list(value, lookup.all());
    if residual == value && residual != 0 {
        f.debug_tuple(type_name).field(&format_args!("{value:#X}")).finish()
    } else {
        match bitflags_str(lookup, list, residual) {
            Some(bitflags_str) => f
                .debug_tuple(type_name)
                .field(&format_args!("{bitflags_str} = {value:#X}"))
                .finish(),
            _ => f.debug_tuple(type_name).field(&format_args!("{value:#X}")).finish(),
        }
    }
}

pub fn bitflags_list<'a>(
    value: libmqm_sys::lib::MQLONG,
    source: impl Iterator<Item = ConstantItem<'a>>,
) -> (impl Iterator<Item = ConstantItem<'a>>, libmqm_sys::lib::MQLONG) {
    let mut bitflags_list = Vec::new();
    let residual = source
        .into_iter()
        .filter(|(value, name)| *value != 0 && !name.ends_with("_MASK"))
        .fold(value, |acc, item @ (val, ..)| {
            let masked = value & val;
            if masked == val {
                bitflags_list.push(item);
                acc & !masked
            } else {
                acc
            }
        });
    (bitflags_list.into_iter(), residual)
}

pub fn bitflags_str<'a>(
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
    impl_bitflags!(MaskOne);
    const NO_ZERO: &[ConstantItem] = &[(1, "ONE")];
    define_new_type!(NoZero, sys::MQLONG, NO_ZERO);
    impl_bitflags!(NoZero);

    #[test]
    fn bitflags_type() {
        let mut one = MaskOne(1);
        let two = (one & MaskOne(2)) | MaskOne(7);
        one |= MaskOne(2);

        let one_copy = one;
        assert_eq!(one, one_copy);
        assert_eq!(two, MaskOne(7));
    }

    #[test]
    fn bitflags_debug() {
        assert_eq!(format!("{:?}", MaskOne(1)), "MaskOne(ONE|ONEB = 0x1)");
        assert_eq!(format!("{:?}", MaskOne(0)), "MaskOne(ZERO = 0x0)");
        assert_eq!(format!("{:?}", MaskOne(0b101)), "MaskOne(ONE|ONEB|0x4 = 0x5)");
        assert_eq!(format!("{:?}", MaskOne(0b100)), "MaskOne(0x4)");
        assert_eq!(format!("{:?}", NoZero(0)), "NoZero(0x0)");
    }
}
