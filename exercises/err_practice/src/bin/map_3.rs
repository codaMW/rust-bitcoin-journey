fn format_block_height(height: Option<u32>) -> Option<String> {
    // If height exists, format as "Block #{}"
    // Example: Some(800000) -> Some("Block #800000".to_string())
    // None -> None

    height.map(|x| format!("Block #{}", x))
}

fn main() {}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn block_format() {
        let result = format_block_height(Some(800000));

        assert_eq!(result, Some("Block #800000".to_string()));
    }

    #[test]
    fn no_nlock() {
        let result_2 = format_block_height(None);
        
        assert_eq!(result_2, None);
    }
}