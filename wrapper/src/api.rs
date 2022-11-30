use ethers_core::{
    abi::{encode, Abi, AbiParser, HumanReadableParser, ParamType, Token},
    types::{
        transaction::eip2718::TypedTransaction, Address, BlockId, Bytes, Signature,
        Transaction, TransactionReceipt, TransactionRequest, H256, U256, Eip1559TransactionRequest,
    },
    utils::{format_ether, parse_ether, serialize},
};
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Middleware, Provider, ProviderError};
use ethers_signers::Signer;
use crate::block_on;

use crate::error::WrapperError;
use crate::format::{params_to_types, tokenize_values};
use crate::provider::{GasWorkaround, PolywrapProvider};
use crate::signer::PolywrapSigner;
use crate::mapping::EthersTxOptions;

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

pub fn sign_message(signer: &PolywrapSigner, message: &str) -> Signature {
    block_on(async {
        signer.sign_message(message).await.unwrap()
    })
}

pub fn encode_params(types: Vec<String>, values: Vec<String>) -> Vec<u8> {
    let kinds: Vec<ParamType> = types
        .iter()
        .map(|t| HumanReadableParser::parse_type(&t).unwrap())
        .collect();
    let tokens: Vec<Token> = tokenize_values(&values, &kinds);
    let bytes = encode(&tokens);
    bytes
}

pub fn encode_function(method: &str, args: Vec<String>) -> Vec<u8> {
    let function = AbiParser::default().parse_function(method).unwrap();
    let kinds: Vec<ParamType> = params_to_types(&function.inputs);
    let tokens: Vec<Token> = tokenize_values(&args, &kinds);
    let bytes = function.encode_input(&tokens).unwrap();
    bytes
}

pub fn decode_function(method: &str, data: Vec<u8>) -> Vec<Token> {
    let function = AbiParser::default().parse_function(method).unwrap();
    let sig = function.short_signature();
    let mut has_sig = false;

    if data[0..4] == sig {
        has_sig = true;
    }

    let arg_bytes: &[u8] = match has_sig {
        true => &data[4..],
        false => &data[0..]
    };

    function.decode_input(arg_bytes).unwrap()
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

pub fn send_rpc(provider: &Provider<PolywrapProvider>, method: &str, params: Vec<String>) -> String {
    block_on(async {
        let res: serde_json::Value = provider.request(method, params).await.unwrap();
        res
    }).to_string()
}

pub fn estimate_transaction_gas(provider: &Provider<PolywrapProvider>, tx: TypedTransaction) -> U256 {
    block_on(async {
        provider.estimate_gas(&tx).await.unwrap()
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

pub fn sign_and_send_transaction(client: &SignerMiddleware<Provider<PolywrapProvider>, PolywrapSigner>, tx: &mut TypedTransaction) -> H256 {
    block_on(async {
        let address = client.signer().address();
        client.provider().fill_gas_fees(tx).await.unwrap();
        client.fill_transaction(tx, None).await.unwrap();
        let signature = client.sign_transaction(&tx, address).await.unwrap();
        let signed_tx: Bytes = tx.rlp_signed(&signature);
        let rlp = serialize(&signed_tx);
        let tx_hash: H256 = client
            .inner()
            .request("eth_sendRawTransaction", [rlp])
            .await
            .unwrap();
        tx_hash
    })
}

pub fn create_deploy_contract_transaction(
    abi: Abi,
    bytecode: Bytes,
    params: &Vec<String>,
    options: &EthersTxOptions
) -> Result<TypedTransaction, WrapperError> {
    let data: Bytes = match (abi.constructor(), params.is_empty()) {
        (None, false) => {
            return Err(WrapperError::ContractError(
                ethers_contract::ContractError::ConstructorError,
            ))
        },
        (None, true) => bytecode.clone(),
        (Some(constructor), _) => {
            let kinds: Vec<ParamType> = params_to_types(&constructor.inputs);
            let tokens: Vec<Token> = tokenize_values(&params, &kinds);
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
    let function = AbiParser::default().parse_function(method).unwrap();
    let kinds: Vec<ParamType> = params_to_types(&function.inputs);
    let tokens: Vec<Token> = tokenize_values(&args, &kinds);
    let data: Bytes = function.encode_input(&tokens).map(Into::into).unwrap();
    let tx: TypedTransaction = create_transaction(Some(address), data, options);
    block_on(async {
        provider.estimate_gas(&tx).await.unwrap()
    })
}

pub fn call_contract_view(
    provider: &Provider<PolywrapProvider>,
    address: Address,
    method: &str,
    args: &Vec<String>
) -> Vec<Token> {
    let function = AbiParser::default().parse_function(method).unwrap();
    let kinds: Vec<ParamType> = params_to_types(&function.inputs);
    let tokens: Vec<Token> = tokenize_values(&args, &kinds);
    let data: Bytes = function.encode_input(&tokens).map(Into::into).unwrap();

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
    let function = AbiParser::default().parse_function(method)?;
    let kinds: Vec<ParamType> = params_to_types(&function.inputs);
    let tokens: Vec<Token> = tokenize_values(&args, &kinds);

    let data: Bytes = function.encode_input(&tokens).map(Into::into)?;

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
    let function = AbiParser::default().parse_function(method).unwrap();
    let kinds: Vec<ParamType> = params_to_types(&function.inputs);
    let tokens: Vec<Token> = tokenize_values(&args, &kinds);
    let data: Bytes = function.encode_input(&tokens).map(Into::into).unwrap();
    let mut tx: TypedTransaction = create_transaction(Some(address), data, options);
    let tx_hash: H256 = sign_and_send_transaction(client, &mut tx);
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