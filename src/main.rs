mod utils;
mod config;

use std::error::Error;
use iroha_client::client::Client;

use crate::config::{get_config, get_config_path};


fn main() -> Result<(), Box<dyn Error>> {

    //It's supposed that you have already passed the "Configuration" steps earlier
    let config = get_config(get_config_path()?);
    let iroha_client: Client = Client::new(&config)?;

    let config = iroha_client.get_config();
    print!("{:?}", config);

    Ok(())
}
