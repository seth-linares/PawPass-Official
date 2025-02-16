## src/crypto/key_hierarchy.rs

```rust
#[derive(Debug)]
pub struct KeyHierarchy {
    master_key: SecureMemory<Vec<u8>>,
    mek: SecureMemory<Vec<u8>>,
    key_derivation: KeyDerivation,
}

// Implemented Functions
impl KeyHierarchy {
    // Constructor and initialization
    pub fn new(master_password: &[u8]) -> Result<(Self, Vec<u8>), CryptoError> {}
    pub fn from_existing(
        master_password: &[u8],
        encrypted_mek: &EncryptedData,
        salt: &[u8]
    ) -> Result<Self, CryptoError> {}

    // Function to handle updating key derivation parameters
    pub fn update_key_derivation(
        &mut self,
        master_password: &[u8],
        old_salt: &[u8],    
        new_key_derivation: KeyDerivation
    ) -> Result<(Vec<u8>, EncryptedData), CryptoError> {}
    
    // Key management
    pub fn encrypted_mek(&self) -> Result<EncryptedData, CryptoError> {}
    pub fn change_master_password(
        &mut self,
        old_password: &[u8],
        new_password: &[u8],
        old_salt: &[u8]
    ) -> Result<Vec<u8>, CryptoError> {}
    
    // Encryption operations
    pub fn encrypt_data(&self, data: &[u8]) -> Result<EncryptedData, CryptoError> {}
    pub fn decrypt_data(&self, encrypted: &EncryptedData) -> Result<Vec<u8>, CryptoError> {}
    
    // Password verification
    pub fn verify_master_password(&self, password: &[u8], salt: &[u8]) -> Result<bool, CryptoError> {}
}

```

## src/crypto/key_derivation.rs

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyDerivation {
    memory_cost: u32,     // Memory size in KiB
    time_cost: u32,       // Number of iterations
    parallelism: u32,     // Degree of parallelism
    key_length: usize,    // Length of derived keys in bytes
}

// Trait Implementations
impl Default for KeyDerivation {
    fn default() -> Self {}
}

impl KeyDerivation {
    // Constructor and builder
    pub fn new(
        memory_cost: u32,
        time_cost: u32,
        parallelism: u32,
        key_length: usize,
    ) -> Result<Self, CryptoError> {}
    
    // Validate, used when changing key derivation parameters
    pub fn validate(&self) -> Result<&Self, CryptoError> {}
    
    // Salt operations
    pub fn generate_salt(&self) -> Result<Vec<u8>, CryptoError> {}
    pub fn validate_salt(&self, salt: &[u8]) -> Result<(), CryptoError> {}
    
    
    // Key derivation
    pub fn derive_key(
        &self,
        password: &[u8],
        salt: &[u8],
    ) -> Result<SecureMemory<Vec<u8>>, CryptoError> {}
    
    pub fn derive_key_with_context(
        &self,
        password: &[u8],
        salt: &[u8],
        context: &[u8],
    ) -> Result<SecureMemory<Vec<u8>>, CryptoError> {}
    
    // Getters
    pub fn memory_cost(&self) -> u32 {}
    pub fn time_cost(&self) -> u32 {}
    pub fn parallelism(&self) -> u32 {}
    pub fn key_length(&self) -> usize {}
}

```

## src/entry/overview.rs

```rust
/// Lightweight view model for dashboard display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryOverview {
    pub(crate) id: Uuid,
    pub(crate) title: String,
    pub(crate) username: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) category_id: Option<Uuid>,
    pub(crate) favorite: bool,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

/// Input structure for creating new entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryData {
    pub title: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub category_id: Option<Uuid>,
    pub favorite: bool,
}

impl EntryData {
    /// Validates the new entry data
    pub fn validate(&self) -> Result<(), EntryError> {}
}

```

## src/entry/entry.rs

```rust

/// Complete password entry with both public and encrypted sensitive data
#[derive(Serialize, Deserialize)]
pub struct Entry {
    // Core fields
    id: Uuid,
    title: String,
    username: Option<String>,
    url: Option<String>,
    category_id: Option<Uuid>,
    favorite: bool,

    // Timestamps
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    // Encrypted sensitive data
    pub(crate) sensitive_data: EncryptedSensitiveData,
}

