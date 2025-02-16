// #[cfg(test)]
// mod crypto_integration_tests {
//     use crate::crypto::{
//         KeyDerivation,
//         KeyHierarchy,
//     };
//     use crate::error::crypto_error::CryptoError;
//
//     const TEST_PASSWORD: &[u8] = b"test_password123";
//     const TEST_DATA: &[u8] = b"sensitive_data_to_protect";
//
//     // Helper to create a test setup with all components
//     fn create_test_setup() -> Result<(KeyHierarchy, Vec<u8>, KeyDerivation), CryptoError> {
//         let key_derivation = KeyDerivation::default();
//         let salt = key_derivation.generate_salt()?;
//         let (hierarchy, generated_salt) = KeyHierarchy::new(TEST_PASSWORD)?;
//
//         assert_eq!(salt.len(), generated_salt.len());
//         Ok((hierarchy, generated_salt, key_derivation))
//     }
//
//     #[test]
//     fn test_full_encryption_flow() -> Result<(), CryptoError> {
//         // 1. Set up key derivation
//         let key_derivation = KeyDerivation::default();
//         let salt = key_derivation.generate_salt()?;
//
//         // 2. Derive an initial key using SecureMemory
//         let derived_key = key_derivation.derive_key(TEST_PASSWORD, &salt)?;
//         assert!(derived_key.as_ref().len() > 0);
//
//         // 3. Create key hierarchy
//         let (hierarchy, hierarchy_salt) = KeyHierarchy::new(TEST_PASSWORD)?;
//
//         // 4. Encrypt data
//         let encrypted = hierarchy.encrypt_data(TEST_DATA)?;
//
//         // 5. Create new hierarchy instance with same password
//         let encrypted_mek = hierarchy.encrypted_mek()?;
//         let new_hierarchy = KeyHierarchy::from_existing(
//             TEST_PASSWORD,
//             &encrypted_mek,
//             &hierarchy_salt
//         )?;
//
//         // 6. Decrypt with new hierarchy
//         let decrypted = new_hierarchy.decrypt_data(&encrypted)?;
//         assert_eq!(decrypted, TEST_DATA);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_key_derivation_consistency() -> Result<(), CryptoError> {
//         let (_, salt, key_derivation) = create_test_setup()?;
//
//         // Derive keys multiple times with same parameters
//         let key1 = key_derivation.derive_key(TEST_PASSWORD, &salt)?;
//         let key2 = key_derivation.derive_key(TEST_PASSWORD, &salt)?;
//
//         // Keys should be identical when derived with same parameters
//         assert_eq!(key1.as_ref(), key2.as_ref());
//
//         // Create another KeyDerivation instance with same parameters
//         let key_derivation2 = KeyDerivation::default();
//         let key3 = key_derivation2.derive_key(TEST_PASSWORD, &salt)?;
//
//         // Key should be same even with new instance
//         assert_eq!(key1.as_ref(), key3.as_ref());
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_secure_memory_in_key_hierarchy() -> Result<(), CryptoError> {
//         let (hierarchy, salt, _) = create_test_setup()?;
//
//         // Get MEK in encrypted form
//         let encrypted_mek = hierarchy.encrypted_mek()?;
//
//         // Create multiple hierarchies and verify they can all decrypt the same data
//         let encrypted_data = hierarchy.encrypt_data(TEST_DATA)?;
//
//         let hierarchy2 = KeyHierarchy::from_existing(TEST_PASSWORD, &encrypted_mek, &salt)?;
//         let hierarchy3 = KeyHierarchy::from_existing(TEST_PASSWORD, &encrypted_mek, &salt)?;
//
//         // All should be able to decrypt the data
//         assert_eq!(hierarchy2.decrypt_data(&encrypted_data)?, TEST_DATA);
//         assert_eq!(hierarchy3.decrypt_data(&encrypted_data)?, TEST_DATA);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_encrypted_data_compatibility() -> Result<(), CryptoError> {
//         let (hierarchy, _, _) = create_test_setup()?;
//
//         // Create multiple encrypted versions of the same data
//         let encrypted1 = hierarchy.encrypt_data(TEST_DATA)?;
//         let encrypted2 = hierarchy.encrypt_data(TEST_DATA)?;
//
//         // Verify they're different (due to different nonces)
//         assert_ne!(encrypted1.ciphertext(), encrypted2.ciphertext());
//         assert_ne!(encrypted1.nonce(), encrypted2.nonce());
//
//         // But both decrypt to the same data
//         assert_eq!(hierarchy.decrypt_data(&encrypted1)?, TEST_DATA);
//         assert_eq!(hierarchy.decrypt_data(&encrypted2)?, TEST_DATA);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_memory_cleanup() -> Result<(), CryptoError> {
//         let key_derivation = KeyDerivation::default();
//         let salt = key_derivation.generate_salt()?;
//
//         // Create a new scope for testing cleanup
//         {
//             // Derive a key in SecureMemory
//             let derived_key = key_derivation.derive_key(TEST_PASSWORD, &salt)?;
//
//             // Verify the derived key has the expected length
//             assert_eq!(derived_key.as_ref().len(), key_derivation.key_length());
//             // Verify the key isn't all zeros (extremely unlikely for a properly derived key)
//             assert!(derived_key.as_ref().iter().any(|&byte| byte != 0));
//
//             // Create hierarchy
//             let (hierarchy, _) = KeyHierarchy::new(TEST_PASSWORD)?;
//
//             // Use the hierarchy
//             let encrypted = hierarchy.encrypt_data(TEST_DATA)?;
//             assert_eq!(hierarchy.decrypt_data(&encrypted)?, TEST_DATA);
//
//             // Let everything drop here
//         }
//
//         // Create new hierarchy to verify independence
//         let (new_hierarchy, _) = KeyHierarchy::new(TEST_PASSWORD)?;
//         let new_encrypted = new_hierarchy.encrypt_data(TEST_DATA)?;
//
//         // Should work fine even though previous instances were dropped
//         assert_eq!(new_hierarchy.decrypt_data(&new_encrypted)?, TEST_DATA);
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_password_change_with_multiple_entries() -> Result<(), CryptoError> {
//         let (mut hierarchy, salt, _) = create_test_setup()?;
//
//         // Encrypt multiple pieces of data
//         let encrypted1 = hierarchy.encrypt_data(b"data1")?;
//         let encrypted2 = hierarchy.encrypt_data(b"data2")?;
//         let encrypted3 = hierarchy.encrypt_data(b"data3")?;
//
//         // Change password
//         let new_password = b"new_secure_password";
//         let new_salt = hierarchy.change_master_password(TEST_PASSWORD, new_password, &salt)?;
//
//         // Verify everything still works
//         assert_eq!(hierarchy.decrypt_data(&encrypted1)?, b"data1");
//         assert_eq!(hierarchy.decrypt_data(&encrypted2)?, b"data2");
//         assert_eq!(hierarchy.decrypt_data(&encrypted3)?, b"data3");
//
//         // Create new hierarchy with new password
//         let encrypted_mek = hierarchy.encrypted_mek()?;
//         let new_hierarchy = KeyHierarchy::from_existing(new_password, &encrypted_mek, &new_salt)?;
//
//         // Verify new hierarchy can decrypt old data
//         assert_eq!(new_hierarchy.decrypt_data(&encrypted1)?, b"data1");
//         assert_eq!(new_hierarchy.decrypt_data(&encrypted2)?, b"data2");
//         assert_eq!(new_hierarchy.decrypt_data(&encrypted3)?, b"data3");
//
//         Ok(())
//     }
// }