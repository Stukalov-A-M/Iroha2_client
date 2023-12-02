mod config;
mod modules;
mod utils;
use crate::config::*;
use crate::modules::block_listener::*;
use iroha_client::client::Client;
use iroha_crypto::KeyPair;
use iroha_data_model::account::NewAccount;
use iroha_data_model::prelude::{Account, AccountId};
use iroha_data_model::IdentifiableBox;
use std::error::Error;
use std::fs::metadata;
use std::process::id;
use std::str::FromStr;
//use iroha_data_model::isi::RegisterExpr;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config(get_config_path()?);
    let iroha_client: Client = Client::new(&config)?;

    //println!("{:?}", iroha_client.get_status()?);

    /*
       let binding = KeyPair::generate().unwrap();
       let pk = binding.public_key();
       let account_id: AccountId ="artem@wonderland".parse().unwrap() ;
       //let register_expression: RegisterExpr = RegisterExpr::new(RegistrableBox::Account(Account::new(account_id, )));
       let account = Account::new(account_id, [pk.clone()]);
       let register_instruction = RegisterExpr::new(account);
       iroha_client.submit(register_instruction).unwrap();


    */

    initiate_block_listener(&iroha_client, 1)?;

    Ok(())
}
