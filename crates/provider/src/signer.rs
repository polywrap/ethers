use ethers_providers::ProviderError;
use thiserror::Error;
use ethers_core::types::{
    transaction::{
        eip2718::TypedTransaction,eip712::Eip712
    },
    Signature, BlockId, Bytes
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
