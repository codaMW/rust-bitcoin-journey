# Study Notes: Bitcoin RPC Crate
## Mastering Bitcoin Core RPC with Rust

---

## **Table of Contents**
1. [Foundation Concepts](#foundation)
2. [Setting Up Bitcoin Core](#setup)
3. [Understanding RPC Basics](#rpc-basics)
4. [The BitcoinRPC Client](#client)
5. [Wallet Operations](#wallet)
6. [Advanced RPC Calls](#advanced)
7. [Error Handling & Best Practices](#errors)

---

## **1. Foundation Concepts** {#foundation}

### **1.1 Mental Model: Bitcoin Core as a Remote Server**

```
┌─────────────────┐      RPC Calls      ┌─────────────────┐
│   Your Rust     │  ─────────────────►  │   Bitcoin Core  │
│   Application   │  ◄─────────────────  │     Daemon      │
│                 │      JSON Responses  │                 │
└─────────────────┘                       └─────────────────┘
         │                                        │
    ┌────┴────------                         ┌────┴────--┐
    │bitcoincore-rpc│                        │bitcoind   │
    │   crate       │                        │bitcoin-cli│
    └───────────────┘                        └───────────┘
```

### **1.2 The RPC Architecture**

Think of Bitcoin Core as a web server that exposes API endpoints:
- **bitcoind**: The Bitcoin daemon (server)
- **RPC Interface**: HTTP endpoints that accept JSON commands
- **bitcoincore-rpc**: Rust crate that wraps these calls in type-safe functions

### **1.3 Key Components We'll Learn**

```rust
// The main types we'll work with:
- Client      // The RPC connection handle
- JsonRpc     // Low-level JSON-RPC handling
- Auth        // Authentication methods
- Error       // Error types
- types::*    // Bitcoin data types (Txid, Address, etc.)
```

---

## **2. Setting Up Bitcoin Core** {#setup}

### **2.1 Installation & Configuration**

```bash
# Install Bitcoin Core (macOS with Homebrew)
brew install bitcoin

# Create data directory
mkdir ~/bitcoin-data

# Start Bitcoin Core in regtest mode (local testnet)
bitcoind -regtest -daemon \
  -rpcuser=yourusername \
  -rpcpassword=yourpassword \
  -rpcport=18443 \
  -datadir=~/bitcoin-data
```

### **2.2 Understanding Regtest Mode**

```
                    MAINNET
┌─────────────────────────────────────┐
│ Real money, real miners, real world │
└─────────────────────────────────────┘
                    │
                    ▼
                    TESTNET
┌─────────────────────────────────────┐
│   Test money, real mining difficulty│
└─────────────────────────────────────┘
                    │
                    ▼
                    REGTEST
┌─────────────────────────────────────┐
│   Complete control, instant blocks  │
│   No other nodes, perfect for dev   │
└─────────────────────────────────────┘
```

**Why Regtest?**
- You control the network
- Blocks are instant (use `generatetoaddress`)
- No synchronization needed
- Perfect for learning

### **2.3 Project Setup**

```bash
# Create new Rust project
cargo new bitcoin-rpc-learn
cd bitcoin-rpc-learn

# Add dependencies to Cargo.toml
```

```toml
[package]
name = "bitcoin-rpc-learn"
version = "0.1.0"
edition = "2021"

[dependencies]
bitcoincore-rpc = "0.16.0"
bitcoin = "0.30.0"  # Bitcoin data types
serde_json = "1.0"  # For JSON handling
anyhow = "1.0"      # For easy error handling
```

---

## **3. Understanding RPC Basics** {#rpc-basics}

### **3.1 Mental Model: JSON-RPC Protocol**

```
Request:
┌─────────────────────────────────────┐
│ {                                   │
│   "jsonrpc": "2.0",                 │
│   "method": "getblockchaininfo",    │
│   "params": [],                     │
│   "id": 1                           │
│ }                                   │
└─────────────────────────────────────┘
                    │
                    ▼
Response:
┌─────────────────────────────────────┐
│ {                                   │
│   "result": {                       │
│     "chain": "regtest",             │
│     "blocks": 0,                    │
│     ...                             │
│   },                                │
│   "error": null,                    │
│   "id": 1                           │
│ }                                   │
└─────────────────────────────────────┘
```

### **3.2 Exercise 1: Your First RPC Call**

```rust
use bitcoincore_rpc::{Auth, Client, RpcApi};
use anyhow::Result;

fn main() -> Result<()> {
    // Step 1: Create connection URL
    let rpc_url = "http://localhost:18443".to_string();
    
    // Step 2: Set up authentication
    let rpc_auth = Auth::UserPass(
        "yourusername".to_string(),
        "yourpassword".to_string()
    );
    
    // Step 3: Create client
    let client = Client::new(&rpc_url, rpc_auth)
        .expect("Failed to create client");
    
    // Step 4: Make first RPC call
    let blockchain_info = client.get_blockchain_info()?;
    
    println!("Bitcoin Core is running!");
    println!("Chain: {}", blockchain_info.chain);
    println!("Blocks: {}", blockchain_info.blocks);
    println!("Headers: {}", blockchain_info.headers);
    
    Ok(())
}
```

### **3.3 Understanding the `RpcApi` Trait**

```rust
// The RpcApi trait defines ALL available RPC methods
pub trait RpcApi {
    fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResult>;
    fn get_block_count(&self) -> Result<u64>;
    fn get_new_address(&self, ...) -> Result<Address>;
    // ... hundreds more methods
}

// Your client implements this trait
impl RpcApi for Client {
    // ... all methods implemented
}
```

### **3.4 Exercise 2: Explore Different RPCs**

```rust
use bitcoincore_rpc::{Auth, Client, RpcApi};
use anyhow::Result;

fn explore_bitcoin_rpcs() -> Result<()> {
    let client = Client::new(
        "http://localhost:18443",
        Auth::UserPass("user".to_string(), "pass".to_string())
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
```

---

## **4. The BitcoinRPC Client** {#client}

### **4.1 Mental Model: Client Architecture**

```
┌─────────────────────────────────────┐
│         Your Application             │
├─────────────────────────────────────┤
│              Client                   │
│  ┌─────────────────────────────┐    │
│  │      Connection Pool        │    │
│  ├─────────────────────────────┤    │
│  │      HTTP Transport         │    │
│  ├─────────────────────────────┤    │
│  │      JSON Serializer        │    │
│  └─────────────────────────────┘    │
│              │                       │
│              ▼                       │
│         ┌──────────┐                 │
│         │  Result  │                 │
│         │  Types   │                 │
│         └──────────┘                 │
└─────────────────────────────────────┘
```

### **4.2 Creating and Configuring the Client**

```rust
use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::time::Duration;

fn create_client() -> Result<Client> {
    // Method 1: Basic authentication
    let client1 = Client::new(
        "http://localhost:18443",
        Auth::UserPass("user".to_string(), "pass".to_string())
    )?;
    
    // Method 2: Cookie authentication (more secure)
    let client2 = Client::new(
        "http://localhost:18443",
        Auth::CookieFile(PathBuf::from("/path/to/.cookie"))
    )?;
    
    // Method 3: No authentication (only for local regtest)
    let client3 = Client::new(
        "http://localhost:18443",
        Auth::None
    )?;
    
    Ok(client1)
}
```

### **4.3 Understanding Return Types**

```rust
use bitcoincore_rpc::jsonrpc::error::RpcError;
use bitcoincore_rpc::Error as BitcoinRpcError;

// The Result type combines multiple error sources
type Result<T> = std::result::Result<T, BitcoinRpcError>;

// BitcoinRpcError can be:
// - RpcError (JSON-RPC errors)
// - Json(serde_json::Error)
// - Hex(hex::FromHexError)
// - ... and more
```

### **4.4 Exercise 3: Build a Client Wrapper**

```rust
use bitcoincore_rpc::{Auth, Client, RpcApi};
use anyhow::Result;
use std::time::Duration;

struct BitcoinNode {
    client: Client,
    name: String,
}

impl BitcoinNode {
    fn new(url: &str, user: &str, pass: &str, name: &str) -> Result<Self> {
        let client = Client::new(
            url,
            Auth::UserPass(user.to_string(), pass.to_string())
        )?;
        
        Ok(BitcoinNode {
            client,
            name: name.to_string(),
        })
    }
    
    fn get_info(&self) -> Result<()> {
        println!("\n=== Node: {} ===", self.name);
        
        let info = self.client.get_blockchain_info()?;
        println!("Chain: {}", info.chain);
        println!("Blocks: {}", info.blocks);
        
        let balance = self.client.get_balance(None, None)?;
        println!("Balance: {} BTC", balance);
        
        Ok(())
    }
    
    fn is_ready(&self) -> bool {
        self.client.get_blockchain_info().is_ok()
    }
}

fn main() -> Result<()> {
    let node = BitcoinNode::new(
        "http://localhost:18443",
        "user",
        "pass",
        "Regtest Node"
    )?;
    
    if node.is_ready() {
        node.get_info()?;
    }
    
    Ok(())
}
```

---

## **5. Wallet Operations** {#wallet}

### **5.1 Mental Model: Bitcoin Wallet**

```
┌─────────────────────────────────────┐
│          Bitcoin Wallet              │
├─────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  │
│  │  Address 1  │  │  Address 2  │  │
│  │  Balance    │  │  Balance    │  │
│  └─────────────┘  └─────────────┘  │
│  ┌─────────────┐  ┌─────────────┐  │
│  │  Address 3  │  │  Address 4  │  │
│  │  Balance    │  │  Balance    │  │
│  └─────────────┘  └─────────────┘  │
├─────────────────────────────────────┤
│       Total Balance: 50 BTC         │
└─────────────────────────────────────┘
```

### **5.2 Creating and Loading Wallets**

```rust
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::json::{CreateWalletOptions, WalletCreateFundedPsbtOptions};
use bitcoin::Address;
use std::str::FromStr;

fn wallet_operations() -> Result<()> {
    let client = Client::new(
        "http://localhost:18443",
        Auth::UserPass("user".to_string(), "pass".to_string())
    )?;
    
    // Step 1: Create a new wallet
    let options = CreateWalletOptions {
        passphrase: None,
        avoid_reuse: Some(false),
        descriptors: Some(true),
        load_on_startup: Some(true),
    };
    
    let result = client.create_wallet("mywallet", Some(options))?;
    println!("Wallet created: {}", result.name);
    
    // Step 2: Load the wallet (if not auto-loaded)
    let load_result = client.load_wallet("mywallet")?;
    println!("Wallet loaded: {}", load_result.warning);
    
    // Step 3: Get wallet info
    let wallet_info = client.get_wallet_info()?;
    println!("Wallet name: {}", wallet_info.walletname);
    println!("Format: {}", wallet_info.format);
    
    Ok(())
}
```

### **5.3 Working with Addresses**

```rust
fn address_operations(client: &Client) -> Result<()> {
    // Generate new address
    let address = client.get_new_address(None, None)?;
    println!("New address: {}", address);
    
    // Generate address with label
    let labeled_address = client.get_new_address(
        Some("savings"),
        Some(bitcoincore_rpc::json::AddressType::Bech32)
    )?;
    println!("Labeled address: {}", labeled_address);
    
    // Get address info
    let address_info = client.get_address_info(&labeled_address)?;
    println!("Is mine: {:?}", address_info.ismine);
    println!("Label: {:?}", address_info.labels);
    
    // Validate address
    let validation = client.validate_address(&address)?;
    println!("Valid: {}", validation.isvalid);
    println!("Address type: {:?}", validation.script_type);
    
    Ok(())
}
```

### **5.4 The Transaction Lifecycle**

```
                      TRANSACTION LIFECYCLE
                          
        Create                         Sign
    ┌─────────┐                    ┌─────────┐
    │  Create │                    │  Sign   │
    │ Raw TX  │                    │  TX     │
    └─────────┘                    └─────────┘
         │                              │
         ▼                              ▼
    ┌─────────┐     Send          ┌─────────┐
    │ Fund    │    ──────────►    │ Send    │
    │ PSBT    │                    │ TX      │
    └─────────┘                    └─────────┘
         │                              │
         ▼                              ▼
    ┌─────────┐    Confirm         ┌─────────┐
    │ Decode  │    ◄──────────     │ Get     │
    │ PSBT    │                    │ TX      │
    └─────────┘                    └─────────┘
```

### **5.5 Exercise 4: Complete Wallet Example**

```rust
use bitcoincore_rpc::{Auth, Client, RpcApi};
use anyhow::Result;
use bitcoin::Amount;
use std::thread;
use std::time::Duration;

struct Wallet {
    client: Client,
    name: String,
}

impl Wallet {
    fn new(client: Client, name: &str) -> Self {
        Wallet {
            client,
            name: name.to_string(),
        }
    }
    
    fn get_balance(&self) -> Result<Amount> {
        let balance = self.client.get_balance(None, None)?;
        Ok(balance)
    }
    
    fn get_new_address(&self, label: &str) -> Result<bitcoin::Address> {
        let address = self.client.get_new_address(
            Some(label),
            Some(bitcoincore_rpc::json::AddressType::Bech32)
        )?;
        Ok(address)
    }
    
    fn send_to_address(
        &self,
        address: &bitcoin::Address,
        amount: Amount,
    ) -> Result<bitcoin::Txid> {
        let txid = self.client.send_to_address(
            address,
            amount,
            None,
            None,
            None,
            None,
            None,
            None,
        )?;
        Ok(txid)
    }
    
    fn list_transactions(&self, count: usize) -> Result<()> {
        let transactions = self.client.list_transactions(
            None,
            Some(count),
            Some(0),
            None,
        )?;
        
        for (i, tx) in transactions.iter().enumerate() {
            println!("\nTransaction {}:", i + 1);
            println!("  TxID: {}", tx.txid);
            println!("  Amount: {} BTC", tx.amount);
            println!("  Confirmations: {}", tx.confirmations);
            println!("  Time: {}", tx.time);
        }
        
        Ok(())
    }
}

fn main() -> Result<()> {
    // Setup client
    let client = Client::new(
        "http://localhost:18443",
        Auth::UserPass("user".to_string(), "pass".to_string())
    )?;
    
    // Create wallet
    println!("Creating wallet...");
    let _ = client.create_wallet("mywallet", None)?;
    
    // Switch to wallet context
    let wallet_client = Client::new(
        "http://localhost:18443/wallet/mywallet",
        Auth::UserPass("user".to_string(), "pass".to_string())
    )?;
    
    let wallet = Wallet::new(wallet_client, "mywallet");
    
    // Generate some blocks to get funds (in regtest)
    let mining_client = Client::new(
        "http://localhost:18443",
        Auth::UserPass("user".to_string(), "pass".to_string())
    )?;
    
    // Get an address to mine to
    let mining_address = wallet.get_new_address("mining")?;
    
    // Mine 101 blocks (coinbase mature after 100 blocks)
    let block_hashes = mining_client.generate_to_address(101, &mining_address)?;
    println!("Mined {} blocks", block_hashes.len());
    
    // Check balance
    let balance = wallet.get_balance()?;
    println!("Wallet balance: {} BTC", balance);
    
    // Create a second address to send to
    let recipient_address = wallet.get_new_address("recipient")?;
    println!("Recipient address: {}", recipient_address);
    
    // Send some coins
    let send_amount = Amount::from_btc(10.0)?;
    let txid = wallet.send_to_address(&recipient_address, send_amount)?;
    println!("Sent transaction: {}", txid);
    
    // Mine a block to confirm
    mining_client.generate_to_address(1, &mining_address)?;
    println!("Mined confirmation block");
    
    // List transactions
    println!("\nTransaction history:");
    wallet.list_transactions(10)?;
    
    Ok(())
}
```

---

## **6. Advanced RPC Calls** {#advanced}

### **6.1 Working with Raw Transactions**

```rust
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::json::*;
use bitcoin::consensus::encode::serialize_hex;

fn raw_transaction_example() -> Result<()> {
    let client = Client::new(
        "http://localhost:18443/wallet/mywallet",
        Auth::UserPass("user".to_string(), "pass".to_string())
    )?;
    
    // Get unspent outputs
    let utxos = client.list_unspent(None, None, None, None, None)?;
    
    for utxo in utxos {
        println!("UTXO:");
        println!("  TxID: {}", utxo.txid);
        println!("  Vout: {}", utxo.vout);
        println!("  Amount: {} BTC", utxo.amount);
        println!("  Address: {:?}", utxo.address);
    }
    
    // Create raw transaction
    let outputs = std::collections::HashMap::from([
        (
            "bcrt1q...".to_string(),  // recipient address
            1.0  // amount in BTC
        )
    ]);
    
    let raw_tx = client.create_raw_transaction(
        &[],  // inputs (empty for now)
        &outputs,
        None,  // locktime
        None,  // replaceable
    )?;
    
    println!("Raw transaction hex: {}", raw_tx);
    
    // Fund the transaction
    let options = FundRawTransactionOptions {
        change_address: None,
        change_position: None,
        include_watching: None,
        lock_unspents: None,
        fee_rate: None,
        subtract_fee_from_outputs: None,
        replaceable: None,
        confirmation_target: None,
        estimate_mode: None,
    };
    
    let funded = client.fund_raw_transaction(&raw_tx, Some(options), None)?;
    println!("Funded transaction hex: {}", funded.hex);
    
    // Sign the transaction
    let signed = client.sign_raw_transaction_with_wallet(
        &funded.hex,
        None,
        None,
        None,
    )?;
    
    if signed.complete {
        println!("Transaction signed successfully");
        
        // Send the transaction
        let txid = client.send_raw_transaction(&signed.hex, None)?;
        println!("Transaction sent: {}", txid);
    }
    
    Ok(())
}
```

### **6.2 Mental Model: UTXO Set**

```
                    UTXO SET
┌─────────────────────────────────────────┐
│  Transaction A: Output 0 (Unspent)      │
│  └─► Address: bc1q...                   │
│  └─► Amount: 5 BTC                       │
├─────────────────────────────────────────┤
│  Transaction B: Output 1 (Unspent)      │
│  └─► Address: bc1p...                   │
│  └─► Amount: 3 BTC                       │
├─────────────────────────────────────────┤
│  Transaction C: Output 2 (Unspent)      │
│  └─► Address: bc1q...                   │
│  └─► Amount: 2 BTC                       │
└─────────────────────────────────────────┘
                          │
           When spending, select UTXOs
                          ▼
┌─────────────────────────────────────────┐
│  New Transaction                        │
│  Inputs:                                 │
│  └─► UTXO A (5 BTC)                      │
│  └─► UTXO B (3 BTC)                      │
│  Outputs:                                │
│  └─► Send 7 BTC to recipient             │
│  └─► Send 1 BTC back as change          │
└─────────────────────────────────────────┘
```

### **6.3 Exercise 5: Build a UTXO Manager**

```rust
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::json::ListUnspentResultEntry;
use anyhow::Result;
use bitcoin::{Address, Amount, Txid};
use std::collections::HashMap;

struct UtxoManager {
    client: Client,
    utxos: Vec<ListUnspentResultEntry>,
}

impl UtxoManager {
    fn new(client: Client) -> Self {
        UtxoManager {
            client,
            utxos: Vec::new(),
        }
    }
    
    fn refresh_utxos(&mut self) -> Result<()> {
        self.utxos = self.client.list_unspent(
            Some(1),  // min confirmations
            None,     // max confirmations
            None,     // addresses
            None,     // include_unsafe
            None,     // query_options
        )?;
        Ok(())
    }
    
    fn total_balance(&self) -> Amount {
        self.utxos
            .iter()
            .fold(Amount::ZERO, |acc, utxo| acc + utxo.amount)
    }
    
    fn select_utxos(&self, target: Amount) -> Option<Vec<ListUnspentResultEntry>> {
        let mut selected = Vec::new();
        let mut total = Amount::ZERO;
        
        // Simple greedy algorithm
        for utxo in &self.utxos {
            if total < target {
                selected.push(utxo.clone());
                total += utxo.amount;
            } else {
                break;
            }
        }
        
        if total >= target {
            Some(selected)
        } else {
            None
        }
    }
    
    fn create_transaction(
        &mut self,
        to_address: &Address,
        amount: Amount,
    ) -> Result<Option<Txid>> {
        self.refresh_utxos()?;
        
        let selected_utxos = match self.select_utxos(amount) {
            Some(utxos) => utxos,
            None => return Ok(None),
        };
        
        // Create raw transaction
        let mut outputs = HashMap::new();
        outputs.insert(to_address.to_string(), amount.to_btc());
        
        // Calculate change
        let total_input = selected_utxos
            .iter()
            .fold(Amount::ZERO, |acc, u| acc + u.amount);
        
        let change = total_input - amount;
        
        if change > Amount::from_sat(1000) {  // dust threshold
            let change_address = self.client.get_raw_change_address(None)?;
            outputs.insert(change_address.to_string(), change.to_btc());
        }
        
        // Build inputs
        let inputs: Vec<_> = selected_utxos
            .iter()
            .map(|u| CreateRawTransactionInput {
                txid: u.txid,
                vout: u.vout,
                sequence: None,
            })
            .collect();
        
        let raw_tx = self.client.create_raw_transaction(
            &inputs,
            &outputs,
            None,
            None,
        )?;
        
        // Fund and sign
        let funded = self.client.fund_raw_transaction(&raw_tx, None, None)?;
        let signed = self.client.sign_raw_transaction_with_wallet(
            &funded.hex,
            None,
            None,
            None,
        )?;
        
        if signed.complete {
            let txid = self.client.send_raw_transaction(&signed.hex, None)?;
            Ok(Some(txid))
        } else {
            Ok(None)
        }
    }
    
    fn analyze_utxos(&self) {
        println!("\n=== UTXO Analysis ===");
        println!("Total UTXOs: {}", self.utxos.len());
        println!("Total Balance: {} BTC", self.total_balance());
        
        // Group by address
        let mut address_map: HashMap<String, Vec<&ListUnspentResultEntry>> = HashMap::new();
        for utxo in &self.utxos {
            if let Some(ref addr) = utxo.address {
                address_map
                    .entry(addr.to_string())
                    .or_insert_with(Vec::new)
                    .push(utxo);
            }
        }
        
        for (addr, utxos) in address_map {
            println!("\nAddress: {}", addr);
            println!("  UTXOs: {}", utxos.len());
            let total: Amount = utxos.iter().map(|u| u.amount).sum();
            println!("  Total: {} BTC", total);
        }
    }
}

fn main() -> Result<()> {
    let client = Client::new(
        "http://localhost:18443/wallet/mywallet",
        Auth::UserPass("user".to_string(), "pass".to_string())
    )?;
    
    let mut manager = UtxoManager::new(client);
    manager.refresh_utxos()?;
    manager.analyze_utxos();
    
    Ok(())
}
```

---

## **7. Error Handling & Best Practices** {#errors}

### **7.1 Mental Model: Error Hierarchy**

```
                    anyhow::Error
                          │
                    ┌─────┴─────┐
                    │           │
            BitcoinRpcError   Other Errors
                    │
        ┌───────────┼───────────┐
        │           │           │
    RpcError   JsonError    HexError
        │
    ┌───┴───┐
    │       │
   JSON-RPC Transport
   Errors   Errors
```

### **7.2 Comprehensive Error Handling**

```rust
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::Error as BitcoinRpcError;
use bitcoincore_rpc::jsonrpc::error::RpcError;
use thiserror::Error;

#[derive(Error, Debug)]
enum WalletError {
    #[error("Bitcoin RPC error: {0}")]
    Rpc(#[from] BitcoinRpcError),
    
    #[error("Insufficient funds: have {have}, need {need}")]
    InsufficientFunds {
        have: Amount,
        need: Amount,
    },
    
    #[error("Wallet not found: {0}")]
    WalletNotFound(String),
    
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
}

struct RobustWallet {
    client: Client,
    name: String,
    retry_count: usize,
}

impl RobustWallet {
    fn new(url: &str, auth: Auth, name: &str, retries: usize) -> Result<Self, WalletError> {
        let client = Client::new(url, auth)
            .map_err(|e| WalletError::Rpc(e))?;
        
        Ok(RobustWallet {
            client,
            name: name.to_string(),
            retry_count: retries,
        })
    }
    
    fn with_retry<F, T>(&self, operation: F) -> Result<T, WalletError>
    where
        F: Fn(&Client) -> Result<T, BitcoinRpcError>,
    {
        let mut last_error = None;
        
        for attempt in 1..=self.retry_count {
            match operation(&self.client) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    println!("Attempt {} failed: {}", attempt, e);
                    last_error = Some(e);
                    
                    if attempt < self.retry_count {
                        // Exponential backoff
                        std::thread::sleep(std::time::Duration::from_millis(
                            100 * 2u64.pow(attempt as u32)
                        ));
                    }
                }
            }
        }
        
        Err(WalletError::Rpc(last_error.unwrap()))
    }
    
    fn send_payment(
        &self,
        to_address: &str,
        amount_btc: f64,
    ) -> Result<bitcoin::Txid, WalletError> {
        // Validate address
        let address = match bitcoin::Address::from_str(to_address) {
            Ok(addr) => addr,
            Err(_) => return Err(WalletError::InvalidAddress(to_address.to_string())),
        };
        
        // Check balance
        let balance = self.with_retry(|c| c.get_balance(None, None))?;
        let amount = Amount::from_btc(amount_btc)
            .map_err(|_| WalletError::TransactionFailed("Invalid amount".to_string()))?;
        
        if balance < amount {
            return Err(WalletError::InsufficientFunds {
                have: balance,
                need: amount,
            });
        }
        
        // Send transaction with retry
        let txid = self.with_retry(|c| {
            c.send_to_address(
                &address,
                amount,
                None,
                None,
                None,
                None,
                None,
                None,
            )
        })?;
        
        Ok(txid)
    }
    
    fn ensure_wallet_loaded(&self) -> Result<(), WalletError> {
        match self.with_retry(|c| c.get_wallet_info()) {
            Ok(_) => Ok(()),
            Err(BitcoinRpcError::Rpc(RpcError::Rpc(rpc_error))) 
                if rpc_error.code == -18 => {  // Wallet not loaded
                // Try to load wallet
                self.with_retry(|c| {
                    c.load_wallet(&self.name)
                })?;
                Ok(())
            }
            Err(e) => Err(WalletError::Rpc(e)),
        }
    }
}

fn main() -> Result<(), WalletError> {
    let auth = Auth::UserPass("user".to_string(), "pass".to_string());
    
    let wallet = RobustWallet::new(
        "http://localhost:18443/wallet/mywallet",
        auth,
        "mywallet",
        3,  // retry 3 times
    )?;
    
    // Ensure wallet is loaded
    wallet.ensure_wallet_loaded()?;
    
    // Try to send payment
    match wallet.send_payment("bcrt1q...", 0.1) {
        Ok(txid) => println!("Payment sent: {}", txid),
        Err(WalletError::InsufficientFunds { have, need }) => {
            println!("Insufficient funds! Have {} BTC, need {} BTC", have, need);
        }
        Err(WalletError::InvalidAddress(addr)) => {
            println!("Invalid address: {}", addr);
        }
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}
```

### **7.3 Best Practices Checklist**

```rust
/// ✅ DO: Use type-safe Bitcoin amounts
fn good_example() {
    let amount = Amount::from_btc(1.5).unwrap();
    let txid = client.send_to_address(&addr, amount, ...);
}

/// ❌ DON'T: Use raw floats
fn bad_example() {
    let amount = 1.5;  // Never use f64 for monetary values!
    // Floating point errors will accumulate!
}

/// ✅ DO: Handle wallet context properly
fn good_wallet_handling() {
    // Use wallet-specific URL
    let wallet_url = "http://localhost:18443/wallet/mywallet";
    let client = Client::new(wallet_url, auth)?;
    
    // Or switch context
    let client = Client::new("http://localhost:18443", auth)?;
    let _ = client.load_wallet("mywallet")?;
}

/// ✅ DO: Implement retry logic for network issues
fn with_retry<F, T>(f: F) -> Result<T>
where
    F: Fn() -> Result<T>,
{
    let max_retries = 3;
    for i in 0..max_retries {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) if i < max_retries - 1 => {
                eprintln!("Attempt {} failed: {}. Retrying...", i + 1, e);
                std::thread::sleep(Duration::from_secs(2u64.pow(i)));
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}

/// ✅ DO: Validate inputs before RPC calls
fn validate_address(address: &str) -> Result<bitcoin::Address> {
    bitcoin::Address::from_str(address)
        .map_err(|_| anyhow::anyhow!("Invalid bitcoin address"))
}

/// ✅ DO: Use proper error types
#[derive(Debug, Error)]
enum AppError {
    #[error("RPC error: {0}")]
    Rpc(#[from] bitcoincore_rpc::Error),
    
    #[error("Invalid configuration: {0}")]
    Config(String),
    
    #[error("Transaction error: {0}")]
    Transaction(String),
}
```

---

## **Final Exercise: Build a Complete Bitcoin Wallet CLI**

```rust
use bitcoincore_rpc::{Auth, Client, RpcApi};
use anyhow::Result;
use std::io::{self, Write};
use bitcoin::{Address, Amount, Txid};
use std::str::FromStr