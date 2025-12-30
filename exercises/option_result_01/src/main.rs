fn normalize_currency(currency: Option<&str>) -> String {
    match currency {
        Some(value) => value.to_uppercase(),
        None => "UNKNOWN".to_string(),
    }
}

fn main() {
    let usd = normalize_currency(Some("usd"));
    let euro = normalize_currency(Some("euro"));
    let unknown = normalize_currency(None);

    println!("{}", usd);
    println!("{}", euro);
    println!("{}", unknown);
}
