use ethers_core::{
    types::U256,
    utils::{format_units, parse_ether},
};

pub fn to_wei(eth: String) -> U256 {
    match parse_ether(eth) {
        Ok(wei) => wei,
        Err(e) => panic!("{}", e.to_string()),
    }
}

pub fn to_eth(wei: String) -> String {
    let wei = match U256::from_dec_str(&wei) {
        Ok(w) => w,
        Err(_) => panic!("Invalid Wei number: {}", wei),
    };

    let parsed_eth = format_units(wei, "ether").unwrap();

    if parsed_eth.contains(".") {
        parsed_eth
            .trim_end_matches("0")
            .trim_end_matches(".")
            .to_string()
    } else {
        parsed_eth
    }
}

#[cfg(test)]
mod tests {
    use super::to_eth;

    #[test]
    fn parse_to_eth_integer() {
        assert_eq!(to_eth("1000000000000000000".to_string()), "1".to_string())
    }

    #[test]
    #[should_panic = "Invalid Wei number: 10000000000.00000000"]
    fn parse_to_eth_not_accepted_decimal() {
        to_eth("10000000000.00000000".to_string());
    }

    #[test]
    fn parse_to_decimal() {
        assert_eq!(to_eth("1000000000000000".to_string()), "0.001".to_string())
    }

    #[test]
    fn parse_to_decimal_with_zeros_in_between() {
        assert_eq!(
            to_eth("1004000000000000".to_string()),
            "0.001004".to_string()
        )
    }
}
