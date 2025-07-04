//! Database queries for the staratlas schema

use crate::connection::DbPool;
use crate::error::{DbError, Result};
use crate::models::{NewPlayer, NewToken, Player, Token};

/// Retrieves all tokens from the database
///
/// # Arguments
/// * `pool` - The database connection pool
///
/// # Returns
/// A vector of all tokens in the database
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_all_tokens(pool: &DbPool) -> Result<Vec<Token>> {
    let tokens = sqlx::query_as::<_, Token>(
        r#"
        SELECT id, mint, name, symbol, token_type
        FROM staratlas.tokens
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(tokens)
}

/// Retrieves a token by its ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `id` - The ID of the token to retrieve
///
/// # Returns
/// The token with the specified ID, or None if no such token exists
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_token_by_id(pool: &DbPool, id: i32) -> Result<Option<Token>> {
    let token = sqlx::query_as::<_, Token>(
        r#"
        SELECT id, mint, name, symbol, token_type
        FROM staratlas.tokens
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(token)
}

/// Creates a new token in the database
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `new_token` - The token to create
///
/// # Returns
/// The created token with its assigned ID
///
/// # Errors
/// Returns an error if the query fails
pub async fn create_token(pool: &DbPool, new_token: &NewToken) -> Result<Token> {
    let token = sqlx::query_as::<_, Token>(
        r#"
        INSERT INTO staratlas.tokens (
            mint, name, symbol, token_type
        )
        VALUES (
            $1, $2, $3, $4
        )
        RETURNING id, mint, name, symbol, token_type
        "#,
    )
    .bind(&new_token.mint)
    .bind(&new_token.name)
    .bind(&new_token.symbol)
    .bind(&new_token.token_type)
    .fetch_one(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(token)
}

/// Retrieves all players from the database
///
/// # Arguments
/// * `pool` - The database connection pool
///
/// # Returns
/// A vector of all players in the database
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_all_players(pool: &DbPool) -> Result<Vec<Player>> {
    let players = sqlx::query_as::<_, Player>(
        r#"
        SELECT id, wallet_address, username, first_seen, last_active
        FROM staratlas.players
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(players)
}

/// Retrieves a player by their ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `id` - The ID of the player to retrieve
///
/// # Returns
/// The player with the specified ID, or None if no such player exists
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_player_by_id(pool: &DbPool, id: i32) -> Result<Option<Player>> {
    let player = sqlx::query_as::<_, Player>(
        r#"
        SELECT id, wallet_address, username, first_seen, last_active
        FROM staratlas.players
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(player)
}

/// Retrieves a player by their wallet address
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `wallet_address` - The wallet address of the player to retrieve
///
/// # Returns
/// The player with the specified wallet address, or None if no such player exists
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_player_by_wallet_address(
    pool: &DbPool,
    wallet_address: &str,
) -> Result<Option<Player>> {
    let player = sqlx::query_as::<_, Player>(
        r#"
        SELECT id, wallet_address, username, first_seen, last_active
        FROM staratlas.players
        WHERE wallet_address = $1
        "#,
    )
    .bind(wallet_address)
    .fetch_optional(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(player)
}

/// Creates a new player in the database
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `new_player` - The player to create
///
/// # Returns
/// The created player with its assigned ID
///
/// # Errors
/// Returns an error if the query fails
pub async fn create_player(pool: &DbPool, new_player: &NewPlayer) -> Result<Player> {
    let player = sqlx::query_as::<_, Player>(
        r#"
        INSERT INTO staratlas.players (
            wallet_address, username, first_seen, last_active
        )
        VALUES (
            $1, $2, $3, $4
        )
        RETURNING id, wallet_address, username, first_seen, last_active
        "#,
    )
    .bind(&new_player.wallet_address)
    .bind(&new_player.username)
    .bind(new_player.first_seen)
    .bind(new_player.last_active)
    .fetch_one(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(player)
}
