#![allow(clippy::allow_attributes)] // reason = "Macro include 'allow' for generation purposes"

use std::borrow::Cow;

use libmqm_sys::lib::MQLONG;

macro_rules! impl_default_value {
    ($t:path, $source:path) => {
        impl Default for $t {
            fn default() -> Self {
                Self::from($source)
            }
        }
    };
}
pub(crate) use impl_default_value;

macro_rules! define_value {
    ($vis:vis $i:ident, $source:path) => {
        define_value!($vis $i, $source, "");
    };
    ($vis:vis $i:ident, $source:path, $lit:literal) => {
        #[allow(unused_imports)]
        use $crate::lookup::{HasConstLookup as _, ConstLookup as _, HasMqNames as _};

        #[allow(clippy::empty_docs,non_camel_case_types)]
        #[doc = $lit]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, derive_more::From)]
        #[repr(transparent)]
        $vis struct $i(pub libmqm_sys::lib::MQLONG);

        #[allow(dead_code)]
        impl $i {
            #[must_use]
            pub const fn value(&self) -> libmqm_sys::lib::MQLONG {
                self.0
            }
        }

        impl $crate::lookup::HasConstLookup for $i {
            fn const_lookup<'a>() -> &'a (impl $crate::lookup::ConstLookup + 'static) {
                &$source
            }
        }

        impl std::str::FromStr for $i {
            type Err = <libmqm_sys::lib::MQLONG as std::str::FromStr>::Err;

            fn from_str(name: &str) -> Result<Self, Self::Err> {
                Ok(Self(
                    Self::const_lookup().by_name(name).map_or_else(|| std::str::FromStr::from_str(name), Ok)?,
                ))
            }
        }

        impl $crate::lookup::MqConstant for $i {
            fn mq_value(&self) -> libmqm_sys::lib::MQLONG {
                let Self(value) = self;
                *value
            }
        }

        impl std::fmt::Display for $i {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let Self(attribute) = self;
                $crate::value::value_display(*attribute, self.mq_primary_name(), f)
            }
        }

        impl std::fmt::Debug for $i {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let Self(attribute) = self;
                $crate::value::value_debug(stringify!($i), *attribute, self.mq_names(), f)
            }
        }
    };
}
pub(crate) use define_value;

pub fn value_display(value: MQLONG, primary_name: Option<&str>, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let code = primary_name.map_or_else(|| Cow::from(value.to_string()), Cow::from);
    f.write_str(&code)
}

pub fn value_debug(
    type_name: &str,
    value: MQLONG,
    names: impl Iterator<Item = &'static str>,
    f: &mut std::fmt::Formatter,
) -> std::fmt::Result {
    let names_str = names.map(Cow::from).reduce(|acc, name| Cow::from(format!("{acc}|{name}")));
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

    const LOOKUP: &[ConstantItem] = &[(0, "ZERO"), (0, "ZERO_ALIAS"), (1, "ONE"), (1, "ONE_ALIAS")];

    define_value!(pub X, LOOKUP);

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
