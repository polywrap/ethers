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
    deserialize_access_item,
    read_access_item,
    serialize_access_item,
    write_access_item
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessItem {
    pub address: String,
    pub storage_keys: Vec<String>,
}

impl AccessItem {
    pub fn new() -> AccessItem {
        AccessItem {
            address: String::new(),
            storage_keys: vec![],
        }
    }

    pub fn to_buffer(args: &AccessItem) -> Result<Vec<u8>, EncodeError> {
        serialize_access_item(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<AccessItem, DecodeError> {
        deserialize_access_item(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &AccessItem, writer: &mut W) -> Result<(), EncodeError> {
        write_access_item(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<AccessItem, DecodeError> {
        read_access_item(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
