// // tests/entry_collection_tests.rs

// mod tests {
//     use std::collections::HashSet;
//     use uuid::Uuid;
//     use crate::entry::collection::{EntryCollection, SearchQuery};
//     use crate::entry::overview::EntryData;
//     use crate::crypto::KeyHierarchy;
//     use crate::entry::{MAX_TITLE_LENGTH, MAX_USERNAME_LENGTH};
//     use crate::error::entry_error::EntryError;

//     // Helper function to create default EntryData
//     fn create_entry_data(title: &str) -> EntryData {
//         EntryData {
//             title: title.to_string(),
//             username: Some("user@example.com".to_string()),
//             url: Some("https://example.com".to_string()),
//             category_id: None,
//             favorite: false,
//             password: Some("password123".to_string()),
//             notes: Some("Some notes".to_string()),
//         }
//     }

//     // Helper function to create a KeyHierarchy for testing
//     fn create_key_hierarchy() -> KeyHierarchy {
//         let master_password = b"master_password";
//         let (key_hierarchy, _salt) = KeyHierarchy::new(master_password)
//             .expect("Failed to create KeyHierarchy");
//         key_hierarchy
//     }
//     #[test]
//     fn test_create_entry_with_all_fields() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let entry_data = create_entry_data("Test Entry");

//         let result = collection.create_entry(entry_data, &key_hierarchy);

//         assert!(result.is_ok(), "Entry creation failed");
//         let entry_id = result.unwrap();
//         assert!(
//             collection.entries.contains_key(&entry_id),
//             "Entry not stored"
//         );
//     }

//     #[test]
//     fn test_create_entry_with_optional_fields_omitted() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let entry_data = EntryData {
//             title: "No Optional Fields".to_string(),
//             username: None,
//             url: None,
//             category_id: None,
//             favorite: false,
//             password: Some("password123".to_string()),
//             notes: None,
//         };

//         let result = collection.create_entry(entry_data, &key_hierarchy);

//         assert!(result.is_ok(), "Entry creation failed");
//         let entry_id = result.unwrap();
//         assert!(
//             collection.entries.contains_key(&entry_id),
//             "Entry not stored"
//         );
//     }

//     #[test]
//     fn test_create_entry_with_empty_title() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let entry_data = create_entry_data("");

//         let result = collection.create_entry(entry_data, &key_hierarchy);

//         assert!(
//             matches!(result, Err(EntryError::InvalidTitle)),
//             "Expected InvalidTitle error"
//         );
//     }

//     #[test]
//     fn test_update_existing_entry_basic_fields() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let mut entry_data = create_entry_data("Original Title");

//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         // Prepare changes
//         entry_data.title = "Updated Title".to_string();
//         entry_data.username = Some("newuser@example.com".to_string());
//         entry_data.url = Some("https://newexample.com".to_string());

//         let result = collection.update_entry(&entry_id, entry_data, &key_hierarchy);

//         assert!(result.is_ok(), "Entry update failed");

//         let updated_entry = collection.entries.get(&entry_id).unwrap();
//         assert_eq!(updated_entry.title, "Updated Title");
//         assert_eq!(
//             updated_entry.username,
//             Some("newuser@example.com".to_string())
//         );
//         assert_eq!(
//             updated_entry.url,
//             Some("https://newexample.com".to_string())
//         );
//     }

//     #[test]
//     fn test_update_entry_category_change() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let mut entry_data = create_entry_data("Entry with Category");

//         // Create a category
//         let category_id = collection
//             .categories
//             .create_category("Test Category".to_string())
//             .expect("Failed to create category");
//         entry_data.category_id = Some(category_id);

//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         // Verify entry is in the category
//         let category_entries = collection
//             .categories
//             .get_entries_in_category(&category_id)
//             .expect("Failed to get category entries");
//         assert!(category_entries.contains(&entry_id));

//         // Create a new category and update the entry to use it
//         let new_category_id = collection
//             .categories
//             .create_category("New Category".to_string())
//             .expect("Failed to create new category");
//         entry_data.category_id = Some(new_category_id);

