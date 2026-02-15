fn main() {

    let num: Option<i32> = Some(25);

    let num2: i32 = num.unwrap();

    println!("{}", num2);
}