pub(crate) mod secure;
pub(crate) mod key_derivation;
pub(crate) mod key_hierarchy;
pub(crate) mod encrypted_data;

pub use secure::SecureMemory;
pub use key_derivation::KeyDerivation;
pub use key_hierarchy::KeyHierarchy;
pub use encrypted_data::EncryptedData;

// Key Derivation Constants
pub const RECOMMENDED_MEMORY_COST: u32 = 46_080;  // 45 MiB in KiB
pub const RECOMMENDED_TIME_COST: u32 = 1;

pub const KEY_LENGTH: usize = 32;  // 256 bits
pub const SALT_LENGTH: usize = 16; // 128 bits
pub const MIN_KEY_LENGTH: usize = 16;
pub const MAX_KEY_LENGTH: usize = 64;  // 512 bits max
pub const MAX_MEMORY_COST: u32 = 8_388_608; // 8 GiB in KiB
pub const MAX_TIME_COST: u32 = 50;
pub const MAX_PARALLELISM: u32 = 16;
const KEY_SIZE: usize = 32; // 256 bits


