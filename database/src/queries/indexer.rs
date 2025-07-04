//! Database queries for the indexer table

use sqlx::Error as SqlxError;

use crate::connection::DbPool;
use crate::error::{DbError, Result};
use crate::models::{Indexer, NewIndexer, UpdateIndexer};
use crate::types::PublicKeyType;

/// Retrieves all indexers from the database
///
/// # Arguments
/// * `pool` - The database connection pool
///
/// # Returns
/// A vector of all indexers in the database
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_all_indexers(pool: &DbPool) -> Result<Vec<Indexer>> {
    let indexers = sqlx::query_as::<_, Indexer>(
        r#"
        SELECT id, name, direction, program_id, start_signature, before_signature, 
               start_block, before_block, finished
        FROM indexer.indexer
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(indexers)
}

/// Retrieves an indexer by its ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `id` - The ID of the indexer to retrieve
///
/// # Returns
/// The indexer with the specified ID, or None if no such indexer exists
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_indexer_by_id(pool: &DbPool, id: i32) -> Result<Option<Indexer>> {
    let indexer = sqlx::query_as::<_, Indexer>(
        r#"
        SELECT id, name, direction, program_id, start_signature, before_signature, 
               start_block, before_block, finished
        FROM indexer.indexer
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(indexer)
}

/// Retrieves indexers by program ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `program_id` - The program ID to search for
///
/// # Returns
/// A vector of indexers with the specified program ID
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_indexers_by_program_id(
    pool: &DbPool,
    program_id: &PublicKeyType,
) -> Result<Vec<Indexer>> {
    let indexers = sqlx::query_as::<_, Indexer>(
        r#"
        SELECT id, name, direction, program_id, start_signature, before_signature, 
               start_block, before_block, finished
        FROM indexer.indexer
        WHERE program_id = $1
        ORDER BY id
        "#,
    )
    .bind(program_id.clone())
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(indexers)
}

/// Retrieves indexers by name
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `name` - The name to search for
///
/// # Returns
/// A vector of indexers with the specified name
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_indexers_by_name(pool: &DbPool, name: &str) -> Result<Vec<Indexer>> {
    let indexers = sqlx::query_as::<_, Indexer>(
        r#"
        SELECT id, name, direction, program_id, start_signature, before_signature, 
               start_block, before_block, finished
        FROM indexer.indexer
        WHERE name = $1
        ORDER BY id
        "#,
    )
    .bind(name.to_string())
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(indexers)
}

/// Creates a new indexer in the database
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `new_indexer` - The indexer to create
///
/// # Returns
/// The created indexer with its assigned ID
///
/// # Errors
/// Returns an error if the query fails
pub async fn create_indexer(pool: &DbPool, new_indexer: &NewIndexer) -> Result<Indexer> {
    let indexer = sqlx::query_as::<_, Indexer>(
        r#"
        INSERT INTO indexer.indexer (
            id, name, direction, program_id, before_signature, until_signature, 
            before_block, until_block, finished
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9
        )
        RETURNING id, name, direction, program_id, until_signature, before_signature, 
                  start_block, before_block, finished
        "#,
    )
    .bind(new_indexer.id)
    .bind(&new_indexer.name)
    .bind(&new_indexer.direction)
    .bind(&new_indexer.program_id)
    .bind(&new_indexer.until_signature)
    .bind(&new_indexer.before_signature)
    .bind(new_indexer.start_block)
    .bind(new_indexer.before_block)
    .bind(new_indexer.finished)
    .fetch_one(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(indexer)
}

/// Updates an existing indexer in the database
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `id` - The ID of the indexer to update
/// * `update` - The updates to apply
///
/// # Returns
/// The updated indexer
///
/// # Errors
/// Returns an error if the query fails or if no indexer with the specified ID exists
pub async fn update_indexer(pool: &DbPool, id: i32, update: &UpdateIndexer) -> Result<Indexer> {
    let indexer = sqlx::query_as::<_, Indexer>(
        r#"
        UPDATE indexer.indexer
        SET 
            direction = COALESCE($1, direction),
            before_signature = $2,
            until_signature = $2,
            before_block = $3,
            unitl_block = $3,
            finished = $4
        WHERE id = $5
        RETURNING id, name, direction, program_id, until_signature, before_signature, 
                  start_block, before_block, finished
        "#,
    )
    .bind(&update.direction)
    .bind(&update.before_signature)
    .bind(update.until_block)
    .bind(update.finished)
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        SqlxError::RowNotFound => DbError::NotFound,
        _ => DbError::SqlxError(e),
    })?;

    Ok(indexer)
}

/// Deletes an indexer from the database
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `id` - The ID of the indexer to delete
///
/// # Returns
/// `true` if an indexer was deleted, `false` if no indexer with the specified ID exists
///
/// # Errors
/// Returns an error if the query fails
pub async fn delete_indexer(pool: &DbPool, id: i32) -> Result<bool> {
    let result = sqlx::query("DELETE FROM indexer.indexer WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(DbError::SqlxError)?;

    Ok(result.rows_affected() > 0)
}
