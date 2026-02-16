// Returns Some(fee) only if fee >= 1000 sats
// Returns None if tx missing OR fee too low
fn get_high_fee_tx(tx_id: u32) -> Option<u64> {
    // lookup table:
    // tx 1 → fee 2000  (passes)
    // tx 2 → fee 500   (too low)
    // tx 3 → fee 1000  (exactly 1000 — passes)
    // tx 4 → fee 999   (just under — fails)
    // tx 99 → None     (doesn't exist)

    // YOUR CODE HERE

    if tx_id == 1 {Some(2000)}
    else if tx_id == 2 {Some(500)}
    else if tx_id == 3 {Some(1000)}
    else if tx_id == 4 {Some(999)}
    else {None}

}

fn get_value(tx_id: u32) -> Option<u64> {

        match get_high_fee_tx(tx_id) {
            Some(val) => {
                if val >= 1000 {Some(val)}
                else{None}
            },
            None => None,
        }

}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // Write FIVE tests with descriptive names:
    //
    // 1. tx with fee 2000 returns Some(2000)
    // 2. tx with fee 500 returns None (too low)
    // 3. tx with fee exactly 1000 returns Some(1000) ← boundary
    // 4. tx with fee 999 returns None ← just under boundary
    // 5. missing tx returns None

    #[test]
    fn fee_return_passes() {
        let result = get_value(1);

        assert_eq!(result, Some(2000));
    }

    #[test]
    fn fee_too_low() {
        let result = get_value(2);

        assert_eq!(result, None);
    }
    #[test]
    fn fee_boundary() {
        let result = get_value(3);

        assert_eq!(result, Some(1000));
    }
    #[test]
    fn fee_just_under_baoundary() {
        let result = get_value(4);

        assert_eq!(result, None);
    }
    #[test]
    fn tx_not_available() {
        let result = get_value(5);

        assert_eq!(result, None);
    }
}