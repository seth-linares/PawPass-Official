use crate::crypto::secure::SecureMemory;
use crate::error::crypto_error::CryptoError;
use argon2::{
    Argon2,
    Params,
    Version,
    Algorithm,
    password_hash::{rand_core::OsRng, rand_core::RngCore},
};
use serde::{Deserialize, Serialize};
use crate::crypto::{
    RECOMMENDED_MEMORY_COST,
    RECOMMENDED_TIME_COST,
    MAX_PARALLELISM,
    MAX_MEMORY_COST,
    KEY_LENGTH,
    MAX_TIME_COST,
    SALT_LENGTH,
};

// Function to get number of cores
pub(crate) fn get_recommended_parallelism() -> u32 {
    let cores = num_cpus::get_physical() as u32;
    cores.clamp(1, 4)
}

/// KeyDerivation handles secure key derivation using Argon2id
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct KeyDerivation {
    #[serde(rename = "memoryCost")]
    memory_cost: u32,     // Memory size in KiB
    #[serde(rename = "timeCost")]
    time_cost: u32,       // Number of iterations
    parallelism: u32,     // Degree of parallelism
}

impl Default for KeyDerivation {
    fn default() -> Self {
        Self {
            memory_cost: RECOMMENDED_MEMORY_COST,
            time_cost: RECOMMENDED_TIME_COST,
            parallelism: get_recommended_parallelism(),
        }
    }
}

impl KeyDerivation {
    pub fn new(
        memory_cost: u32,
        time_cost: u32,
        parallelism: u32,
    ) -> Result<Self, CryptoError> {
        Self::validate_params(memory_cost, time_cost, parallelism)?;
        Ok(Self {
            memory_cost,
            time_cost,
            parallelism,
        })
    }

    fn validate_params(
        memory_cost: u32,
        time_cost: u32,
        parallelism: u32,
    ) -> Result<(), CryptoError> {
        println!("🔍 Validating key derivation parameters:");
        println!("   Memory Cost: {} KiB", memory_cost);
        println!("   Time Cost: {} iterations", time_cost);
        println!("   Parallelism: {} threads", parallelism);

        // Memory cost validation
        if memory_cost < 8 * parallelism {
            println!("❌ Invalid memory cost: must be at least 8 * parallelism");
            return Err(CryptoError::InvalidMemoryCost);
        }
        if memory_cost > MAX_MEMORY_COST {
            println!("❌ Invalid memory cost: exceeds maximum");
            return Err(CryptoError::InvalidMemoryCost);
        }

        // Time cost validation
        if time_cost < 1 || time_cost > MAX_TIME_COST {
            println!("❌ Invalid time cost");
            return Err(CryptoError::InvalidTimeCost);
        }

        // Parallelism validation
        if parallelism < 1 || parallelism > MAX_PARALLELISM {
            println!("❌ Invalid parallelism");
            return Err(CryptoError::InvalidParallelism);
        }

        println!("✅ Parameter validation successful");
        Ok(())
    }

    /// Validates the current KeyDerivation parameters
    pub fn validate(&self) -> Result<&Self, CryptoError> {
        Self::validate_params(
            self.memory_cost,
            self.time_cost,
            self.parallelism,
        )?;
        Ok(self)
    }

    /// Generates a cryptographically secure salt
    pub fn generate_salt(&self) -> Result<Vec<u8>, CryptoError> {
        let mut salt = vec![0u8; SALT_LENGTH];
        OsRng.fill_bytes(&mut salt);
        Ok(salt)
    }

    /// Validates if provided salt meets requirements
    pub fn validate_salt(&self, salt: &[u8]) -> Result<(), CryptoError> {
        if salt.len() != SALT_LENGTH {
            return Err(CryptoError::InvalidSaltLength);
        }
        Ok(())
    }


    /// Validates context length requirements
    fn validate_context(&self, context: &[u8]) -> Result<(), CryptoError> {
        if context.len() > argon2::MAX_SECRET_LEN {
            return Err(CryptoError::InvalidContextLength);
        }
        Ok(())
    }

    /// Derives a key from a password and salt using Argon2id
    pub fn derive_key(
        &self,
        password: &[u8],
        salt: &[u8],
    ) -> Result<SecureMemory<Vec<u8>>, CryptoError> {
        if password.is_empty() {
            return Err(CryptoError::InvalidPasswordLength);
        }
        self.validate_salt(salt)?;

        // Create Argon2id context with our parameters
        let params = Params::new(
            self.memory_cost,
            self.time_cost,
            self.parallelism,
            Some(KEY_LENGTH),
        )
            .map_err(|_| CryptoError::KeyDerivationFailed)?;

        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            params,
        );

        // Allocate output buffer
        let mut output_key = vec![0u8; KEY_LENGTH];

        // Perform key derivation
        argon2
            .hash_password_into(password, salt, &mut output_key)
            .map_err(|_| CryptoError::KeyDerivationFailed)?;

        // Wrap the derived key in SecureMemory
        Ok(SecureMemory::new(output_key))
    }

    /// Derives a key with additional associated data (for key hierarchy)
    pub fn derive_key_with_context(
        &self,
        password: &[u8],
        salt: &[u8],
        context: &[u8],
    ) -> Result<SecureMemory<Vec<u8>>, CryptoError> {
        self.validate_salt(salt)?;
        self.validate_context(context)?;

        let params = Params::new(
            self.memory_cost,
            self.time_cost,
            self.parallelism,
            Some(KEY_LENGTH), // Always use constant KEY_LENGTH
        )?;

        let argon2 = Argon2::new_with_secret(
            context,
            Algorithm::Argon2id,
            Version::V0x13,
            params,
        ).map_err(|_| CryptoError::KeyDerivationFailed)?;

        let mut output_key = vec![0u8; KEY_LENGTH];

        argon2
            .hash_password_into(password, salt, &mut output_key)
            .map_err(|_| CryptoError::KeyDerivationFailed)?;

        Ok(SecureMemory::new(output_key))
    }

    // Getters for parameters
    pub fn memory_cost(&self) -> u32 { self.memory_cost }
    pub fn time_cost(&self) -> u32 { self.time_cost }
    pub fn parallelism(&self) -> u32 { self.parallelism }
}
