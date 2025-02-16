use crate::{protected_command, VaultState};
use password_manager_backend::{category_favorite::category::Category, error::app_error::AppError};
use uuid::Uuid;
use tauri::State;

#[tauri::command]
pub async fn create_category(
    state: State<'_, VaultState>,
    name: String,
) -> Result<(), AppError> {
    protected_command!(state, {
        let mut vault_manager_lock = state.vault_manager.write().await;
        let manager = vault_manager_lock.as_mut().ok_or(AppError::VaultNotFound)?;
        let _ = manager.entry_collection.categories.create_category(name)?;
        state.storage.save_vault(manager).await?;
        Ok(())
    })
}

#[tauri::command]
pub async fn rename_category(
    state: State<'_, VaultState>,
    id: Uuid,
    new_name: String,
) -> Result<(), AppError> {
    protected_command!(state, {
        let mut vault_manager_lock = state.vault_manager.write().await;
        let manager = vault_manager_lock.as_mut().ok_or(AppError::VaultNotFound)?;
        manager.entry_collection.categories.rename_category(
            id,
            new_name,
            &mut manager.entry_collection.entries
        )?;
        state.storage.save_vault(manager).await?;
        Ok(())
    })
}

#[tauri::command]
pub async fn delete_category(
    state: State<'_, VaultState>,
    id: Uuid,
) -> Result<(), AppError> {
    protected_command!(state, {
        let mut vault_manager_lock = state.vault_manager.write().await;
        let manager = vault_manager_lock.as_mut().ok_or(AppError::VaultNotFound)?;
        manager.entry_collection.categories.delete_category(
            &id,
            &mut manager.entry_collection.entries
        )?;
        state.storage.save_vault(manager).await?;
        Ok(())
    })
}


#[tauri::command]
pub async fn search_categories(
    state: State<'_, VaultState>,
    query: String,
) -> Result<Vec<Category>, AppError> {
    protected_command!(state, {
        let vault_manager_lock = state.vault_manager.read().await;
        let manager = vault_manager_lock.as_ref().ok_or(AppError::VaultNotFound)?;
        let categories = manager.entry_collection.categories.search_categories(&query);
        Ok(categories.into_iter().cloned().collect())
    })
}

