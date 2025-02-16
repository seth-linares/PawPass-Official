use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::crypto::KeyHierarchy;
use crate::category_favorite::category_collection::CategoryCollection;
use crate::category_favorite::favorite_collection::FavoriteCollection;
use crate::entry::Entry;
use crate::entry::entry::DecryptedEntry;
use crate::entry::overview::{EntryData, EntryOverview};
use crate::error::entry_error::EntryError;
use crate::entry::search::{SearchIndex, SearchQuery};

/// Represents detailed search results including category distribution
#[derive(Debug, Serialize)]
pub struct EnhancedSearchResults {
    // The filtered entries
    pub entries: Vec<EntryOverview>,
    // Total count of filtered entries
    #[serde(rename = "totalCount")]
    pub total_count: usize,
    // Maximum count of entries before applying category filter
    #[serde(rename = "maxCount")]
    pub max_count: usize,
    // Distribution of entries across categories in the filtered results
    #[serde(rename = "categoryDistribution")]
    pub category_distribution: Vec<CategoryCount>,
}

/// Represents a category and the count of entries it contains in the current filter
#[derive(Debug, Serialize)]
pub struct CategoryCount {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "entryCount")]
    pub entry_count: usize,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct EntryCollection {
    pub entries: HashMap<Uuid, Entry>,
    pub categories: CategoryCollection,
    pub favorites: FavoriteCollection,
    pub search_index: SearchIndex,
    pub entry_count: usize,
}

impl EntryCollection {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            categories: CategoryCollection::new(),
            favorites: FavoriteCollection::new(),
            search_index: SearchIndex::new(),
            entry_count: 0,
        }
    }

    pub fn create_entry(
        &mut self, 
        entry_data: EntryData,
        key_hierarchy: &KeyHierarchy,
    ) -> Result<Uuid, EntryError> {
        let category_id = if let Some(cat_name) = &entry_data.category_name {
            match self.categories.get_category_id_by_name(cat_name) {
                Some(id) => Some(id),
                None => Some(self.categories.create_category(cat_name.clone())?)
            }
        } else {
            None 
        };

        let entry = Entry::new_internal(entry_data, category_id, key_hierarchy)?;
        let id = entry.id;

        if let Some(cat_id) = category_id {
            self.categories.add_entry_to_category(&cat_id, id)?;
        }

        if entry.favorite {
            self.favorites.add_favorite(id);
        }

        self.add_to_search_indexes(entry.id, &entry.title, &entry.username, &entry.url);
        self.entries.insert(id, entry);
        self.entry_count += 1;

        Ok(id)
    }

    pub fn update_entry(
        &mut self,
        id: &Uuid,
        changes: EntryData, 
        key_hierarchy: &KeyHierarchy,
    ) -> Result<(), EntryError> {
        changes.validate()?;

        let category_id = if let Some(cat_name) = &changes.category_name {
            match self.categories.get_category_id_by_name(cat_name) {
                Some(id) => Some(id),
                None => Some(self.categories.create_category(cat_name.clone())?)
            }
        } else {
            None
        };

        let entry = self.entries.get(id).ok_or(EntryError::NotFound(*id))?;
        let old_data: (Option<Uuid>, bool, String, Option<String>, Option<String>) = (
            entry.category_id,
            entry.favorite,
            entry.title.clone(),
            entry.username.clone(),
            entry.url.clone()
        );

        if old_data.0 != category_id {
            if let Some(old_id) = old_data.0 {
                self.categories.remove_entry_from_category(&old_id, id)?;
            }
            if let Some(new_id) = category_id {
                self.categories.add_entry_to_category(&new_id, *id)?;
            }
        }

        if old_data.1 != changes.favorite {
            if changes.favorite {
                self.favorites.add_favorite(*id);
            } else {
                self.favorites.remove_favorite(id);
            }
        }

        self.remove_from_search_indexes_data(
            *id,
            &old_data.2,
            &old_data.3,
            &old_data.4
        );

        let (new_title, new_username, new_url) = {
            let entry = self.entries.get_mut(id).ok_or(EntryError::NotFound(*id))?;
            entry.update_internal(changes, category_id, key_hierarchy)?;
            (
                entry.title.clone(),
                entry.username.clone(),
                entry.url.clone()
            )
        };

        self.add_to_search_indexes(*id, &new_title, &new_username, &new_url);

        Ok(())
    }

    pub fn delete_entry(&mut self, id: &Uuid) -> Result<(), EntryError> {
        let entry = self.entries.get(id).ok_or(EntryError::NotFound(*id))?;

        let category_id = entry.category_id;
        let title = entry.title.clone();
        let username = entry.username.clone();
        let url = entry.url.clone();
        let was_favorite = entry.favorite;

        self.entries.remove(id);
        self.entry_count -= 1;

        if let Some(category_id) = category_id {
            self.categories.remove_entry_from_category(&category_id, id)?;
        }

        if was_favorite {
            self.favorites.remove_favorite(id);
        }

        self.remove_from_search_indexes_data(*id, &title, &username, &url);

        Ok(())
    }

    pub fn get_decrypted_entry(
        &self,
        id: &Uuid,
        key_hierarchy: &KeyHierarchy,
    ) -> Result<DecryptedEntry, EntryError> {
        let entry = self.entries.get(id).ok_or(EntryError::NotFound(*id))?;
        entry.decrypt(key_hierarchy)
    }

    pub fn get_entry_overview(&self, id: &Uuid) -> Result<EntryOverview, EntryError> {
        let entry = self.entries.get(id).ok_or(EntryError::NotFound(*id))?;
        Ok(entry.to_overview())
    }

    pub fn get_all_overviews(&self) -> Vec<EntryOverview> {
        self.entries
            .values()
            .map(|entry| entry.to_overview())
            .collect()
    }
}

