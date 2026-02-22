trait Validate {
    fn is_valid(&self) -> bool;
}

trait Summarize {  
    fn summary(&self) -> String;
}

struct BitcoinAddress { address: String }
impl Validate for BitcoinAddress {
    fn is_valid(&self) -> bool { 
        self.address.starts_with("bc1")
            ||self.address.starts_with("bcrt1")
            ||self.address.starts_with("1")
            ||self.address.starts_with("3")
    }
}

impl Summarize for BitcoinAddress {
    fn summary(&self) -> String {

        format!("Address: {}", self.address)

    }
}
struct TransactionId  { txid: String }
impl Validate for TransactionId {
    fn is_valid(&self) -> bool { 
        self.txid.len() == 64
    }
}

impl Summarize for TransactionId {
    fn summary(&self) -> String {

        format!("TXID: {}", &self.txid[..8])

    }
}



struct BlockHash   { hash: String }
impl Validate for BlockHash {
    fn is_valid(&self) -> bool {
        self.hash.starts_with("00")
    }
}


fn validate_and_report<T>(item: &T)
where
    T: Validate + Summarize,
{
    println!("{}", item.summary());
    if item.is_valid() {
        println!("✓ VALID\n");
    } else {
        println!("✗ INVALID\n");
    }
}


fn just_validate<T>(item: &T)
where
    T: Validate,
{
    if item.is_valid() {
        println!("✓ VALID");
    } else {
        println!("✗ INVALID");
    }
}

fn main() {
    let valid_addr = BitcoinAddress {
        address: String::from("bcrt1qq2yshcmzdlznnpxx258xswqlmqcxjs4dssfxt2"),
    };
    let invalid_addr = BitcoinAddress {
        address: String::from("notanaddress"),
    };
    let valid_txid = TransactionId {
        txid: String::from("05fa7843326d501fecbf7870c887064ce10b6dd18f5b1a232821aa98dea4c0e8"),
    };
    let valid_hash = BlockHash {
        hash: String::from("00000000000000000002a7c4c1e48d76c5a37902165a270156b7a8d72728a054"),
    };

    validate_and_report(&valid_addr);
    validate_and_report(&invalid_addr);

    validate_and_report(&valid_txid);

    just_validate(&valid_hash);

}