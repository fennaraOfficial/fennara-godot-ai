use crate::release_identity::{
    ReleaseIdentity, ReleaseSelector, ReleaseTrack, channel_pointer_asset_name, channel_pointer_ref,
};

const SOURCE_COMMIT: &str = "0123456789abcdef0123456789abcdef01234567";

#[test]
fn validates_stable_and_pull_request_staging_identities() {
    let stable = identity("stable", "0.3.9", None, None);
    assert!(ReleaseIdentity::parse(&stable, "0.3.9").is_ok());

    let staging = identity(
        "staging",
        "0.3.9-pr.101.2",
        Some("pr-101"),
        Some(SOURCE_COMMIT),
    );
    let parsed = ReleaseIdentity::parse(&staging, "0.3.9-pr.101.2").unwrap();
    assert_eq!(parsed.track, ReleaseTrack::Staging);
    assert_eq!(parsed.channel.as_deref(), Some("pr-101"));
}

#[test]
fn rejects_cross_channel_or_unidentified_prerelease_identity() {
    let wrong_channel = identity(
        "staging",
        "0.3.9-pr.125.1",
        Some("pr-101"),
        Some(SOURCE_COMMIT),
    );
    assert!(ReleaseIdentity::parse(&wrong_channel, "0.3.9-pr.125.1").is_err());

    let legacy_error = ReleaseIdentity::load(
        std::path::Path::new("path-that-does-not-exist"),
        "0.3.9-pr.101.1",
    )
    .unwrap_err();
    assert!(legacy_error.contains("requires release.json"));
}

#[test]
fn rejects_release_identity_versions_with_build_metadata() {
    let release = identity("stable", "0.3.9+build", None, None);
    let error = ReleaseIdentity::parse(&release, "0.3.9+build").unwrap_err();
    assert!(error.contains("must not contain SemVer build metadata"));
}

#[test]
fn maps_selectors_to_isolated_github_tags() {
    assert_eq!(ReleaseSelector::StableLatest.github_tag(), "latest");
    assert_eq!(
        ReleaseSelector::staging("pr-101").unwrap().github_tag(),
        "staging-pr-101"
    );
    assert_eq!(
        ReleaseSelector::from_version_request("channel:pr-101").unwrap(),
        ReleaseSelector::StagingChannel("pr-101".into())
    );
    assert_eq!(
        ReleaseSelector::exact("0.3.9-pr.101.2")
            .unwrap()
            .github_tag(),
        "v0.3.9-pr.101.2"
    );
    assert_eq!(
        channel_pointer_asset_name("pr-101").unwrap(),
        "fennara-staging-channel-pr-101.json"
    );
    assert_eq!(
        channel_pointer_ref("pr-101").unwrap(),
        "fennara-staging/pr-101"
    );
}

fn identity(
    track: &str,
    version: &str,
    channel: Option<&str>,
    source_commit: Option<&str>,
) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "schema_version": 1,
        "track": track,
        "version": version,
        "release_tag": format!("v{version}"),
        "channel": channel,
        "source_commit": source_commit,
    }))
    .unwrap()
}
