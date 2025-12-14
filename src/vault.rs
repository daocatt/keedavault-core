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

    /// Get all entries from the database
    pub fn get_entries(&self) -> Result<Vec<Entry>> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_ref().ok_or(VaultError::VaultLocked)?;
        let mut entries = Vec::new();

        // Recursively collect all entries from all groups
        Self::collect_entries_from_group(&database.root, &mut entries);

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

    /// Add a new entry to a specific group
    pub fn add_entry(&mut self, entry: Entry) -> Result<String> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_mut().ok_or(VaultError::VaultLocked)?;

        // Find the target group
        let group = Self::find_group_mut(&mut database.root, &entry.group_id)
            .ok_or_else(|| VaultError::GroupNotFound(entry.group_id.clone()))?;

        // Convert our Entry to keepass::db::Entry
        let mut kp_entry = keepass::db::Entry::default();

        // Set UUID to match our entry ID
        if let Ok(uuid) = uuid::Uuid::parse_str(&entry.id) {
            kp_entry.uuid = uuid;
        }

        // Set standard fields
        kp_entry.fields.insert(
            "Title".to_string(),
            keepass::db::Value::Unprotected(entry.title.clone()),
        );
        kp_entry.fields.insert(
            "UserName".to_string(),
            keepass::db::Value::Unprotected(entry.username.clone()),
        );
        kp_entry.fields.insert(
            "Password".to_string(),
            keepass::db::Value::Protected(entry.password.as_bytes().to_vec().into()),
        );

        if !entry.url.is_empty() {
            kp_entry.fields.insert(
                "URL".to_string(),
                keepass::db::Value::Unprotected(entry.url.clone()),
            );
        }

        if !entry.notes.is_empty() {
            kp_entry.fields.insert(
                "Notes".to_string(),
                keepass::db::Value::Unprotected(entry.notes.clone()),
            );
        }

        // Add TOTP if present
        if let Some(totp_secret) = &entry.totp_secret {
            kp_entry.fields.insert(
                "otp".to_string(),
                keepass::db::Value::Protected(totp_secret.as_bytes().to_vec().into()),
            );
        }

        // Add custom fields
        for field in &entry.custom_fields {
            let value = if field.protected {
                keepass::db::Value::Protected(field.value.as_bytes().to_vec().into())
            } else {
                keepass::db::Value::Unprotected(field.value.clone())
            };
            kp_entry.fields.insert(field.key.clone(), value);
        }

        // Add to group (add_child will wrap it in Node::Entry)
        group.add_child(kp_entry);

        Ok(entry.id.clone())
    }

    /// Update an existing entry
    pub fn update_entry(&mut self, id: &str, entry: Entry) -> Result<()> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_mut().ok_or(VaultError::VaultLocked)?;

        // Find and update the entry
        let kp_entry = Self::find_entry_mut(&mut database.root, id)
            .ok_or_else(|| VaultError::EntryNotFound(id.to_string()))?;

        // Update standard fields
        kp_entry.fields.insert(
            "Title".to_string(),
            keepass::db::Value::Unprotected(entry.title.clone()),
        );
        kp_entry.fields.insert(
            "UserName".to_string(),
            keepass::db::Value::Unprotected(entry.username.clone()),
        );
        kp_entry.fields.insert(
            "Password".to_string(),
            keepass::db::Value::Protected(entry.password.as_bytes().to_vec().into()),
        );
        kp_entry.fields.insert(
            "URL".to_string(),
            keepass::db::Value::Unprotected(entry.url.clone()),
        );

        if !entry.notes.is_empty() {
            kp_entry.fields.insert(
                "Notes".to_string(),
                keepass::db::Value::Unprotected(entry.notes.clone()),
            );
        } else {
            kp_entry.fields.remove("Notes");
        }

        // Update TOTP
        if let Some(totp_secret) = &entry.totp_secret {
            kp_entry.fields.insert(
                "otp".to_string(),
                keepass::db::Value::Protected(totp_secret.as_bytes().to_vec().into()),
            );
        } else {
            kp_entry.fields.remove("otp");
        }

        // Update custom fields - remove old ones and add new ones
        let standard_fields = ["Title", "UserName", "Password", "URL", "Notes", "otp"];
        kp_entry
            .fields
            .retain(|k, _| standard_fields.contains(&k.as_str()));

        for field in &entry.custom_fields {
            let value = if field.protected {
                keepass::db::Value::Protected(field.value.as_bytes().to_vec().into())
            } else {
                keepass::db::Value::Unprotected(field.value.clone())
            };
            kp_entry.fields.insert(field.key.clone(), value);
        }

        Ok(())
    }

    /// Delete an entry by moving it to the recycle bin
    pub fn delete_entry(&mut self, id: &str) -> Result<()> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_mut().ok_or(VaultError::VaultLocked)?;

        // Find the entry first
        let entry_node = Self::find_and_remove_entry(&mut database.root, id)
            .ok_or_else(|| VaultError::EntryNotFound(id.to_string()))?;

        // Get or create recycle bin
        let recycle_bin = Self::find_or_create_recycle_bin(&mut database.root);

        // Move to recycle bin
        recycle_bin.children.push(entry_node);

        Ok(())
    }

    /// Permanently delete an entry (only works for items in recycle bin)
    pub fn permanently_delete_entry(&mut self, id: &str) -> Result<()> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_mut().ok_or(VaultError::VaultLocked)?;

        // Check if the entry is in the recycle bin
        if !Self::is_in_recycle_bin(&database.root, id) {
            return Err(VaultError::EntryNotFound(format!(
                "Entry {} is not in recycle bin. Move to recycle bin first.",
                id
            )));
        }

        // Find and remove the entry permanently
        if Self::remove_entry_from_group(&mut database.root, id) {
            Ok(())
        } else {
            Err(VaultError::EntryNotFound(id.to_string()))
        }
    }

    /// Get all groups from the database
    pub fn get_groups(&self) -> Result<Vec<Group>> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_ref().ok_or(VaultError::VaultLocked)?;
        let mut groups = Vec::new();

        // Recursively collect all groups
        Self::collect_groups_from_group(&database.root, None, &mut groups);

        Ok(groups)
    }

    /// Add a new group
    pub fn add_group(&mut self, group: Group) -> Result<String> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_mut().ok_or(VaultError::VaultLocked)?;

        // Find parent group or use root
        let parent = if let Some(parent_id) = &group.parent_id {
            Self::find_group_mut(&mut database.root, parent_id)
                .ok_or_else(|| VaultError::GroupNotFound(parent_id.clone()))?
        } else {
            &mut database.root
        };

        // Create new keepass group
        let mut kp_group = keepass::db::Group::default();

        // Set UUID to match our group ID
        if let Ok(uuid) = uuid::Uuid::parse_str(&group.id) {
            kp_group.uuid = uuid;
        }

        kp_group.name = group.name.clone();
        kp_group.notes = Some(group.notes.clone());

        parent.add_child(kp_group);

        Ok(group.id.clone())
    }

    /// Update a group
    pub fn update_group(&mut self, id: &str, group: Group) -> Result<()> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_mut().ok_or(VaultError::VaultLocked)?;

        // Find and update the group
        let kp_group = Self::find_group_mut(&mut database.root, id)
            .ok_or_else(|| VaultError::GroupNotFound(id.to_string()))?;

        kp_group.name = group.name.clone();
        kp_group.notes = Some(group.notes.clone());

        Ok(())
    }

    /// Delete a group by moving it to the recycle bin
    pub fn delete_group(&mut self, id: &str) -> Result<()> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_mut().ok_or(VaultError::VaultLocked)?;

        // Don't allow deleting the root group
        if database.root.uuid.to_string() == id {
            return Err(VaultError::GroupNotFound(
                "Cannot delete root group".to_string(),
            ));
        }

        // Find and remove the group
        let group_node = Self::find_and_remove_group(&mut database.root, id)
            .ok_or_else(|| VaultError::GroupNotFound(id.to_string()))?;

        // Check if this is the recycle bin itself
        if let keepass::db::Node::Group(g) = &group_node {
            if g.name == "Recycle Bin" || g.name == "回收站" {
                // Don't move recycle bin to itself, just remove it
                return Ok(());
            }
        }

        // Get or create recycle bin
        let recycle_bin = Self::find_or_create_recycle_bin(&mut database.root);

        // Move to recycle bin
        recycle_bin.children.push(group_node);

        Ok(())
    }

    /// Permanently delete a group (only works for items in recycle bin)
    pub fn permanently_delete_group(&mut self, id: &str) -> Result<()> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_mut().ok_or(VaultError::VaultLocked)?;

        // Don't allow deleting the root group
        if database.root.uuid.to_string() == id {
            return Err(VaultError::GroupNotFound(
                "Cannot delete root group".to_string(),
            ));
        }

        // Check if the group is in the recycle bin
        if !Self::is_in_recycle_bin(&database.root, id) {
            return Err(VaultError::GroupNotFound(format!(
                "Group {} is not in recycle bin. Move to recycle bin first.",
                id
            )));
        }

        if Self::remove_group_from_parent(&mut database.root, id) {
            Ok(())
        } else {
            Err(VaultError::GroupNotFound(id.to_string()))
        }
    }

    /// Empty the recycle bin
    pub fn empty_recycle_bin(&mut self) -> Result<()> {
        if self.is_locked {
            return Err(VaultError::VaultLocked);
        }

        let database = self.database.as_mut().ok_or(VaultError::VaultLocked)?;

        // Find recycle bin
        if let Some(recycle_bin) = Self::find_recycle_bin(&mut database.root) {
            recycle_bin.children.clear();
        }

        Ok(())
    }

    // Helper methods for traversing the database tree

    /// Recursively collect entries from a group and its children
    fn collect_entries_from_group(group: &keepass::db::Group, entries: &mut Vec<Entry>) {
        // Process entries in this group
        for entry in group.entries() {
            if let Some(converted) = Self::convert_keepass_entry(entry, &group.uuid.to_string()) {
                entries.push(converted);
            }
        }

        // Recursively process child groups
        for child in &group.children {
            if let keepass::db::Node::Group(child_group) = child {
                Self::collect_entries_from_group(child_group, entries);
            }
        }
    }

    /// Convert a keepass entry to our Entry structure
    fn convert_keepass_entry(kp_entry: &keepass::db::Entry, group_id: &str) -> Option<Entry> {
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

    /// Recursively collect groups from a group
    fn collect_groups_from_group(
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
                Self::collect_groups_from_group(child_group, Some(group_id.clone()), groups);
            }
        }
    }

    /// Find a mutable reference to a group by ID (static method to avoid borrow checker issues)
    fn find_group_mut<'a>(
        group: &'a mut keepass::db::Group,
        id: &str,
    ) -> Option<&'a mut keepass::db::Group> {
        // Check if this is the group we're looking for
        if group.uuid.to_string() == id {
            return Some(group);
        }

        // Search in children
        for child in &mut group.children {
            if let keepass::db::Node::Group(child_group) = child {
                // Check if the child is the one we're looking for
                if child_group.uuid.to_string() == id {
                    return Some(child_group);
                }
                // Recursively search in the child's children
                if let Some(found) = Self::find_group_mut(child_group, id) {
                    return Some(found);
                }
            }
        }
        None
    }

    /// Find a mutable reference to an entry by ID
    ///
    /// This directly iterates over the children Vec<Node>, which avoids
    /// the borrow checker issues with entries_mut()
    fn find_entry_mut<'a>(
        group: &'a mut keepass::db::Group,
        id: &str,
    ) -> Option<&'a mut keepass::db::Entry> {
        // Search in this group's children
        for child in &mut group.children {
            match child {
                keepass::db::Node::Entry(entry) => {
                    if entry.uuid.to_string() == id {
                        return Some(entry);
                    }
                }
                keepass::db::Node::Group(child_group) => {
                    // Recursively search in child groups
                    if let Some(found) = Self::find_entry_mut(child_group, id) {
                        return Some(found);
                    }
                }
            }
        }

        None
    }

    /// Remove an entry from a group by ID
    fn remove_entry_from_group(group: &mut keepass::db::Group, id: &str) -> bool {
        // Try to remove from this group's children
        if let Some(pos) = group.children.iter().position(|child| {
            if let keepass::db::Node::Entry(e) = child {
                e.uuid.to_string() == id
            } else {
                false
            }
        }) {
            group.children.remove(pos);
            return true;
        }

        // Recursively try child groups
        for child in &mut group.children {
            if let keepass::db::Node::Group(child_group) = child {
                if Self::remove_entry_from_group(child_group, id) {
                    return true;
                }
            }
        }

        false
    }

    /// Remove a group from its parent (static method to avoid borrow checker issues)
    fn remove_group_from_parent(group: &mut keepass::db::Group, id: &str) -> bool {
        // Try to remove from children
        if let Some(pos) = group.children.iter().position(|child| {
            if let keepass::db::Node::Group(g) = child {
                g.uuid.to_string() == id
            } else {
                false
            }
        }) {
            group.children.remove(pos);
            return true;
        }

        // Try recursively
        for child in &mut group.children {
            if let keepass::db::Node::Group(child_group) = child {
                if Self::remove_group_from_parent(child_group, id) {
                    return true;
                }
            }
        }

        false
    }

    /// Find the recycle bin group
    fn find_recycle_bin<'a>(
        group: &'a mut keepass::db::Group,
    ) -> Option<&'a mut keepass::db::Group> {
        for child in &mut group.children {
            match child {
                keepass::db::Node::Group(g) => {
                    if g.name == "Recycle Bin" || g.name == "回收站" {
                        return Some(g);
                    }
                    // Recursively search in child groups
                    if let Some(found) = Self::find_recycle_bin(g) {
                        return Some(found);
                    }
                }
                _ => {}
            }
        }

        None
    }

    /// Find or create the recycle bin group
    fn find_or_create_recycle_bin<'a>(
        root: &'a mut keepass::db::Group,
    ) -> &'a mut keepass::db::Group {
        // Check if recycle bin exists
        let exists = root.children.iter().any(|child| {
            if let keepass::db::Node::Group(g) = child {
                g.name == "Recycle Bin" || g.name == "回收站"
            } else {
                false
            }
        });

        if !exists {
            // Create new recycle bin
            let mut recycle_bin = keepass::db::Group::default();
            recycle_bin.name = "Recycle Bin".to_string();
            recycle_bin.notes = Some("Deleted items".to_string());
            root.add_child(recycle_bin);
        }

        // Now find and return it
        Self::find_recycle_bin(root).unwrap()
    }

    /// Find and remove an entry, returning the Node
    fn find_and_remove_entry(
        group: &mut keepass::db::Group,
        id: &str,
    ) -> Option<keepass::db::Node> {
        // Try to find and remove from this group's children
        if let Some(pos) = group.children.iter().position(|child| {
            if let keepass::db::Node::Entry(e) = child {
                e.uuid.to_string() == id
            } else {
                false
            }
        }) {
            return Some(group.children.remove(pos));
        }

        // Recursively try child groups
        for child in &mut group.children {
            if let keepass::db::Node::Group(child_group) = child {
                if let Some(found) = Self::find_and_remove_entry(child_group, id) {
                    return Some(found);
                }
            }
        }

        None
    }

    /// Find and remove a group, returning the Node
    fn find_and_remove_group(
        group: &mut keepass::db::Group,
        id: &str,
    ) -> Option<keepass::db::Node> {
        // Try to remove from children
        if let Some(pos) = group.children.iter().position(|child| {
            if let keepass::db::Node::Group(g) = child {
                g.uuid.to_string() == id
            } else {
                false
            }
        }) {
            return Some(group.children.remove(pos));
        }

        // Try recursively
        for child in &mut group.children {
            if let keepass::db::Node::Group(child_group) = child {
                if let Some(found) = Self::find_and_remove_group(child_group, id) {
                    return Some(found);
                }
            }
        }

        None
    }

    /// Check if an entry or group is in the recycle bin
    fn is_in_recycle_bin(root: &keepass::db::Group, id: &str) -> bool {
        // Find recycle bin
        for child in &root.children {
            if let keepass::db::Node::Group(g) = child {
                if g.name == "Recycle Bin" || g.name == "回收站" {
                    // Check if the item is in recycle bin
                    return Self::contains_item(g, id);
                }
            }
        }
        false
    }

    /// Check if a group contains an item (entry or group) with the given ID
    fn contains_item(group: &keepass::db::Group, id: &str) -> bool {
        for child in &group.children {
            match child {
                keepass::db::Node::Entry(e) => {
                    if e.uuid.to_string() == id {
                        return true;
                    }
                }
                keepass::db::Node::Group(g) => {
                    if g.uuid.to_string() == id {
                        return true;
                    }
                    // Recursively check child groups
                    if Self::contains_item(g, id) {
                        return true;
                    }
                }
            }
        }
        false
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

    #[test]
    fn test_add_and_get_entry() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path().join("test.kdbx");

        let mut vault = Vault::create(&vault_path, "test123", VaultConfig::default()).unwrap();

        // Get root group ID
        let groups = vault.get_groups().unwrap();
        let root_id = groups[0].id.clone();

        // Create and add an entry
        let mut entry = Entry::new("Test Entry".to_string(), root_id.clone());
        entry.username = "testuser".to_string();
        entry.password = "testpass".to_string();
        entry.url = "https://example.com".to_string();

        let entry_id = vault.add_entry(entry.clone()).unwrap();

        // Save and reload
        vault.save().unwrap();
        drop(vault);

        let vault = Vault::open(&vault_path, "test123").unwrap();

        // Verify entry was saved
        let retrieved = vault.get_entry(&entry_id).unwrap();
        assert_eq!(retrieved.title, "Test Entry");
        assert_eq!(retrieved.username, "testuser");
        assert_eq!(retrieved.password, "testpass");
    }

    #[test]
    fn test_update_entry() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path().join("test.kdbx");

        let mut vault = Vault::create(&vault_path, "test123", VaultConfig::default()).unwrap();

        let groups = vault.get_groups().unwrap();
        let root_id = groups[0].id.clone();

        // Add entry
        let mut entry = Entry::new("Original".to_string(), root_id);
        entry.username = "user1".to_string();
        let entry_id = vault.add_entry(entry.clone()).unwrap();

        // Update entry
        entry.title = "Updated".to_string();
        entry.username = "user2".to_string();
        vault.update_entry(&entry_id, entry).unwrap();

        // Verify update
        let updated = vault.get_entry(&entry_id).unwrap();
        assert_eq!(updated.title, "Updated");
        assert_eq!(updated.username, "user2");
    }

    #[test]
    fn test_delete_entry() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path().join("test.kdbx");

        let mut vault = Vault::create(&vault_path, "test123", VaultConfig::default()).unwrap();

        let groups = vault.get_groups().unwrap();
        let root_id = groups[0].id.clone();

        // Add entry
        let entry = Entry::new("To Delete".to_string(), root_id);
        let entry_id = vault.add_entry(entry).unwrap();

        // Verify it exists
        assert!(vault.get_entry(&entry_id).is_ok());

        // Delete it (moves to recycle bin)
        vault.delete_entry(&entry_id).unwrap();

        // Verify it still exists (in recycle bin)
        assert!(vault.get_entry(&entry_id).is_ok());

        // Permanently delete it
        vault.permanently_delete_entry(&entry_id).unwrap();

        // Now it should be gone
        assert!(vault.get_entry(&entry_id).is_err());
    }

    #[test]
    fn test_add_and_get_group() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path().join("test.kdbx");

        let mut vault = Vault::create(&vault_path, "test123", VaultConfig::default()).unwrap();

        let groups = vault.get_groups().unwrap();
        let root_id = groups[0].id.clone();

        // Add a new group
        let group = Group::new("Test Group".to_string(), Some(root_id));
        let group_id = vault.add_group(group).unwrap();

        // Verify it was added
        let all_groups = vault.get_groups().unwrap();
        assert!(all_groups
            .iter()
            .any(|g| g.id == group_id && g.name == "Test Group"));
    }
}
