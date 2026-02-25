trait Signable {
    fn sign(&self, private_key: &str) -> String;
    fn verify(&self, signature: &str, public_key: &str) -> bool;
}

struct TxInput {
    txid: String,
    vout: u32,
    amount: f64,
}

impl Signable for TxInput {
    fn sign(&self, private_key: &str) -> String {
        let raw = format!("{}:{}:{}", self.txid, self.vout, self.amount);
        format!("SIG[{}]OVER[{}]", private_key, raw)
    }

    fn verify(&self, signature: &str, public_key: &str) -> bool {
        // Simulated: in real Bitcoin, ECDSA verification happens here
        signature.starts_with("SIG[") && !public_key.is_empty()
    }
}

fn main() {
    let input = TxInput {
        txid: String::from("a1b2c3d4e5f6"),
        vout: 0,
        amount: 0.75,
    };

    let private_key = "priv_key_wif_abc123";
    let public_key = "pub_key_compressed_xyz";

    let sig = input.sign(private_key);
    println!("Signature: {}", sig);
    println!("Valid: {}", input.verify(&sig, public_key));
}

