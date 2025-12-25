#[derive(Debug, PartialEq)]
#[allow(dead_code)]

pub enum AddressType {

    P2PKH{ pubkey_hash: String },
    P2SH { script_hash: String },
    P2WPKH { pubkey_hash: String },
    P2TR { x_only_pubkey: String },
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]

pub enum Network {
    
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]


pub struct Address {

    pub payload: String,
    pub network: Network,
    pub address_type: AddressType,
}

impl Address {

    pub fn validate_network(&self) -> bool {

        if self.network == Network::Mainnet || self.network == Network::Testnet {
            true
        } else {
            false
        }
    }

}
