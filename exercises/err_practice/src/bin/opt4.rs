fn main() {

    let num: Option<i32> = None;

    let num2 = num.expect("None valid number");

    println!("{}", num2);
}