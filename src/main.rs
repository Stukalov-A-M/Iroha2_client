#![feature(iter_collect_into)]

use iroha_client::client::{Client, ResultSet};
use iroha_stable::client::get_client;
use iroha_stable::queries::{get_all_accounts, get_all_asset_definitions};
use std::error::Error;
use iroha_stable::database::*;

fn main() -> Result<(), Box<dyn Error>> {
    let iroha_client: Client = get_client();
    let status = iroha_client.get_status()?;
    println!("{:?}", status);
    println!("-------\n");
    queries::print_all_users();




    Ok(())
}
