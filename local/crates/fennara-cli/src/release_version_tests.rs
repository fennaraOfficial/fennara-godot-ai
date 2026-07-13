use crate::release_version::{compare_versions, parse_release_version};
use std::cmp::Ordering;

#[test]
fn compares_stable_and_prerelease_versions_with_semver_precedence() {
    assert_eq!(compare_versions("0.3.9", "0.3.9"), Some(Ordering::Equal));
    assert_eq!(
        compare_versions("0.3.9-pr.101.2", "0.3.9-pr.101.1"),
        Some(Ordering::Greater)
    );
    assert_eq!(
        compare_versions("0.3.9-pr.101.2", "0.3.9"),
        Some(Ordering::Less)
    );
    assert_eq!(compare_versions("0.3", "0.3.0"), None);
}

#[test]
fn rejects_build_metadata_and_invalid_numeric_prerelease_identifiers() {
    assert!(parse_release_version("0.3.9+build.1").is_err());
    assert!(parse_release_version("0.3.9-pr.0101.1").is_err());
}
