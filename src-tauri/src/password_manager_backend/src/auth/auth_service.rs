// password_manager_backend/src/auth/auth_service.rs

use serde::{Deserialize, Serialize};
use zeroize::Zeroize;
use crate::crypto::{
    key_derivation::KeyDerivation,
    key_hierarchy::KeyHierarchy,
    secure::SecureMemory,
    EncryptedData,
};
use crate::error::auth_error::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthService {
    // Store password hash for quick validation without full key derivation
    master_password_hash: Vec<u8>,

    // Salt used for password hashing
    salt: Vec<u8>,

    // Encrypted Master Encryption Key (MEK)
    encrypted_mek: EncryptedData,

    // Key derivation settings
    key_derivation: KeyDerivation,
}

impl AuthService {
    /// Creates a new AuthService for a fresh vault
    /// This is called during initial vault setup
    pub fn new(
        master_password: SecureMemory<String>,
    ) -> Result<(Self, KeyHierarchy), AuthError> {

        let password_bytes = master_password.as_ref().as_bytes();

        let (key_hierarchy, salt) = KeyHierarchy::new(password_bytes)
            .map_err(|e| AuthError::from(e))?;

        let master_password_hash;
        unsafe {
            master_password_hash = key_hierarchy.key_derivation.derive_key(password_bytes, salt.as_slice())?.into_inner();
        }

        let encrypted_mek = key_hierarchy.encrypted_mek(&master_password_hash)?;

        let auth_service = AuthService {
            master_password_hash,
            salt,
            encrypted_mek,
            key_derivation: KeyDerivation::default(),
        };

        println!("Created new AuthService");
        Ok((auth_service, key_hierarchy))
    }

    /// Unlocks the _vault with the provided master password
    /// This is what we call after loading the AuthService from disk
    pub fn unlock(
        &mut self,
        master_password: SecureMemory<String>
    ) -> Result<KeyHierarchy, AuthError> {
        let input_bytes = master_password.as_ref().as_bytes();
        let mut input_hash;
        unsafe {
            input_hash = self.key_derivation.derive_key(input_bytes, self.salt.as_slice())?.into_inner();
        }

        if !input_hash.eq(&self.master_password_hash) {
            return Err(AuthError::InvalidPassword);
        }

        // Use the new method with current settings
        let key_hierarchy = KeyHierarchy::from_existing(
            input_bytes, 
            &self.encrypted_mek, 
            &self.salt,
            self.key_derivation.clone()
        )?;

        input_hash.zeroize();

        Ok(key_hierarchy)
    }

    /// Changes the master password
    pub fn change_password(
        &mut self,
        old_password: SecureMemory<String>,
        new_password: SecureMemory<String>,
    ) -> Result<KeyHierarchy, AuthError> {
        let mut old_password = unsafe {
            old_password.into_inner()
        };
    
        // First verify the old password
        self.verify_master_password(SecureMemory::new(old_password.clone()))?;
    
        let new_password_bytes = new_password.as_ref().as_bytes();
        let old_password_bytes = old_password.as_bytes();
    
        // Use current settings when creating new key hierarchy
        let mut key_hierarchy = KeyHierarchy::from_existing(
            old_password_bytes, 
            &self.encrypted_mek, 
            &self.salt,
            self.key_derivation.clone()  // Pass current settings
        )?;
    
        // Update salt
        self.salt = key_hierarchy.change_master_password(
            old_password_bytes, 
            new_password_bytes, 
            &self.salt
        )?;
    
        // Update master password hash using current settings
        unsafe {
            self.master_password_hash = self.key_derivation.derive_key(
                new_password_bytes, 
                &self.salt
            )?.into_inner();
        }
    
        // Update encrypted mek
        self.encrypted_mek = key_hierarchy.encrypted_mek(&self.master_password_hash)?;
    
        old_password.zeroize();
    
        Ok(key_hierarchy)
    }

    pub fn update_key_derivation(
        &mut self,
        master_password: SecureMemory<String>,
        new_key_derivation: KeyDerivation
    ) -> Result<KeyHierarchy, AuthError> {
        println!("🔄 AuthService: Starting key derivation update");
        println!("📊 Current settings - Memory: {}, Time: {}, Parallel: {}", 
            self.key_derivation.memory_cost(),
            self.key_derivation.time_cost(),
            self.key_derivation.parallelism()
        );

        let mut key_hierarchy = KeyHierarchy::from_existing(
            master_password.as_ref().as_bytes(),
            &self.encrypted_mek,
            &self.salt,
            self.key_derivation.clone()
        )?;

        let (new_salt, encrypted_mek) = key_hierarchy.update_key_derivation(
            master_password.as_ref().as_bytes(),
            &self.salt,
            new_key_derivation.clone()
        )?;

        println!("🔒 Updating master password hash with new settings");
        unsafe {
            self.master_password_hash = new_key_derivation
                .derive_key(master_password.as_ref().as_bytes(), &new_salt)?
                .into_inner();
        }

        println!("📝 Updating AuthService state with new values");
        self.key_derivation = new_key_derivation;
        self.encrypted_mek = encrypted_mek;
        self.salt = new_salt;

        println!("✅ AuthService key derivation update complete");
        Ok(key_hierarchy)
    }

    /// Quick check if a password matches without full unlock
    pub fn verify_master_password(
        &self,
        password: SecureMemory<String>,
    ) -> Result<bool, AuthError> {
        // 1. Hash password with stored salt
        let password_bytes = password.as_ref().as_bytes();
        let derived_hash = self
            .key_derivation
            .derive_key(password_bytes, &self.salt)?;

        let result = derived_hash.as_ref().eq(&self.master_password_hash);

        drop(password);
        drop(derived_hash);

        // 2. Compare with stored hash
        Ok(result)
    }

    // Getters
    pub fn key_derivation(&self) -> &KeyDerivation {
        &self.key_derivation
    }

    // Expose encrypted MEK for storage
    pub fn encrypted_mek(&self) -> &EncryptedData {
        &self.encrypted_mek
    }

    // Expose master password hash for storage
    pub fn master_password_hash(&self) -> &[u8] {
        &self.master_password_hash
    }

    // Expose salt for storage
    pub fn salt(&self) -> &[u8] {
        &self.salt
    }
}


impl Zeroize for AuthService {
    fn zeroize(&mut self) {
        self.master_password_hash.zeroize();
        self.salt.zeroize();
        self.encrypted_mek.zeroize();
    }
}