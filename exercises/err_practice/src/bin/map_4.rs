#[allow(unused)]
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

    match wallet {
        Some(x) => wallet.map(|x| x.balance.unwrap() * 2),
        None => None,
    }

}

fn main() {

    let wallet_1 = Wallet {
        name: String::from("Yankho"),
        balance: Some(400),
    };

    let wallet_2 = Wallet {
        name: String::from("Grant"),
        balance: None,
    };

    let doubled = double_wallet_balance(Some(&wallet_1));
    match doubled {
        Some(val) => println!("doubled balance = {} sats", val),
        None => println!("Empty Wallet")
    };

    let wallet_but_no_value = double_wallet_balance(Some(&wallet_2));
    match wallet_but_no_value {
        Some(val) => println!("doubled balance = {} sats", val),
        None => println!("Empty Wallet")
    };

    let no_wallet = double_wallet_balance(None);
    match no_wallet{
        Some(val) => println!("doubled balance = {} sats", val),
        None => println!("Empty Wallet")
    }
}