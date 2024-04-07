#![feature(iter_collect_into)]

use core::str::FromStr;
use iroha_client::client::{Client, QueryOutput};
use iroha_data_model::account::{Account, AccountId};
use iroha_data_model::asset::AssetId;
use iroha_data_model::name::Name;
use iroha_data_model::prelude::{Metadata, MetadataLimits, QueryBox, UnlimitedMetadata};
use iroha_data_model::{NumericValue, Value};
use iroha_primitives::fixed::Fixed;
use iroha_stable::client::get_client;
use std::error::Error;
use iroha_data_model::query::account::model::FindAccountById;

fn main() -> Result<(), Box<dyn Error>> {
    let iroha_client: Client = get_client();
    let status = iroha_client.get_status()?;
    println!("{:?}", status);

    /*
    let trigger_id = TriggerId::new("cbdc".parse().ok(), "transaction_processing_with_overdraft".parse().ok().unwrap());

    print_all_registered_triggers();
    print_all_accounts_with_assets();
    println!("----------\n");

    iroha_client.submit_blocking(ExecuteTriggerExpr::new(trigger_id));

     */

    /*
       let admin_account_id: AccountId = "admin@cbdc".parse().unwrap();

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
                       admin_account_id.clone(),
                       TriggeringFilterBox::ExecuteTrigger(ExecuteTriggerEventFilter::new(trigger_id, admin_account_id)))));
       iroha_client.submit_blocking(register_trigger).unwrap();


    */

    Ok(())
}

fn account_metadata_configuration(
    account_id: AccountId,
    asset_id: AssetId,
    asset_value: NumericValue,
    destination_account: AccountId,
    overdraft_flag: bool,
    overdraft_available_amount: Fixed,
) {
    let mut account: Account = get_client()
        .request(QueryBox::FindAccountById(FindAccountById::new(
            account_id.clone(),
        )))
        .unwrap()
        .try_into()
        .unwrap();

    let limits = MetadataLimits::new(256, 256);

    let (instructions, asset, value, dest_account) = (
        Name::from_str("instructions").unwrap(),
        Name::from_str("asset").unwrap(),
        Name::from_str("assetValue").unwrap(),
        Name::from_str("destAccount").unwrap(),
    );

    let mut instructions_metadata = Metadata::new(); {}
    instructions_metadata.insert_with_limits(asset, Value::from(asset_id), limits).unwrap();
    instructions_metadata.insert_with_limits(value, Value::from(asset_value), limits).unwrap();
    instructions_metadata.insert_with_limits(dest_account, Value::from(destination_account), limits).unwrap();

    let (overdraft, available, available_amount) = (
        Name::from_str("overdraft").unwrap(),
        Name::from_str("available").unwrap(),
        Name::from_str("available_amount").unwrap(),
    );

    let mut overdraft_metadata = Metadata::new();
    overdraft_metadata.insert_with_limits(available, Value::from(overdraft_flag), limits).unwrap();
    overdraft_metadata.insert_with_limits(available_amount, Value::from(overdraft_available_amount), limits).unwrap();

    let mut account_metadata = Metadata::new();
    account_metadata.insert_with_limits(instructions, Value::from(instructions_metadata), limits);
    account_metadata.insert_with_limits(overdraft, Value::from(overdraft_metadata), limits);
}
