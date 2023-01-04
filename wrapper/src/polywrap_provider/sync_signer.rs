use crate::polywrap_provider::signer::{PolywrapSigner, SignerError};
use ethers_core::types::{transaction::{eip2718::TypedTransaction, eip712::Eip712}, Address, Signature};
use ethers_signers::{to_eip155_v, Signer};

pub trait SyncSigner {
    fn sign_message_sync<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Signature, SignerError>;

    fn sign_transaction_sync(&self, tx: &TypedTransaction) -> Result<Signature, SignerError>;
}

impl SyncSigner for PolywrapSigner {
    fn sign_message_sync<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Signature, SignerError> {
        let bytes = message.as_ref().to_vec();
        self.sign_bytes(bytes).map_err(|e| SignerError::Eip712Error(e))
    }

    fn sign_transaction_sync(&self, tx: &TypedTransaction) -> Result<Signature, SignerError> {
        // rlp must have the same chain id as v in the signature
        let chain_id = tx.chain_id().map(|id| id.as_u64()).unwrap_or(self.chain_id());
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
}