use serde_json::json;

fn main() {
    let serialized = json!([
        0,
        "pubkey_hex",
        1700000000,
        1,
        [],
        "Hello Nostr"
    ]);

    println!("{}", serialized.to_string());
}