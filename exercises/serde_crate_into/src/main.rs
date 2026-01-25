use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct UTXO {
    txid: String,
    vout: u8,
}

fn main() {
    let utxo1: UTXO = UTXO {
        txid: "2c7bf1ca94bf5311645436de8a8381e185e35db0ec2833dfccb19ab5be42e2ce".to_string(),
        vout: 0,
    };

    let json_rep = serde_json::to_string(&utxo1).unwrap();

    println!("JSON: {}", json_rep);

    let back_to_block: UTXO = serde_json::from_str(&json_rep).unwrap();

    println!("{:#?}", back_to_block);
}
