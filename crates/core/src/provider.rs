use ethers_core::types::{NameOrAddress, BlockId};

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