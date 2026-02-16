fn get_peer_count(node_id: u32) -> Option<u32> {
    if node_id == 1 { Some(8) }   // 8 peers connected
    else if node_id == 2 { Some(0) } // connected but no peers
    else if node_id == 3 { Some(125) } // max peers
    else { None }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_with_peers_returns_some() {
        let result = get_peer_count(1);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_node_with_no_peers_is_still_some() {
        // 0 peers ≠ None — node EXISTS just has no peers
        let result = get_peer_count(2);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_missing_node_returns_none() {
        let result = get_peer_count(99);
        assert_eq!(result, None);
    }

    // NEW CONCEPT: assert! with is_some() and is_none()
    #[test]
    fn test_node_3_has_peers_using_is_some() {
        let result = get_peer_count(3);
        assert!(result.is_some()); // just check SOMETHING exists
    }

    #[test]
    fn test_missing_node_using_is_none() {
        let result = get_peer_count(99);
        assert!(result.is_none()); // just check it's empty
    }

    // YOUR JOB: Write these two tests
    // test 1: called test_node_1_is_not_none
    //         use assert_ne! to prove node 1 does NOT return None

    #[test]
    fn test_node_1_is_not_none() {
        let result = get_peer_count(1);

        assert_ne!(result, None);
    }
    //
    // test 2: called test_max_peers
    //         prove node 3 returns Some(125)

    #[test]

    fn test_max_peers() {
        let result = get_peer_count(3);

        assert_eq!(result, Some(125));
    }

}