//! Database library for rogue.hub.v2
//!
//! This library provides a well-structured interface to interact with the PostgreSQL database
//! used by the rogue.hub.v2 project, with a focus on indexer and signature data retrieval.

mod connection;
mod error;
mod models;
pub mod queries;
mod types;

pub use connection::{establish_connection, DbPool};
pub use error::{DbError, Result};
pub use types::*;

pub use models::*;
pub use queries::*;
