use super::list;

use std::collections::{HashMap, HashSet};
use std::ffi::CStr;
use std::str;

use libmqm_sys::lib as mqsys;

const CONST_IGNORE: &[&str] = &[
    r".+_CURRENT_LENGTH.*",
    r".+_STRUC.*_LENGTH.*",
    r".+_LENGTH_\d+.*",
    r".+_VERSION_\d+.*",
    r".+_CURRENT_VERSION.*",
    r"^MQ_.+_LEN(GTH)?.*",
];

pub fn const_ignore_regex() -> impl Iterator<Item = regex_lite::Regex> {
    CONST_IGNORE
        .iter()
        .map(|r| regex_lite::Regex::new(r).expect("regex should compile"))
}

// Load the `MQI_BY_NAME_STR` into a Vec
pub fn by_name(by_name_mqi: &[mqsys::MQI_BY_NAME_STR]) -> impl Iterator<Item = (&str, i32)> {
    by_name_mqi
        .iter()
        .map(|entry| {
            (
                unsafe { str::from_utf8_unchecked(CStr::from_ptr(entry.name).to_bytes()) },
                entry.value,
            )
        })
        .filter(|(name, ..)| !name.is_empty())
}

/// Load the `MQI_BY_VALUE_STR` into a Vec
fn by_value(by_value_mqi: &[mqsys::MQI_BY_VALUE_STR]) -> impl Iterator<Item = (i32, &str)> {
    by_value_mqi
        .iter()
        .map(|entry| {
            (entry.value, unsafe {
                str::from_utf8_unchecked(CStr::from_ptr(entry.name).to_bytes())
            })
        })
        .filter(|(.., name)| !name.is_empty())
}

pub fn as_array(by_value: &[(mqsys::MQLONG, &str, Option<&str>)]) -> String {
    use std::fmt::Write as _;
    let mut result = String::new();
    result.push('[');
    for (value, name, _) in by_value {
        let _ = write!(result, "({value},\"{name}\"),");
    }
    result.push(']');
    result
}

pub fn as_phf(by_value: &[(mqsys::MQLONG, &str, Option<&str>)]) -> String {
    let mut phf_set = phf_codegen::Map::new();
    for (value, name, _) in by_value {
        phf_set.entry(*value, &format!("\"{name}\""));
    }
    phf_set.build().to_string()
}

pub fn generate_constants<F, E>(f: F) -> Result<(), E>
where
    F: FnOnce(
        &[(
            &str,
            (
                Option<&str>,
                &str,
                &str,
                &str,
                Option<&str>,
                &[(i32, &str, Option<&str>)],
                Vec<(i32, &str, Option<&str>)>,
            ),
        )],
    ) -> Result<(), E>,
{
    let by_value_mqi = unsafe { &mqsys::MQI_BY_VALUE_STR };
    let by_value: Vec<_> = by_value(by_value_mqi).collect();

    let doc_map = list::CONSTANTS_DOC.iter().copied().collect::<HashMap<_, _>>();

    // Gather the list of constants for each prefix by using
    // the _STR c functions and CONSTANTS which was derived from
    // the header file
    let primary_constants = list::CONSTANTS
        .iter()
        .copied()
        .map(
            |list::ConstantEntry {
                 prefix,
                 new_type,
                 str_fn,
                 orig_type,
                 usage,
                 feature,
                 doc,
                 ..
             }| {
                let mut by_value_set: Vec<_> = by_value
                    .iter()
                    .copied()
                    .filter(|(value, name)| unsafe {
                        str::from_utf8_unchecked(std::ffi::CStr::from_ptr(str_fn(*value)).to_bytes()) == *name
                    })
                    .map(|(v, n)| (v, n, doc_map.get(n).copied()))
                    .collect();
                by_value_set.sort_by_key(|(k, ..)| *k);
                (prefix, (feature, new_type, orig_type, usage, doc, by_value_set))
            },
        )
        .chain(
            list::PREFIX_CONSTANTS
                .iter()
                .map(|(prefix, new_type, orig_type, usage, feature, doc)| {
                    (
                        *prefix,
                        (
                            *feature,
                            *new_type,
                            *orig_type,
                            *usage,
                            *doc,
                            by_value
                                .iter()
                                .copied()
                                .filter(|(_, name)| name.starts_with(prefix))
                                .map(|(v, n)| (v, n, doc_map.get(n).copied()))
                                .collect(),
                        ),
                    )
                }),
        )
        .collect::<HashMap<_, _>>();

    // Collect a list of constants that are assigned to a prefix
    let primary_set = primary_constants
        .values()
        .flat_map(|(.., v)| v)
        .map(|(.., name, _)| *name)
        .collect::<HashSet<_>>();

    let unassigned_filter: Vec<_> = const_ignore_regex().collect();

    // List of unassigned constants
    let unassigned_constants = by_value
        .iter()
        .copied()
        .filter(|(.., name)| !primary_set.contains(name))
        .filter(|(.., name)| {
            // Ignore some constants that are used for MQI structures
            // and ranges
            !unassigned_filter.iter().any(|r| r.is_match(name))
        })
        .collect::<Vec<_>>();

    // Create a map of primary and extra constants
    let mut prefix_constants = primary_constants
        .iter()
        .map(|(prefix, (feature, new_type, orig_type, usage, doc, .., primary))| {
            // Similar prefixes ie prefixes that start with another prefix.
            // This need to be excluded from the "extra" list
            let similar: HashSet<_> = primary_constants
                .iter()
                .filter_map(|(&other_prefix, ..)| {
                    (*prefix != other_prefix && other_prefix.starts_with(prefix)).then_some(other_prefix)
                })
                .collect();
            // 'extra' are the constants that were _not_ yielded from the _STR c functions
            // They are still useful
            let extra: Vec<_> = unassigned_constants
                .iter()
                .filter(|(.., name)| {
                    name.starts_with(prefix) && !similar.iter().any(|other_prefix| name.starts_with(other_prefix))
                })
                .map(|(v, n)| (*v, *n, doc_map.get(*n).copied()))
                .collect();
            (*prefix, (*feature, *new_type, *orig_type, *usage, *doc, &**primary, extra))
        })
        .collect::<Vec<_>>();

    prefix_constants.sort_by_key(|(prefix, ..)| *prefix);

    f(&prefix_constants)
}
