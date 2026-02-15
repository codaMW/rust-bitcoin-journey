#[derive(Debug, PartialEq)]
enum BitcoinAddressType {
    P2PKH, // Legacy (starts with 1)
    P2SH,  // Segwit (starts with 3)
    Bech32, // Native Segwit (starts with bc1)
    Unknown,
}

fn parse_address_type(address: &str) -> Option<BitcoinAddressType> {
    // TODO: Return the appropriate address type based on the prefix
    // Return None if the address is empty

    if address.is_empty() {
        return None;
    }

    if address.starts_with("1") {
        Some(BitcoinAddressType::P2PKH)
    } else if address.starts_with("3") {
        Some(BitcoinAddressType::P2SH)
    } else if address.starts_with("bc1") {
        Some(BitcoinAddressType::Bech32)

    } else {
        Some(BitcoinAddressType::Unknown)
    }
        
}

#[test]
fn test_parse_address_type() {
    assert_eq!(parse_address_type("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"), Some(BitcoinAddressType::P2PKH));
    assert_eq!(parse_address_type("3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy"), Some(BitcoinAddressType::P2SH));
    assert_eq!(parse_address_type("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq"), Some(BitcoinAddressType::Bech32));
    assert_eq!(parse_address_type(""), None);
    assert_eq!(parse_address_type("invalid"), Some(BitcoinAddressType::Unknown));
}