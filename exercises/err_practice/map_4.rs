struct Wallet {
    name: String,
    balance: Option<u64>,  // Balance might be unknown
}

fn double_wallet_balance(wallet: Option<&Wallet>) -> Option<u64> {
    // If wallet exists and has a balance, double it
    // If wallet exists but balance is None, return None
    // If wallet is None, return None
    
    // Hint: You need to handle Option<Option<u64>> carefully!
    // Try using map first, then see what type you get

    wallet.map(|x| x.balance * 2)
}

fn main() {

    let wal = Wallet {
        name: String::from("Yankho"),
        balance: None,
    };

    let dd = double_wallet_balance(Some(&wal));

    println!("{}", dd.unwrap_or(0));
}