//! Models for the indexer.signatures and indexer.program_signatures tables

use crate::types::{PublicKeyType, SignatureType};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Represents a signature record in the indexer.signatures table
#[derive(Debug, FromRow, Clone)]
pub struct Signature {
    /// The signature itself
    pub signature: SignatureType,

    /// The slot number
    pub slot: i64,

    /// The timestamp of the signature
    pub timestamp: DateTime<Utc>,
}

/// Parameters for creating a new signature record in the indexer.signatures table
#[derive(Debug)]
pub struct NewSignature {
    /// The signature itself
    pub signature: SignatureType,

    /// The slot number
    pub slot: i64,

    /// The timestamp of the signature
    pub timestamp: DateTime<Utc>,
}

/// Represents a program signature record in the indexer.program_signatures table
#[derive(Debug, FromRow, Clone)]
pub struct ProgramSignature {
    /// Program ID associated with the signature
    pub program_id: PublicKeyType,

    /// The signature itself
    pub signature: SignatureType,

    /// Whether the signature has been processed
    pub processed: bool,
}

/// Parameters for creating a new program signature record in the indexer.program_signatures table
#[derive(Debug)]
pub struct NewProgramSignature {
    /// Program ID associated with the signature
    pub program_id: PublicKeyType,

    /// The signature itself
    pub signature: SignatureType,

    /// Whether the signature has been processed
    pub processed: bool,
}

/// Represents a program record in the indexer.programs table
#[derive(Debug, FromRow, Clone)]
pub struct Program {
    /// Program ID
    pub program_id: PublicKeyType,
}

/// Parameters for creating a new program record in the indexer.programs table
#[derive(Debug)]
pub struct NewProgram {
    /// Program ID
    pub program_id: PublicKeyType,
}
