use ethers_providers::{JsonRpcClient, ProviderError, Provider, Middleware};

use crate::wrap::imported::ArgsRequest;
use crate::wrap::{IProviderModule, IProviderConnection, Connection};
use crate::iprovider::get_iprovider;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

#[derive(Debug)]
pub struct PolywrapProvider {
    connection: Option<IProviderConnection>,
    iprovider: IProviderModule,
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
pub trait GasWorkaround {
    async fn fill_gas_fees(&self, tx: &mut TypedTransaction) -> Result<(), ProviderError>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl GasWorkaround for Provider<PolywrapProvider> {
    async fn fill_gas_fees(&self, tx: &mut TypedTransaction) -> Result<(), ProviderError> {
        match tx {
            TypedTransaction::Eip2930(_) | TypedTransaction::Legacy(_) => {
                let gas_price = if tx.gas_price().is_some() {
                    tx.gas_price().unwrap()
                } else {
                    self.get_gas_price().await?
                };
                tx.set_gas_price(gas_price);
            }
            TypedTransaction::Eip1559(ref mut inner) => {
                if inner.max_fee_per_gas.is_none() || inner.max_priority_fee_per_gas.is_none() {
                    let (max_fee_per_gas, max_priority_fee_per_gas) =
                        self.estimate_eip1559_fees(None).await?;
                    inner.max_fee_per_gas = Some(max_fee_per_gas);
                    inner.max_priority_fee_per_gas = Some(max_priority_fee_per_gas);
                };
            }
        }
        Ok(())
    }
}
