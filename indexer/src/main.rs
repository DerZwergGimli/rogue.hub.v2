use crate::args::Args;
use chrono::DateTime;
use clap::Parser;
use db::{NewProgramSignature, NewSignature, UpdateIndexer};
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

    let _args = Args::parse();

    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    let pool = db::establish_connection().await?;

    let db_indexers = db::get_all_indexers(&pool).await?;

    let client = RpcClient::new_with_commitment(
        String::from(env::var("RPC_URL").expect("RPC_URL must be set")),
        CommitmentConfig::confirmed(),
    );

    let indexer_id = db_indexers[0].id;
    let program_id = Pubkey::from_str(db_indexers[0].program_id.as_str())?;

    log::info!("Program_ID = {}", program_id);

    loop {
        let db_indexer = db::get_indexer_by_id(&pool, indexer_id).await?.unwrap();

        let before_signature = match db_indexer.before_signature {
            None => None,
            Some(signature) => Some(Signature::from_str(signature.as_str())?),
        };

        let signatures_for_config = GetConfirmedSignaturesForAddress2Config {
            before: before_signature,
            until: None,
            limit: Some(100),
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

            let new_indexer = UpdateIndexer {
                before_signature: Some(signature.signature.clone()),
                before_block: Some(signature.slot as i64),
                finished: Some(false),
            };
            db::update_indexer(&pool, db_indexer.id, &new_indexer).await?;
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
