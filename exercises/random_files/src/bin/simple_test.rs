fn double(x: u32) -> u32 {
    x * 2
}

fn main() {
    println!("double(21) = {}", double(21));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_double() {
        assert_eq!(double(21), 42);
    }
}