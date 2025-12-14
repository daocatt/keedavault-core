// UniFFI bindings for KeedaVault Core
//
// This module provides the FFI layer for iOS/Swift integration using proc-macros

use crate::search;
use crate::totp;
use crate::vault::Vault as CoreVault;
use crate::{VaultConfig as CoreVaultConfig, VaultError as CoreVaultError};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

// ============================================================================
// Type Definitions for UniFFI
// ============================================================================

/// Entry data structure for UniFFI (simplified with i64 timestamps)
#[derive(uniffi::Record, Clone)]
pub struct Entry {
    pub id: String,
    pub group_id: String,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub tags: Vec<String>,
    pub totp_secret: Option<String>,
    pub custom_fields: Vec<CustomField>,
    pub created_at: i64,         // Unix timestamp
    pub modified_at: i64,        // Unix timestamp
    pub accessed_at: i64,        // Unix timestamp
    pub expires_at: Option<i64>, // Unix timestamp
    pub is_favorite: bool,
}

/// Group data structure for UniFFI
#[derive(uniffi::Record, Clone)]
pub struct Group {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub icon_id: u32,
    pub notes: String,
    pub is_recycle_bin: bool,
    pub is_expanded: bool,
}

/// Custom field
#[derive(uniffi::Record, Clone)]
pub struct CustomField {
    pub key: String,
    pub value: String,
    pub protected: bool,
}

/// Vault configuration
#[derive(uniffi::Record, Clone)]
pub struct VaultConfig {
    pub kdf_iterations: u64,
    pub argon2_memory: u64,
    pub argon2_parallelism: u32,
}

/// Error type for UniFFI
#[derive(uniffi::Error, Debug, thiserror::Error)]
pub enum VaultError {
    #[error("Vault is locked")]
    VaultLocked,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Entry not found")]
    EntryNotFound,
    #[error("Group not found")]
    GroupNotFound,
    #[error("Invalid entry")]
    InvalidEntry,
    #[error("Encryption error")]
    EncryptionError,
    #[error("Decryption error")]
    DecryptionError,
    #[error("IO error")]
    IoError,
    #[error("Serialization error")]
    SerializationError,
    #[error("KeePass error")]
    KeePassError,
    #[error("Unknown error")]
    Unknown,
}

// ============================================================================
// Type Conversions
// ============================================================================

impl From<crate::Entry> for Entry {
    fn from(e: crate::Entry) -> Self {
        Entry {
            id: e.id.clone(),
            group_id: e.group_id.clone(),
            title: e.title.clone(),
            username: e.username.clone(),
            password: e.password.clone(),
            url: e.url.clone(),
            notes: e.notes.clone(),
            tags: e.tags.clone(),
            totp_secret: e.totp_secret.clone(),
            custom_fields: e.custom_fields.iter().map(|f| f.clone().into()).collect(),
            created_at: e.created_at.timestamp(),
            modified_at: e.modified_at.timestamp(),
            accessed_at: e.accessed_at.timestamp(),
            expires_at: e.expires_at.map(|t| t.timestamp()),
            is_favorite: e.is_favorite,
        }
    }
}

impl From<Entry> for crate::Entry {
    fn from(e: Entry) -> Self {
        use chrono::{TimeZone, Utc};
        crate::Entry {
            id: e.id,
            group_id: e.group_id,
            title: e.title,
            username: e.username,
            password: e.password,
            url: e.url,
            notes: e.notes,
            tags: e.tags,
            totp_secret: e.totp_secret,
            custom_fields: e.custom_fields.into_iter().map(|f| f.into()).collect(),
            created_at: Utc.timestamp_opt(e.created_at, 0).unwrap(),
            modified_at: Utc.timestamp_opt(e.modified_at, 0).unwrap(),
            accessed_at: Utc.timestamp_opt(e.accessed_at, 0).unwrap(),
            expires_at: e.expires_at.map(|t| Utc.timestamp_opt(t, 0).unwrap()),
            is_favorite: e.is_favorite,
        }
    }
}

impl From<crate::Group> for Group {
    fn from(g: crate::Group) -> Self {
        Group {
            id: g.id,
            parent_id: g.parent_id,
            name: g.name,
            icon_id: g.icon_id,
            notes: g.notes,
            is_recycle_bin: g.is_recycle_bin,
            is_expanded: g.is_expanded,
        }
    }
}

