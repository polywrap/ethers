use ethers_core::abi::Abi;
use ethers_core::types::{Address, BlockId, BlockNumber, Bytes, H256};
use polywrap_provider::provider::{Provider, WrapProvider};
use polywrap_provider::signer::{Signer, WrapSigner};
use polywrap_wasm_rs::{BigInt, BigIntWrapper, JSON};
use std::str::FromStr;
use wrap::module::{Module, ModuleTrait};
mod wrap;
use wrap::prelude::*;

mod polywrap_provider;

use helpers::{format, mapping};
use polywrap_provider::provider;

mod api;
mod helpers;

impl ModuleTrait for Module {
    fn get_chain_id(args: ArgsGetChainId) -> Result<String, String> {
        let provider = WrapProvider::new(&args.connection);
        Ok(provider.get_chainid().unwrap().to_string())
    }

    fn get_balance(args: ArgsGetBalance) -> Result<BigIntWrapper, String> {
        let provider = WrapProvider::new(&args.connection);
        let address = match Address::from_str(&args.address) {
            Ok(addr) => Ok(addr),
            Err(e) => Err(format!("Invalid address: {}. Error: {}", &args.address, e)),
        };

        if address.is_err() {
            return Err(address.unwrap_err());
        }
        let block_tag: BlockId = BlockNumber::Latest.into();
        let balance = provider.get_balance(address.unwrap(), Some(block_tag));

        if let Err(error) = balance {
            return Err(format!("Error in get_balance: {}", error.to_string()));
        }
        Ok(BigIntWrapper(
            BigInt::from_str(&balance.unwrap().to_string()).unwrap(),
        ))
    }

    fn get_transaction(args: ArgsGetTransaction) -> Result<TxResponse, String> {
        let provider = WrapProvider::new(&args.connection);
        let transaction = provider.get_transaction(H256::from_str(&args.hash).unwrap());
        if let Ok(tx) = transaction {
            if let Some(tx) = tx {
                Ok(mapping::to_wrap_response(&provider, tx))
            } else {
                return Err(format!("Transaction with hash {} not found", args.hash));
            }
        } else {
            return Err("Error fetching transaction".to_string());
        }
    }

    fn check_address(args: ArgsCheckAddress) -> Result<bool, String> {
        Ok(match Address::from_str(&args.address) {
            Ok(_) => true,
            Err(_) => false,
        })
    }

    fn get_gas_price(args: ArgsGetGasPrice) -> Result<BigIntWrapper, String> {
        let provider = WrapProvider::new(&args.connection);
        let price = provider.get_gas_price().unwrap();
        Ok(BigIntWrapper(BigInt::from_str(&price.to_string()).unwrap()))
    }

    fn estimate_eip1559_fees(args: ArgsEstimateEip1559Fees) -> Result<Eip1559FeesEstimate, String> {
        let provider = WrapProvider::new(&args.connection);
        let price = provider.estimate_eip1559_fees(None).unwrap();
        Ok(Eip1559FeesEstimate {
            max_fee_per_gas: BigIntWrapper(BigInt::from_str(&price.0.to_string()).unwrap()),
            max_priority_fee_per_gas: BigIntWrapper(
                BigInt::from_str(&price.1.to_string()).unwrap(),
            ),
        })
    }

    fn get_signer_address(args: ArgsGetSignerAddress) -> Result<String, String> {
        let address = WrapSigner::new(&args.connection).address();
        Ok(format!("{:#x}", address))
    }

    fn get_signer_balance(args: ArgsGetSignerBalance) -> Result<BigIntWrapper, String> {
        let provider = WrapProvider::new(&args.connection);
        let address = WrapSigner::new(&args.connection).address();
        let block_tag: BlockId = BlockNumber::Latest.into();
        let balance = provider.get_balance(address, Some(block_tag)).unwrap();
        Ok(BigIntWrapper(
            BigInt::from_str(&balance.to_string()).unwrap(),
        ))
    }

    fn get_signer_transaction_count(
        args: ArgsGetSignerTransactionCount,
    ) -> Result<BigIntWrapper, String> {
        let provider = WrapProvider::new(&args.connection);
        let address = WrapSigner::new(&args.connection).address();
        let block_tag: BlockId = BlockNumber::Latest.into();
        let count = provider
            .get_transaction_count(address, Some(block_tag))
            .unwrap();
        Ok(BigIntWrapper(BigInt::from_str(&count.to_string()).unwrap()))
    }

    fn sign_message(args: ArgsSignMessage) -> Result<String, String> {
        let signer = WrapSigner::new(&args.connection);
        let signature = signer.sign_message(&args.message).unwrap();
        let bytes: Bytes = signature.to_vec().into();
        Ok(format!("{}", bytes).to_string())
    }

    fn sign_message_bytes(args: ArgsSignMessageBytes) -> Result<String, String> {
        let signer = WrapSigner::new(&args.connection);
        let signature = signer.sign_message(&args.bytes).unwrap();
        let bytes: Bytes = signature.to_vec().into();
        Ok(format!("{}", bytes).to_string())
    }

