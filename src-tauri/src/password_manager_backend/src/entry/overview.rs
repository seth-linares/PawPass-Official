use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use crate::entry::{MAX_NOTES_LENGTH, MAX_PASS_CHAR_LENGTH, MAX_TITLE_LENGTH, MAX_URL_LENGTH, MAX_USERNAME_LENGTH};
use crate::error::entry_error::EntryError;

/// Validates a URL string according to the password manager's requirements
pub(crate) fn validate_url(url: &str) -> Result<(), EntryError> {
    if url.len() > MAX_URL_LENGTH {
        return Err(EntryError::InvalidLength("URL".to_string()));
    }
    // Only parse non-empty URLs
    if !url.is_empty() {
        Url::parse(url).map_err(|_| EntryError::InvalidUrl)?;
    }
    Ok(())
}

/// Lightweight view model for dashboard display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryOverview {
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
}

/// Input structure for creating new entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryData {
    // Title is the only required field when creating a new entry
    pub title: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
    #[serde(rename = "categoryName")]
    pub category_name: Option<String>,
    pub favorite: bool,
}

impl EntryData {
    /// Validates the new entry data
    pub fn validate(&self) -> Result<(), EntryError> {
        // Title validation
        if self.title.is_empty() {
            return Err(EntryError::InvalidTitle);
        }
        if self.title.len() > MAX_TITLE_LENGTH {
            return Err(EntryError::InvalidLength("Title".to_string()));
        }

        // Username validation
        if let Some(username) = &self.username {
            if username.len() > MAX_USERNAME_LENGTH {
                return Err(EntryError::InvalidLength("Username".to_string()));
            }
        }

        // Password validation if present
        if let Some(password) = &self.password {
            if password.len() > MAX_PASS_CHAR_LENGTH {
                return Err(EntryError::InvalidLength("Password".to_string()));
            }
        }

        // URL validation
        if let Some(url) = &self.url {
            validate_url(url)?;
        }

        // Notes validation
        if let Some(notes) = &self.notes {
            if notes.len() > MAX_NOTES_LENGTH {
                return Err(EntryError::InvalidLength("Notes".to_string()));
            }
        }

        Ok(())
    }
}