
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TRANS {
    tx_id: String,
    amount_sats: u64,
    confirmed: bool,
}

fn main() {

    let json_str = r#"
    {
        "txId": "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b", 
        "amountSats": 50000,
        "confirmed": true
    }"#;

    let trans: Result<TRANS, _> = serde_json::from_str::<TRANS>(&json_str);

    match trans {
        Ok(val) => println!("{:#?}", val),
        Err(_) => println!("Unable to parse Json file"),
    }
}

