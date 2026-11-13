pub fn is_enabled(name: &str) -> bool {
    std::env::var("CARGO_FEATURE_".to_string() + name.to_uppercase().as_str()).is_ok()
}

/// A list of build inputs together with the features they are gated behind.
///
/// An entry without features is always included.
pub struct FeatureFilter<'a, T> {
    pub sources: &'a [T],
    pub features: Option<&'a [&'a str]>,
}

/// Filter iterator by selected build feature
pub fn filtered<'a, T: 'a>(features: impl IntoIterator<Item = &'a FeatureFilter<'a, T>>) -> impl Iterator<Item = &'a T> {
    features
        .into_iter()
        .filter(|filter| filter.features.is_none_or(|names| names.iter().copied().all(is_enabled)))
        .flat_map(|filter| filter.sources)
}
