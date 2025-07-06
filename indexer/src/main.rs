use crate::args::Args;
use chrono::DateTime;
use clap::Parser;
use db::{Direction, NewIndexer, NewProgramSignature, NewSignature, UpdateIndexer};
use solana_client::client_error::ClientError;
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
const GAP_FILL_LIMIT: usize = 100;
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    use chrono::Utc;
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

    log::info!("> Indexer: [{}] [{}]", indexer_name, program_id);

    // ----------- STEP 1: GAP FILL if direction is UP -----------
    let mut gap_filled_count = 0;
    if db_indexer.direction == Direction::UP {
        if let Some(ref last_sig) = db_indexer.signature {
            log::info!("Performing gap fill to ensure no missed signatures...");

            let mut before: Option<Signature> = None;
            let mut caught_up = false;

            while !caught_up {
                let config = GetConfirmedSignaturesForAddress2Config {
                    before,
                    until: None,
                    limit: Some(GAP_FILL_LIMIT),
                    commitment: CommitmentConfig::finalized().into(),
                };

                let signatures =
                    client.get_signatures_for_address_with_config(&program_id, config)?;

                if signatures.is_empty() {
                    break;
                }

                for sig_info in &signatures {
                    if sig_info.signature == *last_sig {
                        caught_up = true;
                        break;
                    }
                    gap_filled_count += 1;

                    db::create_signature(
                        &pool,
                        &NewSignature {
                            signature: sig_info.signature.to_string(),
                            slot: sig_info.slot as i64,
                            timestamp: DateTime::from_timestamp(sig_info.block_time.unwrap(), 0)
                                .unwrap(),
                        },
                    )
                    .await?;

                    db::create_program_signature(
                        &pool,
                        &NewProgramSignature {
                            program_id: program_id.to_string(),
                            signature: sig_info.signature.to_string(),
                            processed: false,
                        },
                    )
                    .await?;
                }

                before = Some(Signature::from_str(&signatures.last().unwrap().signature)?);

                // If less than limit, we've hit the beginning of available data.
                if signatures.len() < GAP_FILL_LIMIT {
                    break;
                }
            }

            log::info!(
                "Gap fill complete [{}]. Now polling for new signatures.",
                gap_filled_count
            );
        }
    }

    if gap_filled_count > 0 {
        // Fetch the latest (highest slot) signature for the program from your DB
        if let Some(last_signature) =
            db::get_newest_program_signature_by_program_id(&pool, &program_id.to_string()).await?
        {
            let latest_signature = last_signature.signature.clone();

            db::update_indexer(
                &pool,
                indexer_name.clone(),
                &UpdateIndexer {
                    signature: Some(latest_signature),
                    block: None,
                    timestamp: None,
                    direction: None,
                    finished: None,
                    fetch_limit: None,
                },
            )
            .await?;
        }
    }

    // ----------- STEP 2: MAIN POLLING LOOP -----------
    loop {
        let db_indexer = db::get_indexer_by_name(&pool, &indexer_name).await?;

        let mut before_signature = None;
        let mut until_signature = None;

        match db_indexer.direction {
            Direction::UP => {
                until_signature = match db_indexer.signature {
                    None => None,
                    Some(ref signature) => Some(Signature::from_str(signature.as_str())?),
                };
            }
            Direction::DOWN => {
                before_signature = match db_indexer.signature {
                    None => {
                        // If signature is None, get the latest signature from DB as starting point
                        Some(Signature::from_str(
                            &db::get_oldest_program_signature_by_program_id(
                                &pool,
                                &program_id.to_string(),
                            )
                            .await?
                            .unwrap()
                            .signature,
                        )?)
                    }
                    Some(ref signature) => Some(Signature::from_str(signature.as_str())?),
                };
            }
        }

        let signatures = rpc_with_retry(
            || {
                let signatures_for_config = GetConfirmedSignaturesForAddress2Config {
                    before: before_signature,
                    until: until_signature,
                    limit: Some(db_indexer.fetch_limit as usize),
                    commitment: CommitmentConfig::finalized().into(),
                };
                client.get_signatures_for_address_with_config(&program_id, signatures_for_config)
            },
            5, // max_attempts
        )
        .await?;

        for sig_info in signatures.clone() {
            db::create_signature(
                &pool,
                &NewSignature {
                    signature: sig_info.signature.to_string(),
                    slot: sig_info.slot as i64,
                    timestamp: DateTime::from_timestamp(sig_info.block_time.unwrap(), 0).unwrap(),
                },
            )
            .await?;
            db::create_program_signature(
                &pool,
                &NewProgramSignature {
                    program_id: program_id.to_string(),
                    signature: sig_info.signature.to_string(),
                    processed: false,
                },
            )
            .await?;

            let new_indexer = match db_indexer.direction {
                Direction::UP => UpdateIndexer {
                    direction: None,
                    signature: Some(signatures.first().unwrap().clone().signature),
                    block: Some(signatures.first().unwrap().clone().slot as i64),
                    timestamp: Some(
                        DateTime::from_timestamp(sig_info.block_time.unwrap(), 0).unwrap(),
                    ),
                    finished: Some(false),
                    fetch_limit: None,
                },
                Direction::DOWN => UpdateIndexer {
                    direction: None,
                    signature: Some(sig_info.signature.clone()),
                    block: Some(sig_info.slot as i64),
                    timestamp: Some(
                        DateTime::from_timestamp(sig_info.block_time.unwrap(), 0).unwrap(),
                    ),
                    finished: Some(false),
                    fetch_limit: None,
                },
            };

            match db_indexer.direction {
                Direction::UP => match db_indexer.block {
                    None => {
                        db::update_indexer(&pool, db_indexer.name.clone(), &new_indexer).await?;
                    }
                    Some(until_block) => {
                        if until_block < sig_info.slot as i64 {
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

        if signatures.is_empty() {
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

async fn rpc_with_retry<F, T>(mut f: F, max_attempts: usize) -> Result<T, ClientError>
where
    F: FnMut() -> Result<T, ClientError>,
{
    let mut attempt = 0;
    loop {
        match f() {
            Ok(val) => return Ok(val),
            Err(e) => {
                attempt += 1;
                if attempt >= max_attempts {
                    eprintln!(
                        "[ERROR] RPC error: {}. Max attempts ({}) reached. Giving up.",
                        e, max_attempts
                    );
                    return Err(e); // Or anyhow::Error::from(e) if you prefer
                }
                let wait = std::cmp::min(30, attempt * 3);
                eprintln!(
                    "[WARN] RPC error: {}. Attempt {}/{}. Retrying in {}s...",
                    e, attempt, max_attempts, wait
                );
                sleep(Duration::from_secs(wait as u64)).await;
            }
        }
    }
}
