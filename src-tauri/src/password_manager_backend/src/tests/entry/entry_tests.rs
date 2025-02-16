// #[cfg(test)]
// mod tests {
//     use crate::crypto::KeyHierarchy;
//     use crate::entry::overview::EntryData;
//     use chrono::TimeDelta;
//     use uuid::Uuid;
//
//     // Test helpers
//     fn create_test_key_hierarchy() -> KeyHierarchy {
//         let master_password = b"test_password_123";
//         let (key_hierarchy, _) = KeyHierarchy::new(master_password)
//             .expect("Failed to create test key hierarchy");
//         key_hierarchy
//     }
//
//     fn create_basic_new_entry() -> EntryData {
//         EntryData {
//             title: "Test Entry".to_string(),
//             username: None,
//             password: "test_password".to_string(),
//             url: None,
//             notes: None,
//             category_id: None,
//         }
//     }
//
//     fn create_full_new_entry() -> EntryData {
//         EntryData {
//             title: "Full Test Entry".to_string(),
//             username: Some("testuser".to_string()),
//             password: "test_password_123".to_string(),
//             url: Some("https://example.com".to_string()),
//             notes: Some("Test notes".to_string()),
//             category_id: Some(Uuid::new_v4()),
//         }
//     }
//
//     mod construction {
//         use super::*;
//         use crate::entry::Entry;
//         use crate::error::entry_error::EntryError;
//         use chrono::Utc;
//
//         #[test]
//         fn test_new_entry_basic_creation() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_basic_new_entry();
//             let entry = Entry::new(input.clone(), &key_hierarchy).unwrap();
//
//             assert_eq!(entry.title(), input.title);
//             assert_eq!(entry.username(), input.username.as_deref());
//             assert_eq!(entry.url(), input.url.as_deref());
//             assert_eq!(entry.category_id(), input.category_id.as_ref());
//             assert!(!entry.favorite());
//
//             // Verify timestamps
//             assert!(entry.updated_at() >= entry.created_at());
//
//             // Verify password was properly encrypted and can be decrypted
//             let decrypted = entry.decrypt_sensitive(&key_hierarchy).unwrap();
//             assert_eq!(decrypted.password(), input.password);
//             assert_eq!(decrypted.notes(), None);
//         }
//
//         #[test]
//         fn test_new_entry_full_creation() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_full_new_entry();
//             let entry = Entry::new(input.clone(), &key_hierarchy).unwrap();
//
//             assert_eq!(entry.title(), input.title);
//             assert_eq!(entry.username(), input.username.as_deref());
//             assert_eq!(entry.url(), input.url.as_deref());
//             assert_eq!(entry.category_id(), input.category_id.as_ref());
//             assert!(!entry.favorite());
//
//             // Verify timestamps
//             assert!(entry.updated_at() >= entry.created_at());
//
//             // Verify sensitive data
//             let decrypted = entry.decrypt_sensitive(&key_hierarchy).unwrap();
//             assert_eq!(decrypted.password(), input.password);
//             assert_eq!(decrypted.notes(), input.notes.as_deref());
//         }
//
//         #[test]
//         fn test_new_entry_empty_title() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let mut input = create_basic_new_entry();
//             input.title = String::new();
//
//             let result = Entry::new(input, &key_hierarchy);
//             assert!(matches!(result, Err(EntryError::InvalidTitle)));
//         }
//
//         #[test]
//         fn test_from_raw_parts_valid() {
//             let key_hierarchy = create_test_key_hierarchy();
//
//             // First create a normal entry to get valid encrypted data
//             let input = create_basic_new_entry();
//             let entry = Entry::new(input.clone(), &key_hierarchy).unwrap();
//
//             // Now create from raw parts
//             let result = Entry::from_raw_parts(
//                 Uuid::new_v4(),
//                 "Test Title".to_string(),
//                 Some("testuser".to_string()),
//                 Some("https://example.com".to_string()),
//                 Some(Uuid::new_v4()),
//                 true,
//                 Utc::now(),
//                 Utc::now(),
//                 entry.sensitive_data,
//             );
//
//             assert!(result.is_ok());
//         }
//
//         #[test]
//         fn test_from_raw_parts_invalid_timestamps() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_basic_new_entry();
//             let entry = Entry::new(input, &key_hierarchy).unwrap();
//
//             let now = Utc::now();
//             let earlier = now - TimeDelta::try_hours(1).unwrap();
//
//             let result = Entry::from_raw_parts(
//                 Uuid::new_v4(),
//                 "Test Title".to_string(),
//                 None,
//                 None,
//                 None,
//                 false,
//                 now,
//                 earlier, // updated_at before created_at
//                 entry.sensitive_data,
//             );
//
//             assert!(matches!(
//                 result,
//                 Err(EntryError::ValidationError(_))
//             ));
//         }
//     }
//
//     mod overview {
//         use super::*;
//         use crate::entry::overview::UpdateEntry;
//         use crate::entry::Entry;
//         use crate::error::entry_error::EntryError;
//
//         #[test]
//         fn test_to_overview_basic() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_basic_new_entry();
//             let entry = Entry::new(input.clone(), &key_hierarchy).unwrap();
//
//             let overview = entry.to_overview();
//
//             assert_eq!(overview.id, *entry.id());
//             assert_eq!(overview.title, entry.title());
//             assert_eq!(overview.username, entry.username().map(String::from));
//             assert_eq!(overview.url, entry.url().map(String::from));
//             assert_eq!(overview.category_id, entry.category_id().cloned());
//             assert_eq!(overview.favorite, entry.favorite());
//             assert_eq!(overview.updated_at, entry.updated_at());
//         }
//
//         #[test]
//         fn test_to_overview_full() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_full_new_entry();
//             let entry = Entry::new(input.clone(), &key_hierarchy).unwrap();
//
//             let overview = entry.to_overview();
//
//             assert_eq!(overview.id, *entry.id());
//             assert_eq!(overview.title, entry.title());
//             assert_eq!(overview.username, entry.username().map(String::from));
//             assert_eq!(overview.url, entry.url().map(String::from));
//             assert_eq!(overview.category_id, entry.category_id().cloned());
//             assert_eq!(overview.favorite, entry.favorite());
//             assert_eq!(overview.updated_at, entry.updated_at());
//
//             // Verify overview doesn't contain sensitive data
//             let overview_debug = format!("{:?}", overview);
//             assert!(!overview_debug.contains("password"));
//             assert!(!overview_debug.contains("notes"));
//         }
//
//         #[test]
//         fn test_update_title() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_basic_new_entry();
//             let mut entry = Entry::new(input, &key_hierarchy).unwrap();
//             let original_updated_at = entry.updated_at();
//
//             let changes = UpdateEntry {
//                 title: Some("New Title".to_string()),
//                 username: None,
//                 password: None,
//                 url: None,
//                 notes: None,
//                 category_id: None,
//                 favorite: None,
//             };
//
//             entry.update(changes, &key_hierarchy).unwrap();
//
//             assert_eq!(entry.title(), "New Title");
//             assert!(entry.updated_at() > original_updated_at);
//         }
//
//         #[test]
//         fn test_update_username() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_basic_new_entry();
//             let mut entry = Entry::new(input, &key_hierarchy).unwrap();
//
//             // Add username
//             let add_username = UpdateEntry {
//                 title: None,
//                 username: Some(Some("newuser".to_string())),
//                 password: None,
//                 url: None,
//                 notes: None,
//                 category_id: None,
//                 favorite: None,
//             };
//             entry.update(add_username, &key_hierarchy).unwrap();
//             assert_eq!(entry.username(), Some("newuser"));
//
//             // Remove username
//             let remove_username = UpdateEntry {
//                 title: None,
//                 username: Some(None),
//                 password: None,
//                 url: None,
//                 notes: None,
//                 category_id: None,
//                 favorite: None,
//             };
//             entry.update(remove_username, &key_hierarchy).unwrap();
//             assert_eq!(entry.username(), None);
//         }
//
//         #[test]
//         fn test_update_password() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_basic_new_entry();
//             let old_password = input.password.clone();
//             let mut entry = Entry::new(input, &key_hierarchy).unwrap();
//
//             let changes = UpdateEntry {
//                 title: None,
//                 username: None,
//                 password: Some("newpassword123".to_string()),
//                 url: None,
//                 notes: None,
//                 category_id: None,
//                 favorite: None,
//             };
//
//             entry.update(changes, &key_hierarchy).unwrap();
//
//             let decrypted = entry.decrypt_sensitive(&key_hierarchy).unwrap();
//             assert_eq!(decrypted.password(), "newpassword123");
//             assert_ne!(decrypted.password(), old_password);
//         }
//
//         #[test]
//         fn test_update_notes() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_basic_new_entry();
//             let mut entry = Entry::new(input, &key_hierarchy).unwrap();
//
//             // Add notes
//             let add_notes = UpdateEntry {
//                 title: None,
//                 username: None,
//                 password: None,
//                 url: None,
//                 notes: Some(Some("New notes".to_string())),
//                 category_id: None,
//                 favorite: None,
//             };
//             entry.update(add_notes, &key_hierarchy).unwrap();
//
//             let decrypted = entry.decrypt_sensitive(&key_hierarchy).unwrap();
//             assert_eq!(decrypted.notes(), Some("New notes"));
//
//             // Remove notes
//             let remove_notes = UpdateEntry {
//                 title: None,
//                 username: None,
//                 password: None,
//                 url: None,
//                 notes: Some(None),
//                 category_id: None,
//                 favorite: None,
//             };
//             entry.update(remove_notes, &key_hierarchy).unwrap();
//
//             let decrypted = entry.decrypt_sensitive(&key_hierarchy).unwrap();
//             assert_eq!(decrypted.notes(), None);
//         }
//
//         #[test]
//         fn test_update_favorite() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_basic_new_entry();
//             let mut entry = Entry::new(input, &key_hierarchy).unwrap();
//             assert!(!entry.favorite());
//
//             let changes = UpdateEntry {
//                 title: None,
//                 username: None,
//                 password: None,
//                 url: None,
//                 notes: None,
//                 category_id: None,
//                 favorite: Some(true),
//             };
//
//             entry.update(changes, &key_hierarchy).unwrap();
//             assert!(entry.favorite());
//         }
//
//         #[test]
//         fn test_update_empty_title() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_basic_new_entry();
//             let mut entry = Entry::new(input, &key_hierarchy).unwrap();
//
//             let changes = UpdateEntry {
//                 title: Some("".to_string()),
//                 username: None,
//                 password: None,
//                 url: None,
//                 notes: None,
//                 category_id: None,
//                 favorite: None,
//             };
//
//             let result = entry.update(changes, &key_hierarchy);
//             assert!(matches!(result, Err(EntryError::InvalidTitle)));
//         }
//     }
//
//     mod sensitive_data {
//         use super::*;
//         use crate::entry::Entry;
//         use crate::error::entry_error::EntryError;
//
//         #[test]
//         fn test_decrypt_password() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_basic_new_entry();
//             let password = input.password.clone();
//             let entry = Entry::new(input, &key_hierarchy).unwrap();
//
//             let decrypted = entry.decrypt_sensitive(&key_hierarchy).unwrap();
//             assert_eq!(decrypted.password(), password);
//         }
//
//         #[test]
//         fn test_decrypt_notes() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let mut input = create_basic_new_entry();
//             input.notes = Some("Test notes".to_string());
//             let notes = input.notes.clone();
//             let entry = Entry::new(input, &key_hierarchy).unwrap();
//
//             let decrypted = entry.decrypt_sensitive(&key_hierarchy).unwrap();
//             assert_eq!(decrypted.notes(), notes.as_deref());
//         }
//
//         #[test]
//         fn test_sensitive_data_updated_timestamp() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_basic_new_entry();
//             let mut entry = Entry::new(input, &key_hierarchy).unwrap();
//             let original_updated_at = entry.updated_at();
//
//             // Update password
//             entry.update_sensitive(
//                 Some("newpassword456"),
//                 None,
//                 &key_hierarchy,
//             ).unwrap();
//
//             assert!(entry.updated_at() > original_updated_at);
//         }
//
//         #[test]
//         fn test_no_sensitive_data_in_debug() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_full_new_entry();
//             let entry = Entry::new(input, &key_hierarchy).unwrap();
//
//             let debug_output = format!("{:?}", entry);
//             assert!(!debug_output.contains(&entry.decrypt_sensitive(&key_hierarchy).unwrap().password()));
//             assert!(!debug_output.contains("password"));
//             assert!(debug_output.contains("[REDACTED]"));
//         }
//
//         #[test]
//         fn test_update_sensitive_only_password() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_full_new_entry();
//             let mut entry = Entry::new(input, &key_hierarchy).unwrap();
//
//             let new_password = "newpassword789";
//             entry.update_sensitive(Some(new_password), None, &key_hierarchy).unwrap();
//
//             let decrypted = entry.decrypt_sensitive(&key_hierarchy).unwrap();
//             assert_eq!(decrypted.password(), new_password);
//             // Verify notes remained unchanged
//             assert_eq!(decrypted.notes(), Some("Test notes"));
//         }
//
//         #[test]
//         fn test_update_sensitive_empty_password() {
//             let key_hierarchy = create_test_key_hierarchy();
//             let input = create_basic_new_entry();
//             let mut entry = Entry::new(input, &key_hierarchy).unwrap();
//
//             let result = entry.update_sensitive(Some(""), None, &key_hierarchy);
//             assert!(matches!(result, Err(EntryError::InvalidPassword)));
//         }
//     }
// }