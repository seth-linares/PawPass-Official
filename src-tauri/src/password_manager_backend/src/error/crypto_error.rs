use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum CryptoError {
    // Key Derivation Errors
    #[error("Key derivation operation failed")]
    KeyDerivationFailed,
    
    #[error("Invalid key length specified")]
    InvalidKeyLength,
    
    #[error("Invalid parameters provided for operation")]
    InvalidParameters,

    // Salt Errors
    #[error("Failed to generate salt")]
    SaltGenerationFailed,
    
    #[error("Invalid salt length")]
    InvalidSaltLength,

    // Memory/Parameter Errors
    #[error("Insufficient memory for operation")]
    InsufficientMemory,
    
    #[error("Invalid memory cost")]
    InvalidMemoryCost,
    
    #[error("Invalid time cost")]
    InvalidTimeCost,
    
    #[error("Invalid parallelism parameter")]
    InvalidParallelism,
    
    #[error("Invalid context length")]
    InvalidContextLength,
    
    #[error("Invalid password length")]
    InvalidPasswordLength,
    
    #[error("Empty password")]
    EmptyPassword,

    // Errors for EncryptedData
    #[error("Invalid or empty ciphertext")]
    InvalidCiphertext,
    
    #[error("Invalid nonce length")]
    InvalidNonceLength,
    
    #[error("Invalid authentication tag length")]
    InvalidTagLength,

    // Key Hierarchy errors
    #[error("Failed to derive master key")]
    MasterKeyDerivationFailed,
    
    #[error("Failed to generate master encryption key")]
    MekGenerationFailed,
    
    #[error("Failed to encrypt master encryption key")]
    MekEncryptionFailed,
    
    #[error("Failed to decrypt master encryption key")]
    MekDecryptionFailed,
    
    #[error("Invalid master password provided")]
    InvalidMasterPassword,
    
    #[error("Cannot encrypt empty data")]
    EmptyData,

    // General Crypto Errors
    #[error("Encryption operation failed")]
    EncryptionFailed,
    
    #[error("Decryption operation failed")]
    DecryptionFailed,
    
    #[error("Invalid or corrupted key")]
    InvalidKey,

    // External Errors
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Argon2 error: {0}")]
    Argon2Error(String),
}

impl From<argon2::Error> for CryptoError {
    fn from(err: argon2::Error) -> Self {
        Self::Argon2Error(err.to_string())
    }
}

impl From<std::io::Error> for CryptoError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}