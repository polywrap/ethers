use std::fmt::Debug;

use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{
    Block, BlockId, BlockNumber, Bytes, FeeHistory, NameOrAddress, Transaction, TransactionReceipt,
    TxHash, U256,
};
use ethers_core::utils;
use ethers_providers::{ProviderError, RpcError};
use polywrap_wasm_rs::JSON;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::wrap::connection::Connection;
use crate::wrap::imported::{
    ArgsRequest, ArgsWaitForTransaction, ProviderConnection, ProviderModule,
};

pub trait Provider {
    fn get_transaction_count<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        from: T,
        block: Option<BlockId>,
    ) -> Result<U256, ProviderError>;

    fn get_block_gen<Tx: Default + Serialize + DeserializeOwned + Debug>(
        &self,
        id: BlockId,
        include_txs: bool,
    ) -> Result<Option<Block<Tx>>, ProviderError>;

    fn get_block<T: Into<BlockId> + Send + Sync>(
        &self,
        block_hash_or_number: T,
    ) -> Result<Option<Block<TxHash>>, ProviderError>;

    fn fee_history<T: Into<U256> + Send + Sync>(
        &self,
        block_count: T,
        last_block: BlockNumber,
        reward_percentiles: &[f64],
    ) -> Result<FeeHistory, ProviderError>;

    fn fill_gas_fees(&self, tx: &mut TypedTransaction) -> Result<(), ProviderError>;

    fn get_gas_price(&self) -> Result<U256, ProviderError>;

    fn estimate_eip1559_fees(
        &self,
        estimator: Option<fn(U256, Vec<Vec<U256>>) -> (U256, U256)>,
    ) -> Result<(U256, U256), ProviderError>;

    fn estimate_gas(
        &self,
        tx: &TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<U256, ProviderError>;

    fn fill_transaction(
        &self,
        tx: &mut TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<(), ProviderError>;

    fn get_chainid(&self) -> Result<U256, ProviderError>;

    fn get_balance<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        from: T,
        block: Option<BlockId>,
    ) -> Result<U256, ProviderError>;

    fn get_transaction<T: Send + Sync + Into<TxHash>>(
        &self,
        transaction_hash: T,
    ) -> Result<Option<Transaction>, ProviderError>;

    fn get_transaction_receipt<T: Send + Sync + Into<TxHash>>(
        &self,
        transaction_hash: T,
    ) -> Result<Option<TransactionReceipt>, ProviderError>;

    fn call(
        &self,
        tx: &TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<Bytes, ProviderError>;
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
    /// Error in rpc request using provider
    #[error(transparent)]
    RpcClientError(ProviderError)

}

// @TODO(cbrzn): This need to be implemented before merging new packages structure
// Relevant:
// https://docs.rs/ethers/2.0.3/ethers/providers/struct.JsonRpcError.html
// https://github.com/ethers-io/ethers.js/blob/9f990c57f0486728902d4b8e049536f2bb3487ee/packages/providers/src.ts/json-rpc-provider.ts#L25-L53
impl RpcError for ClientError {
    fn as_error_response(&self) -> Option<&ethers_providers::JsonRpcError> {
        todo!()
    }

    fn as_serde_error(&self) -> Option<&JSON::Error> {
        todo!()
    }
}

impl From<ClientError> for ProviderError {
    fn from(src: ClientError) -> Self {
        match src {
            _ => ProviderError::JsonRpcClientError(Box::new(src)),
        }
    }
}

#[derive(Debug)]
pub struct WrapProvider {
    pub(super) connection: Option<ProviderConnection>,
}

impl WrapProvider {
    pub fn new(connection: &Option<Connection>) -> Self {
        let iprovider_connection = connection.as_ref().map(|conn| ProviderConnection {
            network_name_or_chain_id: conn.network_name_or_chain_id.clone(),
            node: conn.node.clone(),
        });
        Self {
            connection: iprovider_connection,
        }
    }

    pub fn request<T: Serialize + Send + Sync, R: DeserializeOwned>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, ProviderError> {
        let params_v: serde_json::Value = JSON::to_value(&params).unwrap();
        let res = ProviderModule::request(&ArgsRequest {
            method: method.to_string(),
            params: Some(params_v.into()),
            connection: self.connection.clone(),
        })
        .map_err(|err| ClientError::Error(err))?;
        let res = JSON::from_value(res.into()).map_err(|err| ClientError::SerdeJson {
            err,
            text: "from str failed".to_string(),
        })?;
        Ok(res)
    }

    pub fn await_transaction<T: Send + Sync + Into<TxHash>>(
        &self,
        transaction_hash: T,
        confirmations: u32,
        timeout: Option<u32>,
    ) -> Result<bool, ProviderError> {
        let hash = transaction_hash.into();

        let res = ProviderModule::wait_for_transaction(&ArgsWaitForTransaction {
            tx_hash: format!("{:#x}", hash),
            confirmations,
            timeout,
            connection: self.connection.clone(),
        })
        .map_err(|err| ClientError::Error(err))?;
        Ok(res)
    }
}

impl Provider for WrapProvider {
    /// Returns the nonce of the address
    fn get_transaction_count<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        from: T,
        block: Option<BlockId>,
    ) -> Result<U256, ProviderError> {
        let from = match from.into() {
            NameOrAddress::Name(ens_name) => {
                return Err(ProviderError::EnsError(format!(
                    "Cannot resolve ENS name {ens_name}. ENS name resolution is not supported."
                )))
            }
            NameOrAddress::Address(addr) => addr,
        };

        let from = utils::serialize(&from);
        let block = utils::serialize(&block.unwrap_or_else(|| BlockNumber::Latest.into()));
        self.request("eth_getTransactionCount", [from, block])
    }

    fn get_block_gen<Tx: Default + Serialize + DeserializeOwned + Debug>(
        &self,
        id: BlockId,
        include_txs: bool,
    ) -> Result<Option<Block<Tx>>, ProviderError> {
        let include_txs = utils::serialize(&include_txs);

        Ok(match id {
            BlockId::Hash(hash) => {
                let hash = utils::serialize(&hash);
                self.request("eth_getBlockByHash", [hash, include_txs])?
            }
            BlockId::Number(num) => {
                let num = utils::serialize(&num);
                self.request("eth_getBlockByNumber", [num, include_txs])?
            }
        })
    }

    /// Gets the block at `block_hash_or_number` (transaction hashes only)
    fn get_block<T: Into<BlockId> + Send + Sync>(
        &self,
        block_hash_or_number: T,
    ) -> Result<Option<Block<TxHash>>, ProviderError> {
        self.get_block_gen(block_hash_or_number.into(), false)
    }

    fn fee_history<T: Into<U256> + Send + Sync>(
        &self,
        block_count: T,
        last_block: BlockNumber,
        reward_percentiles: &[f64],
    ) -> Result<FeeHistory, ProviderError> {
        let block_count = block_count.into();
        let last_block = utils::serialize(&last_block);
        let reward_percentiles = utils::serialize(&reward_percentiles);

        // The blockCount param is expected to be an unsigned integer up to geth v1.10.6.
        // Geth v1.10.7 onwards, this has been updated to a hex encoded form. Failure to
        // decode the param from client side would fallback to the old API spec.
        match self.request::<_, FeeHistory>(
            "eth_feeHistory",
            [
                utils::serialize(&block_count),
                last_block.clone(),
                reward_percentiles.clone(),
            ],
        ) {
            success @ Ok(_) => success,
            err @ Err(_) => {
                let fallback = self.request::<_, FeeHistory>(
                    "eth_feeHistory",
                    [
                        utils::serialize(&block_count.as_u64()),
                        last_block,
                        reward_percentiles,
                    ],
                );

                if fallback.is_err() {
                    // if the older fallback also resulted in an error, we return the error from the
                    // initial attempt
                    return err;
                }
                fallback
            }
        }
    }

    fn fill_gas_fees(&self, tx: &mut TypedTransaction) -> Result<(), ProviderError> {
        match tx {
            TypedTransaction::Eip2930(_) | TypedTransaction::Legacy(_) => {
                let gas_price = if tx.gas_price().is_some() {
                    tx.gas_price().unwrap()
                } else {
                    self.get_gas_price()?
                };
                tx.set_gas_price(gas_price);
            }
            TypedTransaction::Eip1559(ref mut inner) => {
                if inner.max_fee_per_gas.is_none() || inner.max_priority_fee_per_gas.is_none() {
                    let (max_fee_per_gas, max_priority_fee_per_gas) =
                        self.estimate_eip1559_fees(None)?;
                    inner.max_fee_per_gas = Some(max_fee_per_gas);
                    inner.max_priority_fee_per_gas = Some(max_priority_fee_per_gas);
                };
            }
        }
        Ok(())
    }

    /// Gets the current gas price as estimated by the node
    fn get_gas_price(&self) -> Result<U256, ProviderError> {
        self.request("eth_gasPrice", ())
    }

    /// Gets a heuristic recommendation of max fee per gas and max priority fee per gas for
    /// EIP-1559 compatible transactions.
    fn estimate_eip1559_fees(
        &self,
        estimator: Option<fn(U256, Vec<Vec<U256>>) -> (U256, U256)>,
    ) -> Result<(U256, U256), ProviderError> {
        let base_fee_per_gas = self
            .get_block(BlockNumber::Latest)?
            .ok_or_else(|| ProviderError::CustomError("Latest block not found".into()))?
            .base_fee_per_gas
            .ok_or_else(|| ProviderError::CustomError("EIP-1559 not activated".into()))?;

        let fee_history = self.fee_history(
            utils::EIP1559_FEE_ESTIMATION_PAST_BLOCKS,
            BlockNumber::Latest,
            &[utils::EIP1559_FEE_ESTIMATION_REWARD_PERCENTILE],
        )?;

        // use the provided fee estimator function, or fallback to the default implementation.
        let (max_fee_per_gas, max_priority_fee_per_gas) = if let Some(es) = estimator {
            es(base_fee_per_gas, fee_history.reward)
        } else {
            utils::eip1559_default_estimator(base_fee_per_gas, fee_history.reward)
        };

        Ok((max_fee_per_gas, max_priority_fee_per_gas))
    }

    /// Sends a transaction to a single Ethereum node and return the estimated amount of gas
    /// required (as a U256) to send it This is free, but only an estimate. Providing too little
    /// gas will result in a transaction being rejected (while still consuming all provided
    /// gas).
    fn estimate_gas(
        &self,
        tx: &TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<U256, ProviderError> {
        let tx = utils::serialize(tx);
        // Some nodes (e.g. old Optimism clients) don't support a block ID being passed as a param,
        // so refrain from defaulting to BlockNumber::Latest.
        let params = if let Some(block_id) = block {
            vec![tx, utils::serialize(&block_id)]
        } else {
            vec![tx]
        };
        self.request("eth_estimateGas", params)
    }

    fn fill_transaction(
        &self,
        tx: &mut TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<(), ProviderError> {
        // set the ENS name
        if let Some(NameOrAddress::Name(ref ens_name)) = tx.to() {
            return Err(ProviderError::EnsError(format!(
                "Cannot resolve ENS name {ens_name}. ENS name resolution is not supported."
            )));
        }

        // fill gas price
        self.fill_gas_fees(tx)?;

        // Set gas to estimated value only if it was not set by the caller,
        // even if the access list has been populated and saves gas
        if tx.gas().is_none() {
            let gas_estimate = self.estimate_gas(tx, block)?;
            tx.set_gas(gas_estimate);
        }

        Ok(())
    }

    /// Returns the currently configured chain id, a value used in replay-protected
    /// transaction signing as introduced by EIP-155.
    fn get_chainid(&self) -> Result<U256, ProviderError> {
        self.request("eth_chainId", ())
    }

    /// Returns the account's balance
    fn get_balance<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        from: T,
        block: Option<BlockId>,
    ) -> Result<U256, ProviderError> {
        let from = match from.into() {
            NameOrAddress::Name(ens_name) => {
                return Err(ProviderError::EnsError(format!(
                    "Cannot resolve ENS name {ens_name}. ENS name resolution is not supported."
                )))
            }
            NameOrAddress::Address(addr) => addr,
        };

        let from = utils::serialize(&from);
        let block = utils::serialize(&block.unwrap_or_else(|| BlockNumber::Latest.into()));
        self.request("eth_getBalance", [from, block])
    }

    /// Gets the transaction with `transaction_hash`
    fn get_transaction<T: Send + Sync + Into<TxHash>>(
        &self,
        transaction_hash: T,
    ) -> Result<Option<Transaction>, ProviderError> {
        let hash = transaction_hash.into();
        self.request("eth_getTransactionByHash", [hash])
    }

    /// Gets the transaction receipt with `transaction_hash`
    fn get_transaction_receipt<T: Send + Sync + Into<TxHash>>(
        &self,
        transaction_hash: T,
    ) -> Result<Option<TransactionReceipt>, ProviderError> {
        let hash = transaction_hash.into();
        self.request("eth_getTransactionReceipt", [hash])
    }

    /// Sends the read-only (constant) transaction to a single Ethereum node and return the result
    /// (as bytes) of executing it. This is free, since it does not change any state on the
    /// blockchain.
    fn call(&self, tx: &TypedTransaction, block: Option<BlockId>) -> Result<Bytes, ProviderError> {
        let tx = utils::serialize(tx);
        let block = utils::serialize(&block.unwrap_or_else(|| BlockNumber::Latest.into()));
        self.request("eth_call", [tx, block])
    }
}
