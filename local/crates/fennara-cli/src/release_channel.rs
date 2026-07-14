use crate::release_client;
use crate::release_identity::{
    ReleaseIdentity, ReleaseSelector, ReleaseTrack, channel_pointer_asset_name, channel_pointer_ref,
};
use crate::release_manifest::ReleaseManifest;
use serde::{Deserialize, Serialize};

const CHANNEL_POINTER_SCHEMA_VERSION: u64 = 1;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct ChannelPointer {
    pub schema_version: u64,
    pub channel: String,
    pub version: String,
    pub release_tag: String,
    pub source_commit: String,
    pub release_manifest_sha256: String,
}

impl ChannelPointer {
    pub(crate) fn parse(bytes: &[u8], expected_channel: &str) -> Result<Self, String> {
        let pointer: Self = serde_json::from_slice(bytes)
            .map_err(|error| format!("failed to parse staging channel pointer: {error}"))?;
        pointer.validate(expected_channel)?;
        Ok(pointer)
    }

    pub(crate) fn validate(&self, expected_channel: &str) -> Result<(), String> {
        if self.schema_version != CHANNEL_POINTER_SCHEMA_VERSION {
            return Err(format!(
                "unsupported staging channel pointer schema {}",
                self.schema_version
            ));
        }
        if self.channel != expected_channel {
            return Err(format!(
                "staging channel pointer {:?} does not match requested channel {expected_channel:?}",
                self.channel
            ));
        }
        let identity = ReleaseIdentity {
            schema_version: 1,
            track: ReleaseTrack::Staging,
            version: self.version.clone(),
            release_tag: self.release_tag.clone(),
            channel: Some(self.channel.clone()),
            source_commit: Some(self.source_commit.clone()),
        };
        identity.validate(&self.version)?;
        if self.release_manifest_sha256.len() != 64
            || !self
                .release_manifest_sha256
                .chars()
                .all(|character| character.is_ascii_hexdigit())
        {
            return Err("staging channel pointer has invalid release manifest SHA-256".into());
        }
        Ok(())
    }

    pub(crate) fn exact_selector(&self) -> Result<ReleaseSelector, String> {
        ReleaseSelector::exact(&self.version)
    }

    pub(crate) fn validate_manifest_identity(
        &self,
        manifest: &ReleaseManifest,
    ) -> Result<(), String> {
        let identity = manifest.release_identity()?.ok_or_else(|| {
            "staging release manifest is missing its release identity".to_string()
        })?;
        if identity.track != ReleaseTrack::Staging
            || identity.version != self.version
            || identity.release_tag != self.release_tag
            || identity.channel.as_deref() != Some(self.channel.as_str())
            || identity.source_commit.as_deref() != Some(self.source_commit.as_str())
        {
            return Err("staging release manifest identity does not match channel pointer".into());
        }
        Ok(())
    }
}

pub(crate) fn fetch(channel: &str) -> Result<ChannelPointer, String> {
    ReleaseSelector::staging(channel)?;
    let asset_name = channel_pointer_asset_name(channel)?;
    let reference = channel_pointer_ref(channel)?;
    let bytes = release_client::download_github_contents(&reference, &asset_name)?;
    ChannelPointer::parse(&bytes, channel)
}