impl From<Group> for crate::Group {
    fn from(g: Group) -> Self {
        crate::Group {
            id: g.id,
            parent_id: g.parent_id,
            name: g.name,
            icon_id: g.icon_id,
            notes: g.notes,
            is_recycle_bin: g.is_recycle_bin,
            is_expanded: g.is_expanded,
        }
    }
}

impl From<crate::CustomField> for CustomField {
    fn from(f: crate::CustomField) -> Self {
        CustomField {
            key: f.key.clone(),
            value: f.value.clone(),
            protected: f.protected,
        }
    }
}

impl From<CustomField> for crate::CustomField {
    fn from(f: CustomField) -> Self {
        crate::CustomField {
            key: f.key,
            value: f.value,
            protected: f.protected,
        }
    }
}

impl From<VaultConfig> for CoreVaultConfig {
    fn from(c: VaultConfig) -> Self {
        CoreVaultConfig {
            kdf_iterations: c.kdf_iterations,
            argon2_memory: c.argon2_memory,
            argon2_parallelism: c.argon2_parallelism,
        }
    }
}

impl From<CoreVaultError> for VaultError {
    fn from(e: CoreVaultError) -> Self {
        match e {
            CoreVaultError::OpenError(_) => VaultError::IoError,
            CoreVaultError::SaveError(_) => VaultError::IoError,
            CoreVaultError::VaultLocked => VaultError::VaultLocked,
            CoreVaultError::InvalidPassword => VaultError::InvalidPassword,
            CoreVaultError::EntryNotFound(_) => VaultError::EntryNotFound,
            CoreVaultError::GroupNotFound(_) => VaultError::GroupNotFound,
            CoreVaultError::InvalidEntry(_) => VaultError::InvalidEntry,
            CoreVaultError::EncryptionError(_) => VaultError::EncryptionError,
            CoreVaultError::DecryptionError(_) => VaultError::DecryptionError,
            CoreVaultError::IoError(_) => VaultError::IoError,
            CoreVaultError::SerializationError(_) => VaultError::SerializationError,
            CoreVaultError::KeePassError(_) => VaultError::KeePassError,
            CoreVaultError::Unknown(_) => VaultError::Unknown,
        }
    }
}

// ============================================================================
// Vault Wrapper for UniFFI
// ============================================================================

#[derive(uniffi::Object)]
pub struct Vault {
    inner: Mutex<CoreVault>,
}

#[uniffi::export]
impl Vault {
    /// Lock the vault
    pub fn lock(&self) -> Result<(), VaultError> {
        self.inner.lock().unwrap().lock();
        Ok(())
    }

    /// Save the vault
    pub fn save(&self) -> Result<(), VaultError> {
        self.inner.lock().unwrap().save().map_err(|e| e.into())
    }

    /// Check if vault is locked
    pub fn is_locked(&self) -> bool {
        self.inner.lock().unwrap().is_locked()
    }

    // Entry CRUD
    pub fn add_entry(&self, entry: Entry) -> Result<String, VaultError> {
        self.inner
            .lock()
            .unwrap()
            .add_entry(entry.into())
            .map_err(|e| e.into())
    }

    pub fn get_entry(&self, id: String) -> Result<Entry, VaultError> {
        self.inner
            .lock()
            .unwrap()
            .get_entry(&id)
            .map(|e| e.into())
            .map_err(|e| e.into())
    }

    pub fn get_entries(&self) -> Result<Vec<Entry>, VaultError> {
        self.inner
            .lock()
            .unwrap()
            .get_entries()
            .map(|entries| entries.into_iter().map(|e| e.into()).collect())
            .map_err(|e| e.into())
    }

    pub fn update_entry(&self, id: String, entry: Entry) -> Result<(), VaultError> {
        self.inner
            .lock()
            .unwrap()
            .update_entry(&id, entry.into())
            .map_err(|e| e.into())
    }

    pub fn delete_entry(&self, id: String) -> Result<(), VaultError> {
        self.inner
            .lock()
            .unwrap()
            .delete_entry(&id)
            .map_err(|e| e.into())
    }

