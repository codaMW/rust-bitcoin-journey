
use std::fmt;

trait BitcoinSerializable {

    fn serialize(&self) -> Vec<u8>;
    
    fn size(&self) -> usize {
        self.serialize().len()
    }
}

#[derive(Debug, Clone)]
struct TxOut {
    value: u64,           
    script_pubkey: Vec<u8>, 
}

impl BitcoinSerializable for TxOut {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        bytes.extend_from_slice(&self.value.to_le_bytes());
        
        bytes.push(self.script_pubkey.len() as u8);
        bytes.extend_from_slice(&self.script_pubkey);
        
        bytes
    }
}

// Step 4: Create a transaction input
#[derive(Debug, Clone)]
struct TxIn {
    prev_txid: [u8; 32],  // Previous transaction ID
    prev_vout: u32,        // Previous output index
    script_sig: Vec<u8>,   // Unlocking script
    sequence: u32,         // Sequence number
}

impl BitcoinSerializable for TxIn {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Previous transaction ID (32 bytes, little-endian? Actually big-endian in Bitcoin!)
        bytes.extend_from_slice(&self.prev_txid);
        
        // Previous vout (4 bytes, little-endian)
        bytes.extend_from_slice(&self.prev_vout.to_le_bytes());
        
        // ScriptSig length and data
        bytes.push(self.script_sig.len() as u8);
        bytes.extend_from_slice(&self.script_sig);
        
        // Sequence (4 bytes, little-endian)
        bytes.extend_from_slice(&self.sequence.to_le_bytes());
        
        bytes
    }
}

#[derive(Debug)]
struct Transaction {
    version: i32,
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    lock_time: u32,
}

// YOUR TASK: Implement BitcoinSerializable for Transaction
// HINT: Bitcoin transaction serialization format:
// - version (4 bytes, little-endian)
// - input count (CompactSize)
// - inputs (each serialized)
// - output count (CompactSize)
// - outputs (each serialized)
// - lock_time (4 bytes, little-endian)

impl BitcoinSerializable for Transaction {
    fn serialize(&self) -> Vec<u8> {
        todo!("Implement transaction serialization")
    }
}

// Step 6: Create a trait for transaction validation
trait TransactionValidator {
    fn is_coinbase(&self) -> bool;
    fn has_valid_outputs(&self) -> bool;
    fn total_output_value(&self) -> u64;
}

// YOUR TASK: Implement TransactionValidator for Transaction
impl TransactionValidator for Transaction {
    fn is_coinbase(&self) -> bool {
        todo!("A coinbase tx has exactly one input with null previous txid")
    }
    
    fn has_valid_outputs(&self) -> bool {
        todo!("Check: no outputs with value < dust (546 sats), total value <= 21M BTC")
    }
    
    fn total_output_value(&self) -> u64 {
        todo!("Sum all output values")
    }
}

fn main() {
    println!("🧱 Building Bitcoin Transaction System with Traits\n");
    
    // Let's create a simple transaction
    let tx = Transaction {
        version: 2,
        inputs: vec![
            TxIn {
                prev_txid: [1u8; 32], // Dummy txid
                prev_vout: 0,
                script_sig: vec![],
                sequence: 0xffffffff,
            }
        ],
        outputs: vec![
            TxOut {
                value: 100_000_000, // 1 BTC in sats
                script_pubkey: vec![0x00, 0x14, 0xaa; 20], // Dummy P2WPKH
            }
        ],
        lock_time: 0,
    };
    
    println!("Transaction: {:?}", tx);
    println!("Total output value: {} sats", tx.total_output_value());
    println!("Is coinbase? {}", tx.is_coinbase());
    
    // TODO: Print serialized bytes when implemented
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_txout_serialization() {
        let txout = TxOut {
            value: 100_000_000,
            script_pubkey: vec![0x00, 0x14, 0xaa; 20],
        };
        
        let bytes = txout.serialize();
        assert_eq!(bytes.len(), 8 + 1 + 20); // 8 bytes value + 1 len + 20 script
    }
    
    // TODO: Add more tests as you implement
}