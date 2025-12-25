mod enum_01;
use enum_01::Network;

fn main() {

    let mainnet = Network::Mainnet;
    let testnet = Network::Testnet;
    let signet = Network::Signet;
    let regtest = Network::Regtest;

    println!("\n{}", Network::magic_bytes(mainnet));
    println!("\n{}", Network::magic_bytes(testnet));
    println!("\n{}", Network::magic_bytes(signet));
    println!("\n{}", Network::magic_bytes(regtest));



}
