use std::path::PathBuf;
use std::sync::Arc;
use chrono::Utc;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::error::app_error::AppError;


use super::vault_manager::VaultManager;


pub struct VaultStorage {
    // File system paths will not change -> set to app data directory
    pub vault_path: PathBuf,          // Where the main vault file lives
    pub backup_dir: PathBuf,          // Directory for backup files
    pub temp_dir: PathBuf,            // Directory for temporary files during atomic saves

    // Concurrency control
    pub file_lock: Arc<RwLock<()>>,   // Prevents concurrent file access
}

impl VaultStorage {
    pub fn new() -> Self {
        let app_data_dir = dirs::data_dir().expect("Could not find the app data directory");
        let vault_path = app_data_dir.join("PawPass");
        let backup_dir = vault_path.join("backups");
        let temp_dir = vault_path.join("temp");

        Self {
            vault_path,
            backup_dir,
            temp_dir,
            file_lock: Arc::new(RwLock::new(())),
        }
    }

    // Load the vault manager from disk
    pub async fn load_vault(&self) -> Result<VaultManager, AppError> {
        // Acquire read lock to prevent concurrent access
        let _guard = self.file_lock.read().await;

        println!("Loading vault from disk");
        // Ensure directories exist first
        self.ensure_directories().await?;
        println!("Directories ensured");

        // Attempt to read the vault file
        let vault_data = tokio::fs::read(self.vault_path.join("vault.dat"))
            .await
            .map_err(|e| AppError::from(e))?;
        println!("Vault data read");

        // Deserialize the vault data
        let vault_manager: VaultManager = serde_cbor::from_slice(&vault_data)
            .map_err(|e| AppError::DeserializationFailed(e.to_string()))?;
        println!("Vault deserialized"); 

        Ok(vault_manager)
    }


    pub async fn save_vault(&self, vault_manager: &VaultManager) -> Result<(), AppError> {
        // Use write lock instead of read lock since saving is a critical operation
        // that shouldn't happen concurrently with other saves or loads
        let _guard = self.file_lock.write().await;

        // Ensure directories exist first
        self.ensure_directories().await?;


        // Generate unique temporary filename to prevent conflicts
        let temp_filename = format!("vault_{}.tmp", Uuid::new_v4());
        let temp_path = self.temp_dir.join(temp_filename);
        let final_path = self.vault_path.join("vault.dat");

        // Serialize and write in separate steps for better error handling
        let serialized = serde_cbor::to_vec(&vault_manager)
            .map_err(|e| AppError::SerializationFailed(e.to_string()))?;

        // Write to temporary file with specific permissions
        tokio::fs::write(&temp_path, &serialized)
            .await
            .map_err(|e| AppError::TemporaryFileError(e.to_string()))?;

        // Verify the written data by reading it back
        let verification_data = tokio::fs::read(&temp_path)
            .await
            .map_err(|e| AppError::VerificationFailed(e.to_string()))?;

        if verification_data != serialized {
            // Clean up and return error if verification fails
            let _ = tokio::fs::remove_file(&temp_path).await;
            return Err(AppError::DataCorruption("Serialization verification failed during save".to_string()));
        }

        // Perform atomic rename
        tokio::fs::rename(&temp_path, &final_path)
            .await
            .map_err(|e| AppError::AtomicSaveFailed(
                temp_path.to_string_lossy().to_string(), 
                e.to_string() 
            ))?;

        // Clean up any old temporary files that might exist
        self.cleanup_temp_files().await?;

        Ok(())
    }


    pub async fn create_backup(&self, vault_manager: &mut VaultManager) -> Result<(), AppError> {
        // Acquire write lock to ensure consistency during backup
        let _guard = self.file_lock.write().await;


        // Create backup filename with timestamp for easy identification
        let timestamp = Utc::now();

        vault_manager.last_backup_time = timestamp;

        let backup_filename = format!(
            "vault_backup_{}.dat",
            timestamp.format("%Y%m%d_%H%M%S")
        );
        let backup_path = self.backup_dir.join(backup_filename);
        // Serialize vault manager
        let serialized = serde_cbor::to_vec(vault_manager)
            .map_err(|e| AppError::SerializationFailed(e.to_string()))?;

        // Ensure backup directory exists
        self.ensure_directories().await?;


        // Write to temporary backup file first
        let temp_backup = self.temp_dir.join(format!("backup_{}.tmp", Uuid::new_v4()));
        tokio::fs::write(&temp_backup, &serialized)
            .await
            .map_err(|e| AppError::BackupCreationFailed(
                temp_backup.to_string_lossy().to_string(),
                e.to_string(),
            ))?;

        // Move temporary backup to final location
        tokio::fs::rename(&temp_backup, &backup_path)
            .await
            .map_err(|e| AppError::AtomicSaveFailed(
                temp_backup.to_string_lossy().to_string(),
                e.to_string()
            ))?;

        self.cleanup_temp_files().await?;


        Ok(())
    }

