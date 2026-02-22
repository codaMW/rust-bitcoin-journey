trait RpcSerialize {
    fn method_name(&self) -> String;
    fn to_rpc_params(&self) -> String;

    fn full_rpc_call(&self) -> String {
        format!(
            "METHOD: {} | PARAMS: {}",
            self.method_name(),
            self.to_rpc_params()
        )
    }
}


struct GetBlockchainInfo;

impl RpcSerialize for GetBlockchainInfo {
    fn method_name(&self) -> String {
        String::from("getblockchaininfo")
    }

    fn to_rpc_params(&self) -> String {
        format!("[]")
    }

}

#[allow(unused)]
struct GenerateToAddress {
    blocks: u32,
    address: String
}

impl RpcSerialize for GenerateToAddress {
    fn method_name(&self) -> String {
        String::from("generatetoaddress")
    }

    fn to_rpc_params(&self) -> String {
        format!("[{}, \"{}\"]", self.blocks, self.address)
    }
}

#[allow(unused)]
struct SendToAddress{
    address: String,
    amount_btc: f64
}

impl RpcSerialize for SendToAddress {
    fn method_name(&self) -> String {
        String::from("sendtoaddress")
    }

    fn to_rpc_params(&self) -> String {
        format!("[\"{}\", {}]", self.address, self.amount_btc)
    }
}


fn execute_rpc<T>(call: &T) where T: RpcSerialize {
    println!("{}", call.full_rpc_call());
}

fn main() {

    let getblockchaininfo = GetBlockchainInfo;
    let generatetoaddress = GenerateToAddress {blocks: 201, address: String::from("bcrt1qq2yshcmzdlznnpxx258xswqlmqcxjs4dssfxt2")};
    let sendtoaddress = SendToAddress {address: String::from("bcrt1qq2yshcmzdlznnpxx258xswqlmqcxjs4dssfxt2"), amount_btc: 100.0};

    execute_rpc(&getblockchaininfo);
    execute_rpc(&generatetoaddress);
    execute_rpc(&sendtoaddress);
}