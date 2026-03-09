use sha2::{Sha256, Digest};

fn build_message_header(command: &str, payload: &[u8]) -> Vec<u8> {
    let magic_bytes: [u8; 4] = [0xf9, 0xbe, 0xb4, 0xd9];
    
    let mut header = Vec::new();
    header.extend_from_slice(&magic_bytes);

    let mut command_bytes = [0u8; 12];
    let ascii = command.as_bytes();
    command_bytes[..ascii.len()].copy_from_slice(ascii);

    header.extend_from_slice(&command_bytes);
    header
}

fn main() {
    let header = build_message_header("verack", &[]);
    println!("{}", hex::encode(&header));
}