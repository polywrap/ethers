use polywrap_wasm_rs::{
  wrap_load_env
};

use crate::{
    get_chain_id,
    ArgsGetChainId,
    deserialize_get_chain_id_args,
    serialize_get_chain_id_result,
    call_contract_view,
    ArgsCallContractView,
    deserialize_call_contract_view_args,
    serialize_call_contract_view_result,
    call_contract_static,
    ArgsCallContractStatic,
    deserialize_call_contract_static_args,
    serialize_call_contract_static_result,
    encode_params,
    ArgsEncodeParams,
    deserialize_encode_params_args,
    serialize_encode_params_result,
    encode_function,
    ArgsEncodeFunction,
    deserialize_encode_function_args,
    serialize_encode_function_result,
    decode_function,
    ArgsDecodeFunction,
    deserialize_decode_function_args,
    serialize_decode_function_result,
    get_signer_address,
    ArgsGetSignerAddress,
    deserialize_get_signer_address_args,
    serialize_get_signer_address_result,
    get_signer_balance,
    ArgsGetSignerBalance,
    deserialize_get_signer_balance_args,
    serialize_get_signer_balance_result,
    get_balance,
    ArgsGetBalance,
    deserialize_get_balance_args,
    serialize_get_balance_result,
    get_gas_price,
    ArgsGetGasPrice,
    deserialize_get_gas_price_args,
    serialize_get_gas_price_result,
    estimate_eip1559_fees,
    ArgsEstimateEip1559Fees,
    deserialize_estimate_eip1559_fees_args,
    serialize_estimate_eip1559_fees_result,
    send_rpc,
    ArgsSendRpc,
    deserialize_send_rpc_args,
    serialize_send_rpc_result,
    get_signer_transaction_count,
    ArgsGetSignerTransactionCount,
    deserialize_get_signer_transaction_count_args,
    serialize_get_signer_transaction_count_result,
    check_address,
    ArgsCheckAddress,
    deserialize_check_address_args,
    serialize_check_address_result,
    to_wei,
    ArgsToWei,
    deserialize_to_wei_args,
    serialize_to_wei_result,
    to_eth,
    ArgsToEth,
    deserialize_to_eth_args,
    serialize_to_eth_result,
    estimate_transaction_gas,
    ArgsEstimateTransactionGas,
    deserialize_estimate_transaction_gas_args,
    serialize_estimate_transaction_gas_result,
    await_transaction,
    ArgsAwaitTransaction,
    deserialize_await_transaction_args,
    serialize_await_transaction_result,
    send_transaction,
    ArgsSendTransaction,
    deserialize_send_transaction_args,
    serialize_send_transaction_result,
    send_transaction_and_wait,
    ArgsSendTransactionAndWait,
    deserialize_send_transaction_and_wait_args,
    serialize_send_transaction_and_wait_result,
    deploy_contract,
    ArgsDeployContract,
    deserialize_deploy_contract_args,
    serialize_deploy_contract_result,
    estimate_contract_call_gas,
    ArgsEstimateContractCallGas,
    deserialize_estimate_contract_call_gas_args,
    serialize_estimate_contract_call_gas_result,
    call_contract_method,
    ArgsCallContractMethod,
    deserialize_call_contract_method_args,
    serialize_call_contract_method_result,
    call_contract_method_and_wait,
    ArgsCallContractMethodAndWait,
    deserialize_call_contract_method_and_wait_args,
    serialize_call_contract_method_and_wait_result,
    sign_message,
    ArgsSignMessage,
    deserialize_sign_message_args,
    serialize_sign_message_result
};


