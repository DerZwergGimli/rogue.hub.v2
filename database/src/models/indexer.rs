//! Models for the indexer table

use crate::types::{Direction, PublicKeyType, SignatureType};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Represents an indexer record in the indexer.indexer table
#[derive(Debug, FromRow, Clone)]
pub struct Indexer {
    pub id: i32,
    pub name: Option<String>,
    pub direction: Direction,
    pub program_id: PublicKeyType,
    pub signature: Option<SignatureType>,
    pub block: Option<i64>,
    pub timestamp: Option<DateTime<Utc>>,
    pub finished: Option<bool>,
    pub fetch_limit: i32,
}

/// Parameters for creating a new indexer in the indexer.indexer table
#[derive(Debug)]
pub struct NewIndexer {
    pub id: i32,
    pub name: Option<String>,
    pub direction: Direction,
    pub program_id: PublicKeyType,
    pub signature: Option<SignatureType>,
    pub block: Option<i64>,
    pub timestamp: Option<DateTime<Utc>>,
    pub finished: Option<bool>,
    pub fetch_limit: i32,
}

/// Parameters for updating an existing indexer in the indexer.indexer table
#[derive(Debug)]
pub struct UpdateIndexer {
    pub direction: Option<Direction>,
    pub signature: Option<SignatureType>,
    pub block: Option<i64>,
    pub timestamp: Option<DateTime<Utc>>,
    pub finished: Option<bool>,
    pub fetch_limit: Option<i32>,
}
