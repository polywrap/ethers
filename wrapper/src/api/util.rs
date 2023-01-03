use ethers_core::{
    types::{Address, BlockId, U256},
    utils::{format_ether, parse_ether},
};
use ethers_providers::{Middleware, Provider};
use crate::polywrap_provider::sync_provider::SyncProvider;
use crate::provider::{PolywrapProvider};

pub fn to_wei(eth: String) -> U256 {
    match parse_ether(eth) {
        Ok(wei) => wei,
        Err(e) => panic!("{}", e.to_string()),
    }
}

pub fn to_eth(wei: String) -> U256 {
    let wei = match U256::from_dec_str(&wei) {
        Ok(w) => w,
        Err(_) => panic!("Invalid Wei number: {}", wei),
    };
    format_ether(wei)
}
