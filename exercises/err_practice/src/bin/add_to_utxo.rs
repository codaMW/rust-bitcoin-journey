struct Utxo {
    amount: u64,
    // other fields omitted
}

fn total_with_fee(utxo: Option<&Utxo>) -> Option<u64> {
    // Add 1000 sats fee if UTXO exists
    // If UTXO amount is 50000, return Some(51000)
    // If None, return None

    utxo.map(|x| x.amount + 1000)

}

fn main() {
    let utxo = Utxo {amount: 50000};

    println!("update utxo amount = {}",total_with_fee(Some(&utxo)).unwrap_or(0));
}