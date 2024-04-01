#![feature(iter_collect_into)]

use iroha_client::client::Client;
use iroha_stable::client::get_client;
use std::error::Error;

use iroha_stable::database::queries::print_all_users;
use iroha_stable::queries::get_all_accounts;

fn main() -> Result<(), Box<dyn Error>> {
    let iroha_client: Client = get_client();
    let status = iroha_client.get_status()?;
    println!("{:?}", status);
    get_all_accounts();

    Ok(())
}
