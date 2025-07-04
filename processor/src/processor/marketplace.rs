use crate::convert::convert_to_decimal;
use chrono::DateTime;
use db::DbPool;
use decoder::staratlas::marketplace::{DecodedInstruction, ProcessExchange};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{UiInstruction, UiParsedInstruction};

pub struct MarketplaceProcessor {
    pub pool: DbPool,
}

#[derive(Debug, Clone)]
pub struct MarketplaceExchangeInner {
    pub program_id: String,
    pub mint: Option<String>,
    pub source: Option<String>,
    pub amount: Option<u64>,
    pub decimals: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct MarketplaceExchangeInnerParsed {
    pub side: String,
    pub currency_amount: Decimal,
    pub asset_amount: Decimal,
    pub fee_amount: Decimal,
    pub buddy_amount: Decimal,
    pub price: Decimal,
    pub volume: Decimal,
}

impl MarketplaceProcessor {
    pub fn new(pool: DbPool) -> Self {
        MarketplaceProcessor { pool }
    }

    pub async fn process(
        &self,
        slot: u64,
        block_time: i64,
        signature: String,
        index: usize,
        data: Vec<u8>,
        accounts: Vec<Pubkey>,
        inner_instructions: Vec<UiInstruction>,
    ) -> anyhow::Result<()> {
        match decoder::staratlas::marketplace::decode_instruction(data.as_slice()) {
            Some(DecodedInstruction::ProcessExchange(exchange)) => {
                let accounts_map = ProcessExchange::map_accounts(accounts.as_slice());

                let inner_data = Self::map_inner_exchange_transfers(
                    inner_instructions,
                    accounts_map["currency_mint"].to_string(),
                );

                // Create an ExchangeWithDependencies struct
                let exchange_data = db::ExchangeWithDependencies {
                    slot: slot as i32,
                    signature: signature.clone(),
                    index: index as i32,
                    timestamp: DateTime::from_timestamp(block_time, 0).unwrap(),
                    side: inner_data.side.clone(),
                    buyer_wallet: accounts_map["order_taker"].to_string(),
                    seller_wallet: accounts_map["order_initializer"].to_string(),
                    asset_mint: accounts_map["asset_mint"].to_string(),
                    pair_mint: accounts_map["currency_mint"].to_string(),
                    price: inner_data.price.to_f64().unwrap_or_default(),
                    size: inner_data.asset_amount.to_i32().unwrap_or_default(),
                    volume: inner_data.volume.to_f64().unwrap_or_default(),
                    fee: inner_data.fee_amount.to_f64().unwrap_or_default(),
                    buddy: inner_data.buddy_amount.to_f64().unwrap_or_default(),
                };

                db::create_exchange_with_dependencies(&self.pool, &exchange_data).await?;
                log::info!("Found process_exchange: {:?}", signature);

                Ok(())
            }

            Some(DecodedInstruction::ProcessInitializeBuy(_))
            | Some(DecodedInstruction::ProcessInitializeSell(_))
            | Some(DecodedInstruction::ProcessCancel)
            | Some(DecodedInstruction::InitializeOpenOrdersCounter) => Ok(()),

            _ => panic!(
                "Unhandled marketplace instruction [{}] {:?}",
                signature,
                hex::encode(data)
            ),
        }
    }

