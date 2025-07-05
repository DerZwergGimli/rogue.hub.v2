//! Models for the market schema

use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};

/// Represents an exchange record in the market.exchanges table
#[derive(Debug, FromRow, Clone)]
pub struct Exchange {
    /// Unique identifier for the exchange
    pub id: i32,

    /// Block number of the exchange
    pub slot: i32,

    /// Transaction signature
    pub signature: String,

    /// Index within the transaction
    pub index: i32,

    /// Timestamp of the exchange
    pub timestamp: DateTime<Utc>,

    /// Side of the exchange (buy/sell)
    pub side: String,

    /// Buyer ID (references staratlas.players)
    pub buyer: i32,

    /// Seller ID (references staratlas.players)
    pub seller: i32,

    /// Asset ID (references staratlas.tokens)
    pub asset: i32,

    /// Pair ID (references staratlas.tokens)
    pub pair: i32,

    /// Price of the exchange
    pub price: f64,

    /// Size of the exchange
    pub size: i32,

    /// Volume of the exchange
    pub volume: f64,

    /// Fee of the exchange
    pub fee: f64,

    /// Buddy fee of the exchange
    pub buddy: f64,
}

/// Parameters for creating a new exchange
#[derive(Debug)]
pub struct NewExchange {
    /// Block number of the exchange
    pub slot: i32,

    /// Transaction signature
    pub signature: String,

    /// Index within the transaction
    pub index: i32,

    /// Timestamp of the exchange
    pub timestamp: DateTime<Utc>,

    /// Side of the exchange (buy/sell)
    pub side: String,

    /// Buyer ID (references staratlas.players)
    pub buyer: i32,

    /// Seller ID (references staratlas.players)
    pub seller: i32,

    /// Asset ID (references staratlas.tokens)
    pub asset: i32,

    /// Pair ID (references staratlas.tokens)
    pub pair: i32,

    /// Price of the exchange
    pub price: f64,

    /// Size of the exchange
    pub size: i32,

    /// Volume of the exchange
    pub volume: f64,

    /// Fee of the exchange
    pub fee: f64,

    /// Buddy fee of the exchange
    pub buddy: f64,
}

/// Parameters for creating a new exchange with its dependent entities
#[derive(Debug)]
pub struct ExchangeWithDependencies {
    /// Block number of the exchange
    pub slot: i32,

    /// Transaction signature
    pub signature: String,

    /// Index within the transaction
    pub index: i32,

    /// Timestamp of the exchange
    pub timestamp: DateTime<Utc>,

    /// Side of the exchange (buy/sell)
    pub side: String,

    /// Wallet address of the buyer
    pub buyer_wallet: String,

    /// Wallet address of the seller
    pub seller_wallet: String,

    /// Mint address of the asset token
    pub asset_mint: String,

    /// Mint address of the pair token
    pub pair_mint: String,

    /// Price of the exchange
    pub price: f64,

    /// Size of the exchange
    pub size: i32,

    /// Volume of the exchange
    pub volume: f64,

    /// Fee of the exchange
    pub fee: f64,

    /// Buddy fee of the exchange
    pub buddy: f64,
}
