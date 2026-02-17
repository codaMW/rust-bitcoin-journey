fn main() {
    // TASK 1: Create a closure called `add_fee`
    // that takes two parameters and adds them
    // Test it with add_fee(100, 50) — should print 150

    let add_fee = |a,b| a + b;
    println!("{}",add_fee(100,50));

    // TASK 2: Create a closure called `is_high_fee`
    // that takes one parameter and returns true if > 500
    // Test with is_high_fee(600) and is_high_fee(200)

    let is_high_fee = |a| a > 500;

    println!("{}", is_high_fee(600));
    println!("{}", is_high_fee(200));

    // TASK 3: Create a closure called `print_tx`
    // that takes no parameters and prints "Processing transaction..."
    // Call it twice

    let print_tx = || println!("Processing transaction...");

    print_tx();
    print_tx();
}

#[cfg(test)]
mod tests {
    // TASK 4: Write a test that creates a closure
    // and asserts that double(5) == 10
    // where double is |x| x * 2
    #[test]
    fn closure() {
    let double = |x| x * 2;
    assert_eq!(double(5), 10);
}
}