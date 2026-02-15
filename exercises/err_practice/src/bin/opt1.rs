fn main() {

    let num: Option<i32> = None;

    match num {
        Some(val) => println!("{}", val),
        None => println!("No value")
    }
}