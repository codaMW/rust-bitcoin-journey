fn get_raw_tx(tx_id: u32) -> Option<u64> {
    if tx_id == 1 { Some(800) }
    else if tx_id == 2 { Some(300) }
    else if tx_id == 3 { Some(1500) }
    else if tx_id == 4 { Some(600) }
    else { None }
}

// Filter 1: fee must be >= 500
fn filter_min_fee(tx_id: u32) -> Option<u64> {
    // your code here

    match get_raw_tx(tx_id) {
        Some(val) => {
            if val >= 500 {Some(val)}
            else {None}
        },
        None => None
    }
}

// Filter 2: fee must be <= 1000
// Takes result of filter_min_fee as input
fn filter_max_fee(fee: Option<u64>) -> Option<u64> {
    // your code here
    // if fee is None → return None
    // if fee is Some(f) and f <= 1000 → return Some(f)
    // if fee is Some(f) and f > 1000  → return None
    match fee {
        Some(val) => {
            if val <= 1000 {Some(val)}
            else {None}
        }
        None => None
    }
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    // Test filter_min_fee independently — 4 tests
    // tx1=800 passes, tx2=300 fails, tx3=1500 passes, tx4=600 passes

    #[test]
    fn transaction_1_min_fee() {
        let result = filter_min_fee(1);
        assert_eq!(result, Some(800));
    }
    #[test]
    fn transaction_2_min_fee() {
        let result = filter_min_fee(2);
        assert_eq!(result, None);
    }

    #[test]
    fn transaction_3_min_fee() {
        let result = filter_min_fee(3);
        assert_eq!(result, Some(1500));
    }

    #[test]
    fn transaction_4_min_fee() {
        let result = filter_min_fee(4);
        assert_eq!(result, Some(600));
    }

    // Test filter_max_fee independently — 3 tests
    // Some(800) passes, Some(1500) fails, None stays None

    #[test]
    fn trans_1_max_fee() {
        let result = filter_max_fee(Some(800));
        assert_eq!(result, Some(800));
    }

    #[test]
    fn trans_2_max_fee() {
        let result = filter_max_fee(Some(1500));
        assert_eq!(result, None);
    }

    #[test]
    fn trans_3_max_fee() {
        let result: Option<u64> = filter_max_fee(None);
        assert_eq!(result, None);
    }

    // Test the full chain — 4 tests
    // chain: let result = filter_max_fee(filter_min_fee(id))
    // tx1=800  → passes both → Some(800)
    // tx2=300  → fails min   → None
    // tx3=1500 → fails max   → None
    // tx4=600  → passes both → Some(600)

    #[test]
    fn full_chain_test_1(){
        let result = filter_max_fee(filter_min_fee(1));
        assert_eq!(result, Some(800));
    }

    #[test]
    fn full_chain_test_2(){
        let result = filter_max_fee(filter_min_fee(2));
        assert_eq!(result, None);
    }

    #[test]
    fn full_chain_test_3(){
        let result = filter_max_fee(filter_min_fee(3));
        assert_eq!(result, None);
    }

    #[test]
    fn full_chain_test_4(){
        let result = filter_max_fee(filter_min_fee(4));
        assert_eq!(result, Some(600));
    }
}