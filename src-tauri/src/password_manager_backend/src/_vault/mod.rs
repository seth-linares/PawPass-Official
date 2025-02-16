pub mod password_generation;
pub mod vault_manager;
pub mod vault_storage;

// Password Generation Constants
pub const ABSOLUTE_MIN_LENGTH: usize = 5;
pub const ABSOLUTE_MAX_LENGTH: usize = 256;
pub const DEFAULT_LENGTH: usize = 16;
pub const DEFAULT_MIN_NUMBERS: usize = 2;
pub const DEFAULT_MIN_SYMBOLS: usize = 2;



