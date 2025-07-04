use crate::args::Args;

use crate::convert::{processor_accounts, processor_data, processor_inner};
use crate::processor::marketplace::MarketplaceProcessor;
use base64::Engine;
use clap::Parser;
use db::update_program_signature_processed;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::packet::Encode;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::{
    EncodedTransaction, UiInstruction, UiMessage, UiParsedInstruction, UiTransactionEncoding,
};
use std::env;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

mod args;
mod convert;
mod processor;

const SLEEP: Duration = Duration::from_secs(5);

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let args = Args::parse();

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

    let client = RpcClient::new_with_commitment(
        String::from(env::var("RPC_URL").expect("RPC_URL must be set")),
        CommitmentConfig::confirmed(),
    );

    let transaction_config = RpcTransactionConfig {
        commitment: CommitmentConfig::finalized().into(),
        encoding: UiTransactionEncoding::JsonParsed.into(),
        max_supported_transaction_version: Some(0),
    };

    let program_id = Pubkey::from_str(
        env::var("PROGRAM_ID")
            .expect("PROGRAM_ID must be set")
            .as_str(),
    )?;

    let pool = db::establish_connection().await?;

    loop {
        let db_signatures: Vec<String> = match args.signature.as_ref() {
            Some(signature) => vec![signature.clone()],
            None => db::get_unprocessed_program_signatures_by_program_id(
                &pool,
                &program_id.to_string(),
                1000,
            )
            .await?
            .iter()
            .map(|signature| signature.signature.clone())
            .collect(),
        };

        for db_signature in db_signatures {
            log::info!("Processing signature: {:?}", db_signature);

            let transaction = client.get_transaction_with_config(
                &Signature::from_str(db_signature.as_str()).unwrap(),
                transaction_config,
            )?;

            let transaction_meta = transaction.transaction.meta.unwrap();

            match transaction.transaction.transaction {
                EncodedTransaction::Json(json) => match json.message {
                    UiMessage::Parsed(parsed) => {
                        for (instruction_index, instruction) in
                            parsed.instructions.into_iter().enumerate()
                        {
                            match instruction {
                                UiInstruction::Parsed(parsed) => match parsed {
                                    UiParsedInstruction::PartiallyDecoded(instruction) => {
                                        match Pubkey::from_str(instruction.program_id.as_str())? {
                                            decoder::staratlas::marketplace::ID => {
                                                MarketplaceProcessor::new(pool.clone())
                                                    .process(
                                                        transaction.slot,
                                                        transaction.block_time.unwrap(),
                                                        db_signature.clone(),
                                                        instruction_index,
                                                        processor_data(instruction.data),
                                                        processor_accounts(instruction.accounts),
                                                        processor_inner(
                                                            transaction_meta.clone(),
                                                            instruction_index,
                                                        ),
                                                    )
                                                    .await?;

                                                //UPDATE DB
                                                update_program_signature_processed(
                                                    &pool,
                                                    &decoder::staratlas::marketplace::ID
                                                        .to_string(),
                                                    &db_signature,
                                                    true,
                                                )
                                                .await?;
                                            }
                                            _ => {}
                                        }
                                    }
                                    UiParsedInstruction::Parsed(instruction) => {
                                        match Pubkey::from_str(instruction.program_id.as_str())? {
                                            decoder::staratlas::marketplace::ID => {
                                                panic!("unimplemented for marketplace")
                                            }
                                            _ => {}
                                        }
                                    }
                                },
                                _ => panic!("Unhandled UiInstruction type"),
                            }
                        }
                    }
                    _ => panic!("Unhandled UiMessage type"),
                },
                _ => panic!("Unhandled EncodedTransaction type"),
            };
        }
        sleep(SLEEP).await;
    }
}
