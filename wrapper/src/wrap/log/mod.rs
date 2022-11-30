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
    deserialize_log,
    read_log,
    serialize_log,
    write_log
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Log {
    pub block_number: BigInt,
    pub block_hash: String,
    pub transaction_index: u32,
    pub removed: bool,
    pub address: String,
    pub data: String,
    pub topics: Vec<String>,
    pub transaction_hash: String,
    pub log_index: u32,
}

impl Log {
    pub fn new() -> Log {
        Log {
            block_number: BigInt::default(),
            block_hash: String::new(),
            transaction_index: 0,
            removed: false,
            address: String::new(),
            data: String::new(),
            topics: vec![],
            transaction_hash: String::new(),
            log_index: 0,
        }
    }

    pub fn to_buffer(args: &Log) -> Result<Vec<u8>, EncodeError> {
        serialize_log(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<Log, DecodeError> {
        deserialize_log(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &Log, writer: &mut W) -> Result<(), EncodeError> {
        write_log(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Log, DecodeError> {
        read_log(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
