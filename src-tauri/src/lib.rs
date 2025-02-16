mod helpers;
mod authentication_commands;
mod entry_commands;
mod category_commands;

use std::sync::Arc;
use helpers::VaultPaths;
use password_manager_backend::{
    _vault::{
        password_generation::PasswordGeneratorSettings, 
        vault_manager::VaultManager, 
        vault_storage::VaultStorage
    }, 
    crypto::{KeyDerivation, KeyHierarchy, SecureMemory}, 
    error::app_error::AppError
};
use tauri::{async_runtime::RwLock, State};

// State Management
#[derive(Default)]
pub struct VaultState {
    vault_manager: Arc<RwLock<Option<VaultManager>>>,
    storage: VaultStorage,
    key_hierarchy: Arc<RwLock<Option<KeyHierarchy>>>,
    session_active: Arc<RwLock<bool>>,
}

impl VaultState {
    pub async fn is_session_active(&self) -> bool {
        *self.session_active.read().await
    }

    pub async fn set_session_active(&self, active: bool) {
        let mut session = self.session_active.write().await;
        *session = active;
    }
}

#[derive(serde::Serialize)]
struct VaultStatus {
    #[serde(rename = "sessionActive")]
    session_active: bool,
    #[serde(rename = "keyHierarchyPresent")]
    key_hierarchy_present: bool,
    #[serde(rename = "vaultManagerPresent")]
    vault_manager_present: bool,
}

#[tauri::command]
async fn get_vault_status(state: State<'_, VaultState>) -> Result<VaultStatus, AppError> {
    let session_active = state.is_session_active().await;
    let key_hierarchy_lock = state.key_hierarchy.read().await;
    let vault_manager_lock = state.vault_manager.read().await;

    Ok(VaultStatus {
        session_active,
        key_hierarchy_present: key_hierarchy_lock.is_some(),
        vault_manager_present: vault_manager_lock.is_some(),
    })
}

// Vault Management Commands
#[tauri::command]
async fn get_vault_paths(
    state: State<'_, VaultState>,
) -> Result<VaultPaths, AppError> {
    protected_command!(state, {
        let storage = &state.storage;
        Ok(VaultPaths {
            vault_path: storage.get_vault_path().to_string_lossy().to_string(),
            backup_dir: storage.get_backup_dir().to_string_lossy().to_string(),
            temp_dir: storage.get_temp_dir().to_string_lossy().to_string(),
        })
    })
}

#[tauri::command]
async fn initialize_app(
    state: State<'_, VaultState>, 
    password: String, 
    confirm_password: String
) -> Result<(), AppError> {
    if state.storage.vault_path.join("vault.dat").exists() {
        return Err(AppError::VaultAlreadyExists);
    }

    helpers::validate_password(&password, &confirm_password)?;

    let (manager, key_hierarchy) = VaultManager::new(SecureMemory::new(password))?;
    
    let mut vault_manager_lock = state.vault_manager.write().await;
    let mut key_hierarchy_lock = state.key_hierarchy.write().await;

    state.storage.save_vault(&manager).await?;

    state.set_session_active(true).await;

    // Update both states while holding the locks
    key_hierarchy_lock.replace(key_hierarchy);
    vault_manager_lock.replace(manager);
    
    Ok(())
}

#[tauri::command]
async fn get_key_derivation_settings(state: State<'_, VaultState>) -> Result<KeyDerivation, AppError> {
    let vault_manager_lock = state.vault_manager.read().await;
    let manager = vault_manager_lock.as_ref().ok_or(AppError::VaultNotFound)?;

    println!("Getting key derivation settings");
    let settings = manager.key_derivation_settings.clone();
    println!("Got key derivation settings:\n{:?}", settings);

    Ok(settings)
}

#[tauri::command]
async fn create_backup(
    state: State<'_, VaultState>,
) -> Result<(), AppError> {
    protected_command!(state, {
        let mut vault_manager_lock = state.vault_manager.write().await;
        let manager = vault_manager_lock.as_mut().ok_or(AppError::VaultNotFound)?;

        state.storage.create_backup(manager).await?;
        state.storage.save_vault(manager).await?;
        
        Ok(())
    })
}

#[tauri::command]
async fn restore_from_backup(
    state: State<'_, VaultState>,
    backup_path: String,
) -> Result<(), AppError> {
    let backup_path = std::path::PathBuf::from(backup_path);
    let restored_vault = state.storage.restore_from_file(backup_path).await?;

    // Save the restored vault to the state
    let mut vault_manager_lock = state.vault_manager.write().await;
    vault_manager_lock.replace(restored_vault);

    let new_manager = vault_manager_lock.as_ref().ok_or(AppError::VaultNotFound)?;

    state.storage.save_vault(new_manager).await?;

    Ok(())
}

// Password Generator Commands
#[tauri::command]
async fn get_password_generator_settings(state: State<'_, VaultState>) -> Result<PasswordGeneratorSettings, AppError> {
    protected_command!(state, {
        let vault_manager_lock = state.vault_manager.read().await;
        let manager = vault_manager_lock.as_ref().ok_or(AppError::VaultNotFound)?;
        Ok(manager.password_generator_settings.get_settings())
    })
}

#[tauri::command]
async fn update_password_generator_settings(
    state: State<'_, VaultState>, 
    settings: PasswordGeneratorSettings
) -> Result<(), AppError> {
    protected_command!(state, {
        let mut vault_manager_lock = state.vault_manager.write().await;
        let manager = vault_manager_lock.as_mut().ok_or(AppError::VaultNotFound)?;
        manager.password_generator_settings.configure(settings)?;
        state.storage.save_vault(manager).await?;
        Ok(())
    })
}

#[tauri::command]
async fn generate_password(state: State<'_, VaultState>) -> Result<String, AppError> {
    protected_command!(state, {
        let vault_manager_lock = state.vault_manager.read().await;
        let manager = vault_manager_lock.as_ref().ok_or(AppError::VaultNotFound)?;
        let password = manager.password_generator_settings.generate()?;
        Ok(password)
    })
}


#[tauri::command]
async fn calculate_password_entropy(state: State<'_, VaultState>) -> Result<f64, AppError> {
    protected_command!(state, {
        let vault_manager_lock = state.vault_manager.read().await;
        let manager = vault_manager_lock.as_ref().ok_or(AppError::VaultNotFound)?;
        let entropy = manager.password_generator_settings.calculate_entropy();
        Ok(entropy)
    })
}

#[tauri::command]
async fn check_vault_exists(
    state: State<'_, VaultState>,
) -> Result<bool, AppError> {
    Ok(state.storage.vault_exists().await)
}

// Application Entry Point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(VaultState::default())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let window = tauri::Manager::get_webview_window(_app, "main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Basic Commands
            check_vault_exists,
            
            // Vault Management
            initialize_app,
            get_vault_paths,
            get_key_derivation_settings,
            create_backup,
            restore_from_backup,
            
            // Password Generator
            get_password_generator_settings,
            update_password_generator_settings,
            generate_password,
            calculate_password_entropy,
            
            // Authentication Commands
            authentication_commands::login,
            authentication_commands::logout,
            authentication_commands::change_master_password,
            authentication_commands::update_key_derivation,
            
            // Entry Commands
            entry_commands::create_entry,            
            entry_commands::update_entry,            
            entry_commands::delete_entry,            
            entry_commands::get_decrypted_entry,      
            entry_commands::search_entries,          

            // Category Commands
            category_commands::create_category,
            category_commands::rename_category,
            category_commands::delete_category,
            category_commands::search_categories,
            
            // Vault Status
            get_vault_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}