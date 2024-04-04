#![feature(unwrap_infallible)]

use iroha_trigger::prelude::*;
use std::str::FromStr;

/// Mint 1 rose for owner
#[iroha_trigger::main]
fn main(_id: TriggerId, _owner: AccountId, event: Event) {
    let account_id = if let Event::ExecuteTrigger(event) = event {
        event.authority().clone()
    } else {
        return;
    };

    let account: Account = QueryBox::FindAccountById(FindAccountById::new(account_id))
        .execute()
        .unwrap()
        .try_into()
        .unwrap();

    // Transaction construction
    let (instructions, asset, asset_value, d_account) = (
        Name::from_str("instructions").unwrap(),
        Name::from_str("asset").unwrap(),
        Name::from_str("assetValue").unwrap(),
        Name::from_str("destAccount").unwrap(),
    );

    let asset_id: AssetId;
    let transfer_amount: Fixed;
    let dest_account: AccountId;

    if let Some(Value::LimitedMetadata(instructions)) = account.metadata().get(&instructions) {
        if instructions.iter().len() == 3 {
            if let Some(Value::Id(IdBox::AssetId(id))) = instructions.get(&asset) {
                asset_id = id.clone()
            } else {
                return;
            };
            if let Some(Value::Numeric(NumericValue::Fixed(value))) = instructions.get(&asset_value)
            {
                transfer_amount = value.clone();
            } else {
                return;
            };
            if let Some(Value::Id(IdBox::AccountId(id))) = instructions.get(&d_account) {
                dest_account = id.clone()
            } else {
                return;
            };
        } else {
            return;
        };
    } else {
        return;
    };

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
