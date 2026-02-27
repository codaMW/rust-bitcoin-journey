use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    id: String,
    pubkey: String,
    created_at: u64,
    kind: u32,
    tags: Vec<Vec<String>>,
    content: String,
    sig: String,
}

fn main() {
    let event = Event {
        id: String::new(),
        pubkey: "pubkey_hex".to_string(),
        created_at: 1700000000,
        kind: 1,
        tags: vec![],
        content: "Hello Nostr".to_string(),
        sig: String::new(),
    };

    println!("{:?}", event);
}