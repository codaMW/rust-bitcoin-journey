fn main() {

    let num: Option<i32> = Some(2);

    let double = num.map(|n| n * 2);

    println!("{}", double.unwrap());
}