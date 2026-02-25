trait Validatable {
    fn is_valid(&self) -> bool;
    fn validation_error(&self) -> Option<String>;
}

struct Utxo {
    amount: f64,
    spent: bool,
    confirmations: u32,
}

impl Validatable for Utxo {
    fn is_valid(&self) -> bool {
        !self.spent && self.amount > 0.0 && self.confirmations >= 1
    }

    fn validation_error(&self) -> Option<String> {
        if self.spent {
            return Some(String::from("UTXO already spent"));
        }
        if self.amount <= 0.0 {
            return Some(String::from("Invalid UTXO amount"));
        }
        if self.confirmations < 1 {
            return Some(String::from("UTXO not yet confirmed"));
        }
        None
    }
}

fn main() {
    let utxo1 = Utxo { amount: 0.5, spent: false, confirmations: 3 };
    let utxo2 = Utxo { amount: 0.5, spent: true, confirmations: 3 };
    let utxo3 = Utxo { amount: 0.0, spent: false, confirmations: 0 };

    for (i, utxo) in [&utxo1, &utxo2, &utxo3].iter().enumerate() {
        println!("UTXO {}: valid={}, error={:?}", i + 1, utxo.is_valid(), utxo.validation_error());
    }
}


