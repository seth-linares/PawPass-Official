use crate::{protected_command, VaultState};
use uuid::Uuid;
use password_manager_backend::{entry::{collection::EnhancedSearchResults, entry::DecryptedEntry, overview::EntryData, search::SearchQuery}, error::app_error::AppError};
use tauri::State;

#[tauri::command]
pub async fn create_entry(
    state: State<'_, VaultState>, 
    entry: EntryData
) -> Result<(), AppError> {
    protected_command!(state, {
        let mut vault_manager_lock = state.vault_manager.write().await;
        let key_hierarchy_lock = state.key_hierarchy.read().await;
        let manager = vault_manager_lock.as_mut().ok_or(AppError::VaultNotFound)?;
        let key_hierarchy = key_hierarchy_lock.as_ref().ok_or(AppError::VaultLocked)?;
        manager.entry_collection.create_entry(entry, key_hierarchy)?;
        state.storage.save_vault(manager).await?;
        Ok(())
    })
}

#[tauri::command]
pub async fn update_entry( 
    state: State<'_, VaultState>, 
    entry: EntryData,
    id: Uuid
) -> Result<(), AppError> {
    protected_command!(state, {
        let mut vault_manager_lock = state.vault_manager.write().await;
        let key_hierarchy_lock = state.key_hierarchy.read().await;
        let manager = vault_manager_lock.as_mut().ok_or(AppError::VaultNotFound)?;
        let key_hierarchy = key_hierarchy_lock.as_ref().ok_or(AppError::VaultLocked)?;
        manager.entry_collection.update_entry(&id, entry, key_hierarchy)?;
        state.storage.save_vault(manager).await?;
        Ok(())
    })
}

#[tauri::command]
pub async fn delete_entry(
    state: State<'_, VaultState>, 
    id: Uuid
) -> Result<(), AppError> {
    protected_command!(state, {
        let mut vault_manager_lock = state.vault_manager.write().await;
        let manager = vault_manager_lock.as_mut().ok_or(AppError::VaultNotFound)?;
        manager.entry_collection.delete_entry(&id)?;
        state.storage.save_vault(manager).await?;
        Ok(())
    })
}

// Get Decrypted Entry to fill in fields for the "Edit Entry" page
#[tauri::command]
pub async fn get_decrypted_entry(
    state: State<'_, VaultState>,
    id: Uuid,
) -> Result<DecryptedEntry, AppError> {
    protected_command!(state, {
        let key_hierarchy_lock = state.key_hierarchy.read().await;
        let vault_manager_lock = state.vault_manager.read().await;
        let key_hierarchy = key_hierarchy_lock.as_ref().ok_or(AppError::VaultLocked)?;
        let manager = vault_manager_lock.as_ref().ok_or(AppError::VaultNotFound)?;
        let entry = manager.entry_collection.get_decrypted_entry(&id, key_hierarchy)?;
        Ok(entry)
    })
}


// This becomes our primary command for retrieving entries and their metadata
#[tauri::command]
pub async fn search_entries(
    state: State<'_, VaultState>,
    query: SearchQuery,
) -> Result<EnhancedSearchResults, AppError> {
    protected_command!(state, {
        let vault_manager_lock = state.vault_manager.read().await;
        let manager = vault_manager_lock.as_ref().ok_or(AppError::VaultNotFound)?;
        
        // When no filters are provided, this returns all entries with their category distribution
        Ok(manager.entry_collection.search(query))
    })
}
