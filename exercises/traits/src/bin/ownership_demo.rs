struct Node {
    url: String,
}

trait Ping {
    fn ping(self) -> String;  // notice: self, not &self
}

impl Ping for Node {
    fn ping(self) -> String {
        format!("Pinging {}", self.url)
    }
}

fn main() {
    let node = Node { url: String::from("127.0.0.1:18443") };
    println!("{}", node.ping());
    println!("{}", node.ping()); // what happens here?
}