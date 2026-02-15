fn get_transaction_fee(tx_id: u32) -> Option<u64> {
    if tx_id == 1 { Some(500) }       // 500 satoshis
    else if tx_id == 2 { Some(1200) } // 1200 satoshis
    else if tx_id == 3 { Some(300) }  // 300 satoshis
    else { None }                     // unknown transaction
}

fn main() {
    let fee_a = get_transaction_fee(2);
    let fee_b = get_transaction_fee(7); // doesn't exist

    // TASK 1:
    // match on fee_a
    // If Some — print "Transaction fee is X satoshis"
    // If None — print "Transaction not found"

    // TASK 2:
    // match on fee_b
    // Same messages

    // TASK 3:
    // match on get_transaction_fee(3) directly
    // If Some — print the fee DOUBLED (multiply val by 2 inside the match arm)
    // If None — print "Transaction not found"

    //TASK 1
    match fee_a{
        Some(val) => println!("Transaction fee is {} satoshis", val),
        None => println!("Transaction not found"),
    }

    //TASK 2
    match fee_b{
        Some(val) => println!("Transaction fee is {} satoshis", val),
        None => println!("Transaction not found"),
    }

    //TASK 3
    match get_transaction_fee(3) {
        Some(val) => println!("fee Doubled {} satoshis", val * 2),
        None => println!("Transaction not found"),
    }

    
}