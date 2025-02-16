// password_manager_backend/src/category_favorite/favorite_collection.rs

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct FavoriteCollection {
    // Store favorite entry IDs in a HashSet for O(1) lookups
    favorite_entries: HashSet<Uuid>,
    entry_count: usize,
}

impl FavoriteCollection {
    /// Create a new empty collection
    pub fn new() -> Self {
        Self {
            favorite_entries: HashSet::new(),
            entry_count: 0,
        }
    }


    pub fn add_favorite(&mut self, entry_id: Uuid) -> bool {
        self.entry_count += 1;
        self.favorite_entries.insert(entry_id)
    }

    
    pub fn remove_favorite(&mut self, entry_id: &Uuid) -> bool {
        if self.favorite_entries.remove(entry_id) {
            self.entry_count -= 1;
            true
        } else {
            false
        }
    }

    pub fn toggle_favorite(&mut self, entry_id: Uuid) -> bool {
        if self.favorite_entries.contains(&entry_id) {
            self.favorite_entries.remove(&entry_id);
            false
        } else {
            self.favorite_entries.insert(entry_id);
            true
        }
    }

    /// Check if an entry is favorited
    pub fn is_favorite(&self, entry_id: &Uuid) -> bool {
        self.favorite_entries.contains(entry_id)
    }

    /// Get all favorite entry IDs
    /// Returns a reference to avoid cloning the whole set
    pub fn get_all_favorites(&self) -> &HashSet<Uuid> {
        &self.favorite_entries
    }

    /// Get count of favorite entries
    pub fn favorite_count(&self) -> usize {
        self.entry_count
    }
}
