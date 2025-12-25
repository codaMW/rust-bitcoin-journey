mod combo;

use combo::{Network, AddressType, Address};


fn main() {

    let p2w_addr = Address {
        payload: String::from("0f3a9c2e7b1d"),
        network: Network::Mainnet,
        address_type: AddressType::P2WPKH{ pubkey_hash: String::from("P2WPKH")},
    };


    let p2t_addr = Address {
        payload: String::from("a4c91d77e2b8"),
        network: Network::Testnet,
        address_type: AddressType::P2TR{ x_only_pubkey: String::from("P2TR")},
    };
    
    println!("{:?}", p2w_addr);
    println!("{:?}", p2t_addr);

    println!("{}",Address::validate_network(&p2t_addr));
}
