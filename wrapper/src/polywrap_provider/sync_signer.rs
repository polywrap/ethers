use ethers_core::types::{transaction::eip2718::TypedTransaction, BlockId, Chain, TransactionRequest};
use ethers_signers::{Signer};
use ethers_middleware::signer::SignerMiddlewareError;
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Middleware, Provider};
use crate::polywrap_provider::provider::{PolywrapProvider};
use crate::polywrap_provider::signer::PolywrapSigner;
use crate::polywrap_provider::sync_provider::SyncProvider;

pub trait SyncSigner<M: Middleware, S: Signer> {
    fn fill_transaction_sync(
        &self,
        tx: &mut TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<(), SignerMiddlewareError<M, S>>;
}

impl SyncSigner<Provider<PolywrapProvider>, PolywrapSigner> for SignerMiddleware<Provider<PolywrapProvider>, PolywrapSigner> {
    /// Helper for filling a transaction's nonce using the wallet
    fn fill_transaction_sync(
        &self,
        tx: &mut TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<(), SignerMiddlewareError<Provider<PolywrapProvider>, PolywrapSigner>> {
        // get the `from` field's nonce if it's set, else get the signer's nonce
        let from = if tx.from().is_some() && tx.from() != Some(&self.address()) {
            *tx.from().unwrap()
        } else {
            self.signer().address()
        };
        tx.set_from(from);

        // get the signer's chain_id if the transaction does not set it
        let chain_id = self.signer().chain_id();
        if tx.chain_id().is_none() {
            tx.set_chain_id(chain_id);
        }

        // If a chain_id is matched to a known chain that doesn't support EIP-1559, automatically
        // change transaction to be Legacy type.
        if let Some(chain_id) = tx.chain_id() {
            let chain = Chain::try_from(chain_id.as_u64());
            if chain.unwrap_or_default().is_legacy() {
                if let TypedTransaction::Eip1559(inner) = tx {
                    let tx_req: TransactionRequest = inner.clone().into();
                    *tx = TypedTransaction::Legacy(tx_req);
                }
            }
        }

        let nonce = if tx.nonce().is_some() {
            tx.nonce().cloned().unwrap()
        } else {
            self.provider()
                .get_transaction_count_sync(from, block)
                .map_err(SignerMiddlewareError::MiddlewareError)?
        };
        tx.set_nonce(nonce);

        self.provider()
            .fill_transaction_sync(tx, block)
            .map_err(SignerMiddlewareError::MiddlewareError)?;
        Ok(())
    }
}
