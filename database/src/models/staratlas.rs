//! Models for the staratlas schema

use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};

/// Represents a token record in the staratlas.tokens table
#[derive(Debug, FromRow, Clone)]
pub struct Token {
    /// Unique identifier for the token
    pub id: i32,

    /// Mint address of the token
    pub mint: String,

    /// Name of the token
    pub name: Option<String>,

    /// Symbol of the token
    pub symbol: Option<String>,

    /// Type of the token
    pub token_type: Option<String>,
}

/// Parameters for creating a new token
#[derive(Debug)]
pub struct NewToken {
    /// Mint address of the token
    pub mint: String,

    /// Name of the token
    pub name: Option<String>,

    /// Symbol of the token
    pub symbol: Option<String>,

    /// Type of the token
    pub token_type: Option<String>,
}

/// Represents a player record in the staratlas.players table
#[derive(Debug, FromRow, Clone)]
pub struct Player {
    /// Unique identifier for the player
    pub id: i32,

    /// Wallet address of the player
    pub wallet_address: String,

    /// Username of the player
    pub username: Option<String>,

    /// When the player was first seen
    pub first_seen: DateTime<Utc>,

    /// When the player was last active
    pub last_active: DateTime<Utc>,
}

/// Parameters for creating a new player
#[derive(Debug)]
pub struct NewPlayer {
    /// Wallet address of the player
    pub wallet_address: String,

    /// Username of the player
    pub username: Option<String>,

    /// When the player was first seen
    pub first_seen: DateTime<Utc>,

    /// When the player was last active
    pub last_active: DateTime<Utc>,
}