impl EntryCollection {
    fn add_to_search_indexes(
        &mut self,
        id: Uuid,
        title: &str,
        username: &Option<String>,
        url: &Option<String>,) {
        self.search_index.add_entry(id, title, username, url);
    }

    fn remove_from_search_indexes_data(
        &mut self,
        id: Uuid,
        title: &str,
        username: &Option<String>,
        url: &Option<String>,
    ) {
        self.search_index.remove_entry(id, title, username, url);
    }
}

impl EntryCollection {
    pub fn search(&self, query: SearchQuery) -> EnhancedSearchResults {
        // Start with favorites filter if enabled (usually most restrictive)
        let mut result_ids: HashSet<Uuid> = if query.favorites_only {
            self.favorites.get_all_favorites().iter().cloned().collect()
        } else {
            self.entries.keys().cloned().collect()
        };

        // Calculate max_count after favorites filter but before category filter
        let mut max_count = result_ids.len();

        // Apply text search next (second most restrictive)
        if let Some(text) = &query.text {
            let matched_ids = self.search_index.search_text(text);
            result_ids = result_ids.intersection(&matched_ids).cloned().collect();
            max_count = result_ids.len();
        }

        // Apply category filter last
        if let Some(category_name) = &query.category_name {
            if let Ok(category_entries) = self.categories.get_entries_in_category(category_name.to_string()) {
                let category_set: HashSet<_> = category_entries.iter().cloned().collect();
                result_ids = result_ids.intersection(&category_set).cloned().collect();
            } else {
                result_ids.clear();
            }
        }

        // Convert filtered IDs to EntryOverviews
        let entries: Vec<EntryOverview> = result_ids
            .iter()
            .filter_map(|id| self.get_entry_overview(id).ok())
            .collect();

        // Get unique category IDs from filtered entries
        let category_ids: Vec<Uuid> = entries
            .iter()
            .filter_map(|entry| entry.category_id)
            .collect();

        // Batch lookup categories
        let categories = self.categories.get_categories_by_ids(&category_ids);
        
        // Calculate distribution
        let mut category_counts: HashMap<Uuid, (String, usize)> = HashMap::new();
        for entry in &entries {
            if let Some(category_id) = entry.category_id {
                if let Some(category) = categories.get(&category_id) {
                    category_counts
                        .entry(category_id)
                        .or_insert_with(|| (category.name().to_string(), 0))
                        .1 += 1;
                }
            }
        }

        // Convert category counts to sorted Vec<CategoryCount>
        let mut category_distribution: Vec<CategoryCount> = category_counts
            .into_iter()
            .map(|(id, (name, count))| CategoryCount {
                id,
                name,
                entry_count: count,
            })
            .collect();

        // Sort categories by name
        category_distribution.sort_by(|a, b| a.name.cmp(&b.name));

        EnhancedSearchResults {
            total_count: entries.len(),
            max_count,
            entries,
            category_distribution,
        }
    }
}