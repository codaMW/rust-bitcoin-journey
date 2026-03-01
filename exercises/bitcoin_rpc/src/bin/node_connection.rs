use bitcoincore_rpc::{Auth, Client, RpcApi};

const RPC_URL:  &str = "http://127.0.0.1:18443";
const RPC_USER: &str = "alice";
const RPC_PASS: &str = "password";

fn main() -> bitcoincore_rpc::Result<()> {

    // Step 1: Connect to the node
    let rpc = Client::new(
        RPC_URL,
        Auth::UserPass(RPC_USER.to_string(), RPC_PASS.to_string()),
    )?;

    // Step 2: Call getblockchaininfo
    let info = rpc.get_blockchain_info()?;

    // Step 3: Print what we got back
    //println!("Chain:        {}", info.chain);
    //println!("Block height: {}", info.blocks);
    //println!("Best hash:    {}", info.best_block_hash);
    //println!("Difficulty:   {}", info.difficulty);
    println!("{:?}", info);

    Ok(())
}