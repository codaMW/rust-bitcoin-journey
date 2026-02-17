fn get_fee_rate(tx_id: u32) -> Option<u64> {
    if tx_id == 1 { Some(25) }   // 25 sat/byte — good
    else if tx_id == 2 { Some(8) }   // 8 sat/byte — too low
    else if tx_id == 3 { Some(10) }  // exactly 10 — acceptable
    else if tx_id == 4 { Some(100) } // 100 sat/byte — high priority
    else if tx_id == 5 { Some(9) }   // 9 sat/byte — just under limit
    else { None }
}

// YOUR JOB: implement this function
// Return Some(fee_rate) only if tx exists AND fee_rate >= 10
// Otherwise return None
fn get_mineable_tx(tx_id: u32) -> Option<u64> {
    // your code here

    match get_fee_rate(tx_id) {

        Some(val) => {
            if val >= 10 {Some(val)}
            else {None}
        },
        None => None,
    }
}

fn main() {
    let test_ids = [1, 2, 3, 4, 5, 99];

    for id in test_ids {
        match get_mineable_tx(id) {
            Some(rate) => println!("tx {} is mineable at {} sat/byte", id, rate),
            None       => println!("tx {} rejected or missing", id),
        }
    }
}
