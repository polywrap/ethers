use futures::executor::block_on;

use ethers_core::types::{Address, BlockId, BlockNumber, Bytes, H256};
use ethers_core::abi::{Function, HumanReadableParser};
use polywrap_wasm_rs::BigInt;
use std::str::FromStr;
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Middleware, Provider};
use ethers_signers::Signer;

mod wrap;
use wrap::*;
use crate::provider::{GasWorkaround, PolywrapProvider};
use crate::signer::PolywrapSigner;

mod api;
mod polywrap_provider;
mod helpers;
use polywrap_provider::{iprovider, provider, signer, error};
use helpers::{format, mapping};

pub fn get_chain_id(args: wrap::ArgsGetChainId) -> String {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    api::get_chain_id(&provider).to_string()
}

pub fn get_balance(args: wrap::ArgsGetBalance) -> BigInt {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid address: {}. Error: {}", &args.address, e),
    };
    let block_tag: BlockId = BlockNumber::Latest.into();
    let balance = api::get_balance(&provider, address, block_tag);
    BigInt::from_str(&balance.to_string()).unwrap()
}

pub fn check_address(args: wrap::ArgsCheckAddress) -> bool {
    match Address::from_str(&args.address) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_gas_price(args: wrap::ArgsGetGasPrice) -> BigInt {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let price = api::get_gas_price(&provider);
    BigInt::from_str(&price.to_string()).unwrap()
}

pub fn estimate_eip1559_fees(args: wrap::ArgsEstimateEip1559Fees) -> wrap::Eip1559FeesEstimate {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let price = api::estimate_eip1559_fees(&provider);
    wrap::Eip1559FeesEstimate {
        max_fee_per_gas: BigInt::from_str(&price.0.to_string()).unwrap(),
        max_priority_fee_per_gas: BigInt::from_str(&price.1.to_string()).unwrap(),
    }
}

pub fn get_signer_address(args: wrap::ArgsGetSignerAddress) -> String {
    let address = PolywrapSigner::new(&args.connection).address();
    format!("{}", address)
}

pub fn get_signer_balance(args: wrap::ArgsGetSignerBalance) -> BigInt {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let address = PolywrapSigner::new(&args.connection).address();
    let block_tag: BlockId = BlockNumber::Latest.into();
    let balance = api::get_signer_balance(&provider, address, block_tag);
    BigInt::from_str(&balance.to_string()).unwrap()
}

pub fn get_signer_transaction_count(args: wrap::ArgsGetSignerTransactionCount) -> BigInt {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let address = PolywrapSigner::new(&args.connection).address();
    let block_tag: BlockId = BlockNumber::Latest.into();
    let count = api::get_signer_transaction_count(&provider, address, block_tag);
    BigInt::from_str(&count.to_string()).unwrap()

}

pub fn sign_message(args: wrap::ArgsSignMessage) -> String {
    let signer = PolywrapSigner::new(&args.connection);
    let signature = api::sign_message(&signer, &args.message);
    let bytes: Bytes = signature.to_vec().into();
    format!("{}", bytes).to_string()
}

pub fn encode_params(input: wrap::ArgsEncodeParams) -> String {
    let bytes: Bytes = api::encode_params(input.types, input.values).into();
    format!("{}", bytes).to_string()
}

pub fn encode_function(input: wrap::ArgsEncodeFunction) -> String {
    let args: Vec<String> = input.args.unwrap_or(vec![]);
    let (_, bytes): (Function, Bytes) = api::encode_function(&input.method, &args).unwrap();
    format!("{}", bytes).to_string()
}

pub fn decode_function(input: wrap::ArgsDecodeFunction) -> Vec<String> {
    let data = Bytes::from_str(&input.data).unwrap().to_vec();
    let tokens = api::decode_function(&input.method, data);
    tokens
        .iter()
        .map(|t| format::format_token(t))
        .collect()
}

pub fn to_wei(input: ArgsToWei) -> String {
    api::to_wei(input.eth).to_string()
}

pub fn to_eth(input: ArgsToEth) -> String {
    api::to_eth(input.wei).to_string()
}

pub fn send_rpc(args: wrap::ArgsSendRpc) -> String {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let response = api::send_rpc(&provider, &args.method, args.params);
    response
}

pub fn estimate_transaction_gas(args: wrap::ArgsEstimateTransactionGas) -> BigInt {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let tx = mapping::from_wrap_request(args.tx);
    let gas = api::estimate_transaction_gas(&provider, tx);
    BigInt::from_str(&gas.to_string()).unwrap()
}

pub fn await_transaction(args: wrap::ArgsAwaitTransaction) -> wrap::TxReceipt {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let tx_hash = H256::from_str(&args.tx_hash).unwrap();
    let receipt = api::get_transaction_receipt(&provider, tx_hash);
    let tx_receipt = mapping::to_wrap_receipt(receipt);
    tx_receipt
}

pub fn send_transaction(args: wrap::ArgsSendTransaction) -> wrap::TxResponse {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let signer = PolywrapSigner::new(&args.connection);
    let client = SignerMiddleware::new(provider, signer);

    let mut tx = mapping::from_wrap_request(args.tx);

    let tx_hash = api::sign_and_send_transaction(&client, &mut tx);
    let response = api::get_transaction_response(client.provider(), tx_hash);
    let tx_response = mapping::to_wrap_response(client.provider(), response);
    tx_response
}

pub fn send_transaction_and_wait(args: wrap::ArgsSendTransactionAndWait) -> wrap::TxReceipt {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let signer = PolywrapSigner::new(&args.connection);
    let client = SignerMiddleware::new(provider, signer);

    let mut tx = mapping::from_wrap_request(args.tx);

    let tx_hash = api::sign_and_send_transaction(&client, &mut tx);
    let receipt = api::get_transaction_receipt(client.provider(), tx_hash);
    let tx_receipt = mapping::to_wrap_receipt(receipt);
    tx_receipt
}

pub fn deploy_contract(args: wrap::ArgsDeployContract) -> String {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let signer = PolywrapSigner::new(&args.connection);
    let client = SignerMiddleware::new(provider, signer);

    let abi = serde_json::from_str(&args.abi).unwrap();
    let bytecode = Bytes::from_str(&args.bytecode).unwrap();
    let params: Vec<String> = args.args.unwrap_or(vec![]);
    let mut tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

    // todo: implement custom gas price and fee estimation to work around wasm bindgen crashes
    if tx_options.max_fee_per_gas.is_none() && tx_options.max_priority_fee_per_gas.is_none() && tx_options.gas_price.is_none() {
        tx_options.gas_price = Some(api::get_gas_price(client.provider()));
    }

    let mut tx = api::create_deploy_contract_transaction(abi, bytecode, &params, &tx_options).unwrap();

    let tx_hash = api::sign_and_send_transaction(&client, &mut tx);
    let receipt = api::get_transaction_receipt(client.provider(), tx_hash);
    let address = receipt.contract_address.expect("Contract failed to deploy.");
    format!("{:#x}", address)
}

pub fn estimate_contract_call_gas(args: wrap::ArgsEstimateContractCallGas) -> BigInt {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));

    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
    };
    let params: Vec<String> = args.args.unwrap_or(vec![]);
    let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

    let gas = api::estimate_contract_call_gas(&provider, address, &args.method, &params, &tx_options);
    BigInt::from_str(&gas.to_string()).unwrap()
}

