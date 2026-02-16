fn get_tx_confirmations(tx_id: u32) -> Option<u32> {
    if tx_id == 1 { Some(12) }   // 12 confirmations — safe
    else if tx_id == 2 { Some(3) }  // 3 confirmations — too few
    else if tx_id == 3 { Some(6) }  // exactly 6 — safe
    else if tx_id == 4 { Some(1) }  // 1 confirmation — too few
    else { None }                   // tx doesn't exist
}

// YOUR JOB: implement this function
// Return Some(confirmations) if tx exists AND has 6 or more confirmations
// Return None if tx doesn't exist OR has fewer than 6 confirmations
fn get_confirmed_tx(tx_id: u32) -> Option<u32> {
    // your code here

    match get_tx_confirmations(tx_id) {
        Some(val) => {
            if val >= 6 {Some(val)}
            else {None}
        },
        None => None
    }
}

fn main() {
    // tx 1 — 12 confirmations → Some(12)
    match get_confirmed_tx(1) {
        Some(c) => println!("tx 1 confirmed with {} confirmations", c),
        None    => println!("tx 1 not confirmed or missing"),
    }

    // tx 2 — 3 confirmations → None
    match get_confirmed_tx(2) {
        Some(c) => println!("tx 2 confirmed with {} confirmations", c),
        None    => println!("tx 2 not confirmed or missing"),
    }

    // tx 3 — exactly 6 → Some(6)
    match get_confirmed_tx(3) {
        Some(c) => println!("tx 3 confirmed with {} confirmations", c),
        None    => println!("tx 3 not confirmed or missing"),
    }

    // tx 4 — 1 confirmation → None
    match get_confirmed_tx(4) {
        Some(c) => println!("tx 4 confirmed with {} confirmations", c),
        None    => println!("tx 4 not confirmed or missing"),
    }

    // tx 99 — doesn't exist → None
    match get_confirmed_tx(99) {
        Some(c) => println!("tx 99 confirmed with {} confirmations", c),
        None    => println!("tx 99 not confirmed or missing"),
    }
}