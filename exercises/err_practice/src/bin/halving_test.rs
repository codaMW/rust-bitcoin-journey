fn get_block(block_id: u32) -> Option<u32> {
    // returns block height if block exists
    if block_id == 1 { Some(840_000) }
    else if block_id == 2 { Some(839_999) }
    else if block_id == 3 { Some(840_001) }
    else { None }
}

fn get_halving_tx(block_id: u32) -> Option<Option<u32>> {
    // A block is a "halving block" if its height == 840_000
    // Return Some(Some(height)) if block exists AND is halving block
    // Return Some(None)         if block exists BUT is not halving block
    // Return None               if block doesn't exist
    // YOUR CODE HERE

    match get_block(block_id){
        Some(height) => {
            if height == 840_000 {
                return Some(Some(height));
            } else {
                return Some(None);
            }
        },
        None => return None
    }
    
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // Write SIX tests:
    // 1. block 1 exists AND is halving block → Some(Some(840_000))
    // 2. block 2 exists BUT not halving      → Some(None)
    // 3. block 3 exists BUT not halving      → Some(None)
    // 4. block 99 doesn't exist              → None
    // 5. use is_some() on block 1 outer layer
    // 6. use is_none() on block 99

    #[test]
    fn halving_block() {
        let result = get_halving_tx(1);

        assert_eq!(result, Some(Some(840_000)));
    }

    #[test]
    fn present_but_not_halving_block() {
        let result = get_halving_tx(2);

        assert_eq!(result, Some(None));
    }

    #[test]
    fn present_but_not_halving_block_2() {
        let result = get_halving_tx(3);

        assert_eq!(result, Some(None));
    }

    #[test]
    fn absent_block() {
        let result = get_halving_tx(99);

        assert_eq!(result, None);
    }

    #[test]
    fn is_block_present_1() {
        let result = get_halving_tx(1);

        assert!(result.is_some());
    }

    #[test]
    fn is_block_absent() {
        let result = get_block(99);

        assert!(result.is_none());
    }
}