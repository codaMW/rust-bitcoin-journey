fn main() {
    // ===== FN (Just reads) =====
    /*let price = 50_000;
    
    // This closure just reads — it's Fn
    let is_expensive = || price > 40_000;
    
    println!("Fn - can call many times:");
    println!("{}", is_expensive());  // true
    println!("{}", is_expensive());  // true
    println!("{}", is_expensive());  // true
    
    // ===== FNMUT (Changes things) =====
    let mut counter = 0;
    
    // This closure changes counter — it's FnMut
    let mut increment = || {
        counter += 1;
        println!("Counter is now: {}", counter);
    };
    
    println!("\nFnMut - changes state each time:");
    increment();  // counter: 1
    increment();  // counter: 2
    increment();  // counter: 3
    
    // ===== FNONCE (Consumes things) =====
    let data = vec!["utxo1", "utxo2", "utxo3"];
    
    // This closure takes ownership of data — it's FnOnce
    let consume = || {
        println!("Consuming: {:?}", data);
        drop(data);  // explicitly drop (though it would drop anyway)
    };
    
    println!("\nFnOnce - can only call once:");
    consume();  // works
    // consume();  // ❌ If you uncomment this, it won't compile
    
    // ===== YOUR TASKS =====
    
    // TASK 1: Create a Fn closure that checks if a block height is > 800,000
    // Capture a variable `min_height` set to 800,000
    // Call it twice and print results*/

    let check_height = |height| height > 800_000;
    let min_height = 800_000; 
    println!("{}", check_height(min_height));
    println!("{}", check_height(min_height));
    
    // TASK 2: Create a FnMut closure that keeps track of how many transactions
    // you've processed. It should increment a counter each time it's called
    // and print "Processed {} transactions"
    let mut count = 0;
    let mut tx_counter = || {
        count += 1;

        println!("Processed {} transactions", count);
    };

    tx_counter();
    tx_counter();
    tx_counter();
    
    // TASK 3: Create a FnOnce closure that takes ownership of a String
    // containing your node's version, and prints it
    // Call it once

    let my_node_verison = String::from("v2");
    let node_version = || {
        println!("Version = {}", my_node_verison);
        drop(my_node_verison);
    };

    node_version();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fn_closure() {
        let threshold = 500;
        let check = |x| x > threshold;  // Fn - just reads
        assert_eq!(check(600), true);
        assert_eq!(check(400), false);
    }
    
    #[test]
    fn test_fnmut_closure() {
        let mut count = 0;
        let mut increment = || count += 1;  // FnMut - changes
        
        increment();
        increment();
        increment();
        
        assert_eq!(count, 3);
    }
    
    // YOUR JOB: Write a test for a FnOnce closure
    // Create a String, capture it in a closure that consumes it
    // Call the closure, then verify (with a bool flag) that it executed
}