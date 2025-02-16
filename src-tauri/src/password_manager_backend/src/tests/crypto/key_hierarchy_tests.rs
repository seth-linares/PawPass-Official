// #[cfg(test)]
// mod tests {
//     use std::vec;
//     use crate::crypto::{EncryptedData, KeyHierarchy};
//     use crate::error::crypto_error::CryptoError;
//
//     const TEST_PASSWORD: &[u8] = b"test_password123";
//     const TEST_DATA: &[u8] = b"Hello, World!";
//     const WRONG_PASSWORD: &[u8] = b"wrong_password123";
//
//     fn create_test_hierarchy() -> (KeyHierarchy, Vec<u8>) {
//         KeyHierarchy::new(TEST_PASSWORD).unwrap()
//     }
//
//     #[test]
//     fn test_new_creation() {
//         let (hierarchy, salt) = create_test_hierarchy();
//
//         // Test that we can encrypt and decrypt data
//         let encrypted = hierarchy.encrypt_data(TEST_DATA).unwrap();
//         let decrypted = hierarchy.decrypt_data(&encrypted).unwrap();
//
//         assert_eq!(decrypted, TEST_DATA);
//         assert!(!salt.is_empty());
//     }
//
//     #[test]
//     fn test_from_existing() {
//         // Create initial hierarchy
//         let (hierarchy, salt) = create_test_hierarchy();
//
//         // Get encrypted MEK
//         let encrypted_mek = hierarchy.encrypted_mek().unwrap();
//
//         // Create new instance from existing data
//         let new_hierarchy = KeyHierarchy::from_existing(
//             TEST_PASSWORD,
//             &encrypted_mek,
//             &salt
//         ).unwrap();
//
//         // Test that both can decrypt the same data
//         let encrypted = hierarchy.encrypt_data(TEST_DATA).unwrap();
//         let decrypted = new_hierarchy.decrypt_data(&encrypted).unwrap();
//
//         assert_eq!(decrypted, TEST_DATA);
//     }
//
//     #[test]
//     fn test_verify_master_password() {
//         let (hierarchy, salt) = create_test_hierarchy();
//
//         // Test correct password
//         assert!(hierarchy.verify_master_password(TEST_PASSWORD, &salt).unwrap());
//
//         // Test incorrect password
//         assert!(!hierarchy.verify_master_password(WRONG_PASSWORD, &salt).unwrap());
//     }
//
//     #[test]
//     fn test_change_master_password() {
//         let (mut hierarchy, old_salt) = create_test_hierarchy();
//
//         // Encrypt some data before password change
//         let encrypted = hierarchy.encrypt_data(TEST_DATA).unwrap();
//
//         // Change password
//         let new_password = b"new_password123";
//         let new_salt = hierarchy.change_master_password(
//             TEST_PASSWORD,
//             new_password,
//             &old_salt
//         ).unwrap();
//
//         // Verify old password fails
//         assert!(!hierarchy.verify_master_password(TEST_PASSWORD, &new_salt).unwrap());
//
//         // Verify new password works
//         assert!(hierarchy.verify_master_password(new_password, &new_salt).unwrap());
//
//         // Verify can still decrypt old data
//         let decrypted = hierarchy.decrypt_data(&encrypted).unwrap();
//         assert_eq!(decrypted, TEST_DATA);
//     }
//
//     #[test]
//     fn test_invalid_master_password() {
//         let (hierarchy, salt) = create_test_hierarchy();
//
//         // Test wrong password with from_existing
//         let encrypted_mek = hierarchy.encrypted_mek().unwrap();
//         let result = KeyHierarchy::from_existing(
//             WRONG_PASSWORD,
//             &encrypted_mek,
//             &salt
//         );
//         assert!(matches!(result.unwrap_err(), CryptoError::MekDecryptionFailed));
//
//         // Test empty password
//         let result = KeyHierarchy::new(&[]);
//         assert!(matches!(result.unwrap_err(), CryptoError::EmptyPassword));
//     }
//
//     #[test]
//     fn test_data_encryption() {
//         let (hierarchy, _) = create_test_hierarchy();
//
//         // Test basic encryption
//         let encrypted1 = hierarchy.encrypt_data(TEST_DATA).unwrap();
//         let encrypted2 = hierarchy.encrypt_data(TEST_DATA).unwrap();
//
//         // Verify different ciphertexts for same data (due to different nonces)
//         assert_ne!(encrypted1.ciphertext(), encrypted2.ciphertext());
//
//         // Verify different nonces
//         assert_ne!(encrypted1.nonce(), encrypted2.nonce());
//
//         // Test empty data fails
//         let result = hierarchy.encrypt_data(&[]);
//         assert!(matches!(result.unwrap_err(), CryptoError::EmptyData));
//     }
//
//     #[test]
//     fn test_data_decryption() {
//         let (hierarchy, _) = create_test_hierarchy();
//
//         // Test round trip with different data sizes
//         let test_cases = vec![
//             TEST_DATA,
//             b"Small",
//             &[42; 1000],  // Large data
//         ];
//
//         for data in test_cases {
//             let encrypted = hierarchy.encrypt_data(data).unwrap();
//             let decrypted = hierarchy.decrypt_data(&encrypted).unwrap();
//             assert_eq!(decrypted, data);
//         }
//     }
//
//     #[test]
//     fn test_mek_encryption() {
//         let (hierarchy, _) = create_test_hierarchy();
//
//         // Get encrypted MEK
//         let encrypted_mek = hierarchy.encrypted_mek().unwrap();
//
//         // Verify encrypted MEK is not empty
//         assert!(!encrypted_mek.ciphertext().is_empty());
//
//         // Verify nonce and tag are present
//         assert_eq!(encrypted_mek.nonce().len(), 12);
//         assert_eq!(encrypted_mek.tag().len(), 16);
//     }
//
//     #[test]
//     fn test_change_master_password_with_wrong_password() {
//         let (mut hierarchy, salt) = create_test_hierarchy();
//
//         let result = hierarchy.change_master_password(
//             WRONG_PASSWORD,
//             b"new_password",
//             &salt
//         );
//
//         assert!(matches!(result.unwrap_err(), CryptoError::InvalidMasterPassword));
//     }
//
//     #[test]
//     fn test_from_existing_with_invalid_salt() {
//         let (hierarchy, _) = create_test_hierarchy();
//         let encrypted_mek = hierarchy.encrypted_mek().unwrap();
//
//         // Try with wrong salt length
//         let result = KeyHierarchy::from_existing(
//             TEST_PASSWORD,
//             &encrypted_mek,
//             &[1, 2, 3] // Invalid salt length
//         );
//
//         assert!(matches!(result.unwrap_err(), CryptoError::InvalidSaltLength));
//     }
//
//     #[test]
//     fn test_data_tampering() {
//         let (hierarchy, _) = create_test_hierarchy();
//         let encrypted = hierarchy.encrypt_data(TEST_DATA).unwrap();
//
//         // Create modified version by tampering with ciphertext
//         let mut tampered_data = encrypted.ciphertext().to_vec();
//         if let Some(byte) = tampered_data.first_mut() {
//             *byte ^= 1; // Flip one bit
//         }
//
//         let tampered = EncryptedData::new(
//             tampered_data,
//             *encrypted.nonce(),
//             *encrypted.tag()
//         ).unwrap();
//
//         // Decryption should fail
//         let result = hierarchy.decrypt_data(&tampered);
//         assert!(matches!(result.unwrap_err(), CryptoError::DecryptionFailed));
//     }
//
//     #[test]
//     fn test_key_isolation() {
//         // Create two hierarchies
//         let (hierarchy1, _) = create_test_hierarchy();
//         let (hierarchy2, _) = KeyHierarchy::new(b"different_password").unwrap();
//
//         // Encrypt with first hierarchy
//         let encrypted = hierarchy1.encrypt_data(TEST_DATA).unwrap();
//
//         // Try to decrypt with second hierarchy (should fail)
//         let result = hierarchy2.decrypt_data(&encrypted);
//         assert!(matches!(result.unwrap_err(), CryptoError::DecryptionFailed));
//     }
//
//     #[test]
//     fn test_password_change_maintains_encryption() {
//         let (mut hierarchy, old_salt) = create_test_hierarchy();
//
//         // Create multiple encrypted items
//         let encrypted1 = hierarchy.encrypt_data(b"data1").unwrap();
//         let encrypted2 = hierarchy.encrypt_data(b"data2").unwrap();
//
//         // Change password
//         let new_salt = hierarchy.change_master_password(
//             TEST_PASSWORD,
//             b"new_password123",
//             &old_salt
//         ).unwrap();
//
//         // Verify new salt works with new password
//         assert!(hierarchy.verify_master_password(b"new_password123", &new_salt).unwrap());
//
//         // Verify all old data can still be decrypted
//         assert_eq!(
//             hierarchy.decrypt_data(&encrypted1).unwrap(),
//             b"data1"
//         );
//         assert_eq!(
//             hierarchy.decrypt_data(&encrypted2).unwrap(),
//             b"data2"
//         );
//     }
//
//     #[test]
//     fn test_concurrent_operations() {
//         use std::thread;
//
//         let (hierarchy, _) = create_test_hierarchy();
//         let hierarchy = std::sync::Arc::new(hierarchy);
//
//         let mut handles = vec![];
//
//         // Spawn multiple threads doing encryption/decryption
//         for i in 0..10 {
//             let hierarchy = hierarchy.clone();
//             let handle = thread::spawn(move || {
//                 let data = format!("data{}", i);
//                 let encrypted = hierarchy.encrypt_data(data.as_bytes()).unwrap();
//                 let decrypted = hierarchy.decrypt_data(&encrypted).unwrap();
//                 assert_eq!(data.as_bytes(), decrypted);
//             });
//             handles.push(handle);
//         }
//
//         // Wait for all threads
//         for handle in handles {
//             handle.join().unwrap();
//         }
//     }
//
//     #[test]
//     fn test_large_data_handling() {
//         let (hierarchy, _) = create_test_hierarchy();
//
//         // Test with 1MB of data
//         let large_data = vec![42u8; 1024 * 1024];
//         let encrypted = hierarchy.encrypt_data(&large_data).unwrap();
//         let decrypted = hierarchy.decrypt_data(&encrypted).unwrap();
//
//         assert_eq!(large_data, decrypted);
//     }
// }