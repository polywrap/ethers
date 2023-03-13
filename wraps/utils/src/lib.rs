use ethers_core::abi::{encode_packed as encode_packed_tokens, Token, Function, AbiEncode};
use ethers_core::utils::{keccak256 as keccak256_ethers, get_create2_address};
use ethers_core::types::{Bytes, Address};
use ethers_utils::{
    encode_params as utils_encode_params,
    encode_function as utils_encode_function,
    solidity_pack as utils_solidity_pack,
};
use polywrap_wasm_rs::{BigInt};
use std::str::FromStr;

mod wrap;
use wrap::*;

pub fn keccak256(args: wrap::ArgsKeccak256) -> String {
    let decoded = Bytes::from_str(&args.value).unwrap();
    let hash = keccak256_ethers(decoded);
    format!("{}", Bytes::from(hash))
}

pub fn keccak256_bytes_encode_packed(args: wrap::ArgsKeccak256BytesEncodePacked) -> String {
    let bytes = Bytes::from_str(&args.value).unwrap();
    let token = Token::Bytes(bytes.to_vec());
    let encoded = keccak256_ethers(encode_packed_tokens(&[token]).unwrap());
    format!("{}", Bytes::from(encoded))
}

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

pub fn encode_meta_transaction(args: wrap::ArgsEncodeMetaTransaction) -> String {
    let mut op_bytes: [u8; 1] = [0];

    if let Some(op) = args.operation {
        if BigInt::from(1) == op {
            op_bytes[0] = 1;
        }
    }

    let operation = Token::FixedBytes(op_bytes.into());
    let to = args.to.parse::<Address>().unwrap();

    let value = utils_encode_params(
        vec!["uint256".into()],
        vec![args.value.to_string()]
    );

    let data = Bytes::from_str(&args.data).unwrap();
    let data_len = utils_encode_params(
        vec!["uint256".into()],
        vec![data.len().to_string()]
    );

    let encoded = encode_packed_tokens(&[
        operation, 
        Token::Address(to), 
        Token::Bytes(value.to_vec()),
        Token::Bytes(data_len),
        Token::Bytes(data.to_vec())
    ]).unwrap();

    format!("{}", Bytes::from(encoded))
}

pub fn encode_params(input: wrap::ArgsEncodeParams) -> String {
    let bytes: Bytes = utils_encode_params(input.types, input.values).into();
    format!("{}", bytes)
}

pub fn encode_function(input: wrap::ArgsEncodeFunction) -> String {
    let args: Vec<String> = input.args.unwrap_or(vec![]);
    let (_, bytes): (Function, Bytes) = utils_encode_function(&input.method, &args).unwrap();
    format!("{}", bytes)
}

pub fn encode_packed(input: wrap::ArgsEncodePacked) -> String {
    let encoded = utils_solidity_pack(input.types, input.values);
    format!("{}", Bytes::from(encoded))
}