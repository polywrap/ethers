use ethers_core::types::{Address, BlockId, BlockNumber, Bytes, H256};
use ethers_core::abi::{Abi};
use polywrap_provider::provider::WrapProvider;
use polywrap_provider::signer::WrapSigner;
use polywrap_wasm_rs::{BigInt,JSON};
use std::str::FromStr;
use ethers_provider::{Provider, Signer};
mod wrap;
use wrap::*;

mod api;
mod polywrap_provider;
mod helpers;
use polywrap_provider::{provider, error};
use helpers::{format, mapping};


pub fn get_chain_id(args: wrap::ArgsGetChainId) -> String {
    let provider = WrapProvider::new(&args.connection);
    provider.get_chainid().unwrap().to_string()
}

pub fn get_balance(args: wrap::ArgsGetBalance) -> BigInt {
    let provider = WrapProvider::new(&args.connection);
    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid address: {}. Error: {}", &args.address, e),
    };
    let block_tag: BlockId = BlockNumber::Latest.into();
    let balance = provider.get_balance(address, Some(block_tag)).unwrap();
    BigInt::from_str(&balance.to_string()).unwrap()
}

pub fn check_address(args: wrap::ArgsCheckAddress) -> bool {
    match Address::from_str(&args.address) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_gas_price(args: wrap::ArgsGetGasPrice) -> BigInt {
    let provider = WrapProvider::new(&args.connection);
    let price = provider.get_gas_price().unwrap();
    BigInt::from_str(&price.to_string()).unwrap()
}

pub fn estimate_eip1559_fees(args: wrap::ArgsEstimateEip1559Fees) -> wrap::Eip1559FeesEstimate {
    let provider = WrapProvider::new(&args.connection);
    let price = provider.estimate_eip1559_fees(None).unwrap();
    wrap::Eip1559FeesEstimate {
        max_fee_per_gas: BigInt::from_str(&price.0.to_string()).unwrap(),
        max_priority_fee_per_gas: BigInt::from_str(&price.1.to_string()).unwrap(),
    }
}

pub fn get_signer_address(args: wrap::ArgsGetSignerAddress) -> String {
    let address = WrapSigner::new(&args.connection).address();
    format!("{:#x}", address).to_string().to_lowercase()
}

pub fn get_signer_balance(args: wrap::ArgsGetSignerBalance) -> BigInt {
    let provider = WrapProvider::new(&args.connection);
    let address = WrapSigner::new(&args.connection).address();
    let block_tag: BlockId = BlockNumber::Latest.into();
    let balance = provider.get_balance(address, Some(block_tag)).unwrap();
    BigInt::from_str(&balance.to_string()).unwrap()
}

pub fn get_signer_transaction_count(args: wrap::ArgsGetSignerTransactionCount) -> BigInt {
    let provider = WrapProvider::new(&args.connection);
    let address = WrapSigner::new(&args.connection).address();
    let block_tag: BlockId = BlockNumber::Latest.into();
    let count = provider.get_transaction_count(address, Some(block_tag)).unwrap();
    BigInt::from_str(&count.to_string()).unwrap()
}

pub fn sign_message(args: wrap::ArgsSignMessage) -> String {
    let signer = WrapSigner::new(&args.connection);
    let signature = signer.sign_message(&args.message).unwrap();
    let bytes: Bytes = signature.to_vec().into();
    format!("{}", bytes).to_string()
}

pub fn sign_message_bytes(args: wrap::ArgsSignMessageBytes) -> String {
    let signer = WrapSigner::new(&args.connection);
    let signature = signer.sign_message(&args.bytes).unwrap();
    let bytes: Bytes = signature.to_vec().into();
    format!("{}", bytes).to_string()
}

pub fn sign_transaction(args: wrap::ArgsSignTransaction) -> String {
    let signer = WrapSigner::new(&args.connection);
    let tx = mapping::from_wrap_request(args.tx);
    let signature = signer.sign_transaction(&tx).unwrap();
    let bytes: Bytes = signature.to_vec().into();
    format!("{}", bytes).to_string()
}

pub fn sign_typed_data(args: wrap::ArgsSignTypedData) -> String {
    let address = WrapSigner::new(&args.connection).address();
    let address_value = JSON::Value::String(format!("{:#x}", address));
    let params = JSON::Value::Array(vec![address_value, args.payload]);
    let provider = WrapProvider::new(&args.connection);
    provider.request("eth_signTypedData", params).unwrap()
}

pub fn to_wei(input: ArgsToWei) -> String {
    ethers_utils::to_wei(input.eth).to_string()
}

pub fn to_eth(input: ArgsToEth) -> String {
    ethers_utils::to_eth(input.wei).to_string()
}

pub fn send_rpc(args: wrap::ArgsSendRpc) -> String {
    let provider = WrapProvider::new(&args.connection);
    let res: serde_json::Value = provider.request(&args.method, args.params).unwrap();
    res.to_string()
}

pub fn estimate_transaction_gas(args: wrap::ArgsEstimateTransactionGas) -> BigInt {
    let provider = WrapProvider::new(&args.connection);
    let tx = mapping::from_wrap_request(args.tx);
    let gas = provider.estimate_gas(&tx, None).unwrap();
    BigInt::from_str(&gas.to_string()).unwrap()
}

pub fn await_transaction(args: wrap::ArgsAwaitTransaction) -> wrap::TxReceipt {
    let provider = WrapProvider::new(&args.connection);
    let tx_hash = H256::from_str(&args.tx_hash).unwrap();
    provider.await_transaction(tx_hash.clone(), args.confirmations, args.timeout).unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).unwrap().unwrap();
    let tx_receipt = mapping::to_wrap_receipt(receipt, args.confirmations);
    tx_receipt
}

