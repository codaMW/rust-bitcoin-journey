// src/bin/addr_val.rs
use bitcoin::address::Address;
use bitcoin::Network;
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::PublicKey;
use rand::rngs::OsRng;
use rand::RngCore;
use std::str::FromStr;

fn main() {
    println!("🔐 Bitcoin Address Validator & Generator");
    println!("=========================================");
    
    // Example: Validate an address
    let test_address = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
    match validate_bitcoin_address(test_address) {
        Ok(is_valid) => println!("Address '{}' is valid: {}", test_address, is_valid),
        Err(e) => println!("Error validating address: {}", e),
    }
    
    // Generate a new key pair
    match generate_keypair() {
        Ok((priv_key, address)) => {
            println!("\n✨ Generated new Bitcoin address:");
            println!("Private key (hex): {}", priv_key);
            println!("Address: {}", address);
            // Get network by requiring network-checked address
            let address = address.require_network(Network::Bitcoin).unwrap();
            println!("Network: {:?}", Network::Bitcoin);
        }
        Err(e) => println!("Generation error: {}", e),
    }
    
    // Test with an invalid address
    let invalid_addr = "invalid_address_123";
    match validate_bitcoin_address(invalid_addr) {
        Ok(is_valid) => println!("\nAddress '{}' is valid: {}", invalid_addr, is_valid),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn validate_bitcoin_address(address: &str) -> Result<bool, String> {
    // Parse address without network validation first
    match Address::from_str(address) {
        Ok(addr) => {
            // Try to validate it against mainnet
            match addr.require_network(Network::Bitcoin) {
                Ok(_) => Ok(true),
                Err(_) => {
                    // Try testnet
                    match addr.require_network(Network::Testnet) {
                        Ok(_) => Ok(true),
                        Err(_) => {
                            // Try signet
                            match addr.require_network(Network::Signet) {
                                Ok(_) => Ok(true),
                                Err(_) => {
                                    // Try regtest
                                    match addr.require_network(Network::Regtest) {
                                        Ok(_) => Ok(true),
                                        Err(_) => Ok(false),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => Err(format!("Invalid address format: {}", e)),
    }
}

pub fn generate_keypair() -> Result<(String, Address<NetworkChecked>), Box<dyn std::error::Error>> {
    use bitcoin::address::NetworkChecked;
    
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    
    // Generate random private key (32 bytes)
    let mut seed = [0u8; 32];
    rng.fill_bytes(&mut seed);
    let secret_key = SecretKey::from_slice(&seed)?;
    
    // Get public key
    let public_key = PublicKey::new(secp256k1::PublicKey::from_secret_key(&secp, &secret_key));
    
    // Create P2PKH address (starts with 1) for mainnet
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    
    Ok((
        secret_key.display_secret().to_string(),
        address
    ))
}

// Helper function to demonstrate different address types
pub fn generate_segwit_address() -> Result<(String, Address<NetworkChecked>), Box<dyn std::error::Error>> {
    use bitcoin::address::NetworkChecked;
    
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    
    let mut seed = [0u8; 32];
    rng.fill_bytes(&mut seed);
    let secret_key = SecretKey::from_slice(&seed)?;
    
    let public_key = PublicKey::new(secp256k1::PublicKey::from_secret_key(&secp, &secret_key));
    
    // Create P2WPKH address (bech32, starts with bc1)
    let address = Address::p2wpkh(&public_key, Network::Bitcoin)?;
    
    Ok((
        secret_key.display_secret().to_string(),
        address
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mainnet_address() {
        let addr = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
        assert!(validate_bitcoin_address(addr).unwrap());
    }

    #[test]
    fn test_valid_testnet_address() {
        let addr = "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx";
        assert!(validate_bitcoin_address(addr).unwrap());
    }

    #[test]
    fn test_valid_segwit_address() {
        let addr = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        assert!(validate_bitcoin_address(addr).unwrap());
    }

    #[test]
    fn test_invalid_address() {
        let addr = "invalid_address_123";
        assert!(validate_bitcoin_address(addr).is_err());
    }

    #[test]
    fn test_generate_keypair() {
        let result = generate_keypair();
        assert!(result.is_ok());
        let (priv_key, address) = result.unwrap();
        assert!(!priv_key.is_empty());
        assert!(address.to_string().starts_with('1') || address.to_string().starts_with('3'));
    }

    #[test]
    fn test_generate_segwit_address() {
        let result = generate_segwit_address();
        assert!(result.is_ok());
        let (priv_key, address) = result.unwrap();
        assert!(!priv_key.is_empty());
        assert!(address.to_string().starts_with("bc1"));
    }
}