//         let result = collection.update_entry(&entry_id, entry_data, &key_hierarchy);
//         assert!(result.is_ok(), "Entry update failed");

//         // Verify entry is moved to new category
//         let old_category_entries = collection
//             .categories
//             .get_entries_in_category(&category_id)
//             .expect("Failed to get old category entries");
//         assert!(!old_category_entries.contains(&entry_id));

//         let new_category_entries = collection
//             .categories
//             .get_entries_in_category(&new_category_id)
//             .expect("Failed to get new category entries");
//         assert!(new_category_entries.contains(&entry_id));
//     }

//     #[test]
//     fn test_update_entry_favorite_status() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let mut entry_data = create_entry_data("Favorite Entry");
//         entry_data.favorite = false;

//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         assert!(
//             !collection.favorites.is_favorite(&entry_id),
//             "Entry should not be favorite initially"
//         );

//         // Update favorite status
//         entry_data.favorite = true;
//         let result = collection.update_entry(&entry_id, entry_data, &key_hierarchy);
//         assert!(result.is_ok(), "Failed to update favorite status");

//         assert!(
//             collection.favorites.is_favorite(&entry_id),
//             "Entry should be favorite after update"
//         );
//     }

//     #[test]
//     fn test_update_entry_searchable_fields_indexes() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let mut entry_data = create_entry_data("Index Entry");

//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         // Verify indexes contain the original data
//         let title_key = entry_data.title.to_lowercase();
//         assert!(
//             collection
//                 .title_index
//                 .get(&title_key)
//                 .unwrap()
//                 .contains(&entry_id),
//             "Title index should contain entry"
//         );

//         // Update the title
//         entry_data.title = "Updated Index Entry".to_string();
//         let result = collection.update_entry(&entry_id, entry_data.clone(), &key_hierarchy);
//         assert!(result.is_ok(), "Failed to update entry");

//         // Verify indexes are updated
//         assert!(
//             !collection
//                 .title_index
//                 .get(&"index entry".to_string())
//                 .map_or(false, |set| set.contains(&entry_id)),
//             "Old title index should not contain entry"
//         );
//         assert!(
//             collection
//                 .title_index
//                 .get(&entry_data.title.to_lowercase())
//                 .unwrap()
//                 .contains(&entry_id),
//             "New title index should contain entry"
//         );
//     }

//     #[test]
//     fn test_update_nonexistent_entry() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let entry_data = create_entry_data("Nonexistent Entry");
//         let fake_id = Uuid::new_v4();

//         let result = collection.update_entry(&fake_id, entry_data, &key_hierarchy);

//         assert!(
//             matches!(result, Err(EntryError::NotFound(_))),
//             "Expected NotFound error"
//         );
//     }

//     #[test]
//     fn test_delete_existing_entry() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let entry_data = create_entry_data("Entry to Delete");

//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         assert!(collection.entries.contains_key(&entry_id), "Entry not found");

//         let result = collection.delete_entry(&entry_id);
//         assert!(result.is_ok(), "Failed to delete entry");

//         assert!(
//             !collection.entries.contains_key(&entry_id),
//             "Entry should be removed"
//         );
//     }

//     #[test]
//     fn test_delete_entry_removes_from_category() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let mut entry_data = create_entry_data("Entry in Category");

//         // Create a category and assign entry to it
//         let category_id = collection
//             .categories
//             .create_category("Category".to_string())
//             .expect("Failed to create category");
//         entry_data.category_id = Some(category_id);

//         let entry_id = collection
//             .create_entry(entry_data, &key_hierarchy)
//             .expect("Failed to create entry");

//         // Verify entry is in category
//         let category_entries = collection
//             .categories
//             .get_entries_in_category(&category_id)
//             .expect("Failed to get category entries");
//         assert!(category_entries.contains(&entry_id));

//         // Delete entry
//         let result = collection.delete_entry(&entry_id);
//         assert!(result.is_ok(), "Failed to delete entry");

