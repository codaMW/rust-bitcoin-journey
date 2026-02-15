use bitcoin::{Address, PublicKey, Network};
use bitcoin::secp256k1::{rand, Secp256k1};

// Generate random key pair.

fn main() 
{
    let s = Secp256k1::new();
    let public_key = PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);

    println!("{}", &public_key);

    // Generate pay-to-pubkey-hash address.
    let address = Address::p2pkh(&public_key, Network::Testnet);

    println!("{}", address)
}
