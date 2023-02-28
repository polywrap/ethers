use ethers_core::types::{Address, BlockId, BlockNumber, Bytes, H256, U256};
use ethers_core::abi::{encode_packed, FixedBytes, Abi, Function, Token, encode};
use ethers_core::utils::{keccak256, get_create2_address};
use polywrap_wasm_rs::{BigInt,JSON};
use std::str::FromStr;

mod wrap;
use wrap::*;
use crate::provider::{PolywrapProvider};
use crate::signer::PolywrapSigner;

mod api;
mod polywrap_provider;
mod helpers;
use polywrap_provider::{iprovider, provider, signer, error, sync_provider::SyncProvider};
use helpers::{format, mapping};
use crate::polywrap_provider::sync_signer::SyncSigner;

pub fn get_chain_id(args: wrap::ArgsGetChainId) -> String {
    let provider = PolywrapProvider::new(&args.connection);
    provider.get_chainid_sync().unwrap().to_string()
}

pub fn get_balance(args: wrap::ArgsGetBalance) -> BigInt {
    let provider = PolywrapProvider::new(&args.connection);
    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid address: {}. Error: {}", &args.address, e),
    };
    let block_tag: BlockId = BlockNumber::Latest.into();
    let balance = provider.get_balance_sync(address, Some(block_tag)).unwrap();
    BigInt::from_str(&balance.to_string()).unwrap()
}

