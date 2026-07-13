use crate::release_version::parse_release_version;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub(crate) const ADDON_IDENTITY_FILE: &str = "release.json";
const IDENTITY_SCHEMA_VERSION: u64 = 1;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReleaseTrack {
    Stable,
    Staging,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct ReleaseIdentity {
    pub schema_version: u64,
    pub track: ReleaseTrack,
    pub version: String,
    pub release_tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ReleaseSelector {
    StableLatest,
    StagingChannel(String),
    ExactVersion(String),
}

impl ReleaseIdentity {
    pub(crate) fn parse(bytes: &[u8], expected_version: &str) -> Result<Self, String> {
        let identity: Self = serde_json::from_slice(bytes)
            .map_err(|error| format!("failed to parse addon release identity: {error}"))?;
        identity.validate(expected_version)?;
        Ok(identity)
    }

    pub(crate) fn load(addon_dir: &Path, expected_version: &str) -> Result<Self, String> {
        let path = addon_dir.join(ADDON_IDENTITY_FILE);
        if !path.is_file() {
            return Self::legacy_stable(expected_version);
        }
        let bytes = fs::read(&path).map_err(|error| {
            format!(
                "failed to read addon release identity {}: {error}",
                path.display()
            )
        })?;
        Self::parse(&bytes, expected_version)
    }

    pub(crate) fn validate(&self, expected_version: &str) -> Result<(), String> {
        if self.schema_version != IDENTITY_SCHEMA_VERSION {
            return Err(format!(
                "unsupported addon release identity schema {}",
                self.schema_version
            ));
        }
        let parsed = parse_release_version(&self.version)?;
        if self.version != expected_version {
            return Err(format!(
                "addon release identity version {:?} does not match VERSION {:?}",
                self.version, expected_version
            ));
        }
        let expected_tag = format!("v{}", self.version);
        if self.release_tag != expected_tag {
            return Err(format!(
                "addon release identity tag {:?} does not match {expected_tag:?}",
                self.release_tag
            ));
        }

        match self.track {
            ReleaseTrack::Stable => {
                if !parsed.pre.is_empty() {
                    return Err(
                        "stable addon release identity must not use a prerelease version".into(),
                    );
                }
                if self.channel.is_some() {
                    return Err("stable addon release identity must not include a channel".into());
                }
                if let Some(source_commit) = self.source_commit.as_deref() {
                    validate_source_commit(source_commit)?;
                }
            }
            ReleaseTrack::Staging => {
                let channel = self.channel.as_deref().ok_or_else(|| {
                    "staging addon release identity is missing channel".to_string()
                })?;
                let pull_request = pull_request_number(channel)?;
                let expected_prefix = format!("pr.{pull_request}.");
                let prerelease = parsed.pre.as_str();
                let candidate = prerelease.strip_prefix(&expected_prefix).ok_or_else(|| {
                    format!(
                        "staging version {:?} does not belong to channel {channel:?}",
                        self.version
                    )
                })?;
                if candidate
                    .parse::<u64>()
                    .ok()
                    .filter(|value| *value > 0)
                    .is_none()
                    || candidate.starts_with('0')
                {
                    return Err("staging candidate number must be a positive integer".into());
                }
                validate_source_commit(self.source_commit.as_deref().unwrap_or_default())?;
            }
        }
        Ok(())
    }

    fn legacy_stable(version: &str) -> Result<Self, String> {
        let parsed = parse_release_version(version)?;
        if !parsed.pre.is_empty() {
            return Err(format!(
                "prerelease addon version {version:?} requires {ADDON_IDENTITY_FILE}"
            ));
        }
        Ok(Self {
            schema_version: IDENTITY_SCHEMA_VERSION,
            track: ReleaseTrack::Stable,
            version: version.to_string(),
            release_tag: format!("v{version}"),
            channel: None,
            source_commit: None,
        })
    }
}

impl ReleaseSelector {
    pub(crate) fn staging(channel: &str) -> Result<Self, String> {
        pull_request_number(channel)?;
        Ok(Self::StagingChannel(channel.to_string()))
    }

    pub(crate) fn exact(version: &str) -> Result<Self, String> {
        parse_release_version(version)?;
        Ok(Self::ExactVersion(version.to_string()))
    }

    pub(crate) fn from_version_request(value: &str) -> Result<Self, String> {
        if value == "latest" {
            Ok(Self::StableLatest)
        } else if let Some(channel) = value.strip_prefix("channel:") {
            Self::staging(channel)
        } else {
            Self::exact(value)
        }
    }

    pub(crate) fn github_tag(&self) -> String {
        match self {
            Self::StableLatest => "latest".to_string(),
            Self::StagingChannel(channel) => format!("staging-{channel}"),
            Self::ExactVersion(version) => format!("v{version}"),
        }
    }
}

pub(crate) fn pull_request_number(channel: &str) -> Result<u64, String> {
    let number = channel
        .strip_prefix("pr-")
        .ok_or_else(|| format!("staging channel {channel:?} must use pr-<number> format"))?;
    if number.starts_with('0') {
        return Err(format!(
            "staging channel {channel:?} must use a positive pull-request number"
        ));
    }
    number
        .parse::<u64>()
        .ok()
        .filter(|value| *value > 0)
        .ok_or_else(|| format!("staging channel {channel:?} must use pr-<number> format"))
}

pub(crate) fn channel_pointer_asset_name(channel: &str) -> Result<String, String> {
    pull_request_number(channel)?;
    Ok(format!("fennara-staging-channel-{channel}.json"))
}

fn validate_source_commit(value: &str) -> Result<(), String> {
    if value.len() != 40 || !value.chars().all(|character| character.is_ascii_hexdigit()) {
        return Err("staging source commit must be a full 40-character Git SHA".into());
    }
    if value
        .chars()
        .any(|character| character.is_ascii_uppercase())
    {
        return Err("staging source commit must use lowercase hexadecimal".into());
    }
    Ok(())
}
