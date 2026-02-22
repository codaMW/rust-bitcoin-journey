
trait TransactionInfo {

    //Required methods
    fn txid(&self) -> &str;
    fn fee_sats(&self) -> u64;
    fn vsize(&self) -> u32;

    //Default methods
    fn fee_rate(&self) -> f64 {
        self.fee_sats() as f64 / self.vsize() as f64   
    }

    fn is_high_fee(&self) -> bool {
        self.fee_rate() > 50.0
    }

    fn summary(&self) -> String {

        format!(
            "TX {} | {} sats | {} vBytes | {:.2} sats/vB | high_fee: {}",
            self.txid(),
            self.fee_sats(),
            self.vsize(),
            self.fee_rate(),
            self.is_high_fee()
        )

    }

}


struct SegwitTx {
    txid: String,
    fee_sats: u64,
    vsize: u32
}

impl TransactionInfo for SegwitTx {
    fn txid(&self) -> &str {
        &self.txid
    }

    fn fee_sats(&self) -> u64 {
        self.fee_sats
    }

    fn vsize(&self) -> u32 {
        self.vsize
    }

}

struct LegacyTx {
    txid: String,
    fee_sats: u64,
    size: u32
}

impl TransactionInfo for LegacyTx {
    fn txid(&self) -> &str {
        &self.txid
    }
    fn fee_sats(&self) -> u64 {
        self.fee_sats
    }
    fn vsize(&self) -> u32 {
        self.size
    }

    
}



fn main() {
    let seg = SegwitTx  { txid: "05fa7843326d501fecbf7870c887064ce10b6dd18f5b1a232821aa98dea4c0e8".to_string(), fee_sats: 2940, vsize: 140 };
    let leg = LegacyTx  { txid: "def456...64chars".to_string(), fee_sats: 4746, size: 226  };

    println!("{}", seg.summary());
    println!("{}", leg.summary());
}




//TX 05fa7843... | 2940 sats | 140 vBytes | 21.00 sats/vB | high_fee: false
//TX def456...   | 4746 sats | 226 vBytes | 21.00 sats/vB | high_fee: false