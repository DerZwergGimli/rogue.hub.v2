//! API implementations for the Star Atlas Data API
//!
//! This module contains the API implementations for the indexer, marketplace, and Star Atlas endpoints.

mod indexer;

mod staratlas;

pub use indexer::IndexerApi;
pub use staratlas::StarAtlasApi;
