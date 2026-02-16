// YOUR JOB: Write this function
// Returns Some(balance) if wallet exists
// Returns None if wallet doesn't exist
// Wallet data:
//   wallet 1 → 1_000_000 sats
//   wallet 2 → 500_000 sats
//   wallet 3 → 0 sats        ← exists but empty, still Some(0)
//   anything else → None
fn get_wallet_balance(wallet_id: u32) -> Option<u64> {
    // your code here

    if wallet_id == 1 {Some(1000000)}
    else if wallet_id == 2 {Some(500000)}
    else if wallet_id == 3 {Some(0)}
    else {None}
}  

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // YOUR JOB: Write FOUR tests
    //
    // test 1: wallet 1 returns Some(1_000_000)
    // test 2: wallet 2 returns Some(500_000)
    // test 3: wallet 3 returns Some(0)   ← remember 0 ≠ None
    // test 4: wallet 99 returns None

    #[test]
    fn balance_is_a_million_sats() {
        let result = get_wallet_balance(1);
        assert_eq!(result, Some(1000000));
    }
    
    #[test]
    fn balance_is_five_hundred_thousand_sats() {
        let result = get_wallet_balance(2);
        assert_eq!(result, Some(500000));
    }

    #[test]
    fn balance_is_zero_sats() {
        let result = get_wallet_balance(3);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn wallet_does_not_exist() {
        let result = get_wallet_balance(99);
        assert_eq!(result, None);
    }
}