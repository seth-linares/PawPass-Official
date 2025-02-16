use serde::Serialize;
use thiserror::Error;
use crate::error::{
    auth_error::AuthError,
    crypto_error::CryptoError,
    entry_error::EntryError,
    password_generation_error::PasswordGenerationError,
};

#[derive(Debug, Error, Serialize)]
pub enum VaultError {
    // State errors
    #[error("Operation failed: vault is locked")]
    VaultLocked,
    
    #[error("Vault is already unlocked")]
    VaultAlreadyUnlocked,
    
    #[error("Vault is not initialized")]
    VaultNotInitialized,
    
    #[error("Vault already exists at specified path")]
    VaultAlreadyExists,
    
    #[error("Vault not found at specified path")]
    VaultNotFound,

    // Component errors
    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),
    
    #[error("Entry error: {0}")]
    EntryError(#[from] EntryError),
    
    #[error("Cryptographic error: {0}")]
    CryptoError(#[from] CryptoError),
    
    #[error("Password generation error: {0}")]
    PasswordGenerationError(#[from] PasswordGenerationError),

    // File/Storage errors
    #[error("Storage error: {0}")]
    AppError(String),
    
    #[error("Invalid vault path specified")]
    InvalidVaultPath,
    
    #[error("IO error: {0}")]
    IoError(String),

    // Settings errors
    #[error("Invalid key derivation settings")]
    InvalidKeyDerivationSettings,
    
    #[error("Invalid password generator settings")]
    InvalidPasswordGeneratorSettings,

    #[error("Settings are out of sync between components")]
    SettingsOutOfSync,

    // Other errors
    #[error("Auto-lock error: {0}")]
    AutoLockError(String),
    
    #[error("Backup error: {0}")]
    BackupError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Invalid password")]
    InvalidPassword,

    // CBOR serialization errors
    #[error("CBOR serialization error: {0}")]
    SerializationError(String),
}

impl From<std::io::Error> for VaultError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}

impl From<serde_cbor::error::Error> for VaultError {
    fn from(err: serde_cbor::error::Error) -> Self {
        Self::SerializationError(err.to_string())
    }
}