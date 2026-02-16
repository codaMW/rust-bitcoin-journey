fn get_node_version(node_id: u32) -> Option<String> {
    if node_id == 1 { Some(String::from("0.21.0")) }
    else if node_id == 2 { Some(String::from("0.20.1")) }
    else if node_id == 3 { Some(String::from("0.19.0")) }
    else { None }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_node_returns_version() {
        let result = get_node_version(1);
        assert_eq!(result, Some(String::from("0.21.0")));
    }

    #[test]
    fn test_another_known_node() {
        let result = get_node_version(3);
        assert_eq!(result, Some(String::from("0.19.0")));
    }

    // YOUR JOB 1:
    // Write a test called test_unknown_node_returns_none
    // that proves get_node_version(99) returns None
    #[test]
    fn test_unknown_node_returns_none() {
        // your code here

        let result = get_node_version(99);

        assert_eq!(result, None);
    }

    // YOUR JOB 2:
    // Write a test called test_node_2_version
    // that proves get_node_version(2) returns Some("0.20.1")
    #[test]
    fn test_node_2_version() {
        // your code here

        let result = get_node_version(2);

        assert_eq!(result, Some("0.20.1".to_string()));
    }
}