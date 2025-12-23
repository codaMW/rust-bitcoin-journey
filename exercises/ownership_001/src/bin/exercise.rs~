#[derive(Debug)]
#[allow(dead_code)]

struct Transaction {
    txid: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
}



fn consume_transaction(tx: Transaction) -> Transaction {
    println!("{:?}", tx);
    return tx;
}

fn inspect_transaction(tx: &Transaction) {

    println!("inputs {:?}\noutputs {:?}", &tx.inputs, &tx.outputs);
}
fn main() {
    let tx = Transaction {
        txid: "abc123".to_string(),
        inputs: vec!["input1".to_string(), "input2".to_string()],
        outputs: vec!["output1".to_string()],
    };
    
    inspect_transaction(&tx);

    let tx = consume_transaction(tx);

    inspect_transaction(&tx);
}
