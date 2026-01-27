/*
========================================================================
                            EXERCISE
========================================================================
Your JSON input:
    {
    "txid": "a1b2c3d4...",
    "version": 2,
    "locktime": 0,
    "inputs": [
    {"txid": "prev123", "vout": 0, "scriptSig": "abc..."},
    {"txid": "prev456", "vout": 1, "scriptSig": "def..."}
    ],
    "outputs": [
    {"value": 50000, "scriptPubKey": "xyz..."},
    {"value": 25000, "scriptPubKey": "uvw..."}
    ]
}

Tasks:
    1. Create structs for Transaction, Input, and Output
    2. Parse the JSON above
    3. Calculate total input/output values
    4. Calculate total value of the outputs for this transaction
    5. Serialize it back to json
*/

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};


//Creating Transaction struct
#[derive(Debug, Deserialize, Serialize)]
struct Transaction {
    txid: String,
    version: u32,
    locktime: u32,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}


//Creating Input struct
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Input {
    txid: String,
    vout: u32,
    script_sig: String,
}


//Creating Output struct
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Output {
    value: u64,
    script_pub_key: String,
}

fn main() {
    let json_str = r#"{"txid": "a1b2c3d4...", "version": 2, "locktime": 0, "inputs": [{"txid": "prev123", "vout": 0, "scriptSig": "abc..."},
    {"txid": "prev456", "vout": 1, "scriptSig": "def..."}], "outputs": [{"value": 50000, "scriptPubKey": "xyz..."},{"value": 25000, "scriptPubKey": "uvw..."}
    ]}"#;
    
    //parsing the json_str to the Rust struct
    let parse_json = from_str::<Transaction>(json_str);

    match &parse_json {
        Ok(val) => println!("{:#?}", val),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n");

    //Calculating total inputs
    match &parse_json {
        Ok(val) => println!("Total number of inputs: {:#?}", val.inputs.len()),
        Err(e) => println!("Error: {}", e),
    }

    //Calculate total outputs
    match &parse_json {
        Ok(val) => println!("Total number of outputs: {:#?}", val.outputs.len()),
        Err(e) => println!("Error: {}", e),
    }


    //Calculating total value of the outputs for this transaction
    match &parse_json {
        Ok(val) => println!("Total value of this transaction: {:#?}", val.outputs.iter().map(|x| x.value).sum::<u64>()),
        Err(e) => println!("Error: {}", e),
    }

    //Serializinf our Rust struct, back to json
    match &parse_json {
        Ok(val) => {
            let json_again = to_string_pretty(&val);

            match json_again {
                Ok(value) => println!("{}", value),
                Err(er) => println!("Error: {}", er),
            };
        },
        Err(e) => println!("Error: {}", e),
    }
    

}