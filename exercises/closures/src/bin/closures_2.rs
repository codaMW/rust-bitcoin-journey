fn main() {
    let min_fee: u64 = 1000;       // network minimum
    let max_fee: u64 = 100_000;    // spam protection limit
    let network = "mainnet";       // current network

    // TASK 1: Create a closure called `is_valid_fee`
    // that captures BOTH min_fee and max_fee
    // returns true if fee is >= min_fee AND <= max_fee
    // Test with: 500 (too low), 1000 (exact min), 
    //            50_000 (valid), 100_000 (exact max), 200_000 (too high)

    let is_valid_fee = |fee| fee >= min_fee && fee <= max_fee;

    println!("{}", is_valid_fee(500));
    println!("{}", is_valid_fee(1000));
    println!("{}", is_valid_fee(50_000));
    println!("{}", is_valid_fee(100_000));
    println!("{}", is_valid_fee(200_000));

    // TASK 2: Create a closure called `announce_tx`
    // that captures `network`
    // takes one parameter: tx_id: u32
    // prints "Broadcasting tx {} to {}", tx_id, network
    // Call it with tx_id 42 and tx_id 99

    let announce_tx = |tx_id: i32| println!("Broadcasting tx {} to {}", tx_id, network);
    announce_tx(42);
    announce_tx(99);

    // TASK 3: Create a closure called `fee_with_network_multiplier`
    // captures network
    // takes one parameter: base_fee: u64
    // if network == "mainnet" → return base_fee * 2
    // if network == "testnet" → return base_fee
    // else                    → return 0
    // Test with base_fee 500

    let fee_with_network_multiplier = |base_fee: u64| if network == "mainnet" { return base_fee * 2} else if network == "testnet" { return base_fee} else { return 0 };
    println!("{}", fee_with_network_multiplier(500));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_capturing_min_fee() {
        let min_fee: u64 = 1000;
        let is_valid = |fee: u64| fee >= min_fee;
        assert_eq!(is_valid(500), false);
        assert_eq!(is_valid(1000), true);
        assert_eq!(is_valid(5000), true);
    }

    // YOUR JOB: Write one more test
    // Create a `network` variable set to "mainnet"
    // Create a closure that captures it
    // and returns true if network == "mainnet"
    // assert it returns true

    #[test]
    fn test_network() {
        let network = "mainnet";
        let net_cap = |net| net == "mainnet";

        assert_eq!(net_cap(network), true);
    }
}