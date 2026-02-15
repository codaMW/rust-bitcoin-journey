fn get_block_hash(height: u32) -> Option<String> {
    if height == 100 { Some(String::from("000000abc123")) }
    else if height == 200 { Some(String::from("000000def456")) }
    else { None }
}

fn get_block_size(height: u32) -> Option<u64> {
    if height == 100 { Some(1_200_000) } // 1.2 
    else if height == 200 { Some(980_000) }
    else { None }
}

#[allow(unused)]
fn main() {
    let hash = get_block_hash(100);
    let size = get_block_size(100);

    // TASK 1:
    // Match on BOTH hash and size at the same time
    // Hint: you can match a tuple  →  match (hash, size) { ... }
    // If BOTH are Some — print "Block 000000abc123 is 1200000 bytes"
    // If EITHER is None — print "Incomplete block data"

    match (hash, size) {
    (Some(h), Some(s)) => println!("Block {} is {} bytes", h, s),

    _ => println!("Incomplete block data"),
    }

    // TASK 2:
    // Do the same for height 200

    let hash_2 = get_block_hash(200);
    let size_2 = get_block_size(200);

    match (hash_2, size_2) {
    (Some(h), Some(s)) => println!("Block {} is {} bytes", h, s),

    _ => println!("Incomplete block data"),
    }

    // TASK 3:
    // Do the same for height 999 (doesn't exist)
    // Both will be None — should print "Incomplete block data"

    let hash_3 = get_block_hash(999);
    let size_3 = get_block_size(999);

    match (hash_3, size_3) {
    (Some(h), Some(s)) => println!("Block {} is {} bytes", h, s),

    _ => println!("Incomplete block data"),
    }

}