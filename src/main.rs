mod utils;
mod config;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use iroha_client::client::Client;
use iroha_data_model::account::{Account, AccountId};
use iroha_data_model::events::{TriggeringFilterBox};
use iroha_data_model::isi::*;
use iroha_data_model::prelude::{AcceptAll, AccountEventFilter, AccountFilter, Action, BySome, Metadata, MetadataLimits, Repeats, Trigger, TriggerId, WasmSmartContract};
use iroha_data_model::prelude::DataEntityFilter::ByAccount;
use iroha_data_model::query::account::model::FindAccountById;
use iroha_data_model::query::QueryBox::FindAllAccounts;


use crate::config::{get_config, get_config_path};


fn main() -> Result<(), Box<dyn Error>> {

    let config = get_config(get_config_path()?);
    let iroha_client: Client = Client::new(&config)?;

    let admin_account_id = "alice@wonderland".parse().unwrap();

    let mut file = File::open("C:\\RustProjects\\iroha_stable\\src\\resources\\create_asset_for_any_user_smartcontract.wasm").unwrap();
    let mut buff = Vec::new();
    file.read_to_end(&mut buff).unwrap();

    let trigger_id = TriggerId::new("wonderland".parse().ok(), "create_asset_for_any_user_smartcontract".parse().ok().unwrap());

    let register_trigger =
        RegisterExpr::new(
            Trigger::new(
                trigger_id,
                Action::new(
                    WasmSmartContract::from_compiled(buff),
                    Repeats::Indefinitely,
                    admin_account_id,
                    TriggeringFilterBox::Data(BySome(ByAccount(BySome(AccountFilter::new(
                        AcceptAll,
                        BySome(AccountEventFilter::ByCreated)))))))));
    iroha_client.submit(register_trigger).unwrap();

    let key_pair = iroha_crypto::KeyPair::generate().unwrap();
    let pk = vec![key_pair.public_key().clone()];
    let account_id: AccountId = "vice@wonderland".parse().unwrap();
    let limits = MetadataLimits::new(256, 256);
    let mut metadata = Metadata::new();
    metadata.insert_with_limits("mintAsset".parse().unwrap(), true.into(), limits).unwrap();
    let new_user = Account::new(account_id.clone(), pk).with_metadata(metadata);

    iroha_client.submit(RegisterExpr::new(new_user)).unwrap();

    let user: Vec<Account> = iroha_client.request(FindAllAccounts()).unwrap();
    let asset_qty = user.asset(&"rose#wonderland".parse().unwrap()).unwrap().value();
    println!("{}", asset_qty);

    Ok(())
}
