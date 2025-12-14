use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Represents a password entry in the vault
///
/// # Security
///
/// This struct implements `ZeroizeOnDrop` to ensure that sensitive fields
/// (password, totp_secret) are securely erased from memory when dropped.
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Entry {
    /// Unique identifier
    #[zeroize(skip)]
    pub id: String,

    /// Parent group ID
    #[zeroize(skip)]
    pub group_id: String,

    /// Entry title
    #[zeroize(skip)]
    pub title: String,

    /// Username
    pub username: String,

    /// Password (encrypted in storage, zeroized on drop)
    pub password: String,

    /// URL
    #[zeroize(skip)]
    pub url: String,

    /// Notes
    pub notes: String,

    /// Tags
    #[zeroize(skip)]
    pub tags: Vec<String>,

    /// TOTP secret (if any, zeroized on drop)
    pub totp_secret: Option<String>,

    /// Custom fields
    pub custom_fields: Vec<CustomField>,

    /// Creation time
    #[zeroize(skip)]
    pub created_at: DateTime<Utc>,

    /// Last modification time
    #[zeroize(skip)]
    pub modified_at: DateTime<Utc>,

    /// Last access time
    #[zeroize(skip)]
    pub accessed_at: DateTime<Utc>,

    /// Expiry time (if any)
    #[zeroize(skip)]
    pub expires_at: Option<DateTime<Utc>>,

    /// Whether this entry is a favorite
    #[zeroize(skip)]
    pub is_favorite: bool,
}

/// Custom field in an entry
///
/// # Security
///
/// Protected fields are zeroized on drop to prevent sensitive data leakage.
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct CustomField {
    #[zeroize(skip)]
    pub key: String,
    pub value: String,
    #[zeroize(skip)]
    pub protected: bool,
}

impl Entry {
    /// Create a new entry with default values
    pub fn new(title: String, group_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            group_id,
            title,
            username: String::new(),
            password: String::new(),
            url: String::new(),
            notes: String::new(),
            tags: Vec::new(),
            totp_secret: None,
            custom_fields: Vec::new(),
            created_at: now,
            modified_at: now,
            accessed_at: now,
            expires_at: None,
            is_favorite: false,
        }
    }

    /// Update the modification timestamp
    pub fn touch(&mut self) {
        self.modified_at = Utc::now();
    }

    /// Mark as accessed
    pub fn mark_accessed(&mut self) {
        self.accessed_at = Utc::now();
    }

    /// Check if the entry has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            expires_at < Utc::now()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_entry() {
        let entry = Entry::new("Test Entry".to_string(), "group-1".to_string());
        assert_eq!(entry.title, "Test Entry");
        assert_eq!(entry.group_id, "group-1");
        assert!(!entry.id.is_empty());
    }

    #[test]
    fn test_entry_not_expired_by_default() {
        let entry = Entry::new("Test".to_string(), "group-1".to_string());
        assert!(!entry.is_expired());
    }
}
