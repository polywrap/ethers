use std::fmt::Debug;
use ethers_providers::{ProviderError, Provider, Middleware, ens};

use crate::wrap::imported::ArgsRequest;
use ethabi::ParamType;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, Block, BlockId, BlockNumber, Bytes, FeeHistory, NameOrAddress, Selector, Transaction, TransactionReceipt, TxHash, U256};
use ethers_core::{abi, utils};
use ethers_core::abi::Detokenize;
use serde::{de::DeserializeOwned, Serialize};
use crate::api::decode_bytes;
use crate::polywrap_provider::provider::{ClientError, PolywrapProvider};

pub trait SyncProvider {
    fn request_sync<T: Serialize + Send + Sync, R: DeserializeOwned>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, ProviderError>;

    fn get_transaction_count_sync<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        from: T,
        block: Option<BlockId>,
    ) -> Result<U256, ProviderError>;

    fn get_block_gen_sync<Tx: Default + Serialize + DeserializeOwned + Debug>(
        &self,
        id: BlockId,
        include_txs: bool,
    ) -> Result<Option<Block<Tx>>, ProviderError>;

    fn get_block_sync<T: Into<BlockId> + Send + Sync>(
        &self,
        block_hash_or_number: T,
    ) -> Result<Option<Block<TxHash>>, ProviderError>;

    fn fee_history_sync<T: Into<U256> + Send + Sync>(
        &self,
        block_count: T,
        last_block: BlockNumber,
        reward_percentiles: &[f64],
    ) -> Result<FeeHistory, ProviderError>;

    fn fill_gas_fees_sync(&self, tx: &mut TypedTransaction) -> Result<(), ProviderError>;

    fn get_gas_price_sync(&self) -> Result<U256, ProviderError>;

    fn estimate_eip1559_fees_sync(
        &self,
        estimator: Option<fn(U256, Vec<Vec<U256>>) -> (U256, U256)>,
    ) -> Result<(U256, U256), ProviderError>;

    fn estimate_gas_sync(
        &self,
        tx: &TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<U256, ProviderError>;

    fn fill_transaction_sync(
        &self,
        tx: &mut TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<(), ProviderError>;

    fn get_chainid_sync(&self) -> Result<U256, ProviderError>;

    fn get_balance_sync<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        from: T,
        block: Option<BlockId>,
    ) -> Result<U256, ProviderError>;

    fn get_transaction_sync<T: Send + Sync + Into<TxHash>>(
        &self,
        transaction_hash: T,
    ) -> Result<Option<Transaction>, ProviderError>;

    fn get_transaction_receipt_sync<T: Send + Sync + Into<TxHash>>(
        &self,
        transaction_hash: T,
    ) -> Result<Option<TransactionReceipt>, ProviderError>;

    fn call_sync(
        &self,
        tx: &TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<Bytes, ProviderError>;

    fn resolve_name_sync(&self, ens_name: &str, ens_address: Option<Address>) -> Result<Address, ProviderError>;

    fn query_resolver_parameters_sync<T: Detokenize>(
        &self,
        param: ParamType,
        ens_name: &str,
        selector: Selector,
        parameters: Option<&[u8]>,
        ens_address: Option<Address>,
    ) -> Result<T, ProviderError>;

    fn validate_resolver_sync(
        &self,
        resolver_address: Address,
        selector: Selector,
        ens_name: &str,
    ) -> Result<(), ProviderError>;
}

impl SyncProvider for Provider<PolywrapProvider> {

    /// Sends a POST request with the provided method and the params serialized as JSON over HTTP
    fn request_sync<T: Serialize + Send + Sync, R: DeserializeOwned>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, ProviderError> {
        let params_s = serde_json::to_string(&params).unwrap();
        let res = self.as_ref().iprovider.request(&ArgsRequest {
            method: method.to_string(),
            params: Some(params_s),
            connection: self.as_ref().connection.clone(),
        })
            .map_err(|err| ClientError::Error(err))?;
        let res = serde_json::from_str(&res).map_err(|err| ClientError::SerdeJson {
            err,
            text: "from str failed".to_string(),
        })?;
        Ok(res)
    }

    /// Returns the nonce of the address
    fn get_transaction_count_sync<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        from: T,
        block: Option<BlockId>,
    ) -> Result<U256, ProviderError> {
        let from = match from.into() {
            NameOrAddress::Name(ens_name) => self.resolve_name_sync(&ens_name, None)?,
            NameOrAddress::Address(addr) => addr,
        };

        let from = utils::serialize(&from);
        let block = utils::serialize(&block.unwrap_or_else(|| BlockNumber::Latest.into()));
        self.request_sync("eth_getTransactionCount", [from, block])
    }

    fn get_block_gen_sync<Tx: Default + Serialize + DeserializeOwned + Debug>(
        &self,
        id: BlockId,
        include_txs: bool,
    ) -> Result<Option<Block<Tx>>, ProviderError> {
        let include_txs = utils::serialize(&include_txs);

        Ok(match id {
            BlockId::Hash(hash) => {
                let hash = utils::serialize(&hash);
                self.request_sync("eth_getBlockByHash", [hash, include_txs])?
            }
            BlockId::Number(num) => {
                let num = utils::serialize(&num);
                self.request_sync("eth_getBlockByNumber", [num, include_txs])?
            }
        })
    }

    /// Gets the block at `block_hash_or_number` (transaction hashes only)
    fn get_block_sync<T: Into<BlockId> + Send + Sync>(
        &self,
        block_hash_or_number: T,
    ) -> Result<Option<Block<TxHash>>, ProviderError> {
        self.get_block_gen_sync(block_hash_or_number.into(), false)
    }

    fn fee_history_sync<T: Into<U256> + Send + Sync>(
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
        match self
            .request_sync::<_, FeeHistory>(
                "eth_feeHistory",
                [utils::serialize(&block_count), last_block.clone(), reward_percentiles.clone()],
            )
        {
            success @ Ok(_) => success,
            err @ Err(_) => {
                let fallback = self
                    .request_sync::<_, FeeHistory>(
                        "eth_feeHistory",
                        [utils::serialize(&block_count.as_u64()), last_block, reward_percentiles],
                    );

                if fallback.is_err() {
                    // if the older fallback also resulted in an error, we return the error from the
                    // initial attempt
                    return err
                }
                fallback
            }
        }
    }

    fn fill_gas_fees_sync(&self, tx: &mut TypedTransaction) -> Result<(), ProviderError> {
        match tx {
            TypedTransaction::Eip2930(_) | TypedTransaction::Legacy(_) => {
                let gas_price = if tx.gas_price().is_some() {
                    tx.gas_price().unwrap()
                } else {
                    self.get_gas_price_sync()?
                };
                tx.set_gas_price(gas_price);
            }
            TypedTransaction::Eip1559(ref mut inner) => {
                if inner.max_fee_per_gas.is_none() || inner.max_priority_fee_per_gas.is_none() {
                    let (max_fee_per_gas, max_priority_fee_per_gas) =
                        self.estimate_eip1559_fees_sync(None)?;
                    inner.max_fee_per_gas = Some(max_fee_per_gas);
                    inner.max_priority_fee_per_gas = Some(max_priority_fee_per_gas);
                };
            }
        }
        Ok(())
    }

    /// Gets the current gas price as estimated by the node
    fn get_gas_price_sync(&self) -> Result<U256, ProviderError> {
        self.request_sync("eth_gasPrice", ())
    }

    /// Gets a heuristic recommendation of max fee per gas and max priority fee per gas for
    /// EIP-1559 compatible transactions.
    fn estimate_eip1559_fees_sync(
        &self,
        estimator: Option<fn(U256, Vec<Vec<U256>>) -> (U256, U256)>,
    ) -> Result<(U256, U256), ProviderError> {
        let base_fee_per_gas = self
            .get_block_sync(BlockNumber::Latest)?
            .ok_or_else(|| ProviderError::CustomError("Latest block not found".into()))?
            .base_fee_per_gas
            .ok_or_else(|| ProviderError::CustomError("EIP-1559 not activated".into()))?;

        let fee_history = self
            .fee_history_sync(
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
    fn estimate_gas_sync(
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
        self.request_sync("eth_estimateGas", params)
    }

    fn fill_transaction_sync(
        &self,
        tx: &mut TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<(), ProviderError> {
        if let Some(default_sender) = self.default_sender() {
            if tx.from().is_none() {
                tx.set_from(default_sender);
            }
        }

        // set the ENS name
        if let Some(NameOrAddress::Name(ref ens_name)) = tx.to() {
            let addr = self.resolve_name_sync(&ens_name, None)?;
            tx.set_to(addr);
        }

        // fill gas price
        self.fill_gas_fees_sync(tx)?;

        // Set gas to estimated value only if it was not set by the caller,
        // even if the access list has been populated and saves gas
        if tx.gas().is_none() {
            let gas_estimate = self.estimate_gas_sync(tx, block)?;
            tx.set_gas(gas_estimate);
        }

        Ok(())
    }

    /// Returns the currently configured chain id, a value used in replay-protected
    /// transaction signing as introduced by EIP-155.
    fn get_chainid_sync(&self) -> Result<U256, ProviderError> {
        self.request_sync("eth_chainId", ())
    }

    /// Returns the account's balance
    fn get_balance_sync<T: Into<NameOrAddress> + Send + Sync>(
        &self,
        from: T,
        block: Option<BlockId>,
    ) -> Result<U256, ProviderError> {
        let from = match from.into() {
            NameOrAddress::Name(ens_name) => self.resolve_name_sync(&ens_name, None)?,
            NameOrAddress::Address(addr) => addr,
        };

        let from = utils::serialize(&from);
        let block = utils::serialize(&block.unwrap_or_else(|| BlockNumber::Latest.into()));
        self.request_sync("eth_getBalance", [from, block])
    }

    /// Gets the transaction with `transaction_hash`
    fn get_transaction_sync<T: Send + Sync + Into<TxHash>>(
        &self,
        transaction_hash: T,
    ) -> Result<Option<Transaction>, ProviderError> {
        let hash = transaction_hash.into();
        self.request_sync("eth_getTransactionByHash", [hash])
    }

    /// Gets the transaction receipt with `transaction_hash`
    fn get_transaction_receipt_sync<T: Send + Sync + Into<TxHash>>(
        &self,
        transaction_hash: T,
    ) -> Result<Option<TransactionReceipt>, ProviderError> {
        let hash = transaction_hash.into();
        self.request_sync("eth_getTransactionReceipt", [hash])
    }

    /// Sends the read-only (constant) transaction to a single Ethereum node and return the result
    /// (as bytes) of executing it. This is free, since it does not change any state on the
    /// blockchain.
    fn call_sync(
        &self,
        tx: &TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<Bytes, ProviderError> {
        let tx = utils::serialize(tx);
        let block = utils::serialize(&block.unwrap_or_else(|| BlockNumber::Latest.into()));
        self.request_sync("eth_call", [tx, block])
    }

    /// Returns the address that the `ens_name` resolves to (or None if not configured).
    ///
    /// # Panics
    ///
    /// If the bytes returned from the ENS registrar/resolver cannot be interpreted as
    /// an address. This should theoretically never happen.
    fn resolve_name_sync(&self, ens_name: &str, ens_address: Option<Address>) -> Result<Address, ProviderError> {
        self.query_resolver_parameters_sync(ParamType::Address, ens_name, ens::ADDR_SELECTOR, None, ens_address)
    }

    fn query_resolver_parameters_sync<T: Detokenize>(
        &self,
        param: ParamType,
        ens_name: &str,
        selector: Selector,
        parameters: Option<&[u8]>,
        ens_address: Option<Address>,
    ) -> Result<T, ProviderError> {
        // Get the ENS address, prioritize the local override variable
        let ens_addr = ens_address.unwrap_or(ens::ENS_ADDRESS);

        // first get the resolver responsible for this name
        // the call will return a Bytes array which we convert to an address
        let data = self.call_sync(&ens::get_resolver(ens_addr, ens_name).into(), None)?;

        // otherwise, decode_bytes panics
        if data.0.is_empty() {
            return Err(ProviderError::EnsError(ens_name.to_string()))
        }

        let resolver_address: Address = decode_bytes(ParamType::Address, data);
        if resolver_address == Address::zero() {
            return Err(ProviderError::EnsError(ens_name.to_string()))
        }

        if let ParamType::Address = param {
            // Reverse resolver reverts when calling `supportsInterface(bytes4)`
            self.validate_resolver_sync(resolver_address, selector, ens_name)?;
        }

        // resolve
        let data = self.call_sync(&ens::resolve(resolver_address, selector, ens_name, parameters).into(), None)?;

        Ok(decode_bytes(param, data))
    }

    /// Validates that the resolver supports `selector`.
    fn validate_resolver_sync(
        &self,
        resolver_address: Address,
        selector: Selector,
        ens_name: &str,
    ) -> Result<(), ProviderError> {
        let data =
            self.call_sync(&ens::supports_interface(resolver_address, selector).into(), None)?;

        if data.is_empty() {
            return Err(ProviderError::EnsError(format!(
                "`{}` resolver ({:?}) is invalid.",
                ens_name, resolver_address
            )))
        }

        let supports_selector = abi::decode(&[ParamType::Bool], data.as_ref())
            .map(|token| token[0].clone().into_bool().unwrap_or_default())
            .unwrap_or_default();

        if !supports_selector {
            return Err(ProviderError::EnsError(format!(
                "`{}` resolver ({:?}) does not support selector {}.",
                ens_name,
                resolver_address,
                hex::encode(selector)
            )))
        }

        Ok(())
    }
}
