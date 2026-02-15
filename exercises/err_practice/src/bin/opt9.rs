fn get_mempool_tx(tx_id: u32) -> Option<u64> {
    // Returns the fee in satoshis if tx exists in mempool
    if tx_id == 10 { Some(250) }
    else if tx_id == 20 { Some(1800) }
    else if tx_id == 30 { Some(75) }
    else { None }
}

// YOUR JOB: Write this function
// It should:
// 1. Call get_mempool_tx with the given tx_id
// 2. If the tx exists AND the fee is greater than 500 satoshis
//    → return Some(fee)
// 3. If the tx exists BUT fee is 500 or less
//    → return None  (too cheap, we don't care about it)
// 4. If the tx doesn't exist
//    → return None
fn get_high_priority_tx(tx_id: u32) -> Option<u64> {
    // your code here
    match get_mempool_tx(tx_id) {
        Some(val) => {
            if val > 500 {
                Some(val)
            } else {
                None
            }
        }

        None => None
    }
}

fn main() {
    // Test all three cases:

    // tx 20 exists and fee is 1800 — should print "High priority tx found: 1800 satoshis"
    match get_high_priority_tx(20) {
        Some(fee) => println!("High priority tx found: {} satoshis", fee),
        None      => println!("No high priority tx"),
    }

    // tx 10 exists but fee is only 250 — should print "No high priority tx"
    match get_high_priority_tx(10) {
        Some(fee) => println!("High priority tx found: {} satoshis", fee),
        None      => println!("No high priority tx"),
    }

    // tx 99 doesn't exist — should print "No high priority tx"
    match get_high_priority_tx(99) {
        Some(fee) => println!("High priority tx found: {} satoshis", fee),
        None      => println!("No high priority tx"),
    }
}