pub fn call_contract_view(args: wrap::ArgsCallContractView) -> String {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));

    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
    };
    let params: Vec<String> = args.args.unwrap_or(vec![]);

    let tokens = api::call_contract_view(&provider, address, &args.method, &params);
    format::format_tokens(&tokens)
}

pub fn call_contract_static(args: ArgsCallContractStatic) -> wrap::StaticTxResult {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let signer = PolywrapSigner::new(&args.connection);
    let client = SignerMiddleware::new(provider, signer);

    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
    };
    let params: Vec<String> = args.args.unwrap_or(vec![]);
    let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

    let result = api::call_contract_static(&client, address, &args.method, &params, &tx_options);
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
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let signer = PolywrapSigner::new(&args.connection);
    let client = SignerMiddleware::new(provider, signer);

    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
    };
    let params: Vec<String> = args.args.unwrap_or(vec![]);
    let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

    let tx_hash = api::call_contract_method(&client, address, &args.method, &params, &tx_options);

    let response = api::get_transaction_response(client.provider(), tx_hash);
    let tx_response = mapping::to_wrap_response(client.provider(), response);
    tx_response
}

pub fn call_contract_method_and_wait(
    args: wrap::ArgsCallContractMethodAndWait,
) -> wrap::TxReceipt {
    let provider = Provider::new(PolywrapProvider::new(&args.connection));
    let signer = PolywrapSigner::new(&args.connection);
    let client = SignerMiddleware::new(provider, signer);

    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
    };
    let params: Vec<String> = args.args.unwrap_or(vec![]);
    let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

    let tx_hash = api::call_contract_method(&client, address, &args.method, &params, &tx_options);
    let receipt = api::get_transaction_receipt(client.provider(), tx_hash);
    let tx_receipt = mapping::to_wrap_receipt(receipt);
    tx_receipt
}
