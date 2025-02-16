use serde::Serialize;
use thiserror::Error;
use crate::error::crypto_error::CryptoError;

#[derive(Debug, Error, Serialize)]
pub enum AuthError {
    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    #[error("Invalid or incorrect master password")]
    InvalidPassword,

    #[error("New password and confirmation do not match")]
    PasswordMismatch,

    #[error("Vault is locked; unlock it before proceeding")]
    VaultLocked,

    #[error("Vault is already unlocked")]
    VaultUnlocked,

    #[error("Key derivation error: {0}")]
    KeyDerivationError(String),

    #[error("Operation error: {0}")]
    OperationError(String),
}

impl From<CryptoError> for AuthError {
    fn from(err: CryptoError) -> Self {
        AuthError::CryptoError(err.to_string())
    }
}