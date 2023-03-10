use crate::polywrap_provider::signer::{PolywrapSigner, SignerError};
use ethers_core::types::{transaction::{eip2718::TypedTransaction,eip712::Eip712}, Signature};
use ethers_signers::{to_eip155_v};
use crate::imported::ArgsNonce;
pub trait SyncSigner {
    fn sign_message_sync<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Signature, SignerError>;

    fn sign_transaction_sync(&self, tx: &mut TypedTransaction) -> Result<Signature, SignerError>;

    fn sign_typed_data_sync<T: Eip712 + Send + Sync>(
        &self,
        _payload: &T,
    ) -> Result<Signature, SignerError>;
}

impl SyncSigner for PolywrapSigner {
    fn sign_message_sync<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Signature, SignerError> {
        let bytes = message.as_ref().to_vec();
        self.sign_bytes(bytes).map_err(|e| SignerError::Eip712Error(e))
    }

    fn sign_transaction_sync(&self, tx: &mut TypedTransaction) -> Result<Signature, SignerError> {
        // rlp must have the same chain id as v in the signature
        if tx.chain_id().is_none(){
            let chain_id = tx.chain_id().map(|id| id.as_u64()).unwrap_or(self.chain_id());
            tx.set_chain_id(chain_id);
        }

        if tx.nonce().is_none() {
            let nonce = self.iprovider.nonce(&ArgsNonce {
                connection: self.connection.clone(),
            }).unwrap();
            tx.set_nonce(nonce);
        }

        let rlp = tx.rlp().to_vec();

        match self.sign_rlp(rlp) {
            Ok(mut sig) => {
                // sign_hash sets `v` to recid + 27, so we need to subtract 27 before normalizing
                sig.v = to_eip155_v(sig.v as u8 - 27, tx.chain_id().unwrap().as_u64());
                Ok(sig)
            },
            Err(e) => Err(SignerError::Eip712Error(e))
        }
    }

    fn sign_typed_data_sync<T: Eip712 + Send + Sync>(
        &self,
        _payload: &T,
    ) -> Result<Signature, SignerError> {
        panic!("{} Not implemented.", "PolywrapSigner.sign_typed_data");
        // TODO: implement sign_typed_data
        // let encoded = payload
        //     .encode_eip712()
        //     .map_err(|e| Self::Error::Eip712Error(e.to_string()))?;
        // self.sign_bytes(encoded.to_vec().unwrap()).map_err(|e| SignerError::Eip712Error(e))
    }
}
