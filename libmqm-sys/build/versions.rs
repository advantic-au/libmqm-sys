use regex_lite::Regex;

const MQC_FEATURE_PATTERN: &str = r"(?i)mqc_(\d+)_(\d+)_(\d+)_(\d+)";
const VERSION_PATTERN: &str = r"(\d+)\.(\d+)\.(\d+)\.(\d+)";

fn ver_into_u32((a, b, c, d): (&str, &str, &str, &str)) -> Result<u32, std::num::ParseIntError> {
    Ok((a.parse::<u32>()? << 24) | (b.parse::<u32>()? << 16) | (c.parse::<u32>()? << 8) | (d.parse::<u32>()?))
}

pub fn parse_mqc_feature(feature: &str) -> Option<u32> {
    let mqc_regex = Regex::new(MQC_FEATURE_PATTERN).ok()?;
    let (_, version) = mqc_regex.captures(feature).map(|captures| captures.extract())?;
    ver_into_u32(version.into()).ok()
}

pub fn parse_version(version: &str) -> Option<u32> {
    let version_check = Regex::new(VERSION_PATTERN).ok()?;

    let (_, version) = version_check.captures(version).map(|captures| captures.extract())?;
    ver_into_u32(version.into()).ok()
}

pub fn generate_version(target: &mut impl std::io::Write, version: &str) -> std::io::Result<()> {
    let version_int = parse_version(version)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid version: {version}")))?;

    writeln!(target, "pub const CLIENT_BUILD_VERSION: &str = \"{version}\";")?;
    writeln!(target, "pub const CLIENT_BUILD_VERSION_INT: u32 = {version_int:#010x};")?;

    Ok(())
}
