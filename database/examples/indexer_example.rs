//! Example demonstrating how to use the database library to interact with the indexer table

use db::{
    create_indexer, delete_indexer, establish_connection, get_all_indexers, get_indexer_by_id,
    get_indexers_by_program_id, update_indexer, NewIndexer, UpdateIndexer,
};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature as SolanaSignature;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logger
    env_logger::init();

    // Establish database connection
    let pool = establish_connection().await?;
    println!("Connected to database");

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
    println!("Created indexer: {:?}", created_indexer);

    // Get all indexers
    let all_indexers = get_all_indexers(&pool).await?;
    println!("All indexers: {:?}", all_indexers);

    // Get indexer by ID
    let indexer_id = created_indexer.id;
    let indexer = get_indexer_by_id(&pool, indexer_id).await?;
    println!("Indexer by ID: {:?}", indexer);

    // Get indexers by program ID
    let program_id_key = PublicKey(program_id.to_string());
    let program_indexers = get_indexers_by_program_id(&pool, &program_id_key).await?;
    println!("Indexers by program ID: {:?}", program_indexers);

    // Update indexer
    let update = UpdateIndexer {
        name: Some("Updated Example Indexer".to_string()),
        before_signature: None,
        until_block: Some(200),
        finished: Some(true),
    };

    let updated_indexer = update_indexer(&pool, indexer_id, &update).await?;
    println!("Updated indexer: {:?}", updated_indexer);

    // Delete indexer
    let deleted = delete_indexer(&pool, indexer_id).await?;
    println!("Indexer deleted: {}", deleted);

    Ok(())
}
