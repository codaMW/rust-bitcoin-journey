fn get_block_reward(height: u32) -> Option<u64> {
    if height == 1 { Some(5_000_000_000) }  // 50 BTC in sats
    else if height == 2 { Some(2_500_000_000) }
    else { None }
}

fn main() {
    // TASK 1: Get reward for block 1, use 0 as fallback
    // Use .unwrap_or() — one line
    let reward_1 = get_block_reward(1).unwrap_or(0);// your code here
    println!("Block 1 reward: {}", reward_1);

    // TASK 2: Get reward for block 99, use 0 as fallback
    let reward_99 = get_block_reward(99).unwrap_or(0);// your code here
    println!("Block 99 reward: {}", reward_99);

    // TASK 3: Get reward for block 2, use 1_000_000_000 as fallback
    let reward_2 = get_block_reward(2).unwrap_or(1_000_000_000); // your code here
    println!("Block 2 reward: {}", reward_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_1_returns_actual_reward() {
        let result = get_block_reward(1).unwrap_or(0);
        assert_eq!(result, 5_000_000_000);
    }

    #[test]
    fn test_missing_block_returns_fallback() {
        let result = get_block_reward(99).unwrap_or(0);
        assert_eq!(result, 0);
    }

    // YOUR JOB: Write one more test
    // Test that block 2 with fallback 999 returns the actual reward (not fallback)

    #[test]
    fn actual_reward() {
        let result = get_block_reward(2).unwrap_or(0);
        assert_eq!(result, 2_500_000_000);
    }
}