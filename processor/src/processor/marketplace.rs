use crate::convert::convert_to_decimal;
use decoder::staratlas::marketplace::{DecodedInstruction, ProcessExchange};
use rust_decimal::Decimal;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{UiInstruction, UiParsedInstruction};

pub struct MarketplaceProcessor {}

#[derive(Debug, Clone)]
pub struct MarketplaceExchangeInner {
    pub program_id: String,
    pub mint: Option<String>,
    pub amount: Option<u64>,
    pub decimals: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct MarketplaceExchangeInnerParsed {
    pub side: String,
    pub currency_amount: Decimal,
    pub asset_amount: Decimal,
    pub fee_amount: Decimal,
    pub price: Decimal,
    pub volume: Decimal,
}

impl MarketplaceProcessor {
    pub fn process(
        signature: String,
        data: Vec<u8>,
        accounts: Vec<Pubkey>,
        inner_instructions: Vec<UiInstruction>,
    ) {
        match decoder::staratlas::marketplace::decode_instruction(data.as_slice()) {
            Some(DecodedInstruction::ProcessExchange(exchange)) => {
                let accounts_map = ProcessExchange::map_accounts(accounts.as_slice());

                println!("exchange={:?}", exchange);
                println!("accounts_map={:?}", accounts_map);
                let inner_data = Self::map_inner_exchange_transfers(
                    inner_instructions,
                    accounts_map["currency_mint"].to_string(),
                );
                println!("inner_data={:?}", inner_data);
            }

            Some(DecodedInstruction::ProcessInitializeBuy(_)) => {}
            Some(DecodedInstruction::ProcessInitializeSell(_)) => {}
            Some(DecodedInstruction::ProcessCancel) => {}

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
        for inner in inner_instructions.clone() {
            match inner {
                UiInstruction::Parsed(parsed) => match parsed {
                    UiParsedInstruction::Parsed(parsed) => {
                        //println!("parsed={:?}", parsed);
                        mapped_inner.push(MarketplaceExchangeInner {
                            program_id: parsed.program_id,
                            mint: parsed
                                .parsed
                                .get("info")
                                .and_then(|info| info.get("mint"))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            amount: parsed
                                .parsed
                                .get("info")
                                .and_then(|info| info.get("tokenAmount"))
                                .and_then(|token_amount| token_amount.get("amount"))
                                .and_then(|v| v.as_str())
                                .map(|s| s.parse::<u64>().unwrap()),
                            decimals: parsed
                                .parsed
                                .get("info")
                                .and_then(|info| info.get("tokenAmount"))
                                .and_then(|token_amount| token_amount.get("decimals"))
                                .and_then(|v| v.as_u64())
                                .map(|s| s.try_into().unwrap()),
                        });
                    }
                    UiParsedInstruction::PartiallyDecoded(partially) => {
                        if (partially.program_id == decoder::extra::transferHook::ID.to_string()) {
                            continue;
                        }

                        if partially.program_id == decoder::staratlas::buddy::ID.to_string() {
                            mapped_inner.push(MarketplaceExchangeInner {
                                program_id: partially.program_id.clone(),
                                mint: None,
                                amount: None,
                                decimals: None,
                            })
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
                side = Self::get_side(currency_mint, &mut mapped_inner);
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
                side = Self::get_side(currency_mint, &mut mapped_inner);
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
                side = Self::get_side(currency_mint, &mut mapped_inner);
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

            _ => panic!("Unhandled inner instructions"),
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
            price,
            volume,
        }
    }

    fn get_side(currency_mint: String, mapped_inner: &mut Vec<MarketplaceExchangeInner>) -> String {
        match mapped_inner[1]
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
