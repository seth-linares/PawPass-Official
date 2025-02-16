// use std::collections::HashSet;

// #[cfg(test)]
// mod password_generation_tests {
//     use std::collections::HashMap;
//     use crate::_vault::DEFAULT_LENGTH;
//     use crate::_vault::password_generation::PasswordGenerator;
//     use super::*;

//     // Helper function to analyze a generated password
//     fn analyze_password(password: &str) -> (usize, bool, bool, bool, bool) {
//         let length = password.len();
//         let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
//         let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
//         let has_number = password.chars().any(|c| c.is_ascii_digit());
//         let has_symbol = password.chars().any(|c| !c.is_ascii_alphanumeric());

//         (length, has_lower, has_upper, has_number, has_symbol)
//     }

//     #[test]
//     /// Verifies that default settings produce valid passwords
//     fn test_default_password_generation() {
//         let generator = PasswordGenerator::default();
//         let password = generator.generate().expect("Failed to generate password");

//         let (length, has_lower, has_upper, has_number, has_symbol) = analyze_password(&password);

//         assert_eq!(length, DEFAULT_LENGTH, "Password should be default length");
//         assert!(has_lower, "Should contain lowercase letters");
//         assert!(has_upper, "Should contain uppercase letters");
//         assert!(has_number, "Should contain numbers");
//         assert!(has_symbol, "Should contain symbols");
//     }

//     #[test]
//     /// Verifies that minimum character requirements are met
//     fn test_minimum_requirements() {
//         let generator = PasswordGenerator::default();
//         let password = generator.generate().expect("Failed to generate password");

//         // Count numbers and symbols
//         let number_count = password.chars().filter(|c| c.is_ascii_digit()).count();
//         let symbol_count = password.chars().filter(|c| !c.is_ascii_alphanumeric()).count();

//         assert!(number_count >= generator.min_numbers,
//                 "Should contain at least {} numbers, found {}", generator.min_numbers, number_count);
//         assert!(symbol_count >= generator.min_symbols,
//                 "Should contain at least {} symbols, found {}", generator.min_symbols, symbol_count);
//     }

//     #[test]
//     /// Verifies that ambiguous characters are properly handled
//     fn test_ambiguous_character_exclusion() {
//         let generator = PasswordGenerator::default();
//         let password = generator.generate().expect("Failed to generate password");

//         println!("{}", password);

//         // Check each ambiguous group
//         for group in &generator.available_chars.ambiguous {
//             let unique_matches: HashSet<char> = password.chars()
//                 .filter(|c| group.contains(c))
//                 .collect();

//             assert!(unique_matches.len() <= 1,
//                     "Should not contain multiple characters from ambiguous group: {:?}", unique_matches);
//         }
//     }

//     #[test]
//     /// Tests randomness by generating multiple passwords and checking uniqueness
//     fn test_password_randomness() {
//         let generator = PasswordGenerator::default();
//         let mut passwords = HashSet::new();

//         // Generate 100 passwords and ensure they're all different
//         for _ in 0..100 {
//             let password = generator.generate().expect("Failed to generate password");
//             assert!(passwords.insert(password), "Generated duplicate password");
//         }
//     }

//     #[test]
//     /// Verifies that generated passwords maintain proper character distribution
//     fn test_character_distribution() {
//         let generator = PasswordGenerator::default();
//         let password = generator.generate().expect("Failed to generate password");

//         // No character should appear more than 30% of the time
//         let char_counts: HashMap<char, usize> = password.chars()
//             .fold(HashMap::new(), |mut map, c| {
//                 *map.entry(c).or_insert(0) += 1;
//                 map
//             });

//         let max_count = (password.len() as f64 * 0.3).ceil() as usize;
//         for (c, count) in char_counts {
//             assert!(count <= max_count,
//                     "Character '{}' appears too frequently: {} times", c, count);
//         }
//     }
// }

// #[cfg(test)]
// mod configuration_tests {
//     use crate::_vault::ABSOLUTE_MIN_LENGTH;
//     use crate::_vault::password_generation::PasswordGenerator;

//     #[test]
//     /// Tests that configuration changes are properly applied
//     fn test_basic_configuration() {
//         let mut generator = PasswordGenerator::default();

//         generator.configure(
//             20,             // length
//             true,          // lowercase
//             true,          // uppercase
//             false,         // numbers
//             true,          // symbols
//             0,             // min_numbers
//             2,             // min_symbols
//             true,          // exclude_ambiguous
//         ).expect("Failed to configure generator");

//         let password = generator.generate().expect("Failed to generate password");

//         assert_eq!(password.len(), 20, "Password length should match configuration");
//         assert!(!password.chars().any(|c| c.is_ascii_digit()),
//                 "Password should not contain numbers when disabled");
//     }

