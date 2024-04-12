#![feature(iter_collect_into)]

use core::str::FromStr;
use iroha_client::client::{Client, QueryOutput, ResultSet};
use iroha_data_model::account::{Account, AccountId};
use iroha_data_model::asset::{AssetDefinition, AssetId};
use iroha_data_model::isi::{SetKeyValueExpr, UnregisterExpr};
use iroha_data_model::name::Name;
use iroha_data_model::prelude::{Action, ExecuteTriggerEventFilter, ExecuteTriggerExpr, GrantExpr, Metadata, MetadataLimits, RegisterExpr, RemoveKeyValueExpr, Repeats, Trigger, TriggerId, TriggeringFilterBox, WasmSmartContract};
use iroha_data_model::{HasMetadata, NumericValue, Value};
use iroha_primitives::fixed::Fixed;
use iroha_stable::client::get_client;
use iroha_stable::queries::{get_all_accounts, print_all_accounts_with_assets};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use iroha_data_model::permission::PermissionToken;
use iroha_data_model::query::permission::model::FindPermissionTokensByAccountId;
use serde_json::json;
use iroha_stable::asset::mint_asset;
use iroha_stable::trigger::print_all_registered_triggers;

fn main() -> Result<(), Box<dyn Error>> {
    let iroha_client: Client = get_client();
    let status = iroha_client.get_status()?;
    println!("{:?}", status);

/*
    let metadata = Name::from_str("metadata").unwrap();
    let test_account: AccountId = "artem@first".parse().unwrap();


    iroha_client.submit_blocking(RemoveKeyValueExpr::new(test_account, metadata)).unwrap();

*/



    let test_account: AccountId = "artem@first".parse().unwrap();
    let test_asset: AssetId = "eur#cbdc#admin@cbdc".parse().unwrap();
    let test_tx_value: Fixed = Fixed::from_str("6").unwrap();
    let test_dest_account: AccountId = "alina@second".parse().unwrap();
    let test_overdraft_amount: Fixed = Fixed::from_str("1000000").unwrap();

    account_metadata_configuration(
        test_account.clone(),
        test_asset,
        test_tx_value,
        test_dest_account,
        true,
        test_overdraft_amount,
    );




    //unregister_trigger();
    //register_trigger();




    //mint_asset("eur#cbdc".parse().unwrap(), "admin@cbdc".parse().unwrap(), NumericValue::Fixed(Fixed::from_str("5").unwrap()));

    print_all_registered_triggers();
    print_all_accounts_with_assets();
    println!("----------\n");
    //can_execute_user_trigger("artem@first", "transaction_processing_with_overdraft$cbdc");
    execute_trigger();

    //print_metadata();
    //print_account_permissions("artem@first");

    print_all_accounts_with_assets();


    fn can_execute_user_trigger(target_account: &str, trigger_id: &str) {
        let account_id: AccountId = target_account.parse().unwrap();

        let can_execute_user_trigger_token = PermissionToken::new(
            "CanExecuteUserTrigger".parse().unwrap(),
            &json!({ "trigger_id": trigger_id, }),
        );

        get_client().submit_blocking(GrantExpr::new(can_execute_user_trigger_token, account_id)).unwrap();
    }
    fn can_mint_asset_permission(target_account: AccountId, asset_definition_id: &str) {

        let can_mint_asset_with_definition_token = PermissionToken::new(
            "CanMintAssetsWithDefinition".parse().unwrap(),
            &json!({ "asset_definition_id": asset_definition_id }),
        );

        get_client().submit_blocking(GrantExpr::new(can_mint_asset_with_definition_token, target_account)).unwrap();
    }

    fn print_account_permissions(account_id: &str) {
        let account_id: AccountId = account_id.parse().unwrap();

        let result: ResultSet<PermissionToken> = get_client().request(FindPermissionTokensByAccountId::new(account_id)).unwrap();
        result.for_each(|token|println!("{}", token.unwrap()))
    }

    fn print_metadata() {
        get_all_accounts().into_iter().for_each(|account| {
            account
                .unwrap()
                .metadata()
                .iter()
                .for_each(|metadata| println!("{:?}", metadata))
        });
    }

fn unregister_trigger() {
    let trigger_id = TriggerId::new("cbdc".parse().ok(), "transaction_processing_with_overdraft".parse().ok().unwrap());
    get_client().submit_blocking(UnregisterExpr::new(trigger_id)).unwrap();

}

fn execute_trigger() {
    let trigger_id = TriggerId::new("cbdc".parse().ok(), "transaction_processing_with_overdraft".parse().ok().unwrap());

    get_client().submit_blocking(ExecuteTriggerExpr::new(trigger_id)).unwrap();
}

fn register_trigger() {
    let admin: AccountId = "admin@cbdc".parse().unwrap();
    let artem: AccountId = "artem@first".parse().unwrap();

    let mut file = File::open("C:\\RustProjects\\iroha_stable\\src\\resources\\transaction_processing_with_overdraft.wasm").unwrap();
    let mut buff = Vec::new();
    file.read_to_end(&mut buff).unwrap();

    let trigger_id = TriggerId::new("cbdc".parse().ok(), "transaction_processing_with_overdraft".parse().ok().unwrap());

    let register_trigger =
        RegisterExpr::new(
            Trigger::new(
                trigger_id.clone(),
                Action::new(
                    WasmSmartContract::from_compiled(buff),
                    Repeats::Indefinitely,
                    admin.clone(),
                    TriggeringFilterBox::ExecuteTrigger(ExecuteTriggerEventFilter::new(trigger_id, artem)))));
    get_client().submit_blocking(register_trigger).unwrap();
}





    Ok(())
}

fn account_metadata_configuration(
    account_id: AccountId,
    asset_id: AssetId,
    asset_value: Fixed,
    destination_account: AccountId,
    overdraft_flag: bool,
    overdraft_available_amount: Fixed,
) {
    //Metadata general
    let limits = MetadataLimits::new(256, 256);

    let (mut instructions_metadata, mut overdraft_metadata) =
        (Metadata::new(), Metadata::new());

    // Transaction instructions metadata
    let (instructions, asset, value, dest_account) = (
        Name::from_str("instructions").unwrap(),
        Name::from_str("asset").unwrap(),
        Name::from_str("assetValue").unwrap(),
        Name::from_str("destAccount").unwrap(),
    );

    instructions_metadata
        .insert_with_limits(asset, Value::from(asset_id), limits)
        .unwrap();
    instructions_metadata
        .insert_with_limits(value, Value::from(asset_value), limits)
        .unwrap();
    instructions_metadata
        .insert_with_limits(dest_account, Value::from(destination_account), limits)
        .unwrap();

    // Overdraft metadata
    let (overdraft, available, available_amount) = (
        Name::from_str("overdraft").unwrap(),
        Name::from_str("available").unwrap(),
        Name::from_str("available_amount").unwrap(),
    );

    overdraft_metadata
        .insert_with_limits(available, Value::from(overdraft_flag), limits)
        .unwrap();
    overdraft_metadata
        .insert_with_limits(
            available_amount,
            Value::from(overdraft_available_amount),
            limits,
        )
        .unwrap();



    get_client()
        .submit_all_blocking(vec![SetKeyValueExpr::new(account_id.clone(), instructions, instructions_metadata), SetKeyValueExpr::new(account_id, overdraft, overdraft_metadata)])
        .unwrap();
}
