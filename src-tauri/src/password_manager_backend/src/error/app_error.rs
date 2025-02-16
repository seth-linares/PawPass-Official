use serde::Serialize;
use crate::error::auth_error::AuthError;
use super::{
    category_error::CategoryError, crypto_error::CryptoError, entry_error::EntryError, password_generation_error::PasswordGenerationError, serializable_error::SerializableError, vault_error::VaultError
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // File system related errors
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Path not found: {0}")]
    PathNotFound(String),
    
    #[error("Insufficient permissions for path: {0}")]
    InsufficientPermissions(String),
    
    #[error("Failed to create directory: {0}")]
    DirectoryCreationFailed(String),
    
    #[error("Failed to read directory: {0}")]
    DirectoryReadFailed(String),
    
    #[error("Data corruption: {0}")]
    DataCorruption(String),

    // Master Password errors
    #[error("Passwords do not match")]
    PasswordsDoNotMatch,
    
    #[error("Password is too short")]
    PasswordTooShort,
    
    #[error("Password is too long")]
    PasswordTooLong,

    #[error("Password is too weak")]
    PasswordTooWeak,

    // Password Generation errors
    #[error("Password generation error: {0}")]
    PasswordGenerationError(#[from] PasswordGenerationError),

    // Auth Errors
    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),

    // Data integrity errors
    #[error("Corrupted vault file at {0}: {1}")]
    CorruptedFile(String, String),
    
    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    // Serialization errors
    #[error("Failed to serialize vault data: {0}")]
    SerializationFailed(String),
    
    #[error("Failed to deserialize vault data: {0}")]
    DeserializationFailed(String),

    // Backup related errors
    #[error("Failed to create backup at {0}: {1}")]
    BackupCreationFailed(String, String),
    
    #[error("Failed to restore from backup {0}: {1}")]
    BackupRestoreFailed(String, String),
    
    #[error("Failed to delete backup files")]
    BackupDeletionFailed,

    // Atomic operation errors
    #[error("Atomic save failed for temporary file {0}: {1}")]
    AtomicSaveFailed(String, String),
    
    #[error("Temporary file error: {0}")]
    TemporaryFileError(String),

    // Vault state errors
    #[error("Cannot perform operation: vault is locked")]
    VaultLocked,
    
    #[error("Vault is not initialized")]
    VaultNotInitialized,
    
    #[error("Invalid vault state: {0}")]
    InvalidVaultState(String),
    
    #[error("Vault already exists")]
    VaultAlreadyExists,
    
    #[error("Vault not found")]
    VaultNotFound,

    // Entry errors
    #[error(transparent)]
    EntryError(#[from] EntryError),

    // Category errors
    #[error(transparent)]
    CategoryError(#[from] CategoryError),

    // Crypto errors
    #[error(transparent)]
    CryptoError(#[from] CryptoError),

    // Vault errors
    #[error(transparent)]
    VaultError(#[from] VaultError),
}

// Only keep necessary From implementations that need custom conversion logic
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}

impl From<serde_cbor::Error> for AppError {
    fn from(err: serde_cbor::Error) -> Self {
        if err.is_data() {
            Self::DeserializationFailed(err.to_string())
        } else {
            Self::SerializationFailed(err.to_string())
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Convert AppError into our SerializableError format
        let serializable = match self {
            // Authentication Errors
            AppError::AuthError(auth_err) => {
                match auth_err {
                    AuthError::InvalidPassword => SerializableError::Auth {
                        code: "AUTH001".to_string(),
                        message: "Invalid or incorrect master password".to_string(),
                        details: None,
                    },
                    AuthError::VaultLocked => SerializableError::Auth {
                        code: "AUTH002".to_string(),
                        message: "Vault is locked; unlock it before proceeding".to_string(),
                        details: None,
                    },
                    AuthError::VaultUnlocked => SerializableError::Auth {
                        code: "AUTH003".to_string(),
                        message: "Vault is already unlocked".to_string(),
                        details: None,
                    },
                    AuthError::KeyDerivationError(details) => SerializableError::Auth {
                        code: "AUTH004".to_string(),
                        message: "Key derivation failed".to_string(),
                        details: Some(details.to_string()),
                    },
                    AuthError::OperationError(details) => SerializableError::Auth {
                        code: "AUTH005".to_string(),
                        message: "Operation failed".to_string(),
                        details: Some(details.to_string()),
                    },
                    _ => SerializableError::Auth {
                        code: "AUTH999".to_string(),
                        message: auth_err.to_string(),
                        details: None,
                    },
                }
            },

            // Cryptographic Errors
            AppError::CryptoError(crypto_err) => {
                match crypto_err {
                    CryptoError::KeyDerivationFailed => SerializableError::Crypto {
                        code: "CRYPTO001".to_string(),
                        message: "Key derivation operation failed".to_string(),
                        details: None,
                    },
                    CryptoError::InvalidKeyLength => SerializableError::Crypto {
                        code: "CRYPTO002".to_string(),
                        message: "Invalid key length specified".to_string(),
                        details: None,
                    },
                    CryptoError::EmptyPassword => SerializableError::Crypto {
                        code: "CRYPTO003".to_string(),
                        message: "Password cannot be empty".to_string(),
                        details: None,
                    },
                    CryptoError::InvalidParameters => SerializableError::Crypto {
                        code: "CRYPTO004".to_string(),
                        message: "Invalid parameters provided for operation".to_string(),
                        details: None,
                    },
                    _ => SerializableError::Crypto {
                        code: "CRYPTO999".to_string(),
                        message: crypto_err.to_string(),
                        details: None,
                    },
                }
            },

            // Category Errors
            AppError::CategoryError(cat_err) => {
                match cat_err {
                    CategoryError::InvalidName(name) => SerializableError::Category {
                        code: "CAT001".to_string(),
                        message: "Invalid category name".to_string(),
                        category_name: Some(name.clone()),
                        category_id: None,
                    },
                    CategoryError::DuplicateName(name) => SerializableError::Category {
                        code: "CAT002".to_string(),
                        message: "Category with this name already exists".to_string(),
                        category_name: Some(name.clone()),
                        category_id: None,
                    },
                    CategoryError::NotFound(id) => SerializableError::Category {
                        code: "CAT003".to_string(),
                        message: "Category not found".to_string(),
                        category_id: Some(id.to_string()),
                        category_name: None,
                    },
                    CategoryError::NotFoundByName(name) => SerializableError::Category {
                        code: "CAT004".to_string(),
                        message: "Category not found".to_string(),
                        category_id: None,
                        category_name: Some(name.clone()),
                    },
                    _ => SerializableError::Category {
                        code: "CAT999".to_string(),
                        message: cat_err.to_string(),
                        category_id: None,
                        category_name: None,
                    },
                }
            },

            // Entry Errors
            AppError::EntryError(entry_err) => {
                match entry_err {
                    EntryError::InvalidPassword => SerializableError::Entry {
                        code: "ENTRY001".to_string(),
                        message: "Invalid password: cannot be empty".to_string(),
                        entry_id: None,
                        field_name: Some("password".to_string()),
                    },
                    EntryError::InvalidTitle => SerializableError::Entry {
                        code: "ENTRY002".to_string(),
                        message: "Invalid title: cannot be empty".to_string(),
                        entry_id: None,
                        field_name: Some("title".to_string()),
                    },
                    EntryError::NotFound(id) => SerializableError::Entry {
                        code: "ENTRY003".to_string(),
                        message: "Entry not found".to_string(),
                        entry_id: Some(id.to_string()),
                        field_name: None,
                    },
                    _ => SerializableError::Entry {
                        code: "ENTRY999".to_string(),
                        message: entry_err.to_string(),
                        entry_id: None,
                        field_name: None,
                    },
                }
            },

            // IO/File System Errors
            AppError::IoError(details) => SerializableError::Io {
                code: "IO001".to_string(),
                message: "IO operation failed".to_string(),
                path: None,
                operation: Some(details.clone()),
            },

            AppError::PathNotFound(path) => SerializableError::Io {
                code: "IO002".to_string(),
                message: "Path not found".to_string(),
                path: Some(path.clone()),
                operation: None,
            },

            // Password Generation Errors
            AppError::PasswordGenerationError(gen_err) => {
                let length = match gen_err {
                    PasswordGenerationError::LengthTooShort(len) => Some(*len),
                    PasswordGenerationError::LengthTooLong(len) => Some(*len),
                    PasswordGenerationError::ExcessiveMinimums(_, len) => Some(*len),
                    _ => None,
                };
                
                SerializableError::PasswordGeneration {
                    code: "PWD001".to_string(),
                    message: "Password generation failed".to_string(),
                    length,
                    details: Some(gen_err.to_string()),
                }
            },
            AppError::PasswordTooShort => SerializableError::PasswordGeneration {
                code: "PWD002".to_string(),
                message: "Password is too short".to_string(),
                length: None,
                details: None,
            },
            AppError::PasswordTooLong => SerializableError::PasswordGeneration {
                code: "PWD003".to_string(),
                message: "Password is too long".to_string(),
                length: None,
                details: None,
            },
            AppError::PasswordTooWeak => SerializableError::PasswordGeneration {
                code: "PWD004".to_string(),
                message: "Password does not meet strength requirements".to_string(),
                length: None,
                details: None,
            },
            AppError::PasswordsDoNotMatch => SerializableError::Validation {
                code: "VAL001".to_string(),
                message: "Passwords do not match".to_string(),
                field: Some("password".to_string()),
                reason: None,
            },

            // Vault Errors
            AppError::VaultError(vault_err) => {
                match vault_err {
                    VaultError::SettingsOutOfSync => SerializableError::Vault {
                        code: "VAULT006".to_string(),
                        message: "Settings are out of sync between components".to_string(),
                        path: None,
                        details: None,
                    },
                    _ => SerializableError::Vault {
                        code: "VAULT001".to_string(),
                        message: "Vault operation failed".to_string(),
                        path: None,
                        details: Some(vault_err.to_string()),
                    },
                }
            },
            AppError::VaultLocked => SerializableError::Vault {
                code: "VAULT002".to_string(),
                message: "Vault is locked".to_string(),
                path: None,
                details: None,
            },
            AppError::VaultNotInitialized => SerializableError::Vault {
                code: "VAULT003".to_string(),
                message: "Vault is not initialized".to_string(),
                path: None,
                details: None,
            },
            AppError::VaultAlreadyExists => SerializableError::Vault {
                code: "VAULT004".to_string(),
                message: "Vault already exists".to_string(),
                path: None,
                details: None,
            },
            AppError::VaultNotFound => SerializableError::Vault {
                code: "VAULT005".to_string(),
                message: "Vault not found".to_string(),
                path: None,
                details: None,
            },

            // Data Related Errors
            AppError::DataCorruption(details) => SerializableError::Data {
                code: "DATA001".to_string(),
                message: "Data corruption detected".to_string(),
                details: Some(details.clone()),
            },
            AppError::SerializationFailed(details) => SerializableError::Data {
                code: "DATA002".to_string(),
                message: "Failed to serialize data".to_string(),
                details: Some(details.clone()),
            },
            AppError::DeserializationFailed(details) => SerializableError::Data {
                code: "DATA003".to_string(),
                message: "Failed to deserialize data".to_string(),
                details: Some(details.clone()),
            },

            // Backup Related Errors
            AppError::BackupCreationFailed(path, details) => SerializableError::Vault {
                code: "BACKUP001".to_string(),
                message: "Failed to create backup".to_string(),
                path: Some(path.clone()),
                details: Some(details.clone()),
            },
            AppError::BackupRestoreFailed(path, details) => SerializableError::Vault {
                code: "BACKUP002".to_string(),
                message: "Failed to restore backup".to_string(),
                path: Some(path.clone()),
                details: Some(details.clone()),
            },
            AppError::BackupDeletionFailed => SerializableError::Vault {
                code: "BACKUP003".to_string(),
                message: "Failed to delete backup files".to_string(),
                path: None,
                details: None,
            },

            // Other File System Errors
            AppError::InsufficientPermissions(path) => SerializableError::Io {
                code: "IO003".to_string(),
                message: "Insufficient permissions".to_string(),
                path: Some(path.clone()),
                operation: None,
            },
            AppError::DirectoryCreationFailed(path) => SerializableError::Io {
                code: "IO004".to_string(),
                message: "Failed to create directory".to_string(),
                path: Some(path.clone()),
                operation: None,
            },
            AppError::DirectoryReadFailed(path) => SerializableError::Io {
                code: "IO005".to_string(),
                message: "Failed to read directory".to_string(),
                path: Some(path.clone()),
                operation: None,
            },

            // Fallback for any unhandled errors
            _ => SerializableError::Unknown {
                code: "UNKNOWN".to_string(),
                message: self.to_string(),
                details: None,
            },
        };

        // Let serde handle the actual serialization
        serializable.serialize(serializer)
    }
}