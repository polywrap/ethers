pub mod entry;
pub mod tx_request;
pub use tx_request::TxRequest;
pub mod access_item;
pub use access_item::AccessItem;
pub mod tx_response;
pub use tx_response::TxResponse;
pub mod log;
pub use log::Log;
pub mod tx_receipt;
pub use tx_receipt::TxReceipt;
pub mod tx_options;
pub use tx_options::TxOptions;
pub mod static_tx_result;
pub use static_tx_result::StaticTxResult;
pub mod eip1559_fees_estimate;
pub use eip1559_fees_estimate::Eip1559FeesEstimate;
pub mod imported;
pub use imported::provider_connection::ProviderConnection;
pub use imported::provider_module::ProviderModule;
pub mod module;
pub use module::{
    deserialize_get_chain_id_args,
    serialize_get_chain_id_result,
    get_chain_id_wrapped,
    ArgsGetChainId,
    deserialize_call_contract_view_args,
    serialize_call_contract_view_result,
    call_contract_view_wrapped,
    ArgsCallContractView,
    deserialize_call_contract_static_args,
    serialize_call_contract_static_result,
    call_contract_static_wrapped,
    ArgsCallContractStatic,
    deserialize_encode_params_args,
    serialize_encode_params_result,
    encode_params_wrapped,
    ArgsEncodeParams,
    deserialize_encode_function_args,
    serialize_encode_function_result,
    encode_function_wrapped,
    ArgsEncodeFunction,
    deserialize_decode_function_args,
    serialize_decode_function_result,
    decode_function_wrapped,
    ArgsDecodeFunction,
    deserialize_get_signer_address_args,
    serialize_get_signer_address_result,
    get_signer_address_wrapped,
    ArgsGetSignerAddress,
    deserialize_get_signer_balance_args,
    serialize_get_signer_balance_result,
    get_signer_balance_wrapped,
    ArgsGetSignerBalance,
    deserialize_get_balance_args,
    serialize_get_balance_result,
    get_balance_wrapped,
    ArgsGetBalance,
    deserialize_get_gas_price_args,
    serialize_get_gas_price_result,
    get_gas_price_wrapped,
    ArgsGetGasPrice,
    deserialize_estimate_eip1559_fees_args,
    serialize_estimate_eip1559_fees_result,
    estimate_eip1559_fees_wrapped,
    ArgsEstimateEip1559Fees,
    deserialize_send_rpc_args,
    serialize_send_rpc_result,
    send_rpc_wrapped,
    ArgsSendRpc,
    deserialize_get_signer_transaction_count_args,
    serialize_get_signer_transaction_count_result,
    get_signer_transaction_count_wrapped,
    ArgsGetSignerTransactionCount,
    deserialize_check_address_args,
    serialize_check_address_result,
    check_address_wrapped,
    ArgsCheckAddress,
    deserialize_to_wei_args,
    serialize_to_wei_result,
    to_wei_wrapped,
    ArgsToWei,
    deserialize_to_eth_args,
    serialize_to_eth_result,
    to_eth_wrapped,
    ArgsToEth,
    deserialize_estimate_transaction_gas_args,
    serialize_estimate_transaction_gas_result,
    estimate_transaction_gas_wrapped,
    ArgsEstimateTransactionGas,
    deserialize_await_transaction_args,
    serialize_await_transaction_result,
    await_transaction_wrapped,
    ArgsAwaitTransaction,
    deserialize_send_transaction_args,
    serialize_send_transaction_result,
    send_transaction_wrapped,
    ArgsSendTransaction,
    deserialize_send_transaction_and_wait_args,
    serialize_send_transaction_and_wait_result,
    send_transaction_and_wait_wrapped,
    ArgsSendTransactionAndWait,
    deserialize_deploy_contract_args,
    serialize_deploy_contract_result,
    deploy_contract_wrapped,
    ArgsDeployContract,
    deserialize_estimate_contract_call_gas_args,
    serialize_estimate_contract_call_gas_result,
    estimate_contract_call_gas_wrapped,
    ArgsEstimateContractCallGas,
    deserialize_call_contract_method_args,
    serialize_call_contract_method_result,
    call_contract_method_wrapped,
    ArgsCallContractMethod,
    deserialize_call_contract_method_and_wait_args,
    serialize_call_contract_method_and_wait_result,
    call_contract_method_and_wait_wrapped,
    ArgsCallContractMethodAndWait,
    deserialize_sign_message_args,
    serialize_sign_message_result,
    sign_message_wrapped,
    ArgsSignMessage
};

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
