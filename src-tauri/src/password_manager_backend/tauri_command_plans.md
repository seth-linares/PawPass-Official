```rust
/*
VaultState Design and Command Structure

This document outlines the state management and command interface for a password vault application.
The design uses a hybrid approach combining persistent storage with efficient runtime state management.

Key Design Decisions:
- VaultState serves as the central state container for Tauri commands
- Fine-grained locking for better concurrency
- Stateless password generation with persistent settings
- Clear separation between storage and runtime state

State Structure Overview:
- VaultState: Main state container accessible by Tauri commands
- VaultStorage: Handles persistence and file operations
- VaultManager: Manages vault operations and contains core components
*/

// Main state container for Tauri commands
#[derive(Default)]
pub struct VaultState {
    // Core vault components with granular locking
    vault_manager: Arc<RwLock<Option<VaultManager>>>,
    storage: Arc<RwLock<VaultStorage>>,
    // Stateless password generator for concurrent operations
    password_generator: PasswordGenerator,
}

// Status information returned by get_vault_status
pub struct VaultStatus {
    is_locked: bool,
    is_initialized: bool,
    last_modified: Option<DateTime<Utc>>,
    entry_count: usize,
    category_count: usize,
}

/*
Command Interface Implementation:
All commands are async and use tauri::State<'_, VaultState>
Commands are grouped by functionality
*/

// === Authentication Commands (All Stateful) ===

#[tauri::command]
async fn init_vault(
    state: tauri::State<'_, VaultState>,
    password: String
) -> Result<(), VaultError> {
    // Initializes a new vault with master password
    // Requires: No existing vault
    // Creates: New VaultManager, AuthService
    // Errors: VaultAlreadyExists, InvalidPassword
}

#[tauri::command]
async fn login(
    state: tauri::State<'_, VaultState>,
    password: String
) -> Result<(), AuthError> {
    // Unlocks the vault with master password
    // Requires: Initialized vault
    // Validates: Master password
    // Errors: InvalidPassword, VaultUnlocked
}

#[tauri::command]
async fn logout(
    state: tauri::State<'_, VaultState>
) -> Result<(), AuthError> {
    // Locks the vault and clears sensitive memory
    // Requires: Unlocked vault
    // Errors: VaultLocked
}

#[tauri::command]
async fn change_master_password(
    state: tauri::State<'_, VaultState>,
    old_password: String,
    new_password: String
) -> Result<(), AuthError> {
    // Changes vault master password
    // Requires: Unlocked vault, valid old password
    // Updates: Encrypted MEK
    // Errors: InvalidPassword, PasswordMismatch
}

// === Entry Management Commands (All Stateful) ===

#[tauri::command]
async fn create_entry(
    state: tauri::State<'_, VaultState>,
    entry_data: EntryData
) -> Result<Uuid, EntryError> {
    // Creates new encrypted entry
    // Requires: Unlocked vault
    // Returns: New entry UUID
    // Errors: InvalidTitle, DuplicateEntry, InvalidUrl
}

#[tauri::command]
async fn get_entry_details(
    state: tauri::State<'_, VaultState>,
    id: Uuid
) -> Result<DecryptedEntry, EntryError> {
    // Retrieves and decrypts entry
    // Requires: Unlocked vault
    // Returns: Decrypted entry data
    // Errors: NotFound, DecryptionError
}

// === Password Generation Commands (Stateless) ===

#[tauri::command]
async fn generate_password(
    state: tauri::State<'_, VaultState>
) -> Result<String, PasswordGenerationError> {
    // Generates password using current settings
    // Uses: Stateless password generator
    // Returns: Generated password
    // Errors: Various PasswordGenerationError types
}

#[tauri::command]
async fn update_password_settings(
    state: tauri::State<'_, VaultState>,
    settings: PasswordGeneratorSettings
) -> Result<(), PasswordGenerationError> {
    // Updates password generator settings
    // Requires: Valid settings configuration
    // Persists: Settings in VaultManager
    // Errors: InvalidLength, ConflictingRequirements
}

// === Vault Management Commands ===

#[tauri::command]
async fn create_backup(
    state: tauri::State<'_, VaultState>
) -> Result<(), AppError> {
    // Creates encrypted backup of vault
    // Requires: Unlocked vault
    // Updates: Backup count and timestamp
    // Errors: BackupCreationFailed, InsufficientPermissions
}

#[tauri::command]
async fn get_vault_status(
    state: tauri::State<'_, VaultState>
) -> Result<VaultStatus, VaultError> {
    // Returns current vault status
    // Provides: Lock state, initialization state, counts
    // Errors: VaultNotInitialized
}

/*
Implementation Notes:
1. All stateful commands should handle VaultLocked errors
2. File operations should use storage lock
3. Vault operations should use vault_manager lock
4. Password generation operations can run concurrently
5. Settings updates must be persisted to disk
*/
```