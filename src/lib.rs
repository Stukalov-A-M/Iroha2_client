#![feature(error_reporter)]
#![feature(exact_size_is_empty)]

mod config;
pub mod database;
pub mod models;
pub mod schema;
mod utils;

pub mod client {
    use crate::config::*;
    use iroha_client::client::Client;

    pub fn get_client() -> Client {
        let config = get_config(get_config_path().unwrap());
        Client::new(&config).unwrap()
    }
}
pub mod block_listener {
    use crate::utils::non_zero_handler::*;
    use iroha_client::client::Client;
    use parity_scale_codec::Encode;
    use std::error::Error;

    /// A block listener configuration
    /// iroha_client - Your iroha client implementation
    /// initial_block_number - The number of a block listener should start from.
    /// To get total quantity of blocks, you may use method iroha_client.get_status().
    pub fn initiate_block_listener(
        iroha_client: &Client,
        initial_block_number: u64,
    ) -> Result<(), Box<dyn Error>> {
        // Processing the non zero value from the u64
        let block_number = non_zero_handler(initial_block_number).encode();
        println!("{:?}", block_number);
        println!("{:02X?}", block_number);
        let block_number = non_zero_handler(initial_block_number);
        // Initiating the block listener object
        let block_iter = iroha_client.listen_for_blocks(block_number).unwrap();
        // Initiating iteration by blocks. The iterator is infinite
        block_iter.for_each(|block| match block {
            Ok(block) => println!("Received block: {}", block.payload()),
            Err(e) => println!("Error happened: {}", e),
        });
        Ok(())
    }
}
pub mod account {
    use crate::client::get_client;
    use crate::database::queries::add_user;
    use crate::models::NewUser;
    use eyre::Report;
    use iroha_crypto::{HashOf, KeyPair};
    use iroha_data_model::prelude::{
        Account, AccountId, RegisterExpr, TransactionPayload, UnregisterExpr,
    };

    pub fn register_new_account(account_id: AccountId) -> HashOf<TransactionPayload> {
        let key_pair = KeyPair::generate().unwrap();
        let public_key = vec![key_pair.public_key().clone()];

        let new_account = Account::new(account_id.clone(), public_key);

        let user = NewUser {
            name: account_id.to_string(),
            publicKey: key_pair.public_key().to_string(),
            privateKey: key_pair.private_key().to_string(),
        };

        add_user(user);

        get_client()
            .submit_blocking(RegisterExpr::new(new_account))
            .unwrap()
    }

    pub fn unregister_account(account_id: AccountId) -> Result<(), Report> {
        match get_client().submit_blocking(UnregisterExpr::new(account_id.clone())) {
            Ok(..) => {
                println!("{} is unregistered", account_id);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
pub mod asset {
    use crate::client::get_client;
    use iroha_crypto::HashOf;
    use iroha_data_model::prelude::{
        AccountId, Asset, AssetDefinition, AssetDefinitionId, AssetId, AssetValueType, MintExpr,
        RegisterExpr, TransactionPayload,
    };
    use iroha_data_model::NumericValue;
    use std::str::FromStr;

    pub fn register_asset_definitions(asset_definition_id: &str) -> HashOf<TransactionPayload> {
        let asset_definition_id = AssetDefinitionId::from_str(asset_definition_id).unwrap();

        //Then we need to define the asset value type
        //There are only 4 asset value types and they are defined in the AssetValueType struct
        let asset_value_type: AssetValueType = AssetValueType::Quantity;

        //Then we need to create an asset definition object
        let asset_definition = AssetDefinition::new(asset_definition_id, asset_value_type);

        //And finally we need to send the transaction
        get_client()
            .submit(RegisterExpr::new(asset_definition))
            .unwrap()
    }

    pub fn register_asset(
        asset_definition_id: &str,
        account_id: &str,
        quantity: u32,
    ) -> HashOf<TransactionPayload> {
        //Precondition: The asset definition was registered
        // First we need to define the asset definition id
        let asset_definition_id = AssetDefinitionId::from_str(asset_definition_id).unwrap();

        //Then we need to create an assetId object
        //The AssetId is a complex object which consists of asset definition id and account id
        //And the Asset also is a complex object which consists of asset id and its quantity
        let asset_id: AssetId = AssetId::new(
            asset_definition_id,
            AccountId::from_str(account_id).unwrap(),
        );
        let asset: Asset = Asset::new(asset_id, quantity);

        //And finally we need to send the transaction
        get_client()
            .submit_blocking(RegisterExpr::new(asset))
            .unwrap()
    }

    pub fn mint_asset(
        asset_definition_id: AssetDefinitionId,
        account_id: AccountId,
        quantity: NumericValue,
    ) -> HashOf<TransactionPayload> {
        //Then we need to create an assetId object
        //The AssetId is a complex object which consists of asset definition id and account id
        let asset_id: AssetId = AssetId::new(asset_definition_id, account_id);

        //And finally we need to send the transaction
        get_client()
            .submit_blocking(MintExpr::new(quantity, asset_id))
            .unwrap()
    }
}

pub mod queries {
    use crate::client::get_client;
    use iroha_client::client::{Client, ResultSet};
    use iroha_data_model::account::Account;
    use iroha_data_model::asset::AssetDefinition;
    use iroha_data_model::query::account::model::FindAllAccounts;
    use iroha_data_model::query::asset::model::FindAllAssetsDefinitions;

    pub fn get_all_accounts() -> ResultSet<Account> {
        let iroha_client: Client = get_client();
        let result: ResultSet<Account> = iroha_client.request(FindAllAccounts).unwrap();
        result
    }

    pub fn print_all_accounts_with_assets() {
        let accounts =
            get_all_accounts().filter(|account| !account.as_ref().unwrap().assets().is_empty());
        for account in accounts {
            for asset in account.as_ref().unwrap().assets() {
                println!("Account = {}, Asset = {}", account.as_ref().unwrap(), asset)
            }
        }
    }
    pub fn get_all_asset_definitions() {
        let iroha_client: Client = get_client();
        let result: ResultSet<AssetDefinition> =
            iroha_client.request(FindAllAssetsDefinitions).unwrap();

        result.for_each(|asset_definition| {
            println!("AssetDefinition = {}", asset_definition.unwrap())
        });
    }
}
pub mod domain {
    use crate::client::get_client;
    use iroha_client::client::ResultSet;
    use iroha_crypto::HashOf;
    use iroha_data_model::prelude::{Domain, RegisterExpr, TransactionPayload};
    use iroha_data_model::query::domain::model::FindAllDomains;

    pub fn register_domain(name: &str) -> HashOf<TransactionPayload> {
        get_client()
            .submit_blocking(RegisterExpr::new(Domain::new(name.parse().unwrap())))
            .unwrap()
    }

    pub fn print_all_domains() {
        let result: ResultSet<Domain> = get_client().request(FindAllDomains).unwrap();

        result
            .into_iter()
            .for_each(|domain| println!("Domain = {}", domain.unwrap()))
    }
}
pub mod trigger {
    use crate::client::get_client;
    use iroha_data_model::prelude::FindAllActiveTriggerIds;

    pub fn print_all_registered_triggers() {
        let result = get_client().request(FindAllActiveTriggerIds).unwrap();
        result
            .into_iter()
            .for_each(|trigger_id| println!("Active trigger = {}", trigger_id.unwrap()))
    }
}
