fn parse_btc_amount(input: &str) -> Result<f64, String> {

    match input.parse::<f64>() {
        Ok(amount) => {
            if amount < 0 {
                Err("Amount cannot be negative") else {
                    Ok(amount)
                }
            }
        }

    _ => Err("Invalid amount:{input}")
    }

}

fn main() {
    parse_btc_amount("1.5")?;
    parse_btc_amount("abc")?;
    parse_btc_amount("0.5")?;
}



