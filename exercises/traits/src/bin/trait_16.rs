trait FeeEstimator {
    fn estimate_fee(&self, tx_size_bytes: usize) -> f64;
    fn priority(&self) -> &str;
}

struct LowFeeEstimator { sat_per_byte: f64 }
struct HighFeeEstimator { sat_per_byte: f64 }

impl FeeEstimator for LowFeeEstimator {
    fn estimate_fee(&self, tx_size_bytes: usize) -> f64 {
        self.sat_per_byte * tx_size_bytes as f64
    }
    fn priority(&self) -> &str { "low — confirms in ~24h" }
}

impl FeeEstimator for HighFeeEstimator {
    fn estimate_fee(&self, tx_size_bytes: usize) -> f64 {
        self.sat_per_byte * tx_size_bytes as f64
    }
    fn priority(&self) -> &str { "high — confirms in next block" }
}

fn print_fee_estimate(estimator: &dyn FeeEstimator, tx_size: usize) {
    let fee = estimator.estimate_fee(tx_size);
    println!(
        "Priority: {} | Size: {} bytes | Fee: {:.0} sats ({:.8} BTC)",
        estimator.priority(),
        tx_size,
        fee,
        fee / 100_000_000.0
    );
}

fn main() {
    let tx_size = 250; // typical P2PKH transaction
    let low = LowFeeEstimator { sat_per_byte: 5.0 };
    let high = HighFeeEstimator { sat_per_byte: 50.0 };

    print_fee_estimate(&low, tx_size);
    print_fee_estimate(&high, tx_size);
}