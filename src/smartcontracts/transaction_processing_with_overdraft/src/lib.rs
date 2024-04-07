#![no_std]

extern crate alloc;
#[cfg(not(test))]
extern crate panic_halt;

use lol_alloc::{FreeListAllocator, LockedAllocator};

#[global_allocator]
static ALLOC: LockedAllocator<FreeListAllocator> = LockedAllocator::new(FreeListAllocator::new());

use core::str::FromStr;
use iroha_trigger::prelude::*;

/// Makes transfer asset and mints asset if overdraft is available
/// and account doesn't have enough asset value for transfer
#[iroha_trigger::main]
fn main(_id: TriggerId, _owner: AccountId, event: Event) {
    let source_account_id = match event {
        Event::ExecuteTrigger(event) => event.authority().clone(),
        _ => return,
    };

    let source_account: Account =
        QueryBox::FindAccountById(FindAccountById::new(source_account_id.clone()))
            .execute()
            .unwrap()
            .try_into()
            .unwrap();

    // Instructions deconstruction
    let (instructions, asset, asset_value, dest_account) = (
        Name::from_str("instructions").unwrap(),
        Name::from_str("asset").unwrap(),
        Name::from_str("assetValue").unwrap(),
        Name::from_str("destAccount").unwrap(),
    );

    let asset_id: AssetId;
    let transferred_amount: Fixed;
    let destination_account: AccountId;

    match source_account.metadata().get(&instructions) {
        Some(Value::LimitedMetadata(instructions)) if instructions.iter().len() == 3 => {
            asset_id = match instructions.get(&asset) {
                Some(Value::Id(IdBox::AssetId(id))) => id.clone(),
                _ => return,
            };
            transferred_amount = match instructions.get(&asset_value) {
                Some(Value::Numeric(NumericValue::Fixed(value))) => *value,
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

    let available_overdraft_amount: Fixed = source_account
        .metadata()
        .get(&overdraft)
        .and_then(|value| match value {
            Value::LimitedMetadata(overdraft) => Some(overdraft),
            _ => None,
        })
        .and_then(|overdraft| match overdraft.get(&available) {
            Some(Value::Bool(true)) => Some(overdraft),
            _ => None,
        })
        .and_then(|overdraft| match overdraft.get(&available_amount) {
            Some(Value::Numeric(NumericValue::Fixed(fixed_amount))) => Some(*fixed_amount),
            _ => None,
        })
        .unwrap_or(Fixed::ZERO);

    //Transfer expression construction

    let source_account_asset_amount =
        match source_account.asset(&asset_id).map(|asset| asset.value()) {
            Some(AssetValue::Fixed(fixed_amount)) => *fixed_amount,
            _ => return,
        };

    let minted_amount = transferred_amount
        .checked_sub(source_account_asset_amount)
        .unwrap_or(Fixed::ZERO);

    if minted_amount > Fixed::ZERO && available_overdraft_amount >= minted_amount {
        MintExpr::new(minted_amount, source_account_id)
            .execute()
            .unwrap()
    }

    TransferExpr::new(asset_id, transferred_amount, destination_account)
        .execute()
        .unwrap()
}
