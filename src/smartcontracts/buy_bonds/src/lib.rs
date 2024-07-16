//! Bla-bla-bla
#![no_std]

extern crate alloc;
#[cfg(not(test))]
extern crate panic_halt;

use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use core::error::Error;

use iroha_trigger::prelude::*;
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[global_allocator]
static ALLOC: LockedAllocator<FreeListAllocator> = LockedAllocator::new(FreeListAllocator::new());

use iroha_trigger::prelude::QueryBox::FindAssetDefinitionById;

#[iroha_trigger::main]
fn trigger_entry_point(_id: TriggerId, owner: AccountId, event: Event) {
    // Catch SetKeyValue event for the trigger
    let event = match event {
        Event::Data(DataEvent::Trigger(TriggerEvent::MetadataInserted(event))) => event,
        _ => return,
    };

    // Check that event is related to bonds
    // and contains of "buy_bonds" container
    if event.key().as_ref() != "buy_bonds" {
        return;
    }

    // Defining the transaction properties
    let bond_asset_definition_id: AssetDefinitionId;
    let bonds_quantity: NumericValue;
    let transaction_amount: NumericValue;
    let buyer_account_id: AccountId;
    let bonds_currency_asset_definition_id: AssetDefinitionId;

    // Deconstruct the "buy_bonds" container
    if let Value::LimitedMetadata(metadata) = event.value() {
        bond_asset_definition_id = metadata
            .get("bonds_id")
            .dbg_unwrap()
            .to_owned()
            .try_into()
            .dbg_unwrap();
        bonds_quantity = metadata
            .get("quantity")
            .dbg_unwrap()
            .to_owned()
            .try_into()
            .dbg_unwrap();
        transaction_amount = metadata
            .get("amount")
            .dbg_unwrap()
            .to_owned()
            .try_into()
            .dbg_unwrap();
        buyer_account_id = metadata
            .get("buyer_account_id")
            .dbg_unwrap()
            .to_owned()
            .try_into()
            .dbg_unwrap();
    } else {
        return;
    }

    // Seek for bonds asset_definition currency id
    if let Ok(value) = FindAssetDefinitionById(
        iroha_trigger::data_model::query::asset::FindAssetDefinitionById::new(
            bond_asset_definition_id.clone(),
        ),
    )
    .execute()
    {
        if let Ok(bonds_asset_definition) = TryInto::<AssetDefinition>::try_into(value) {
            match bonds_asset_definition
                .metadata()
                .get("currency_asset_definition_id")
            {
                Some(value) => {
                    if let Ok(currency_asset_definition_id) =
                        TryInto::<AssetDefinitionId>::try_into(value.to_owned())
                    {
                        bonds_currency_asset_definition_id = currency_asset_definition_id
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        } else {
            return;
        }
    } else {
        return;
    }

    let buyer_money_asset_id =
        AssetId::new(bonds_currency_asset_definition_id, buyer_account_id.clone());

    let seller_bonds_asset_id = AssetId::new(bond_asset_definition_id, owner.clone());

    if !are_assets_enough(buyer_money_asset_id.clone(), &transaction_amount, seller_bonds_asset_id.clone(), &bonds_quantity) {return;}

    // Send money from buyer to seller
    // and then send bonds from seller to buyer
    InstructionExpr::Transfer(TransferExpr::new(
        buyer_money_asset_id,
        transaction_amount,
        owner.clone(),
    ))
    .execute()
    .expect("Send money validation has been failed");
    InstructionExpr::Transfer(TransferExpr::new(
        seller_bonds_asset_id,
        bonds_quantity,
        buyer_account_id,
    ))
    .execute()
    .expect("Send bonds validation has been failed");
}

// Checks:
// * The Account has this asset.
// * The AssetValue has a NumericValue type
// * The Account has enough asset quantity for transaction.
fn check_account_asset_amount(
    asset_id: AssetId,
    asset_amount: &NumericValue,
) -> Result<bool, Box<dyn Error>> {
    if let Ok(value) = QueryBox::FindAssetById(FindAssetById::new(asset_id)).execute() {
        match TryInto::<Asset>::try_into(value) {
            Ok(owned_asset) => {
                match TryInto::<NumericValue>::try_into(owned_asset.value().to_owned()) {
                    Ok(owned_asset_value) => {
                        if owned_asset_value < *asset_amount {
                            Err("The asset owner has not enough asset value".into())
                        } else {
                            Ok(true)
                        }
                    }
                    _ => Err("The asset value has not a NumericValue type".into()),
                }
            }
            // Should be impossible to reach
            _ => Err("The found asset has not an Asset type".into()),
        }
    } else {
        Err("The asset cannot be found by ID".into())
    }
}

fn are_assets_enough(
    buyer_money_asset_id: AssetId,
    transaction_amount: &NumericValue,
    seller_bonds_asset_id: AssetId,
    bonds_quantity: &NumericValue,
) -> bool {
    check_account_asset_amount(buyer_money_asset_id, transaction_amount).dbg_unwrap()
        == check_account_asset_amount(seller_bonds_asset_id, bonds_quantity).dbg_unwrap()
}