//     #[test]
//     /// Verifies that invalid configurations are rejected
//     fn test_invalid_configurations() {
//         let mut generator = PasswordGenerator::default();

//         // Test length below minimum
//         assert!(generator.configure(
//             2,              // length too short
//             true, true, true, true,
//             0, 0, true
//         ).is_err(), "Should reject length below minimum");

//         // Test length above maximum
//         assert!(generator.configure(
//             1000,           // length too long
//             true, true, true, true,
//             0, 0, true
//         ).is_err(), "Should reject length above maximum");

//         // Test no character types enabled
//         assert!(generator.configure(
//             16,             // valid length
//             false, false, false, false,  // no character types
//             0, 0, true
//         ).is_err(), "Should reject configuration with no character types");
//     }

//     #[test]
//     /// Tests edge cases in configuration
//     fn test_configuration_edge_cases() {
//         let mut generator = PasswordGenerator::default();

//         // Test minimum length with maximum requirements
//         let min_length = ABSOLUTE_MIN_LENGTH;
//         let result = generator.configure(
//             min_length,
//             true, true, true, true,
//             1, 1, true  // Minimum requirements that should fit
//         );
//         assert!(result.is_ok(), "Should accept valid minimum configuration");

//         // Test configuration with exactly fitting requirements
//         let length = 10;
//         let result = generator.configure(
//             length,
//             true, true, true, true,
//             3, 3, true  // Requirements that exactly fit the length
//         );
//         assert!(result.is_ok(), "Should accept exactly fitting requirements");
//     }

//     #[test]
//     /// Verifies that configuration affects password generation correctly
//     fn test_configuration_effects() {
//         let mut generator = PasswordGenerator::default();

//         // Configure for a specific case
//         generator.configure(
//             15,            // length
//             true,         // lowercase
//             true,         // uppercase
//             true,         // numbers
//             false,        // no symbols
//             2,            // min_numbers
//             0,            // min_symbols
//             false,        // don't exclude ambiguous
//         ).expect("Failed to configure generator");

//         let password = generator.generate().expect("Failed to generate password");

//         assert_eq!(password.len(), 15, "Length should match configuration");
//         assert!(!password.chars().any(|c| !c.is_ascii_alphanumeric()),
//                 "Should not contain symbols when disabled");

//         let number_count = password.chars().filter(|c| c.is_ascii_digit()).count();
//         assert!(number_count >= 2,
//                 "Should contain at least 2 numbers, found {}", number_count);
//     }
// }

// #[cfg(test)]
// mod character_set_tests {
//     use crate::_vault::password_generation::PasswordGenerator;

//     /// Helper function to verify if a string contains any characters from a set
//     fn contains_any_from_set(password: &str, char_set: &[char]) -> bool {
//         password.chars().any(|c| char_set.contains(&c))
//     }

//     /// Helper function to verify if a string contains all specified character types
//     fn verify_character_types(
//         password: &str,
//         expect_lower: bool,
//         expect_upper: bool,
//         expect_numbers: bool,
//         expect_symbols: bool,
//     ) -> bool {
//         let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
//         let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
//         let has_numbers = password.chars().any(|c| c.is_ascii_digit());
//         let has_symbols = password.chars().any(|c| !c.is_ascii_alphanumeric());

//         (has_lower == expect_lower) &&
//             (has_upper == expect_upper) &&
//             (has_numbers == expect_numbers) &&
//             (has_symbols == expect_symbols)
//     }

//     #[test]
//     /// Verifies that all required character types appear in generated passwords
//     fn test_required_character_types() {
//         let mut generator = PasswordGenerator::default();

//         // Test all combinations of character types
//         let test_cases = [
//             // (lowercase, uppercase, numbers, symbols)
//             (true, true, true, true),    // All enabled
//             (true, false, false, false), // Only lowercase
//             (false, true, false, false), // Only uppercase
//             (false, false, true, false), // Only numbers
//             (false, false, false, true), // Only symbols
//             (true, true, false, false),  // Letters only
//             (false, false, true, true),  // Numbers and symbols
//         ];

//         for (use_lower, use_upper, use_numbers, use_symbols) in test_cases {
//             // Configure generator for this test case
//             generator.configure(
//                 16,                 // reasonable length
//                 use_lower,
//                 use_upper,
//                 use_numbers,
//                 use_symbols,
//                 if use_numbers { 1 } else { 0 }, // min numbers
//                 if use_symbols { 1 } else { 0 }, // min symbols
//                 false,              // don't exclude ambiguous for this test
//             ).expect("Failed to configure generator");

//             // Generate and verify password
//             let password = generator.generate().expect("Failed to generate password");

