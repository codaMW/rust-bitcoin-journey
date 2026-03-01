use bitcoincore_rpc::{Auth, Client, RpcApi};
use anyhow::Result;

fn main() -> Result<()> {
    let client = Client::new(
        "http://127.0.0.1:18443",
        Auth::UserPass("alice".to_string(), "password".to_string())
    )?;
    
    // Get basic info
    println!("=== Blockchain Info ===");
    let info = client.get_blockchain_info()?;
    println!("Difficulty: {}", info.difficulty);
    println!("Size on disk: {} MB", info.size_on_disk / 1_000_000);

      // Get network info
    println!("\n=== Network Info ===");
    let network_info = client.get_network_info()?;
    println!("Version: {}", network_info.version);
    println!("Subversion: {}", network_info.subversion);
    println!("Connections: {}", network_info.connections);
    
    // Get mining info
    println!("\n=== Mining Info ===");
    let mining_info = client.get_mining_info()?;
    println!("Network hash rate: {}", mining_info.network_hash_ps);
    println!("Pooled transactions: {}", mining_info.pooled_tx);
    
    Ok(())
}
    