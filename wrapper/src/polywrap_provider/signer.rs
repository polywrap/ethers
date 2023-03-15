use crate::wrap::imported::{ArgsRequest, ArgsSignMessage, ArgsSignTransaction, ArgsSignerAddress};
use crate::wrap::{Connection, ProviderConnection, ProviderModule};
use ethers_core::types::{Address, Signature};
use polywrap_wasm_rs::{JSON};
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct PolywrapSigner {
    /// The wallet's address
    address: Address,
    /// The wallet's chain id (for EIP-155)
    chain_id: u64,
    /// Ethereum connection to use
    connection: Option<ProviderConnection>,
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
        let iprovider_connection = connection.as_ref().map(|conn| ProviderConnection {
            network_name_or_chain_id: conn.network_name_or_chain_id.clone(),
            node: conn.node.clone(),
        });

        let address = ProviderModule::signer_address(&ArgsSignerAddress {
            connection: iprovider_connection.clone(),
        })
        .unwrap()
        .unwrap();
        let chain_id: JSON::Value = ProviderModule::request(&ArgsRequest {
            method: "eth_chainId".to_string(),
            params: None,
            connection: iprovider_connection.clone(),
        })
        .expect("failed to obtain signer chain id from provider plugin");
        let chain_id = chain_id.as_str().unwrap();
        Self {
            address: Address::from_str(&address.as_str()).unwrap(),
            chain_id: u64::from_str_radix(&chain_id[2..], 16).unwrap(),
            connection: iprovider_connection,
        }
    }

    pub(super) fn sign_rlp(&self, rlp: Vec<u8>) -> Result<Signature, String> {
        let signature = ProviderModule::sign_transaction(&ArgsSignTransaction {
            rlp,
            connection: self.connection.clone(),
        })?;
        Ok(Signature::from_str(&signature).unwrap())
    }

    pub(super) fn sign_bytes(&self, message: Vec<u8>) -> Result<Signature, String> {
        let signature = ProviderModule::sign_message(&ArgsSignMessage {
            message,
            connection: self.connection.clone(),
        })?;
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
