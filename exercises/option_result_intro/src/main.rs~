/*
 * In Bitcoin, some transactions do not have a locktime.
 *
 * Task
 * Implement a function that:
 *
 * Takes Option<u32> representing lock_time
 * Return a human-readable  description
 *
 */

fn describe_locktime(lock_time: Option<u32>) -> String {
    match lock_time {
        //TODO()
        Some(0) => "No Locktime".to_string(),
        Some(value) => format!("Locktime provided is {}", value),
        None => "No Locktime provided".to_string(),
    }
}

fn main() {
    let some_0 = describe_locktime(Some(0));
    let some_500 = describe_locktime(Some(500));
    let none_value = describe_locktime(None);

    println!("{}", some_0);
    println!("{}", some_500);
    println!("{}", none_value);
}
