pub enum Network {

    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

impl Network {

    pub fn magic_bytes(network: Network) -> String {

        match network {

            Self::Mainnet => String::from("Mainnet magic bytes: f9beb4d9"),
            Self::Testnet => String::from("Testnet magic bytes: Ob119090"),
            Self::Signet => String::from("Signet magic bytes: tb1q00gh8"),
            Self::Regtest => String::from("Regtest magic bytes: fabfb5da"),
        }
    }
}




