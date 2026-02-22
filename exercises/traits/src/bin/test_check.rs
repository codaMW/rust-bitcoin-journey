fn main() {
    let test = String::from("bcrt1qq2yshcmzdlznnpxx258xswqlmqcxjs4dssfxt2");
    println!("starts with bc1:  {}", test.starts_with("bc1"));
    println!("starts with bcrt1: {}", test.starts_with("bcrt1"));
}