mod config;
mod modules;
mod utils;
use crate::modules::block_listener::*;
use crate::config::*;
use iroha_client::client::Client;
use std::error::Error;



fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config(get_config_path()?);
    let iroha_client: Client = Client::new(&config)?;
    initiate_block_listener(&iroha_client, 1)?;

    Ok(())
}


