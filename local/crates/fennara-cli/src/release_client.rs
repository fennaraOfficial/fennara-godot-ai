use crate::app_layout::display_path;
use crate::operation::{self, FailureClass, Phase};
use crate::release_channel::ChannelPointer;
use crate::release_identity::ReleaseSelector;
use crate::release_manifest::ReleaseManifest;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::time::Duration;
use zip::ZipArchive;

const REPO: &str = "fennaraOfficial/fennara-godot-ai";
const HTTP_CONNECT_TIMEOUT_SECS: u64 = 20;
const HTTP_READ_TIMEOUT_SECS: u64 = 120;
const HTTP_WRITE_TIMEOUT_SECS: u64 = 30;

#[derive(Clone)]
pub struct ReleaseAsset {
    pub name: String,
    pub url: String,
    pub api_url: Option<String>,
    pub version: Option<String>,
}

pub struct Release {
    pub tag: String,
    pub assets: Value,
    pub(crate) channel_pointer: Option<ChannelPointer>,
}

impl Release {
    pub(crate) fn is_channel_release(&self) -> bool {
        self.channel_pointer.is_some()
    }

    pub fn asset(&self, prefix: &str) -> Option<ReleaseAsset> {
        self.asset_by_prefix_suffix(prefix, ".zip")
    }

    pub fn manifest_asset(&self) -> Option<ReleaseAsset> {
        self.asset_by_prefix_suffix("fennara-release-manifest-v", ".json")
    }

    pub fn asset_by_name(&self, expected_name: &str) -> Option<ReleaseAsset> {
        self.assets.as_array()?.iter().find_map(|asset| {
            let name = asset.get("name")?.as_str()?;
            if name != expected_name {
                return None;
            }
            let url = asset.get("browser_download_url")?.as_str()?;
            Some(ReleaseAsset {
                name: name.to_string(),
                url: url.to_string(),
                api_url: asset.get("url").and_then(Value::as_str).map(str::to_string),
                version: version_from_asset_name(name),
            })
        })
    }

    fn asset_by_prefix_suffix(&self, prefix: &str, suffix: &str) -> Option<ReleaseAsset> {
        self.assets.as_array()?.iter().find_map(|asset| {
            let name = asset.get("name")?.as_str()?;
            if !name.starts_with(prefix) || !name.ends_with(suffix) {
                return None;
            }
            let url = asset.get("browser_download_url")?.as_str()?;
            Some(ReleaseAsset {
                name: name.to_string(),
                url: url.to_string(),
                api_url: asset.get("url").and_then(Value::as_str).map(str::to_string),
                version: version_from_asset_name(name),
            })
        })
    }
}

pub struct DownloadAsset<'a> {
    pub url: &'a str,
    pub expected_sha256: Option<&'a str>,
    pub label: &'a str,
}

pub fn fetch_release(version: &str) -> Result<Release, String> {
    let selector = ReleaseSelector::from_version_request(version)?;
    match &selector {
        ReleaseSelector::StagingChannel(channel) => {
            let pointer = crate::release_channel::fetch(channel)?;
            let mut release = fetch_release_for_selector(&pointer.exact_selector()?)?;
            release.channel_pointer = Some(pointer);
            Ok(release)
        }
        _ => fetch_release_for_selector(&selector),
    }
}

pub(crate) fn fetch_release_for_selector(selector: &ReleaseSelector) -> Result<Release, String> {
    let tag = selector.github_tag();
    let url = format!("https://api.github.com/repos/{REPO}/releases/tags/{tag}");
    operation::phase(Phase::Checking, "Fetching release metadata")?;
    println!("release: fetching metadata from {url}");
    let response = http_agent()
        .get(&url)
        .set("User-Agent", "fennara-cli")
        .call()
        .map_err(|err| {
            operation::failure(
                FailureClass::ReleaseMetadataDownload,
                format!("failed to fetch release metadata from {url}: {err}"),
            )
        })?;
    let value: Value = response.into_json().map_err(|err| {
        operation::failure(
            FailureClass::ManifestInvalid,
            format!("failed to parse release metadata: {err}"),
        )
    })?;

    let release = Release {
        tag: value
            .get("tag_name")
            .and_then(Value::as_str)
            .unwrap_or(&tag)
            .to_string(),
        assets: value.get("assets").cloned().unwrap_or(Value::Null),
        channel_pointer: None,
    };
    println!("release: {}", release.tag);
    operation::set_component("release", release.tag.trim_start_matches('v'))?;
    Ok(release)
}

pub fn download_release_manifest(
    release: &Release,
    asset: &ReleaseAsset,
) -> Result<ReleaseManifest, String> {
    let (bytes, actual_sha256) = download_bytes_with_hash(&asset.url, &asset.name)?;
    parse_release_manifest(release, &bytes, &actual_sha256, &asset.name)
}

pub(crate) fn parse_release_manifest(
    release: &Release,
    bytes: &[u8],
    actual_sha256: &str,
    label: &str,
) -> Result<ReleaseManifest, String> {
    if let Some(pointer) = release.channel_pointer.as_ref() {
        verify_expected_hash(label, &pointer.release_manifest_sha256, actual_sha256)?;
    }
    let manifest = ReleaseManifest::parse(bytes)
        .map_err(|error| operation::failure(FailureClass::ManifestInvalid, error))?;
    manifest
        .release_identity()
        .map_err(|error| operation::failure(FailureClass::ManifestInvalid, error))?;
    if let Some(pointer) = release.channel_pointer.as_ref() {
        pointer
            .validate_manifest_identity(&manifest)
            .map_err(|error| operation::failure(FailureClass::ManifestInvalid, error))?;
    }
    Ok(manifest)
}