    fn sign_transaction(args: ArgsSignTransaction) -> Result<String, String> {
        let signer = WrapSigner::new(&args.connection);
        let tx = mapping::from_wrap_request(args.tx);
        let signature = signer.sign_transaction(&tx).unwrap();
        let bytes: Bytes = signature.to_vec().into();
        Ok(format!("{}", bytes).to_string())
    }

    fn sign_typed_data(args: ArgsSignTypedData) -> Result<String, String> {
        let address = WrapSigner::new(&args.connection).address();
        let address_value = JSON::Value::String(format!("{:#x}", address));
        let params = JSON::Value::Array(vec![address_value, args.payload.into()]);
        let provider = WrapProvider::new(&args.connection);
        provider
            .request("eth_signTypedData_v4", params)
            .map_err(|e| format!("Error in sign_typed_data method: {}", e.to_string()))
    }

    fn send_rpc(args: ArgsSendRpc) -> Result<String, String> {
        let provider = WrapProvider::new(&args.connection);
        let res: serde_json::Value = provider.request(&args.method, args.params).unwrap();
        Ok(res.to_string())
    }

    fn estimate_transaction_gas(args: ArgsEstimateTransactionGas) -> Result<BigIntWrapper, String> {
        let provider = WrapProvider::new(&args.connection);
        let tx = mapping::from_wrap_request(args.tx);
        let gas = provider.estimate_gas(&tx, None).unwrap();
        Ok(BigIntWrapper(BigInt::from_str(&gas.to_string()).unwrap()))
    }

    fn await_transaction(args: ArgsAwaitTransaction) -> Result<TxReceipt, String> {
        let provider = WrapProvider::new(&args.connection);
        let tx_hash = H256::from_str(&args.tx_hash).unwrap();
        provider
            .await_transaction(tx_hash.clone(), args.confirmations, args.timeout)
            .unwrap();
        let receipt = provider.get_transaction_receipt(tx_hash).unwrap().unwrap();
        let tx_receipt = mapping::to_wrap_receipt(receipt, args.confirmations);
        Ok(tx_receipt)
    }

    fn send_transaction(args: ArgsSendTransaction) -> Result<TxResponse, String> {
        let provider = WrapProvider::new(&args.connection);
        let signer = WrapSigner::new(&args.connection);

        let mut tx = mapping::from_wrap_request(args.tx);

        let tx_hash = api::send_transaction(&provider, &signer, &mut tx);
        let response = provider.get_transaction(tx_hash).unwrap().unwrap();
        let tx_response = mapping::to_wrap_response(&provider, response);
        Ok(tx_response)
    }

    fn send_transaction_and_wait(args: ArgsSendTransactionAndWait) -> Result<TxReceipt, String> {
        let provider = WrapProvider::new(&args.connection);
        let signer = WrapSigner::new(&args.connection);

        let mut tx = mapping::from_wrap_request(args.tx);

        let tx_hash = api::send_transaction(&provider, &signer, &mut tx);
        provider
            .await_transaction(tx_hash.clone(), 1, None)
            .unwrap();
        let receipt = provider.get_transaction_receipt(tx_hash).unwrap().unwrap();
        let tx_receipt = mapping::to_wrap_receipt(receipt, 1);
        Ok(tx_receipt)
    }

    fn deploy_contract(args: ArgsDeployContract) -> Result<String, String> {
        let provider = WrapProvider::new(&args.connection);
        let signer = WrapSigner::new(&args.connection);

        let abi: Abi = serde_json::from_str(&args.abi).unwrap();
        let bytecode = Bytes::from_str(&args.bytecode).unwrap();
        let params: Vec<String> = args.args.unwrap_or(vec![]);
        let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

        let mut tx =
            api::create_deploy_contract_transaction(&abi, bytecode, &params, &tx_options).unwrap();

        let tx_hash = api::send_transaction(&provider, &signer, &mut tx);
        provider
            .await_transaction(tx_hash.clone(), 1, None)
            .unwrap();
        let receipt = provider.get_transaction_receipt(tx_hash).unwrap().unwrap();
        let address = receipt
            .contract_address
            .expect("Contract failed to deploy.");
        Ok(format!("{:#x}", address))
    }

    fn estimate_contract_call_gas(
        args: ArgsEstimateContractCallGas,
    ) -> Result<BigIntWrapper, String> {
        let provider = WrapProvider::new(&args.connection);
        let signer = WrapSigner::new(&args.connection);

        let address = match Address::from_str(&args.address) {
            Ok(addr) => addr,
            Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
        };
        let params: Vec<String> = args.args.unwrap_or(vec![]);
        let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

        let gas = api::estimate_contract_call_gas(
            &provider,
            &signer,
            address,
            &args.method,
            &params,
            &tx_options,
        );
        {
            let this = BigInt::from_str(&gas.to_string());
            match this {
                Ok(t) => Ok(BigIntWrapper(t)),
                Err(e) => Err((|e: polywrap_wasm_rs::DecodeError| e.to_string())(e.into())),
            }
        }
    }

