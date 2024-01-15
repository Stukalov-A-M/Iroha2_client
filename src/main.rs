mod utils;
mod config;

use std::error::Error;
use std::io::Read;
use iroha_client::client::{Client};
use iroha_data_model::{HasMetadata, Identifiable};
use iroha_data_model::isi::*;
use serde::de::MapAccess;


use crate::config::{get_config, get_config_path};


fn main() -> Result<(), Box<dyn Error>> {

    let config = get_config(get_config_path()?);
    let iroha_client: Client = Client::new(&config)?;


    Ok(())
}
