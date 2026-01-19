use bitcoin::Transaction;
use bitcoin::consensus::deserialize;
use hex::FromHex;

pub fn transaction_parser(raw_hex: &str) {
    let raw_bytes = Vec::<u8>::from_hex(raw_hex).expect("valid hex");

    // Parse / deserialize
    let tx: Transaction = deserialize(&raw_bytes).expect("valid transaction");

    println!("Version: {}", tx.version);
    println!("Number of inputs: {}", tx.input.len());
    println!("Number of outputs: {}", tx.output.len());
    println!("Locktime: {}", tx.lock_time);

    // Access fields
    for input in &tx.input {
        println!(
            "Prevout: {}:{}",
            input.previous_output.txid, input.previous_output.vout
        );
    }

    for output in &tx.output {
        println!("Value: {} sat", output.value);
        println!("ScriptPubKey: {}", output.script_pubkey);
    }
}
