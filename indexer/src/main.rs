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

    let indexer_name_to_use =
        String::from(env::var("INDEXER_NAME").expect("INDEXER_NAME must be set"));
    let startup_delay = Duration::from_millis(
        env::var("STARTUP_DELAY")
            .unwrap_or_else(|_| "100".to_string())
            .parse::<u64>()
            .unwrap(),
    );
    sleep(startup_delay).await;

    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    let pool = db::establish_connection().await?;

    let db_indexer = match db::get_indexer_by_name(&pool, indexer_name_to_use.as_str()).await {
        Ok(indexer) => indexer,
        Err(_) => {
            log::error!("No indexer named {:?} found!", &indexer_name_to_use);
            return Ok(());
        }
    };

    let client = RpcClient::new_with_commitment(
        String::from(env::var("RPC_URL").expect("RPC_URL must be set")),
        CommitmentConfig::confirmed(),
    );

    let indexer_name = db_indexer.name;
    let program_id = Pubkey::from_str(db_indexer.program_id.as_str())?;

    log::info!("Indexer = [{}]", indexer_name);
    log::info!("Program_ID = {}", program_id);

    loop {
        let db_indexer = db::get_indexer_by_name(&pool, &indexer_name).await?;

        let mut before_signature = None;
        let mut until_signature = None;

        match db_indexer.direction {
            Direction::UP => {
                until_signature = match db_indexer.signature {
                    None => None,
                    Some(signature) => Some(Signature::from_str(signature.as_str())?),
                };
            }
            Direction::DOWN => {
                before_signature = match db_indexer.signature {
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
                Direction::UP => {
                    UpdateIndexer {
                        direction: None, // Using None to keep the existing direction
                        signature: Some(signatures.first().unwrap().clone().signature),
                        block: Some(signatures.first().unwrap().clone().slot as i64),
                        timestamp: Some(
                            DateTime::from_timestamp(signature.block_time.unwrap(), 0).unwrap(),
                        ),
                        finished: Some(false),
                        fetch_limit: None, // Keep the existing fetch_limit
                    }
                }
                Direction::DOWN => {
                    UpdateIndexer {
                        direction: None, // Using None to keep the existing direction
                        signature: Some(signature.signature),
                        block: Some(signature.slot as i64),
                        timestamp: Some(
                            DateTime::from_timestamp(signature.block_time.unwrap(), 0).unwrap(),
                        ),
                        finished: Some(false),
                        fetch_limit: None, // Keep the existing fetch_limit
                    }
                }
            };

            match db_indexer.direction {
                Direction::UP => match db_indexer.block {
                    None => {
                        db::update_indexer(&pool, db_indexer.name.clone(), &new_indexer).await?;
                    }
                    Some(until_block) => {
                        if until_block < signature.slot as i64 {
                            db::update_indexer(&pool, db_indexer.name.clone(), &new_indexer)
                                .await?;
                        }
                    }
                },
                Direction::DOWN => {
                    db::update_indexer(&pool, db_indexer.name.clone(), &new_indexer).await?;
                }
            }
        }

        if signatures.len() == 0 {
            log::info!(
                "[{:?}] no new signatures for {}",
                db_indexer.name,
                program_id
            );
            if db_indexer.direction == Direction::DOWN {
                println!("Finished all");
                return Ok(());
            }
        } else {
            log::info!(
                "[{:?}] added {} signatures for {}",
                db_indexer.name,
                signatures.len(),
                program_id
            );
        }
        sleep(SLEEP).await;
    }
}