pub fn get_chain_id_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_get_chain_id_args(args) {
        Ok(args) => {
            let result = get_chain_id(ArgsGetChainId {
                connection: args.connection,
            });
            serialize_get_chain_id_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn call_contract_view_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_call_contract_view_args(args) {
        Ok(args) => {
            let result = call_contract_view(ArgsCallContractView {
                address: args.address,
                method: args.method,
                args: args.args,
                connection: args.connection,
            });
            serialize_call_contract_view_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn call_contract_static_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_call_contract_static_args(args) {
        Ok(args) => {
            let result = call_contract_static(ArgsCallContractStatic {
                address: args.address,
                method: args.method,
                args: args.args,
                options: args.options,
                connection: args.connection,
            });
            serialize_call_contract_static_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn encode_params_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_encode_params_args(args) {
        Ok(args) => {
            let result = encode_params(ArgsEncodeParams {
                types: args.types,
                values: args.values,
                connection: args.connection,
            });
            serialize_encode_params_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn encode_function_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_encode_function_args(args) {
        Ok(args) => {
            let result = encode_function(ArgsEncodeFunction {
                method: args.method,
                args: args.args,
                connection: args.connection,
            });
            serialize_encode_function_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn decode_function_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_decode_function_args(args) {
        Ok(args) => {
            let result = decode_function(ArgsDecodeFunction {
                method: args.method,
                data: args.data,
                connection: args.connection,
            });
            serialize_decode_function_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn get_signer_address_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_get_signer_address_args(args) {
        Ok(args) => {
            let result = get_signer_address(ArgsGetSignerAddress {
                connection: args.connection,
            });
            serialize_get_signer_address_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn get_signer_balance_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_get_signer_balance_args(args) {
        Ok(args) => {
            let result = get_signer_balance(ArgsGetSignerBalance {
                block_tag: args.block_tag,
                connection: args.connection,
            });
            serialize_get_signer_balance_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn get_balance_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_get_balance_args(args) {
        Ok(args) => {
            let result = get_balance(ArgsGetBalance {
                address: args.address,
                block_tag: args.block_tag,
                connection: args.connection,
            });
            serialize_get_balance_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn get_gas_price_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_get_gas_price_args(args) {
        Ok(args) => {
            let result = get_gas_price(ArgsGetGasPrice {
                connection: args.connection,
            });
            serialize_get_gas_price_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn estimate_eip1559_fees_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_estimate_eip1559_fees_args(args) {
        Ok(args) => {
            let result = estimate_eip1559_fees(ArgsEstimateEip1559Fees {
                connection: args.connection,
            });
            serialize_estimate_eip1559_fees_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn send_rpc_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_send_rpc_args(args) {
        Ok(args) => {
            let result = send_rpc(ArgsSendRpc {
                method: args.method,
                params: args.params,
                connection: args.connection,
            });
            serialize_send_rpc_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn get_signer_transaction_count_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_get_signer_transaction_count_args(args) {
        Ok(args) => {
            let result = get_signer_transaction_count(ArgsGetSignerTransactionCount {
                block_tag: args.block_tag,
                connection: args.connection,
            });
            serialize_get_signer_transaction_count_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn check_address_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_check_address_args(args) {
        Ok(args) => {
            let result = check_address(ArgsCheckAddress {
                address: args.address,
                connection: args.connection,
            });
            serialize_check_address_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn to_wei_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_to_wei_args(args) {
        Ok(args) => {
            let result = to_wei(ArgsToWei {
                eth: args.eth,
            });
            serialize_to_wei_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn to_eth_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_to_eth_args(args) {
        Ok(args) => {
            let result = to_eth(ArgsToEth {
                wei: args.wei,
            });
            serialize_to_eth_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn estimate_transaction_gas_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_estimate_transaction_gas_args(args) {
        Ok(args) => {
            let result = estimate_transaction_gas(ArgsEstimateTransactionGas {
                tx: args.tx,
                connection: args.connection,
            });
            serialize_estimate_transaction_gas_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn await_transaction_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_await_transaction_args(args) {
        Ok(args) => {
            let result = await_transaction(ArgsAwaitTransaction {
                tx_hash: args.tx_hash,
                connection: args.connection,
            });
            serialize_await_transaction_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn send_transaction_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_send_transaction_args(args) {
        Ok(args) => {
            let result = send_transaction(ArgsSendTransaction {
                tx: args.tx,
                connection: args.connection,
            });
            serialize_send_transaction_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn send_transaction_and_wait_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_send_transaction_and_wait_args(args) {
        Ok(args) => {
            let result = send_transaction_and_wait(ArgsSendTransactionAndWait {
                tx: args.tx,
                connection: args.connection,
            });
            serialize_send_transaction_and_wait_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn deploy_contract_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_deploy_contract_args(args) {
        Ok(args) => {
            let result = deploy_contract(ArgsDeployContract {
                abi: args.abi,
                bytecode: args.bytecode,
                args: args.args,
                options: args.options,
                connection: args.connection,
            });
            serialize_deploy_contract_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn estimate_contract_call_gas_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_estimate_contract_call_gas_args(args) {
        Ok(args) => {
            let result = estimate_contract_call_gas(ArgsEstimateContractCallGas {
                address: args.address,
                method: args.method,
                args: args.args,
                options: args.options,
                connection: args.connection,
            });
            serialize_estimate_contract_call_gas_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn call_contract_method_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_call_contract_method_args(args) {
        Ok(args) => {
            let result = call_contract_method(ArgsCallContractMethod {
                address: args.address,
                method: args.method,
                args: args.args,
                options: args.options,
                connection: args.connection,
            });
            serialize_call_contract_method_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn call_contract_method_and_wait_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_call_contract_method_and_wait_args(args) {
        Ok(args) => {
            let result = call_contract_method_and_wait(ArgsCallContractMethodAndWait {
                address: args.address,
                method: args.method,
                args: args.args,
                options: args.options,
                connection: args.connection,
            });
            serialize_call_contract_method_and_wait_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

pub fn sign_message_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_sign_message_args(args) {
        Ok(args) => {
            let result = sign_message(ArgsSignMessage {
                message: args.message,
                connection: args.connection,
            });
            serialize_sign_message_result(&result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}
