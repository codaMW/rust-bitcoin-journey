use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde::Deserialize;
use serde_json::json;

const RPC_URL:  &str = "http://127.0.0.1:18443";
const RPC_USER: &str = "alice";
const RPC_PASS: &str = "password";
const WALLET:   &str = "testwallet";

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

fn main() -> bitcoincore_rpc::Result<()> {

    // ─────────────────────────────────────
    // Step 1: Connect to node
    // ─────────────────────────────────────
    let rpc = Client::new(RPC_URL, auth())?;

    let info = rpc.get_blockchain_info()?;
    println!("Connected to {} | height: {}", info.chain, info.blocks);

    // ─────────────────────────────────────
    // Step 2: Create or load wallet
    // ─────────────────────────────────────
    if wallet_exists(&rpc)? {
        println!("Wallet exists — loading...");
        let _ = rpc.call::<serde_json::Value>(
            "loadwallet",
            &[json!(WALLET)]
        );
    } else {
        println!("Creating wallet: {}", WALLET);
        rpc.create_wallet(WALLET, None, None, None, None)?;
    }

    // ─────────────────────────────────────
    // Step 3: Wallet-scoped client
    // ─────────────────────────────────────
    let wrpc = Client::new(&wallet_url(), auth())?;

    // ─────────────────────────────────────
    // Step 4: Generate address
    // ─────────────────────────────────────
    let address = wrpc
        .get_new_address(None, None)?
        .assume_checked();

    println!("New address: {}", address);

    // ─────────────────────────────────────
    // Step 5: Check balance
    // ─────────────────────────────────────
    let balance = wrpc.get_balance(None, None)?;
    println!("Balance: {} BTC", balance.to_btc());

    // ─────────────────────────────────────
    // Step 6: Mine 101 blocks
    // ─────────────────────────────────────
    println!("Mining 101 blocks...");
    wrpc.generate_to_address(101, &address)?;

    // ─────────────────────────────────────
    // Step 7: Check balance again
    // ─────────────────────────────────────
    let balance = wrpc.get_balance(None, None)?;
    println!("Balance after mining: {} BTC", balance.to_btc());

    // ─────────────────────────────────────
    // Step 8: List UTXOs
    // ─────────────────────────────────────
    let utxos = wrpc.list_unspent(Some(1), None, None, None, None)?;
    println!("Spendable UTXOs: {}", utxos.len());
    for utxo in &utxos {
        println!(
            "  txid: {}...{} | amount: {} BTC | confirmations: {}",
            &utxo.txid.to_string()[..8],
            &utxo.txid.to_string()[56..],
            utxo.amount.to_btc(),
            utxo.confirmations
        );
    }

    Ok(())
}