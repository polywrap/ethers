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
    deserialize_tx_request,
    read_tx_request,
    serialize_tx_request,
    write_tx_request
};

use crate::AccessItem;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TxRequest {
    pub to: Option<String>,
    pub from: Option<String>,
    pub data: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<u32>,
    pub chain_id: Option<BigInt>,
    pub access_list: Option<Vec<AccessItem>>,
    pub gas_limit: Option<BigInt>,
    pub max_fee_per_gas: Option<BigInt>,
    pub max_priority_fee_per_gas: Option<BigInt>,
    pub gas_price: Option<BigInt>,
    pub value: Option<BigInt>,
    pub nonce: Option<u32>,
}

impl TxRequest {
    pub fn new() -> TxRequest {
        TxRequest {
            to: None,
            from: None,
            data: None,
            _type: None,
            chain_id: None,
            access_list: None,
            gas_limit: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            gas_price: None,
            value: None,
            nonce: None,
        }
    }

    pub fn to_buffer(args: &TxRequest) -> Result<Vec<u8>, EncodeError> {
        serialize_tx_request(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<TxRequest, DecodeError> {
        deserialize_tx_request(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &TxRequest, writer: &mut W) -> Result<(), EncodeError> {
        write_tx_request(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<TxRequest, DecodeError> {
        read_tx_request(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
