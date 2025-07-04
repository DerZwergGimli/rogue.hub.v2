use crate::args::Args;
use chrono::DateTime;
use clap::Parser;
use db::{Direction, NewProgramSignature, NewSignature, UpdateIndexer};
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::env;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

mod args;

const SLEEP: Duration = Duration::from_secs(5);

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let indexer_name = String::from(env::var("INDEXER_NAME").expect("RPC_URL must be set"));

    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    let pool = db::establish_connection().await?;

    let db_indexers = db::get_indexers_by_name(&pool, &indexer_name).await?;

    let client = RpcClient::new_with_commitment(
        String::from(env::var("RPC_URL").expect("RPC_URL must be set")),
        CommitmentConfig::confirmed(),
    );

    let indexer_id = db_indexers[0].id;
    let program_id = Pubkey::from_str(db_indexers[0].program_id.as_str())?;

    log::info!("Program_ID = {}", program_id);

    loop {
        let db_indexer = db::get_indexer_by_id(&pool, indexer_id).await?.unwrap();

        let mut before_signature = None;
        let mut until_signature = None;

        match db_indexer.direction {
            Direction::New => {
                until_signature = match db_indexer.until_signature {
                    None => None,
                    Some(signature) => Some(Signature::from_str(signature.as_str())?),
                };
            }
            Direction::Old => {
                before_signature = match db_indexer.before_signature {
                    None => Some(Signature::from_str(
                        &db::get_last_program_signature_by_program_id(
                            &pool,
                            &program_id.to_string(),
                        )
                        .await?
                        .unwrap()
                        .signature,
                    )?),
                    Some(signature) => Some(Signature::from_str(signature.as_str())?),
                };
            }
        }

        let signatures_for_config = GetConfirmedSignaturesForAddress2Config {
            before: before_signature,
            until: until_signature,
            limit: Some(db_indexer.fetch_limit as usize),
            commitment: CommitmentConfig::finalized().into(),
        };

        let signatures =
            client.get_signatures_for_address_with_config(&program_id, signatures_for_config)?;

        for signature in signatures.clone() {
            db::create_signature(
                &pool,
                &NewSignature {
                    signature: signature.signature.to_string(),
                    slot: signature.slot as i64,
                    timestamp: DateTime::from_timestamp(signature.block_time.unwrap(), 0).unwrap(),
                },
            )
            .await?;
            db::create_program_signature(
                &pool,
                &NewProgramSignature {
                    program_id: program_id.to_string(),
                    signature: signature.signature.to_string(),
                    processed: false,
                },
            )
            .await?;

            let new_indexer = match db_indexer.direction {
                Direction::New => {
                    UpdateIndexer {
                        direction: None, // Using None to keep the existing direction
                        before_signature: None,
                        until_signature: Some(signature.signature.clone()),
                        before_block: None,
                        until_block: Some(signature.slot as i64),
                        finished: Some(false),
                        fetch_limit: None, // Keep the existing fetch_limit
                    }
                }
                Direction::Old => {
                    UpdateIndexer {
                        direction: None, // Using None to keep the existing direction
                        before_signature: Some(signature.signature.clone()),
                        until_signature: None,
                        before_block: Some(signature.slot as i64),
                        until_block: None,
                        finished: Some(false),
                        fetch_limit: None, // Keep the existing fetch_limit
                    }
                }
            };

            match db_indexer.direction {
                Direction::New => {
                    let temp_indexer = db::get_indexer_by_id(&pool, db_indexer.id).await?.unwrap();
                    match temp_indexer.until_block {
                        None => {
                            db::update_indexer(&pool, db_indexer.id, &new_indexer).await?;
                        }
                        Some(until_block) => {
                            if until_block < signature.slot as i64 {
                                db::update_indexer(&pool, db_indexer.id, &new_indexer).await?;
                            }
                        }
                    }
                }
                Direction::Old => {
                    db::update_indexer(&pool, db_indexer.id, &new_indexer).await?;
                }
            }
        }

        if signatures.len() == 0 {
            log::info!(
                "[{:?}] no new signatures for {}",
                db_indexer.name,
                program_id
            );
            return Ok(());
        }

        log::info!(
            "[{:?}] added {} signatures for {}",
            db_indexer.name,
            signatures.len(),
            program_id
        );

        sleep(SLEEP).await;
    }
}
