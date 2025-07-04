//! Database queries for the market schema

use crate::connection::DbPool;
use crate::error::{DbError, Result};
use crate::models::{
    Exchange, ExchangeWithDependencies, NewExchange, NewPlayer, NewToken, Player, Token,
};
use crate::queries::staratlas;
use sqlx::types::chrono::{DateTime, Utc};

/// Retrieves all exchanges from the database
///
/// # Arguments
/// * `pool` - The database connection pool
///
/// # Returns
/// A vector of all exchanges in the database
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_all_exchanges(pool: &DbPool) -> Result<Vec<Exchange>> {
    let exchanges = sqlx::query_as::<_, Exchange>(
        r#"
        SELECT id, slot, signature, index, timestamp , side, buyer, seller, asset, pair, price, size, volume, fee, buddy
        FROM market.exchanges
        ORDER BY timestamp DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(exchanges)
}

/// Retrieves an exchange by its ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `id` - The ID of the exchange to retrieve
///
/// # Returns
/// The exchange with the specified ID, or None if no such exchange exists
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_exchange_by_id(pool: &DbPool, id: i32) -> Result<Option<Exchange>> {
    let exchange = sqlx::query_as::<_, Exchange>(
        r#"
        SELECT id, slot, signature, index, timestamp , side, buyer, seller, asset, pair, price, size, volume, fee, buddy
        FROM market.exchanges
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(exchange)
}

/// Retrieves exchanges by buyer ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `buyer_id` - The ID of the buyer to search for
///
/// # Returns
/// A vector of exchanges with the specified buyer ID
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_exchanges_by_buyer_id(pool: &DbPool, buyer_id: i32) -> Result<Vec<Exchange>> {
    let exchanges = sqlx::query_as::<_, Exchange>(
        r#"
        SELECT id, slot, signature, index, timestamp , side, buyer, seller, asset, pair, price, size, volume, fee, buddy
        FROM market.exchanges
        WHERE buyer = $1
        ORDER BY timestamp DESC
        "#,
    )
    .bind(buyer_id)
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(exchanges)
}

/// Retrieves exchanges by seller ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `seller_id` - The ID of the seller to search for
///
/// # Returns
/// A vector of exchanges with the specified seller ID
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_exchanges_by_seller_id(pool: &DbPool, seller_id: i32) -> Result<Vec<Exchange>> {
    let exchanges = sqlx::query_as::<_, Exchange>(
        r#"
        SELECT id, slot, signature, index, timestamp, side, buyer, seller, asset, pair, price, size, volume, fee, buddy
        FROM market.exchanges
        WHERE seller = $1
        ORDER BY timestamp DESC
        "#,
    )
    .bind(seller_id)
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(exchanges)
}

/// Retrieves exchanges by asset ID
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `asset_id` - The ID of the asset to search for
///
/// # Returns
/// A vector of exchanges with the specified asset ID
///
/// # Errors
/// Returns an error if the query fails
pub async fn get_exchanges_by_asset_id(pool: &DbPool, asset_id: i32) -> Result<Vec<Exchange>> {
    let exchanges = sqlx::query_as::<_, Exchange>(
        r#"
        SELECT id, slot, signature, index, timestamp, side, buyer, seller, asset, pair, price, size, volume, fee, buddy
        FROM market.exchanges
        WHERE asset = $1
        ORDER BY timestamp DESC
        "#,
    )
    .bind(asset_id)
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(exchanges)
}

/// Creates a new exchange in the database
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `new_exchange` - The exchange to create
///
/// # Returns
/// The created exchange with its assigned ID
///
/// # Errors
/// Returns an error if the query fails
pub async fn create_exchange(pool: &DbPool, new_exchange: &NewExchange) -> Result<Exchange> {
    let exchange = sqlx::query_as::<_, Exchange>(
        r#"
        INSERT INTO market.exchanges (
            slot, signature, index, timestamp, side, buyer, seller, asset, pair, price, size, volume, fee, buddy
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
        )
        RETURNING id, slot, signature, index, timestamp, side, buyer, seller, asset, pair, price, size, volume, fee, buddy
        "#,
    )
    .bind(new_exchange.slot)
    .bind(&new_exchange.signature.as_str())
    .bind(new_exchange.index)
    .bind(new_exchange.timestamp)
    .bind(&new_exchange.side.as_str())
    .bind(new_exchange.buyer)
    .bind(new_exchange.seller)
    .bind(new_exchange.asset)
    .bind(new_exchange.pair)
    .bind(new_exchange.price)
    .bind(new_exchange.size)
    .bind(new_exchange.volume)
    .bind(new_exchange.fee)
    .bind(new_exchange.buddy)
    .fetch_one(pool)
    .await
    .map_err(DbError::SqlxError)?;

    Ok(exchange)
}

/// Creates a new exchange in the database with its dependent entities
///
/// This function ensures that the buyer, seller, asset, and pair entities exist
/// in their respective tables before creating the exchange. If they don't exist,
/// they will be created.
///
/// # Arguments
/// * `pool` - The database connection pool
/// * `exchange_data` - The exchange data with its dependent entities
///
/// # Returns
/// The created exchange with its assigned ID
///
/// # Errors
/// Returns an error if any of the database operations fail
pub async fn create_exchange_with_dependencies(
    pool: &DbPool,
    exchange_data: &ExchangeWithDependencies,
) -> Result<Exchange> {
    // Get or create buyer
    let buyer = get_or_create_player(
        pool,
        exchange_data.buyer_wallet.clone(),
        exchange_data.timestamp,
    )
    .await?;

    // Get or create seller
    let seller = get_or_create_player(
        pool,
        exchange_data.seller_wallet.clone(),
        exchange_data.timestamp,
    )
    .await?;

    // Get or create asset token
    let asset =
        get_or_create_token(pool, exchange_data.asset_mint.clone(), None, None, None).await?;

    // Get or create pair token
    let pair = get_or_create_token(pool, exchange_data.pair_mint.clone(), None, None, None).await?;

    // Create the exchange
    let new_exchange = NewExchange {
        slot: exchange_data.slot,
        signature: exchange_data.signature.clone(),
        index: exchange_data.index,
        timestamp: exchange_data.timestamp,
        side: exchange_data.side.clone(),
        buyer: buyer.id,
        seller: seller.id,
        asset: asset.id,
        pair: pair.id,
        price: exchange_data.price,
        size: exchange_data.size,
        volume: exchange_data.volume,
        fee: exchange_data.fee,
        buddy: exchange_data.buddy,
    };

    create_exchange(pool, &new_exchange).await
}

/// Helper function to get a player by wallet address or create a new one if it doesn't exist
async fn get_or_create_player(
    pool: &DbPool,
    wallet_address: String,
    timestamp: DateTime<Utc>,
) -> Result<Player> {
    // Try to get the player by wallet address
    if let Some(player) = staratlas::get_player_by_wallet_address(pool, &wallet_address).await? {
        return Ok(player);
    }

    // Player doesn't exist, create a new one
    let new_player = NewPlayer {
        wallet_address,
        username: None,
        first_seen: timestamp,
        last_active: timestamp,
    };

    staratlas::create_player(pool, &new_player).await
}

/// Helper function to get a token by mint address or create a new one if it doesn't exist
async fn get_or_create_token(
    pool: &DbPool,
    mint: String,
    name: Option<String>,
    symbol: Option<String>,
    token_type: Option<String>,
) -> Result<Token> {
    // Try to find the token by mint address
    let tokens = sqlx::query_as::<_, Token>(
        r#"
        SELECT id, mint, name, symbol, token_type
        FROM staratlas.tokens
        WHERE mint = $1
        "#,
    )
    .bind(&mint.as_str())
    .fetch_all(pool)
    .await
    .map_err(DbError::SqlxError)?;

    if let Some(token) = tokens.into_iter().next() {
        return Ok(token);
    }

    // Token doesn't exist, create a new one
    let new_token = NewToken {
        mint,
        name,
        symbol,
        token_type,
    };

    staratlas::create_token(pool, &new_token).await
}
