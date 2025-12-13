use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a group (folder) in the vault
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    /// Unique identifier
    pub id: String,

    /// Parent group ID (None for root groups)
    pub parent_id: Option<String>,

    /// Group name
    pub name: String,

    /// Icon identifier
    pub icon_id: u32,

    /// Notes
    pub notes: String,

    /// Whether this is the recycle bin
    pub is_recycle_bin: bool,

    /// Whether this group is expanded in UI
    pub is_expanded: bool,
}

impl Group {
    /// Create a new group
    pub fn new(name: String, parent_id: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            parent_id,
            name,
            icon_id: 48, // Default folder icon
            notes: String::new(),
            is_recycle_bin: false,
            is_expanded: true,
        }
    }

    /// Create the recycle bin group
    pub fn new_recycle_bin() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            parent_id: None,
            name: "Recycle Bin".to_string(),
            icon_id: 43, // Trash icon
            notes: String::new(),
            is_recycle_bin: true,
            is_expanded: false,
        }
    }

    /// Check if this is a root group
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_group() {
        let group = Group::new("Test Group".to_string(), None);
        assert_eq!(group.name, "Test Group");
        assert!(group.is_root());
        assert!(!group.is_recycle_bin);
    }

    #[test]
    fn test_recycle_bin() {
        let bin = Group::new_recycle_bin();
        assert!(bin.is_recycle_bin);
        assert_eq!(bin.name, "Recycle Bin");
    }
}
