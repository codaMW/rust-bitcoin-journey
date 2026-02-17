fn get_reward(height: u32) -> u64 {
    if height < 210_000 { 50_000_000_000 }
    else if height < 420_000 { 25_000_000_000 }
    else { 12_500_000_000 }
}

fn main() {
    println!("Block 0 reward: {} sats", get_reward(0));
    println!("Block 300000 reward: {} sats", get_reward(300_000));
}