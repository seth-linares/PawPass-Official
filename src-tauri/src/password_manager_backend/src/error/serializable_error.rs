use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "kind", content = "payload")]
#[serde(rename_all = "camelCase")]
pub enum SerializableError {
    // Authentication Errors
    Auth {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    },

    // Cryptographic Errors
    Crypto {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    },

    // Category Management Errors
    Category {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        category_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        category_name: Option<String>,
    },

    // Entry Management Errors
    Entry {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        entry_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        field_name: Option<String>,
    },

    // Password Generation Errors
    PasswordGeneration {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        length: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    },

    // Vault Errors
    Vault {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    },

    // File System Errors
    Io {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        operation: Option<String>,
    },

    // Data Errors
    Data {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    },

    // Validation Errors
    Validation {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        field: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    },

    // Generic/Unknown Errors
    Unknown {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    }
}