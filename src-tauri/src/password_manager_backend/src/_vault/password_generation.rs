use std::collections::HashSet;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use crate::_vault::{ABSOLUTE_MAX_LENGTH, ABSOLUTE_MIN_LENGTH, DEFAULT_LENGTH};
use crate::error::password_generation_error::PasswordGenerationError;
use rand_chacha::ChaCha20Rng;

/// PasswordGenerator: A secure and configurable password generation system
/// Provides both configuration and generation capabilities in a single struct
/// while maintaining strong security guarantees and flexibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordGenerator {
    // Core Configuration
    min_required: usize,      // Minimum password length (Absolute minimum: 5)
    length: usize,          // Current password length (default: 16)

    // Character Set Controls
    use_lowercase: bool,    // Include lowercase letters
    use_uppercase: bool,    // Include uppercase letters
    use_numbers: bool,      // Include numbers
    use_symbols: bool,      // Include special characters

    // Minimum Requirements (ensures password strength)
    pub(crate) min_numbers: usize,     // Minimum required numbers
    pub(crate) min_symbols: usize,     // Minimum required symbols

    // Readability Controls
    exclude_ambiguous: bool, // Exclude ambiguous characters (1/l/I, 0/O, etc.)

    // Pre-computed Character Sets (computed once at initialization)
    pub(crate) available_chars: CharacterSets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordGeneratorSettings {
    pub length: usize,
    #[serde(rename = "useLowercase")]
    pub use_lowercase: bool,
    #[serde(rename = "useUppercase")]
    pub use_uppercase: bool,
    #[serde(rename = "useNumbers")]
    pub use_numbers: bool,
    #[serde(rename = "useSymbols")]
    pub use_symbols: bool,
    #[serde(rename = "minNumbers")]
    pub min_numbers: usize,
    #[serde(rename = "minSymbols")]
    pub min_symbols: usize,
    #[serde(rename = "excludeAmbiguous")]
    pub exclude_ambiguous: bool,
}

/// Separate struct to manage the various character sets
/// This helps keep the main struct clean while allowing
/// efficient character set operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSets {
    pub(crate) lowercase: Vec<char>,  // a-z
    pub(crate) uppercase: Vec<char>,  // A-Z
    pub(crate) numbers: Vec<char>,    // 0-9
    pub(crate) symbols: Vec<char>,    // Special characters
    pub(crate) ambiguous: Vec<HashSet<char>>,    // Characters to exclude if ambiguous exclusion is on
}



impl PasswordGenerator {
    /// Helper function to calculate the minimum required length based on settings
    fn calculate_min_required(&self, settings: &PasswordGeneratorSettings) -> usize {
        // Calculate minimum from enabled character sets and their requirements
        let required_from_sets = 
            (settings.use_lowercase as usize) +
            (settings.use_uppercase as usize) +
            (if settings.use_numbers { settings.min_numbers } else { 0 }) +
            (if settings.use_symbols { settings.min_symbols } else { 0 });

        // Always enforce absolute minimum length, even if requirements are less
        required_from_sets.max(ABSOLUTE_MIN_LENGTH)
    }

    /// Configures the password generator with the given settings
    pub fn configure(&mut self, settings: PasswordGeneratorSettings) -> Result<(), PasswordGenerationError> {
        // First validate all settings before applying any changes
        self.validate_settings(&settings)?;

        // If validation passed, apply all settings
        self.length = settings.length;
        self.use_lowercase = settings.use_lowercase;
        self.use_uppercase = settings.use_uppercase;
        self.use_numbers = settings.use_numbers;
        self.use_symbols = settings.use_symbols;
        self.min_numbers = settings.min_numbers;
        self.min_symbols = settings.min_symbols;
        self.exclude_ambiguous = settings.exclude_ambiguous;

        // Calculate minimum required length based on chosen options
        self.min_required = self.calculate_min_required(&settings);

        Ok(())
    }

