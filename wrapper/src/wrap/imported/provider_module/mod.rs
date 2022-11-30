use serde::{Serialize, Deserialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    Read,
    Write,
    JSON,
    subinvoke,
};
pub mod serialization;
pub use serialization::{
    deserialize_request_result,
    serialize_request_args,
    ArgsRequest,
    deserialize_sign_message_result,
    serialize_sign_message_args,
    ArgsSignMessage,
    deserialize_sign_transaction_result,
    serialize_sign_transaction_args,
    ArgsSignTransaction,
    deserialize_address_result,
    serialize_address_args,
    ArgsAddress,
    deserialize_chain_id_result,
    serialize_chain_id_args,
    ArgsChainId
};

use crate::ProviderConnection;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProviderModule {}

impl ProviderModule {
    pub const URI: &'static str = "wrap://ens/ethereum-provider.polywrap.eth";

    pub fn new() -> ProviderModule {
        ProviderModule {}
    }

    pub fn request(args: &ArgsRequest) -> Result<String, String> {
        let uri = ProviderModule::URI;
        let args = serialize_request_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "request",
            args,
        )?;
        deserialize_request_result(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn sign_message(args: &ArgsSignMessage) -> Result<String, String> {
        let uri = ProviderModule::URI;
        let args = serialize_sign_message_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "signMessage",
            args,
        )?;
        deserialize_sign_message_result(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn sign_transaction(args: &ArgsSignTransaction) -> Result<String, String> {
        let uri = ProviderModule::URI;
        let args = serialize_sign_transaction_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "signTransaction",
            args,
        )?;
        deserialize_sign_transaction_result(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn address(args: &ArgsAddress) -> Result<String, String> {
        let uri = ProviderModule::URI;
        let args = serialize_address_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "address",
            args,
        )?;
        deserialize_address_result(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn chain_id(args: &ArgsChainId) -> Result<String, String> {
        let uri = ProviderModule::URI;
        let args = serialize_chain_id_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "chainId",
            args,
        )?;
        deserialize_chain_id_result(result.as_slice()).map_err(|e| e.to_string())
    }
}
