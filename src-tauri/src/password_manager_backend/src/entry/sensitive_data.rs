use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};
use crate::crypto::{KeyHierarchy, EncryptedData, SecureMemory};
use crate::entry::{MAX_NOTES_LENGTH, MAX_PASS_CHAR_LENGTH};
use crate::error::entry_error::EntryError;
use crate::error::crypto_error::CryptoError::DecryptionFailed;

fn encrypt_data(data: Option<String>, key_hierarchy: &KeyHierarchy) -> Result<Option<EncryptedData>, EntryError> {
    match data {
        Some(text) if !text.is_empty() => {
            Ok(Some(key_hierarchy.encrypt_data(text.as_bytes()).map_err(EntryError::EncryptionError)?))
        }
        _ => Ok(None)
    }
}

/// Represents encrypted sensitive data fields of an entry.
/// All fields are optional to support entries that may not have
/// sensitive data or only have certain fields filled.
#[derive(Debug, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct EncryptedSensitiveData {
    pub(crate) password: Option<EncryptedData>,
    pub(crate) notes: Option<EncryptedData>,
}

impl EncryptedSensitiveData {


    /// Creates a new EncryptedSensitiveData by encrypting the provided password and optional notes
    pub fn new(
        password: Option<String>,
        notes: Option<String>,
        key_hierarchy: &KeyHierarchy,
    ) -> Result<Self, EntryError> {

        if let Some(pass) = &password {
            if pass.len() > MAX_PASS_CHAR_LENGTH {
                return Err(EntryError::InvalidLength("Password".to_string()))
            }
        }
        if let Some(note) = &notes {
            if note.len() > MAX_NOTES_LENGTH {
                return Err(EntryError::InvalidLength("Notes".to_string()))
            }
        }

        Ok(Self {
            password: encrypt_data(password, key_hierarchy)?,
            notes: encrypt_data(notes, key_hierarchy)?,
        })
    }

    /// Decrypts the sensitive data using the provided key hierarchy
    pub fn decrypt(&self, key_hierarchy: &KeyHierarchy)
                   -> Result<Option<DecryptedSensitiveData>, EntryError>
    {
        // Early return if no sensitive data
        if self.password.is_none() && self.notes.is_none() {
            return Ok(None);
        }

        // Helper function for decrypting single field
        fn decrypt_field(encrypted: &EncryptedData, key_hierarchy: &KeyHierarchy) -> Result<SecureMemory<String>, EntryError> {
            let bytes = key_hierarchy
                .decrypt_data(encrypted)
                .map_err(EntryError::DecryptionError)?;

            String::from_utf8(bytes)
                .map_err(|_| EntryError::DecryptionError(DecryptionFailed))
                .map(SecureMemory::new)
        }

        // Decrypt each field
        let password = self.password
            .as_ref()
            .map(|enc| decrypt_field(enc, key_hierarchy))
            .transpose()?;

        let notes = self.notes
            .as_ref()
            .map(|enc| decrypt_field(enc, key_hierarchy))
            .transpose()?;

        Ok(Some(DecryptedSensitiveData { password, notes }))
    }
}


pub struct DecryptedSensitiveData {
    pub(crate) password: Option<SecureMemory<String>>,
    pub(crate) notes: Option<SecureMemory<String>>,
}


// impl DecryptedSensitiveData {
//     /// Returns a reference to the decrypted password
//     pub fn password(&self) -> Option<&String> {
//         self.password.as_ref().map(|p| p.as_ref())
//     }

//     /// Returns a reference to the decrypted notes if they exist
//     pub fn notes(&self) -> Option<&String> {
//         self.notes.as_ref().map(|n| n.as_ref())
//     }
// }