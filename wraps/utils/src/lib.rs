use ethers_core::abi::{encode_packed, Function, Token};
use ethers_core::types::{Address, Bytes};
use ethers_core::utils::{get_create2_address, keccak256 as keccak256_ethers};
use ethers_utils::{
    encode_function as utils_encode_function,
    encode_params as utils_encode_params,
    solidity_pack as utils_solidity_pack,
    to_eth as utils_to_eth,
    to_wei as utils_to_wei
};
use polywrap_wasm_rs::{BigInt, JSON};
use std::str::FromStr;

mod wrap;
use wrap::*;
use wrap::module::{Module, ModuleTrait};

impl ModuleTrait for Module {
    fn keccak256(args: wrap::ArgsKeccak256) -> Result<String, String> {
        let hash = keccak256_ethers(args.value.as_bytes());
        Ok(format!("{}", Bytes::from(hash)).to_string())
    }

    fn keccak256_bytes_encode_packed(args: wrap::ArgsKeccak256BytesEncodePacked) -> Result<String, String> {
        let bytes = Bytes::from_str(&args.value).unwrap();
        let bytes = Token::Bytes(bytes.to_vec());
        let encoded = keccak256_ethers(encode_packed(&[bytes]).unwrap());
        Ok(format!("{}", Bytes::from(encoded)).to_string())
    }

    fn generate_create2_address(args: wrap::ArgsGenerateCreate2Address) -> Result<String, String> {
        let salt = Bytes::from_str(&args.salt).unwrap();
        let init_code = Bytes::from_str(&args.init_code).unwrap();
        let address = args.address.parse::<Address>().unwrap();
        let generated_address = get_create2_address(address, salt, init_code);
        Ok(format!("{:?}", generated_address))
    }

    fn encode_meta_transaction(args: wrap::ArgsEncodeMetaTransaction) -> Result<String, String> {
        let mut op_bytes: [u8; 1] = [0];

        if let Some(op) = args.operation {
            if BigInt::from(1) == op {
                op_bytes[0] = 1;
            }
        }

        let operation = Token::FixedBytes(op_bytes.into());
        let to = args.to.parse::<Address>().unwrap();

        let value = utils_encode_params(vec!["uint256".into()], vec![args.value.to_string()]);

        let data = Bytes::from_str(&args.data).unwrap();
        let data_len = utils_encode_params(vec!["uint256".into()], vec![data.len().to_string()]);

        let encoded = encode_packed(&[
            operation,
            Token::Address(to),
            Token::Bytes(value.to_vec()),
            Token::Bytes(data_len),
            Token::Bytes(data.to_vec()),
        ])
        .unwrap();

        Ok(format!("{}", Bytes::from(encoded)))
    }

    fn encode_params(input: wrap::ArgsEncodeParams) -> Result<String, String> {
        let bytes: Bytes = utils_encode_params(input.types, input.values).into();
        Ok(format!("{}", bytes))
    }

    fn encode_function(input: wrap::ArgsEncodeFunction) -> Result<String, String> {
        let args: Vec<String> = input.args.unwrap_or(vec![]);
        let (_, bytes): (Function, Bytes) = utils_encode_function(&input.method, &args).unwrap();
        Ok(format!("{}", bytes))
    }

    fn to_wei(input: ArgsToWei) -> Result<String, String> {
        Ok(utils_to_wei(input.eth).to_string())
    }

    fn to_eth(input: ArgsToEth) -> Result<String, String> {
        Ok(utils_to_eth(input.wei).to_string())
    }

    fn solidity_pack(args: wrap::ArgsSolidityPack) -> Result<String, String> {
        return utils_solidity_pack(args.types, args.values);
    }
}
