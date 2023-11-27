use std::error::Error;
use iroha_client::client::Client;
use crate::utils::non_zero_handler::*;

/// A block listener configuration
/// iroha_client - Your iroha client implementation
/// initial_block_number - The number of a block listener should start from.
/// To get total quantity of blocks, you may use method iroha_client.get_status().
pub fn initiate_block_listener(
    iroha_client: &Client,
    initial_block_number: u64,
) -> Result<(), Box<dyn Error>> {
    // Processing the non zero value from the u64
    let block_number = non_zero_handler(initial_block_number);
    // Initiating the block listener object
    let block_iter = iroha_client.listen_for_blocks(block_number)?;
    // Initiating iteration by blocks. The iterator is infinite
    for block in block_iter {
        match &block {
            Ok(block) => println!("Received block: {}", block.payload().to_string()),
            Err(e) => println!("Error happened: {}", e),
        }
    }
    Ok(())
}