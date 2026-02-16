// This function is supposed to return Some(price) 
// only if the sat price is between 10 and 1000 inclusive
// But there might be a bug somewhere...
fn get_valid_sat_price(price: u64) -> Option<u64> {
    //Fixing the bug
    if price >= 10 && price <= 1000 {
        Some(price)
    } else {
        None
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // Write SIX tests that thoroughly check this function
    // including the exact boundary values
    // At least one test should FAIL — revealing the bug
    //
    // Boundaries to test:
    //   price = 9    → should be None
    //   price = 10   → should be Some(10)   ← boundary
    //   price = 11   → should be Some(11)
    //   price = 999  → should be Some(999)
    //   price = 1000 → should be Some(1000) ← boundary
    //   price = 1001 → should be None

    #[test]
    fn below_threshold() {
        let result = get_valid_sat_price(9);
        assert_eq!(result, None);
    }

    #[test]
    fn lower_bound_with_bug() {
        let result = get_valid_sat_price(10);
        assert_eq!(result, Some(10));
    }
    #[test]
    fn just_above_bound() {
        let result = get_valid_sat_price(11);
        assert_eq!(result, Some(11));
    }
    #[test]
    fn just_below_upper_bound() {
        let result = get_valid_sat_price(999);
        assert_eq!(result, Some(999));
    }
    #[test]
    fn upper_bound_fixed_bug_locally() {
        let result = get_valid_sat_price(1000);
        assert_eq!(result, Some(1000));
    }
    #[test]
    fn above_threshold() {
        let result = get_valid_sat_price(1001);
        assert_eq!(result, None);
    }
}