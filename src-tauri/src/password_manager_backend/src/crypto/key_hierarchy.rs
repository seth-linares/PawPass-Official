use rand::{RngCore, rngs::OsRng};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};

use crate::crypto::secure::SecureMemory;
use crate::crypto::key_derivation::KeyDerivation;
use crate::crypto::encrypted_data::EncryptedData;
use crate::crypto::KEY_SIZE;
use crate::error::crypto_error::CryptoError;

/// Key Hierarchy structure that contains the:
/// ```markdown
/// 1. Master key
/// 2. MEK
/// 3. Key Derivation `struct`
/// ```
#[derive(Debug)]
pub struct KeyHierarchy {
    pub master_key: SecureMemory<Vec<u8>>,
    pub mek: SecureMemory<Vec<u8>>,
    pub key_derivation: KeyDerivation,
}

impl KeyHierarchy {
    /// Creates a new KeyHierarchy from a master password
    /// Returns the KeyHierarchy and the salt used for master key derivation
    pub fn new(master_password: &[u8]) -> Result<(Self, Vec<u8>), CryptoError> {
        // Create key derivation with default parameters
        let key_derivation = KeyDerivation::default();

        // Generate salt for master key
        let salt = key_derivation.generate_salt()?;

        // Derive master key
        let master_key = key_derivation.derive_key(master_password, &salt)?;

        // Generate random MEK
        let mek = Self::generate_mek()?;

        Ok((Self {
            master_key,
            mek,
            key_derivation,
        }, salt))
    }
    
    pub fn from_existing(
        master_password: &[u8],
        encrypted_mek: &EncryptedData,
        salt: &[u8],
        key_derivation_settings: KeyDerivation,
    ) -> Result<Self, CryptoError> {
        let key_derivation = key_derivation_settings;
        key_derivation.validate_salt(salt)?;
        
        let master_key = key_derivation.derive_key(master_password, salt)?;
        
        let mek_vec = Self::decrypt_mek_with_key(encrypted_mek, master_key.as_ref())
            .map_err(|_| CryptoError::MekDecryptionFailed)?;

        if mek_vec.len() != KEY_SIZE {
            return Err(CryptoError::InvalidKeyLength);
        }

        let mek = SecureMemory::new(mek_vec);

        Ok(Self {
            master_key,
            mek,
            key_derivation,
        })
    }

    /// Changes the master password and returns the new salt
    pub fn change_master_password(
        &mut self,
        old_password: &[u8],
        new_password: &[u8],
        old_salt: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        println!("🔐 KeyHierarchy: Starting master password change");
        println!("📊 Current KeyHierarchy state:");
        println!("  Master key length: {}", self.master_key.as_ref().len());
        println!("  MEK length: {}", self.mek.as_ref().len());
        println!("  Key derivation settings: {:?}", self.key_derivation);
        println!("  Old password length: {}", old_password.len());
        println!("  New password length: {}", new_password.len());
        println!("  Old salt length: {}", old_salt.len());
        
        println!("🔍 Verifying old password by deriving key");
        let old_master_key = self.key_derivation.derive_key(old_password, old_salt)?;

        println!("🔄 Comparing derived key with stored master key");
        if old_master_key.as_ref() != self.master_key.as_ref() {
            println!("❌ Password verification failed: derived key doesn't match stored key");
            return Err(CryptoError::InvalidMasterPassword);
        }

        println!("🎲 Generating new salt");
        let new_salt = self.key_derivation.generate_salt()?;

        println!("🔑 Deriving new master key");
        let new_master_key = self.key_derivation.derive_key(new_password, &new_salt)?;

        println!("📝 Updating master key");
        self.master_key = new_master_key;

        println!("✅ Master password change completed successfully");
        Ok(new_salt)
    }

    pub fn update_key_derivation(
        &mut self,
        master_password: &[u8],
        old_salt: &[u8],
        new_key_derivation: KeyDerivation
    ) -> Result<(Vec<u8>, EncryptedData), CryptoError> {
        println!("🔐 KeyHierarchy: Verifying master password");
        if !self.verify_master_password(master_password, old_salt)? {
            println!("❌ Master password verification failed");
            return Err(CryptoError::InvalidMasterPassword);
        }

        println!("🎲 Generating new salt with new settings");
        let new_salt = new_key_derivation.generate_salt()?;

        println!("🔑 Deriving new master key with new parameters");
        let new_master_key = new_key_derivation.derive_key(
            master_password,
            &new_salt
        )?;

        println!("🔒 Re-encrypting MEK with new master key");
        let encrypted_mek = self.encrypted_mek(new_master_key.as_ref())?;

        println!("✅ Validating new encryption by test decryption");
        let decrypted_mek = Self::decrypt_mek_with_key(&encrypted_mek, new_master_key.as_ref())?;
        if decrypted_mek.as_slice() != self.mek.as_ref() {
            println!("❌ Validation failed: MEK mismatch after re-encryption");
            return Err(CryptoError::KeyDerivationFailed);
        }

        println!("📝 Updating KeyHierarchy state");
        self.master_key = new_master_key;
        self.key_derivation = new_key_derivation;

        println!("✨ KeyHierarchy update complete");
        Ok((new_salt, encrypted_mek))
    }

