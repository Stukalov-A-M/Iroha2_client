//! Trigger which mints asset for the created user
//! Activates only if user has "mintAsset" name in metadata
//! This module isn't included in the build-tree,
//! but instead it is being built by a `client/build.rs`
#![no_std]

extern crate alloc;
#[cfg(not(test))]
extern crate panic_halt;

use lol_alloc::{FreeListAllocator, LockedAllocator};

#[global_allocator]
static ALLOC: LockedAllocator<FreeListAllocator> = LockedAllocator::new(FreeListAllocator::new());

use iroha_trigger::{data_model::prelude::*, debug::DebugUnwrapExt};

#[iroha_trigger::main]
fn trigger_entry_point(_owner: AccountId, event: Event) {
    let account_id: AccountId = {
        if let Event::Data(DataEvent::Account(AccountEvent::Created(account))) = event {
            if account
                .metadata()
                .contains(&"mintAsset".parse().dbg_unwrap())
            {
                account.id().clone()
            } else {
                return;
            }
        } else {
            return;
        }
    };

    let asset_definition_id: AssetDefinitionId = "cat#wonderland".parse().unwrap();
    if let Err(_e) = FindAssetDefinitionById::new(asset_definition_id.clone()).execute() {
        let asset_definition = AssetDefinition::quantity(asset_definition_id.clone());

        RegisterExpr::new(asset_definition).execute().dbg_unwrap();
    }
    let asset_id = AssetId::new(asset_definition_id, account_id);

    MintExpr::new(1_u32, asset_id).execute().dbg_unwrap()
}