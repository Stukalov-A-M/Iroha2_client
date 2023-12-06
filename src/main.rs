mod config;
mod modules;
mod utils;

use crate::config::*;
use crate::modules::block_listener::initiate_block_listener;
use crate::modules::web_socket_client;
use iroha_client::client::Client;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config(get_config_path()?);

    let iroha_client: Client = Client::new(&config)?;

    match iroha_client.get_status() {
        Ok(status) => println!("Successful connection. Status: {:?}", status),
        Err(e) => println!("Connection error: {:?}", e),
    }

    //web_socket_client::socket_init().unwrap();

    /*
    //Register a user
        Name::from_str("artem")?, DomainId::new(Name::from_str("wonderland")?, ())
    let domain: DomainId = DomainId::{name: Name = Name::from_str(wonderland)};
    let account_id: AccountId = AccountId::new(Name::from_str("artem")?,DomainId::new(Name::from_str("wonderland")?));
    let register_expression: RegisterExpr = RegisterExpr::new(RegistrableBox::Account(Account::new(account_id, )));
    let register_instruction: InstructionExpr = InstructionExpr::Register(RegisterExpr::new(RegistrableBox::Account()))
    let instructions: Vec<InstructionExpr> =
    let execute = Executable::Instructions()
     */
    initiate_block_listener(&iroha_client, 1)?;

    Ok(())
}
