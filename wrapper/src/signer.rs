use super::wrap::imported::{ArgsAddress, ArgsChainId, ArgsSignMessage, ArgsSignTransaction};
use super::wrap::{IProviderModule, IProviderConnection};
use crate::iprovider::get_iprovider;
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
    pub fn new(connection: &Option<IProviderConnection>) -> Self {
        let iprovider = get_iprovider();
        let address = iprovider.address(&ArgsAddress { connection: connection.clone() }).unwrap();
        let chain_id = iprovider.chain_id(&ArgsChainId { connection: connection.clone() })
            .expect("failed to obtain signer chain id from provider plugin");
        Self {
            address: Address::from_str(&address).unwrap(),
            chain_id: u64::from_str(&chain_id).unwrap(),
            connection: connection.clone(),
            iprovider,
        }
    }

    fn sign_rlp(&self, rlp: Vec<u8>) -> Result<Signature, String> {
        let signature = self.iprovider.sign_transaction(&ArgsSignTransaction { rlp, connection: self.connection.clone(), })?;
        Ok(Signature::from_str(&signature).unwrap())
    }

    fn sign_bytes(&self, message: Vec<u8>) -> Result<Signature, String> {
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

    // TODO: format errors correctly for Eip712 spec

    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Signature, Self::Error> {
        let bytes = message.as_ref().to_vec();
        self.sign_bytes(bytes).map_err(|e| SignerError::Eip712Error(e))
    }

    async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, Self::Error> {
        // rlp must have the same chain id as v in the signature
        let chain_id = tx.chain_id().map(|id| id.as_u64()).unwrap_or(self.chain_id);
        let mut tx = tx.clone();
        tx.set_chain_id(chain_id);
        let rlp = tx.rlp().to_vec();

        match self.sign_rlp(rlp) {
            Ok(mut sig) => {
                // sign_hash sets `v` to recid + 27, so we need to subtract 27 before normalizing
                sig.v = to_eip155_v(sig.v as u8 - 27, chain_id);
                Ok(sig)
            },
            Err(e) => Err(SignerError::Eip712Error(e))
        }
    }

    async fn sign_typed_data<T: Eip712 + Send + Sync>(
        &self,
        payload: &T,
    ) -> Result<Signature, Self::Error> {
        return Err(SignerError::Eip712Error("not implemented".to_string()));
        // TODO: need to turn encoded into a form that can be signed with sign_bytes or sign_rlp
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
