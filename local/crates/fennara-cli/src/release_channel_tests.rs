use crate::release_channel::ChannelPointer;
use crate::release_identity::ReleaseSelector;

const SOURCE_COMMIT: &str = "0123456789abcdef0123456789abcdef01234567";

#[test]
fn accepts_an_exact_pointer_for_the_requested_channel() {
    let pointer = ChannelPointer::parse(&pointer("pr-101", "0.3.9-pr.101.2"), "pr-101").unwrap();
    assert_eq!(
        pointer.exact_selector().unwrap(),
        ReleaseSelector::ExactVersion("0.3.9-pr.101.2".into())
    );
}

#[test]
fn rejects_cross_channel_and_malformed_pointers() {
    let error = ChannelPointer::parse(&pointer("pr-125", "0.3.9-pr.125.1"), "pr-101").unwrap_err();
    assert!(error.contains("does not match requested channel"));

    let mut value: serde_json::Value =
        serde_json::from_slice(&pointer("pr-101", "0.3.9-pr.101.1")).unwrap();
    value["release_manifest_sha256"] = serde_json::json!("bad");
    assert!(ChannelPointer::parse(&serde_json::to_vec(&value).unwrap(), "pr-101").is_err());
}

fn pointer(channel: &str, version: &str) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "schema_version": 1,
        "channel": channel,
        "version": version,
        "release_tag": format!("v{version}"),
        "source_commit": SOURCE_COMMIT,
        "release_manifest_sha256": "a".repeat(64),
    }))
    .unwrap()
}
