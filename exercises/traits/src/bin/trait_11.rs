trait Mineable {
    fn mine(&self, difficulty: usize) -> String;
    fn meets_target(&self, hash: &str, difficulty: usize) -> bool;
}

struct Block {
    index: u64,
    data: String,
    previous_hash: String,
}

impl Mineable for Block {
    fn mine(&self, difficulty: usize) -> String {
        let prefix = "0".repeat(difficulty);
        let mut nonce = 0u64;
        loop {
            let hash = format!("{}{}{}{}", self.previous_hash, self.index, self.data, nonce);
            let simulated = format!("{:0>width$}", nonce % 100000, width = difficulty + 5);
            if simulated.starts_with(&prefix) {
                println!("Block #{} mined! nonce={} hash={}", self.index, nonce, simulated);
                return simulated;
            }
            nonce += 1;
            if nonce > 100000 {
                return format!("{}TIMEOUT", prefix);
            }
        }
    }

    fn meets_target(&self, hash: &str, difficulty: usize) -> bool {
        hash.starts_with(&"0".repeat(difficulty))
    }
}

fn main() {
    let block = Block {
        index: 1,
        data: String::from("Alice->Bob: 1 BTC"),
        previous_hash: String::from("0000abc123"),
    };
    let hash = block.mine(2);
    println!("Meets target: {}", block.meets_target(&hash, 2));
}

