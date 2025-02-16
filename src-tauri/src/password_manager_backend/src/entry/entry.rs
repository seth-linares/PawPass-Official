use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use zeroize::{Zeroize, ZeroizeOnDrop};
use crate::crypto::KeyHierarchy;
use crate::error::entry_error::EntryError;
use crate::entry::{
    sensitive_data::{EncryptedSensitiveData, DecryptedSensitiveData},
    overview::{EntryOverview, EntryData}
};


/// Represents a complete password entry with both public and encrypted sensitive data
#[derive(Serialize, Deserialize)]
pub struct Entry {
    // Core fields
    pub(crate) id: Uuid,
    pub(crate) title: String,
    pub(crate) username: Option<String>,
    pub(crate) url: Option<String>,
    #[serde(rename = "categoryId")]
    pub(crate) category_id: Option<Uuid>,
    #[serde(rename = "categoryName")]
    pub(crate) category_name: Option<String>,
    pub(crate) favorite: bool,

    // Timestamps
    #[serde(rename = "createdAt")]
    pub(crate) created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub(crate) updated_at: DateTime<Utc>,

    // Encrypted sensitive data
    pub(crate) sensitive_data: EncryptedSensitiveData,
}



impl Entry {
    /// Creates a new Entry from validated input
    ///
    /// This method is only meant to be called by EntryCollection to ensure
    /// proper validation and indexing of entries.
    pub(crate) fn new_internal(input: EntryData, category_id: Option<Uuid>, key_hierarchy: &KeyHierarchy) -> Result<Self, EntryError> {
        input.validate()?;

        // only need to use now() once and will make sure that the timestamps match
        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            title: input.title,
            username: input.username,
            url: input.url,
            category_id: category_id,
            category_name: input.category_name.map(|s| s.to_string()),
            favorite: input.favorite,
            created_at: now,
            updated_at: now,
            sensitive_data: EncryptedSensitiveData::new(input.password, input.notes, key_hierarchy)?,
        })
    }

    /// Update the Entry; only meant to be called by EntryCollection
    pub(crate) fn update_internal(&mut self,
                                  changes: EntryData,
                                  category_id: Option<Uuid>,
                                  key_hierarchy: &KeyHierarchy)
        -> Result<(), EntryError> {
        changes.validate()?;

        *self = Self {
            id: self.id,
            title: changes.title,
            username: changes.username,
            url: changes.url,
            category_id: category_id,
            category_name: changes.category_name.map(|s| s.to_string()),
            favorite: changes.favorite,
            created_at: self.created_at,
            updated_at: Utc::now(),
            sensitive_data: EncryptedSensitiveData::new(changes.password, changes.notes, key_hierarchy)?,
        };

        Ok(())
    }

    /// Securely and temporarily access encrypted data; used in Tauri Commands
    pub fn decrypt(&self, key_hierarchy: &KeyHierarchy) -> Result<DecryptedEntry, EntryError> {
        // Destructure decrypted password and notes from our EncryptedSensitiveData object
        let (password, notes) = self.sensitive_data.decrypt(key_hierarchy)?
            .map(|data: DecryptedSensitiveData| {
                (
                    data.password.map(|p| unsafe{ p.into_inner() }),
                    data.notes.map(|n| unsafe{ n.into_inner() })
                )
            })
            .unwrap_or((None, None));

        Ok(DecryptedEntry {
            id: self.id,
            title: self.title.clone(),
            username: self.username.clone(),
            url: self.url.clone(),
            category_id: self.category_id,
            category_name: self.category_name.clone(),
            favorite: self.favorite,
            created_at: self.created_at,
            updated_at: self.updated_at,
            password,
            notes,
        })
    }

    /// Created an EntryOverview object to display the entry on the dashboard
    pub(crate) fn to_overview(&self) -> EntryOverview {
        EntryOverview {
            id: self.id,
            title: self.title.clone(),
            username: self.username.clone(),
            url: self.url.clone(),
            category_id: self.category_id,
            category_name: self.category_name.clone(),
            favorite: self.favorite,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }



    // Getter methods

    pub fn id(&self) -> &Uuid { &self.id }

    pub fn title(&self) -> &str { &self.title }

    pub fn username(&self) -> Option<&str> { self.username.as_deref() }

    pub fn url(&self) -> Option<&str> { self.url.as_deref() }

    pub fn category_id(&self) -> Option<&Uuid> { self.category_id.as_ref() }

    pub fn favorite(&self) -> bool { self.favorite }

    pub fn created_at(&self) -> DateTime<Utc> { self.created_at }

    pub fn updated_at(&self) -> DateTime<Utc> { self.updated_at }

}

// Custom Debug implementation to protect sensitive data
impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry")
            .field("id", &self.id)
            .field("title", &self.title)
            .field("username", &self.username)
            .field("url", &self.url)
            .field("category_id", &self.category_id)
            .field("favorite", &self.favorite)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .field("sensitive_data", &"[REDACTED]")
            .finish()
    }
}

impl Zeroize for Entry {
    fn zeroize(&mut self) {
        self.title.zeroize();
        if let Some(username) = &mut self.username {
            username.zeroize();
        }
        if let Some(url) = &mut self.url {
            url.zeroize();
        }
        // category_id and id are UUIDs and don't need zeroizing
        self.sensitive_data.zeroize();
        // timestamps don't need zeroizing
    }
}

impl ZeroizeOnDrop for Entry {}

/// Represents a fully decrypted entry with all fields accessible
#[derive(Serialize, Deserialize)]
pub struct DecryptedEntry {
    pub id: Uuid,
    pub title: String,
    pub username: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "categoryId")]
    pub category_id: Option<Uuid>,
    #[serde(rename = "categoryName")]
    pub category_name: Option<String>,
    pub favorite: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub password: Option<String>,
    pub notes: Option<String>,
}

impl Zeroize for DecryptedEntry {
    fn zeroize(&mut self) {
        self.title.zeroize();
        if let Some(username) = &mut self.username {
            username.zeroize();
        }
        if let Some(url) = &mut self.url {
            url.zeroize();
        }
        if let Some(password) = &mut self.password {
            password.zeroize();
        }
        if let Some(notes) = &mut self.notes {
            notes.zeroize();
        }
    }
}

impl ZeroizeOnDrop for DecryptedEntry {}

