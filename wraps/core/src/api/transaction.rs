use ethers_core::{
    abi::{Abi, Token, Function},
    types::{
        transaction::eip2718::TypedTransaction, Address, Bytes,
        TransactionRequest, H256, U256, Eip1559TransactionRequest,
    },
    utils::{serialize},
};
use ethers_core::types::{BlockId, Chain};
use ethers_provider::Provider;
use ethers_providers::ProviderError;
use crate::{error::WrapperError, polywrap_provider::{provider::WrapProvider, signer::WrapSigner}};

use crate::mapping::EthersTxOptions;

pub fn send_transaction(provider: &WrapProvider, signer: &WrapSigner, tx: &mut TypedTransaction) -> H256 {
    fill_transaction(provider, signer, tx, None).unwrap();
    let rlp = serialize(tx);
    let tx_hash: H256 = provider.request("eth_sendTransaction", [rlp]).unwrap();
    tx_hash
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
            let tokens: Vec<Token> = ethers_utils::tokenize_values(&values, &constructor.inputs);
                //TODO (cbrzn): Remove unwrap
                constructor.encode_input(bytecode.to_vec(), &tokens).unwrap().into()
        },
    };
    let tx: TypedTransaction = create_transaction(None, data, options);
    Ok(tx)
}

pub fn estimate_contract_call_gas(
    provider: &WrapProvider,
    signer: &WrapSigner,
    address: Address,
    method: &str,
    args: &Vec<String>,
    options: &EthersTxOptions) -> U256 {
    let (_, data): (Function, Bytes) = ethers_utils::encode_function(method, args).unwrap();
    let mut tx: TypedTransaction = create_transaction(Some(address), data, options);
    fill_transaction(provider, signer, &mut tx, None).unwrap();
    if let Some(gas_limit) = tx.as_eip1559_ref().unwrap().gas {
        return gas_limit;
    }
    provider.estimate_gas(&tx, None).unwrap()
}

pub fn call_contract_view(
    provider: &WrapProvider,
    address: Address,
    method: &str,
    args: &Vec<String>
) -> Vec<Token> {
    let (function, data): (Function, Bytes) = ethers_utils::encode_function(method, args).unwrap();

    let tx: TypedTransaction = TransactionRequest {
        to: Some(address.into()),
        data: Some(data),
        ..Default::default()
    }.into();

    let bytes: Bytes = provider.call(&tx, None).unwrap();

    let tokens: Vec<Token> = function.decode_output(&bytes).unwrap();

    tokens
}

pub fn call_contract_static(
    provider: &WrapProvider,
    signer: &WrapSigner,
    address: Address,
    method: &str,
    args: &Vec<String>,
    options: &EthersTxOptions,
) -> Result<Vec<Token>, WrapperError> {
    let (function, data): (Function, Bytes) = ethers_utils::encode_function(method, args)?;

    let mut tx: TypedTransaction = create_transaction(Some(address), data, options);
    fill_transaction(provider, signer, &mut tx, None)?;
    let bytes: Result<Bytes, WrapperError> = provider.call(&tx, None).map_err(|e| e.into());

    if bytes.is_err() {
        Err(bytes.unwrap_err())
    } else {
        //TODO (cbrzn): Remove unwrap
        let tokens: Vec<Token> = function.decode_output(&bytes.unwrap()).unwrap();
        Ok(tokens)
    }
}

pub fn call_contract_method(
    provider: &WrapProvider,
    signer: &WrapSigner,
    address: Address,
    method: &str,
    args: &Vec<String>,
    options: &EthersTxOptions,
) -> H256 {
    let (_, encode_data): (Function, Bytes) = ethers_utils::encode_function(method, args).unwrap();

    let mut tx: TypedTransaction = create_transaction(
        Some(address),
        Bytes::from(encode_data),
        options
    );
    let tx_hash: H256 = send_transaction(provider, signer, &mut tx);
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

/// Helper for filling a transaction's nonce using the wallet
fn fill_transaction(
    provider: &WrapProvider,
    signer: &WrapSigner,
    tx: &mut TypedTransaction,
    block: Option<BlockId>,
) -> Result<(), ProviderError> {
    // get the `from` field's nonce if it's set, else get the signer's nonce
    let from = if tx.from().is_some() && tx.from() != Some(&signer.address()) {
        *tx.from().unwrap()
    } else {
        signer.address()
    };
    tx.set_from(from);

    // get the signer's chain_id if the transaction does not set it
    let chain_id = signer.chain_id();
    if tx.chain_id().is_none() {
        tx.set_chain_id(chain_id);
    }

    // If a chain_id is matched to a known chain that doesn't support EIP-1559, automatically
    // change transaction to be Legacy type.
    if let Some(chain_id) = tx.chain_id() {
        let chain = Chain::try_from(chain_id.as_u64());
        if chain.unwrap_or_default().is_legacy() {
            if let TypedTransaction::Eip1559(inner) = tx {
                let tx_req: TransactionRequest = inner.clone().into();
                *tx = TypedTransaction::Legacy(tx_req);
            }
        }
    }

    provider
        .fill_transaction(tx, block)?;
    Ok(())
}