pub fn send_transaction(args: wrap::ArgsSendTransaction) -> wrap::TxResponse {
    let provider = WrapProvider::new(&args.connection);
    let signer = WrapSigner::new(&args.connection);

    let mut tx = mapping::from_wrap_request(args.tx);

    let tx_hash = api::send_transaction(&provider, &signer, &mut tx);
    let response = provider.get_transaction(tx_hash).unwrap().unwrap();
    let tx_response = mapping::to_wrap_response(&provider, response);
    tx_response
}

pub fn send_transaction_and_wait(args: wrap::ArgsSendTransactionAndWait) -> wrap::TxReceipt {
    let provider = WrapProvider::new(&args.connection);
    let signer = WrapSigner::new(&args.connection);

    let mut tx = mapping::from_wrap_request(args.tx);

    let tx_hash = api::send_transaction(&provider, &signer, &mut tx);
    provider.await_transaction(tx_hash.clone(), 1, None).unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).unwrap().unwrap();
    let tx_receipt = mapping::to_wrap_receipt(receipt, 1);
    tx_receipt
}

pub fn deploy_contract(args: wrap::ArgsDeployContract) -> String {
    let provider = WrapProvider::new(&args.connection);
    let signer = WrapSigner::new(&args.connection);

    let abi: Abi = serde_json::from_str(&args.abi).unwrap();
    let bytecode = Bytes::from_str(&args.bytecode).unwrap();
    let params: Vec<String> = args.args.unwrap_or(vec![]);
    let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

    let mut tx = api::create_deploy_contract_transaction(&abi, bytecode, &params, &tx_options).unwrap();

    let tx_hash = api::send_transaction(&provider, &signer, &mut tx);
    provider.await_transaction(tx_hash.clone(), 1, None).unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).unwrap().unwrap();
    let address = receipt.contract_address.expect("Contract failed to deploy.");
    format!("{:#x}", address)
}

pub fn estimate_contract_call_gas(args: wrap::ArgsEstimateContractCallGas) -> BigInt {
    let provider = WrapProvider::new(&args.connection);
    let signer = WrapSigner::new(&args.connection);

    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
    };
    let params: Vec<String> = args.args.unwrap_or(vec![]);
    let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

    let gas = api::estimate_contract_call_gas(&provider, &signer, address, &args.method, &params, &tx_options);
    BigInt::from_str(&gas.to_string()).unwrap()
}

pub fn call_contract_view(args: wrap::ArgsCallContractView) -> String {
    let provider = WrapProvider::new(&args.connection);

    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
    };
    let params: Vec<String> = args.args.unwrap_or(vec![]);

    let tokens = api::call_contract_view(&provider, address, &args.method, &params);
    format::format_tokens(&tokens)
}

pub fn call_contract_static(args: ArgsCallContractStatic) -> wrap::StaticTxResult {
    let provider = WrapProvider::new(&args.connection);
    let signer = WrapSigner::new(&args.connection);

    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
    };
    let params: Vec<String> = args.args.unwrap_or(vec![]);
    let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

    let result = api::call_contract_static(&provider, &signer, address, &args.method, &params, &tx_options);
    match result {
        Ok(tokens) => wrap::StaticTxResult {
            result: format::format_tokens(&tokens),
            error: false,
        },
        Err(e) => wrap::StaticTxResult {
            result: e.to_string(),
            error: true,
        },
    }
}

pub fn call_contract_method(args: wrap::ArgsCallContractMethod) -> wrap::TxResponse {
    let provider = WrapProvider::new(&args.connection);
    let signer = WrapSigner::new(&args.connection);

    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
    };
    let params: Vec<String> = args.args.unwrap_or(vec![]);
    let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

    let tx_hash = api::call_contract_method(&provider, &signer, address, &args.method, &params, &tx_options);

    let response = provider.get_transaction(tx_hash).unwrap().unwrap();
    let tx_response = mapping::to_wrap_response(&provider, response);
    tx_response
}

pub fn call_contract_method_and_wait(
    args: wrap::ArgsCallContractMethodAndWait,
) -> wrap::TxReceipt {
    let provider = WrapProvider::new(&args.connection);
    let signer = WrapSigner::new(&args.connection);

    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
    };
    let params: Vec<String> = args.args.unwrap_or(vec![]);
    let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

    let tx_hash = api::call_contract_method(&provider, &signer, address, &args.method, &params, &tx_options);
    provider.await_transaction(tx_hash.clone(), 1, None).unwrap();
    let receipt = provider.get_transaction_receipt(tx_hash).unwrap().unwrap();
    let tx_receipt = mapping::to_wrap_receipt(receipt, 1);
    tx_receipt
}