trait Serializable {
    fn serialize(&self) -> Vec<u8>;
}

struct RawTransaction {
    version: u32,
    locktime: u32,
}

impl Serializable for RawTransaction {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.version.to_le_bytes());
        bytes.extend_from_slice(&self.locktime.to_le_bytes());
        bytes
    }
}

fn main() {
    let tx = RawTransaction { version: 1, locktime: 0 };
    let serialized = tx.serialize();
    println!("Serialized transaction bytes: {:?}", serialized);
    println!("Total bytes: {}", serialized.len());
}

