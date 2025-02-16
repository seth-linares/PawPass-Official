use password_manager_backend::{entry::overview::EntryOverview, error::app_error::AppError};
use serde::Serialize;

const MIN_PASSWORD_LENGTH: usize = 8;
const MAX_PASSWORD_LENGTH: usize = 256;

fn check_password_complexity(password: &str) -> bool {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_number = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    has_uppercase && has_lowercase && has_number && has_special
}

pub fn validate_password(password: &str, confirm_password: &str) -> Result<(), AppError> {
    if password != confirm_password {
        return Err(AppError::PasswordsDoNotMatch);
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(AppError::PasswordTooLong);
    }

    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(AppError::PasswordTooShort);
    }

    if !check_password_complexity(password) {
        return Err(AppError::PasswordTooWeak);
    }

    Ok(())
}

#[derive(Debug, Serialize)]
pub struct EntryOverviewResults {
    pub entries: Vec<EntryOverview>,
    #[serde(rename = "totalCount")]
    pub total_count: usize,
}

#[derive(Debug, Serialize)]
pub struct VaultPaths {
    #[serde(rename = "vaultPath")]
    pub vault_path: String,
    #[serde(rename = "backupDir")]
    pub backup_dir: String,
    #[serde(rename = "tempDir")]
    pub temp_dir: String,
}


/*
 This macro is used to wrap commands that require the session to be active.
 I made this macro because I wanted to make sure that the session was active before running any commands that required it.
 Just a simple way to ensure that the command is being used only when the user is actually logged in.s
*/
#[macro_export]
macro_rules! protected_command {
    ($state:expr, $action:block) => {{
        if !$state.is_session_active().await {
            return Err(AppError::VaultLocked);
        }
        $action
    }};
}