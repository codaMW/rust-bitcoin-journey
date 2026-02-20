fn is_high_fee(fee: u64) -> bool {
    fee > 10_000
}

fn main() {
    println!("5000 sats: {}", is_high_fee(5_000));
    println!("15000 sats: {}", is_high_fee(15_000));
}
