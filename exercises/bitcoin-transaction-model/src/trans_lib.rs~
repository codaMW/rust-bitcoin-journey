#[derive(Debug)]
#[allow(dead_code)]
pub enum Version {
    V1(u64),
    V2(u64),
}

impl Version {
    pub fn get_version(&self) -> u64 {
        match self {
            Self::V1(value) | Self::V2(value) => *value,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Inputs {
    pub txid: String,
    pub index: u64,
    pub script_len: u64,
    pub script_sig: String,
    pub nsequence: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Outputs {
    pub amount: u64,
    pub script_len: u64,
    pub script_pub_key: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Transaction {
    pub version: Version,
    pub input_counter: u64,
    pub input: Vec<Inputs>,
    pub output_counter: u64,
    pub output: Vec<Outputs>,
    pub locktime: u64,
}

impl Transaction {
    pub fn input_counter_vec(&self) -> u64 {
        self.input.len() as u64
    }

    pub fn output_counter_vec(&self) -> u64 {
        self.output.len() as u64
    }
}
