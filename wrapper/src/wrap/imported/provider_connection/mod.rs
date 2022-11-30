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
    deserialize_provider_connection,
    read_provider_connection,
    serialize_provider_connection,
    write_provider_connection
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProviderConnection {
    pub node: Option<String>,
    pub network_name_or_chain_id: Option<String>,
}

impl ProviderConnection {
    pub const URI: &'static str = "wrap://ens/ethereum-provider.polywrap.eth";

    pub fn new() -> ProviderConnection {
        ProviderConnection {
            node: None,
            network_name_or_chain_id: None,
        }
    }

    pub fn to_buffer(args: &ProviderConnection) -> Result<Vec<u8>, EncodeError> {
        serialize_provider_connection(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ProviderConnection, DecodeError> {
        deserialize_provider_connection(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ProviderConnection, writer: &mut W) -> Result<(), EncodeError> {
        write_provider_connection(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ProviderConnection, DecodeError> {
        read_provider_connection(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