/// Fully decrypted entry with all fields accessible
pub struct DecryptedEntry<'a> {
    pub id: &'a Uuid,
    pub title: &'a str,
    pub username: Option<&'a str>,
    pub url: Option<&'a str>,
    pub category_id: Option<&'a Uuid>,
    pub favorite: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub password: Option<SecureMemory<String>>,
    pub notes: Option<SecureMemory<String>>,
}

impl Entry {

    // Getters
    pub fn id(&self) -> &Uuid {}
    pub fn title(&self) -> &str {}
    pub fn username(&self) -> Option<&str> {}
    pub fn url(&self) -> Option<&str> {}
    pub fn category_id(&self) -> Option<&Uuid> {}
    pub fn favorite(&self) -> bool {}
    pub fn created_at(&self) -> DateTime<Utc> {}
    pub fn updated_at(&self) -> DateTime<Utc> {}
}

// Custom Debug implementation to protect sensitive data
impl std::fmt::Debug for Entry {}

```


## src/category_favorite/category_collection.rs

```rust
// CategoryCollection.rs - Manages collection of categories and their relationships
// Handles CRUD operations and entry associations for categories

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryCollection {
    categories: HashMap<Uuid, Category>,                   // Main category storage
    category_entry_index: HashMap<Uuid, HashSet<Uuid>>    // Category-entry relationships
}

impl CategoryCollection {
    // Constructor for new empty collection
    pub fn new() -> Self {}

    // Category Management Operations
    // Creates new category, validates name, prevents duplicates
    // Returns: Result<Uuid, CategoryError>
    pub fn create_category(&mut self, name: String) -> Result<Uuid, CategoryError> {}

    // Updates category name with validation
    pub fn rename_category(&mut self, id: Uuid, new_name: String) -> Result<(), CategoryError> {}

    // Removes category and its entry associations
    pub fn delete_category(&mut self, id: &Uuid) -> Result<(), CategoryError> {}

    // Entry Association Management
    // Links/unlinks entries to categories
    pub fn add_entry_to_category(&mut self, category_id: &Uuid, entry_id: Uuid) -> Result<(), CategoryError> {}
    pub fn remove_entry_from_category(&mut self, category_id: &Uuid, entry_id: &Uuid) -> Result<(), CategoryError> {}

    // Query Operations
    pub fn get_entries_in_category(&self, category_id: &Uuid) -> Result<&HashSet<Uuid>, CategoryError> {}
    pub fn get_category(&self, id: &Uuid) -> Option<&Category> {}
    pub fn get_all_categories(&self) -> Vec<&Category> {}
    pub fn search_categories(&self, query: &str) -> Vec<&Category> {}
}

```

## src/category_favorite/favorite_collection.rs

```rust
// FavoriteCollection.rs - Manages favorite entries in the password manager
// Provides efficient O(1) operations for favoriting/unfavoriting entries

#[derive(Debug, Serialize, Deserialize)]
pub struct FavoriteCollection {
    favorite_entries: HashSet<Uuid>,   // Stores favorite entry IDs efficiently
    entry_count: usize                  // Total number of favorite entries
}

impl FavoriteCollection {
    // Creates new empty favorites collection
    pub fn new() -> Self {}

    // Favorite Management Operations
    // Returns: bool indicating if operation changed collection state
    pub fn add_favorite(&mut self, entry_id: Uuid) -> bool {}
    pub fn remove_favorite(&mut self, entry_id: &Uuid) -> bool {}
    
    // Toggles favorite status
    // Returns: new favorite status (true = favorited)
    pub fn toggle_favorite(&mut self, entry_id: Uuid) -> bool {}

    // Query Operations
    pub fn is_favorite(&self, entry_id: &Uuid) -> bool {}
    pub fn get_all_favorites(&self) -> &HashSet<Uuid> {}
    pub fn favorite_count(&self) -> usize {}
}

```

## src/entry/collection.rs

```rust
// EntryCollection.rs - Core management system for password entries
// Handles CRUD operations, search functionality, and maintains relationships
// between entries, categories, and favorites

// Search query structure for filtering entries
#[derive(Debug)]
pub struct SearchQuery {
    text: Option<String>,         // Optional text to search across all fields
    category_id: Option<Uuid>,    // Optional category filter
    favorites_only: bool          // Filter for favorite entries only
}

