trait MerkleNode {
    fn compute_hash(&self) -> String;
    fn is_leaf(&self) -> bool;
}

struct LeafNode {
    tx_hash: String,
}

struct BranchNode {
    left: String,
    right: String,
}

impl MerkleNode for LeafNode {
    fn compute_hash(&self) -> String {
        format!("hash({})", self.tx_hash)
    }
    fn is_leaf(&self) -> bool { true }
}

impl MerkleNode for BranchNode {
    fn compute_hash(&self) -> String {
        format!("hash({} + {})", self.left, self.right)
    }
    fn is_leaf(&self) -> bool { false }
}

fn build_merkle_root(nodes: Vec<Box<dyn MerkleNode>>) -> String {
    let hashes: Vec<String> = nodes.iter().map(|n| n.compute_hash()).collect();
    format!("MerkleRoot({})", hashes.join(", "))
}

fn main() {
    let leaf1 = Box::new(LeafNode { tx_hash: String::from("tx1abc") });
    let leaf2 = Box::new(LeafNode { tx_hash: String::from("tx2def") });
    let branch = Box::new(BranchNode {
        left: leaf1.compute_hash(),
        right: leaf2.compute_hash(),
    });

    println!("Leaf1 hash: {}", leaf1.compute_hash());
    println!("Leaf2 hash: {}", leaf2.compute_hash());
    println!("Branch hash: {}", branch.compute_hash());
    println!("Merkle Root: {}", build_merkle_root(vec![leaf1, leaf2, branch]));
}



