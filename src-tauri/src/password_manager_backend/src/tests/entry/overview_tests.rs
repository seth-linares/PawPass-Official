// use chrono::Utc;
// use uuid::Uuid;
// use crate::entry::entry_management::UpdateEntry;
// use crate::entry::overview::{EntryOverview, NewEntry, UpdateEntry};
//
// // Test helper functions
// fn create_valid_new_entry() -> NewEntry {
//     NewEntry {
//         title: "Test Entry".to_string(),
//         username: Some("testuser".to_string()),
//         password: Some("securepassword123".to_string()),
//         url: Some("https://example.com".to_string()),
//         notes: Some("Test notes".to_string()),
//         category_id: Some(Uuid::new_v4()),
//     }
// }
//
// fn create_valid_update_entry() -> UpdateEntry {
//     UpdateEntry {
//         title: Some("Updated Title".to_string()),
//         username: Some(Some("updateduser".to_string())),
//         password: Some("newpassword123".to_string()),
//         url: Some(Some("https://updated.example.com".to_string())),
//         notes: Some(Some("Updated notes".to_string())),
//         category_id: Some(Some(Uuid::new_v4())),
//         favorite: Some(true),
//     }
// }
//
// fn create_valid_overview() -> EntryOverview {
//     EntryOverview {
//         id: Uuid::new_v4(),
//         title: "Test Entry".to_string(),
//         username: Some("testuser".to_string()),
//         url: Some("https://example.com".to_string()),
//         category_id: Some(Uuid::new_v4()),
//         favorite: false,
//         updated_at: Utc::now(),
//     }
// }
//
// fn create_max_length_strings() -> (String, String, String) {
//     (
//         "a".repeat(100),  // MAX_TITLE_LENGTH
//         "a".repeat(100),  // MAX_USERNAME_LENGTH
//         "a".repeat(10000) // MAX_NOTES_LENGTH
//     )
// }
//
// #[cfg(test)]
// mod new_entry_validation_tests {
//     use crate::tests::entry::overview_tests::create_valid_new_entry;
//
//     // 1.1 Title Tests
//     #[test]
//     fn test_empty_title_fails() {
//         let mut entry = create_valid_new_entry();
//         entry.title = "".to_string();
//         assert!(entry.validate().is_err());
//     }
//
//     #[test]
//     fn test_title_too_long_fails() {
//         let mut entry = create_valid_new_entry();
//         entry.title = "a".repeat(101); // MAX_TITLE_LENGTH + 1
//         assert!(entry.validate().is_err());
//     }
//
//     #[test]
//     fn test_valid_title_passes() {
//         let entry = create_valid_new_entry();
//         assert!(entry.validate().is_ok());
//     }
//
//     // 1.2 Username Tests
//     #[test]
//     fn test_valid_username_passes() {
//         let entry = create_valid_new_entry();
//         assert!(entry.validate().is_ok());
//     }
//
//     #[test]
//     fn test_username_too_long_fails() {
//         let mut entry = create_valid_new_entry();
//         entry.username = Some("a".repeat(101)); // MAX_USERNAME_LENGTH + 1
//         assert!(entry.validate().is_err());
//     }
//
//     #[test]
//     fn test_empty_username_valid() {
//         let mut entry = create_valid_new_entry();
//         entry.username = None;
//         assert!(entry.validate().is_ok());
//     }
//
//     // 1.3 Password Tests
//     #[test]
//     fn test_empty_password_fails() {
//         let mut entry = create_valid_new_entry();
//         entry.password = "".to_string();
//         assert!(entry.validate().is_err());
//     }
//
//     #[test]
//     fn test_valid_password_passes() {
//         let entry = create_valid_new_entry();
//         assert!(entry.validate().is_ok());
//     }
//
//     // 1.4 URL Tests
//     #[test]
//     fn test_valid_url_passes() {
//         let mut entry = create_valid_new_entry();
//         entry.url = Some("https://example.com/path?query=value".to_string());
//         assert!(entry.validate().is_ok());
//     }
//
//     #[test]
//     fn test_invalid_url_format_fails() {
//         let mut entry = create_valid_new_entry();
//         entry.url = Some("not a url".to_string());
//         assert!(entry.validate().is_err());
//     }
//
//     #[test]
//     fn test_url_too_long_fails() {
//         let mut entry = create_valid_new_entry();
//         entry.url = Some("https://example.com/".to_string() + &"a".repeat(2048));
//         assert!(entry.validate().is_err());
//     }
//
//     #[test]
//     fn test_empty_url_valid() {
//         let mut entry = create_valid_new_entry();
//         entry.url = None;
//         assert!(entry.validate().is_ok());
//     }
//
//     // 1.5 Notes Tests
//     #[test]
//     fn test_valid_notes_passes() {
//         let entry = create_valid_new_entry();
//         assert!(entry.validate().is_ok());
//     }
//
//     #[test]
//     fn test_notes_too_long_fails() {
//         let mut entry = create_valid_new_entry();
//         entry.notes = Some("a".repeat(10001)); // MAX_NOTES_LENGTH + 1
//         assert!(entry.validate().is_err());
//     }
//
//     #[test]
//     fn test_empty_notes_valid() {
//         let mut entry = create_valid_new_entry();
//         entry.notes = None;
//         assert!(entry.validate().is_ok());
//     }
//
//     // 1.6 Category Tests
//     #[test]
//     fn test_valid_category_id_passes() {
//         let entry = create_valid_new_entry();
//         assert!(entry.validate().is_ok());
//     }
//
//     #[test]
//     fn test_none_category_id_valid() {
//         let mut entry = create_valid_new_entry();
//         entry.category_id = None;
//         assert!(entry.validate().is_ok());
//     }
// }
//
// #[cfg(test)]
// mod update_entry_partial_tests {
//     use crate::entry::overview::UpdateEntry;
//
//
//
//
//     #[test]
//     fn test_empty_update_valid() {
//         let update = UpdateEntry {
//             title: None,
//             username: None,
//             password: None,
//             url: None,
//             notes: None,
//             category_id: None,
//             favorite: None,
//         };
//         assert!(update.validate().is_ok());
//     }
//
//     #[test]
//     fn test_partial_title_update() {
//         let mut update = UpdateEntry {
//             title: Some("New Title".to_string()),
//             username: None,
//             password: None,
//             url: None,
//             notes: None,
//             category_id: None,
//             favorite: None,
//         };
//         assert!(update.validate().is_ok());
//
//         update.title = Some("".to_string());
//         assert!(update.validate().is_err());
//
//         update.title = Some("a".repeat(101));
//         assert!(update.validate().is_err());
//     }
//
//     #[test]
//     fn test_partial_username_update() {
//         let update = UpdateEntry {
//             title: None,
//             username: Some(Some("newuser".to_string())),
//             password: None,
//             url: None,
//             notes: None,
//             category_id: None,
//             favorite: None,
//         };
//         assert!(update.validate().is_ok());
//     }
//
//     #[test]
//     fn test_partial_password_update() {
//         let mut update = UpdateEntry {
//             title: None,
//             username: None,
//             password: Some("newpassword123".to_string()),
//             url: None,
//             notes: None,
//             category_id: None,
//             favorite: None,
//         };
//         assert!(update.validate().is_ok());
//
//         update.password = Some("".to_string());
//         assert!(update.validate().is_err());
//     }
//
//     #[test]
//     fn test_partial_url_update() {
//         let mut update = UpdateEntry {
//             title: None,
//             username: None,
//             password: None,
//             url: Some(Some("https://new.example.com".to_string())),
//             notes: None,
//             category_id: None,
//             favorite: None,
//         };
//         assert!(update.validate().is_ok());
//
//         update.url = Some(Some("not a url".to_string()));
//         assert!(update.validate().is_err());
//     }
//
//     #[test]
//     fn test_partial_notes_update() {
//         let update = UpdateEntry {
//             title: None,
//             username: None,
//             password: None,
//             url: None,
//             notes: Some(Some("New notes".to_string())),
//             category_id: None,
//             favorite: None,
//         };
//         assert!(update.validate().is_ok());
//     }
// }
//
// #[cfg(test)]
// mod update_entry_optional_tests {
//     use crate::entry::overview::UpdateEntry;
//
//     #[test]
//     fn test_clear_username() {
//         let update = UpdateEntry {
//             title: None,
//             username: Some(None), // Setting username to None
//             password: None,
//             url: None,
//             notes: None,
//             category_id: None,
//             favorite: None,
//         };
//         assert!(update.validate().is_ok());
//     }
//
//     #[test]
//     fn test_clear_url() {
//         let update = UpdateEntry {
//             title: None,
//             username: None,
//             password: None,
//             url: Some(None), // Setting URL to None
//             notes: None,
//             category_id: None,
//             favorite: None,
//         };
//         assert!(update.validate().is_ok());
//     }
//
//     #[test]
//     fn test_clear_notes() {
//         let update = UpdateEntry {
//             title: None,
//             username: None,
//             password: None,
//             url: None,
//             notes: Some(None), // Setting notes to None
//             category_id: None,
//             favorite: None,
//         };
//         assert!(update.validate().is_ok());
//     }
//
//     #[test]
//     fn test_clear_category() {
//         let update = UpdateEntry {
//             title: None,
//             username: None,
//             password: None,
//             url: None,
//             notes: None,
//             category_id: Some(None), // Setting category_favorite to None
//             favorite: None,
//         };
//         assert!(update.validate().is_ok());
//     }
// }
//
// #[cfg(test)]
// mod update_entry_validation_tests {
//     use crate::entry::overview::UpdateEntry;
//     use crate::tests::entry::overview_tests::create_valid_update_entry;
//
//     #[test]
//     fn test_update_invalid_title() {
//         let update = UpdateEntry {
//             title: Some("".to_string()),
//             username: None,
//             password: None,
//             url: None,
//             notes: None,
//             category_id: None,
//             favorite: None,
//         };
//         assert!(update.validate().is_err());
//     }
//
//     #[test]
//     fn test_update_invalid_url() {
//         let update = UpdateEntry {
//             title: None,
//             username: None,
//             password: None,
//             url: Some(Some("not a valid url".to_string())),
//             notes: None,
//             category_id: None,
//             favorite: None,
//         };
//         assert!(update.validate().is_err());
//     }
//
//     #[test]
//     fn test_update_invalid_length_fields() {
//         // Test title length
//         let mut update = create_valid_update_entry();
//         update.title = Some("a".repeat(101));
//         assert!(update.validate().is_err());
//
//         // Test username length
//         let mut update = create_valid_update_entry();
//         update.username = Some(Some("a".repeat(101)));
//         assert!(update.validate().is_err());
//
//         // Test notes length
//         let mut update = create_valid_update_entry();
//         update.notes = Some(Some("a".repeat(10001)));
//         assert!(update.validate().is_err());
//     }
// }
//
// #[cfg(test)]
// mod entry_overview_tests {
//     use chrono::Utc;
//     use uuid::Uuid;
//     use super::*;
//     use crate::entry::overview::EntryOverview;
//
//
//     #[test]
//     fn test_overview_creation() {
//         let overview = create_valid_overview();
//         assert!(!overview.title.is_empty());
//         assert!(overview.username.is_some());
//         assert!(overview.url.is_some());
//     }
//
//     #[test]
//     fn test_overview_fields_accessible() {
//         let overview = create_valid_overview();
//         assert_eq!(overview.title, "Test Entry");
//         assert_eq!(overview.username, Some("testuser".to_string()));
//         assert_eq!(overview.url, Some("https://example.com".to_string()));
//         assert!(!overview.favorite);
//     }
//
//     #[test]
//     fn test_overview_with_all_optional_fields() {
//         let overview = create_valid_overview();
//         // All optional fields are set in create_valid_overview()
//         assert!(overview.username.is_some());
//         assert!(overview.url.is_some());
//         assert!(overview.category_id.is_some());
//     }
//
//     #[test]
//     fn test_overview_with_no_optional_fields() {
//         let overview = EntryOverview {
//             id: Uuid::new_v4(),
//             title: "Test Entry".to_string(),
//             username: None,
//             url: None,
//             category_id: None,
//             favorite: false,
//             updated_at: Utc::now(),
//         };
//         assert!(overview.username.is_none());
//         assert!(overview.url.is_none());
//         assert!(overview.category_id.is_none());
//     }
// }
//
// #[cfg(test)]
// mod integration_tests {
//     use crate::entry::entry_management::UpdateEntry;
//     use super::*;
//
//     #[test]
//     fn test_update_preserves_unchanged_fields() {
//         let original = create_valid_new_entry();
//         let update = UpdateEntry {
//             title: Some("New Title".to_string()),
//             username: None, // Not updating username
//             password: None, // Not updating password
//             url: None,     // Not updating url
//             notes: None,   // Not updating notes
//             category_id: None, // Not updating category_favorite
//             favorite: Some(true),
//         };
//
//         assert!(update.validate().is_ok());
//         // The fields we didn't update should remain unchanged when applied
//     }
//
//     #[test]
//     fn test_valid_characters_in_fields() {
//         let mut entry = create_valid_new_entry();
//
//         // Test special characters in title
//         entry.title = "Test Entry !@#$%^&*()_+-=[]{}|;:,.<>?".to_string();
//         assert!(entry.validate().is_ok());
//
//         // Test Unicode characters
//         entry.title = "Test Entry 测试条目 テストエントリ".to_string();
//         assert!(entry.validate().is_ok());
//
//         // Test emojis
//         entry.title = "Test Entry 📝🔐🎉".to_string();
//         assert!(entry.validate().is_ok());
//     }
//
//     #[test]
//     fn test_multiple_sequential_updates() {
//         // Initial update
//         let update1 = UpdateEntry {
//             title: Some("First Update".to_string()),
//             username: None,
//             password: None,
//             url: None,
//             notes: None,
//             category_id: None,
//             favorite: None,
//         };
//         assert!(update1.validate().is_ok());
//
//         // Second update
//         let update2 = UpdateEntry {
//             title: None,
//             username: Some(Some("second_update".to_string())),
//             password: None,
//             url: None,
//             notes: None,
//             category_id: None,
//             favorite: None,
//         };
//         assert!(update2.validate().is_ok());
//
//         // Third update
//         let update3 = UpdateEntry {
//             title: None,
//             username: None,
//             password: Some("third_update".to_string()),
//             url: None,
//             notes: None,
//             category_id: None,
//             favorite: Some(true),
//         };
//         assert!(update3.validate().is_ok());
//     }
// }
