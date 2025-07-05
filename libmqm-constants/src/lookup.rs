use libmqm_sys::MQLONG;

use crate::mapping;

pub trait MqConstant {
    type Value;
    fn mq_value(&self) -> Self::Value;
}

pub type ConstantItem<'a> = (MQLONG, &'a str);

/// Provides an MQ lookup functions to a type
pub trait ConstLookup {
    /// All the constant names for the provided value.
    /// The first value returned by the iterator is the primary constant for the value.
    fn by_value(&self, value: MQLONG) -> impl Iterator<Item = &str>;
    /// The constant value for the provided name.
    fn by_name(&self, name: &str) -> Option<MQLONG>;
    /// The complete list of value and name constants
    fn all(&self) -> impl Iterator<Item = ConstantItem<'_>>;
}

/// MQ constant repository with primary and secondary constants
pub struct ConstSource<P, S>(pub(crate) P, pub(crate) S);
pub type PhfSource<'a> = ConstSource<&'a ::phf::Map<MQLONG, &'a str>, &'a [ConstantItem<'a>]>;
pub type LinearSource<'a> = ConstSource<&'a [ConstantItem<'a>], &'a [ConstantItem<'a>]>;
pub type BinarySearchSource<'a> = ConstSource<BinarySearch<'a>, &'a [ConstantItem<'a>]>;

/// MQ constant repository to be searched by value using binary search
pub struct BinarySearch<'a>(pub(crate) &'a [ConstantItem<'a>]);

/// Associated constant lookup table with a type
pub trait HasConstLookup {
    /// Retrieve the static constant lookup table
    fn const_lookup<'a>() -> &'a (impl ConstLookup + 'static);
}

impl<P: ConstLookup, S: ConstLookup> ConstLookup for ConstSource<P, S> {
    fn by_value(&self, value: MQLONG) -> impl Iterator<Item = &str> {
        let Self(source, extra) = self;
        source.by_value(value).chain(extra.by_value(value))
    }

    fn by_name(&self, name: &str) -> Option<MQLONG> {
        let Self(source, extra) = self;
        source.by_name(name).or_else(|| extra.by_name(name))
    }

    fn all(&self) -> impl Iterator<Item = ConstantItem<'_>> {
        let Self(source, extra) = self;
        source.all().chain(extra.all())
    }
}

impl ConstLookup for BinarySearch<'_> {
    fn by_value(&self, value: MQLONG) -> impl Iterator<Item = &str> {
        let Self(list) = self;
        list.binary_search_by_key(&value, |(value, ..)| *value)
            .map(|index| list[index].1)
            .into_iter()
    }

    fn by_name(&self, name: &str) -> Option<MQLONG> {
        let &Self(list) = self;
        list.by_name(name)
    }

    fn all(&self) -> impl Iterator<Item = ConstantItem<'_>> {
        let list = &self.0;
        list.iter().copied()
    }
}

// phf map of ConstItem
impl ConstLookup for &::phf::Map<MQLONG, &str> {
    fn by_value(&self, value: MQLONG) -> impl Iterator<Item = &str> {
        self.get(&value).copied().into_iter()
    }

    fn by_name(&self, name: &str) -> Option<MQLONG> {
        let value: Option<&MQLONG> = mapping::MQI_BY_STRING.get(name);
        value.filter(|v| self.get(v) == Some(&name)).copied()
    }

    fn all(&self) -> impl Iterator<Item = ConstantItem<'_>> {
        self.entries().map(|(&v, &n)| (v, n))
    }
}

// Ordered array of ConstItems
impl ConstLookup for &[ConstantItem<'_>] {
    fn by_value(&self, value: MQLONG) -> impl Iterator<Item = &str> {
        self.iter()
            .take_while(move |(v, ..)| *v <= value)
            .filter_map(move |(v, name)| (*v == value).then_some(*name))
    }

    fn by_name(&self, name: &str) -> Option<MQLONG> {
        self.iter().find_map(|(value, n)| (*n == name).then_some(*value))
    }

    fn all(&self) -> impl Iterator<Item = ConstantItem<'_>> {
        self.iter().copied()
    }
}

pub trait HasMqNames {
    fn mq_names(&self) -> impl Iterator<Item = &'static str>;
    fn mq_primary_name(&self) -> Option<&'static str>;
}

impl<T: MqConstant + HasConstLookup> HasMqNames for T
where
    T::Value: TryInto<MQLONG>,
{
    fn mq_names(&self) -> impl Iterator<Item = &'static str> {
        self.mq_value()
            .try_into()
            .map(|value| Self::const_lookup().by_value(value))
            .into_iter()
            .flatten()
    }
    fn mq_primary_name(&self) -> Option<&'static str> {
        self.mq_names().next()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    const ZERO: LinearSource = ConstSource(&[], &[]);
    const ONE: LinearSource = ConstSource(&[(1, "ONE")], &[]);
    const ONEB: LinearSource = ConstSource(&[(1, "ONE")], &[(1, "ONEB"), (2, "TWO")]);

    #[test]
    fn const_source() {
        assert_eq!(ZERO.by_name("TEST"), None);
        assert_eq!(ONE.by_name("ONE"), Some(1));
        assert_eq!(ONE.by_name("ZERO"), None);
        assert_eq!(ONEB.by_name("ONEB"), Some(1));
        assert_eq!(ONEB.by_name("THREE"), None);

        assert_eq!(ONEB.by_value(1).collect::<Vec<_>>(), &["ONE", "ONEB"]);
        assert_eq!(ONEB.by_value(0).collect::<Vec<_>>(), Vec::<&str>::new());
    }
}
