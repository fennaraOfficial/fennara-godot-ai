use crate::release_channel::ChannelPointer;
use crate::release_client::Release;
use crate::release_update::resolved_release_track;

#[test]
fn identifies_stable_and_staging_release_targets_before_handoff() {
    let stable = Release {
        tag: "latest".into(),
        assets: serde_json::json!([]),
        channel_pointer: None,
    };
    assert_eq!(resolved_release_track(&stable), "stable");

    let staging = Release {
        tag: "staging-pr-101".into(),
        assets: serde_json::json!([]),
        channel_pointer: Some(ChannelPointer {
            schema_version: 1,
            channel: "pr-101".into(),
            version: "0.3.9-pr.101.2".into(),
            release_tag: "v0.3.9-pr.101.2".into(),
            source_commit: "0123456789abcdef0123456789abcdef01234567".into(),
            release_manifest_sha256: "1".repeat(64),
        }),
    };
    assert_eq!(resolved_release_track(&staging), "staging");
}
