pub fn divide(a: f64, b: f64) -> Result<f64, String>{
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a/b)
    }
}

fn main() {

    let result = divide(10.0, 2.0);

    match result {
        Ok(val) => println!("{}", val),
        Err(e) => println!("{}", e),
    }

    let result_2 = divide(10.0, 0.0);

    match result_2 {
        Ok(val) => println!("{}", val),
        Err(e) => println!("{}", e),
    }
}