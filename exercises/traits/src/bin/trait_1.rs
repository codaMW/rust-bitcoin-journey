trait Validate {
    fn is_valid(&self) -> bool;
}

struct BitcoinAddress {address: String}
impl Validate for BitcoinAddress {
    fn is_valid(&self) -> bool {
        self.address.starts_with("bc1") 
            || self.address.starts_with("bcrt1")
            || self.address.starts_with("1")
            || self.address.starts_with("3")
    }
}

struct TransactionId {txid: String} 
impl  Validate for TransactionId {
    fn is_valid(&self) -> bool {
        self.txid.len() == 64 
    }
}

struct BlockHash {hash: String}
impl Validate for BlockHash {
    fn is_valid(&self) -> bool {
        if self.hash.starts_with("00") {
            true
        } else {
            false
        }
    }
}


fn check_validity<T>(item: &T) where T: Validate {

    if item.is_valid() {
        println!("✓ VALID");
    } else {
        println!("✗ INVALID");
    }
    
}


fn main() {

    let regtest_segwit = BitcoinAddress {address: String::from("bcrt1qq2yshcmzdlznnpxx258xswqlmqcxjs4dssfxt2")};
    let check = BitcoinAddress {address: String::from("bc1")};
    let invalid_addr = BitcoinAddress {address: String::from("invalidaddress123")};
    check_validity(&check);
    check_validity(&regtest_segwit);
    check_validity(&invalid_addr);

    let valid_len = TransactionId {txid: String::from("05fa7843326d501fecbf7870c887064ce10b6dd18f5b1a232821aa98dea4c0e8")};
    let invalid_len = TransactionId {txid: String::from("abc123")};
    check_validity(&valid_len);
    check_validity(&invalid_len);

    let valid_block = BlockHash {hash: String::from("00000000000000000002a7c4c1e48d76c5a37902165a270156b7a8d72728a054")};
    let invalid_block = BlockHash {hash: String::from("abcdef1234")};

    check_validity(&valid_block);
    check_validity(&invalid_block);

}

