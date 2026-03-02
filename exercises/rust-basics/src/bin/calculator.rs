// exercise2.rs
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    println!("Add: {}", add(10, 5));
    println!("Subtract: {}", subtract(10, 5));
    println!("Multiply: {}", multiply(10, 5));
    
    match divide(10, 2) {
        Some(result) => println!("Divide: {}", result),
        None => println!("Cannot divide by zero!"),
    }
}