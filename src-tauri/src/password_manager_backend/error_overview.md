```rust
// === Auth/Login Related Errors ===
#[derive(Debug, Serialize)]
pub enum AuthError {
// Cryptographic operation failures
CryptoError(String),      // Wraps crypto operation failures

    // Validation errors
    InvalidPassword,          // Wrong or malformed password
    PasswordMismatch,         // New password confirmation failed
    VaultLocked,             // Operation needs unlocked vault
    VaultUnlocked,           // Operation needs locked vault
    
    // Key derivation errors
    KeyDerivationError(String), // Key generation/derivation failed
    
    // Generic errors
    OperationError(String),     // Catchall for other auth operations
}

// === Category Management Errors ===
#[derive(Debug, Serialize)]
pub enum CategoryError {
// Validation errors
InvalidName(String),      // Empty or invalid category name
DuplicateName(String),    // Category name already exists

    // Operation errors
    NotFound(Uuid),          // Category ID doesn't exist
    
    // Generic validation
    ValidationError(String),  // Other validation failures
}

// === Cryptographic Operation Errors ===
#[derive(Debug, Serialize)]
pub enum CryptoError {
// Key Derivation Errors
KeyDerivationFailed,     // General KDF failure
InvalidKeyLength,        // Key size mismatch
InvalidParameters,       // Bad crypto parameters

    // Salt Errors
    SaltGenerationFailed,    // Couldn't generate salt
    InvalidSaltLength,       // Salt size issues
    
    // Memory/Parameter Errors
    InsufficientMemory,      // Not enough memory for operation
    InvalidMemoryCost,       // Bad memory cost parameter
    InvalidTimeCost,         // Bad time cost parameter
    InvalidParallelism,      // Bad parallelism parameter
    InvalidContextLength,    // Context too long/short
    InvalidPasswordLength,   // Password too long
    EmptyPassword,          // Password is empty
    
    // EncryptedData Errors
    InvalidCiphertext,       // Bad/corrupted ciphertext
    InvalidNonceLength,      // Nonce size issues
    InvalidTagLength,        // Auth tag size issues
    
    // Key Hierarchy Errors
    MasterKeyDerivationFailed,  // Couldn't derive master key
    MekGenerationFailed,        // Couldn't generate MEK
    MekEncryptionFailed,        // Couldn't encrypt MEK
    MekDecryptionFailed,        // Couldn't decrypt MEK
    InvalidMasterPassword,      // Wrong master password
    EmptyData,                  // Nothing to encrypt
    
    // General Crypto Errors
    EncryptionFailed,          // General encryption failure
    DecryptionFailed,          // General decryption failure
    InvalidKey,                // Bad/corrupted key
    
    // External Errors
    IoError(String),           // File I/O errors
    Argon2Error(String),       // Argon2 specific errors
}

// === Entry Management Errors ===
#[derive(Debug, Serialize)]
pub enum EntryError {
// Crypto errors
EncryptionError(CryptoError),  // Entry encryption failed
DecryptionError(CryptoError),  // Entry decryption failed

    // Category related
    CategoryError(CategoryError),   // Category operation failed
    
    // Validation errors
    InvalidPassword,               // Entry password invalid
    InvalidTitle,                 // Entry title empty/invalid
    InvalidUrl,                   // Entry URL malformed
    InvalidLength(String),        // Field too long
    
    // Operation errors
    NotFound(Uuid),              // Entry ID doesn't exist
    DuplicateEntry,              // Entry title exists
    
    // Generic errors
    ValidationError(String),      // Other validation failures
}

// === Password Generation Errors ===
#[derive(Debug, Serialize)]
pub enum PasswordGenerationError {
// Configuration Errors
InvalidLength {
min: usize,
max: usize,
message: String,
},
InvalidMinimumRequirement {
requirement_type: String,
requested: usize,
available_length: usize,
},
ConflictingRequirements(String),

    // Character Set Errors
    EmptyCharacterSet,
    InsufficientCharacterPool {
        required: usize,
        available: usize,
    },
    
    // Generation Errors
    RandomGenerationFailed(String),
    ValidationFailed(String),
    
    // Logical Errors
    ImpossibleConfiguration(String),
    
    // System Errors
    SystemError(String),
}

// === Storage Operation Errors ===
#[derive(Debug, Serialize)]
pub enum AppError {
// File system errors
IoError(String),
PathNotFound(String),
InsufficientPermissions(String),
DirectoryCreationFailed(String),
DirectoryReadFailed(String),
DataCorruption(String),

    // Password errors
    PasswordsDoNotMatch,
    PasswordTooShort,
    PasswordTooLong,
    
    // Auth related
    AuthError(String),
    
    // Data integrity
    CorruptedFile {
        path: String,
        details: String,
    },
    VerificationFailed(String),
    
    // Serialization
    SerializationFailed(String),
    DeserializationFailed(String),
    
    // Backup related
    BackupCreationFailed {
        path: String,
        reason: String,
    },
    BackupRestoreFailed {
        path: String,
        reason: String,
    },
    BackupLimitReached(usize),
    BackupDeletionFailed,
    
    // Atomic operation errors
    AtomicSaveFailed {
        temp_path: String,
        reason: String,
    },
    TemporaryFileError(String),
    
    // Vault state errors
    VaultLocked,
    VaultNotInitialized,
    InvalidVaultState(String),
    VaultAlreadyExists,
}

// === General Vault Errors ===
#[derive(Debug, Serialize)]
pub enum VaultError {
// State errors
VaultLocked,
VaultAlreadyUnlocked,
VaultNotInitialized,
VaultAlreadyExists,
VaultNotFound,

    // Component errors
    AuthErrorMessage(String),
    EntryErrorMessage(String),
    CryptoErrorMessage(String),
    PasswordGenerationErrorMessage(String),
    
    // Storage errors
    AppError(String),
    InvalidVaultPath,
    IoError(String),
    
    // Settings errors
    InvalidKeyDerivationSettings,
    InvalidPasswordGeneratorSettings,
    
    // Other errors
    AutoLockError(String),
    BackupError(String),
    ValidationError(String),
    InvalidPassword,
}

/*
Error Handling Strategy:
1. All errors are Serializable for Tauri compatibility
2. Specific errors convert to more general ones (e.g., CryptoError -> AuthError)
3. Each error type includes detailed context where appropriate
4. Error messages are user-friendly but informative
5. Structured errors allow for specific handling on the frontend

Frontend TypeScript example:
type AuthError =
| { type: 'InvalidPassword' }
| { type: 'PasswordMismatch' }
| { type: 'VaultLocked' }
| { type: 'CryptoError'; message: string }
// etc.
*/
```