//         // Verify entry is removed from category
//         let category_entries = collection
//             .categories
//             .get_entries_in_category(&category_id)
//             .expect("Failed to get category entries");
//         assert!(!category_entries.contains(&entry_id));
//     }

//     #[test]
//     fn test_delete_entry_removes_from_favorites() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let mut entry_data = create_entry_data("Favorite Entry");
//         entry_data.favorite = true;

//         let entry_id = collection
//             .create_entry(entry_data, &key_hierarchy)
//             .expect("Failed to create entry");

//         assert!(
//             collection.favorites.is_favorite(&entry_id),
//             "Entry should be favorite"
//         );

//         let result = collection.delete_entry(&entry_id);
//         assert!(result.is_ok(), "Failed to delete entry");

//         assert!(
//             !collection.favorites.is_favorite(&entry_id),
//             "Entry should be removed from favorites"
//         );
//     }

//     #[test]
//     fn test_delete_entry_removes_from_indexes() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let entry_data = create_entry_data("Indexed Entry");

//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         // Verify entry is in indexes
//         let title_key = entry_data.title.to_lowercase();
//         assert!(
//             collection
//                 .title_index
//                 .get(&title_key)
//                 .unwrap()
//                 .contains(&entry_id),
//             "Entry should be in title index"
//         );

//         // Delete entry
//         let result = collection.delete_entry(&entry_id);
//         assert!(result.is_ok(), "Failed to delete entry");

//         // Verify entry is removed from indexes
//         assert!(
//             !collection
//                 .title_index
//                 .get(&title_key)
//                 .map_or(false, |set| set.contains(&entry_id)),
//             "Entry should be removed from title index"
//         );
//     }

//     #[test]
//     fn test_delete_nonexistent_entry() {
//         let mut collection = EntryCollection::new();
//         let fake_id = Uuid::new_v4();

//         let result = collection.delete_entry(&fake_id);

//         assert!(
//             matches!(result, Err(EntryError::NotFound(_))),
//             "Expected NotFound error"
//         );
//     }

//     #[test]
//     fn test_get_decrypted_entry() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let entry_data = create_entry_data("Decrypt Entry");

//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         let decrypted_entry = collection
//             .get_decrypted_entry(&entry_id, &key_hierarchy)
//             .expect("Failed to decrypt entry");

//         let pass_from_secure;
//         unsafe {
//             pass_from_secure = decrypted_entry.password.ok_or(EntryError::InvalidPassword).unwrap().into_inner();
//         }

//         let notes_from_secure;
//         unsafe {
//             notes_from_secure = decrypted_entry.notes.ok_or(EntryError::NotFound(*decrypted_entry.id)).unwrap().into_inner();
//         }



//         assert_eq!(decrypted_entry.title, "Decrypt Entry");
//         assert_eq!(decrypted_entry.username, Some("user@example.com"));
//         assert_eq!(decrypted_entry.url, Some("https://example.com"));
//         assert_eq!(
//             pass_from_secure,
//             "password123".to_string()
//         );
//         assert_eq!(
//             notes_from_secure,
//             "Some notes".to_string()
//         );
//     }

//     #[test]
//     fn test_get_entry_overview() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let entry_data = create_entry_data("Overview Entry");

//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         let overview = collection
//             .get_entry_overview(&entry_id)
//             .expect("Failed to get entry overview");

//         assert_eq!(overview.title, "Overview Entry");
//         assert_eq!(overview.username, Some("user@example.com".to_string()));
//         assert_eq!(overview.url, Some("https://example.com".to_string()));
//         assert_eq!(overview.favorite, false);
//     }

//     #[test]
//     fn test_get_all_overviews() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create multiple entries
//         for i in 1..=5 {
//             let entry_data = create_entry_data(&format!("Entry {}", i));
//             collection
//                 .create_entry(entry_data, &key_hierarchy)
//                 .expect("Failed to create entry");
//         }

//         let overviews = collection.get_all_overviews();

//         assert_eq!(overviews.len(), 5, "Should have 5 entry overviews");
//     }

//     #[test]
//     fn test_get_nonexistent_entry() {
//         let collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let fake_id = Uuid::new_v4();

