// exercise3.rs
fn main() {
    let hello = String::from("Hello");
    let world = String::from("World");
    
    // Concatenation
    let hello_world = format!("{} {}", hello, world);
    println!("{}", hello_world);
    
    // String length
    println!("Length: {}", hello_world.len());
    
    // Uppercase and lowercase
    println!("Uppercase: {}", hello_world.to_uppercase());
    println!("Lowercase: {}", hello_world.to_lowercase());
    
    // Contains
    println!("Contains 'World'? {}", hello_world.contains("World"));
}