pub(crate) fn download_github_api_asset(
    asset: &ReleaseAsset,
    label: &str,
) -> Result<Vec<u8>, String> {
    let api_url = asset.api_url.as_deref().ok_or_else(|| {
        operation::failure(
            FailureClass::ReleaseMetadataDownload,
            format!("GitHub release asset {} is missing its API URL", asset.name),
        )
    })?;
    operation::select_asset(label, None)?;
    operation::phase(Phase::Downloading, &format!("Downloading {label}"))?;
    let response = http_agent()
        .get(api_url)
        .set("Accept", "application/octet-stream")
        .set("User-Agent", "fennara-cli")
        .call()
        .map_err(|error| {
            operation::failure(
                FailureClass::AssetDownload,
                format!("failed to download {label} from GitHub asset API: {error}"),
            )
        })?;
    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(|error| {
            operation::failure(
                FailureClass::AssetDownload,
                format!("failed to read {label} from GitHub asset API: {error}"),
            )
        })?;
    Ok(bytes)
}

pub fn download_zip_to_dir(asset: &DownloadAsset<'_>, target: &Path) -> Result<(), String> {
    operation::select_asset(asset.label, asset.expected_sha256)?;
    fs::create_dir_all(target).map_err(|err| {
        operation::failure(
            FailureClass::StageFilesystem,
            format!("failed to create {}: {err}", display_path(target)),
        )
    })?;
    let (bytes, actual_sha256) = download_bytes_with_hash(asset.url, asset.label)?;
    verify_download_hash(asset, &actual_sha256)?;

    operation::phase(Phase::Staging, &format!("Extracting {}", asset.label))?;
    println!("extracting: {} to {}", asset.label, display_path(target));
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|err| {
        operation::failure(
            FailureClass::ValidationFailed,
            format!("failed to open downloaded zip: {err}"),
        )
    })?;
    archive.extract(target).map_err(|err| {
        operation::failure(
            FailureClass::StageFilesystem,
            format!("failed to extract zip into {}: {err}", display_path(target)),
        )
    })?;
    println!("extracted: {}", asset.label);
    Ok(())
}

pub(crate) fn verify_download_hash(
    asset: &DownloadAsset<'_>,
    actual_sha256: &str,
) -> Result<(), String> {
    if let Some(expected_sha256) = asset.expected_sha256 {
        verify_expected_hash(asset.label, expected_sha256, actual_sha256)?;
    } else {
        operation::record_asset_hash(asset.label, actual_sha256, None)?;
    }
    Ok(())
}

fn verify_expected_hash(
    label: &str,
    expected_sha256: &str,
    actual_sha256: &str,
) -> Result<(), String> {
    operation::phase(Phase::Verifying, &format!("Verifying {label}"))?;
    if !actual_sha256.eq_ignore_ascii_case(expected_sha256) {
        operation::record_asset_hash(label, actual_sha256, Some(false))?;
        return Err(operation::failure(
            FailureClass::HashMismatch,
            format!("{label} sha256 mismatch: expected {expected_sha256}, got {actual_sha256}"),
        ));
    }
    operation::record_asset_hash(label, actual_sha256, Some(true))?;
    println!("sha256: verified {label}");
    Ok(())
}

pub fn download_bytes(url: &str, label: &str) -> Result<Vec<u8>, String> {
    download_bytes_with_hash(url, label).map(|(bytes, _)| bytes)
}

fn download_bytes_with_hash(url: &str, label: &str) -> Result<(Vec<u8>, String), String> {
    operation::select_asset(label, None)?;
    operation::phase(Phase::Downloading, &format!("Downloading {label}"))?;
    println!("download: {label}");
    println!("from: {url}");
    let response = http_agent()
        .get(url)
        .set("User-Agent", "fennara-cli")
        .call()
        .map_err(|err| operation::failure(
            FailureClass::AssetDownload,
            format!(
                "failed to download {label} from {url} within connect/read timeouts ({HTTP_CONNECT_TIMEOUT_SECS}s/{HTTP_READ_TIMEOUT_SECS}s): {err}"
            ),
        ))?;
    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(|err| {
            operation::failure(
                FailureClass::AssetDownload,
                format!("failed to read download for {label}: {err}"),
            )
        })?;
    let actual_sha256 = format!("{:x}", Sha256::digest(&bytes));
    operation::record_asset_hash(label, &actual_sha256, None)?;
    println!("downloaded: {label} ({})", format_bytes(bytes.len()));
    Ok((bytes, actual_sha256))
}

pub fn create_temp_dir(prefix: &str) -> Result<PathBuf, String> {
    let path = env::temp_dir().join(format!(
        "{prefix}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&path).map_err(|err| {
        operation::failure(
            FailureClass::StageFilesystem,
            format!("failed to create {}: {err}", display_path(&path)),
        )
    })?;
    Ok(path)
}

fn version_from_asset_name(name: &str) -> Option<String> {
    let marker = "-v";
    let start = name.rfind(marker)? + marker.len();
    let version = name.get(start..)?.strip_suffix(".zip")?;
    crate::release_version::parse_release_version(version)
        .ok()
        .map(|_| version.to_string())
}

fn http_agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(HTTP_CONNECT_TIMEOUT_SECS))
        .timeout_read(Duration::from_secs(HTTP_READ_TIMEOUT_SECS))
        .timeout_write(Duration::from_secs(HTTP_WRITE_TIMEOUT_SECS))
        .build()
}

fn format_bytes(bytes: usize) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;

    let bytes = bytes as f64;
    if bytes >= MB {
        format!("{:.1} MB", bytes / MB)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes / KB)
    } else {
        format!("{bytes:.0} B")
    }
}
