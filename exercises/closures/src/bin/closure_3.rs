fn main() {
    let min_fee: u64 = 1000;
    let max_fee: u64 = 100_000;
    
    // TASK 1: This closure needs to capture BOTH min_fee and max_fee
    let is_valid_fee = |fee| {
        // your condition here using min_fee and max_fee
        fee >= min_fee && fee <= max_fee
    };
    
    // Test it
    println!("500: {}", is_valid_fee(500));      // should be false
    println!("1000: {}", is_valid_fee(1000));    // should be true
    println!("50000: {}", is_valid_fee(50_000)); // should be true
}