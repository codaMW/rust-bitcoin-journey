trait Eligible {
    fn valid(&self) -> bool;

    fn disp(&self) -> String;
}

#[allow(unused)]
struct Wallet {
    address: String,
    amount: f64,
}

impl Eligible for Wallet {
    fn valid(&self) -> bool {
        let valid_address = self.address.starts_with("bc1");
        let valid_balance = self.amount > 0.0;
        valid_address && valid_balance
    }

    fn disp(&self) -> String {
        format!("Address: {} | Amount: {} sats", self.address, self.amount)
    }
}

fn get_value<T>(item: &T) where T: Eligible {

    if item.valid() == true {
        println!("{}",item.disp());
        println!("Eligible for airdrop");
    } else {
        println!("Wallet not eligible for airdrop");
    }

    

}

fn main() {

    let trans = Wallet {
        address: String::from("bc1qE9ol"),
        amount: 55.78,
    };
    get_value(&trans);

    let trans_2 = Wallet {
        address: String::from("\n1bcqE9ol"),
        amount: 100.78,
    };
    get_value(&trans_2);

    
}
