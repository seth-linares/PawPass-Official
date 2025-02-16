mod sensitive_data;
pub mod overview;
pub mod entry;
pub mod collection;
pub mod search;

pub use crate::entry::entry::Entry;

// Constants for validation
pub const MAX_TITLE_LENGTH: usize = 100;
pub const MAX_USERNAME_LENGTH: usize = 100;
pub const MAX_URL_LENGTH: usize = 2048;
pub const MAX_NOTES_LENGTH: usize = 10000;
pub const MAX_PASS_CHAR_LENGTH: usize = 128;