//         let result = collection.get_decrypted_entry(&fake_id, &key_hierarchy);

//         assert!(
//             matches!(result, Err(EntryError::NotFound(_))),
//             "Expected NotFound error"
//         );
//     }

//     // Section 5: Test Search Functionality

//     #[test]
//     fn test_search_by_title() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create entries
//         let entry_data1 = create_entry_data("First Entry");
//         let entry_data2 = create_entry_data("Second Entry");
//         let entry_data3 = create_entry_data("Another Entry");

//         collection
//             .create_entry(entry_data1.clone(), &key_hierarchy)
//             .expect("Failed to create entry");
//         collection
//             .create_entry(entry_data2.clone(), &key_hierarchy)
//             .expect("Failed to create entry");
//         collection
//             .create_entry(entry_data3.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         // Search for "First"
//         let query = SearchQuery::new(Some("First".to_string()), None, false);
//         let results = collection.search(query);

//         assert_eq!(results.len(), 1, "Should find one entry");
//         assert_eq!(results[0].title, "First Entry");
//     }

//     #[test]
//     fn test_search_by_username() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create entries with different usernames
//         let mut entry_data1 = create_entry_data("Entry One");
//         entry_data1.username = Some("user1@example.com".to_string());

//         let mut entry_data2 = create_entry_data("Entry Two");
//         entry_data2.username = Some("user2@example.com".to_string());

//         collection
//             .create_entry(entry_data1, &key_hierarchy)
//             .expect("Failed to create entry");
//         collection
//             .create_entry(entry_data2, &key_hierarchy)
//             .expect("Failed to create entry");

//         // Search for "user2"
//         let query = SearchQuery::new(Some("user2".to_string()), None, false);
//         let results = collection.search(query);

//         assert_eq!(results.len(), 1, "Should find one entry");
//         assert_eq!(results[0].username, Some("user2@example.com".to_string()));
//     }

//     #[test]
//     fn test_search_by_url() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create entries with different URLs
//         let mut entry_data1 = create_entry_data("Entry One");
//         entry_data1.url = Some("https://site1.com".to_string());

//         let mut entry_data2 = create_entry_data("Entry Two");
//         entry_data2.url = Some("https://site2.com".to_string());

//         collection
//             .create_entry(entry_data1, &key_hierarchy)
//             .expect("Failed to create entry");
//         collection
//             .create_entry(entry_data2, &key_hierarchy)
//             .expect("Failed to create entry");

//         // Search for "site2"
//         let query = SearchQuery::new(Some("site2".to_string()), None, false);
//         let results = collection.search(query);

//         assert_eq!(results.len(), 1, "Should find one entry");
//         assert_eq!(results[0].url, Some("https://site2.com".to_string()));
//     }

//     #[test]
//     fn test_search_by_category() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create a category
//         let category_id = collection
//             .categories
//             .create_category("Work".to_string())
//             .expect("Failed to create category");

//         // Create entries with and without the category
//         let mut entry_data1 = create_entry_data("Work Entry");
//         entry_data1.category_id = Some(category_id);

//         let entry_data2 = create_entry_data("Personal Entry");

//         collection
//             .create_entry(entry_data1, &key_hierarchy)
//             .expect("Failed to create entry");
//         collection
//             .create_entry(entry_data2, &key_hierarchy)
//             .expect("Failed to create entry");

//         // Search for entries in "Work" category
//         let query = SearchQuery::new(None, Some(category_id), false);
//         let results = collection.search(query);

//         assert_eq!(results.len(), 1, "Should find one entry in category");
//         assert_eq!(results[0].title, "Work Entry");
//     }

//     #[test]
//     fn test_search_by_favorite_status() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create entries with different favorite statuses
//         let mut entry_data1 = create_entry_data("Favorite Entry");
//         entry_data1.favorite = true;

//         let entry_data2 = create_entry_data("Regular Entry");

//         collection
//             .create_entry(entry_data1, &key_hierarchy)
//             .expect("Failed to create entry");
//         collection
//             .create_entry(entry_data2, &key_hierarchy)
//             .expect("Failed to create entry");

