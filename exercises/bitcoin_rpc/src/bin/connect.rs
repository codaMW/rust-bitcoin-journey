use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::time::Duration;

fn connect() -> Result<Client> {
    // Method 1: Basic authentication
    let client1 = Client::new(
        "http://127.0.0.1:18443",
        Auth::UserPass("user".to_string(), "pass".to_string())
    )?;
    
    // Method 2: Cookie authentication (more secure)
    let client2 = Client::new(
        "http:127.0.0.1:18443",
        Auth::CookieFile(PathBuf::from("/path/to/.cookie"))
    )?;
    
    // Method 3: No authentication (only for local regtest)
    let client3 = Client::new(
        "http://127.0.0.1:18443",
        Auth::None
    )?;
    
    Ok(client1)
}