impl SearchQuery {
    // Constructor with optional parameters
    pub fn new(text: Option<String>, category_id: Option<Uuid>, favorites_only: bool) -> Self {}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryCollection {
    
    entries: HashMap<Uuid, Entry>,                    // Primary entry storage
    categories: CategoryCollection,                   // Category management
    favorites: FavoriteCollection,                   // Favorites management
    // Search indexes for O(1) lookups
    title_index: HashMap<String, HashSet<Uuid>>,     // Title -> Entry IDs
    username_index: HashMap<String, HashSet<Uuid>>,  // Username -> Entry IDs
    url_index: HashMap<String, HashSet<Uuid>>,        // URL -> Entry IDs
    entry_count: usize,                               // Total number of entries
}

impl EntryCollection {
    // Initialize new empty collection with all required components
    pub fn new() -> Self {}

    pub fn create_entry(
        &mut self,
        entry_data: EntryData,
        key_hierarchy: &KeyHierarchy,
    ) -> Result<Uuid, EntryError> {}

    // Updates entry/entry collection as well as our search indexes (favorite, category, etc.)
    pub fn update_entry(
        &mut self,
        id: &Uuid,
        changes: EntryData,
        key_hierarchy: &KeyHierarchy,
    ) -> Result<(), EntryError> {}

    pub fn delete_entry(&mut self, id: &Uuid) -> Result<(), EntryError> {}

    // Retrieves and decrypts full entry data
    // Returns decrypted entry with all sensitive fields accessible
    pub fn get_decrypted_entry(
        &self,
        id: &Uuid,
        key_hierarchy: &KeyHierarchy,
    ) -> Result<DecryptedEntry, EntryError> {}

    // Gets entry overview (non-sensitive data only)
    pub fn get_entry_overview(&self, id: &Uuid) -> Result<EntryOverview, EntryError> {}

    // Returns overviews of all entries in collection
    pub fn get_all_overviews(&self) -> Vec<EntryOverview> {}

    // Handles searching and filtering entries based on the provided query
    pub fn search(&self, query: SearchQuery) -> Vec<EntryOverview> {}

}

```

## src/auth/auth_service.rs

```rust
struct AuthService {
    // Quick validation hash of master password
    master_password_hash: Vec<u8>,
    // Cryptographic salt for key derivation
    salt: Vec<u8>,
    // Encrypted Master Encryption Key (MEK) for _vault access
    encrypted_mek: EncryptedData,
    // Current lock state of the _vault
    locked: bool,
    // Configuration for password-based key derivation
    key_derivation: KeyDerivation,
}

impl AuthService {
    pub fn new(master_password: SecureMemory<String>) -> Result<(Self, KeyHierarchy), AuthError> {

    }


    pub fn unlock(&mut self, master_password: SecureMemory<String>) -> Result<KeyHierarchy, AuthError> {

    }

    pub fn change_password(
        &mut self,
        old_password: SecureMemory<String>,
        new_password: SecureMemory<String>,
    ) -> Result<(), AuthError> {

    }

    pub fn update_key_derivation(
        &mut self,
        master_password: SecureMemory<String>,
        new_key_derivation: KeyDerivation
    ) -> Result<(), AuthError> {

    }


    pub fn verify_master_password(&self, password: SecureMemory<String>) -> Result<bool, AuthError> {

    }

    // Getter methods for _vault status and stored data
    pub fn is_locked(&self) -> bool {}
    pub fn key_derivation(&self) -> &KeyDerivation {}
    pub fn encrypted_mek(&self) -> &EncryptedData {}
    pub fn master_password_hash(&self) -> &[u8] {}
    pub fn salt(&self) -> &[u8] {}
}

```

## src/vault/password_generation.rs

```rust
// PasswordGenerator: Secure and configurable password generation system
// Provides configuration and generation capabilities with strong security guarantees
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordGenerator {
    // Core Configuration
    min_required: usize,      // Minimum password length (min: 5)
    length: usize,            // Current password length (default: 16)
    
    // Character Set Controls
    use_lowercase: bool,      // Include a-z
    use_uppercase: bool,      // Include A-Z
    use_numbers: bool,        // Include 0-9
    use_symbols: bool,        // Include special chars
    
    // Minimum Requirements
    min_numbers: usize,       // Minimum required numbers
    min_symbols: usize,       // Minimum required symbols
    
    // Readability Controls
    exclude_ambiguous: bool,  // Exclude similar chars (1/l/I, 0/O),
    // Pre-computed Character Sets (computed once at initialization), not modifiable by user directly
    available_chars: CharacterSets,
}

// Settings struct for configuring the generator
pub struct PasswordGeneratorSettings {
    pub length: usize,
    pub use_lowercase: bool,
    pub use_uppercase: bool,
    pub use_numbers: bool,
    pub use_symbols: bool,
    pub min_numbers: usize,
    pub min_symbols: usize,
    pub exclude_ambiguous: bool,
}

impl PasswordGenerator {
    // Configures generator with provided settings
    // Validates settings before applying changes
    // Returns Result<(), PasswordGenerationError>
    pub fn configure(&mut self, settings: PasswordGeneratorSettings) {}

