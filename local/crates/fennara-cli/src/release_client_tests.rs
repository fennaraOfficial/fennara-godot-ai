use crate::release_channel::ChannelPointer;
use crate::release_client::{
    DownloadAsset, Release, download_bytes, parse_release_manifest, release_metadata_url,
    verify_download_hash,
};
use crate::release_identity::ReleaseSelector;
use sha2::{Digest, Sha256};
use std::net::TcpListener;

#[test]
fn hash_mismatch_reports_expected_and_actual_values() {
    let asset = DownloadAsset {
        url: "https://example.invalid/fennara.zip",
        expected_sha256: Some("0000000000000000000000000000000000000000000000000000000000000000"),
        label: "fennara.zip",
    };

    let actual_sha256 = format!("{:x}", Sha256::digest(b"corrupted archive"));
    let error = verify_download_hash(&asset, &actual_sha256).unwrap_err();
    assert!(error.contains("fennara.zip sha256 mismatch"));
    assert!(
        error.contains("expected 0000000000000000000000000000000000000000000000000000000000000000")
    );
    assert!(error.contains("got "));
}

#[test]
fn download_connection_failure_is_actionable() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);

    let error =
        download_bytes(&format!("http://{address}/missing.zip"), "missing.zip").unwrap_err();
    assert!(error.contains("failed to download missing.zip"));
    assert!(error.contains("connect/read timeouts"));
}

#[test]
fn staging_pointer_binds_manifest_hash_and_identity() {
    let manifest = staging_manifest("0123456789abcdef0123456789abcdef01234567");
    let bytes = serde_json::to_vec(&manifest).unwrap();
    let sha256 = format!("{:x}", Sha256::digest(&bytes));
    let release = staging_release(&sha256);

    assert!(parse_release_manifest(&release, &bytes, &sha256, "manifest.json").is_ok());

    let mut tampered = bytes.clone();
    tampered.push(b' ');
    let tampered_sha256 = format!("{:x}", Sha256::digest(&tampered));
    let error = parse_release_manifest(&release, &tampered, &tampered_sha256, "manifest.json")
        .err()
        .unwrap();
    assert!(error.contains("sha256 mismatch"));
}

#[test]
fn staging_pointer_rejects_manifest_from_another_source_commit() {
    let manifest = staging_manifest("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    let bytes = serde_json::to_vec(&manifest).unwrap();
    let sha256 = format!("{:x}", Sha256::digest(&bytes));
    let release = staging_release(&sha256);

    let error = parse_release_manifest(&release, &bytes, &sha256, "manifest.json")
        .err()
        .unwrap();
    assert!(error.contains("identity does not match channel pointer"));
}

#[test]
fn stable_latest_uses_github_latest_while_pinned_versions_use_exact_tags() {
    assert_eq!(
        release_metadata_url(&ReleaseSelector::StableLatest),
        "https://api.github.com/repos/fennaraOfficial/fennara-godot-ai/releases/latest"
    );
    assert_eq!(
        release_metadata_url(&ReleaseSelector::exact("0.3.11").unwrap()),
        "https://api.github.com/repos/fennaraOfficial/fennara-godot-ai/releases/tags/v0.3.11"
    );
    assert_eq!(
        release_metadata_url(&ReleaseSelector::exact("0.3.11-pr.101.1").unwrap()),
        "https://api.github.com/repos/fennaraOfficial/fennara-godot-ai/releases/tags/v0.3.11-pr.101.1"
    );
}

fn staging_release(release_manifest_sha256: &str) -> Release {
    Release {
        tag: "v0.3.9-pr.101.2".into(),
        assets: serde_json::Value::Null,
        channel_pointer: Some(ChannelPointer {
            schema_version: 1,
            channel: "pr-101".into(),
            version: "0.3.9-pr.101.2".into(),
            release_tag: "v0.3.9-pr.101.2".into(),
            source_commit: "0123456789abcdef0123456789abcdef01234567".into(),
            release_manifest_sha256: release_manifest_sha256.into(),
        }),
    }
}

fn staging_manifest(source_commit: &str) -> serde_json::Value {
    serde_json::json!({
        "schema_version": 1,
        "version": "0.3.9-pr.101.2",
        "release": {
            "schema_version": 1,
            "track": "staging",
            "channel": "pr-101",
            "version": "0.3.9-pr.101.2",
            "release_tag": "v0.3.9-pr.101.2",
            "source_commit": source_commit
        },
        "minimum_cli_version": "0.3.3",
        "install_primitives": []
    })
}
