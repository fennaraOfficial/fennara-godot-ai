use crate::project_install::validate_channel_selection;
use crate::release_identity::{ReleaseIdentity, ReleaseTrack};

#[test]
fn validates_requested_staging_channel() {
    let identity = staging_identity(Some("pr-101"));
    assert!(validate_channel_selection(Some(&identity), Some("pr-101")).is_ok());
    assert!(
        validate_channel_selection(Some(&identity), Some("pr-102"))
            .unwrap_err()
            .contains("belongs to pr-101")
    );
}

#[test]
fn existing_staging_channel_is_inherited_when_rerun_omits_channel() {
    let identity = staging_identity(Some("pr-101"));
    assert!(validate_channel_selection(Some(&identity), None).is_ok());
}

#[test]
fn rejects_channels_for_stable_or_incomplete_staging_identity() {
    let stable = ReleaseIdentity {
        schema_version: 1,
        track: ReleaseTrack::Stable,
        version: "0.3.9".into(),
        release_tag: "v0.3.9".into(),
        channel: None,
        source_commit: None,
    };
    assert!(
        validate_channel_selection(Some(&stable), Some("pr-101"))
            .unwrap_err()
            .contains("stable track")
    );
    assert!(
        validate_channel_selection(Some(&staging_identity(None)), None)
            .unwrap_err()
            .contains("missing its channel")
    );
}

fn staging_identity(channel: Option<&str>) -> ReleaseIdentity {
    ReleaseIdentity {
        schema_version: 1,
        track: ReleaseTrack::Staging,
        version: "0.3.9-pr.101.2".into(),
        release_tag: "v0.3.9-pr.101.2".into(),
        channel: channel.map(str::to_string),
        source_commit: Some("0123456789abcdef0123456789abcdef01234567".into()),
    }
}
