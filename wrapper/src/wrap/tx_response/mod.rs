use serde::{Serialize, Deserialize};
pub mod serialization;
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    DecodeError,
    EncodeError,
    Read,
    Write,
    JSON,
};
pub use serialization::{
    deserialize_tx_response,
    read_tx_response,
    serialize_tx_response,
    write_tx_response
};

use crate::AccessItem;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TxResponse {
    pub hash: String,
    pub to: Option<String>,
    pub from: String,
    pub nonce: u32,
    pub gas_limit: BigInt,
    pub max_fee_per_gas: Option<BigInt>,
    pub max_priority_fee_per_gas: Option<BigInt>,
    pub gas_price: Option<BigInt>,
    pub value: BigInt,
    pub chain_id: BigInt,
    pub block_number: Option<BigInt>,
    pub block_hash: Option<String>,
    pub timestamp: Option<u32>,
    pub r: Option<String>,
    pub s: Option<String>,
    pub v: Option<u32>,
    #[serde(rename = "type")]
    pub _type: Option<u32>,
    pub access_list: Option<Vec<AccessItem>>,
}

impl TxResponse {
    pub fn new() -> TxResponse {
        TxResponse {
            hash: String::new(),
            to: None,
            from: String::new(),
            nonce: 0,
            gas_limit: BigInt::default(),
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            gas_price: None,
            value: BigInt::default(),
            chain_id: BigInt::default(),
            block_number: None,
            block_hash: None,
            timestamp: None,
            r: None,
            s: None,
            v: None,
            _type: None,
            access_list: None,
        }
    }

    pub fn to_buffer(args: &TxResponse) -> Result<Vec<u8>, EncodeError> {
        serialize_tx_response(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<TxResponse, DecodeError> {
        deserialize_tx_response(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &TxResponse, writer: &mut W) -> Result<(), EncodeError> {
        write_tx_response(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<TxResponse, DecodeError> {
        read_tx_response(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
