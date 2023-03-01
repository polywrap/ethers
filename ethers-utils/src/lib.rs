use ethers_core::abi::token::{LenientTokenizer, Tokenizer};
use ethers_core::abi::{encode_packed, Token, encode, HumanReadableParser};
use ethers_core::utils::{keccak256, get_create2_address};
use ethers_core::types::{Bytes, Address};
use polywrap_wasm_rs::{BigInt};
use std::str::FromStr;

mod wrap;
use wrap::*;

pub fn solidity_keccak256_bytes(args: wrap::ArgsSolidityKeccak256Bytes) -> String {
    let value = Token::Bytes(args.bytes.as_bytes().to_vec());
    let hash = keccak256(encode(&[value]));

    format!("{}", Bytes::from(hash)).to_string()
}

pub fn encode_bytes_value(args: wrap::ArgsEncodeBytesValue) -> String {
    let mut bytes: Vec<u8> = Vec::with_capacity(args.value.len());
    bytes.extend_from_slice(args.value.as_bytes());
    format!("{}", Bytes::from(bytes)).to_string()
}


pub fn keccak256_bytes(args: wrap::ArgsKeccak256Bytes) -> String {
    let decoded = Bytes::from_str(&args.bytes).unwrap();
    let hash = keccak256(decoded);
    format!("{}", Bytes::from(hash)).to_string()
}

pub fn keccak256_bytes_encode_packed(args: wrap::ArgsKeccak256BytesEncodePacked) -> String {
    let bytes = Bytes::from_str(&args.bytes).unwrap();
    let bytes = Token::Bytes(bytes.to_vec());
    let encoded = keccak256(encode_packed(&[bytes]).unwrap());
    format!("{}", Bytes::from(encoded)).to_string()
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

    let value = encode_params(
        vec!["uint256".into()],
        vec![args.value.to_string()]
    );

    let data = Bytes::from_str(&args.data).unwrap();
    let data_len = encode_params(
        vec!["uint256".into()],
        vec![data.len().to_string()]
    );

    let encoded = encode_packed(&[
        operation, 
        Token::Address(to), 
        Token::Bytes(value.to_vec()),
        Token::Bytes(data_len),
        Token::Bytes(data.to_vec())
    ]).unwrap();
    
    format!("{}", Bytes::from(encoded)).to_string()
}

pub fn encode_params(types: Vec<String>, values: Vec<String>) -> Vec<u8> {
    let tokens: Vec<Token> = values
        .iter()
        .zip(types.iter())
        .map(|(arg, t)| {
            let kind = HumanReadableParser::parse_type(&t).unwrap();
            LenientTokenizer::tokenize(&kind, arg).unwrap()
        })
        .collect();
    let bytes = encode(&tokens);
    bytes
}
