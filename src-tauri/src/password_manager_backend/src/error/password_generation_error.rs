use serde::Serialize;
use thiserror::Error;
use crate::_vault::{ABSOLUTE_MAX_LENGTH, ABSOLUTE_MIN_LENGTH};

#[derive(Debug, Error, Serialize)]
pub enum PasswordGenerationError {
    #[error("Password length {0} is below minimum allowed length {ABSOLUTE_MIN_LENGTH}")]
    LengthTooShort(usize),

    #[error("Password length {0} exceeds maximum allowed length {ABSOLUTE_MAX_LENGTH}")]
    LengthTooLong(usize),

    #[error("No character sets selected for password generation")]
    EmptyCharacterSet,

    #[error("Minimum requirements ({0}) exceed password length ({1})")]
    ExcessiveMinimums(usize, usize),

    #[error("Failed to generate random character: {0}")]
    RandomGenerationFailed(String),

    #[error("No valid characters available with current configuration")]
    NoValidCharacters,

    #[error("System error during password generation: {0}")]
    SystemError(String),
}