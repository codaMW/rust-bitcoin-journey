fn get_tx(tx_id: u32) -> Option<(u64, u32)> {
    //                           ^^^  ^^^
    //                           fee  size in bytes
    if tx_id == 1 { Some((500, 250)) }   // fee=500, size=250 bytes
    else if tx_id == 2 { Some((100, 250)) }  // fee too low
    else if tx_id == 3 { Some((500, 600)) }  // size too large
    else if tx_id == 4 { Some((800, 400)) }  // both good
    else if tx_id == 5 { Some((50, 600)) }   // both bad
    else { None }
}

// YOUR JOB: implement this function
// A tx is acceptable ONLY if:
//   fee  >= 400 satoshis   AND
//   size <= 500 bytes
// Return Some((fee, size)) if both conditions pass
// Return None if either condition fails OR tx doesn't exist
fn get_acceptable_tx(tx_id: u32) -> Option<(u64, u32)> {
    // your code here

    match get_tx(tx_id) {
        Some((val_1, val_2)) => {

            if val_1 >= 400 && val_2 <= 500 {Some((val_1, val_2))}
            else {None}
        },
        None => None,
    }
}

fn main() {
    let test_ids = [1, 2, 3, 4, 5, 99];

    for id in test_ids {
        match get_acceptable_tx(id) {
            Some((fee, size)) => println!("tx {} accepted — fee: {}, size: {} bytes", id, fee, size),
            None              => println!("tx {} rejected or missing", id),
        }
    }
}