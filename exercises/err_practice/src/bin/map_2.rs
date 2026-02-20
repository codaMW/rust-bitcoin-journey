fn uppercase_address(address: Option<&str>) -> Option<String> {
    // Hint: &str has a .to_uppercase() method that returns String
    // Remember: map transforms what's inside the Option

    address.map(|x| x.to_uppercase())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn uppercase() {

        let result = uppercase_address(Some("bc1quhruqrghgcca950rvhtrg7cpd7u8k6svpzgzmrjy8xyukacl5lkq0r8l2d"));
        assert_eq!(result, Some(String::from("BC1QUHRUQRGHGCCA950RVHTRG7CPD7U8K6SVPZGZMRJY8XYUKACL5LKQ0R8L2D")));
    }
}