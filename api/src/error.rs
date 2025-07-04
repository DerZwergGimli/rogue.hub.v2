//! Error types and conversions for the API

use poem::error::ResponseError;
use poem::http::StatusCode;
use thiserror::Error;

/// API-specific error types
#[derive(Debug, Error)]
pub enum ApiError {
    /// Database error
    #[error("Database error: {0}")]
    Database(#[from] db::DbError),

    /// Not found error
    #[error("Resource not found")]
    NotFound,

    /// Internal server error
    #[error("Internal server error: {0}")]
    Internal(String),

    /// Bad request error
    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl ResponseError for ApiError {
    fn status(&self) -> StatusCode {
        match self {
            ApiError::Database(db::DbError::NotFound) => StatusCode::NOT_FOUND,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

/// Result type for API operations
pub type Result<T> = std::result::Result<T, ApiError>;