    pub fn get_settings(&self) -> PasswordGeneratorSettings {
        PasswordGeneratorSettings {
            length: self.length,
            use_lowercase: self.use_lowercase,
            use_uppercase: self.use_uppercase,
            use_numbers: self.use_numbers,
            use_symbols: self.use_symbols,
            min_numbers: self.min_numbers,
            min_symbols: self.min_symbols,
            exclude_ambiguous: self.exclude_ambiguous,
        }
    }

    /// Validates the proposed settings before they are applied
    fn validate_settings(&self, settings: &PasswordGeneratorSettings) -> Result<(), PasswordGenerationError> {
        // Check absolute length bounds
        if settings.length < ABSOLUTE_MIN_LENGTH {
            return Err(PasswordGenerationError::LengthTooShort(settings.length));
        }
        if settings.length > ABSOLUTE_MAX_LENGTH {
            return Err(PasswordGenerationError::LengthTooLong(settings.length));
        }

        // Ensure at least one character type is selected
        if !settings.use_lowercase && !settings.use_uppercase && !settings.use_numbers && !settings.use_symbols {
            return Err(PasswordGenerationError::EmptyCharacterSet);
        }

        // Calculate and check minimum required length
        let required_length = self.calculate_min_required(settings);
        if settings.length < required_length {
            return Err(PasswordGenerationError::ExcessiveMinimums(required_length, settings.length));
        }

        Ok(())
    }
}

impl PasswordGenerator {
    /// Generates a password meeting all configured requirements
    pub fn generate(&self) -> Result<String, PasswordGenerationError> {
        // Initialize our secure RNG
        let mut rng = ChaCha20Rng::from_entropy();

        // Build our initial password with minimum requirements
        let mut password_chars = Vec::with_capacity(self.length);

        // Track which ambiguous groups we've used
        let mut used_ambiguous_groups = HashSet::new();

        // First, satisfy minimum requirements by ensuring we get at least one of each required type
        if self.use_lowercase {
            self.add_random_char_from_set(&mut password_chars, &self.available_chars.lowercase, &mut used_ambiguous_groups, &mut rng)?;
        }

        if self.use_uppercase {
            self.add_random_char_from_set(&mut password_chars, &self.available_chars.uppercase, &mut used_ambiguous_groups, &mut rng)?;
        }

        // Add required numbers
        for _ in 0..self.min_numbers {
            self.add_random_char_from_set(&mut password_chars, &self.available_chars.numbers, &mut used_ambiguous_groups, &mut rng)?;
        }

        // Add required symbols
        for _ in 0..self.min_symbols {
            self.add_random_char_from_set(&mut password_chars, &self.available_chars.symbols, &mut used_ambiguous_groups, &mut rng)?;
        }

        // Fill remaining length with random allowed characters
        while password_chars.len() < self.length {
            self.add_random_allowed_char(
                &mut password_chars,
                &mut used_ambiguous_groups,
                &mut rng
            )?;
        }

        // Shuffle the final password
        self.shuffle_password(&mut password_chars, &mut rng);

        // Convert to string and return
        Ok(password_chars.into_iter().collect())
    }

    /// Adds a random character from a specific character set while respecting ambiguous rules
    fn add_random_char_from_set(
        &self,
        password: &mut Vec<char>,
        char_set: &[char],
        used_groups: &mut HashSet<usize>,
        rng: &mut ChaCha20Rng
    ) -> Result<(), PasswordGenerationError> {
        use rand::seq::SliceRandom;

        // If we're excluding ambiguous characters, build a filtered set
        let available_chars: Vec<&char> = if self.exclude_ambiguous {
            char_set.iter()
                .filter(|c| {
                    // A character is available if it's either not in any ambiguous group
                    // or its group hasn't been used yet
                    !self.available_chars.ambiguous.iter().enumerate()
                        .any(|(idx, group)| group.contains(c) && used_groups.contains(&idx))
                })
                .collect()
        } else {
            char_set.iter().collect()
        };

        if available_chars.is_empty() {
            return Err(PasswordGenerationError::EmptyCharacterSet);
        }

        // Choose a random character from the filtered set
        if let Some(&chosen_char) = available_chars.choose(rng) {
            // If this is an ambiguous character, mark its group as used
            if self.exclude_ambiguous {
                for (group_idx, group) in self.available_chars.ambiguous.iter().enumerate() {
                    if group.contains(chosen_char) {
                        used_groups.insert(group_idx);
                        break;
                    }
                }
            }

            password.push(*chosen_char);
            Ok(())
        } else {
            Err(PasswordGenerationError::RandomGenerationFailed(
                "Failed to choose random character".to_string()
            ))
        }
    }