    // Generates password meeting all configured requirements
    // Uses ChaCha20Rng for secure random generation
    // Returns Result<String, PasswordGenerationError>
    pub fn generate(&self) {}

    // Calculates password entropy based on charset size and length
    // Returns f64 representing bits of entropy
    pub fn calculate_entropy(&self) {}
}

// Default implementation with secure defaults:
// - 16 character length
// - All character sets enabled
// - Min 2 numbers, 2 symbols
// - Ambiguous characters excluded
impl Default for PasswordGenerator {
    fn default() -> Self {}
}

```

## src/vault/vault_manager.rs

```rust
// VaultManager: Core struct managing vault operations, authentication, and entry storage
#[derive(Serialize, Deserialize)]
pub struct VaultManager {
    pub auth_service: AuthService,           // Handles authentication and encryption
    pub entry_collection: EntryCollection,   // Stores vault entries
    pub key_derivation_settings: KeyDerivation,  // Settings for key derivation
    pub password_generator_settings: PasswordGenerator,  // Password generation config
    pub logged_in: bool,                     // Current login state
    pub initialized: bool,                   // Vault initialization state
    pub last_backup_time: DateTime<Utc>,     // Timestamp of last backup
    pub number_of_backups: usize,           // Count of created backups
}

impl VaultManager {
    pub fn new(master_password: SecureMemory<String>) -> Result<Self, VaultError> {
        // Initializes vault with default settings and provided master password
    }

    pub fn login(&mut self, master_password: SecureMemory<String>) -> Result<KeyHierarchy, VaultError> {
        // Verifies password and updates login state
    }

    pub fn logout(&mut self) -> Result<(), VaultError> {
        // Updates login state to false
    }


    pub fn update_key_derivation(&mut self, master_password: String, settings: KeyDerivation) -> Result<(), VaultError> {
        // Updates auth service with new settings
    }

    pub fn logged_in(&self) -> bool {
        // Returns login state
    }
}

```


## src/vault/vault_storage.rs

```rust
// VaultStorage: Manages persistent storage and file operations for the password vault
// Handles atomic file operations, backups, and concurrent access control
struct VaultStorage {
    // Core file system paths
    vault_path: PathBuf,    // Main vault file location
    backup_dir: PathBuf,    // Backup storage location
    temp_dir: PathBuf,      // Temporary file storage
    file_lock: Arc<RwLock<()>>  // Concurrent access control
}

impl VaultStorage {
    // Creates new VaultStorage instance with default paths in app data directory
    fn new() -> Self {
        // Initialize paths and locks
    }

    async fn load_vault(&self) -> Result<VaultManager, AppError> {
        // Read and deserialize vault data with error handling
    }

    pub async fn save_vault(&self, vault_manager: VaultManager) -> Result<(), AppError> {
        // 1. Write to temporary file
        // 2. Verify written data
        // 3. Atomic rename to final location
        // 4. Cleanup temporary files
    }


    pub async fn create_backup(&mut self, vault_manager: VaultManager) -> Result<(), AppError> {
        // Create backup with timestamp
        // Atomic write operations
    }

    pub async fn restore_from_file(&mut self, backup_path: PathBuf) -> Result<VaultManager, AppError> {
        // Verify and load backup
        // Atomic restore operation
    }

    // Logs out and saves vault state
    pub async fn logout(&mut self, mut vault_manager: VaultManager) -> Result<(), AppError> {
        // Lock vault and save state
    }
}

// Helper methods implementation
impl VaultStorage {
    // Ensures required directories exist
    pub async fn ensure_directories(&self) -> Result<(), AppError> {
        // Create necessary directories with proper error handling
    }


    // Getter methods for paths and locks
    pub fn get_vault_path(&self) -> &PathBuf {}
    pub fn get_backup_dir(&self) -> &PathBuf {}
    pub fn get_temp_dir(&self) -> &PathBuf {}
    pub fn get_file_lock(&self) -> Arc<RwLock<()>> {}
}


```