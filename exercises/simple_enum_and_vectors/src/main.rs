#[derive(Debug)]

enum AddressType {

    P2PKH { pubkey_hash: String },
    P2SH { script_hash: String },
    P2WPKH { pubkey_hash: String },
    P2TR { x_only_pubkey: String },
}


fn main() {
    let p2p = AddressType::P2PKH {
        pubkey_hash: "pubdc4be1193c99bf2eb9ee0".to_string(),
    };
    let p2s = AddressType::P2SH {
        script_hash: "7926dc4be1193c99bf2eb9ee0".to_string(),
    };
    let p2w = AddressType::P2WPKH {
        pubkey_hash: "02a83617720912ae0119692e".to_string(),
    };
    let p2t = AddressType::P2PKH {
        pubkey_hash: "c7365983119c890d972d7d59".to_string(),
    };

    let vec = vec![p2p, p2s, p2w, p2t];

    for item in vec {
        println!("{:?}", item);
    }
}
