// Your JSON input:
[
  {
    "txid": "abc123",
    "vout": 0,
    "amount": 100000,
    "scriptType": "P2PKH",
    "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
  },
  {
    "txid": "def456",
    "vout": 1,
    "amount": 200000,
    "scriptType": "P2WPKH",
    "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
  },
  {
    "txid": "ghi789",
    "vout": 0,
    "amount": 300000,
    "scriptType": "P2TR",
    "address": "bc1p5d7rjq7g6rdk2yhzks9smlaqtedr4dekq08ge8ztwac72sfr9rusxg3297"
  }
]

// Tasks:
// 1. Create a ScriptType enum with variants P2PKH, P2WPKH, P2SH, P2WSH, P2TR
// 2. Create a UTXO struct
// 3. Deserialize the array
// 4. Filter and sum all Segwit UTXOs (P2WPKH, P2WSH, P2TR)
// 5. Serialize back with only Taproot outputs


enum ScriptType {

    #[serde(tag = "Legacy")]
    P2PKH,
    #[serde(tag = "Segwit")]
    P2WPKH,
    P2SH,
    #[serde(tag = "Segwit")]
    P2WSH,
    #[serde(tag = "Segwit")]
    P2TR
}

struct UTXO {
    txid: String,
    vout: u64,
    amount: u64,
    script_type: scriptType,
    address: String,
}