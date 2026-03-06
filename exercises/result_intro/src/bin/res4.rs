pub fn validate_amount(amount: f64) -> Result<f64, String>

{
    if amount <= 0.0 {
        Err(String::from("Amount must be greater than zero"))
    } else {
        Ok(amount)
    }
}

pub fn check_balance(amount: f64, balance: f64) -> Result<f64, String> {
    if amount > balance {
        Err(String::from("Insufficient balance"))
    } else {
        Ok(amount)
    }
}

pub fn process_tx(amount: f64, balance: f64) -> Result<(), String> {
    validate_amount(amount)?;
    check_balance(amount, balance)?;
    println!("Transcation of {} sats processed successfully", amount);

    Ok(())
}

fn main() {
    match process_tx(1000.0, 5000.0){

        Ok(()) => (),
        Err(e) => println!("{}", e),

    };

    match process_tx(0.0, 5000.0){

        Ok(()) => (),
        Err(e) => println!("{}", e),

    };

    match process_tx(6000.0, 5000.0){

        Ok(()) => (),
        Err(e) => println!("{}", e),

    };
    


}