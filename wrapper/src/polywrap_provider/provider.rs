use std::fmt::Debug;
use ethers_providers::{ProviderError};

use crate::wrap::imported::{ArgsRequest, ArgsWaitForTransaction};
use crate::wrap::{ProviderModule, ProviderConnection, Connection};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use ethers_core::types::{TxHash};
use polywrap_wasm_rs::JSON;

#[derive(Error, Debug)]
/// Error thrown when sending an HTTP request
pub enum ClientError {
    /// Serde JSON Error
    #[error("Deserialization Error: {err}. Response: {text}")]
    SerdeJson {
        err: serde_json::Error,
        text: String,
    },
    /// Serde JSON Error
    #[error("Client error: {0}")]
    Error(String),
}

impl From<ClientError> for ProviderError {
    fn from(src: ClientError) -> Self {
        match src {
            _ => ProviderError::JsonRpcClientError(Box::new(src)),
        }
    }
}

#[derive(Debug)]
pub struct PolywrapProvider {
    pub(super) connection: Option<ProviderConnection>,
}

impl PolywrapProvider {
    pub fn new(connection: &Option<Connection>) -> Self {
        let iprovider_connection = connection.as_ref().map(|conn| ProviderConnection {
            network_name_or_chain_id: conn.network_name_or_chain_id.clone(),
            node: conn.node.clone(),
        });
        Self {
            connection: iprovider_connection,
        }
    }

    pub fn request_sync<T: Serialize + Send + Sync, R: DeserializeOwned>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, ProviderError> {
        let params_v = JSON::to_value(&params).unwrap();
        let res = ProviderModule::request(&ArgsRequest {
            method: method.to_string(),
            params: Some(params_v),
            connection: self.connection.clone(),
        })
            .map_err(|err| ClientError::Error(err))?;
        let res = JSON::from_value(res).map_err(|err| ClientError::SerdeJson {
            err,
            text: "from str failed".to_string(),
        })?;
        Ok(res)
    }

    pub fn await_transaction_sync<T: Send + Sync + Into<TxHash>>(
        &self,
        transaction_hash: T,
        confirmations: u32,
        timeout: Option<u32>,
    ) -> Result<bool, ProviderError> {
        let hash = transaction_hash.into();

        let res = ProviderModule::wait_for_transaction(&ArgsWaitForTransaction {
            tx_hash: format!("{:#x}", hash),
            confirmations,
            timeout,
            connection: self.connection.clone(),
        })
            .map_err(|err| ClientError::Error(err))?;
        Ok(res)
    }
}