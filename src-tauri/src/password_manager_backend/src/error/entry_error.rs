use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;
use crate::error::crypto_error::CryptoError;
use crate::error::category_error::CategoryError;

#[derive(Debug, Error, Serialize)]
pub enum EntryError {
    // Cryptographic operation errors
    #[error("Encryption error: {0}")]
    EncryptionError(#[from] CryptoError),
    
    #[error("Decryption error: {0}")]
    DecryptionError(CryptoError),

    // Category Errors
    #[error(transparent)]
    CategoryError(#[from] CategoryError),

    // Validation errors
    #[error("Invalid password: cannot be empty")]
    InvalidPassword,
    
    #[error("Invalid title: cannot be empty")]
    InvalidTitle,
    
    #[error("Invalid URL format")]
    InvalidUrl,
    
    #[error("{0} exceeds maximum length")]
    InvalidLength(String),

    // Entry operation errors
    #[error("Entry not found with ID: {0}")]
    NotFound(Uuid),
    
    #[error("Entry with this title already exists")]
    DuplicateEntry,

    // Other errors
    #[error("Validation error: {0}")]
    ValidationError(String),
}
