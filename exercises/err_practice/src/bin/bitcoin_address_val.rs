// TODO: Implement Bitcoin address validation that returns Option
// HINT: Bitcoin addresses start with 1, 3, or bc1

struct AddressValidator;

impl AddressValidator {
    fn validate_legacy(address: &str) -> Option<bool> {
        // Legacy addresses start with '1'
        // Return Some(true) if valid, None if not a legacy address
        //todo!("Implement me!")

        if address.starts_with("1") {
            return Some(true);
        } else {
            return None;
        }
    }
    
    fn validate_segwit(address: &str) -> Option<bool> {
        // SegWit addresses start with '3'
        //todo!("Implement me!")

        if address.starts_with("3") {
            return Some(true);
        } else {
            return None;
        }
    }
    
    fn validate_bech32(address: &str) -> Option<bool> {
        // Bech32 addresses start with 'bc1'
        //todo!("Implement me!")

        if address.starts_with("bc1") {
            return Some(true);
        } else {
            return None;
        }
    }
}

fn main() {}

// Your tests:
// "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa" -> Some(true) for legacy
// "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq" -> Some(true) for bech32
// "invalid" -> None for all methods

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn legacy_address_validator() {

        let result = AddressValidator::validate_legacy("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
        assert_eq!(result, Some(true));
    }


    #[test]
    fn bech32_address_validator() {

        let result = AddressValidator::validate_bech32("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq");
        assert_eq!(result, Some(true));

    }

    #[test]
    fn invalid_address_validator() {

        let result_1 = AddressValidator::validate_legacy("3A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
        assert_eq!(result_1, None);

        let result_2 = AddressValidator::validate_bech32("1A1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq");
        assert_eq!(result_2, None);

        let result_3 = AddressValidator::validate_segwit("bcq1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
        assert_eq!(result_3, None);

    }

}