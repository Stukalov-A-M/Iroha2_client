#![feature(iter_collect_into)]

use iroha_client::client::Client;
use iroha_stable::account::register_new_account;
use iroha_stable::client::get_client;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _iroha_client: Client = get_client();
    register_new_account("artem@wonderland");

    Ok(())
}
