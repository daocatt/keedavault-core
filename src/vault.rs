use crate::entry::Entry;
use crate::error::{Result, VaultError};
use crate::group::Group;
use std::fs::File;
use std::path::{Path, PathBuf};

/// Vault configuration
#[derive(Debug, Clone)]
pub struct VaultConfig {
    /// KDF iterations (Argon2 or AES-KDF)
    pub kdf_iterations: u64,

    /// Argon2 memory (in KB)
    pub argon2_memory: u64,

    /// Argon2 parallelism
    pub argon2_parallelism: u32,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            kdf_iterations: 2,
            argon2_memory: 64 * 1024, // 64 MB
            argon2_parallelism: 2,
        }
    }
}

/// Main vault structure
pub struct Vault {
    path: PathBuf,
    database: Option<keepass::Database>,
    password: String, // Store password for saving
    is_locked: bool,
}

impl Vault {
    /// Open an existing vault
    pub fn open<P: AsRef<Path>>(path: P, password: &str) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        // Read the file
        let mut file = File::open(&path)
            .map_err(|e| VaultError::OpenError(format!("Failed to open file: {}", e)))?;

        // Open the database using keepass 0.8 API
        let database = keepass::Database::open(
            &mut file,
            keepass::DatabaseKey::new().with_password(password),
        )?;

