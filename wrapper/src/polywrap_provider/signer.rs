use crate::wrap::imported::{ArgsAddress, ArgsChainId, ArgsSignMessage, ArgsSignTransaction};
use crate::wrap::{IProviderModule, IProviderConnection, Connection};
use super::iprovider::get_iprovider;
use async_trait::async_trait;
use ethers_core::types::{transaction::{eip2718::TypedTransaction, eip712::Eip712}, Address, Signature};
use ethers_signers::{to_eip155_v, Signer};
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct PolywrapSigner {
    /// The wallet's address
    address: Address,
    /// The wallet's chain id (for EIP-155)
    chain_id: u64,
    /// Ethereum connection to use
    connection: Option<IProviderConnection>,
    iprovider: IProviderModule,
}

#[derive(Error, Debug)]
/// Error thrown when sending an HTTP request
pub enum SignerError {
    /// Error type from Eip712Error message
    #[error("error encoding eip712 struct: {0:?}")]
    Eip712Error(String),
}

impl PolywrapSigner {
    pub fn new(connection: &Option<Connection>) -> Self {
        let iprovider_connection = connection.as_ref().map(|conn| IProviderConnection {
            network_name_or_chain_id: conn.network_name_or_chain_id.clone(),
            node: conn.node.clone(),
        });
        let iprovider = get_iprovider();
        let address = iprovider.address(&ArgsAddress { connection: iprovider_connection.clone() }).unwrap();
        let chain_id = iprovider.chain_id(&ArgsChainId { connection: iprovider_connection.clone() })
            .expect("failed to obtain signer chain id from provider plugin");
        Self {
            address: Address::from_str(&address).unwrap(),
            chain_id: u64::from_str(&chain_id).unwrap(),
            connection: iprovider_connection,
            iprovider,
        }
    }

    pub(super) fn sign_rlp(&self, rlp: Vec<u8>) -> Result<Signature, String> {
        let signature = self.iprovider.sign_transaction(&ArgsSignTransaction { rlp, connection: self.connection.clone(), })?;
        Ok(Signature::from_str(&signature).unwrap())
    }

    pub(super) fn sign_bytes(&self, message: Vec<u8>) -> Result<Signature, String> {
        let signature = self.iprovider.sign_message(&ArgsSignMessage { message, connection: self.connection.clone(), })?;
        Ok(Signature::from_str(&signature).unwrap())
    }

    fn connection(&self) -> Option<IProviderConnection> {
        self.connection.clone()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Signer for PolywrapSigner {
    type Error = SignerError;

    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        _message: S,
    ) -> Result<Signature, Self::Error> {
        panic!("{} Not implemented. Use {} instead.", "PolywrapSigner.sign_message", "PolywrapSigner.sign_message_sync");
    }

    async fn sign_transaction(&self, _tx: &TypedTransaction) -> Result<Signature, Self::Error> {
        panic!("{} Not implemented. Use {} instead.", "PolywrapSigner.sign_transaction", "PolywrapSigner.sign_transaction_sync");
    }

    async fn sign_typed_data<T: Eip712 + Send + Sync>(
        &self,
        _payload: &T,
    ) -> Result<Signature, Self::Error> {
        panic!("{} Not implemented.", "PolywrapSigner.sign_typed_data");
        // TODO: implement sign_typed_data
        // let encoded = payload
        //     .encode_eip712()
        //     .map_err(|e| Self::Error::Eip712Error(e.to_string()))?;
        // self.sign_bytes(encoded.to_vec().unwrap()).map_err(|e| SignerError::Eip712Error(e))
    }

    fn address(&self) -> Address {
        self.address
    }

    /// Gets the wallet's chain id
    fn chain_id(&self) -> u64 {
        self.chain_id
    }

    /// Sets the wallet's chain_id, used in conjunction with EIP-155 signing
    fn with_chain_id<T: Into<u64>>(mut self, chain_id: T) -> Self {
        self.chain_id = chain_id.into();
        self
    }
}
