//! Error types for the database library

use thiserror::Error;

/// A specialized Result type for database operations
pub type Result<T> = std::result::Result<T, DbError>;

/// Errors that can occur during database operations
#[derive(Debug, Error)]
pub enum DbError {
    /// Error from the sqlx library
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),

    /// Error when loading environment variables
    #[error("Environment error: {0}")]
    EnvError(#[from] std::env::VarError),

    /// Error when parsing database URL
    #[error("Database URL parse error: {0}")]
    UrlParseError(String),

    /// Error when a record is not found
    #[error("Record not found")]
    NotFound,

    /// Other errors
    #[error("Other error: {0}")]
    Other(String),
}

impl From<String> for DbError {
    fn from(error: String) -> Self {
        DbError::Other(error)
    }
}

impl From<&str> for DbError {
    fn from(error: &str) -> Self {
        DbError::Other(error.to_string())
    }
}