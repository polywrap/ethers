use crate::{
    get_chain_id_wrapped,
    call_contract_view_wrapped,
    call_contract_static_wrapped,
    encode_params_wrapped,
    encode_function_wrapped,
    decode_function_wrapped,
    get_signer_address_wrapped,
    get_signer_balance_wrapped,
    get_balance_wrapped,
    get_gas_price_wrapped,
    estimate_eip1559_fees_wrapped,
    send_rpc_wrapped,
    get_signer_transaction_count_wrapped,
    check_address_wrapped,
    to_wei_wrapped,
    to_eth_wrapped,
    estimate_transaction_gas_wrapped,
    await_transaction_wrapped,
    send_transaction_wrapped,
    send_transaction_and_wait_wrapped,
    deploy_contract_wrapped,
    estimate_contract_call_gas_wrapped,
    call_contract_method_wrapped,
    call_contract_method_and_wait_wrapped,
    sign_message_wrapped
};
use polywrap_wasm_rs::{
    abort,
    invoke,
    InvokeArgs,
};

#[no_mangle]
pub extern "C" fn _wrap_invoke(method_size: u32, args_size: u32, env_size: u32) -> bool {
    // Ensure the abort handler is properly setup
    abort::wrap_abort_setup();

    let args: InvokeArgs = invoke::wrap_invoke_args(method_size, args_size);

    match args.method.as_str() {
        "getChainId" => invoke::wrap_invoke(args, env_size, Some(get_chain_id_wrapped)),
        "callContractView" => invoke::wrap_invoke(args, env_size, Some(call_contract_view_wrapped)),
        "callContractStatic" => invoke::wrap_invoke(args, env_size, Some(call_contract_static_wrapped)),
        "encodeParams" => invoke::wrap_invoke(args, env_size, Some(encode_params_wrapped)),
        "encodeFunction" => invoke::wrap_invoke(args, env_size, Some(encode_function_wrapped)),
        "decodeFunction" => invoke::wrap_invoke(args, env_size, Some(decode_function_wrapped)),
        "getSignerAddress" => invoke::wrap_invoke(args, env_size, Some(get_signer_address_wrapped)),
        "getSignerBalance" => invoke::wrap_invoke(args, env_size, Some(get_signer_balance_wrapped)),
        "getBalance" => invoke::wrap_invoke(args, env_size, Some(get_balance_wrapped)),
        "getGasPrice" => invoke::wrap_invoke(args, env_size, Some(get_gas_price_wrapped)),
        "estimateEip1559Fees" => invoke::wrap_invoke(args, env_size, Some(estimate_eip1559_fees_wrapped)),
        "sendRpc" => invoke::wrap_invoke(args, env_size, Some(send_rpc_wrapped)),
        "getSignerTransactionCount" => invoke::wrap_invoke(args, env_size, Some(get_signer_transaction_count_wrapped)),
        "checkAddress" => invoke::wrap_invoke(args, env_size, Some(check_address_wrapped)),
        "toWei" => invoke::wrap_invoke(args, env_size, Some(to_wei_wrapped)),
        "toEth" => invoke::wrap_invoke(args, env_size, Some(to_eth_wrapped)),
        "estimateTransactionGas" => invoke::wrap_invoke(args, env_size, Some(estimate_transaction_gas_wrapped)),
        "awaitTransaction" => invoke::wrap_invoke(args, env_size, Some(await_transaction_wrapped)),
        "sendTransaction" => invoke::wrap_invoke(args, env_size, Some(send_transaction_wrapped)),
        "sendTransactionAndWait" => invoke::wrap_invoke(args, env_size, Some(send_transaction_and_wait_wrapped)),
        "deployContract" => invoke::wrap_invoke(args, env_size, Some(deploy_contract_wrapped)),
        "estimateContractCallGas" => invoke::wrap_invoke(args, env_size, Some(estimate_contract_call_gas_wrapped)),
        "callContractMethod" => invoke::wrap_invoke(args, env_size, Some(call_contract_method_wrapped)),
        "callContractMethodAndWait" => invoke::wrap_invoke(args, env_size, Some(call_contract_method_and_wait_wrapped)),
        "signMessage" => invoke::wrap_invoke(args, env_size, Some(sign_message_wrapped)),
        _ => invoke::wrap_invoke(args, env_size, None),
    }
}
