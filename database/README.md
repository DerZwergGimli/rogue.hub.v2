# Database Library for rogue.hub.v2

A well-structured Rust library for interacting with the PostgreSQL database used by the rogue.hub.v2 project, with a focus on indexer and signature data retrieval.

## Features

- Connection management with connection pooling
- Comprehensive error handling
- Type-safe database interactions using sqlx
- Support for all CRUD operations on the indexer and signatures tables
- Conversion between Solana SDK types and database types
- Well-organized code structure with models and queries in separate modules

## Usage

### Connection Management

```rust
use db::establish_connection;

async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Establish database connection
    let pool = establish_connection().await?;

    // Use the connection pool for database operations
    // ...

    Ok(())
}
```

### Retrieving Indexers

```rust
use db::{establish_connection, get_all_indexers, get_indexer_by_id, get_indexers_by_program_id, PublicKey};

async fn main() -> anyhow::Result<()> {
    let pool = establish_connection().await?;

    // Get all indexers
    let all_indexers = get_all_indexers(&pool).await?;

    // Get indexer by ID
    let indexer = get_indexer_by_id(&pool, 1).await?;

    // Get indexers by program ID
    let program_id = PublicKey("traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg".to_string());
    let program_indexers = get_indexers_by_program_id(&pool, &program_id).await?;

    Ok(())
}
```

### Creating an Indexer

```rust
use db::{establish_connection, create_indexer, NewIndexer, PublicKey, Signature};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature as SolanaSignature;
use std::str::FromStr;

async fn main() -> anyhow::Result<()> {
    let pool = establish_connection().await?;

    // Create a new indexer
    let program_id = Pubkey::from_str("traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg")?;
    let start_signature = SolanaSignature::from_str(
        "5DBmWTHaz8VfgKbNrWs66ofUcxjvUGwcK1grZMog8fD4wfqpUuceCbMCY9Rs7iLssyFwL5LzPxLypgs5R5iyVwNr",
    )?;

    let new_indexer = NewIndexer {
        name: Some("Example Indexer".to_string()),
        program_id: program_id.into(),
        start_signature: start_signature.into(),
        before_signature: None,
        start_block: Some(100),
        before_block: None,
        finished: Some(false),
    };

    let created_indexer = create_indexer(&pool, &new_indexer).await?;

    Ok(())
}
```

### Updating an Indexer

```rust
use db::{establish_connection, update_indexer, UpdateIndexer};

async fn main() -> anyhow::Result<()> {
    let pool = establish_connection().await?;

    // Update an indexer
    let update = UpdateIndexer {
        name: Some("Updated Example Indexer".to_string()),
        before_signature: None,
        before_block: Some(200),
        finished: Some(true),
    };

    let updated_indexer = update_indexer(&pool, 1, &update).await?;

    Ok(())
}
```

### Deleting an Indexer

```rust
use db::{establish_connection, delete_indexer};

async fn main() -> anyhow::Result<()> {
    let pool = establish_connection().await?;

    // Delete an indexer
    let deleted = delete_indexer(&pool, 1).await?;

    Ok(())
}
```

### Working with Signatures

The library also provides functionality for working with the signatures table.

#### Retrieving Signatures

```rust
use db::{
    establish_connection, get_all_signatures, get_signature_by_id,
    get_signatures_by_program_id, get_signatures_by_signature, get_signatures_by_timestamp_range,
    PublicKey, Signature,
};
use sqlx::types::time::OffsetDateTime;
use std::time::{SystemTime, Duration};

async fn main() -> anyhow::Result<()> {
    let pool = establish_connection().await?;

    // Get all signatures
    let all_signatures = get_all_signatures(&pool).await?;

    // Get signature by ID
    let signature = get_signature_by_id(&pool, 1).await?;

    // Get signatures by program ID
    let program_id = "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg".to_string();
    // Get all signatures for the program
    let program_signatures = get_signatures_by_program_id(&pool, &program_id, None).await?;

    // Get only the 10 most recent signatures for the program
    let limited_signatures = get_signatures_by_program_id(&pool, &program_id, Some(10)).await?;

    // Get signatures by signature value
    let signature_value = "5DBmWTHaz8VfgKbNrWs66ofUcxjvUGwcK1grZMog8fD4wfqpUuceCbMCY9Rs7iLssyFwL5LzPxLypgs5R5iyVwNr".to_string();
    let signature_matches = get_signatures_by_signature(&pool, &signature_value).await?;

    // Get signatures by date range
    let now = SystemTime::now();
    let one_hour_ago = now - Duration::from_secs(3600);
    let one_hour_ago_offset = OffsetDateTime::from(one_hour_ago);
    let one_hour_later = now + Duration::from_secs(3600);
    let one_hour_later_offset = OffsetDateTime::from(one_hour_later);

    let date_range_signatures = get_signatures_by_timestamp_range(
        &pool, 
        one_hour_ago_offset, 
        one_hour_later_offset
    ).await?;

    Ok(())
}
```

#### Creating a Signature

```rust
use db::{establish_connection, create_signature, NewSignature};
use sqlx::types::time::OffsetDateTime;
use std::time::SystemTime;

async fn main() -> anyhow::Result<()> {
    let pool = establish_connection().await?;

    // Create a new signature
    let program_id = "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg".to_string();
    let signature_value = "5DBmWTHaz8VfgKbNrWs66ofUcxjvUGwcK1grZMog8fD4wfqpUuceCbMCY9Rs7iLssyFwL5LzPxLypgs5R5iyVwNr".to_string();

    // Get current time
    let now = SystemTime::now();
    let now_offset = OffsetDateTime::from(now);

    let new_signature = NewSignature {
        program_id,
        signature: signature_value,
        slot: 12345,
        date: now_offset,
    };

    let created_signature = create_signature(&pool, &new_signature).await?;

    Ok(())
}
```

#### Deleting Signatures

```rust
use db::{establish_connection, delete_signature, delete_signatures_by_program_id};

async fn main() -> anyhow::Result<()> {
    let pool = establish_connection().await?;

    // Delete a signature by ID
    let deleted = delete_signature(&pool, 1).await?;

    // Delete signatures by program ID
    let program_id = "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg".to_string();
    let deleted_count = delete_signatures_by_program_id(&pool, &program_id).await?;

    Ok(())
}
```

## Configuration

The library uses the `DATABASE_URL` environment variable to connect to the database. This can be set in a `.env` file in the project root or in the environment.

Example `.env` file:

```
DATABASE_URL="postgres://db:localtest@localhost:5432/datahub"
```

## Running the Examples

Examples demonstrating the usage of the library are provided in the `examples` directory:

### Indexer Example

To run the indexer example:

```bash
cargo run --example indexer_example
```

### Signature Example

To run the signature example:

```bash
cargo run --example signature_example
```

Make sure the database is running and accessible before running the examples.
