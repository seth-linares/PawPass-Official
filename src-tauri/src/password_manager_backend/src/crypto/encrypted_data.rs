use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};
use crate::error::crypto_error::CryptoError;

#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct EncryptedData {
    ciphertext: Vec<u8>,
    nonce: [u8; 12],     // 96 bits for AES-GCM
    tag: [u8; 16],       // 128 bits authentication tag
}

impl EncryptedData {
    /// Creates a new EncryptedData instance with validation
    pub fn new(ciphertext: Vec<u8>, nonce: [u8; 12], tag: [u8; 16]) -> Result<Self, CryptoError> {
        let encrypted_data = Self {
            ciphertext,
            nonce,
            tag,
        };
        encrypted_data.validate()?;
        Ok(encrypted_data)
    }

    /// Validates the encrypted data structure
    pub fn validate(&self) -> Result<(), CryptoError> {
        // Ciphertext must not be empty
        if self.ciphertext.is_empty() {
            return Err(CryptoError::InvalidCiphertext);
        }
        Ok(())
    }

    /// Returns a reference to the ciphertext
    pub fn ciphertext(&self) -> &[u8] {
        &self.ciphertext
    }

    /// Returns a reference to the nonce
    pub fn nonce(&self) -> &[u8; 12] {
        &self.nonce
    }

    /// Returns a reference to the authentication tag
    pub fn tag(&self) -> &[u8; 16] {
        &self.tag
    }

    /// Creates a zeroed instance (useful for testing)
    #[cfg(test)]
    pub fn zero() -> Self {
        Self {
            ciphertext: vec![0u8; 32],
            nonce: [0u8; 12],
            tag: [0u8; 16],
        }
    }
}

// No Default implementation to prevent accidental creation of invalid instances