pub fn check_address(args: wrap::ArgsCheckAddress) -> bool {
    match Address::from_str(&args.address) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_gas_price(args: wrap::ArgsGetGasPrice) -> BigInt {
    let provider = PolywrapProvider::new(&args.connection);
    let price = provider.get_gas_price_sync().unwrap();
    BigInt::from_str(&price.to_string()).unwrap()
}

pub fn estimate_eip1559_fees(args: wrap::ArgsEstimateEip1559Fees) -> wrap::Eip1559FeesEstimate {
    let provider = PolywrapProvider::new(&args.connection);
    let price = provider.estimate_eip1559_fees_sync(None).unwrap();
    wrap::Eip1559FeesEstimate {
        max_fee_per_gas: BigInt::from_str(&price.0.to_string()).unwrap(),
        max_priority_fee_per_gas: BigInt::from_str(&price.1.to_string()).unwrap(),
    }
}

pub fn get_signer_address(args: wrap::ArgsGetSignerAddress) -> String {
    let address = PolywrapSigner::new(&args.connection).address();
    format!("{:#x}", address)
}

pub fn get_signer_balance(args: wrap::ArgsGetSignerBalance) -> BigInt {
    let provider = PolywrapProvider::new(&args.connection);
    let address = PolywrapSigner::new(&args.connection).address();
    let block_tag: BlockId = BlockNumber::Latest.into();
    let balance = provider.get_balance_sync(address, Some(block_tag)).unwrap();
    BigInt::from_str(&balance.to_string()).unwrap()
}

pub fn get_signer_transaction_count(args: wrap::ArgsGetSignerTransactionCount) -> BigInt {
    let provider = PolywrapProvider::new(&args.connection);
    let address = PolywrapSigner::new(&args.connection).address();
    let block_tag: BlockId = BlockNumber::Latest.into();
    let count = provider.get_transaction_count_sync(address, Some(block_tag)).unwrap();
    BigInt::from_str(&count.to_string()).unwrap()
}

pub fn sign_message(args: wrap::ArgsSignMessage) -> String {
    let signer = PolywrapSigner::new(&args.connection);
    let signature = signer.sign_message_sync(&args.message).unwrap();
    let bytes: Bytes = signature.to_vec().into();
    format!("{}", bytes).to_string()
}

pub fn sign_message_bytes(args: wrap::ArgsSignMessageBytes) -> String {
    let signer = PolywrapSigner::new(&args.connection);
    let signature = signer.sign_message_sync(&args.bytes).unwrap();
    let bytes: Bytes = signature.to_vec().into();
    format!("{}", bytes).to_string()
}

pub fn sign_transaction(args: wrap::ArgsSignTransaction) -> String {
    let signer = PolywrapSigner::new(&args.connection);
    let tx = mapping::from_wrap_request(args.tx);
    let signature = signer.sign_transaction_sync(&tx).unwrap();
    let bytes: Bytes = signature.to_vec().into();
    format!("{}", bytes).to_string()
}

pub fn sign_typed_data(args: wrap::ArgsSignTypedData) -> String {
    let address = PolywrapSigner::new(&args.connection).address();
    let address_value = JSON::Value::String(format!("{:#x}", address));
    let params = JSON::Value::Array(vec![address_value, args.payload]);
    let provider = PolywrapProvider::new(&args.connection);
    provider.request_sync("eth_signTypedData", params).unwrap()
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
    let provider = PolywrapProvider::new(&args.connection);
    let res: serde_json::Value = provider.request_sync(&args.method, args.params).unwrap();
    res.to_string()
}

pub fn estimate_transaction_gas(args: wrap::ArgsEstimateTransactionGas) -> BigInt {
    let provider = PolywrapProvider::new(&args.connection);
    let tx = mapping::from_wrap_request(args.tx);
    let gas = provider.estimate_gas_sync(&tx, None).unwrap();
    BigInt::from_str(&gas.to_string()).unwrap()
}

pub fn await_transaction(args: wrap::ArgsAwaitTransaction) -> wrap::TxReceipt {
    let provider = PolywrapProvider::new(&args.connection);
    let tx_hash = H256::from_str(&args.tx_hash).unwrap();
    provider.await_transaction_sync(tx_hash.clone(), args.confirmations, args.timeout).unwrap();
    let receipt = provider.get_transaction_receipt_sync(tx_hash).unwrap().unwrap();
    let tx_receipt = mapping::to_wrap_receipt(receipt, args.confirmations);
    tx_receipt
}

pub fn send_transaction(args: wrap::ArgsSendTransaction) -> wrap::TxResponse {
    let provider = PolywrapProvider::new(&args.connection);
    let signer = PolywrapSigner::new(&args.connection);

    let mut tx = mapping::from_wrap_request(args.tx);

    let tx_hash = api::send_transaction(&provider, &signer, &mut tx);
    let response = provider.get_transaction_sync(tx_hash).unwrap().unwrap();
    let tx_response = mapping::to_wrap_response(&provider, response);
    tx_response
}

pub fn send_transaction_and_wait(args: wrap::ArgsSendTransactionAndWait) -> wrap::TxReceipt {
    let provider = PolywrapProvider::new(&args.connection);
    let signer = PolywrapSigner::new(&args.connection);

    let mut tx = mapping::from_wrap_request(args.tx);

    let tx_hash = api::send_transaction(&provider, &signer, &mut tx);
    provider.await_transaction_sync(tx_hash.clone(), 1, None).unwrap();
    let receipt = provider.get_transaction_receipt_sync(tx_hash).unwrap().unwrap();
    let tx_receipt = mapping::to_wrap_receipt(receipt, 1);
    tx_receipt
}

pub fn deploy_contract(args: wrap::ArgsDeployContract) -> String {
    let provider = PolywrapProvider::new(&args.connection);
    let signer = PolywrapSigner::new(&args.connection);

    let abi: Abi = serde_json::from_str(&args.abi).unwrap();
    let bytecode = Bytes::from_str(&args.bytecode).unwrap();
    let params: Vec<String> = args.args.unwrap_or(vec![]);
    let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

    let mut tx = api::create_deploy_contract_transaction(&abi, bytecode, &params, &tx_options).unwrap();

    let tx_hash = api::send_transaction(&provider, &signer, &mut tx);
    provider.await_transaction_sync(tx_hash.clone(), 1, None).unwrap();
    let receipt = provider.get_transaction_receipt_sync(tx_hash).unwrap().unwrap();
    let address = receipt.contract_address.expect("Contract failed to deploy.");
    format!("{:#x}", address)
}

pub fn estimate_contract_call_gas(args: wrap::ArgsEstimateContractCallGas) -> BigInt {
    let provider = PolywrapProvider::new(&args.connection);

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
    let provider = PolywrapProvider::new(&args.connection);

    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
    };
    let params: Vec<String> = args.args.unwrap_or(vec![]);

    let tokens = api::call_contract_view(&provider, address, &args.method, &params);
    format::format_tokens(&tokens)
}

