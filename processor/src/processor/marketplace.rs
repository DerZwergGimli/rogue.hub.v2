use decoder::staratlas::marketplace::{DecodedInstruction, ProcessExchange};
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{UiInstruction, UiParsedInstruction};

pub struct MarketplaceProcessor {}

impl MarketplaceProcessor {
    pub fn process(
        signature: String,
        data: Vec<u8>,
        accounts: Vec<Pubkey>,
        inner_instructions: Vec<UiInstruction>,
    ) {
        match decoder::staratlas::marketplace::decode_instruction(data.as_slice()) {
            Some(DecodedInstruction::ProcessExchange(exchange)) => {
                for inner in inner_instructions.clone() {
                    match inner {
                        UiInstruction::Parsed(parsed) => match parsed {
                            UiParsedInstruction::Parsed(parsed) => {
                                println!("program_id={:?}", parsed.program_id);
                            }
                            _ => panic!("Unhandled UiParsedInstruction type"),
                        },
                        _ => panic!("Unhandled UiInstruction type"),
                    }
                }

                let accounts_map = ProcessExchange::map_accounts(accounts.as_slice());
                println!("exchange={:?}", exchange);
                println!("accounts_map={:?}", accounts_map);
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

    fn map_inner_transfers(inner_instructions: Vec<UiInstruction>) {
        let mut mapped_inner = vec![];
        for inner in inner_instructions.clone() {
            match inner {
                UiInstruction::Parsed(parsed) => match parsed {
                    UiParsedInstruction::Parsed(parsed) => {
                        mapped_inner.push(parsed.program_id);
                    }
                    _ => panic!("Unhandled UiParsedInstruction type"),
                },
                _ => panic!("Unhandled UiInstruction type"),
            }
        }
    }
}
