use crate::wrap::imported::{ArgsAddress, ArgsChainId, ArgsSignMessage, ArgsSignTransaction};
use crate::wrap::{IProviderModule, IProviderConnection, Connection};
use super::iprovider::get_iprovider;
use ethers_core::types::{Address, Signature};
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

    pub fn address(&self) -> Address {
        self.address
    }

    /// Gets the wallet's chain id
    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }
}