    pub fn permanently_delete_entry(&self, id: String) -> Result<(), VaultError> {
        self.inner
            .lock()
            .unwrap()
            .permanently_delete_entry(&id)
            .map_err(|e| e.into())
    }

    // Group CRUD
    pub fn add_group(&self, group: Group) -> Result<String, VaultError> {
        self.inner
            .lock()
            .unwrap()
            .add_group(group.into())
            .map_err(|e| e.into())
    }

    pub fn get_groups(&self) -> Result<Vec<Group>, VaultError> {
        self.inner
            .lock()
            .unwrap()
            .get_groups()
            .map(|groups| groups.into_iter().map(|g| g.into()).collect())
            .map_err(|e| e.into())
    }

    pub fn update_group(&self, id: String, group: Group) -> Result<(), VaultError> {
        self.inner
            .lock()
            .unwrap()
            .update_group(&id, group.into())
            .map_err(|e| e.into())
    }

    pub fn delete_group(&self, id: String) -> Result<(), VaultError> {
        self.inner
            .lock()
            .unwrap()
            .delete_group(&id)
            .map_err(|e| e.into())
    }

    pub fn permanently_delete_group(&self, id: String) -> Result<(), VaultError> {
        self.inner
            .lock()
            .unwrap()
            .permanently_delete_group(&id)
            .map_err(|e| e.into())
    }

    // Recycle bin
    pub fn empty_recycle_bin(&self) -> Result<(), VaultError> {
        self.inner
            .lock()
            .unwrap()
            .empty_recycle_bin()
            .map_err(|e| e.into())
    }

    // Search
    pub fn search_entries(&self, query: String) -> Result<Vec<Entry>, VaultError> {
        let entries = self
            .inner
            .lock()
            .unwrap()
            .get_entries()
            .map_err(|e: CoreVaultError| -> VaultError { e.into() })?;
        Ok(search::search_entries(&entries, &query)
            .into_iter()
            .map(|e| e.into())
            .collect())
    }

    pub fn filter_by_tag(&self, tag: String) -> Result<Vec<Entry>, VaultError> {
        let entries = self
            .inner
            .lock()
            .unwrap()
            .get_entries()
            .map_err(|e: CoreVaultError| -> VaultError { e.into() })?;
        Ok(search::filter_by_tag(&entries, &tag)
            .into_iter()
            .map(|e| e.into())
            .collect())
    }

    pub fn get_favorites(&self) -> Result<Vec<Entry>, VaultError> {
        let entries = self
            .inner
            .lock()
            .unwrap()
            .get_entries()
            .map_err(|e: CoreVaultError| -> VaultError { e.into() })?;
        Ok(search::get_favorites(&entries)
            .into_iter()
            .map(|e| e.into())
            .collect())
    }
}

// ============================================================================
// Factory Functions
// ============================================================================

#[uniffi::export]
pub fn create_vault(
    path: String,
    password: String,
    config: VaultConfig,
) -> Result<Arc<Vault>, VaultError> {
    let core_vault = CoreVault::create(&PathBuf::from(path), &password, config.into())
        .map_err(|e| -> VaultError { e.into() })?;
    Ok(Arc::new(Vault {
        inner: Mutex::new(core_vault),
    }))
}

#[uniffi::export]
pub fn open_vault(path: String, password: String) -> Result<Arc<Vault>, VaultError> {
    let core_vault =
        CoreVault::open(&PathBuf::from(path), &password).map_err(|e| -> VaultError { e.into() })?;
    Ok(Arc::new(Vault {
        inner: Mutex::new(core_vault),
    }))
}

// ============================================================================
// TOTP Functions
// ============================================================================

#[uniffi::export]
pub fn generate_totp(secret: String) -> Result<String, VaultError> {
    totp::generate_totp(&secret).map_err(|_| VaultError::Unknown)
}

#[uniffi::export]
pub fn validate_totp(secret: String, code: String) -> Result<bool, VaultError> {
    totp::validate_totp(&secret, &code).map_err(|_| VaultError::Unknown)
}

#[uniffi::export]
pub fn get_remaining_seconds() -> u32 {
    totp::get_remaining_seconds() as u32
}

// ============================================================================
// UniFFI Scaffolding
// ============================================================================

// Note: uniffi::setup_scaffolding!() must be called in lib.rs, not here
