mod trans_lib;
use trans_lib::{Version, Inputs, Outputs, Transaction};


fn main() {
    let input_0 = Inputs {
        txid: String::from("4529ed23960cb28c4e77b3f3656dabffdf1879d48c2a714d2b9cd9fdf2281886"),
        index: 0,
        script_len: 146,
        script_sig: String::from("3045022100e18c27bbb0b6e924702b60339069765f96f9ca68a2339b01"),
        nsequence: String::from("0xffffffff"),
    };

    let input_1 = Inputs {
        txid: String::from("fc12dfcb4723715a456c6984e298e00c479706067da81be969e8085544b0ba08"),
        index: 1,
        script_len: 146,
        script_sig: String::from("31002207a016023c2b0c4db9a7d4f9232fcec2193c2f119a69125ad5b"),
        nsequence: String::from("0xffffffff"),
    };

    let output_0 = Outputs {
        amount: 20_00_505,
        script_len: 100,
        script_pub_key: String::from("DUP HASH160 PUSHBYTES_20 484d EQUALVERIFY CHECKSIG"),
    };

    let output_1 = Outputs {
        amount: 1_106_475,
        script_len: 100,
        script_pub_key: String::from("DUP HASH160 PUSHBYTES_20 41e8 EQUALVERIFY CHECKSIG"),
    };

    let transaction_1 = Transaction {
        version: Version::V1(1),
        input_counter: 2,
        input: vec![input_0, input_1],
        output_counter: 2,
        output: vec![output_0, output_1],
        locktime: 0,
    };

    println!("version: {}", Version::get_version(&transaction_1.version));
    println!(
        "input count: {}",
        Transaction::input_counter_vec(&transaction_1)
    );
    println!("{:#?}", transaction_1.input);
    println!(
        "output count: {}",
        Transaction::output_counter_vec(&transaction_1)
    );
    println!("{:#?}", transaction_1.output);
}
