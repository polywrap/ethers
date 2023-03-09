use ethers_core::abi::{encode_packed, Token};
use ethers_core::utils::{keccak256 as keccak256_ethers, get_create2_address};
use ethers_core::types::{Bytes, Address};
use ethers_utils::encode_params;
use polywrap_wasm_rs::{BigInt};
use std::str::FromStr;

mod wrap;
use wrap::*;

pub fn keccak256(args: wrap::ArgsKeccak256) -> String {
    let decoded = Bytes::from_str(&args.value).unwrap();
    let hash = keccak256_ethers(decoded);
    format!("{}", Bytes::from(hash)).to_string()
}

pub fn keccak256_bytes_encode_packed(args: wrap::ArgsKeccak256BytesEncodePacked) -> String {
    let bytes = Bytes::from_str(&args.value).unwrap();
    let bytes = Token::Bytes(bytes.to_vec());
    let encoded = keccak256_ethers(encode_packed(&[bytes]).unwrap());
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