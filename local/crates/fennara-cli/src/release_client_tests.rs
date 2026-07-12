use crate::release_client::{DownloadAsset, download_bytes, verify_download_hash};
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
