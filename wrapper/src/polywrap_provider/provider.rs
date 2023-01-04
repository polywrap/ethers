use std::fmt::Debug;
use ethers_providers::{JsonRpcClient, ProviderError};

use crate::wrap::imported::ArgsRequest;
use crate::wrap::{IProviderModule, IProviderConnection, Connection};
use crate::iprovider::get_iprovider;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

#[derive(Debug)]
pub struct PolywrapProvider {
    pub(super) connection: Option<IProviderConnection>,
    pub(super) iprovider: IProviderModule,
}

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

impl PolywrapProvider {
    pub fn new(connection: &Option<Connection>) -> Self {
        let iprovider_connection = connection.as_ref().map(|conn| IProviderConnection {
            network_name_or_chain_id: conn.network_name_or_chain_id.clone(),
            node: conn.node.clone(),
        });
        Self {
            connection: iprovider_connection,
            iprovider: get_iprovider(),
        }
    }

    pub fn connection(&self) -> Option<IProviderConnection> {
        self.connection.clone()
    }
}

impl Clone for PolywrapProvider {
    fn clone(&self) -> Self {
        Self {
            connection: self.connection.clone(),
            iprovider: self.iprovider.clone(),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl JsonRpcClient for PolywrapProvider {
    type Error = ClientError;

    /// Sends a POST request with the provided method and the params serialized as JSON
    /// over HTTP
    async fn request<T: Serialize + Send + Sync, R: DeserializeOwned>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, Self::Error> {
        let params_s = serde_json::to_string(&params).unwrap();
        let res = self.iprovider.request(&ArgsRequest {
            method: method.to_string(),
            params: Some(params_s),
            connection: self.connection.clone(),
        })
        .map_err(|err| ClientError::Error(err))?;
        let res = serde_json::from_str(&res).map_err(|err| ClientError::SerdeJson {
            err,
            text: "from str failed".to_string(),
        })?;
        Ok(res)
    }
}