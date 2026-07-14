use semver::Version;
use std::cmp::Ordering;

pub(crate) fn parse_release_version(value: &str) -> Result<Version, String> {
    let version = Version::parse(value)
        .map_err(|error| format!("invalid release version {value:?}: {error}"))?;
    if !version.build.is_empty() {
        return Err(format!(
            "release version {value:?} must not contain SemVer build metadata"
        ));
    }
    Ok(version)
}

pub(crate) fn compare_versions(left: &str, right: &str) -> Option<Ordering> {
    Some(
        parse_release_version(left)
            .ok()?
            .cmp(&parse_release_version(right).ok()?),
    )
}
