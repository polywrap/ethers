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
    deserialize_static_tx_result,
    read_static_tx_result,
    serialize_static_tx_result,
    write_static_tx_result
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StaticTxResult {
    pub result: String,
    pub error: bool,
}

impl StaticTxResult {
    pub fn new() -> StaticTxResult {
        StaticTxResult {
            result: String::new(),
            error: false,
        }
    }

    pub fn to_buffer(args: &StaticTxResult) -> Result<Vec<u8>, EncodeError> {
        serialize_static_tx_result(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<StaticTxResult, DecodeError> {
        deserialize_static_tx_result(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &StaticTxResult, writer: &mut W) -> Result<(), EncodeError> {
        write_static_tx_result(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<StaticTxResult, DecodeError> {
        read_static_tx_result(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
