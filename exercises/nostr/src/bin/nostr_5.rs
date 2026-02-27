fn is_replaceable(kind: u32) -> bool {
    kind == 0
        || kind == 3
        || (10000..20000).contains(&kind)
}

fn main() {
    println!("Kind 0 replaceable? {}", is_replaceable(0));
    println!("Kind 1 replaceable? {}", is_replaceable(1));
    println!("Kind 15000 replaceable? {}", is_replaceable(15000));
}