    pub async fn restore_from_file(&self, backup_path: PathBuf) -> Result<VaultManager, AppError> {
        // First verify the file exists and is readable
        if !backup_path.exists() {
            return Err(AppError::PathNotFound(
                backup_path.to_string_lossy().to_string()
            ));
        }
    
        // Read and validate the backup file
        let backup_data = tokio::fs::read(&backup_path)
            .await
            .map_err(|e| AppError::BackupRestoreFailed(
                backup_path.to_string_lossy().to_string(),
                e.to_string(),
            ))?;
    
        let restored_vault: VaultManager = serde_cbor::from_slice(&backup_data)
            .map_err(|e| AppError::DeserializationFailed(e.to_string()))?;
    
        // Acquire the write lock for atomic access
        let _guard = self.file_lock.write().await;
    
        // Write the restored vault to disk
        let serialized = serde_cbor::to_vec(&restored_vault)
            .map_err(|e| AppError::SerializationFailed(e.to_string()))?;
    
        // Write to temporary file first
        let temp_file = self.temp_dir.join(format!("vault_{}.tmp", uuid::Uuid::new_v4()));
        tokio::fs::write(&temp_file, &serialized)
            .await
            .map_err(|e| AppError::TemporaryFileError(e.to_string()))?;
    
        // Atomic rename to final location
        tokio::fs::rename(&temp_file, self.vault_path.join("vault.dat"))
            .await
            .map_err(|e| AppError::BackupRestoreFailed(
                backup_path.to_string_lossy().to_string(),
                e.to_string(),
            ))?;
    
        self.cleanup_temp_files().await?;
        Ok(restored_vault)
    }

    pub async fn vault_exists(&self) -> bool {
        self.vault_path.join("vault.dat").exists()
    }

}

impl VaultStorage {
    // Getters
    pub fn get_vault_path(&self) -> &PathBuf {
        &self.vault_path
    }
    pub fn get_backup_dir(&self) -> &PathBuf {
        &self.backup_dir
    }
    pub fn get_temp_dir(&self) -> &PathBuf {
        &self.temp_dir
    }
    pub fn get_file_lock(&self) -> Arc<RwLock<()>> {
        self.file_lock.clone()
    }
}

// Helper methods
impl VaultStorage {
    /// Ensures that all required directories exist, creating them if they don't
    /// Returns `Ok(())` if successful, or an `AppError` if creation fails
    pub async fn ensure_directories(&self) -> Result<(), AppError> {
        // Iterate through all required directory paths
        for dir in [&self.vault_path, &self.backup_dir, &self.temp_dir] {
            // create_dir_all() will:
            // 1. Do nothing if the directory already exists
            // 2. Create the directory and all parent directories if they don't exist
            tokio::fs::create_dir_all(dir)
                .await
                .map_err(|e| match e.kind() {
                    // Convert permission errors to a specific error type
                    std::io::ErrorKind::PermissionDenied =>
                        AppError::InsufficientPermissions(dir.to_string_lossy().to_string()),
                    // Convert all other errors to a general directory creation error
                    _ => AppError::DirectoryCreationFailed(e.to_string())
                })?;
        }
        Ok(())
    }

    // Helper method to clean up old temporary files
    async fn cleanup_temp_files(&self) -> Result<(), AppError> {
        let mut dir = tokio::fs::read_dir(&self.temp_dir)
            .await
            .map_err(|e| AppError::DirectoryReadFailed(e.to_string()))?;

        while let Ok(Some(entry)) = dir.next_entry().await {
            if entry.file_name().to_string_lossy().ends_with(".tmp") {
                let _ = tokio::fs::remove_file(entry.path()).await;
            }
        }

        Ok(())
    }

}

impl Default for VaultStorage {
    fn default() -> Self {
        Self::new()
    }
}