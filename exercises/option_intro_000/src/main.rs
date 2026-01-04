fn main() {

   let addr: Option<&str> = Some("bcq1mhU89");

   if addr.is_some_and(|s| &s[..3] == "bcq") {
    println!("Segwit address format");
   } else {
    println!("Invalid address type for this operation");
   }
  
}
