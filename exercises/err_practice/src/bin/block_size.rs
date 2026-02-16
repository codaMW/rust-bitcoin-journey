fn block_size(block: u64) -> Option<u64> {

    if block == 1 {return Some(850000);}
    else if block == 2 {return Some(1200000);}
    else if block == 3 {return Some(400000);}
    else if block == 4 {return Some(999999);}
    else if block == 5 {return Some(1000000);}
    else {None}
}

fn block_status(block: u64) -> Option<u64> {

    match block_size(block) {
        Some(val) => {
            if val >= 1000000 {Some(val)}
            else {None}
        },
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_no_value() {
        let result = block_status(1);
        assert!(result.is_none());
    }

    #[test]
    fn check_value() {
        let result = block_status(2);
        assert!(result.is_some());
    }

    #[test]
    fn block_not_completely_full() {
        let result = block_status(1);
        assert_eq!(result, None);
    }

    #[test]
    fn block_completely_full() {
        let result = block_status(2);
        assert_eq!(result, Some(1200000));
    }
    #[test]
    fn not_full_none() {
        let result = block_status(3);
        assert_eq!(result, None);
    }

    #[test]
    fn just_under_boundary() {
        let result = block_status(4);
        assert_eq!(result, None);
    }

    #[test]
    fn full_exactly() {
        let result = block_status(5);
        assert_eq!(result, Some(1000000));
    }

    #[test]
    fn non_existing_block() {
        let result = block_status(99);
        assert_eq!(result, None);
    }
}