use rust_decimal::Decimal;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{UiInstruction, UiTransactionStatusMeta};
use std::str::FromStr;

pub fn processor_data(data: String) -> Vec<u8> {
    bs58::decode(data).into_vec().unwrap()
}
pub fn string_to_hex(data: String) -> String {
    let data_bytes = processor_data(data);
    hex::encode(data_bytes)
}

pub fn processor_accounts(data: Vec<String>) -> Vec<Pubkey> {
    data.iter()
        .map(|acc| Pubkey::from_str(acc).unwrap())
        .collect()
}

pub fn processor_inner(
    transaction_meta: UiTransactionStatusMeta,
    instruction_index: usize,
) -> Vec<UiInstruction> {
    match transaction_meta
        .inner_instructions
        .clone()
        .unwrap()
        .into_iter()
        .find(|inner| inner.index == instruction_index as u8)
        .clone()
    {
        Some(inner) => inner.instructions,
        None => vec![],
    }
}

pub fn convert_to_decimal(amount: u64, decimals: u8) -> Decimal {
    let scale = Decimal::new(1, decimals.into());
    let decimal = Decimal::from(amount) * scale;
    return decimal;
}
