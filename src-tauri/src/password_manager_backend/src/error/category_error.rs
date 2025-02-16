use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, Serialize)]
pub enum CategoryError {
    // Validation errors
    #[error("Invalid category name: '{0}'")]
    InvalidName(String),
    
    #[error("Category with name '{0}' already exists")]
    DuplicateName(String),

    // Operation errors
    #[error("Category not found with ID: {0}")]
    NotFound(Uuid),

    #[error("Category not found with name: '{0}'")]
    NotFoundByName(String),

    // Other errors
    #[error("Validation error: {0}")]
    ValidationError(String),
}