    pub fn encrypted_mek(&self, master_key: &[u8]) -> Result<EncryptedData, CryptoError> {
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::try_from(&nonce_bytes[..])
            .map_err(|_| CryptoError::MekEncryptionFailed)?;

        let cipher = Aes256Gcm::new_from_slice(master_key)
            .map_err(|_| CryptoError::MekEncryptionFailed)?;

        let ciphertext = cipher
            .encrypt(&nonce, self.mek.as_ref().as_slice())
            .map_err(|_| CryptoError::MekEncryptionFailed)?;

        // Extract tag from ciphertext
        let tag_start = ciphertext.len() - 16;
        let mut tag = [0u8; 16];
        tag.copy_from_slice(&ciphertext[tag_start..]);

        let ciphertext = ciphertext[..tag_start].to_vec();

        EncryptedData::new(ciphertext, nonce_bytes, tag)
    }

    /// Encrypts data using the MEK
    pub fn encrypt_data(&self, data: &[u8]) -> Result<EncryptedData, CryptoError> {
        if data.is_empty() {
            return Err(CryptoError::EmptyData);
        }

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::try_from(&nonce_bytes[..])
            .map_err(|_| CryptoError::EncryptionFailed)?;

        let cipher = Aes256Gcm::new_from_slice(self.mek.as_ref())
            .map_err(|_| CryptoError::EncryptionFailed)?;

        let ciphertext = cipher
            .encrypt(&nonce, data)
            .map_err(|_| CryptoError::EncryptionFailed)?;

        // The tag is the last 16 bytes of the ciphertext in AES-GCM
        let tag_start = ciphertext.len() - 16;
        let mut tag = [0u8; 16];
        tag.copy_from_slice(&ciphertext[tag_start..]);

        let ciphertext = ciphertext[..tag_start].to_vec();

        EncryptedData::new(ciphertext, nonce_bytes, tag)
    }

    /// Decrypts data using the MEK
    pub fn decrypt_data(&self, encrypted: &EncryptedData) -> Result<Vec<u8>, CryptoError> {
        let cipher = Aes256Gcm::new_from_slice(self.mek.as_ref())
            .map_err(|_| CryptoError::DecryptionFailed)?;

        let nonce = Nonce::try_from(encrypted.nonce().as_slice())
            .map_err(|_| CryptoError::DecryptionFailed)?;

        // Combine ciphertext and tag for decryption
        let mut ciphertext_with_tag = encrypted.ciphertext().to_vec();
        ciphertext_with_tag.extend_from_slice(encrypted.tag());

        cipher
            .decrypt(&nonce, ciphertext_with_tag.as_slice())
            .map_err(|_| CryptoError::DecryptionFailed)
    }

    pub fn verify_master_password(&self, password: &[u8], salt: &[u8]) -> Result<bool, CryptoError> {
        let derived_key = self.key_derivation.derive_key(password, salt)?;
        Ok(derived_key.as_ref() == self.master_key.as_ref())
    }

    pub fn decrypt_mek_with_key(encrypted_mek: &EncryptedData, master_key: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let cipher = Aes256Gcm::new_from_slice(master_key)
            .map_err(|_| CryptoError::MekDecryptionFailed)?;

        let nonce = Nonce::try_from(encrypted_mek.nonce().as_slice())
            .map_err(|_| CryptoError::MekDecryptionFailed)?;

        // Combine ciphertext and tag for decryption
        let mut ciphertext_with_tag = encrypted_mek.ciphertext().to_vec();
        ciphertext_with_tag.extend_from_slice(encrypted_mek.tag());

        cipher
            .decrypt(&nonce, ciphertext_with_tag.as_slice())
            .map_err(|_| CryptoError::MekDecryptionFailed)
    }

    // Private helper methods
    fn generate_mek() -> Result<SecureMemory<Vec<u8>>, CryptoError> {
        let mut mek = vec![0u8; KEY_SIZE];
        OsRng.fill_bytes(&mut mek);
        Ok(SecureMemory::new(mek))
    }
}