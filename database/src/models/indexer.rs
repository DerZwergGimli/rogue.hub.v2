//! Models for the indexer table

use sqlx::FromRow;

use crate::types::{Direction, PublicKeyType, SignatureType};

/// Represents an indexer record in the indexer.indexer table
#[derive(Debug, FromRow, Clone)]
pub struct Indexer {
    /// Unique identifier for the indexer
    pub id: i32,

    /// Name of the indexer
    pub name: Option<String>,

    /// Direction of indexing (OLD or NEW)
    pub direction: Direction,

    /// Program ID being indexed
    pub program_id: PublicKeyType,

    pub before_signature: Option<SignatureType>,

    pub until_signature: Option<SignatureType>,

    pub before_block: Option<i64>,

    pub until_block: Option<i64>,

    /// Whether the indexing is finished
    pub finished: Option<bool>,

    /// Maximum number of signatures to fetch in a single request
    pub fetch_limit: i32,
}

/// Parameters for creating a new indexer in the indexer.indexer table
#[derive(Debug)]
pub struct NewIndexer {
    /// ID of the indexer
    pub id: i32,

    /// Name of the indexer
    pub name: Option<String>,

    /// Direction of indexing (OLD or NEW)
    pub direction: Direction,

    /// Program ID being indexed
    pub program_id: PublicKeyType,

    pub before_signature: Option<SignatureType>,

    pub until_signature: Option<SignatureType>,

    pub before_block: Option<i64>,

    pub until_block: Option<i64>,

    /// Whether the indexing is finished
    pub finished: Option<bool>,

    /// Maximum number of signatures to fetch in a single request
    pub fetch_limit: i32,
}

/// Parameters for updating an existing indexer in the indexer.indexer table
#[derive(Debug)]
pub struct UpdateIndexer {
    /// Direction of indexing (OLD or NEW)
    pub direction: Option<Direction>,

    pub before_signature: Option<SignatureType>,

    pub until_signature: Option<SignatureType>,

    pub before_block: Option<i64>,

    pub until_block: Option<i64>,

    pub finished: Option<bool>,

    /// Maximum number of signatures to fetch in a single request
    pub fetch_limit: Option<i32>,
}
