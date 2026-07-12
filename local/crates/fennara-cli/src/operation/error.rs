use super::{OperationKind, Phase};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureClass {
    ReleaseMetadataDownload,
    AssetDownload,
    HashMismatch,
    ManifestInvalid,
    ProjectInvalid,
    StageFilesystem,
    HandoffFailed,
    RollbackFailed,
    ValidationFailed,
}

impl FailureClass {
    pub(super) fn code(self, kind: OperationKind) -> String {
        let operation = match kind {
            OperationKind::Install => "INSTALL",
            OperationKind::Update => "UPDATE",
            OperationKind::SelfUpdate => "SELF-UPDATE",
        };
        format!("FEN-{operation}-{}", self.suffix())
    }

    fn suffix(self) -> &'static str {
        match self {
            Self::ReleaseMetadataDownload => "RELEASE-METADATA-DOWNLOAD",
            Self::AssetDownload => "ASSET-DOWNLOAD",
            Self::HashMismatch => "HASH-MISMATCH",
            Self::ManifestInvalid => "MANIFEST-INVALID",
            Self::ProjectInvalid => "PROJECT-INVALID",
            Self::StageFilesystem => "STAGE-FILESYSTEM",
            Self::HandoffFailed => "HANDOFF-FAILED",
            Self::RollbackFailed => "ROLLBACK-FAILED",
            Self::ValidationFailed => "VALIDATION-FAILED",
        }
    }
}

pub(super) fn fallback_code(kind: OperationKind, phase: Phase) -> String {
    let operation = match kind {
        OperationKind::Install => "INSTALL",
        OperationKind::Update => "UPDATE",
        OperationKind::SelfUpdate => "SELF-UPDATE",
    };
    let suffix = match phase {
        Phase::Downloading => "DOWNLOAD",
        Phase::Verifying => "VERIFICATION",
        Phase::Staging | Phase::Applying => "FILESYSTEM",
        _ => "FAILED",
    };
    format!("FEN-{operation}-{suffix}")
}
