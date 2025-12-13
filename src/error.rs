use thiserror::Error;

/// Result type for vault operations
pub type Result<T> = std::result::Result<T, VaultError>;

/// Errors that can occur during vault operations
#[derive(Error, Debug)]
pub enum VaultError {
    #[error("Failed to open vault: {0}")]
    OpenError(String),

    #[error("Failed to save vault: {0}")]
    SaveError(String),

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Vault is locked")]
    VaultLocked,

    #[error("Entry not found: {0}")]
    EntryNotFound(String),

    #[error("Group not found: {0}")]
    GroupNotFound(String),

    #[error("Invalid entry data: {0}")]
    InvalidEntry(String),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("KeePass error: {0}")]
    KeePassError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Convert keepass errors to VaultError
impl From<keepass::error::DatabaseOpenError> for VaultError {
    fn from(err: keepass::error::DatabaseOpenError) -> Self {
        VaultError::KeePassError(err.to_string())
    }
}

impl From<keepass::error::DatabaseSaveError> for VaultError {
    fn from(err: keepass::error::DatabaseSaveError) -> Self {
        VaultError::KeePassError(err.to_string())
    }
}
