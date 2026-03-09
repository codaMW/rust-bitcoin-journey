use sha2::{Sha256, Digest};

fn build_message_header(command: &str, payload: &[u8]) -> Vec<u8> {
    let magic_bytes: [u8; 4] = [0xf9, 0xbe, 0xb4, 0xd9];
    
    let mut header = Vec::new();
    header.extend_from_slice(&magic_bytes);
    header
}

fn main() {
    let header = build_message_header("verack", &[]);
    println!("{}", hex::encode(&header));
}