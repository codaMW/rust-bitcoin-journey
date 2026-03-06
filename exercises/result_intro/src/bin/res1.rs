fn parse_amount(input: &str) -> Result<i64, String> {
    //TODO

    match input.parse::<i64>() {
        Ok(value) => {
            if value <= 0 {
                Err("amount must be positive".to_string())
            } else {
                Ok(value)
            }
        }

        Err(_) => Err("invalid number".to_string()),
    }
}

fn main() {
    let inputs = ["1000", "-5", "abdd"];

    for input in inputs {
        match parse_amount(input) {
            Ok(amount) => println!("Parsed amount: {}", amount),
            Err(err) => println!("Error parsing '{}': {}", input, err),
        }
    }
}
