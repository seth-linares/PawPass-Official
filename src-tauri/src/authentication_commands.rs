use password_manager_backend::{crypto::{KeyDerivation, SecureMemory}, error::app_error::AppError};
use crate::{protected_command, helpers, VaultState};

use tauri::State;

// Maybe should return entry overviews rather than nothing, but perhaps separation of concerns is better
#[tauri::command]
pub async fn login(
    state: State<'_, VaultState>, 
    password: String
) -> Result<(), AppError> {

    println!("Logging in hopefully");

    // Get write lock before performing operations
    let mut vault_manager_lock = state.vault_manager.write().await;
    let mut key_hierarchy_lock = state.key_hierarchy.write().await;

    println!("Locks acquired, loading vault");
    // Load the vault from storage
    let mut loaded_manager = state.storage.load_vault().await?;

    println!("Vault loaded, attempting to login");
    // Validate the master password and login
    let key_hierarchy = loaded_manager.login(SecureMemory::new(password))?;

    println!("Login successful, saving vault");

    // Update state while still holding the locks
    vault_manager_lock.replace(loaded_manager);
    key_hierarchy_lock.replace(key_hierarchy);

    // Set session active after successful login
    state.set_session_active(true).await;

    println!("Successfully logged in");
    
    Ok(())
}

#[tauri::command]
pub async fn logout(
    state: State<'_, VaultState>
) -> Result<(), AppError> {
    println!("Starting logout process");
    
    // Get write locks before performing operations
    let mut vault_manager_lock = state.vault_manager.write().await;
    let mut key_hierarchy_lock = state.key_hierarchy.write().await;

    // Early return if vault is not loaded
    if vault_manager_lock.is_none() {
        return Err(AppError::VaultNotFound);
    }

    // Get mutable reference to vault manager
    let manager = vault_manager_lock.as_mut().ok_or(AppError::VaultNotFound)?;

    println!("BEFORE Auth Service Key Derivation:\n{:?}", manager.auth_service.key_derivation());
    println!("BEFORE Vault Manager Key Derivation:\n{:?}", manager.key_derivation_settings);

    // Save vault state before clearing
    println!("Saving vault state before logout");
    state.storage.save_vault(manager).await?;

    println!("Auth Service Key Derivation:\n{:?}", manager.auth_service.key_derivation());
    println!("Vault Manager Key Derivation:\n{:?}", manager.key_derivation_settings);

    // Take ownership of key hierarchy and clear it
    if let Some(key_hierarchy) = key_hierarchy_lock.take() {
        println!("Clearing key hierarchy");
        drop(key_hierarchy);
    }

    println!("Deactivating session");
    // Deactivate session before clearing state
    state.set_session_active(false).await;

    // Clear the vault manager state
    println!("Clearing vault state");
    vault_manager_lock.take();
    
    Ok(())
}

#[tauri::command]
pub async fn change_master_password(
    state: State<'_, VaultState>, 
    old_password: String, 
    new_password: String, 
    confirm_password: String
) -> Result<(), AppError> {
    protected_command!(state, {
        helpers::validate_password(&new_password, &confirm_password)?;

        let mut vault_manager_lock = state.vault_manager.write().await;
        let mut key_hierarchy_lock = state.key_hierarchy.write().await;

        let manager = vault_manager_lock.as_mut().ok_or(AppError::VaultNotFound)?;
        let key_hierarchy = manager.change_master_password(SecureMemory::new(old_password), SecureMemory::new(new_password))?;

        state.storage.save_vault(manager).await?;
        key_hierarchy_lock.replace(key_hierarchy);
        
        Ok(())
    })
}

#[tauri::command]
pub async fn update_key_derivation(
    state: State<'_, VaultState>, 
    master_password: String, 
    settings: KeyDerivation
) -> Result<(), AppError> {
    protected_command!(state, {
        println!("⭐ Starting key derivation update with settings: {:?}", settings);
        
        let mut vault_manager_lock = state.vault_manager.write().await;
        let mut key_hierarchy_lock = state.key_hierarchy.write().await;
        
        let manager = vault_manager_lock.as_mut().ok_or(AppError::VaultNotFound)?;
        let new_key_hierarchy = manager.update_key_derivation(
            SecureMemory::new(master_password), 
            settings
        )?;

        println!("\n\nORIGINAL MEK {:?}\n\n", key_hierarchy_lock.as_ref().unwrap().mek);
        
        println!("🔄 Updating key hierarchy in state");
        println!("\n\n\n\nOld Key Hierarchy: {:?}\n\n", key_hierarchy_lock.as_ref());
        println!("New Key Hierarchy: {:?}\n\n", new_key_hierarchy);

        key_hierarchy_lock.replace(new_key_hierarchy);

        println!("NOW UPDATED Key Hierarchy: {:?}\n\n\n\n", key_hierarchy_lock.as_ref());

        println!("BEFORE Auth Service Key Derivation:\n{:?}", manager.auth_service.key_derivation());
        println!("BEFORE Vault Manager Key Derivation:\n{:?}", manager.key_derivation_settings);

        println!("\n\nFINAL MEK {:?}\n\n\n\n", key_hierarchy_lock.as_ref().unwrap().mek);

        println!("💾 Saving vault with updated settings");
        state.storage.save_vault(manager).await?;

        println!("Auth Service Key Derivation:\n{:?}", manager.auth_service.key_derivation());
        println!("Vault Manager Key Derivation:\n{:?}", manager.key_derivation_settings);
        
        println!("✅ Key derivation update complete");
        Ok(())
    })
}