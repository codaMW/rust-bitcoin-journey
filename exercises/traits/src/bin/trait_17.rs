use std::collections::HashMap;
use std::fmt;

trait WalletQuery {
    fn label(&self) -> &str;
    fn lookup_utxo(&self, txid: &str) -> Option<u64>;

    // Default implementation
    fn balance(&self, txids: &[&str]) -> u64 {
        txids
            .iter()
            .filter_map(|txid| self.lookup_utxo(txid))
            .sum()
    }
}

struct HotWallet {
    label: String,
    utxos: HashMap<String, u64>,
}

impl HotWallet {
    fn new(label: impl Into<String>) -> Self {
        let mut utxos = HashMap::new();

        utxos.insert("abc123".into(), 5_000_000_000);
        utxos.insert("def456".into(), 1_000_000_000);
        utxos.insert("ghi789".into(), 546);

        Self {
            label: label.into(),
            utxos,
        }
    }
}

impl WalletQuery for HotWallet {
    fn label(&self) -> &str {
        &self.label
    }

    fn lookup_utxo(&self, txid: &str) -> Option<u64> {
        self.utxos.get(txid).copied()
    }
}

impl fmt::Display for HotWallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HotWallet[{}]", self.label)
    }
}

fn main() {
    let wallet = HotWallet::new("mining-wallet");

    // Test lookup_utxo with match
    for txid in ["abc123", "def456", "xyz999"] {
        match wallet.lookup_utxo(txid) {
            Some(value) => println!("UTXO {}: {} sats", txid, value),
            None => println!("UTXO {}: not found", txid),
        }
    }

    // Test balance
    let total = wallet.balance(&["abc123", "def456", "ghi789"]);
    println!("Total balance: {} sats", total);

    // Display
    println!("{}", wallet);
}