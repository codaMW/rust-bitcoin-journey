
fn get_utxo_age(utxo_id: u32) -> Option<u32> {

    if utxo_id == 1 {return Some(150);}
    else if utxo_id == 2 {return Some(50);} 
    else if utxo_id == 3 {return Some(100);} 
    else if utxo_id == 4 {return Some(99);} 
    else if utxo_id == 5 {return Some(200);} 
    else {None} 

}

fn get_mature_utxo(utxo_id: u32) -> Option<u32> {
    match get_utxo_age(utxo_id) {
        Some(val) => {
            if val >= 100 {
                Some(val)
            } else {
                None
            }
        },
        None => None,
    }
}


fn main(){}

    #[cfg(test)]
    mod test{
        use super::*;

        #[test]
        fn mature_utxo_1() {
            let result = get_mature_utxo(1);
            assert_eq!(result, Some(150));
        }

        #[test]
        fn too_young() {
            let result = get_mature_utxo(2);
            assert_eq!(result, None);
        }

        #[test]
        fn exactly_at_bound() {
            let result = get_mature_utxo(3);
            assert_eq!(result, Some(100));
        }

        #[test]
        fn just_under_bound() {
            let result = get_mature_utxo(4);
            assert_eq!(result, None);
        }

        #[test]
        fn mature_utxo_5() {
            let result = get_mature_utxo(5);
            assert_eq!(result, Some(200));
        }

        #[test]
        fn not_available() {
            let result = get_mature_utxo(99);
            assert_eq!(result, None);
        }

        #[test]
        fn is_utxo_1() {
            let result = get_mature_utxo(1);
            assert!(result.is_some());
        }

        #[test]
        fn is_utxo_2() {
            let result = get_mature_utxo(2);
            assert!(result.is_none());
        }
}