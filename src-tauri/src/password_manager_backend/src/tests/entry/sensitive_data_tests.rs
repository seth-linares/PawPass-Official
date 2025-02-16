// #[cfg(test)]
// mod tests {
//     use crate::crypto::{EncryptedData, KeyHierarchy};
//     use std::thread;
//     use std::time::Duration;
//     use crate::entry::EncryptedSensitiveData;
//     use crate::error::entry_error::EntryError;
//
//     fn setup_key_hierarchy() -> (KeyHierarchy, Vec<u8>) {
//         let master_password = b"test_password";
//         let (kh, salt) = KeyHierarchy::new(master_password).unwrap();
//         (kh, salt)
//     }
//
//     #[test]
//     fn test_basic_encryption_decryption() {
//         let (key_hierarchy, _) = setup_key_hierarchy();
//         let password = "mySecurePassword123";
//
//         let sensitive_data = EncryptedSensitiveData::new(
//             password,
//             None,
//             &key_hierarchy
//         ).unwrap();
//
//         let decrypted = sensitive_data.decrypt(&key_hierarchy).unwrap();
//         assert_eq!(decrypted.password(), password);
//         assert_eq!(decrypted.notes(), None);
//     }
//
//     #[test]
//     fn test_with_notes() {
//         let (key_hierarchy, _) = setup_key_hierarchy();
//         let password = "mySecurePassword123";
//         let notes = "These are my secure notes";
//
//         let sensitive_data = EncryptedSensitiveData::new(
//             password,
//             Some(notes),
//             &key_hierarchy
//         ).unwrap();
//
//         let decrypted = sensitive_data.decrypt(&key_hierarchy).unwrap();
//         assert_eq!(decrypted.password(), password);
//         assert_eq!(decrypted.notes(), Some(notes));
//     }
//
//     #[test]
//     fn test_empty_password() {
//         let (key_hierarchy, _) = setup_key_hierarchy();
//         let result = EncryptedSensitiveData::new(
//             "",
//             None,
//             &key_hierarchy
//         );
//         assert!(matches!(result, Err(EntryError::InvalidPassword)));
//     }
//
//     #[test]
//     fn test_decryption_with_wrong_key() {
//         let (key_hierarchy1, _) = setup_key_hierarchy();
//         let (key_hierarchy2, _) = setup_key_hierarchy();
//
//         let password = "mySecurePassword123";
//         let sensitive_data = EncryptedSensitiveData::new(
//             password,
//             None,
//             &key_hierarchy1
//         ).unwrap();
//
//         let result = sensitive_data.decrypt(&key_hierarchy2);
//         assert!(matches!(result, Err(EntryError::DecryptionError(_))));
//     }
//
//     #[test]
//     fn test_memory_cleanup() {
//         let (key_hierarchy, _) = setup_key_hierarchy();
//         let password = "mySecurePassword123";
//         let notes = "These are my secure notes";
//
//         // Create a block to test Drop implementation
//         {
//             let sensitive_data = EncryptedSensitiveData::new(
//                 password,
//                 Some(notes),
//                 &key_hierarchy
//             ).unwrap();
//
//             let decrypted = sensitive_data.decrypt(&key_hierarchy).unwrap();
//
//             // Verify data is accessible
//             assert_eq!(decrypted.password(), password);
//             assert_eq!(decrypted.notes(), Some(notes));
//
//             // Data should be zeroed when decrypted goes out of scope
//         }
//
//         // Give a moment for any async cleanup
//         thread::sleep(Duration::from_millis(10));
//     }
//
//     #[test]
//     fn test_decrypt_sensitive_data_methods() {
//         let (key_hierarchy, _) = setup_key_hierarchy();
//         let password = "mySecurePassword123";
//         let notes = "These are my secure notes";
//
//         let sensitive_data = EncryptedSensitiveData::new(
//             password,
//             Some(notes),
//             &key_hierarchy
//         ).unwrap();
//
//         let decrypted = sensitive_data.decrypt(&key_hierarchy).unwrap();
//
//         // Test password access
//         assert_eq!(decrypted.password(), password);
//
//         // Test notes access with Some value
//         assert_eq!(decrypted.notes(), Some(notes));
//
//         // Create another instance without notes
//         let sensitive_data_no_notes = EncryptedSensitiveData::new(
//             password,
//             None,
//             &key_hierarchy
//         ).unwrap();
//
//         let decrypted_no_notes = sensitive_data_no_notes.decrypt(&key_hierarchy).unwrap();
//
//         // Test notes access with None value
//         assert_eq!(decrypted_no_notes.notes(), None);
//     }
//
//     #[test]
//     fn test_non_utf8_handling() {
//         let (key_hierarchy, _) = setup_key_hierarchy();
//         let password = "mySecurePassword123";
//         let sensitive_data = EncryptedSensitiveData::new(
//             password,
//             None,
//             &key_hierarchy
//         ).unwrap();
//
//         // Manually corrupt the encrypted data (this is a bit hacky and just for testing)
//         let corrupted = EncryptedData::new(
//             vec![0xFF, 0xFF, 0xFF], // Invalid UTF-8
//             sensitive_data.password.nonce().clone(),
//             sensitive_data.password.tag().clone()
//         ).unwrap();
//
//         let corrupted_sensitive = EncryptedSensitiveData {
//             password: corrupted,
//             notes: None,
//         };
//
//         // Attempt to decrypt corrupted data
//         let result = corrupted_sensitive.decrypt(&key_hierarchy);
//         assert!(matches!(result, Err(EntryError::DecryptionError(_))));
//     }
//
//     #[test]
//     fn test_large_data_handling() {
//         let (key_hierarchy, _) = setup_key_hierarchy();
//         let long_password = "a".repeat(1000);
//         let long_notes = "b".repeat(10000);
//
//         let sensitive_data = EncryptedSensitiveData::new(
//             &long_password,
//             Some(&long_notes),
//             &key_hierarchy
//         ).unwrap();
//
//         let decrypted = sensitive_data.decrypt(&key_hierarchy).unwrap();
//         assert_eq!(decrypted.password(), long_password);
//         assert_eq!(decrypted.notes(), Some(long_notes.as_str()));
//     }
//
//     #[test]
//     fn test_empty_notes() {
//         let (key_hierarchy, _) = setup_key_hierarchy();
//         let password = "validPassword";
//         let empty_notes = "";
//
//         let sensitive_data = EncryptedSensitiveData::new(
//             password,
//             Some(empty_notes),
//             &key_hierarchy
//         ).unwrap();
//
//         let decrypted = sensitive_data.decrypt(&key_hierarchy).unwrap();
//         // Empty notes should be stored as None
//         assert_eq!(decrypted.notes(), None);
//     }
//
//     #[test]
//     fn test_special_characters() {
//         let (key_hierarchy, _) = setup_key_hierarchy();
//         let password = "pass🔐word⚡️";
//         let notes = "Note📝with👻emoji🎉";
//
//         let sensitive_data = EncryptedSensitiveData::new(
//             password,
//             Some(notes),
//             &key_hierarchy
//         ).unwrap();
//
//         let decrypted = sensitive_data.decrypt(&key_hierarchy).unwrap();
//         assert_eq!(decrypted.password(), password);
//         assert_eq!(decrypted.notes(), Some(notes));
//     }
//
//     #[test]
//     fn test_concurrent_access() {
//         use std::sync::Arc;
//
//         let (key_hierarchy, _) = setup_key_hierarchy();
//         let key_hierarchy = Arc::new(key_hierarchy);
//         let password = "threadSafePassword";
//
//         let sensitive_data = Arc::new(EncryptedSensitiveData::new(
//             password,
//             None,
//             &key_hierarchy
//         ).unwrap());
//
//         let mut handles = vec![];
//
//         for _ in 0..4 {
//             let sensitive_data = Arc::clone(&sensitive_data);
//             let key_hierarchy = Arc::clone(&key_hierarchy);
//
//             let handle = thread::spawn(move || {
//                 let decrypted = sensitive_data.decrypt(&key_hierarchy).unwrap();
//                 assert_eq!(decrypted.password(), password);
//             });
//
//             handles.push(handle);
//         }
//
//         for handle in handles {
//             handle.join().unwrap();
//         }
//     }
//
//     #[test]
//     fn test_multiple_decryption_same_data() {
//         let (key_hierarchy, _) = setup_key_hierarchy();
//         let password = "multipleDecryptions";
//
//         let sensitive_data = EncryptedSensitiveData::new(
//             password,
//             None,
//             &key_hierarchy
//         ).unwrap();
//
//         // Ensure multiple decryptions of the same data work correctly
//         for _ in 0..5 {
//             let decrypted = sensitive_data.decrypt(&key_hierarchy).unwrap();
//             assert_eq!(decrypted.password(), password);
//             // Let previous decrypted data drop
//         }
//     }
//
//     #[test]
//     fn test_null_characters() {
//         let (key_hierarchy, _) = setup_key_hierarchy();
//         let password = "pass\0word\0";
//         let notes = "note\0with\0nulls";
//
//         let sensitive_data = EncryptedSensitiveData::new(
//             password,
//             Some(notes),
//             &key_hierarchy
//         ).unwrap();
//
//         let decrypted = sensitive_data.decrypt(&key_hierarchy).unwrap();
//         assert_eq!(decrypted.password(), password);
//         assert_eq!(decrypted.notes(), Some(notes));
//     }
//
//     #[test]
//     fn test_memory_isolation() {
//         let (key_hierarchy, _) = setup_key_hierarchy();
//         let password = "memoryTestPassword";
//
//         let sensitive_data = EncryptedSensitiveData::new(
//             password,
//             None,
//             &key_hierarchy
//         ).unwrap();
//
//         // Create multiple decrypted instances
//         let decrypted1 = sensitive_data.decrypt(&key_hierarchy).unwrap();
//         let decrypted2 = sensitive_data.decrypt(&key_hierarchy).unwrap();
//
//         // Verify they have separate memory
//         assert_eq!(decrypted1.password(), password);
//         assert_eq!(decrypted2.password(), password);
//         assert!(!std::ptr::eq(
//             decrypted1.password() as *const str,
//             decrypted2.password() as *const str
//         ));
//     }
// }