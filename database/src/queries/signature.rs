//! Database queries for the signatures table

use sqlx::types::chrono::{DateTime, Utc};

use crate::connection::DbPool;
use crate::error::{DbError, Result};
use crate::models::{NewSignature, SignatureRecord};
use crate::types::{PublicKeyType, SignatureType};

/// Retrieves all signatures from the database
///
/// # Arguments
/// * `pool` - The database connection pool
///
/// # Returns
/// A vector of all signatures in the database
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_all_signatures(pool: &DbPool) -> Result<Vec<SignatureRecord>> {
    let signatures = sqlx::query_as::<_, SignatureRecord>(
        r#"
        SELECT id, program_id, signature, slot, timestamp, processed
        FROM signatures
        ORDER BY timestamp DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(signatures)
}

/// Retrieves a signature by its ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `id` - The ID of the signature to retrieve
///
/// # Returns
/// The signature with the specified ID, or None if no such signature exists
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_signature_by_id(pool: &DbPool, id: i32) -> Result<Option<SignatureRecord>> {
    let signature = sqlx::query_as::<_, SignatureRecord>(
        r#"
        SELECT id, program_id, signature, slot, timestamp, processed
        FROM signatures
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(signature)
}

/// Retrieves signatures by program ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `program_id` - The program ID to search for
/// * `limit` - Optional maximum number of signatures to retrieve
///
/// # Returns
/// A vector of signatures with the specified program ID
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_signatures_by_program_id(
    pool: &DbPool,
    program_id: &PublicKeyType,
    limit: Option<i64>,
) -> Result<Vec<SignatureRecord>> {
    let query = match limit {
        Some(limit_value) => {
            sqlx::query_as::<_, SignatureRecord>(
                r#"
                SELECT id, program_id, signature, slot, timestamp, processed
                FROM signatures
                WHERE program_id = $1
                ORDER BY timestamp DESC
                LIMIT $2
                "#,
            )
            .bind(program_id.clone())
            .bind(limit_value)
        }
        None => {
            sqlx::query_as::<_, SignatureRecord>(
                r#"
                SELECT id, program_id, signature, slot, timestamp, processed
                FROM signatures
                WHERE program_id = $1
                ORDER BY timestamp DESC
                "#,
            )
            .bind(program_id.clone())
        }
    };

    let signatures = query
        .fetch_all(pool)
        .await
        .map_err(DbError::SqlxError)?;

    Ok(signatures)
}

/// Retrieves signatures by signature value
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `signature` - The signature value to search for
///
/// # Returns
/// A vector of signatures with the specified signature value
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_signatures_by_signature(
    pool: &DbPool,
    signature: &SignatureType,
) -> Result<Vec<SignatureRecord>> {
    let signatures = sqlx::query_as::<_, SignatureRecord>(
        r#"
        SELECT id, program_id, signature, slot, timestamp, processed
        FROM signatures
        WHERE signature = $1
        ORDER BY timestamp DESC
        "#,
    )
    .bind(signature.clone())
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(signatures)
}

pub async fn get_signatures_by_timestamp_range(
    pool: &DbPool,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<SignatureRecord>> {
    let signatures = sqlx::query_as::<_, SignatureRecord>(
        r#"
        SELECT id, program_id, signature, slot, timestamp
        FROM signatures
        WHERE timestamp >= $1 AND timestamp <= $2
        ORDER BY timestamp DESC
        "#,
    )
    .bind(start)
    .bind(end)
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(signatures)
}

/// Creates a new signature in the database
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `new_signature` - The signature to create
///
/// # Returns
/// The created signature with its assigned ID
///
/// # Errors
/// Returns an error if the query fails
pub async fn create_signature(
    pool: &DbPool,
    new_signature: &NewSignature,
) -> Result<SignatureRecord> {
    let signature = sqlx::query_as::<_, SignatureRecord>(
        r#"
        INSERT INTO signatures (
            program_id, signature, slot, timestamp, processed
        )
        VALUES (
            $1, $2, $3, $4, $5
        )
        RETURNING id, program_id, signature, slot, timestamp, processed
        "#,
    )
    .bind(&new_signature.program_id)
    .bind(&new_signature.signature)
    .bind(new_signature.slot)
    .bind(new_signature.timestamp)
    .bind(false)
    .fetch_one(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(signature)
}

/// Deletes a signature from the database
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `id` - The ID of the signature to delete
///
/// # Returns
/// `true` if a signature was deleted, `false` if no signature with the specified ID exists
///
/// # Errors
/// Returns an error if the query fails
pub async fn delete_signature(pool: &DbPool, id: i32) -> Result<bool> {
    let result = sqlx::query("DELETE FROM signatures WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(DbError::SqlxError)?;

    Ok(result.rows_affected() > 0)
}

/// Deletes signatures by program ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `program_id` - The program ID of the signatures to delete
///
/// # Returns
/// The number of signatures deleted
///
/// # Errors
/// Returns an error if the query fails
pub async fn delete_signatures_by_program_id(
    pool: &DbPool,
    program_id: &PublicKeyType,
) -> Result<u64> {
    let result = sqlx::query("DELETE FROM signatures WHERE program_id = $1")
        .bind(program_id.clone())
        .execute(pool)
        .await
        .map_err(DbError::SqlxError)?;

    Ok(result.rows_affected())
}

/// Retrieves N unprocessed signatures by program ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `program_id` - The program ID to search for
/// * `limit` - The maximum number of signatures to retrieve
///
/// # Returns
/// A vector of unprocessed signatures with the specified program ID
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_unprocessed_signatures_by_program_id(
    pool: &DbPool,
    program_id: &PublicKeyType,
    limit: i64,
) -> Result<Vec<SignatureRecord>> {
    let signatures = sqlx::query_as::<_, SignatureRecord>(
        r#"
        SELECT id, program_id, signature, slot, timestamp, processed
        FROM signatures
        WHERE program_id = $1 AND processed = false
        ORDER BY timestamp ASC
        LIMIT $2
        "#,
    )
    .bind(program_id.clone())
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(signatures)
}
