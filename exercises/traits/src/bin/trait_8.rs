trait Hashable {
    fn hash(&self) -> String;
}

struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

impl Hashable for Transaction {
    fn hash(&self) -> String {
        format!("{:x}", self.sender.len() + self.receiver.len() + self.amount as usize)
    }
}

fn main() {
    let tx = Transaction {
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
        amount: 1.5,
    };
    println!("Transaction hash: {}", tx.hash());
}