//         // Search for favorite entries
//         let query = SearchQuery::new(None, None, true);
//         let results = collection.search(query);

//         assert_eq!(results.len(), 1, "Should find one favorite entry");
//         assert_eq!(results[0].title, "Favorite Entry");
//     }

//     #[test]
//     fn test_search_combined_criteria() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create a category
//         let category_id = collection
//             .categories
//             .create_category("Work".to_string())
//             .expect("Failed to create category");

//         // Create entries
//         let mut entry_data1 = create_entry_data("Work Favorite Entry");
//         entry_data1.category_id = Some(category_id);
//         entry_data1.favorite = true;

//         let mut entry_data2 = create_entry_data("Work Regular Entry");
//         entry_data2.category_id = Some(category_id);

//         let mut entry_data3 = create_entry_data("Personal Favorite Entry");
//         entry_data3.favorite = true;

//         collection
//             .create_entry(entry_data1, &key_hierarchy)
//             .expect("Failed to create entry");
//         collection
//             .create_entry(entry_data2, &key_hierarchy)
//             .expect("Failed to create entry");
//         collection
//             .create_entry(entry_data3, &key_hierarchy)
//             .expect("Failed to create entry");

//         // Search for favorite entries in "Work" category with "Favorite" in title
//         let query = SearchQuery::new(
//             Some("Favorite".to_string()),
//             Some(category_id),
//             true,
//         );
//         let results = collection.search(query);

//         assert_eq!(results.len(), 1, "Should find one entry matching criteria");
//         assert_eq!(results[0].title, "Work Favorite Entry");
//     }

//     #[test]
//     fn test_search_text_matches_multiple_entries() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create entries with similar titles
//         let entry_data1 = create_entry_data("Common Entry One");
//         let entry_data2 = create_entry_data("Common Entry Two");
//         let entry_data3 = create_entry_data("Unique Entry");

//         collection
//             .create_entry(entry_data1, &key_hierarchy)
//             .expect("Failed to create entry");
//         collection
//             .create_entry(entry_data2, &key_hierarchy)
//             .expect("Failed to create entry");
//         collection
//             .create_entry(entry_data3, &key_hierarchy)
//             .expect("Failed to create entry");

//         // Search for "Common"
//         let query = SearchQuery::new(Some("Common".to_string()), None, false);
//         let results = collection.search(query);

//         assert_eq!(results.len(), 2, "Should find two entries");
//         let titles: HashSet<_> = results.iter().map(|e| e.title.clone()).collect();
//         assert!(titles.contains("Common Entry One"));
//         assert!(titles.contains("Common Entry Two"));
//     }

//     #[test]
//     fn test_search_no_matching_entries() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create entries
//         let entry_data1 = create_entry_data("Entry One");
//         let entry_data2 = create_entry_data("Entry Two");

//         collection
//             .create_entry(entry_data1, &key_hierarchy)
//             .expect("Failed to create entry");
//         collection
//             .create_entry(entry_data2, &key_hierarchy)
//             .expect("Failed to create entry");

//         // Search for "Nonexistent"
//         let query = SearchQuery::new(Some("Nonexistent".to_string()), None, false);
//         let results = collection.search(query);

//         assert_eq!(results.len(), 0, "Should find no entries");
//     }

//     #[test]
//     fn test_search_empty_collection() {
//         let collection = EntryCollection::new();

//         // Search in empty collection
//         let query = SearchQuery::new(Some("Anything".to_string()), None, false);
//         let results = collection.search(query);

//         assert_eq!(results.len(), 0, "Should find no entries in empty collection");
//     }

//     // Section 6: Test Index Maintenance

//     #[test]
//     fn test_indexes_after_adding_entries() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create entries
//         let entry_data1 = create_entry_data("Index Test Entry");
//         let entry_id1 = collection
//             .create_entry(entry_data1.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         // Verify indexes
//         let title_key = entry_data1.title.to_lowercase();
//         assert!(
//             collection
//                 .title_index
//                 .get(&title_key)
//                 .unwrap()
//                 .contains(&entry_id1),
//             "Title index should contain the entry"
//         );
//     }

