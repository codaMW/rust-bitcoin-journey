// TODO: Convert a satoshi amount to bitcoin (1 BTC = 100,000,000 sats)
// Use map to transform the amount

fn sats_to_btc(amount: Option<u64>) -> Option<f64> {
    // Your code here

    amount.map(|x| x as f64 / 100_000_000.0)
}

// Test with:
// Some(100_000_000) -> Some(1.0)
// Some(50_000_000) -> Some(0.5)
// None -> None

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn whole_btc() {
        let result = sats_to_btc(Some(100_000_000));
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn half_btc() {
        let result = sats_to_btc(Some(50_000_000));
        assert_eq!(result, Some(0.5));
    }
    #[test]
    fn no_btc() {
        let result = sats_to_btc(None);
        assert_eq!(result, None);
    }
}