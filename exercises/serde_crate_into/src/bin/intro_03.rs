use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{from_str, to_string_pretty};
use chrono::{DateTime, Utc, TimeZone};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Block {
    #[serde(deserialize_with = "validate_hash")]
    hash: String,
    height: u64,
    #[serde(serialize_with = "serialize_timestamp")]
    timestamp: u64,
    prev_block_hash: String,  // rename_all handles this
    merkle_root: String,
    #[serde(default = "default_difficulty")]
    difficulty: Option<f64>,
    nonce: u64,
}

fn default_difficulty() -> Option<f64> {
    Some(1.0)
}

// Task 4: Custom deserializer to validate hash
fn validate_hash<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let hash = String::deserialize(deserializer)?;
    
    // Check if hash starts with enough zeros (e.g., at least 4)
    if !hash.starts_with("0000") {
        return Err(serde::de::Error::custom(
            format!("Hash must start with at least 4 zeros, got: {}", hash)
        ));
    }
    
    Ok(hash)
}

// Task 1: Serialize timestamp as both u64 and ISO 8601
fn serialize_timestamp<S>(timestamp: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeStruct;
    
    let dt = Utc.timestamp_opt(*timestamp as i64, 0).unwrap();
    let iso_string = dt.to_rfc3339();
    
    // Serialize as a struct with both raw and formatted
    let mut state = serializer.serialize_struct("Timestamp", 2)?;
    state.serialize_field("raw", timestamp)?;
    state.serialize_field("iso8601", &iso_string)?;
    state.end()
}

fn main() {
    let node_data = r#"
    {
        "hash": "00000000000000000001234567890abcdef",
        "height": 800000,
        "timestamp": 1704067200,
        "prevBlockHash": "00000000000000000009876543210fedcba",
        "merkleRoot": "abcdef1234567890",
        "difficulty": 12345678.90,
        "nonce": 123456789
    }
    "#;

    let block_data = from_str::<Block>(&node_data);

    match block_data {
        Ok(val) => {
            println!("Deserialized:\n{:#?}", val);
            println!("\nSerialized:");
            println!("{}", to_string_pretty(&val).unwrap());
        },
        Err(e) => println!("Error: {}", e)
    }
}