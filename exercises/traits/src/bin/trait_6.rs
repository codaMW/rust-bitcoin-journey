use std::fmt;

#[derive(Debug)]
struct BitcoinAmount(u64);

impl fmt::Display for BitcoinAmount {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 >= 100_000_000 {
            write!(f, "{:.8} BTC", self.0 as f64 / 100_000_000.0)
        } else {
            write!(f, "{} sats", self.0)
        }
    }
}

#[derive(Debug)]
struct NodeInfo {
    url: String,
    network: String,
    block_height: u32,
}

impl fmt::Display for NodeInfo{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {} | height: {}", self.network, self.url, self.block_height)
    }
    
}

  
fn main() {

    let amount = BitcoinAmount(5000000000);

    let node = NodeInfo {
        url: String::from("127.0.0.1:18443"),
        network: String::from("regtest"),
        block_height: 201,
    };

    println!("{}", amount);
    println!("{:?}", amount);
    println!("{}", node);
    println!("{:?}", node); 


}