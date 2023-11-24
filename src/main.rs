mod config;
use iroha_client::client::Client;
use std::num::NonZeroU64;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config();
    let iroha_client: Client = Client::new(&config)?;

    initiate_block_listener(&iroha_client, 1)?;

    Ok(())
}
/// Auxiliary method for a block listener
/// You shall implement it first
fn non_zero_handler(number: u64) -> NonZeroU64 {
    NonZeroU64::new(number).map_or_else(
        || {
            println!("The number must be > 0, using default value 1");
            NonZeroU64::new(1).unwrap()
        },
        |non_zero| non_zero,
    )
}
/// A block listener configuration
/// iroha_client - Your iroha client implementation
/// initial_block_number - The number of a block listener should start from.
/// To get total quantity of blocks, you may use method iroha_client.get_status().
fn initiate_block_listener(iroha_client: &Client, initial_block_number: u64) -> Result<(), Box<dyn std::error::Error>> {
    // Processing the non zero value from the u64
    let block_number = non_zero_handler(initial_block_number);
    // Initiating the block listener object
    let block_iter = iroha_client.listen_for_blocks(block_number)?;
    // Initiating iteration by blocks. The iterator is infinite
    for block in block_iter {
        match &block {
            Ok(block) => println!("Received block: {}", block.payload().to_string()),
            Err(e) => println!("Error happened: {}", e)
        }
    }
    Ok(())
}
