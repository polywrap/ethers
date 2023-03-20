use ethers_core::{
    types::{U256},
    utils::{parse_ether, format_units},
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
    parsed_eth.trim_end_matches("0").trim_end_matches(".").to_string()
}