//     #[test]
//     fn test_indexes_after_updating_entries() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create entry
//         let mut entry_data = create_entry_data("Original Title");
//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         // Update entry title
//         entry_data.title = "Updated Title".to_string();
//         let result = collection.update_entry(&entry_id, entry_data.clone(), &key_hierarchy);
//         assert!(result.is_ok(), "Failed to update entry");

//         // Verify old title index is updated
//         assert!(
//             !collection
//                 .title_index
//                 .get(&"original title".to_string())
//                 .map_or(false, |set| set.contains(&entry_id)),
//             "Old title index should not contain the entry"
//         );

//         // Verify new title index
//         assert!(
//             collection
//                 .title_index
//                 .get(&"updated title".to_string())
//                 .unwrap()
//                 .contains(&entry_id),
//             "New title index should contain the entry"
//         );
//     }

//     #[test]
//     fn test_indexes_after_deleting_entries() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create entry
//         let entry_data = create_entry_data("Entry to Delete");
//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         // Delete entry
//         collection
//             .delete_entry(&entry_id)
//             .expect("Failed to delete entry");

//         // Verify indexes
//         let title_key = entry_data.title.to_lowercase();
//         assert!(
//             !collection
//                 .title_index
//                 .get(&title_key)
//                 .map_or(false, |set| set.contains(&entry_id)),
//             "Entry should be removed from title index"
//         );
//     }

//     #[test]
//     fn test_duplicate_keys_in_indexes() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create entries with same title
//         let entry_data1 = create_entry_data("Duplicate Title");
//         let entry_data2 = create_entry_data("Duplicate Title");

//         let entry_id1 = collection
//             .create_entry(entry_data1, &key_hierarchy)
//             .expect("Failed to create entry");
//         let entry_id2 = collection
//             .create_entry(entry_data2, &key_hierarchy)
//             .expect("Failed to create entry");

//         // Verify both entries are in the index
//         let title_key = "duplicate title".to_string();
//         let ids = collection
//             .title_index
//             .get(&title_key)
//             .expect("Title key should exist");
//         assert!(ids.contains(&entry_id1));
//         assert!(ids.contains(&entry_id2));
//     }

//     // Section 7: Test CategoryCollection Interaction

//     #[test]
//     fn test_add_entry_to_category() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create category
//         let category_id = collection
//             .categories
//             .create_category("Work".to_string())
//             .expect("Failed to create category");

//         // Create entry assigned to category
//         let mut entry_data = create_entry_data("Work Entry");
//         entry_data.category_id = Some(category_id);

//         let entry_id = collection
//             .create_entry(entry_data, &key_hierarchy)
//             .expect("Failed to create entry");

//         // Verify entry is in category
//         let entries_in_category = collection
//             .categories
//             .get_entries_in_category(&category_id)
//             .expect("Failed to get entries in category");
//         assert!(entries_in_category.contains(&entry_id));

//         // Verify category entry count
//         let category = collection
//             .categories
//             .get_category(&category_id)
//             .expect("Failed to get category");
//         assert_eq!(category.entry_count(), 1, "Category should have one entry");
//     }

//     #[test]
//     fn test_move_entry_between_categories() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create categories
//         let category_id1 = collection
//             .categories
//             .create_category("Work".to_string())
//             .expect("Failed to create category");
//         let category_id2 = collection
//             .categories
//             .create_category("Personal".to_string())
//             .expect("Failed to create category");

//         // Create entry in first category
//         let mut entry_data = create_entry_data("Movable Entry");
//         entry_data.category_id = Some(category_id1);

//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         // Move entry to second category
//         entry_data.category_id = Some(category_id2);
//         collection
//             .update_entry(&entry_id, entry_data, &key_hierarchy)
//             .expect("Failed to update entry");

//         // Verify entry is in new category
//         let entries_in_new_category = collection
//             .categories
//             .get_entries_in_category(&category_id2)
//             .expect("Failed to get entries in new category");
//         assert!(entries_in_new_category.contains(&entry_id));

