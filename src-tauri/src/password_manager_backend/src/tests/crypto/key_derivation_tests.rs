// #[cfg(test)]
// mod tests {
//     use crate::crypto::KeyDerivation;
//     use crate::crypto::SecureMemory;
//     use crate::error::crypto_error::CryptoError;
//     use crate::crypto::{
//         RECOMMENDED_MEMORY_COST,
//         RECOMMENDED_TIME_COST,
//         MAX_PARALLELISM,
//         MAX_MEMORY_COST,
//         MAX_KEY_LENGTH,
//         MIN_KEY_LENGTH,
//         KEY_LENGTH,
//         MAX_TIME_COST,
//         SALT_LENGTH,
//         get_recommended_parallelism,
//     };
//
//
//
//     /// Helper function to compare two SecureMemory<Vec<u8>> contents
//     fn secure_memory_eq(a: &SecureMemory<Vec<u8>>, b: &SecureMemory<Vec<u8>>) -> bool {
//         a.as_ref() == b.as_ref()
//     }
//
//     // 1. Parameter Validation Tests
//     #[test]
//     fn test_valid_parameters() {
//         let result = KeyDerivation::new(16384, 2, 1, 32);
//         assert!(result.is_ok());
//     }
//
//     #[test]
//     fn test_default_parameters() {
//         let kdf = KeyDerivation::default();
//         assert_eq!(kdf.memory_cost(),RECOMMENDED_MEMORY_COST);
//         assert_eq!(kdf.time_cost(), RECOMMENDED_TIME_COST);
//         assert_eq!(kdf.parallelism(), get_recommended_parallelism());
//         assert_eq!(kdf.key_length(), KEY_LENGTH);
//     }
//
//     #[test]
//     fn test_invalid_memory_cost() {
//         let result = KeyDerivation::new(8, 1, 2, 32);
//         assert!(result.is_err());
//     }
//
//     #[test]
//     fn test_invalid_parallelism() {
//         let result = KeyDerivation::new(16384, 1, 0, 32);
//         assert!(result.is_err());
//     }
//
//     #[test]
//     fn test_invalid_time_cost() {
//         let result = KeyDerivation::new(16384, 0, 1, 32);
//         assert!(result.is_err());
//     }
//
//     #[test]
//     fn test_invalid_key_length() {
//         let result = KeyDerivation::new(16384, 1, 1, MIN_KEY_LENGTH - 1);
//         assert!(result.is_err());
//     }
//
//     // 2. Salt Generation Tests
//     #[test]
//     fn test_salt_generation() {
//         let kdf = KeyDerivation::default();
//         let salt = kdf.generate_salt().unwrap();
//         assert_eq!(salt.len(), SALT_LENGTH);
//     }
//
//     #[test]
//     fn test_salt_uniqueness() {
//         let kdf = KeyDerivation::default();
//         let salt1 = kdf.generate_salt().unwrap();
//         let salt2 = kdf.generate_salt().unwrap();
//         assert_ne!(salt1, salt2);
//     }
//
//     // 3. Salt Validation Tests
//     #[test]
//     fn test_valid_salt_validation() {
//         let kdf = KeyDerivation::default();
//         let salt = kdf.generate_salt().unwrap();
//         assert!(kdf.validate_salt(&salt).is_ok());
//     }
//
//     #[test]
//     fn test_invalid_salt_length() {
//         let kdf = KeyDerivation::default();
//         let invalid_salt = vec![0u8; SALT_LENGTH - 1];
//         assert!(kdf.validate_salt(&invalid_salt).is_err());
//     }
//
//     // 4. Password Validation Tests
//     #[test]
//     fn test_empty_password() {
//         let kdf = KeyDerivation::new(16384, 1, 1, 32).unwrap();
//         let salt = kdf.generate_salt().unwrap();
//         let result = kdf.derive_key(&[], &salt);
//         assert!(result.is_err());
//     }
//
//     #[test]
//     fn test_max_password_length() {
//         let kdf = KeyDerivation::new(16384, 1, 1, 32).unwrap();
//         let salt = kdf.generate_salt().unwrap();
//         let long_password = vec![b'a'; argon2::MAX_PWD_LEN + 1];
//         let result = kdf.derive_key(&long_password, &salt);
//         assert!(result.is_err());
//     }
//
//     // 5. Context Validation Tests
//     #[test]
//     fn test_max_context_length() {
//         let kdf = KeyDerivation::new(16384, 1, 1, 32).unwrap();
//         let salt = kdf.generate_salt().unwrap();
//         let password = b"test_password";
//         let long_context = vec![b'a'; argon2::MAX_SECRET_LEN + 1];
//         let result = kdf.derive_key_with_context(password, &salt, &long_context);
//         assert!(result.is_err());
//     }
//
//     // 6. Builder Pattern Tests
//     #[test]
//     fn test_builder_all_parameters() {
//         let kdf = KeyDerivation::builder()
//             .memory_cost(16384)
//             .time_cost(2)
//             .parallelism(1)
//             .key_length(32)
//             .build()
//             .unwrap();
//
//         assert_eq!(kdf.memory_cost(), 16384);
//         assert_eq!(kdf.time_cost(), 2);
//         assert_eq!(kdf.parallelism(), 1);
//         assert_eq!(kdf.key_length(), 32);
//     }
//
//     #[test]
//     fn test_builder_defaults() {
//         let kdf = KeyDerivation::builder()
//             .build()
//             .unwrap();
//
//         assert_eq!(kdf.memory_cost(), RECOMMENDED_MEMORY_COST);
//         assert_eq!(kdf.time_cost(), RECOMMENDED_TIME_COST);
//         assert_eq!(kdf.parallelism(), get_recommended_parallelism());
//         assert_eq!(kdf.key_length(), KEY_LENGTH);
//     }
//
//     #[test]
//     fn test_builder_partial_parameters() {
//         let kdf = KeyDerivation::builder()
//             .memory_cost(16384)
//             .parallelism(2)
//             .build()
//             .unwrap();
//
//         assert_eq!(kdf.memory_cost(), 16384);
//         assert_eq!(kdf.time_cost(), RECOMMENDED_TIME_COST);
//         assert_eq!(kdf.parallelism(), 2);
//         assert_eq!(kdf.key_length(), KEY_LENGTH);
//     }
//
//     #[test]
//     fn test_builder_invalid_parameters() {
//         let result = KeyDerivation::builder()
//             .memory_cost(8)
//             .parallelism(2)
//             .build();
//         assert!(result.is_err());
//     }
//
//     // 7. Memory Security Tests
//     #[test]
//     fn test_key_zeroing() {
//         let kdf = KeyDerivation::new(16384, 1, 1, 32).unwrap();
//         let password = b"test_password";
//         let salt = kdf.generate_salt().unwrap();
//
//         let key = kdf.derive_key(password, &salt).unwrap();
//         let key_ptr = key.as_ref().as_ptr();
//         let key_len = key.as_ref().len();
//
//         // Create a scope so key is dropped
//         {
//             let _temp = key;
//         }
//
//         // Verify memory is zeroed (this is a bit hacky but demonstrates the concept)
//         let sum: u8 = unsafe {
//             std::slice::from_raw_parts(key_ptr, key_len)
//                 .iter()
//                 .sum()
//         };
//         assert_eq!(sum, 0);
//     }
//
//     #[test]
//     fn test_debug_output() {
//         let kdf = KeyDerivation::new(16384, 1, 1, 32).unwrap();
//         let password = b"test_password";
//         let salt = kdf.generate_salt().unwrap();
//
//         let key = kdf.derive_key(password, &salt).unwrap();
//         let debug_output = format!("{:?}", key);
//
//         // Debug output shouldn't contain actual key bytes
//         assert!(!debug_output.contains("test_password"));
//         assert!(debug_output.contains("SecureMemory"));
//     }
//
//     // 8. Edge Cases
//     #[test]
//     fn test_unicode_password() {
//         let kdf = KeyDerivation::new(16384, 1, 1, 32).unwrap();
//         let password = "🔑password🔒".as_bytes();
//         let salt = kdf.generate_salt().unwrap();
//
//         assert!(kdf.derive_key(password, &salt).is_ok());
//     }
//
//     #[test]
//     fn test_all_ascii_password() {
//         let kdf = KeyDerivation::new(16384, 1, 1, 32).unwrap();
//         let password: Vec<u8> = (0..127).collect();
//         let salt = kdf.generate_salt().unwrap();
//
//         assert!(kdf.derive_key(&password, &salt).is_ok());
//     }
//
//     #[test]
//     fn test_special_chars_password() {
//         let kdf = KeyDerivation::new(16384, 1, 1, 32).unwrap();
//         let password = b"!@#$%^&*()_+-=[]{}|;:,.<>?";
//         let salt = kdf.generate_salt().unwrap();
//
//         assert!(kdf.derive_key(password, &salt).is_ok());
//     }
//
//     #[test]
//     fn test_max_parameters() {
//         // Test with values just above our maximums
//         let result = KeyDerivation::new(
//             MAX_MEMORY_COST + 1,
//             MAX_TIME_COST + 1,
//             MAX_PARALLELISM + 1,
//             MAX_KEY_LENGTH + 1
//         );
//         assert!(result.is_err());
//     }
//
//     // 9. Error Handling Tests
//     #[test]
//     fn test_error_variants() {
//         let kdf = KeyDerivation::new(16384, 1, 1, 32).unwrap();
//
//         // Test invalid salt length
//         let err = kdf.validate_salt(&[0u8; 8]).unwrap_err();
//         assert!(matches!(err, CryptoError::InvalidSaltLength));
//
//         // Test invalid password
//         let salt = kdf.generate_salt().unwrap();
//         let err = kdf.derive_key(&[], &salt).unwrap_err();
//         assert!(matches!(err, CryptoError::EmptyPassword));
//
//         // Test invalid context
//         let long_context = vec![0u8; argon2::MAX_SECRET_LEN + 1];
//         let err = kdf.derive_key_with_context(b"password", &salt, &long_context).unwrap_err();
//         assert!(matches!(err, CryptoError::InvalidContextLength));
//     }
//
//     // 10. Basic Performance Tests
//     #[test]
//     fn test_derivation_time() {
//         use std::time::Instant;
//
//         let kdf = KeyDerivation::new(16384, 1, 1, 32).unwrap();
//         let password = b"test_password";
//         let salt = kdf.generate_salt().unwrap();
//
//         let start = Instant::now();
//         let _ = kdf.derive_key(password, &salt).unwrap();
//         let duration = start.elapsed();
//
//         // Even with minimal parameters, should take some time
//         assert!(duration.as_micros() > 0);
//     }
//
//     #[test]
//     fn test_memory_usage() {
//         let kdf = KeyDerivation::new(16384, 1, 1, 32).unwrap();
//         let password = b"test_password";
//         let salt = kdf.generate_salt().unwrap();
//
//         let key = kdf.derive_key(password, &salt).unwrap();
//
//         // Verify the length of the contained data instead of the Vec's size
//         assert_eq!(key.as_ref().len(), 32);
//     }
//
//     // Additional Validation Tests
//     #[test]
//     fn test_output_key_length() {
//         let kdf = KeyDerivation::new(16384, 1, 1, 32).unwrap();
//         let password = b"test_password";
//         let salt = kdf.generate_salt().unwrap();
//
//         let key = kdf.derive_key(password, &salt).unwrap();
//         assert_eq!(key.as_ref().len(), kdf.key_length());
//     }
//
//     #[test]
//     fn test_consistent_context_derivation() {
//         let kdf = KeyDerivation::new(16384, 1, 1, 32).unwrap();
//         let password = b"test_password";
//         let salt = kdf.generate_salt().unwrap();
//         let context = b"test-context";
//
//         let key1 = kdf.derive_key_with_context(password, &salt, context).unwrap();
//         let key2 = kdf.derive_key_with_context(password, &salt, context).unwrap();
//
//         assert!(secure_memory_eq(&key1, &key2));
//     }
//
//     #[test]
//     fn test_builder_with_invalid_parameters() {
//         // Test all invalid parameter combinations
//         assert!(KeyDerivation::builder()
//             .memory_cost(0)
//             .build()
//             .is_err());
//
//         assert!(KeyDerivation::builder()
//             .parallelism(0)
//             .build()
//             .is_err());
//
//         assert!(KeyDerivation::builder()
//             .key_length(MIN_KEY_LENGTH - 1)
//             .build()
//             .is_err());
//
//         assert!(KeyDerivation::builder()
//             .time_cost(0)
//             .build()
//             .is_err());
//     }
// }