//             assert!(verify_character_types(&password, use_lower, use_upper, use_numbers, use_symbols),
//                     "Password '{}' does not match expected character types: lower={}, upper={}, numbers={}, symbols={}",
//                     password, use_lower, use_upper, use_numbers, use_symbols);
//         }
//     }

//     #[test]
//     /// Verifies that excluded character types never appear in generated passwords
//     fn test_excluded_character_types() {
//         let mut generator = PasswordGenerator::default();

//         // Configure generator to exclude certain character types
//         generator.configure(
//             16,             // length
//             true,          // only use lowercase
//             false,         // no uppercase
//             false,         // no numbers
//             false,         // no symbols
//             0,             // min numbers
//             0,             // min symbols
//             false,         // don't exclude ambiguous
//         ).expect("Failed to configure generator");

//         // Generate multiple passwords to ensure consistency
//         for _ in 0..10 {
//             let password = generator.generate().expect("Failed to generate password");

//             // Verify no excluded character types appear
//             assert!(!password.chars().any(|c| c.is_ascii_uppercase()),
//                     "Password contains uppercase letters when excluded: {}", password);
//             assert!(!password.chars().any(|c| c.is_ascii_digit()),
//                     "Password contains numbers when excluded: {}", password);
//             assert!(!password.chars().any(|c| !c.is_ascii_alphanumeric()),
//                     "Password contains symbols when excluded: {}", password);
//         }
//     }

//     #[test]
//     /// Tests that special characters are handled properly
//     fn test_special_character_handling() {
//         let mut generator = PasswordGenerator::default();

//         // Configure for special characters test
//         generator.configure(
//             20,            // longer length to ensure good distribution
//             false,         // no lowercase
//             false,         // no uppercase
//             false,         // no numbers
//             true,         // only symbols
//             0,            // min numbers
//             5,            // require several symbols
//             false,        // don't exclude ambiguous
//         ).expect("Failed to configure generator");

//         let password = generator.generate().expect("Failed to generate password");

//         // Verify all characters are from the symbol set
//         assert!(password.chars().all(|c| generator.available_chars.symbols.contains(&c)),
//                 "Password contains non-symbol characters: {}", password);

//         // Verify minimum symbol requirement is met
//         let symbol_count = password.chars().count();
//         assert!(symbol_count >= 5,
//                 "Password doesn't contain enough symbols. Required: 5, Found: {}", symbol_count);
//     }

//     #[test]
//     /// Tests comprehensive ambiguous character handling
//     fn test_comprehensive_ambiguous_handling() {
//         let mut generator = PasswordGenerator::default();

//         // Enable ambiguous character exclusion
//         generator.configure(
//             30,            // longer length to increase character variety
//             true, true, true, true,  // enable all character types
//             1, 1,          // minimum requirements
//             true,          // exclude ambiguous
//         ).expect("Failed to configure generator");

//         // Generate and test multiple passwords
//         for _ in 0..10 {
//             let password = generator.generate().expect("Failed to generate password");

//             // Test each ambiguous group
//             for group in &generator.available_chars.ambiguous {
//                 // Count characters from this group in the password
//                 let matching_chars: Vec<char> = password.chars()
//                     .filter(|c| group.contains(c))
//                     .collect();

//                 // Should contain at most one character from each group
//                 assert!(matching_chars.len() <= 1,
//                         "Password '{}' contains multiple characters {:?} from ambiguous group {:?}",
//                         password, matching_chars, group);

//                 // If we found a character from this group, verify it's actually in our character sets
//                 if let Some(&c) = matching_chars.first() {
//                     let is_in_allowed_sets =
//                         generator.available_chars.lowercase.contains(&c) ||
//                             generator.available_chars.uppercase.contains(&c) ||
//                             generator.available_chars.numbers.contains(&c) ||
//                             generator.available_chars.symbols.contains(&c);

//                     assert!(is_in_allowed_sets,
//                             "Ambiguous character '{}' found but not present in any allowed character set", c);
//                 }
//             }
//         }
//     }
// }

// #[cfg(test)]
// mod entropy_tests {
//     use crate::_vault::password_generation::PasswordGenerator;
//     use super::*;

//     #[test]
//     /// Tests entropy calculation with all character types enabled
//     fn test_full_charset_entropy() {
//         let mut generator = PasswordGenerator::default();

//         // Configure for maximum entropy: all character types, no ambiguous exclusion
//         generator.configure(
//             16,             // length
//             true,          // lowercase
//             true,          // uppercase
//             true,          // numbers
//             true,          // symbols
//             1,             // min numbers
//             1,             // min symbols
//             false,         // don't exclude ambiguous
//         ).expect("Failed to configure generator");

//         let entropy = generator.calculate_entropy();

