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
    deserialize_tx_options,
    read_tx_options,
    serialize_tx_options,
    write_tx_options
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TxOptions {
    pub gas_limit: Option<BigInt>,
    pub max_fee_per_gas: Option<BigInt>,
    pub max_priority_fee_per_gas: Option<BigInt>,
    pub gas_price: Option<BigInt>,
    pub value: Option<BigInt>,
    pub nonce: Option<u32>,
}

impl TxOptions {
    pub fn new() -> TxOptions {
        TxOptions {
            gas_limit: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            gas_price: None,
            value: None,
            nonce: None,
        }
    }

    pub fn to_buffer(args: &TxOptions) -> Result<Vec<u8>, EncodeError> {
        serialize_tx_options(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<TxOptions, DecodeError> {
        deserialize_tx_options(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &TxOptions, writer: &mut W) -> Result<(), EncodeError> {
        write_tx_options(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<TxOptions, DecodeError> {
        read_tx_options(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
