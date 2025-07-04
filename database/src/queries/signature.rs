//! Database queries for the indexer.signatures and indexer.program_signatures tables

use sqlx::types::chrono::{DateTime, Utc};

use crate::connection::DbPool;
use crate::error::{DbError, Result};
use crate::models::{
    NewProgram, NewProgramSignature, NewSignature, Program, ProgramSignature, Signature,
};
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
pub async fn get_all_signatures(pool: &DbPool) -> Result<Vec<Signature>> {
    let signatures = sqlx::query_as::<_, Signature>(
        r#"
        SELECT signature, slot, timestamp
        FROM indexer.signatures
        ORDER BY timestamp DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(signatures)
}

/// Retrieves a signature by its value
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `signature` - The signature value to retrieve
///
/// # Returns
/// The signature with the specified value, or None if no such signature exists
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_signature_by_value(
    pool: &DbPool,
    signature: &SignatureType,
) -> Result<Option<Signature>> {
    let signature_record = sqlx::query_as::<_, Signature>(
        r#"
        SELECT signature, slot, timestamp
        FROM indexer.signatures
        WHERE signature = $1
        "#,
    )
    .bind(signature)
    .fetch_optional(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(signature_record)
}

/// Retrieves signatures by timestamp range
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `start` - The start of the timestamp range
/// * `end` - The end of the timestamp range
///
/// # Returns
/// A vector of signatures within the specified timestamp range
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_signatures_by_timestamp_range(
    pool: &DbPool,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<Signature>> {
    let signatures = sqlx::query_as::<_, Signature>(
        r#"
        SELECT signature, slot, timestamp
        FROM indexer.signatures
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
/// The created signature
///
/// # Errors
/// Returns an error if the query fails
pub async fn create_signature(pool: &DbPool, new_signature: &NewSignature) -> Result<Signature> {
    let signature = sqlx::query_as::<_, Signature>(
        r#"
        INSERT INTO indexer.signatures (
            signature, slot, timestamp
        )
        VALUES (
            $1, $2, $3
        )
        RETURNING signature, slot, timestamp
        "#,
    )
    .bind(&new_signature.signature)
    .bind(new_signature.slot)
    .bind(new_signature.timestamp)
    .fetch_one(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(signature)
}

/// Deletes a signature from the database
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `signature` - The signature value to delete
///
/// # Returns
/// `true` if a signature was deleted, `false` if no signature with the specified value exists
///
/// # Errors
/// Returns an error if the query fails
pub async fn delete_signature(pool: &DbPool, signature: &SignatureType) -> Result<bool> {
    let result = sqlx::query("DELETE FROM indexer.signatures WHERE signature = $1")
        .bind(signature)
        .execute(pool)
        .await
        .map_err(DbError::SqlxError)?;

    Ok(result.rows_affected() > 0)
}

/// Retrieves all program signatures from the database
///
/// # Arguments
/// * `pool` - The database connection pool
///
/// # Returns
/// A vector of all program signatures in the database
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_all_program_signatures(pool: &DbPool) -> Result<Vec<ProgramSignature>> {
    let program_signatures = sqlx::query_as::<_, ProgramSignature>(
        r#"
        SELECT program_id, signature, processed
        FROM indexer.program_signatures
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(program_signatures)
}

/// Retrieves program signatures by program ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `program_id` - The program ID to search for
///
/// # Returns
/// A vector of program signatures with the specified program ID
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_program_signatures_by_program_id(
    pool: &DbPool,
    program_id: &PublicKeyType,
) -> Result<Vec<ProgramSignature>> {
    let program_signatures = sqlx::query_as::<_, ProgramSignature>(
        r#"
        SELECT program_id, signature, processed
        FROM indexer.program_signatures
        WHERE program_id = $1
        "#,
    )
    .bind(program_id)
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(program_signatures)
}

/// Retrieves the most recent program signature for a program ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `program_id` - The program ID to search for
///
/// # Returns
/// The most recent program signature for the specified program ID, or None if no signatures exist
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_last_program_signature_by_program_id(
    pool: &DbPool,
    program_id: &PublicKeyType,
) -> Result<Option<ProgramSignature>> {
    let program_signature = sqlx::query_as::<_, ProgramSignature>(
        r#"
        SELECT ps.program_id, ps.signature, ps.processed
        FROM indexer.program_signatures ps
        JOIN indexer.signatures s ON ps.signature = s.signature
        WHERE program_id = $1
        ORDER BY s.timestamp ASC 
        LIMIT 1
        "#,
    )
    .bind(program_id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(program_signature)
}

/// Retrieves unprocessed program signatures by program ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `program_id` - The program ID to search for
/// * `limit` - The maximum number of program signatures to retrieve
///
/// # Returns
/// A vector of unprocessed program signatures with the specified program ID
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_unprocessed_program_signatures_by_program_id(
    pool: &DbPool,
    program_id: &PublicKeyType,
    limit: i64,
) -> Result<Vec<ProgramSignature>> {
    let program_signatures = sqlx::query_as::<_, ProgramSignature>(
        r#"
        SELECT program_id, signature, processed
        FROM indexer.program_signatures
        WHERE program_id = $1 AND processed = false
        LIMIT $2
        "#,
    )
    .bind(program_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(program_signatures)
}

/// Creates a new program signature in the database
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `new_program_signature` - The program signature to create
///
/// # Returns
/// The created program signature
///
/// # Errors
/// Returns an error if the query fails
pub async fn create_program_signature(
    pool: &DbPool,
    new_program_signature: &NewProgramSignature,
) -> Result<ProgramSignature> {
    let program_signature = sqlx::query_as::<_, ProgramSignature>(
        r#"
        INSERT INTO indexer.program_signatures (
            program_id, signature, processed
        )
        VALUES (
            $1, $2, $3
        )
        RETURNING program_id, signature, processed
        "#,
    )
    .bind(&new_program_signature.program_id)
    .bind(&new_program_signature.signature)
    .bind(new_program_signature.processed)
    .fetch_one(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(program_signature)
}

/// Updates the processed status of a program signature
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `program_id` - The program ID of the program signature to update
/// * `signature` - The signature of the program signature to update
/// * `processed` - The new processed status
///
/// # Returns
/// The updated program signature
///
/// # Errors
/// Returns an error if the query fails
pub async fn update_program_signature_processed(
    pool: &DbPool,
    program_id: &PublicKeyType,
    signature: &SignatureType,
    processed: bool,
) -> Result<ProgramSignature> {
    let program_signature = sqlx::query_as::<_, ProgramSignature>(
        r#"
        UPDATE indexer.program_signatures
        SET processed = $3
        WHERE program_id = $1 AND signature = $2
        RETURNING program_id, signature, processed
        "#,
    )
    .bind(program_id)
    .bind(signature)
    .bind(processed)
    .fetch_one(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(program_signature)
}

/// Retrieves all programs from the database
///
/// # Arguments
/// * `pool` - The database connection pool
///
/// # Returns
/// A vector of all programs in the database
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_all_programs(pool: &DbPool) -> Result<Vec<Program>> {
    let programs = sqlx::query_as::<_, Program>(
        r#"
        SELECT program_id
        FROM indexer.programs
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(programs)
}

/// Retrieves a program by its ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `program_id` - The program ID to retrieve
///
/// # Returns
/// The program with the specified ID, or None if no such program exists
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_program_by_id(
    pool: &DbPool,
    program_id: &PublicKeyType,
) -> Result<Option<Program>> {
    let program = sqlx::query_as::<_, Program>(
        r#"
        SELECT program_id
        FROM indexer.programs
        WHERE program_id = $1
        "#,
    )
    .bind(program_id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(program)
}

/// Creates a new program in the database
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `new_program` - The program to create
///
/// # Returns
/// The created program
///
/// # Errors
/// Returns an error if the query fails
pub async fn create_program(pool: &DbPool, new_program: &NewProgram) -> Result<Program> {
    let program = sqlx::query_as::<_, Program>(
        r#"
        INSERT INTO indexer.programs (
            program_id
        )
        VALUES (
            $1
        )
        RETURNING program_id
        "#,
    )
    .bind(&new_program.program_id)
    .fetch_one(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(program)
}
