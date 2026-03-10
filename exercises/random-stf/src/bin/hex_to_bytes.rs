fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Invalid hex length".to_string());
    }

    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

fn main() {
    let hex = "deadbeef";

    match hex_to_bytes(hex) {
        Ok(bytes) => println!("Bytes: {:?}", bytes),
        Err(e) => println!("Error: {}", e),
    }
}
