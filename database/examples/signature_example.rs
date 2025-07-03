//! Example demonstrating how to use the database library to interact with the signatures table

use db::{
    create_signature, delete_signature, delete_signatures_by_program_id, establish_connection, get_all_signatures,
    get_signature_by_id, get_signatures_by_date_range, get_signatures_by_program_id, get_signatures_by_signature,
    NewSignature, PublicKey, Signature,
};
use sqlx::types::time::OffsetDateTime;
use std::time::{Duration, SystemTime};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logger
    env_logger::init();

    // Establish database connection
    let pool = establish_connection().await?;
    println!("Connected to database");

    // Create a new signature
    let program_id = "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg".to_string();
    let signature_value =
        "5DBmWTHaz8VfgKbNrWs66ofUcxjvUGwcK1grZMog8fD4wfqpUuceCbMCY9Rs7iLssyFwL5LzPxLypgs5R5iyVwNr"
            .to_string();

    // Get current time
    let now = SystemTime::now();
    let now_offset = OffsetDateTime::from(now);

    let new_signature = NewSignature {
        program_id,
        signature: signature_value.clone(),
        slot: 12345,
        timestamp: now_offset,
    };

    let created_signature = create_signature(&pool, &new_signature).await?;
    println!("Created signature: {:?}", created_signature);

    // Get all signatures
    let all_signatures = get_all_signatures(&pool).await?;
    println!("All signatures: {:?}", all_signatures);

    // Get signature by ID
    let signature_id = created_signature.id;
    let signature = get_signature_by_id(&pool, signature_id).await?;
    println!("Signature by ID: {:?}", signature);

    // Get signatures by program ID
    let program_id_key = "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg".to_string();
    let program_signatures = get_signatures_by_program_id(&pool, &program_id_key).await?;
    println!("Signatures by program ID: {:?}", program_signatures);

    // Get signatures by signature value
    let signature_value_key = signature_value;
    let signature_matches = get_signatures_by_signature(&pool, &signature_value_key).await?;
    println!("Signatures by signature value: {:?}", signature_matches);

    // Get signatures by date range
    let one_hour_ago = now - Duration::from_secs(3600);
    let one_hour_ago_offset = OffsetDateTime::from(one_hour_ago);
    let one_hour_later = now + Duration::from_secs(3600);
    let one_hour_later_offset = OffsetDateTime::from(one_hour_later);

    let date_range_signatures =
        get_signatures_by_date_range(&pool, one_hour_ago_offset, one_hour_later_offset).await?;
    println!("Signatures in date range: {:?}", date_range_signatures);

    // Delete signature
    let deleted = delete_signature(&pool, signature_id).await?;
    println!("Signature deleted: {}", deleted);

    // Create another signature to test delete by program ID
    let new_signature2 = NewSignature {
        program_id: "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg".to_string(),
        signature: "AnotherSignature123456789".to_string(),
        slot: 54321,
        date: now_offset,
    };

    let created_signature2 = create_signature(&pool, &new_signature2).await?;
    println!("Created another signature: {:?}", created_signature2);

    // Delete signatures by program ID
    let program_id_key = "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg".to_string();
    let deleted_count = delete_signatures_by_program_id(&pool, &program_id_key).await?;
    println!("Deleted {} signatures by program ID", deleted_count);

    Ok(())
}
