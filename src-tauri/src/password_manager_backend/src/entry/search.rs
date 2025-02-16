use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SearchIndex {
    pub title_index: HashMap<String, HashSet<Uuid>>,
    pub username_index: HashMap<String, HashSet<Uuid>>,
    pub url_index: HashMap<String, HashSet<Uuid>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub text: Option<String>,
    #[serde(rename = "categoryName")]
    pub category_name: Option<String>,
    #[serde(rename = "favoritesOnly")]
    pub favorites_only: bool,
}

impl SearchQuery {
    pub fn new(text: Option<String>, category_name: Option<String>, favorites_only: bool) -> Self {
        Self {
            text,
            category_name,
            favorites_only,
        }
    }
}

impl SearchIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry(&mut self, id: Uuid, title: &str, username: &Option<String>, url: &Option<String>) {
        // Index title
        if !title.is_empty() {
            let key = title.to_lowercase();
            self.title_index
                .entry(key)
                .or_insert_with(HashSet::new)
                .insert(id);
        }

        // Index username
        if let Some(username) = username {
            if !username.is_empty() {
                let key = username.to_lowercase();
                self.username_index
                    .entry(key)
                    .or_insert_with(HashSet::new)
                    .insert(id);
            }
        }

        // Index URL
        if let Some(url) = url {
            if !url.is_empty() {
                let key = url.to_lowercase();
                self.url_index
                    .entry(key)
                    .or_insert_with(HashSet::new)
                    .insert(id);
            }
        }
    }

    pub fn remove_entry(&mut self, id: Uuid, title: &str, username: &Option<String>, url: &Option<String>) {
        // Remove from title index
        if !title.is_empty() {
            let key = title.to_lowercase();
            if let Some(ids) = self.title_index.get_mut(&key) {
                ids.remove(&id);
                if ids.is_empty() {
                    self.title_index.remove(&key);
                }
            }
        }

        // Remove from username index
        if let Some(username) = username {
            if !username.is_empty() {
                let key = username.to_lowercase();
                if let Some(ids) = self.username_index.get_mut(&key) {
                    ids.remove(&id);
                    if ids.is_empty() {
                        self.username_index.remove(&key);
                    }
                }
            }
        }

        // Remove from URL index
        if let Some(url) = url {
            if !url.is_empty() {
                let key = url.to_lowercase();
                if let Some(ids) = self.url_index.get_mut(&key) {
                    ids.remove(&id);
                    if ids.is_empty() {
                        self.url_index.remove(&key);
                    }
                }
            }
        }
    }

    pub fn search_text(&self, text: &str) -> HashSet<Uuid> {
        let text_lower = text.to_lowercase();
        let mut matched_ids = HashSet::new();

        // Search all indexes
        for (key, ids) in &self.title_index {
            if key.contains(&text_lower) {
                matched_ids.extend(ids);
            }
        }

        for (key, ids) in &self.username_index {
            if key.contains(&text_lower) {
                matched_ids.extend(ids);
            }
        }

        for (key, ids) in &self.url_index {
            if key.contains(&text_lower) {
                matched_ids.extend(ids);
            }
        }

        matched_ids
    }
}