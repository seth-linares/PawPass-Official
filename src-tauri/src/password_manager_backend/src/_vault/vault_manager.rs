use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::_vault::password_generation::PasswordGenerator;
use crate::auth::auth_service::AuthService;
use crate::crypto::{KeyDerivation, KeyHierarchy, SecureMemory};
use crate::entry::collection::EntryCollection;
use crate::error::vault_error::VaultError;

#[derive(Serialize, Deserialize)]
pub struct VaultManager {
    pub auth_service: AuthService,
    pub entry_collection: EntryCollection,
    pub key_derivation_settings: KeyDerivation,
    pub password_generator_settings: PasswordGenerator,
    pub initialized: bool,
    pub last_backup_time: DateTime<Utc>,
}

impl VaultManager {
    pub fn new(master_password: SecureMemory<String>) -> Result<(Self, KeyHierarchy), VaultError> {
        let (auth_service, key_hierarchy) = AuthService::new(master_password)?;

        println!("Creating new VaultManager");
        let manager = Self {
            auth_service,
            entry_collection: EntryCollection::new(),
            key_derivation_settings: KeyDerivation::default(),
            password_generator_settings: PasswordGenerator::default(),
            initialized: true,
            last_backup_time: Default::default(),
        };
        println!("New VaultManager created");

        Ok((manager, key_hierarchy))
    }

    pub fn login(&mut self, master_password: SecureMemory<String>) -> Result<KeyHierarchy, VaultError> {
        println!("VaultManager::login() called, checking validation");
        self.validate_vault_active()?;
        println!("VaultManager::login() called, validation passed");

        println!("VaultManager::login() called, unlocking auth service");
        let key_hierarchy = self.auth_service.unlock(master_password)?;
        println!("VaultManager::login() called, auth service unlocked");

        Ok(key_hierarchy)
    }

    pub fn update_key_derivation(
        &mut self, 
        master_password: SecureMemory<String>, 
        settings: KeyDerivation
    ) -> Result<KeyHierarchy, VaultError> {
        self.validate_vault_active()?;

        // Update both instances of key derivation settings
        let key_hierarchy = self.auth_service.update_key_derivation(master_password, settings.clone())?;
        
        // Update local settings and verify they match
        self.key_derivation_settings = settings;
        
        // Validations to make sure settings are in sync
        if self.key_derivation_settings != *self.auth_service.key_derivation() {
            return Err(VaultError::SettingsOutOfSync);
        }

        Ok(key_hierarchy)
    }
    
    pub fn change_master_password(
        &mut self, 
        old_password: SecureMemory<String>, 
        new_password: SecureMemory<String>
    ) -> Result<KeyHierarchy, VaultError> {
        self.validate_vault_active()?;

        let key_hierarchy = self.auth_service.change_password(old_password, new_password)?;
        Ok(key_hierarchy)
    }

}

impl VaultManager {

    fn validate_vault_active(&self) -> Result<(), VaultError> {
        if !self.initialized {
            return Err(VaultError::VaultNotInitialized);
        }
        Ok(())
    }
}