    fn call_contract_view(args: ArgsCallContractView) -> Result<String, String> {
        let provider = WrapProvider::new(&args.connection);

        let address = match Address::from_str(&args.address) {
            Ok(addr) => addr,
            Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
        };
        let params: Vec<String> = args.args.unwrap_or(vec![]);

        let tokens = api::call_contract_view(&provider, address, &args.method, &params);
        Ok(format::format_tokens(&tokens))
    }

    fn call_contract_static(args: ArgsCallContractStatic) -> Result<StaticTxResult, String> {
        let provider = WrapProvider::new(&args.connection);
        let signer = WrapSigner::new(&args.connection);

        let address = match Address::from_str(&args.address) {
            Ok(addr) => addr,
            Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
        };
        let params: Vec<String> = args.args.unwrap_or(vec![]);
        let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

        let result = api::call_contract_static(
            &provider,
            &signer,
            address,
            &args.method,
            &params,
            &tx_options,
        );
        Ok(match result {
            Ok(tokens) => StaticTxResult {
                result: format::format_tokens(&tokens),
                error: false,
            },
            Err(e) => StaticTxResult {
                result: e.to_string(),
                error: true,
            },
        })
    }

    fn call_contract_method(args: ArgsCallContractMethod) -> Result<TxResponse, String> {
        let provider = WrapProvider::new(&args.connection);
        let signer = WrapSigner::new(&args.connection);

        let address = match Address::from_str(&args.address) {
            Ok(addr) => addr,
            Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
        };
        let params: Vec<String> = args.args.unwrap_or(vec![]);
        let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

        let tx_hash = api::call_contract_method(
            &provider,
            &signer,
            address,
            &args.method,
            &params,
            &tx_options,
        )?;

        let response = provider.get_transaction(tx_hash).unwrap().unwrap();
        let tx_response = mapping::to_wrap_response(&provider, response);
        Ok(tx_response)
    }

    fn call_contract_method_and_wait(
        args: ArgsCallContractMethodAndWait,
    ) -> Result<TxReceipt, String> {
        let provider = WrapProvider::new(&args.connection);
        let signer = WrapSigner::new(&args.connection);

        let address = match Address::from_str(&args.address) {
            Ok(addr) => addr,
            Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
        };
        let params: Vec<String> = args.args.unwrap_or(vec![]);
        let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

        let tx_hash = api::call_contract_method(
            &provider,
            &signer,
            address,
            &args.method,
            &params,
            &tx_options,
        )?;
        provider
            .await_transaction(tx_hash.clone(), 1, None)
            .unwrap();
        let receipt = provider.get_transaction_receipt(tx_hash).unwrap().unwrap();
        let tx_receipt = mapping::to_wrap_receipt(receipt, 1);
        Ok(tx_receipt)
    }

    // Re-export utils
    fn keccak256(args: ArgsKeccak256) -> Result<String, String> {
        UtilsModule::keccak256(&wrap::imported::utils_module::ArgsKeccak256 { value: args.value })
    }

    fn keccak256_bytes_encode_packed(
        args: ArgsKeccak256BytesEncodePacked,
    ) -> Result<String, String> {
        UtilsModule::keccak256_bytes_encode_packed(
            &wrap::imported::utils_module::ArgsKeccak256BytesEncodePacked { value: args.value },
        )
    }

    fn generate_create2_address(args: ArgsGenerateCreate2Address) -> Result<String, String> {
        UtilsModule::generate_create2_address(&wrap::imported::utils_module::ArgsGenerateCreate2Address {
            address: args.address,
            salt: args.salt,
            init_code: args.init_code,
        })
    }

    fn encode_meta_transaction(args: ArgsEncodeMetaTransaction) -> Result<String, String> {
        UtilsModule::encode_meta_transaction(&wrap::imported::utils_module::ArgsEncodeMetaTransaction {
            operation: args.operation,
            to: args.to,
            value: args.value,
            data: args.data,
        })
    }

    fn encode_params(args: ArgsEncodeParams) -> Result<String, String> {
        UtilsModule::encode_params(&wrap::imported::utils_module::ArgsEncodeParams {
            types: args.types,
            values: args.values,
        })
    }

    fn encode_function(args: ArgsEncodeFunction) -> Result<String, String> {
        UtilsModule::encode_function(&wrap::imported::utils_module::ArgsEncodeFunction {
            method: args.method,
            args: args.args,
        })
    }

    fn to_wei(args: ArgsToWei) -> Result<String, String> {
        UtilsModule::to_wei(&wrap::imported::utils_module::ArgsToWei { eth: args.eth })
    }

    fn to_eth(args: ArgsToEth) -> Result<String, String> {
        UtilsModule::to_eth(&wrap::imported::utils_module::ArgsToEth { wei: args.wei })
    }

    fn solidity_pack(args: ArgsSolidityPack) -> Result<String, String> {
        UtilsModule::solidity_pack(&wrap::imported::utils_module::ArgsSolidityPack {
            types: args.types,
            values: args.values,
        })
    }
}
