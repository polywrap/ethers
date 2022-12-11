use ethers_core::{
    types::{Address, BlockId, U256},
    utils::{format_ether, parse_ether},
};
use ethers_providers::{Middleware, Provider};
use crate::block_on;
use crate::provider::{GasWorkaround, PolywrapProvider};


pub fn get_chain_id(provider: &Provider<PolywrapProvider>) -> U256 {
    block_on(async {
        provider.get_chainid().await.unwrap()
    })
}
pub fn get_balance(provider: &Provider<PolywrapProvider>, address: Address, block_tag: BlockId) -> U256 {
    block_on(async {
        provider
            .get_balance(address, Some(block_tag))
            .await
            .unwrap()
    })
}

pub fn get_gas_price(provider: &Provider<PolywrapProvider>) -> U256 {
    block_on(async {
        provider.get_gas_price().await.unwrap()
    })
}

pub fn estimate_eip1559_fees(provider: &Provider<PolywrapProvider>) -> (U256, U256) {
    block_on(async {
        provider.estimate_eip1559_fees(None).await.unwrap()
    })
}

pub fn get_signer_balance(provider: &Provider<PolywrapProvider>, signer_address: Address, block_tag: BlockId) -> U256 {
    block_on(async {
        provider
            .get_balance(signer_address, Some(block_tag))
            .await
            .unwrap()
    })
}

pub fn get_signer_transaction_count(provider: &Provider<PolywrapProvider>, signer_address: Address, block_tag: BlockId) -> U256 {
    block_on(async {
        provider
            .get_transaction_count(signer_address, Some(block_tag))
            .await
            .unwrap()
    })
}

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