#![feature(iter_collect_into)]

use iroha_client::client::{Client, QueryResult};
use iroha_stable::client::get_client;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use iroha_data_model::account::AccountId;
use iroha_data_model::asset::AssetId;
use iroha_data_model::isi::TransferExpr;
use iroha_data_model::prelude::{Account, Action, ExecuteTriggerEventFilter, Name, RegisterExpr, Repeats, Trigger, TriggerId, TriggeringFilterBox, WasmSmartContract};
use iroha_data_model::query::account::model::FindAccountById;


fn main() -> Result<(), Box<dyn Error>> {
    let iroha_client: Client = get_client();
    let status = iroha_client.get_status()?;
    println!("{:?}", status);

    let name: Name = "overdraft".parse()?;
    Name::

    println!("{name}");

    Ok(())
}
