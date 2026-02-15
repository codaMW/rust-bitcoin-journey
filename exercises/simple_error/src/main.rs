// TODO: Complete this function
// It should return Ok(result) for valid division
// and Err(message) for division by zero

fn divide(a: f64, b: f64) -> Result<f64, String>
{
    if b == 0.0 {
        return Err("Cannot divide by zero".to_string());
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result1 = divide(10.0, 2.0);
    let result2 = divide(5.0, 0.0);

    match result1 {
        Ok(val) => println!("{}", val),
        Err(message) => println!("{}", message)
    }

    match result2 {
        Ok(val) => println!("{}", val),
        Err(message) => println!("{}", message)
    }
}