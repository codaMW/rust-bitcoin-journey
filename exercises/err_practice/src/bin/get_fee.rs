#[allow(unused)]
struct Transaction {
    txid: String,
    fee: u64,
    size: usize,
}

fn get_tx_fee(tx: Option<&Transaction>) -> Option<u64> {
    // Hint: You're extracting a single field from the struct
    // If you have a transaction, get its fee
    // If no transaction, return None

    tx.map(|x| x.fee)
}


fn main() {
        let tx = Transaction {
        txid: String::from("0xweyvk"),
        fee: 2000,
        size: 500,
    };

    let fee = get_tx_fee(Some(&tx));

    println!("fee = {} sat/vBytes", fee.unwrap_or(0));
}