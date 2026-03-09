fn main(){
let mbytes: &[u8] = b"F9BEB4D9";

let payload = "7E1101000000000000000000C515CF6100000000000000000000000000000000000000000000FFFF2E13894A208D000000000000000000000000000000000000FFFF7F000001208D00000000000000000000000000";
let hexed = hex::decode(payload).unwrap();
let s_size = hexed.len() as u32;
let b_size = s_size.to_le_bytes();

println!("{}", s_size);
println!("{:?}", mbytes);
println!("{:?}", b_size);
}