//         // Verify entry is removed from old category
//         let entries_in_old_category = collection
//             .categories
//             .get_entries_in_category(&category_id1)
//             .expect("Failed to get entries in old category");
//         assert!(!entries_in_old_category.contains(&entry_id));
//     }

//     #[test]
//     fn test_remove_entry_from_category_on_deletion() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create category and entry
//         let category_id = collection
//             .categories
//             .create_category("Work".to_string())
//             .expect("Failed to create category");

//         let mut entry_data = create_entry_data("Deletable Entry");
//         entry_data.category_id = Some(category_id);

//         let entry_id = collection
//             .create_entry(entry_data, &key_hierarchy)
//             .expect("Failed to create entry");

//         // Delete entry
//         collection
//             .delete_entry(&entry_id)
//             .expect("Failed to delete entry");

//         // Verify entry is removed from category
//         let entries_in_category = collection
//             .categories
//             .get_entries_in_category(&category_id)
//             .expect("Failed to get entries in category");
//         assert!(!entries_in_category.contains(&entry_id));

//         // Verify category entry count
//         let category = collection
//             .categories
//             .get_category(&category_id)
//             .expect("Failed to get category");
//         assert_eq!(category.entry_count(), 0, "Category should have zero entries");
//     }

//     #[test]
//     fn test_category_entry_counts() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create category
//         let category_id = collection
//             .categories
//             .create_category("Projects".to_string())
//             .expect("Failed to create category");

//         // Create multiple entries in category
//         for i in 1..=3 {
//             let mut entry_data = create_entry_data(&format!("Project Entry {}", i));
//             entry_data.category_id = Some(category_id);

//             collection
//                 .create_entry(entry_data, &key_hierarchy)
//                 .expect("Failed to create entry");
//         }

//         // Verify category entry count
//         let category = collection
//             .categories
//             .get_category(&category_id)
//             .expect("Failed to get category");
//         assert_eq!(category.entry_count(), 3, "Category should have three entries");
//     }

//     // Section 8: Test FavoriteCollection Interaction

//     #[test]
//     fn test_mark_entry_as_favorite() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create entry
//         let mut entry_data = create_entry_data("Favorite Entry");
//         entry_data.favorite = false;

//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         // Mark as favorite
//         entry_data.favorite = true;
//         collection
//             .update_entry(&entry_id, entry_data, &key_hierarchy)
//             .expect("Failed to update entry");

//         // Verify entry is in favorites
//         assert!(
//             collection.favorites.is_favorite(&entry_id),
//             "Entry should be marked as favorite"
//         );
//     }

//     #[test]
//     fn test_unmark_entry_as_favorite() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create entry marked as favorite
//         let mut entry_data = create_entry_data("Un-favorite Entry");
//         entry_data.favorite = true;

//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         // Unmark as favorite
//         entry_data.favorite = false;
//         collection
//             .update_entry(&entry_id, entry_data, &key_hierarchy)
//             .expect("Failed to update entry");

//         // Verify entry is not in favorites
//         assert!(
//             !collection.favorites.is_favorite(&entry_id),
//             "Entry should not be marked as favorite"
//         );
//     }

//     #[test]
//     fn test_favorite_status_after_deleting_entry() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create favorite entry
//         let mut entry_data = create_entry_data("Deletable Favorite");
//         entry_data.favorite = true;

//         let entry_id = collection
//             .create_entry(entry_data, &key_hierarchy)
//             .expect("Failed to create entry");

//         // Delete entry
//         collection
//             .delete_entry(&entry_id)
//             .expect("Failed to delete entry");

//         // Verify entry is removed from favorites
//         assert!(
//             !collection.favorites.is_favorite(&entry_id),
//             "Entry should be removed from favorites after deletion"
//         );
//     }

//     #[test]
//     fn test_retrieving_all_favorite_entries() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create entries with different favorite statuses
//         for i in 1..=5 {
//             let mut entry_data = create_entry_data(&format!("Entry {}", i));
//             entry_data.favorite = i % 2 == 0; // Mark even entries as favorite