pub fn call_contract_static(args: ArgsCallContractStatic) -> wrap::StaticTxResult {
    let provider = PolywrapProvider::new(&args.connection);
    let signer = PolywrapSigner::new(&args.connection);

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
    let provider = PolywrapProvider::new(&args.connection);
    let signer = PolywrapSigner::new(&args.connection);

    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
    };
    let params: Vec<String> = args.args.unwrap_or(vec![]);
    let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

    let tx_hash = api::call_contract_method(&provider, &signer, address, &args.method, &params, &tx_options);

    let response = provider.get_transaction_sync(tx_hash).unwrap().unwrap();
    let tx_response = mapping::to_wrap_response(&provider, response);
    tx_response
}

pub fn call_contract_method_and_wait(
    args: wrap::ArgsCallContractMethodAndWait,
) -> wrap::TxReceipt {
    let provider = PolywrapProvider::new(&args.connection);
    let signer = PolywrapSigner::new(&args.connection);

    let address = match Address::from_str(&args.address) {
        Ok(addr) => addr,
        Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
    };
    let params: Vec<String> = args.args.unwrap_or(vec![]);
    let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

    let tx_hash = api::call_contract_method(&provider, &signer, address, &args.method, &params, &tx_options);
    provider.await_transaction_sync(tx_hash.clone(), 1, None).unwrap();
    let receipt = provider.get_transaction_receipt_sync(tx_hash).unwrap().unwrap();
    let tx_receipt = mapping::to_wrap_receipt(receipt, 1);
    tx_receipt
}

pub fn solidity_keccak256_bytes(args: wrap::ArgsSolidityKeccak256Bytes) -> String {
    let value = Token::Bytes(args.bytes.as_bytes().to_vec());
    let hash = hex::encode(keccak256(encode(&[value])));

    hash
}

pub fn encode_bytes_value(args: wrap::ArgsEncodeBytesValue) -> String {
    let mut bytes: Vec<u8> = Vec::with_capacity(args.value.len());
    bytes.extend_from_slice(args.value.as_bytes());
    format!("{}", Bytes::from(bytes)).to_string()
}


pub fn w_keccak256(args: wrap::ArgsWKeccak256) -> String {
    let decoded = Bytes::from_str(&args.bytes).unwrap();
    let hash = keccak256(decoded);
    format!("{}", Bytes::from(hash)).to_string()
}

pub fn keccak256_encode_bytes(args: wrap::ArgsKeccak256EncodeBytes) -> String {
    let bytes = Bytes::from_str(&args.bytes).unwrap();
    let bytes = Token::Bytes(bytes.to_vec());
    let encoded = keccak256(encode_packed(&[bytes]).unwrap());
    format!("{}", Bytes::from(encoded)).to_string()
}

// TODO(cbrzn): This does not works yet - Trying to replicate
// `abi.encode()` from https://github.com/safe-global/safe-contracts/blob/main/contracts/proxies/SafeProxyFactory.sol#L54
// in the meantime, the function above works in the safe wrapper
// pub fn w_encode_packed(args: wrap::ArgsWEncodePacked) -> String {
//     let bytes = Bytes::from_str(&args.bytes).unwrap();
//     let bytes_as_fixed_array: [u8; 32] = bytes.to_vec().try_into().unwrap();
//     let fixed_bytes = Token::FixedBytes(FixedBytes::from(bytes_as_fixed_array));

//     let uint = Token::Uint(args.uint.parse::<U256>().unwrap());

//     let encoded = encode_packed(&[fixed_bytes, uint]).unwrap();
    
//     format!("{}", Bytes::from(keccak256(encoded))).to_string()
// }

pub fn generate_create2_address(
    args: wrap::ArgsGenerateCreate2Address,
) -> String {
    let salt = Bytes::from_str(&args.salt).unwrap();
    let init_code = Bytes::from_str(&args.init_code).unwrap();
    let address = args.address.parse::<Address>().unwrap();
    let generated_address = get_create2_address(
        address,
        salt,
        init_code
    );

    format!("{:?}", generated_address)
}
