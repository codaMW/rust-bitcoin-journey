use bitcoincore_rpc::{Auth, Client, RpcApi};

const RPC_URL:  &str = "http://127.0.0.1:18443";
const RPC_USER: &str = "alice";
const RPC_PASS: &str = "password";

fn main() -> bitcoincore_rpc::Result<()> {
    let rpc = Client::new(
        RPC_URL,
        Auth::UserPass(RPC_USER.to_string(), RPC_PASS.to_string()),
    )?;

    // get_new_address returns Address<NetworkUnchecked>
    // This means: "I have an address but I haven't verified
    // which network it belongs to yet"
    // You CANNOT use it directly — compiler won't allow it
    let unchecked = rpc
        .get_new_address(None, None)?;

    // Print the type by trying to use it wrong — read the error
    // Uncomment this line, run, read the error, then comment back
    // println!("Address: {}", unchecked);

    // assume_checked() says:
    // "I trust this address is for the correct network"
    // "Convert it from Address<NetworkUnchecked> to Address"
    // Safe here because WE generated it from OUR node
    let checked = unchecked.assume_checked();

    // Now we can use it
    println!("Address: {}", checked);
    println!("Network: {}", checked.network());

    Ok(())
}