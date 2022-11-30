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
    deserialize_tx_receipt,
    read_tx_receipt,
    serialize_tx_receipt,
    write_tx_receipt
};

use crate::Log;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TxReceipt {
    pub to: String,
    pub from: String,
    pub contract_address: String,
    pub transaction_index: u32,
    pub root: Option<String>,
    pub gas_used: BigInt,
    pub logs_bloom: String,
    pub transaction_hash: String,
    pub logs: Vec<Log>,
    pub block_number: BigInt,
    pub block_hash: String,
    pub confirmations: u32,
    pub cumulative_gas_used: BigInt,
    pub effective_gas_price: BigInt,
    #[serde(rename = "type")]
    pub _type: u32,
    pub status: Option<u32>,
}

impl TxReceipt {
    pub fn new() -> TxReceipt {
        TxReceipt {
            to: String::new(),
            from: String::new(),
            contract_address: String::new(),
            transaction_index: 0,
            root: None,
            gas_used: BigInt::default(),
            logs_bloom: String::new(),
            transaction_hash: String::new(),
            logs: vec![],
            block_number: BigInt::default(),
            block_hash: String::new(),
            confirmations: 0,
            cumulative_gas_used: BigInt::default(),
            effective_gas_price: BigInt::default(),
            _type: 0,
            status: None,
        }
    }

    pub fn to_buffer(args: &TxReceipt) -> Result<Vec<u8>, EncodeError> {
        serialize_tx_receipt(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<TxReceipt, DecodeError> {
        deserialize_tx_receipt(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &TxReceipt, writer: &mut W) -> Result<(), EncodeError> {
        write_tx_receipt(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<TxReceipt, DecodeError> {
        read_tx_receipt(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
