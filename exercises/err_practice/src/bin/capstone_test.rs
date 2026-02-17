// Data: Transaction confirmations
fn get_tx_confirmations(tx_id: u32) -> Option<u32> {
    if tx_id == 1 { Some(12) }
    else if tx_id == 2 { Some(6) }
    else if tx_id == 3 { Some(5) }
    else if tx_id == 4 { Some(1) }
    else { None }
}

// Filter 1: tx must have >= 6 confirmations
fn filter_confirmed(tx_id: u32) -> Option<u32> {
    // YOUR CODE HERE
    match get_tx_confirmations(tx_id) {
        Some(val) => {
            if val >= 6 {
                return Some(val);
            } else {
                return None;
            }
        },

        None => None,
    }
}

// Filter 2: Check if tx is "deeply confirmed" (>= 10 confirmations)
// Returns Option<Option<u32>>:
//   None             → tx doesn't pass filter_confirmed
//   Some(None)       → tx confirmed but not deeply
//   Some(Some(conf)) → tx deeply confirmed
fn check_deep_confirmation(tx_id: u32) -> Option<Option<u32>> {
    // YOUR CODE HERE
    // First get filter_confirmed result
    // If None → return None
    // If Some(conf) and conf >= 10 → Some(Some(conf))
    // If Some(conf) and conf < 10  → Some(None)

    match filter_confirmed(tx_id) {
        Some(conf) => {
            if conf >= 10 {
                return Some(Some(conf));
            } else {
                return Some(None);
            }
        }
        None => None
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // Write 10 tests total:
    // 
    // filter_confirmed tests (4):
    //   tx1 (12 conf) passes
    //   tx2 (6 conf) passes
    //   tx3 (5 conf) fails
    //   tx99 missing fails

    #[test]
    fn filter_deep_conf(){
        let result = filter_confirmed(1);

        assert_eq!(result, Some(12));
    }

    #[test]
    fn at_bound(){
        let result = filter_confirmed(2);

        assert_eq!(result, Some(6));
    }

    #[test]
    fn below_conf(){
        let result = filter_confirmed(3);

        assert_eq!(result, None);
    }

    #[test]
    fn missing(){
        let result = filter_confirmed(99);

        assert_eq!(result, None);
    }
    // check_deep_confirmation tests (6):
    //   tx1 (12 conf) → Some(Some(12))
    //   tx2 (6 conf)  → Some(None)
    //   tx3 (5 conf)  → None
    //   tx4 (1 conf)  → None
    //   tx99 missing  → None
    //   use is_some() on tx1 outer layer

    #[test]
    fn nested_deep_conf() {
        let result = check_deep_confirmation(1);
        assert_eq!(result, Some(Some(12)));
    }

    #[test]
    fn nested_deep_conf_none() {
        let result = check_deep_confirmation(2);
        assert_eq!(result, Some(None));
    }
    #[test]
    fn nested_deep_below() {
        let result = check_deep_confirmation(3);
        assert_eq!(result, None);
    }
    #[test]
    fn nested_deep_conf_below_none() {
        let result = check_deep_confirmation(4);
        assert_eq!(result, None);
    }

    #[test]
    fn nested_deep_missing() {
        let result = check_deep_confirmation(99);
        assert_eq!(result, None);
    }

    #[test]
    fn is_outer_layer() {
        let result = check_deep_confirmation(1);
        assert!(result.is_some());
    }


    

}