fn find_block_height(block_id: u32) -> Option<u32> {
    // Pretend we only have blocks 1, 2, and 3 in our tiny "blockchain"
    if block_id == 1 {
        Some(100)   // block 1 is at height 100
    } else if block_id == 2 {
        Some(200)   // block 2 is at height 200
    } else {
        None        // any other block ID — we don't have it
    }
}

fn main() {
    let height_a = find_block_height(1);
    let height_b = find_block_height(99); // we don't have block 99

    // YOUR JOB: Write a match statement for EACH of these
    // that prints a helpful message either way
    // Don't unwrap(). Use match.
    
    // match height_a { ... }
    // match height_b { ... }

    match height_a {
        Some(val) => println!("Block 1 is at height {}", val),
        None => println!("We don't have block 1"),
    }

    match height_b {
        Some(val) => println!("Block 99 is at height {}", val),
        None => println!("We don't have block 99"),
    }
}