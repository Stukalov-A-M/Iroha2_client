#![feature(unwrap_infallible)]

use iroha_trigger::prelude::*;
use std::str::FromStr;

/// Mint 1 rose for owner
#[iroha_trigger::main]
fn main(_id: TriggerId, _owner: AccountId, event: Event) {

    let account_id = match event {
        Event::ExecuteTrigger(event) => event.authority().clone(),
        _=> return

    };

    let account: Account = QueryBox::FindAccountById(FindAccountById::new(account_id))
        .execute()
        .unwrap()
        .try_into()
        .unwrap();

    // Transaction construction
    let (instructions, asset, asset_value, dest_account) = (
        Name::from_str("instructions").unwrap(),
        Name::from_str("asset").unwrap(),
        Name::from_str("assetValue").unwrap(),
        Name::from_str("destAccount").unwrap(),
    );

    let asset_id: AssetId;
    let transfer_amount: Fixed;
    let destination_account: AccountId;

    match account.metadata().get(&instructions) {
        Some(Value::LimitedMetadata(instructions)) if instructions.iter().len() == 3 => {
            asset_id = match instructions.get(&asset) {
                Some(Value::Id(IdBox::AssetId(id))) => id.clone(),
                _ => return,
            };
            transfer_amount = match instructions.get(&asset_value) {
                Some(Value::Numeric(NumericValue::Fixed(value))) => value.clone(),
                _ => return,
            };
            destination_account = match instructions.get(&dest_account) {
                Some(Value::Id(IdBox::AccountId(id))) => id.clone(),
                _ => return,
            };
        }
        _ => return,
    }

    // Overdraft processing

    let (overdraft, available, available_amount) = (
        Name::from_str("overdraft").unwrap(),
        Name::from_str("available").unwrap(),
        Name::from_str("available_amount").unwrap(),
    );

    let amount;

    if let Some(Value::LimitedMetadata(overdraft)) = account.metadata().get(&overdraft) {
        if let Some(Value::Bool(true)) = overdraft.get(&available) {
            if let Some(Value::Numeric(NumericValue::Fixed(fixed_amount))) =
                overdraft.get(&available_amount)
            {
                amount = fixed_amount.clone();
            }
        }
    }
}
