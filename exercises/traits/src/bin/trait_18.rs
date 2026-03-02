trait NodeInfo {
    fn url(&self) -> &str;
    fn network(&self) -> &str;

    fn describe(&self) -> String {
        format!("[{}] {}", self.network(), self.url())
    }
}


struct RegtestNode {
    url: String,
}

impl NodeInfo for RegtestNode {

    fn url(&self) -> &str {
        &self.url
    }

    fn network(&self) -> &str {
        "regtest"
    }

}


struct MainnetNode {
    url: String,
}

impl NodeInfo for MainnetNode {

    fn url(&self) -> &str {
        &self.url
    }

    fn network(&self) -> &str {
        "mainnet"
    }
}

fn main() {

    let reg = RegtestNode {url: "127.0.0.1:18443".to_string()};
    let man = MainnetNode {url: "127.0.0.1:833".to_string()};

    println!("{}", reg.describe());
    println!("{}", man.describe());

}