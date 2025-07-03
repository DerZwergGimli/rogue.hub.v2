//! Database connection management

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::time::Duration;

use crate::error::{DbError, Result};

/// Type alias for the database connection pool
pub type DbPool = PgPool;

/// Establishes a connection to the database using the DATABASE_URL environment variable
///
/// # Returns
/// A connection pool that can be used for database operations
///
/// # Errors
/// Returns an error if the connection cannot be established or if the DATABASE_URL
/// environment variable is not set or is invalid
pub async fn establish_connection() -> Result<DbPool> {
    // Load environment variables from .env file if it exists
    dotenv::dotenv().ok();

    // Get the database URL from the environment
    let database_url = env::var("DATABASE_URL")
        .map_err(|e| {
            log::error!("DATABASE_URL environment variable not set");
            e
        })?;

    // Create a connection pool with reasonable defaults
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .map_err(|e| {
            log::error!("Failed to connect to database: {}", e);
            DbError::SqlxError(e)
        })?;

    // Test the connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|e| {
            log::error!("Failed to execute test query: {}", e);
            DbError::SqlxError(e)
        })?;

    log::info!("Successfully connected to database");
    Ok(pool)
}

/// Establishes a connection to the database using a provided URL
///
/// # Arguments
/// * `database_url` - The URL of the database to connect to
///
/// # Returns
/// A connection pool that can be used for database operations
///
/// # Errors
/// Returns an error if the connection cannot be established
pub async fn establish_connection_with_url(database_url: &str) -> Result<DbPool> {
    // Create a connection pool with reasonable defaults
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
        .map_err(|e| {
            log::error!("Failed to connect to database: {}", e);
            DbError::SqlxError(e)
        })?;

    // Test the connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|e| {
            log::error!("Failed to execute test query: {}", e);
            DbError::SqlxError(e)
        })?;

    log::info!("Successfully connected to database");
    Ok(pool)
}