    /// Adds a random character from any allowed set while respecting ambiguous rules
    /// Adds a random character from any allowed set while respecting ambiguous rules
    fn add_random_allowed_char(
        &self,
        password: &mut Vec<char>,
        used_groups: &mut HashSet<usize>,
        rng: &mut ChaCha20Rng
    ) -> Result<(), PasswordGenerationError> {
        use rand::seq::SliceRandom;

        // Build vector of allowed character sets that we'll choose from
        let mut allowed_sets = Vec::new();
        if self.use_lowercase { allowed_sets.push(&self.available_chars.lowercase); }
        if self.use_uppercase { allowed_sets.push(&self.available_chars.uppercase); }
        if self.use_numbers { allowed_sets.push(&self.available_chars.numbers); }
        if self.use_symbols { allowed_sets.push(&self.available_chars.symbols); }

        // Choose a random set
        if let Some(char_set) = allowed_sets.choose(rng) {
            // Use our helper function to choose from this set
            self.add_random_char_from_set(password, char_set, used_groups, rng)
        } else {
            Err(PasswordGenerationError::EmptyCharacterSet)
        }
    }

    /// Securely shuffles the password characters
    fn shuffle_password(&self, password: &mut Vec<char>, rng: &mut ChaCha20Rng) {
        use rand::seq::SliceRandom;
        password.shuffle(rng);
    }

    pub fn calculate_entropy(&self) -> f64 {
        let charset_size = self.get_effective_charset_size();
        (self.length as f64) * (charset_size).log2()
    }

    fn get_effective_charset_size(&self) -> f64 {
        let mut size = 0.0;
        if self.use_lowercase { size += self.available_chars.lowercase.len() as f64; }
        if self.use_uppercase { size += self.available_chars.uppercase.len() as f64; }
        if self.use_numbers { size += self.available_chars.numbers.len() as f64; }
        if self.use_symbols { size += self.available_chars.symbols.len() as f64; }
        if self.exclude_ambiguous {
            for group in &self.available_chars.ambiguous {
                size -= (group.len() - 1) as f64;
            }
        }
        size
    }

}

impl Default for PasswordGenerator {
    fn default() -> Self {
        // Initialize with secure defaults:
        // - 16 character minimum length
        // - All character sets enabled
        // - At least one of each type required
        // - Similar and ambiguous characters excluded
        let password_generator = Self {
            min_required: 6,
            length: DEFAULT_LENGTH,
            use_lowercase: true,
            use_uppercase: true,
            use_numbers: true,
            use_symbols: true,
            min_numbers: 2,
            min_symbols: 2,
            exclude_ambiguous: true,
            available_chars: CharacterSets::default(),
        };

        password_generator
    }
}

impl CharacterSets {
    pub fn default() -> Self {
        // Initialize with default character sets
        // - Lowercase: a-z
        // - Uppercase: A-Z
        // - Numbers: 0-9
        // - Symbols: !@#$%^&*
        // - Ambiguous: 1lIoO0

        Self {
            lowercase: (b'a'..=b'z').map(char::from).collect(),
            uppercase: (b'A'..=b'Z').map(char::from).collect(),
            numbers: (b'0'..=b'9').map(char::from).collect(),
            symbols: vec!['!', '@', '#', '$', '%', '^', '&', '*'],
            ambiguous: vec![
                ['1', 'l', 'I'].iter().cloned().collect(),
                ['o', 'O', '0'].iter().cloned().collect(),
                ['5', 'S'].iter().cloned().collect(),
                ['2', 'Z'].iter().cloned().collect(),
                ['8', 'B'].iter().cloned().collect(),
            ],
        }
    }
}
