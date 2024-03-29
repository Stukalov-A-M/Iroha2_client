#![feature(iter_collect_into)]

use iroha_client::client::Client;
use iroha_stable::client::get_client;
use iroha_stable::database::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let iroha_client: Client = get_client();
    let status = iroha_client.get_status()?;
    println!("{:?}", status);
    println!("-------");
    queries::print_all_users();

    queries::add_user("Artem".to_string(), "123".to_string(), "456".to_string());

    Ok(())
}
