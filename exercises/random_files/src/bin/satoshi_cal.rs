fn btc_to_sats(btc: f64) -> u64 {
    (btc * 100_000_000.0) as u64
}

fn main() {
    println!("1 BTC = {} sats", btc_to_sats(1.0));
    println!("0.5 BTC = {} sats", btc_to_sats(0.5));
}