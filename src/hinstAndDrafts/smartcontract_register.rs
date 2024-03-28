//mod utils;
//mod config;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use iroha_client::client::{Client, ResultSet};
use iroha_data_model::account::{Account, AccountId};
use iroha_data_model::events::{TriggeringFilterBox};
use iroha_data_model::{HasMetadata, Identifiable};
use iroha_data_model::asset::{AssetDefinitionId};
use iroha_data_model::isi::*;
use iroha_data_model::prelude::{AcceptAll, AccountEventFilter, AccountFilter, Action, BySome, FindAllActiveTriggerIds, Metadata, MetadataLimits, Repeats, Trigger, TriggerId, WasmSmartContract};
use iroha_data_model::prelude::DataEntityFilter::ByAccount;
use iroha_data_model::query::account::model::{FindAllAccounts};
use iroha_data_model::query::asset::model::{FindAssetDefinitionById};
use iroha_data_model::query::permission::model::FindPermissionTokensByAccountId;
use serde::de::MapAccess;


use crate::config::{get_config, get_config_path};


fn main() -> Result<(), Box<dyn Error>> {

    let config = get_config(get_config_path()?);
    let iroha_client: Client = Client::new(&config)?;

    let admin_account_id: AccountId = "alice@wonderland".parse().unwrap();

    let mut file = File::open("C:\\RustProjects\\iroha_stable\\src\\resources\\mint_asset_for_created_user_trigger.wasm").unwrap();
    let mut buff = Vec::new();
    file.read_to_end(&mut buff).unwrap();

    let trigger_id = TriggerId::new("wonderland".parse().ok(), "mint_asset_for_created_user_trigger_1".parse().ok().unwrap());

    let register_trigger =
        RegisterExpr::new(
            Trigger::new(
                trigger_id,
                Action::new(
                    WasmSmartContract::from_compiled(buff),
                    Repeats::Indefinitely,
                    admin_account_id.clone(),
                    TriggeringFilterBox::Data(BySome(ByAccount(BySome(AccountFilter::new(
                        AcceptAll,
                        BySome(AccountEventFilter::ByCreated)))))))));
    iroha_client.submit_blocking(register_trigger).unwrap();






    let key_pair = iroha_crypto::KeyPair::generate().unwrap();
    let pk = vec![key_pair.public_key().clone()];
    let account_id: AccountId = "lastChance@wonderland".parse().unwrap();
    let limits = MetadataLimits::new(256, 256);
    let mut metadata = Metadata::new();
    metadata.insert_with_limits("mintAsset".parse().unwrap(), true.into(), limits).unwrap();
    let new_user = Account::new(account_id.clone(), pk).with_metadata(metadata);

    iroha_client.submit_blocking(RegisterExpr::new(new_user)).unwrap();

    //AssetDefinition search and display

    let asset_definition_id: AssetDefinitionId = "cat#wonderland".parse().unwrap();
    let result = iroha_client.request(FindAssetDefinitionById::new(asset_definition_id)).unwrap();
    println!("AssetDefinitionName = {:?}", result);



    //AccoutId + AssetId + AssetValue search and display
    let result: ResultSet<Account> = iroha_client.request(FindAllAccounts).unwrap();

    for user in result {
        for (k,v) in user.as_ref().unwrap().metadata().iter(){
            println!("AccountName = {}, Data = {}, {}", user.as_ref().unwrap().id(), k, v);
        }
        for asset in user.as_ref().unwrap().assets() {
            println!("{} - {} - {}", user.as_ref().unwrap().id(), asset.id(), asset.value())
        }
    }

/*
    let asset_qty = user.asset(&"rose#wonderland".parse().unwrap()).unwrap().value();
    println!("{}", asset_qty);

 */

    Ok(())
}
