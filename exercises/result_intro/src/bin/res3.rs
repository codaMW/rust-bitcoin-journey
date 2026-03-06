pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by 0.0".to_string())
    } else {
        Ok(a / b)
    }
}
pub fn run() -> Result<(), String> {
    println!("{}", divide(10.0, 2.0)?);
    println!("{}", divide(10.0, 0.0)?);


    Ok(())

}

fn main() {

    match run(){
        Ok(()) => println!("Done"),
        Err(e) => println!("Error: {}", e),
    };


}