//             collection
//                 .create_entry(entry_data, &key_hierarchy)
//                 .expect("Failed to create entry");
//         }

//         // Retrieve favorite entries
//         let favorite_ids = collection.favorites.get_all_favorites();

//         assert_eq!(favorite_ids.len(), 2, "Should have two favorite entries");

//         // Verify favorite entries
//         for entry in collection.entries.values() {
//             let is_favorite = collection.favorites.is_favorite(&entry.id);
//             if is_favorite {
//                 assert!(entry.favorite, "Entry's favorite flag should be true");
//             } else {
//                 assert!(!entry.favorite, "Entry's favorite flag should be false");
//             }
//         }
//     }

//     #[test]
//     fn test_update_entry_with_empty_optional_fields() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();
//         let mut entry_data = create_entry_data("Entry with Optional Fields");

//         // Create entry with all fields
//         let entry_id = collection
//             .create_entry(entry_data.clone(), &key_hierarchy)
//             .expect("Failed to create entry");

//         // Update entry to have empty optional fields
//         entry_data.username = None;
//         entry_data.url = None;

//         let result = collection.update_entry(&entry_id, entry_data.clone(), &key_hierarchy);
//         assert!(result.is_ok(), "Entry update failed");

//         // Verify the entry has empty optional fields
//         let updated_entry = collection.entries.get(&entry_id).unwrap();
//         assert_eq!(updated_entry.username, None, "Username should be None");
//         assert_eq!(updated_entry.url, None, "URL should be None");

//         // Verify indexes are updated
//         assert!(
//             !collection
//                 .username_index
//                 .get(&"user@example.com".to_string())
//                 .map_or(false, |set| set.contains(&entry_id)),
//             "Old username index should not contain entry"
//         );
//         assert!(
//             !collection
//                 .url_index
//                 .get(&"https://example.com".to_string())
//                 .map_or(false, |set| set.contains(&entry_id)),
//             "Old URL index should not contain entry"
//         );
//     }

//     #[test]
//     fn test_entry_with_extremely_long_strings() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create extremely long strings
//         let long_title = "a".repeat(MAX_TITLE_LENGTH + 1);
//         let long_username = "b".repeat(MAX_USERNAME_LENGTH + 1);
//         let long_url = format!("https://{}", "c".repeat(MAX_USERNAME_LENGTH + 1));

//         // Create entry data with long strings
//         let entry_data = EntryData {
//             title: long_title.clone(),
//             username: Some(long_username.clone()),
//             url: Some(long_url.clone()),
//             category_id: None,
//             favorite: false,
//             password: Some("password123".to_string()),
//             notes: Some("Some notes".to_string()),
//         };

//         // Attempt to create entry
//         let result = collection.create_entry(entry_data.clone(), &key_hierarchy);
//         assert!(!result.is_ok(), "Entry creation should fail with long strings");

//         // Verify the error type
//         if let Err(EntryError::InvalidLength(field)) = result {
//             assert!(field == "Title" || field == "Username" || field == "URL", "Unexpected field with invalid length");
//         } else {
//             panic!("Expected InvalidLength error");
//         }
//     }

//     #[test]
//     fn test_search_with_empty_or_null_criteria() {
//         let mut collection = EntryCollection::new();
//         let key_hierarchy = create_key_hierarchy();

//         // Create multiple entries
//         for i in 1..=3 {
//             let entry_data = create_entry_data(&format!("Entry {}", i));
//             collection
//                 .create_entry(entry_data, &key_hierarchy)
//                 .expect("Failed to create entry");
//         }

//         // Search with empty string criteria
//         let query_empty_string = SearchQuery::new(Some("".to_string()), None, false);
//         let results_empty_string = collection.search(query_empty_string);
//         assert_eq!(results_empty_string.len(), 3, "Should return all entries for empty string criteria");

//         // Search with null criteria
//         let query_null = SearchQuery::new(None, None, false);
//         let results_null = collection.search(query_null);
//         assert_eq!(results_null.len(), 3, "Should return all entries for null criteria");
//     }
// }