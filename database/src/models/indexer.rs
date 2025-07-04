//! Models for the indexer table

use sqlx::FromRow;

use crate::types::{PublicKeyType, SignatureType};

/// Represents an indexer record in the indexer.indexer table
#[derive(Debug, FromRow, Clone)]
pub struct Indexer {
    /// Unique identifier for the indexer
    pub id: i32,

    /// Name of the indexer
    pub name: Option<String>,

    /// Direction of indexing (forward or backward)
    pub direction: String,

    /// Program ID being indexed
    pub program_id: PublicKeyType,

    /// Signature to start indexing from
    pub start_signature: Option<SignatureType>,

    /// Signature to stop indexing at (if any)
    pub before_signature: Option<SignatureType>,

    /// Block number to start indexing from
    pub start_block: Option<i64>,

    /// Block number to stop indexing at (if any)
    pub before_block: Option<i64>,

    /// Whether the indexing is finished
    pub finished: Option<bool>,
}

/// Parameters for creating a new indexer in the indexer.indexer table
#[derive(Debug)]
pub struct NewIndexer {
    /// ID of the indexer
    pub id: i32,

    /// Name of the indexer
    pub name: Option<String>,

    /// Direction of indexing (forward or backward)
    pub direction: String,

    /// Program ID being indexed
    pub program_id: PublicKeyType,

    /// Signature to start indexing from
    pub start_signature: Option<SignatureType>,

    /// Signature to stop indexing at (if any)
    pub before_signature: Option<SignatureType>,

    /// Block number to start indexing from
    pub start_block: Option<i64>,

    /// Block number to stop indexing at (if any)
    pub before_block: Option<i64>,

    /// Whether the indexing is finished
    pub finished: Option<bool>,
}

/// Parameters for updating an existing indexer in the indexer.indexer table
#[derive(Debug)]
pub struct UpdateIndexer {
    /// Direction of indexing (forward or backward)
    pub direction: Option<String>,

    /// Signature to stop indexing at (if any)
    pub before_signature: Option<SignatureType>,

    /// Block number to stop indexing at (if any)
    pub before_block: Option<i64>,

    /// Whether the indexing is finished
    pub finished: Option<bool>,
}