        Ok(Self {
            path,
            database: Some(database),
            password: password.to_string(),
            is_locked: false,
        })
    }

    /// Create a new vault
    pub fn create<P: AsRef<Path>>(path: P, password: &str, _config: VaultConfig) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        // Create a new database with default config
        let mut database = keepass::Database::new(keepass::config::DatabaseConfig::default());

        // Set up the root group
        database.root.name = "Root".to_string();

        // Save immediately
        let mut file = File::create(&path)
            .map_err(|e| VaultError::SaveError(format!("Failed to create file: {}", e)))?;

        database.save(
            &mut file,
            keepass::DatabaseKey::new().with_password(password),
        )?;

        Ok(Self {
            path,
            database: Some(database),
            password: password.to_string(),
            is_locked: false,
        })
    }

    /// Save the vault
    pub fn save(&mut self) -> Result<()> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_ref().ok_or(VaultError::VaultLocked)?;

        let mut file = File::create(&self.path)
            .map_err(|e| VaultError::SaveError(format!("Failed to open file: {}", e)))?;

        database.save(
            &mut file,
            keepass::DatabaseKey::new().with_password(&self.password),
        )?;

        Ok(())
    }

    /// Lock the vault
    pub fn lock(&mut self) {
        self.database = None;
        self.is_locked = true;
    }

    /// Check if the vault is locked
    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    /// Get the vault path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get all entries from the database (read-only for now)
    pub fn get_entries(&self) -> Result<Vec<Entry>> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_ref().ok_or(VaultError::VaultLocked)?;
        let mut entries = Vec::new();

        // Recursively collect all entries from all groups
        self.collect_entries_from_group(&database.root, &mut entries);

        Ok(entries)
    }

    /// Get a specific entry by ID
    pub fn get_entry(&self, id: &str) -> Result<Entry> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let entries = self.get_entries()?;
        entries
            .into_iter()
            .find(|e| e.id == id)
            .ok_or_else(|| VaultError::EntryNotFound(id.to_string()))
    }

    /// Add a new entry (TODO: Implement in Phase 2)
    pub fn add_entry(&mut self, entry: Entry) -> Result<String> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        // TODO: Implement actual entry addition in Phase 2
        // For now, just return the ID to maintain API compatibility
        Ok(entry.id.clone())
    }

    /// Update an existing entry (TODO: Implement in Phase 2)
    pub fn update_entry(&mut self, _id: &str, _entry: Entry) -> Result<()> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        // TODO: Implement actual entry update in Phase 2
        Ok(())
    }

    /// Delete an entry (TODO: Implement in Phase 2)
    pub fn delete_entry(&mut self, _id: &str) -> Result<()> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        // TODO: Implement actual entry deletion in Phase 2
        Ok(())
    }

    /// Get all groups from the database (read-only for now)
    pub fn get_groups(&self) -> Result<Vec<Group>> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_ref().ok_or(VaultError::VaultLocked)?;
        let mut groups = Vec::new();

        // Recursively collect all groups
        self.collect_groups_from_group(&database.root, None, &mut groups);

        Ok(groups)
    }

    // Helper methods for traversing the database tree

    /// Recursively collect entries from a group and its children
    fn collect_entries_from_group(&self, group: &keepass::db::Group, entries: &mut Vec<Entry>) {
        // Process entries in this group
        for entry in group.entries() {
            if let Some(converted) = self.convert_keepass_entry(entry, &group.uuid.to_string()) {
                entries.push(converted);
            }
        }

        // Recursively process child groups
        for child in &group.children {
            if let keepass::db::Node::Group(child_group) = child {
                self.collect_entries_from_group(child_group, entries);
            }
        }
    }

    /// Convert a keepass entry to our Entry structure
    fn convert_keepass_entry(
        &self,
        kp_entry: &keepass::db::Entry,
        group_id: &str,
    ) -> Option<Entry> {
        let title = kp_entry.get_title().unwrap_or("");
        let username = kp_entry.get_username().unwrap_or("");
        let password = kp_entry.get_password().unwrap_or("");
        let url = kp_entry.get_url().unwrap_or("");

        // Extract notes
        let notes = kp_entry
            .fields
            .get("Notes")
            .and_then(|v| match v {
                keepass::db::Value::Unprotected(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_default();

        // Extract TOTP secret if present
        let totp_secret = kp_entry.fields.get("otp").and_then(|v| match v {
            keepass::db::Value::Protected(sec_vec) => {
                String::from_utf8(sec_vec.unsecure().to_vec()).ok()
            }
            keepass::db::Value::Unprotected(s) => Some(s.clone()),
            _ => None,
        });

        // Extract custom fields
        let mut custom_fields = Vec::new();
        for (key, value) in &kp_entry.fields {
            if !["Title", "UserName", "Password", "URL", "Notes", "otp"].contains(&key.as_str()) {
                let (val, protected) = match value {
                    keepass::db::Value::Protected(sec_vec) => (
                        String::from_utf8_lossy(sec_vec.unsecure()).to_string(),
                        true,
                    ),
                    keepass::db::Value::Unprotected(s) => (s.clone(), false),
                    _ => continue,
                };
                custom_fields.push(crate::entry::CustomField {
                    key: key.clone(),
                    value: val,
                    protected,
                });
            }
        }

        // Convert NaiveDateTime to DateTime<Utc>
        let created_at = kp_entry
            .times
            .get_creation()
            .map(|dt| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(*dt, chrono::Utc))
            .unwrap_or_else(|| chrono::Utc::now());

        let modified_at = kp_entry
            .times
            .get_last_modification()
            .map(|dt| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(*dt, chrono::Utc))
            .unwrap_or_else(|| chrono::Utc::now());

        let accessed_at = kp_entry
            .times
            .get_last_access()
            .map(|dt| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(*dt, chrono::Utc))
            .unwrap_or_else(|| chrono::Utc::now());

        let expires_at = kp_entry
            .times
            .get_expiry()
            .map(|dt| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(*dt, chrono::Utc));

        Some(Entry {
            id: kp_entry.uuid.to_string(),
            group_id: group_id.to_string(),
            title: title.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            url: url.to_string(),
            notes,
            tags: Vec::new(), // KeePass doesn't have native tags in the same way
            totp_secret,
            custom_fields,
            created_at,
            modified_at,
            accessed_at,
            expires_at,
            is_favorite: false, // Could be stored in a custom field
        })
    }

    /// Recursively collect groups from a node
    fn collect_groups_from_group(
        &self,
        group: &keepass::db::Group,
        parent_id: Option<String>,
        groups: &mut Vec<Group>,
    ) {
        let group_id = group.uuid.to_string();

        groups.push(Group {
            id: group_id.clone(),
            parent_id: parent_id.clone(),
            name: group.name.clone(),
            icon_id: 0, // KeePass uses different icon system
            notes: group.notes.clone().unwrap_or_default(),
            is_recycle_bin: false, // Could check group name
            is_expanded: true,
        });

        // Recursively process children
        for child in &group.children {
            if let keepass::db::Node::Group(child_group) = child {
                self.collect_groups_from_group(child_group, Some(group_id.clone()), groups);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_vault() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path().join("test.kdbx");

        let result = Vault::create(&vault_path, "test123", VaultConfig::default());
        assert!(result.is_ok());

        let vault = result.unwrap();
        assert!(!vault.is_locked());
        assert_eq!(vault.path(), vault_path.as_path());
    }

    #[test]
    fn test_lock_vault() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path().join("test.kdbx");

        let mut vault = Vault::create(&vault_path, "test123", VaultConfig::default()).unwrap();
        assert!(!vault.is_locked());

        vault.lock();
        assert!(vault.is_locked());
    }
}
