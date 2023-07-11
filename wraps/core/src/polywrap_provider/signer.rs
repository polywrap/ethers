use std::str::FromStr;

use ethers_core::{
    abi::Address,
    types::{
        transaction::{eip2718::TypedTransaction, eip712::Eip712},
        BlockId, Bytes, Signature,
    },
};
use ethers_providers::ProviderError;
use ethers_signers::to_eip155_v;
use polywrap_wasm_rs::JSON;
use thiserror::Error;

use crate::wrap::{
    connection::Connection,
    imported::{
        ArgsRequest, ArgsSignMessage, ArgsSignTransaction, ArgsSignerAddress, ProviderConnection,
        ProviderModule,
    },
};

#[derive(Error, Debug)]
pub enum SignerError {
    /// Error type from Eip712Error message
    #[error("error encoding eip712 struct: {0:?}")]
    Eip712Error(String),
    #[error("error in send transaction: {0:?}")]
    SendError(String)
}

pub trait Signer {
    fn send(
        &self,
        tx: &TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<Bytes, ProviderError>;
    fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Signature, SignerError>;
    fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, SignerError>;
    fn sign_typed_data<T: Eip712 + Send + Sync>(
        &self,
        _payload: &T,
    ) -> Result<Signature, SignerError>;
}

#[derive(Clone, Debug)]
pub struct WrapSigner {
    /// The wallet's address
    address: Address,
    /// The wallet's chain id (for EIP-155)
    chain_id: u64,
    /// Ethereum connection to use
    connection: Option<ProviderConnection>,
}

impl WrapSigner {
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
        .expect("failed to obtain signer chain id from provider plugin")
        .into();
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

impl Signer for WrapSigner {
    fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Signature, SignerError> {
        let bytes = message.as_ref().to_vec();
        self.sign_bytes(bytes)
            .map_err(|e| SignerError::Eip712Error(e))
    }

    fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, SignerError> {
        // rlp must have the same chain id as v in the signature
        let chain_id = tx
            .chain_id()
            .map(|id| id.as_u64())
            .unwrap_or(self.chain_id());
        let mut tx = tx.clone();
        tx.set_chain_id(chain_id);
        let rlp = tx.rlp().to_vec();

        match self.sign_rlp(rlp) {
            Ok(mut sig) => {
                // sign_hash sets `v` to recid + 27, so we need to subtract 27 before normalizing
                sig.v = to_eip155_v(sig.v as u8 - 27, chain_id);
                Ok(sig)
            }
            Err(e) => Err(SignerError::Eip712Error(e)),
        }
    }

    fn sign_typed_data<T: Eip712 + Send + Sync>(
        &self,
        _payload: &T,
    ) -> Result<Signature, SignerError> {
        panic!("{} Not implemented.", "WrapSigner.sign_typed_data");
        // TODO: implement sign_typed_data
        // let encoded = payload
        //     .encode_eip712()
        //     .map_err(|e| Self::Error::Eip712Error(e.to_string()))?;
        // self.sign_bytes(encoded.to_vec().unwrap()).map_err(|e| SignerError::Eip712Error(e))
    }

    fn send(
        &self,
        _tx: &TypedTransaction,
        _block: Option<BlockId>,
    ) -> Result<Bytes, ethers_providers::ProviderError> {
        todo!()
    }
}
