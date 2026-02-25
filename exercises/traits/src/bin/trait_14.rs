trait NetworkPeer {
    fn broadcast(&self, message: &str);
    fn receive(&self) -> String;
    fn peer_count(&self) -> usize;
}

struct FullNode {
    address: String,
    peers: Vec<String>,
}

struct LightNode {
    address: String,
}

impl NetworkPeer for FullNode {
    fn broadcast(&self, message: &str) {
        for peer in &self.peers {
            println!("[FullNode {}] -> [{}]: {}", self.address, peer, message);
        }
    }
    fn receive(&self) -> String {
        format!("ACK from full node {}", self.address)
    }
    fn peer_count(&self) -> usize {
        self.peers.len()
    }
}

impl NetworkPeer for LightNode {
    fn broadcast(&self, message: &str) {
        println!("[LightNode {}] -> upstream: {}", self.address, message);
    }
    fn receive(&self) -> String {
        format!("ACK from light node {}", self.address)
    }
    fn peer_count(&self) -> usize { 1 }
}

fn main() {
    let full = FullNode {
        address: String::from("192.168.1.1:8333"),
        peers: vec![
            String::from("10.0.0.1:8333"),
            String::from("10.0.0.2:8333"),
        ],
    };
    let light = LightNode { address: String::from("192.168.1.2:8333") };

    full.broadcast("new block: height=820000");
    println!("{}", full.receive());
    println!("Full node peers: {}", full.peer_count());

    light.broadcast("tx: Alice->Bob 0.1 BTC");
    println!("{}", light.receive());
}
