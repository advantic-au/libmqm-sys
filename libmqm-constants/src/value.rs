#![allow(clippy::allow_attributes)] // reason = "Macro include 'allow' for generation purposes"

use std::borrow::Cow;

macro_rules! impl_default_value {
    ($t:path, $source:path) => {
        impl Default for $t {
            fn default() -> Self {
                $source
            }
        }
    };
}
pub(crate) use impl_default_value;

macro_rules! define_new_type {
    ($vis:vis $name:ident, $type:ty, $mapping:path) => {
        $crate::value::define_new_type!($vis $name, $type, $mapping, "");
    };
    ($vis:vis $name:ident, $type:ty, $mapping:path, $doc:literal) => {
        #[repr(transparent)]
        #[derive(
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            ::derive_more::From,
            ::derive_more::UpperHex,
            ::derive_more::LowerHex,
            ::derive_more::Binary
        )]
        #[allow(clippy::empty_docs,non_camel_case_types)]
        #[doc = $doc]
        $vis struct $name(pub $type);

        impl PartialEq<$type> for $name {
            fn eq(&self, other: &$type) -> bool {
                self.0 == *other
            }
        }

        impl AsRef<$name> for $type {
            fn as_ref(&self) -> &$name {
                // SAFETY: repr(transparent) ensures new type has same memory layout
                unsafe { &*std::ptr::from_ref(self).cast() }
            }
        }

        impl AsMut<$name> for $type {
            fn as_mut(&mut self) -> &mut $name {
                // SAFETY: repr(transparent) ensures new type has same memory layout
                unsafe { &mut *std::ptr::from_mut(self).cast() }
            }
        }

        impl AsRef<$type> for $name {
            fn as_ref(&self) -> &$type {
                &self.0
            }
        }

        impl AsMut<$type> for $name {
            fn as_mut(&mut self) -> &mut $type {
                &mut self.0
            }
        }

        impl $crate::lookup::HasConstLookup for $name {
            fn const_lookup<'a>() -> &'a (impl $crate::lookup::ConstLookup + 'static) {
                &$mapping
            }
        }

    };
}
pub(crate) use define_new_type;

macro_rules! impl_value {
    ($new_type:path) => {
        impl_value!($new_type, ::libmqm_sys::lib::MQLONG);
    };
    ($new_type:path, $orig_type:ty) => {
        #[allow(unused_imports)]
        use $crate::lookup::{ConstLookup as _, HasConstLookup as _, HasMqNames as _};

        impl std::str::FromStr for $new_type {
            type Err = <$orig_type as std::str::FromStr>::Err;

            fn from_str(name: &str) -> Result<Self, Self::Err> {
                Ok($new_type(
                    Self::const_lookup()
                        .by_name(name)
                        .map(Into::into)
                        .map_or_else(|| std::str::FromStr::from_str(name), Ok)?,
                ))
            }
        }

        impl $crate::lookup::MqConstant for $new_type {
            type Value = $orig_type;
            fn mq_value(&self) -> $orig_type {
                let $new_type(value) = self;
                *value
            }
        }

        impl std::fmt::Display for $new_type {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let $new_type(attribute) = self;
                $crate::value::value_display(attribute, self.mq_primary_name(), f)
            }
        }

        impl std::fmt::Debug for $new_type {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let $new_type(attribute) = self;
                $crate::value::value_debug(stringify!($new_type), *attribute, self.mq_names(), f)
            }
        }

        impl PartialOrd for $new_type {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $new_type {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl PartialOrd<$orig_type> for $new_type {
            fn partial_cmp(&self, other: &$orig_type) -> Option<std::cmp::Ordering> {
                Some(self.0.cmp(other))
            }
        }
    };
}
pub(crate) use impl_value;

macro_rules! impl_equivalent_type {
    ($new_type:path, [$($other_type:path),*]) => {
        $(
            impl_equivalent_type!($new_type, $other_type);
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

        impl From<$other_type> for $new_type {
            fn from(value: $other_type) -> Self {
                Self(value.0)
            }
        }

        impl AsRef<$new_type> for $other_type {
            fn as_ref(&self) -> &$new_type {
                // SAFETY: repr(transparent) ensures new type has same memory layout
                unsafe { &*std::ptr::from_ref(self).cast() }
            }
        }

        impl AsMut<$new_type> for $other_type {
            fn as_mut(&mut self) -> &mut $new_type {
                // SAFETY: repr(transparent) ensures new type has same memory layout
                unsafe { &mut *std::ptr::from_mut(self).cast() }
            }
        }
    };
}
pub(crate) use impl_equivalent_type;

pub fn value_display<T: ToString>(value: &T, primary_name: Option<&str>, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let code = primary_name.map_or_else(|| Cow::Owned(value.to_string()), Cow::Borrowed);
    f.write_str(&code)
}

pub fn value_debug<T: std::fmt::Display>(
    type_name: &str,
    value: T,
    names: impl Iterator<Item = &'static str>,
    f: &mut std::fmt::Formatter,
) -> std::fmt::Result {
    let names_str = names
        .map(Cow::Borrowed)
        .reduce(|acc, name| Cow::Owned(format!("{acc}|{name}")));
    if let Some(name_list) = names_str {
        f.debug_tuple(type_name)
            .field(&format_args!("{name_list} = {value}"))
            .finish()
    } else {
        f.debug_tuple(type_name).field(&format_args!("{value}")).finish()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod test {
    use std::{error::Error, str::FromStr};

    use crate::lookup::{ConstantItem, HasMqNames as _};
    use libmqm_sys::lib as sys;

    const LOOKUP: &[ConstantItem] = &[(0, "ZERO"), (0, "ZERO_ALIAS"), (1, "ONE"), (1, "ONE_ALIAS")];

    define_new_type!(pub X, sys::MQLONG, LOOKUP);
    impl_value!(X);

    #[test]
    fn from_str() -> Result<(), Box<dyn Error>> {
        assert_eq!(X::from(0).mq_names().collect::<Vec<_>>(), &["ZERO", "ZERO_ALIAS"]);
        assert_eq!(X::from_str("0")?, X::from(0));
        assert_eq!(X::from_str("ONE")?, X::from(1));
        assert!(X::from_str("TWO").is_err());

        Ok(())
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", X::from(1)), "X(ONE|ONE_ALIAS = 1)");
        assert_eq!(format!("{:?}", X::from(0)), "X(ZERO|ZERO_ALIAS = 0)");
    }

    #[test]
    fn to_string() {
        assert_eq!(format!("{}", X::from(1)), "ONE");
        assert_eq!(format!("{}", X::from(0)), "ZERO");
        assert_eq!(format!("{}", X::from(2)), "2");
    }
}
