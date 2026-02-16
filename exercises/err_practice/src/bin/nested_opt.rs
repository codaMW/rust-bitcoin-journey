fn get_block(block_id: u32) -> Option<Vec<u32>> {
    if block_id == 1 { Some(vec![10, 20, 30]) }
    else if block_id == 2 { Some(vec![40, 50]) }
    else { None }
}

fn find_tx_in_block(block_id: u32, tx_id: u32) -> Option<Option<u32>> {
    match get_block(block_id) {
        None        => None,
        Some(txs)   => {
            match txs.iter().find(|&&t| t == tx_id) {
                Some(_) => Some(Some(tx_id)),  // tx found → what do you return?
                None    => Some(None),  // tx not found → what do you return?
            }
        }
    }
}

fn main() {
    // Case 1: block exists AND tx is in it → Some(Some(20))
    match find_tx_in_block(1, 20) {
        Some(Some(tx)) => println!("Found tx {} in block", tx),
        Some(None)     => println!("Block exists but tx not found"),
        None           => println!("Block does not exist"),
    }

    // Case 2: block exists BUT tx is NOT in it → Some(None)
    match find_tx_in_block(1, 99) {
        Some(Some(tx)) => println!("Found tx {} in block", tx),
        Some(None)     => println!("Block exists but tx not found"),
        None           => println!("Block does not exist"),
    }

    // Case 3: block does NOT exist → None
    match find_tx_in_block(99, 20) {
        Some(Some(tx)) => println!("Found tx {} in block", tx),
        Some(None)     => println!("Block exists but tx not found"),
        None           => println!("Block does not exist"),
    }
}