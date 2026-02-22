trait RpcMethod {
    fn method_name(&self) -> &str;
    fn params(&self) -> String;

    // default
    fn full_call(&self) -> String {
        format!("METHOD: {} | PARAMS: {}", self.method_name(), self.params())
    }
}

trait Validatable {
    fn is_valid(&self) -> bool;

    // default
    fn validation_error(&self) -> Option<String> {
        if self.is_valid() {
            None
        } else {
            Some(format!("Invalid RPC call: {}", "validation failed"))
        }
    }
}

// Executable REQUIRES both RpcMethod and Validatable
trait Executable: RpcMethod + Validatable {
    fn execute(&self) {
        match self.validation_error() {
            None => println!("✓ Executing: {}", self.full_call()),
            Some(err) => println!("✗ Blocked: {}", err),
        }
    }
}

struct SendPayment {
    address: String,
    amount_btc: f64,
    fee_rate: f64,
}

impl RpcMethod for SendPayment {

    fn method_name(&self) -> &str {
        "sendtoaddress"
    }

    fn params(&self) -> String {
        format!("[{}, {}]", self.address, self.amount_btc)
    }

}

impl Validatable for SendPayment {
    fn is_valid(&self) -> bool {

         let valid_address = self.address.starts_with("bcrt1")
            || self.address.starts_with("bc1");

        let valid_amount = self.amount_btc > 0.0;

        let valid_fee = self.fee_rate >= 1.0 && self.fee_rate <= 1000.0;

        valid_address && valid_amount && valid_fee

    }
}

impl Executable for SendPayment {}

fn main() {
    let valid = SendPayment {
        address: String::from("bcrt1qq2yshcmzdlznnpxx258xswqlmqcxjs4dssfxt2"),
        amount_btc: 100.0,
        fee_rate: 21.0,
    };

    let bad_address = SendPayment {
        address: String::from("not_an_address"),
        amount_btc: 100.0,
        fee_rate: 21.0,
    };

    let zero_amount = SendPayment {
        address: String::from("bcrt1qq2yshcmzdlznnpxx258xswqlmqcxjs4dssfxt2"),
        amount_btc: 0.0,
        fee_rate: 21.0,
    };

    valid.execute();
    bad_address.execute();
    zero_amount.execute();
}