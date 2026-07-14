use crate::release_manifest::ReleaseManifest;
use crate::{app_layout::arch_name, app_layout::platform_name};

#[test]
fn selects_cli_asset_before_install_validation() {
    let key = format!("{}-{}", platform_name(), arch_name());
    let mut cli_assets = serde_json::Map::new();
    cli_assets.insert(
        key,
        serde_json::json!({
            "name": "fennara-cli-current-platform-v9.0.0.zip",
            "sha256": "a".repeat(64)
        }),
    );
    let manifest = serde_json::json!({
        "schema_version": 99,
        "version": "9.0.0",
        "minimum_cli_version": "9.0.0",
        "install_primitives": ["future-primitive"],
        "assets": {
            "cli": cli_assets
        }
    });
    let raw = serde_json::to_vec(&manifest).unwrap();

    let parsed = ReleaseManifest::parse(&raw).unwrap();
    let cli = parsed.select_cli_for_current_platform().unwrap();

    assert_eq!(cli.version, "9.0.0");
    assert_eq!(cli.cli.name, "fennara-cli-current-platform-v9.0.0.zip");
    assert!(parsed.validate_for_install().is_err());
}

#[test]
fn validates_optional_release_identity_against_manifest_version() {
    let manifest = serde_json::json!({
        "schema_version": 1,
        "version": "0.3.9-pr.101.1",
        "release": {
            "schema_version": 1,
            "track": "staging",
            "channel": "pr-101",
            "version": "0.3.9-pr.101.1",
            "release_tag": "v0.3.9-pr.101.1",
            "source_commit": "0123456789abcdef0123456789abcdef01234567"
        },
        "minimum_cli_version": "0.3.3",
        "install_primitives": []
    });
    let parsed = ReleaseManifest::parse(&serde_json::to_vec(&manifest).unwrap()).unwrap();
    assert!(parsed.validate_for_install().is_ok());

    let mut mismatched = manifest;
    mismatched["release"]["channel"] = serde_json::json!("pr-125");
    let parsed = ReleaseManifest::parse(&serde_json::to_vec(&mismatched).unwrap()).unwrap();
    assert!(parsed.validate_for_install().is_err());
}

#[test]
fn permits_legacy_stable_identity_but_requires_it_for_prereleases() {
    let stable = manifest_without_identity("0.3.8");
    let parsed = ReleaseManifest::parse(&serde_json::to_vec(&stable).unwrap()).unwrap();
    assert!(parsed.release_identity().unwrap().is_none());

    let prerelease = manifest_without_identity("0.3.9-pr.101.1");
    let parsed = ReleaseManifest::parse(&serde_json::to_vec(&prerelease).unwrap()).unwrap();
    let error = parsed.release_identity().unwrap_err();
    assert!(error.contains("prerelease manifest"));
    assert!(error.contains("requires release identity"));
}

fn manifest_without_identity(version: &str) -> serde_json::Value {
    serde_json::json!({
        "schema_version": 1,
        "version": version,
        "minimum_cli_version": "0.3.3",
        "install_primitives": []
    })
}
