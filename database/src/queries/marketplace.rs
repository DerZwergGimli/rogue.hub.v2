//! Database queries for the market schema

use crate::connection::DbPool;
use crate::error::{DbError, Result};
use crate::models::{Exchange, NewExchange};

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
        SELECT id, block, signature, index, date, side, buyer, seller, asset, pair, price, size, volume, fee, buddy
        FROM market.exchanges
        ORDER BY date DESC
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
        SELECT id, block, signature, index, date, side, buyer, seller, asset, pair, price, size, volume, fee, buddy
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
        SELECT id, block, signature, index, date, side, buyer, seller, asset, pair, price, size, volume, fee, buddy
        FROM market.exchanges
        WHERE buyer = $1
        ORDER BY date DESC
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
        SELECT id, block, signature, index, date, side, buyer, seller, asset, pair, price, size, volume, fee, buddy
        FROM market.exchanges
        WHERE seller = $1
        ORDER BY date DESC
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
        SELECT id, block, signature, index, date, side, buyer, seller, asset, pair, price, size, volume, fee, buddy
        FROM market.exchanges
        WHERE asset = $1
        ORDER BY date DESC
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
            block, signature, index, date, side, buyer, seller, asset, pair, price, size, volume, fee, buddy
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
        )
        RETURNING id, block, signature, index, date, side, buyer, seller, asset, pair, price, size, volume, fee, buddy
        "#,
    )
    .bind(new_exchange.block)
    .bind(&new_exchange.signature)
    .bind(new_exchange.index)
    .bind(new_exchange.date)
    .bind(&new_exchange.side)
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
