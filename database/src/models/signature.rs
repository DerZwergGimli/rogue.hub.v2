//! Models for the signatures table

use crate::types::{PublicKeyType, SignatureType};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Represents a signature record in the database
#[derive(Debug, FromRow)]
pub struct Signature {
    /// Unique identifier for the signature
    pub id: i32,

    /// Program ID associated with the signature
    pub program_id: PublicKeyType,

    /// The signature itself
    pub signature: SignatureType,

    /// The slot number
    pub slot: i64,

    /// The timestamp of the signature
    pub timestamp: DateTime<Utc>,

    pub processed: bool,
}

/// Parameters for creating a new signature record
#[derive(Debug)]
pub struct NewSignature {
    /// Program ID associated with the signature
    pub program_id: PublicKeyType,

    /// The signature itself
    pub signature: SignatureType,

    /// The slot number
    pub slot: i64,

    /// The timestamp of the signature
    pub timestamp: DateTime<Utc>,
}
