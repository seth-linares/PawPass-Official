// password_manager_backend/src/category/category_collection.rs

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use crate::category_favorite::category::Category;
use crate::error::category_error::CategoryError;
use crate::entry::Entry;

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryCollection {
    // Main storage for categories
    categories: HashMap<Uuid, Category>,

    // Relationships between categories and entries
    category_entry_index: HashMap<Uuid, HashSet<Uuid>>,
}

impl CategoryCollection {
    // Create a new empty collection
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
            category_entry_index: HashMap::new(),
        }
    }

    // Create a new category with the given name
    // Returns the ID of the newly created category
    pub fn create_category(&mut self, name: String) -> Result<Uuid, CategoryError> {
        // Validate the name
        let name = name.trim();
        if name.is_empty() {
            return Err(CategoryError::InvalidName(name.to_string()));
        }

        // Check for duplicate category name (case-insensitive)
        if self
            .categories
            .values()
            .any(|cat| cat.name().eq_ignore_ascii_case(name))
        {
            return Err(CategoryError::DuplicateName(name.to_string()));
        }

        // Generate new UUID for category
        let id = Uuid::new_v4();

        // Create new Category struct
        let category = Category::new(id, name.to_string());

        // Add to categories map
        self.categories.insert(id, category);

        // Initialize empty entry set in index
        self.category_entry_index.insert(id, HashSet::new());

        // Return ID
        Ok(id)
    }

    // Rename an existing category by ID
    pub fn rename_category(
        &mut self,
        id: Uuid,
        new_name: String,
        entries: &mut HashMap<Uuid, Entry>
    ) -> Result<(), CategoryError> {
        // Validate the new name
        let new_name = new_name.trim();
        if new_name.is_empty() {
            return Err(CategoryError::InvalidName(new_name.to_string()));
        }

        // Check for duplicate category name (case-insensitive)
        if self
            .categories
            .values()
            .any(|cat| cat.id() != id && cat.name().eq_ignore_ascii_case(new_name))
        {
            return Err(CategoryError::DuplicateName(new_name.to_string()));
        }

        // Find category and update its name
        if let Some(category) = self.categories.get_mut(&id) {
            category.name = new_name.to_string();
            
            // Update category_name in all associated entries
            if let Some(entry_ids) = self.category_entry_index.get(&id) {
                for entry_id in entry_ids {
                    if let Some(entry) = entries.get_mut(entry_id) {
                        entry.category_name = Some(new_name.to_string());
                    }
                }
            }
            Ok(())
        } else {
            Err(CategoryError::NotFound(id))
        }
    }

    // Delete a category and remove all entry associations
    pub fn delete_category(
        &mut self,
        id: &Uuid,
        entries: &mut HashMap<Uuid, Entry>
    ) -> Result<(), CategoryError> {
        // Get associated entries before removing the category
        if let Some(entry_ids) = self.category_entry_index.get(id) {
            // Clone the HashSet to avoid borrow checker issues
            let entry_ids: HashSet<_> = entry_ids.iter().cloned().collect();
            
            // Update all associated entries
            for entry_id in entry_ids {
                if let Some(entry) = entries.get_mut(&entry_id) {
                    entry.category_id = None;
                    entry.category_name = None;
                }
            }
        }

        // Remove category from categories map
        if self.categories.remove(id).is_some() {
            // Remove entry associations from index
            self.category_entry_index.remove(id);
            Ok(())
        } else {
            Err(CategoryError::NotFound(*id))
        }
    }

    pub fn add_entry_to_category(
        &mut self,
        category_id: &Uuid,
        entry_id: Uuid,
    ) -> Result<(), CategoryError> {
        // Find category in index
        if let Some(entry_set) = self.category_entry_index.get_mut(category_id) {
            entry_set.insert(entry_id);
            Ok(())
        } else {
            Err(CategoryError::NotFound(*category_id))
        }
    }

    // Remove an entry from a category
    pub fn remove_entry_from_category(
        &mut self,
        category_id: &Uuid,
        entry_id: &Uuid,
    ) -> Result<(), CategoryError> {
        // Find category in index
        if let Some(entry_set) = self.category_entry_index.get_mut(category_id) {
            entry_set.remove(entry_id);
            Ok(())
        } else {
            Err(CategoryError::NotFound(*category_id))
        }
    }

    // Get all entries in a category
    pub fn get_entries_in_category(
        &self,
        category_name: String,
    ) -> Result<&HashSet<Uuid>, CategoryError> {

        // Find category by name
        let category_id = self.get_category_id_by_name(&category_name)
            .ok_or_else(|| CategoryError::NotFoundByName(category_name))?;

        self.category_entry_index
            .get(&category_id)
            .ok_or_else(|| CategoryError::NotFound(category_id))
    }

    // Get category by ID
    pub fn get_category(&self, id: &Uuid) -> Option<&Category> {
        self.categories.get(id)
    }

    // Get multiple categories by ID for searching
    pub fn get_categories_by_ids(&self, ids: &[Uuid]) -> HashMap<Uuid, &Category> {
        ids.iter()
            .filter_map(|id| self.get_category(id).map(|cat| (*id, cat)))
            .collect()
    }

    // Get category ID by name
    pub fn get_category_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.categories
            .values()
            .find(|cat| cat.name().eq_ignore_ascii_case(name))
            .map(|cat| cat.id())
    }


    // Get all categories
    pub fn get_all_categories(&self) -> Vec<Category> {
        self.categories.values().cloned().collect()
    }

    // Search categories by name for dropdown menu when creating/editing entry (case-insensitive partial match)
    pub fn search_categories(&self, query: &str) -> Vec<&Category> {
        // If query is empty, return all categories
        if query.trim().is_empty() {
            return self.categories.values().collect();
        }
        
        // Otherwise, perform the search
        let query = query.to_lowercase();
        self.categories
            .values()
            .filter(|cat| cat.name().to_lowercase().contains(&query))
            .collect()
    }
}
