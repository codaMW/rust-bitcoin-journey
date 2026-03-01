use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde::Deserialize;
use serde_json::json;
use std::fmt;

// ─────────────────────────────────────────────────
// Configuration
// ─────────────────────────────────────────────────
const RPC_URL:  &str = "http://127.0.0.1:18443";
const RPC_USER: &str = "alice";
const RPC_PASS: &str = "password";
const WALLET:   &str = "Yankho Ngolleka";

// ─────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────

// Represents a snapshot of our wallet state
struct WalletSnapshot {
    label:        String,
    address:      String,
    balance_btc:  f64,
    block_height: u64,
    utxo_count:   usize,
}

// Display trait — clean human readable output
impl fmt::Display for WalletSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "╔══════════════════════════════════════╗\n\
             ║         WALLET SNAPSHOT              ║\n\
             ╠══════════════════════════════════════╣\n\
             ║ Label:    {:<27} ║\n\
             ║ Address:  {:<27} ║\n\
             ║ Balance:  {:<24} BTC ║\n\
             ║ Height:   {:<27} ║\n\
             ║ UTXOs:    {:<27} ║\n\
             ╚══════════════════════════════════════╝",
            self.label,
            &self.address[..27],
            self.balance_btc,
            self.block_height,
            self.utxo_count,
        )
    }
}

// ─────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────

fn auth() -> Auth {
    Auth::UserPass(RPC_USER.to_string(), RPC_PASS.to_string())
}

fn wallet_url() -> String {
    format!("{}/wallet/{}", RPC_URL, WALLET)
}

fn wallet_exists(rpc: &Client) -> bitcoincore_rpc::Result<bool> {
    #[derive(Deserialize)]
    struct WalletName { name: String }

    #[derive(Deserialize)]
    struct ListResult { wallets: Vec<WalletName> }

    let result: ListResult = rpc.call("listwalletdir", &[])?;
    Ok(result.wallets.iter().any(|w| w.name == WALLET))
}

// ─────────────────────────────────────────────────
// Main
// ─────────────────────────────────────────────────

fn main() -> bitcoincore_rpc::Result<()> {

    println!("\n🔗 Connecting to Bitcoin regtest node...");

    // Step 1: Connect
    let rpc = Client::new(RPC_URL, auth())?;
    let info = rpc.get_blockchain_info()?;
    println!("✓ Connected | network: {} | height: {}",
        info.chain, info.blocks);

    // Step 2: Create or load wallet
    println!("\n👛 Setting up wallet...");
    if wallet_exists(&rpc)? {
        println!("  Wallet '{}' found — loading", WALLET);
        let _ = rpc.call::<serde_json::Value>(
            "loadwallet",
            &[json!(WALLET)]
        );
    } else {
        println!("  Creating new wallet '{}'", WALLET);
        rpc.create_wallet(WALLET, None, None, None, None)?;
    }
    println!("✓ Wallet ready");

    // Step 3: Wallet-scoped client
    let wrpc = Client::new(&wallet_url(), auth())?;

    // Step 4: Generate address
    println!("\n📬 Generating address...");
    let address = wrpc
        .get_new_address(None, None)?
        .assume_checked();
    println!("✓ Address: {}", address);

    // Step 5: Mine 201 blocks
    println!("\n⛏️  Mining 201 blocks...");
    wrpc.generate_to_address(201, &address)?;
    println!("✓ Mining complete");

    // Step 6: Get updated chain info
    let info = rpc.get_blockchain_info()?;

    // Step 7: Get balance
    let balance = wrpc.get_balance(None, None)?;

    // Step 8: List UTXOs
    let utxos = wrpc.list_unspent(Some(1), None, None, None, None)?;

    println!("\n📊 UTXO Breakdown:");
    for utxo in &utxos {
        println!(
            "  {}...{} | {:.8} BTC | {} confs",
            &utxo.txid.to_string()[..8],
            &utxo.txid.to_string()[56..],
            utxo.amount.to_btc(),
            utxo.confirmations,
        );
    }

    // Step 9: Build and display snapshot
    let snapshot = WalletSnapshot {
        label:        WALLET.to_string(),
        address:      address.to_string(),
        balance_btc:  balance.to_btc(),
        block_height: info.blocks,
        utxo_count:   utxos.len(),
    };

    println!("\n{}", snapshot);

    Ok(())
}