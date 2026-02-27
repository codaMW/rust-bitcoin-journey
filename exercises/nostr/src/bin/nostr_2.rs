use sha2::{Sha256, Digest};

fn main() {
    let data = r#"[0,"pubkey_hex",1700000000,1,[],"Hello Nostr"]"#;

    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();

    println!("{:x}", result);
}