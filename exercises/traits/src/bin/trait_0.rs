trait Summarize {
    fn summary(&self) -> String;
}

#[derive(Debug)]
struct Block {
    height: u32,
    tx_count: u32,
    miner: String,
}

#[derive(Debug)]
struct MempoolEntry {
    txid: String,
    fee_sats: u64,
    size: u32,
}

impl Summarize for Block {
    fn summary(&self) -> String {
        format!(
            "#{} | {} | mined by {}",
            self.height, self.tx_count, self.miner
        )
    }
}

impl Summarize for MempoolEntry {
    fn summary(&self) -> String {
        format!(
            "TX {} | {} sats | {} bytes",
            self.txid, self.fee_sats, self.size
        )
    }
}

fn announce<T>(item: &T)
where
    T: Summarize,
{
    println!("{}", item.summary());
}

fn main() {
    let block = Block {
        height: 800000,
        tx_count: 3106,
        miner: String::from("Braiins"),
    };

    let mempool = MempoolEntry {
        txid: String::from("015699786"),
        fee_sats: 5106,
        size: 300,
    };

    announce(&block);
    announce(&mempool);
}