    fn map_inner_exchange_transfers(
        inner_instructions: Vec<UiInstruction>,
        currency_mint: String,
    ) -> MarketplaceExchangeInnerParsed {
        let mut mapped_inner = vec![];
        for (inner_idx, inner) in inner_instructions.clone().into_iter().enumerate() {
            //println!("[inner][{}] {:?}", inner_idx, inner);
            match inner {
                UiInstruction::Parsed(ui_parsed_instruction) => match ui_parsed_instruction {
                    UiParsedInstruction::Parsed(parsed_instruction) => {
                        match parsed_instruction
                            .parsed
                            .get("type")
                            .map(|s| s.as_str())
                            .unwrap()
                        {
                            Some("transferChecked") => {
                                mapped_inner.push(MarketplaceExchangeInner {
                                    program_id: parsed_instruction.program_id,
                                    mint: parsed_instruction
                                        .parsed
                                        .get("info")
                                        .and_then(|info| info.get("mint"))
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string()),
                                    source: parsed_instruction
                                        .parsed
                                        .get("info")
                                        .and_then(|info| info.get("source"))
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string()),
                                    amount: parsed_instruction
                                        .parsed
                                        .get("info")
                                        .and_then(|info| info.get("tokenAmount"))
                                        .and_then(|token_amount| token_amount.get("amount"))
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.parse::<u64>().unwrap()),
                                    decimals: parsed_instruction
                                        .parsed
                                        .get("info")
                                        .and_then(|info| info.get("tokenAmount"))
                                        .and_then(|token_amount| token_amount.get("decimals"))
                                        .and_then(|v| v.as_u64())
                                        .map(|s| s.try_into().unwrap()),
                                });
                            }
                            Some("transfer") => {
                                mapped_inner.push(MarketplaceExchangeInner {
                                    program_id: parsed_instruction.program_id,
                                    mint: parsed_instruction
                                        .parsed
                                        .get("info")
                                        .and_then(|info| info.get("mint"))
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string()),
                                    source: parsed_instruction
                                        .parsed
                                        .get("info")
                                        .and_then(|info| info.get("source"))
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string()),
                                    amount: parsed_instruction
                                        .parsed
                                        .get("info")
                                        .and_then(|token_amount| token_amount.get("amount"))
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.parse::<u64>().unwrap()),
                                    decimals: None,
                                });
                            }
                            _ => panic!("Unhandled parsed instruction type"),
                        }
                    }
                    UiParsedInstruction::PartiallyDecoded(partially) => {
                        if (partially.program_id == decoder::extra::transferHook::ID.to_string()) {
                            continue;
                        }

                        if partially.program_id == decoder::staratlas::buddy::ID.to_string() {
                            mapped_inner.push(MarketplaceExchangeInner {
                                program_id: partially.program_id.clone(),
                                mint: None,
                                source: None,
                                amount: None,
                                decimals: None,
                            });
                            continue;
                        }

                        panic!(
                            "Unhandled partially decoded instruction for program_id: {}",
                            partially.program_id
                        );
                    }
                },
                _ => panic!("Unhandled UiInstruction type"),
            }
        }

        //println!("mapped_inner={:?}", mapped_inner);

        let mapped_inner_refs: Vec<&str> =
            mapped_inner.iter().map(|s| s.program_id.as_str()).collect();

        let mut side = "NONE".to_string();
        let mut currency_amount = Decimal::default();
        let mut asset_amount = Decimal::default();
        let mut fee_amount = Decimal::default();
        let mut buddy_amount = Decimal::default();

        match mapped_inner_refs.as_slice() {
            [
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            ] => {
                side = Self::get_side(currency_mint, &mut mapped_inner, 1);
                match side.as_str() {
                    "BUY" => {
                        fee_amount = convert_to_decimal(
                            mapped_inner[0].clone().amount.unwrap(),
                            mapped_inner[0].clone().decimals.unwrap(),
                        );

                        asset_amount = convert_to_decimal(
                            mapped_inner[1].clone().amount.unwrap(),
                            mapped_inner[1].clone().decimals.unwrap(),
                        );

                        currency_amount = convert_to_decimal(
                            mapped_inner[2].clone().amount.unwrap(),
                            mapped_inner[2].clone().decimals.unwrap(),
                        );
                    }
                    "SELL" => {
                        fee_amount = convert_to_decimal(
                            mapped_inner[0].clone().amount.unwrap(),
                            mapped_inner[0].clone().decimals.unwrap(),
                        );

                        currency_amount = convert_to_decimal(
                            mapped_inner[1].clone().amount.unwrap(),
                            mapped_inner[1].clone().decimals.unwrap(),
                        );

                        asset_amount = convert_to_decimal(
                            mapped_inner[2].clone().amount.unwrap(),
                            mapped_inner[2].clone().decimals.unwrap(),
                        );
                    }
                    _ => panic!("Unhandled side"),
                }
            }
            [
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
            ] => {
                side = Self::get_side(currency_mint, &mut mapped_inner, 1);
                match side.as_str() {
                    "BUY" => {
                        fee_amount = convert_to_decimal(
                            mapped_inner[0].clone().amount.unwrap(),
                            mapped_inner[0].clone().decimals.unwrap(),
                        );

                        asset_amount = convert_to_decimal(
                            mapped_inner[1].clone().amount.unwrap(),
                            mapped_inner[1].clone().decimals.unwrap(),
                        );

                        currency_amount = convert_to_decimal(
                            mapped_inner[2].clone().amount.unwrap(),
                            mapped_inner[2].clone().decimals.unwrap(),
                        );
                    }
                    "SELL" => {
                        fee_amount = convert_to_decimal(
                            mapped_inner[0].clone().amount.unwrap(),
                            mapped_inner[0].clone().decimals.unwrap(),
                        );

                        currency_amount = convert_to_decimal(
                            mapped_inner[1].clone().amount.unwrap(),
                            mapped_inner[1].clone().decimals.unwrap(),
                        );

                        asset_amount = convert_to_decimal(
                            mapped_inner[2].clone().amount.unwrap(),
                            mapped_inner[2].clone().decimals.unwrap(),
                        );
                    }
                    _ => panic!("Unhandled side"),
                }
            }

            [
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
                "tHookmPkFZDJGkS9us6sVsnYi2EKHCrVtw8zD6oXYPE",
            ] => {
                side = Self::get_side(currency_mint, &mut mapped_inner, 1);
                match side.as_str() {
                    "BUY" => {
                        fee_amount = convert_to_decimal(
                            mapped_inner[0].clone().amount.unwrap(),
                            mapped_inner[0].clone().decimals.unwrap(),
                        );

                        asset_amount = convert_to_decimal(
                            mapped_inner[1].clone().amount.unwrap(),
                            mapped_inner[1].clone().decimals.unwrap(),
                        );

                        currency_amount = convert_to_decimal(
                            mapped_inner[2].clone().amount.unwrap(),
                            mapped_inner[2].clone().decimals.unwrap(),
                        );
                    }
                    "SELL" => {
                        fee_amount = convert_to_decimal(
                            mapped_inner[0].clone().amount.unwrap(),
                            mapped_inner[0].clone().decimals.unwrap(),
                        );

                        currency_amount = convert_to_decimal(
                            mapped_inner[1].clone().amount.unwrap(),
                            mapped_inner[1].clone().decimals.unwrap(),
                        );

                        asset_amount = convert_to_decimal(
                            mapped_inner[2].clone().amount.unwrap(),
                            mapped_inner[2].clone().decimals.unwrap(),
                        );
                    }
                    _ => panic!("Unhandled side"),
                }
            }

            [
                "BUDDYtQp7Di1xfojiCSVDksiYLQx511DPdj2nbtG9Yu5",
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            ] => {
                side = Self::get_side(currency_mint, &mut mapped_inner, 3);
                match side.as_str() {
                    "BUY" => {
                        buddy_amount = convert_to_decimal(
                            mapped_inner[1].clone().amount.unwrap(),
                            mapped_inner
                                .iter()
                                .find(|inner| {
                                    inner.source == mapped_inner[1].source
                                        && inner.decimals.is_some()
                                })
                                .unwrap()
                                .decimals
                                .unwrap(),
                        );
                        fee_amount = convert_to_decimal(
                            mapped_inner[2].clone().amount.unwrap(),
                            mapped_inner[2].clone().decimals.unwrap(),
                        );

                        asset_amount = convert_to_decimal(
                            mapped_inner[3].clone().amount.unwrap(),
                            mapped_inner[3].clone().decimals.unwrap(),
                        );

                        currency_amount = convert_to_decimal(
                            mapped_inner[4].clone().amount.unwrap(),
                            mapped_inner[4].clone().decimals.unwrap(),
                        );
                    }
                    "SELL" => {
                        buddy_amount = convert_to_decimal(
                            mapped_inner[1].clone().amount.unwrap(),
                            mapped_inner
                                .iter()
                                .find(|inner| {
                                    inner.source == mapped_inner[1].source
                                        && inner.decimals.is_some()
                                })
                                .unwrap()
                                .decimals
                                .unwrap(),
                        );
                        fee_amount = convert_to_decimal(
                            mapped_inner[2].clone().amount.unwrap(),
                            mapped_inner[2].clone().decimals.unwrap(),
                        );

                        currency_amount = convert_to_decimal(
                            mapped_inner[3].clone().amount.unwrap(),
                            mapped_inner[3].clone().decimals.unwrap(),
                        );

                        asset_amount = convert_to_decimal(
                            mapped_inner[4].clone().amount.unwrap(),
                            mapped_inner[4].clone().decimals.unwrap(),
                        );
                    }
                    _ => panic!("Unhandled side"),
                }
            }

            [
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            ] => {
                side = Self::get_side(currency_mint, &mut mapped_inner, 1);
                match side.as_str() {
                    "BUY" => {
                        fee_amount = convert_to_decimal(
                            mapped_inner[0].clone().amount.unwrap(),
                            mapped_inner[0].clone().decimals.unwrap(),
                        );

                        asset_amount = convert_to_decimal(
                            mapped_inner[1].clone().amount.unwrap(),
                            mapped_inner[1].clone().decimals.unwrap(),
                        );

                        currency_amount = convert_to_decimal(
                            mapped_inner[2].clone().amount.unwrap(),
                            mapped_inner[2].clone().decimals.unwrap(),
                        );
                    }
                    "SELL" => {
                        fee_amount = convert_to_decimal(
                            mapped_inner[0].clone().amount.unwrap(),
                            mapped_inner[0].clone().decimals.unwrap(),
                        );

                        currency_amount = convert_to_decimal(
                            mapped_inner[1].clone().amount.unwrap(),
                            mapped_inner[1].clone().decimals.unwrap(),
                        );

                        asset_amount = convert_to_decimal(
                            mapped_inner[2].clone().amount.unwrap(),
                            mapped_inner[2].clone().decimals.unwrap(),
                        );
                    }
                    _ => panic!("Unhandled side"),
                }
            }

            _ => panic!(
                "Unhandled inner instructions [{}]",
                mapped_inner_refs.join(", ")
            ),
        };

        let price = (fee_amount + currency_amount + buddy_amount)
            .checked_div(asset_amount)
            .unwrap_or_default();
        let volume = fee_amount + currency_amount;

        //println!("mapped_inner={:?}", mapped_inner);

        MarketplaceExchangeInnerParsed {
            side,
            currency_amount,
            asset_amount,
            fee_amount,
            buddy_amount,
            price,
            volume,
        }
    }

    fn get_side(
        currency_mint: String,
        mapped_inner: &mut Vec<MarketplaceExchangeInner>,
        idx: usize,
    ) -> String {
        match mapped_inner[idx]
            .clone()
            .mint
            .unwrap()
            .contains(&currency_mint)
        {
            true => "SELL".to_string(),
            false => "BUY".to_string(),
        }
    }
}
