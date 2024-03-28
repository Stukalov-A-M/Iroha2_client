mod config;
mod utils;
pub mod database;
pub mod models;
pub mod schema;

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
        for block in block_iter {
            match &block {
                Ok(block) => println!("Received block: {}", block.payload().to_string()),
                Err(e) => println!("Error happened: {}", e),
            }
        }
        Ok(())
    }
}
pub mod account {
    use crate::client::get_client;
    use iroha_crypto::{HashOf, KeyPair};
    use iroha_data_model::prelude::{Account, RegisterExpr, TransactionPayload};

    pub fn register_new_account(account_id: &str) -> HashOf<TransactionPayload> {
        //Creating a key pair for a new account
        //The key pair must be given to the new account owner
        let key_pair = KeyPair::generate().unwrap();
        //Now you can execute public|private keys by using functions of key_pair variable
        let public_key = vec![key_pair.public_key().clone()];
        //Creating a NewAccount type
        let new_account = Account::new(account_id.parse().unwrap(), public_key);
        //And finally submit the transaction
        get_client().submit(RegisterExpr::new(new_account)).unwrap()
    }
}
pub mod asset {
    use crate::client::get_client;
    use iroha_crypto::HashOf;
    use iroha_data_model::prelude::{
        AccountId, Asset, AssetDefinition, AssetDefinitionId, AssetId, AssetValueType, MintExpr,
        RegisterExpr, TransactionPayload,
    };
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
        asset_definition_id: &str,
        account_id: &str,
        quantity: u32,
    ) -> HashOf<TransactionPayload> {
        //Precondition: The asset definition was registered
        // First we need to define the asset definition id
        let asset_definition_id = AssetDefinitionId::from_str(asset_definition_id).unwrap();

        //Then we need to create an assetId object
        //The AssetId is a complex object which consists of asset definition id and account id
        let asset_id: AssetId = AssetId::new(
            asset_definition_id,
            AccountId::from_str(account_id).unwrap(),
        );

        //Now we need to define the asset quantity regarding to the asset's value type
        let asset_quantity = quantity;

        //And finally we need to send the transaction
        get_client()
            .submit_blocking(MintExpr::new(asset_quantity, asset_id))
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

    pub fn get_all_accounts() {
        let iroha_client: Client = get_client();
        let result: ResultSet<Account> = iroha_client.request(FindAllAccounts).unwrap();

        for account in result {
            println!("AccountName = {}", account.as_ref().unwrap())
        }
    }
    pub fn get_all_asset_definitions() {
        let iroha_client: Client = get_client();
        let result: ResultSet<AssetDefinition> =
            iroha_client.request(FindAllAssetsDefinitions).unwrap();

        for asset_definition in result {
            println!("AssetDefinition = {}", asset_definition.as_ref().unwrap())
        }
    }
}
