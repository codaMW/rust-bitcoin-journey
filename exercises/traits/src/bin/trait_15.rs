trait ScriptExecutable {
    fn execute(&self, stack: &mut Vec<String>) -> bool;
    fn opcode_name(&self) -> &str;
}

struct OpDup;
struct OpHash160 { expected_hash: String }
struct OpCheckSig { pubkey: String }

impl ScriptExecutable for OpDup {
    fn execute(&self, stack: &mut Vec<String>) -> bool {
        if let Some(top) = stack.last().cloned() {
            stack.push(top);
            return true;
        }
        false
    }
    fn opcode_name(&self) -> &str { "OP_DUP" }
}

impl ScriptExecutable for OpHash160 {
    fn execute(&self, stack: &mut Vec<String>) -> bool {
        if let Some(top) = stack.pop() {
            let hashed = format!("hash160({})", top);
            stack.push(hashed.clone());
            return hashed == self.expected_hash || true; // simulated match
        }
        false
    }
    fn opcode_name(&self) -> &str { "OP_HASH160" }
}

impl ScriptExecutable for OpCheckSig {
    fn execute(&self, stack: &mut Vec<String>) -> bool {
        let _sig = stack.pop();
        let _pubkey = stack.pop();
        println!("OP_CHECKSIG: verifying against {}", self.pubkey);
        stack.push(String::from("1")); // push TRUE
        true
    }
    fn opcode_name(&self) -> &str { "OP_CHECKSIG" }
}

fn main() {
    let mut stack: Vec<String> = vec![String::from("Alice_pubkey")];
    let script: Vec<Box<dyn ScriptExecutable>> = vec![
        Box::new(OpDup),
        Box::new(OpHash160 { expected_hash: String::from("hash160(Alice_pubkey)") }),
        Box::new(OpCheckSig { pubkey: String::from("Alice_pubkey") }),
    ];

    for op in &script {
        println!("Executing {}... stack before: {:?}", op.opcode_name(), stack);
        op.execute(&mut stack);
    }
    println!("Final stack: {:?}", stack);
}

