// Search and filter module

use crate::entry::Entry;

/// Search entries by query
pub fn search_entries(entries: &[Entry], query: &str) -> Vec<Entry> {
    let query_lower = query.to_lowercase();

    entries
        .iter()
        .filter(|entry| {
            entry.title.to_lowercase().contains(&query_lower)
                || entry.username.to_lowercase().contains(&query_lower)
                || entry.url.to_lowercase().contains(&query_lower)
                || entry.notes.to_lowercase().contains(&query_lower)
        })
        .cloned()
        .collect()
}

/// Filter entries by tag
pub fn filter_by_tag(entries: &[Entry], tag: &str) -> Vec<Entry> {
    entries
        .iter()
        .filter(|entry| entry.tags.contains(&tag.to_string()))
        .cloned()
        .collect()
}

/// Get favorite entries
pub fn get_favorites(entries: &[Entry]) -> Vec<Entry> {
    entries
        .iter()
        .filter(|entry| entry.is_favorite)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_entries() {
        let entry = Entry::new("Test Entry".to_string(), "group-1".to_string());
        let entries = vec![entry];

        let results = search_entries(&entries, "test");
        assert_eq!(results.len(), 1);

        let results = search_entries(&entries, "nonexistent");
        assert_eq!(results.len(), 0);
    }
}
