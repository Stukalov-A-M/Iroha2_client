
use std::env;
use std::error::Error;
use std::fs::File;
use iroha_client::client::Client;
use std::path::{Path, PathBuf};
use iroha_config::client::Configuration;
use iroha_crypto::KeyPair;
use iroha_data_model::account::Account;
use iroha_data_model::isi::RegisterExpr;
use iroha_data_model::RegistrableBox::*;


fn main() -> Result<(), Box<dyn Error>> {

    //It's supposed that you have already passed the "Configuration" steps earlier
    let config = get_config(get_config_path()?);
    let iroha_client: Client = Client::new(&config)?;

    //Creating a key pair for a new account
    //The key pair must be given to the new account owner
    let key_pair = KeyPair::generate().unwrap();
    //Now you can execute public|private keys by using functions of key_pair variable
    let public_key = vec![key_pair.public_key().clone()];
    //Creating a NewAccount type
    let new_account = Account::new("alex@wonderland".parse()?, public_key);
    //And finally submit the transaction
    let tx = iroha_client.submit(RegisterExpr::new(new_account))?;
    //Optional. To look at the transaction hash
    print!("{:?}", tx);

    Ok(())
}

pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = env::current_exe();
    let binding = exe_path?;
    let ancestors = binding.ancestors();
    for ancestor in ancestors {
        if ancestor.file_name() == Some("target".as_ref()) {
            let source_path = Path::new(ancestor);
            let mut config_path = PathBuf::from(source_path.parent().unwrap());
            config_path.push("src");
            config_path.push("resources");
            config_path.push("config.json");
            return Ok(config_path);
        }
    }
    Err("The source directory was not found in the ancestor path.".into())
}
pub fn get_config(path_buf: PathBuf) -> Configuration {
    let file =
        File::open(&path_buf).unwrap_or_else(|_| panic!("Failed to read file at: {path_buf:?}"));
    serde_json::from_reader(file).unwrap_or_else(|_| panic!("Failed to read config"))
}