//         // Expected charset size:
//         // 26 (lowercase) + 26 (uppercase) + 10 (numbers) + 8 (symbols) = 70 characters
//         // Entropy = 16 * log2(70) ≈ 98.4 bits
//         let expected_entropy = 16.0 * 70.0_f64.log2();
//         assert!((entropy - expected_entropy).abs() < 0.001,
//                 "Entropy calculation incorrect. Expected ≈{:.2}, got {:.2}",
//                 expected_entropy, entropy);
//     }

//     #[test]
//     /// Tests entropy calculation with minimal character set
//     fn test_minimal_charset_entropy() {
//         let mut generator = PasswordGenerator::default();

//         // Configure for minimal entropy: only lowercase letters
//         generator.configure(
//             12,             // length
//             true,          // only lowercase
//             false,         // no uppercase
//             false,         // no numbers
//             false,         // no symbols
//             0,             // min numbers
//             0,             // min symbols
//             false,         // don't exclude ambiguous
//         ).expect("Failed to configure generator");

//         let entropy = generator.calculate_entropy();

//         // Expected charset size: 26 (lowercase only)
//         // Entropy = 12 * log2(26) ≈ 56.4 bits
//         let expected_entropy = 12.0 * 26.0_f64.log2();
//         assert!((entropy - expected_entropy).abs() < 0.001,
//                 "Entropy calculation incorrect. Expected ≈{:.2}, got {:.2}",
//                 expected_entropy, entropy);
//     }

//     #[test]
//     /// Tests entropy calculation with ambiguous characters excluded
//     fn test_entropy_with_ambiguous_excluded() {
//         let mut generator = PasswordGenerator::default();

//         // Configure with all characters but exclude ambiguous ones
//         generator.configure(
//             16,             // length
//             true,          // lowercase
//             true,          // uppercase
//             true,          // numbers
//             true,          // symbols
//             1,             // min numbers
//             1,             // min symbols
//             true,          // exclude ambiguous
//         ).expect("Failed to configure generator");

//         let entropy = generator.calculate_entropy();

//         // Calculate expected charset size:
//         // Start with: 26 + 26 + 10 + 8 = 70 characters
//         // Subtract (group.len() - 1) for each ambiguous group:
//         // ['1', 'l', 'I'] -> subtract 2
//         // ['o', 'O', '0'] -> subtract 2
//         // ['5', 'S'] -> subtract 1
//         // ['2', 'Z'] -> subtract 1
//         // ['8', 'B'] -> subtract 1
//         // Total reduction: 7 characters
//         // Final charset size: 63 characters
//         let expected_entropy = 16.0 * 63.0_f64.log2();
//         assert!((entropy - expected_entropy).abs() < 0.001,
//                 "Entropy calculation with ambiguous exclusion incorrect. Expected ≈{:.2}, got {:.2}",
//                 expected_entropy, entropy);
//     }

//     #[test]
//     /// Tests entropy calculation with different password lengths
//     fn test_entropy_length_scaling() {
//         let mut generator = PasswordGenerator::default();
//         generator.configure(
//             8,              // start with length 8
//             true, true, true, true,  // all character types
//             1, 1, false,    // basic requirements, no ambiguous exclusion
//         ).expect("Failed to configure generator");

//         let entropy_8 = generator.calculate_entropy();

//         // Double the length
//         generator.configure(
//             16,             // double length to 16
//             true, true, true, true,
//             1, 1, false,
//         ).expect("Failed to configure generator");

//         let entropy_16 = generator.calculate_entropy();

//         // Entropy should exactly double when length doubles
//         // (because entropy = length * log2(charset_size))
//         assert!((entropy_16 - (2.0 * entropy_8)).abs() < 0.001,
//                 "Entropy did not scale correctly with length. \
//              Length 8: {:.2}, Length 16: {:.2}, \
//              Expected doubling: {:.2}",
//                 entropy_8, entropy_16, 2.0 * entropy_8);
//     }

//     #[test]
//     /// Tests that entropy properly reflects character set sizes
//     fn test_entropy_charset_comparison() {
//         let mut generator = PasswordGenerator::default();

//         // Configure for letters only
//         generator.configure(
//             12,
//             true, true, false, false,  // only letters
//             0, 0, false,
//         ).expect("Failed to configure generator");
//         let letters_only_entropy = generator.calculate_entropy();

//         // Configure for all character types
//         generator.configure(
//             12,
//             true, true, true, true,    // all types
//             0, 0, false,
//         ).expect("Failed to configure generator");
//         let all_chars_entropy = generator.calculate_entropy();

//         // Entropy should be higher with more character types
//         assert!(all_chars_entropy > letters_only_entropy,
//                 "Entropy with full character set ({:.2}) should be higher than \
//              letters-only entropy ({:.2})",
//                 all_chars_entropy, letters_only_entropy);
//     }
// }
