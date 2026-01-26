use serde::{Deserialize, Serialize};
use serde_json;

/*
Scenario

You receive a Bitcoin RPC-like JSON representing a transaction summary.
JSON Input
{
  "txId": "fd8591f60c84d8b3610a37c9ed4b543c629e7e1a91df5c1bba74f918108cddd6",
  "confirmations": 6,
  "isCoinbase": false
}

Tasks

* Create a Rust struct called RpcTxSummary
* Use snake_case fields
* Use Serde attributes to match JSON
* Deserialize the JSON
* Print the struct
*/

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct RpcTxSummary {
    tx_id: String,
    confirmations: u64,
    is_coinbase: bool,
}

fn main() {
    let json_tx_summary = r#"
    {
        "txId": "fd8591f60c84d8b3610a37c9ed4b543c629e7e1a91df5c1bba74f918108cddd6",
        "confirmations": 6,
        "isCoinbase": true
    }"#;

    let der_tx_summary: Result<RpcTxSummary, _> =
        serde_json::from_str::<RpcTxSummary>(&json_tx_summary);

    match der_tx_summary {
        Ok(val) => println!("{:#?}", val),
        Err(_) => println!("Error parsing Json data"),
    }
}
