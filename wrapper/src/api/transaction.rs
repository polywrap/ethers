use ethers_core::{
    abi::{Abi, Token, Function},
    types::{
        transaction::eip2718::TypedTransaction, Address, Bytes, Signature,
        Transaction, TransactionReceipt, TransactionRequest, H256, U256, Eip1559TransactionRequest,
    },
    utils::{serialize},
};
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Middleware, Provider};
use ethers_signers::Signer;
use crate::block_on;

use crate::error::WrapperError;
use crate::provider::{GasWorkaround, PolywrapProvider};
use crate::signer::PolywrapSigner;
use crate::mapping::EthersTxOptions;

use crate::api::abi::{tokenize_values, encode_function};
use crate::api::get_gas_price;

pub fn sign_message(signer: &PolywrapSigner, message: &str) -> Signature {
    block_on(async {
        signer.sign_message(message).await.unwrap()
    })
}

pub fn send_rpc(provider: &Provider<PolywrapProvider>, method: &str, params: Vec<String>) -> String {
    block_on(async {
        let res: serde_json::Value = provider.request(method, params).await.unwrap();
        res
    }).to_string()
}

pub fn estimate_transaction_gas(provider: &Provider<PolywrapProvider>, tx: TypedTransaction) -> U256 {
    block_on(async {
        provider.estimate_gas(&tx, None).await.unwrap()
    })
}

pub fn get_transaction_response(provider: &Provider<PolywrapProvider>, tx_hash: H256) -> Transaction {
    block_on(async {
        provider.get_transaction(tx_hash).await.unwrap().unwrap()
    })
}

pub fn get_transaction_receipt(provider: &Provider<PolywrapProvider>, tx_hash: H256) -> TransactionReceipt {
    block_on(async {
        provider
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap()
            .unwrap()
    })
}

pub fn send_transaction(client: &SignerMiddleware<Provider<PolywrapProvider>, PolywrapSigner>, tx: &mut TypedTransaction) -> H256 {
    block_on(async {
        client.provider().fill_gas_fees(tx).await.unwrap();
        client.fill_transaction(tx, None).await.unwrap();
        let rlp = serialize(tx);
        let tx_hash: H256 = client
            .inner()
            .request("eth_sendTransaction", [rlp])
            .await
            .unwrap();
        tx_hash
    })
}

pub fn create_deploy_contract_transaction(
    abi: &Abi,
    bytecode: Bytes,
    values: &Vec<String>,
    options: &EthersTxOptions
) -> Result<TypedTransaction, WrapperError> {
    let data: Bytes = match (abi.constructor(), values.is_empty()) {
        (None, false) => {
            let error = "Constructor not found in contract ABI".to_string();
            return Err(WrapperError::ContractError(error));
        },
        (None, true) => bytecode.clone(),
        (Some(constructor), _) => {
            let tokens: Vec<Token> = tokenize_values(&values, &constructor.inputs);
            constructor.encode_input(bytecode.to_vec(), &tokens)?.into()
        },
    };
    let tx: TypedTransaction = create_transaction(None, data, options);
    Ok(tx)
}

pub fn estimate_contract_call_gas(
    provider: &Provider<PolywrapProvider>,
    address: Address,
    method: &str,
    args: &Vec<String>,
    options: &EthersTxOptions) -> U256 {
    let (_, data): (Function, Bytes) = encode_function(method, args).unwrap();
    let tx: TypedTransaction = create_transaction(Some(address), data, options);
    block_on(async {
        provider.estimate_gas(&tx, None).await.unwrap()
    })
}

pub fn call_contract_view(
    provider: &Provider<PolywrapProvider>,
    address: Address,
    method: &str,
    args: &Vec<String>
) -> Vec<Token> {
    let (function, data): (Function, Bytes) = encode_function(method, args).unwrap();

    let tx: TypedTransaction = TransactionRequest {
        to: Some(address.into()),
        data: Some(data),
        ..Default::default()
    }.into();

    let bytes: Bytes = block_on(async { provider.call(&tx, None).await.unwrap() });
    let tokens: Vec<Token> = function.decode_output(&bytes).unwrap();
    tokens
}

pub fn call_contract_static(
    client: &SignerMiddleware<Provider<PolywrapProvider>, PolywrapSigner>,
    address: Address,
    method: &str,
    args: &Vec<String>,
    options: &EthersTxOptions,
) -> Result<Vec<Token>, WrapperError> {
    let (function, data): (Function, Bytes) = encode_function(method, args)?;

    let mut tx: TypedTransaction = create_transaction(Some(address), data, options);

    let bytes: Result<Bytes, WrapperError> = block_on(async {
        client.provider().fill_gas_fees(&mut tx).await?;
        client.fill_transaction(&mut tx, None).await?;
        client.inner().call(&tx, None).await.map_err(|e| e.into())
    });

    if bytes.is_err() {
        Err(bytes.unwrap_err())
    } else {
        let tokens: Vec<Token> = function.decode_output(&bytes.unwrap())?;
        Ok(tokens)
    }
}

pub fn call_contract_method(
    client: &SignerMiddleware<Provider<PolywrapProvider>, PolywrapSigner>,
    address: Address,
    method: &str,
    args: &Vec<String>,
    options: &EthersTxOptions,
) -> H256 {
    let (_, data): (Function, Bytes) = encode_function(method, args).unwrap();
    let mut tx: TypedTransaction = create_transaction(Some(address), data, options);
    let tx_hash: H256 = send_transaction(client, &mut tx);
    tx_hash
}

fn create_transaction(address: Option<Address>, data: Bytes, options: &EthersTxOptions) -> TypedTransaction {
    if options.gas_price.is_some() {
        return TransactionRequest {
            to: address.map(Into::into),
            data: Some(data),
            gas: options.gas_limit,
            gas_price: options.gas_price,
            value: options.value,
            nonce: options.nonce,
            ..Default::default()
        }.into();
    }
    Eip1559TransactionRequest {
        to: address.map(Into::into),
        data: Some(data),
        gas: options.gas_limit,
        max_fee_per_gas: options.max_fee_per_gas,
        max_priority_fee_per_gas: options.max_priority_fee_per_gas,
        value: options.value,
        nonce: options.nonce,
        ..Default::default()
    }.into()
}