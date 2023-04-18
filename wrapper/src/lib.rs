use ethers_core::abi::{encode, encode_packed, Abi, Function, Token};
use ethers_core::types::{Address, BlockId, BlockNumber, Bytes, H256};
use ethers_core::utils::{get_create2_address, keccak256};
use polywrap_wasm_rs::{BigInt, JSON};
use std::str::FromStr;

mod wrap;
use crate::provider::PolywrapProvider;
use crate::signer::PolywrapSigner;
use wrap::module::{Module, ModuleTrait};
use wrap::*;

mod api;
mod helpers;
mod polywrap_provider;
use crate::polywrap_provider::sync_signer::SyncSigner;
use helpers::{format, mapping};
use polywrap_provider::{error, provider, signer, sync_provider::SyncProvider};

impl ModuleTrait for Module {
    fn get_chain_id(args: wrap::ArgsGetChainId) -> Result<String, String> {
        let provider = PolywrapProvider::new(&args.connection);
        Ok(provider.get_chainid_sync().unwrap().to_string())
    }

    fn get_balance(args: wrap::ArgsGetBalance) -> Result<BigInt, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let address = match Address::from_str(&args.address) {
            Ok(addr) => Ok(addr),
            Err(e) => Err(format!("Invalid address: {}. Error: {}", &args.address, e)),
        };

        if address.is_err() {
            return Err(address.unwrap_err())
        }
        let block_tag: BlockId = BlockNumber::Latest.into();
        let balance = provider.get_balance_sync(address.unwrap(), Some(block_tag));

        if let Err(error) = balance {
            return Err(format!("Error in get_balance: {}", error.to_string()));
        }
        Ok(BigInt::from_str(&balance.unwrap().to_string()).unwrap())
    }

    fn check_address(args: wrap::ArgsCheckAddress) -> Result<bool, String> {
        Ok(match Address::from_str(&args.address) {
            Ok(_) => true,
            Err(_) => false,
        })
    }

    fn get_gas_price(args: wrap::ArgsGetGasPrice) -> Result<BigInt, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let price = provider.get_gas_price_sync().unwrap();
        Ok(BigInt::from_str(&price.to_string()).unwrap())
    }

    fn estimate_eip1559_fees(args: wrap::ArgsEstimateEip1559Fees) -> Result<wrap::Eip1559FeesEstimate, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let price = provider.estimate_eip1559_fees_sync(None).unwrap();
        Ok(wrap::Eip1559FeesEstimate {
            max_fee_per_gas: BigInt::from_str(&price.0.to_string()).unwrap(),
            max_priority_fee_per_gas: BigInt::from_str(&price.1.to_string()).unwrap(),
        })
    }

    fn get_signer_address(args: wrap::ArgsGetSignerAddress) -> Result<String, String> {
        let address = PolywrapSigner::new(&args.connection).address();
        Ok(format!("{:#x}", address))
    }

    fn get_signer_balance(args: wrap::ArgsGetSignerBalance) -> Result<BigInt, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let address = PolywrapSigner::new(&args.connection).address();
        let block_tag: BlockId = BlockNumber::Latest.into();
        let balance = provider.get_balance_sync(address, Some(block_tag)).unwrap();
        Ok(BigInt::from_str(&balance.to_string()).unwrap())
    }

    fn get_signer_transaction_count(args: wrap::ArgsGetSignerTransactionCount) -> Result<BigInt, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let address = PolywrapSigner::new(&args.connection).address();
        let block_tag: BlockId = BlockNumber::Latest.into();
        let count = provider
            .get_transaction_count_sync(address, Some(block_tag))
            .unwrap();
        Ok(BigInt::from_str(&count.to_string()).unwrap())
    }

    fn sign_message(args: wrap::ArgsSignMessage) -> Result<String, String> {
        let signer = PolywrapSigner::new(&args.connection);
        let signature = signer.sign_message_sync(&args.message).unwrap();
        let bytes: Bytes = signature.to_vec().into();
        Ok(format!("{}", bytes).to_string())
    }

    fn sign_message_bytes(args: wrap::ArgsSignMessageBytes) -> Result<String, String> {
        let signer = PolywrapSigner::new(&args.connection);
        let signature = signer.sign_message_sync(&args.bytes).unwrap();
        let bytes: Bytes = signature.to_vec().into();
        Ok(format!("{}", bytes).to_string())
    }

    fn sign_transaction(args: wrap::ArgsSignTransaction) -> Result<String, String> {
        let signer = PolywrapSigner::new(&args.connection);
        let tx = mapping::from_wrap_request(args.tx);
        let signature = signer.sign_transaction_sync(&tx).unwrap();
        let bytes: Bytes = signature.to_vec().into();
        Ok(format!("{}", bytes).to_string())
    }

    fn sign_typed_data(args: wrap::ArgsSignTypedData) -> Result<String, String> {
        let address = PolywrapSigner::new(&args.connection).address();
        let address_value = JSON::Value::String(format!("{:#x}", address));
        let params = JSON::Value::Array(vec![address_value, args.payload]);
        let provider = PolywrapProvider::new(&args.connection);
        provider
            .request_sync("eth_signTypedData_v4", params)
            .map_err(|e| format!("Error in sign_typed_data method: {}", e.to_string()))
    }

    fn encode_params(input: wrap::ArgsEncodeParams) -> Result<String, String> {
        let bytes: Bytes = api::encode_params(input.types, input.values).into();
        Ok(format!("{}", bytes).to_string())
    }

    fn encode_function(input: wrap::ArgsEncodeFunction) -> Result<String, String> {
        let args: Vec<String> = input.args.unwrap_or(vec![]);
        let (_, bytes): (Function, Bytes) = api::encode_function(&input.method, &args).unwrap();
        Ok(format!("{}", bytes).to_string())
    }

    fn decode_function(input: wrap::ArgsDecodeFunction) -> Result<Vec<String>, String> {
        let data = Bytes::from_str(&input.data).unwrap().to_vec();
        let tokens = api::decode_function(&input.method, data);
        Ok(tokens.iter().map(|t| format::format_token(t)).collect())
    }

    fn to_wei(input: ArgsToWei) -> Result<String, String> {
        Ok(api::to_wei(input.eth).to_string())
    }

    fn to_eth(input: ArgsToEth) -> Result<String, String> {
        Ok(api::to_eth(input.wei).to_string())
    }

    fn send_rpc(args: wrap::ArgsSendRpc) -> Result<String, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let res: serde_json::Value = provider.request_sync(&args.method, args.params).unwrap();
        Ok(res.to_string())
    }

    fn estimate_transaction_gas(args: wrap::ArgsEstimateTransactionGas) -> Result<BigInt, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let tx = mapping::from_wrap_request(args.tx);
        let gas = provider.estimate_gas_sync(&tx, None).unwrap();
        Ok(BigInt::from_str(&gas.to_string()).unwrap())
    }

    fn await_transaction(args: wrap::ArgsAwaitTransaction) -> Result<wrap::TxReceipt, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let tx_hash = H256::from_str(&args.tx_hash).unwrap();
        provider
            .await_transaction_sync(tx_hash.clone(), args.confirmations, args.timeout)
            .unwrap();
        let receipt = provider
            .get_transaction_receipt_sync(tx_hash)
            .unwrap()
            .unwrap();
        let tx_receipt = mapping::to_wrap_receipt(receipt, args.confirmations);
        Ok(tx_receipt)
    }

    fn send_transaction(args: wrap::ArgsSendTransaction) -> Result<wrap::TxResponse, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let signer = PolywrapSigner::new(&args.connection);

        let mut tx = mapping::from_wrap_request(args.tx);

        let tx_hash = api::send_transaction(&provider, &signer, &mut tx);
        let response = provider.get_transaction_sync(tx_hash).unwrap().unwrap();
        let tx_response = mapping::to_wrap_response(&provider, response);
        Ok(tx_response)
    }

    fn send_transaction_and_wait(
        args: wrap::ArgsSendTransactionAndWait,
    ) -> Result<wrap::TxReceipt, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let signer = PolywrapSigner::new(&args.connection);

        let mut tx = mapping::from_wrap_request(args.tx);

        let tx_hash = api::send_transaction(&provider, &signer, &mut tx);
        provider
            .await_transaction_sync(tx_hash.clone(), 1, None)
            .unwrap();
        let receipt = provider
            .get_transaction_receipt_sync(tx_hash)
            .unwrap()
            .unwrap();
        let tx_receipt = mapping::to_wrap_receipt(receipt, 1);
        Ok(tx_receipt)
    }

    fn deploy_contract(args: wrap::ArgsDeployContract) -> Result<String, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let signer = PolywrapSigner::new(&args.connection);

        let abi: Abi = serde_json::from_str(&args.abi).unwrap();
        let bytecode = Bytes::from_str(&args.bytecode).unwrap();
        let params: Vec<String> = args.args.unwrap_or(vec![]);
        let tx_options: mapping::EthersTxOptions = mapping::from_wrap_tx_options(args.options);

        let mut tx =
            api::create_deploy_contract_transaction(&abi, bytecode, &params, &tx_options).unwrap();

        let tx_hash = api::send_transaction(&provider, &signer, &mut tx);
        provider
            .await_transaction_sync(tx_hash.clone(), 1, None)
            .unwrap();
        let receipt = provider
            .get_transaction_receipt_sync(tx_hash)
            .unwrap()
            .unwrap();
        let address = receipt
            .contract_address
            .expect("Contract failed to deploy.");
        Ok(format!("{:#x}", address))
    }

    fn estimate_contract_call_gas(
        args: wrap::ArgsEstimateContractCallGas,
    ) -> Result<BigInt, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let signer = PolywrapSigner::new(&args.connection);

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
        BigInt::from_str(&gas.to_string()).map_err(|e| e.to_string())
    }

    fn call_contract_view(args: wrap::ArgsCallContractView) -> Result<String, String> {
        let provider = PolywrapProvider::new(&args.connection);

        let address = match Address::from_str(&args.address) {
            Ok(addr) => addr,
            Err(e) => panic!("Invalid contract address: {}. Error: {}", &args.address, e),
        };
        let params: Vec<String> = args.args.unwrap_or(vec![]);

        let tokens = api::call_contract_view(&provider, address, &args.method, &params);
        Ok(format::format_tokens(&tokens))
    }

    fn call_contract_static(args: ArgsCallContractStatic) -> Result<wrap::StaticTxResult, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let signer = PolywrapSigner::new(&args.connection);

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
            Ok(tokens) => wrap::StaticTxResult {
                result: format::format_tokens(&tokens),
                error: false,
            },
            Err(e) => wrap::StaticTxResult {
                result: e.to_string(),
                error: true,
            },
        })
    }

    fn call_contract_method(
        args: wrap::ArgsCallContractMethod,
    ) -> Result<wrap::TxResponse, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let signer = PolywrapSigner::new(&args.connection);

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
        );

        let response = provider.get_transaction_sync(tx_hash).unwrap().unwrap();
        let tx_response = mapping::to_wrap_response(&provider, response);
        Ok(tx_response)
    }

    fn call_contract_method_and_wait(
        args: wrap::ArgsCallContractMethodAndWait,
    ) -> Result<wrap::TxReceipt, String> {
        let provider = PolywrapProvider::new(&args.connection);
        let signer = PolywrapSigner::new(&args.connection);

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
        );
        provider
            .await_transaction_sync(tx_hash.clone(), 1, None)
            .unwrap();
        let receipt = provider
            .get_transaction_receipt_sync(tx_hash)
            .unwrap()
            .unwrap();
        let tx_receipt = mapping::to_wrap_receipt(receipt, 1);
        Ok(tx_receipt)
    }

    fn solidity_keccak256_bytes(args: wrap::ArgsSolidityKeccak256Bytes) -> Result<String, String> {
        let value = Token::Bytes(args.bytes.as_bytes().to_vec());
        let hash = hex::encode(keccak256(encode(&[value])));

        Ok(hash)
    }

    fn encode_bytes_value(args: wrap::ArgsEncodeBytesValue) -> Result<String, String> {
        let mut bytes: Vec<u8> = Vec::with_capacity(args.value.len());
        bytes.extend_from_slice(args.value.as_bytes());
        Ok(format!("{}", Bytes::from(bytes)).to_string())
    }

    fn keccak256_bytes(args: wrap::ArgsKeccak256Bytes) -> Result<String, String> {
        let decoded = Bytes::from_str(&args.bytes).unwrap();
        let hash = keccak256(decoded);
        Ok(format!("{}", Bytes::from(hash)).to_string())
    }

    fn keccak256_bytes_encode_packed(
        args: wrap::ArgsKeccak256BytesEncodePacked,
    ) -> Result<String, String> {
        let bytes = Bytes::from_str(&args.bytes).unwrap();
        let bytes = Token::Bytes(bytes.to_vec());
        let encoded = keccak256(encode_packed(&[bytes]).unwrap());
        Ok(format!("{}", Bytes::from(encoded)).to_string())
    }

    fn solidity_pack(args: wrap::ArgsSolidityPack) -> Result<String, String> {
        let provider = PolywrapProvider::new(&None);
        let params = JSON::json!({
            "types": args.types,
            "values": args.values
        });
        provider
            .request_sync("eth_encodePacked", params)
            .map_err(|e| format!("Solidity pack error: {}", e.to_string()))
    }

    fn generate_create2_address(args: wrap::ArgsGenerateCreate2Address) -> Result<String, String> {
        let salt = Bytes::from_str(&args.salt).unwrap();
        let init_code = Bytes::from_str(&args.init_code).unwrap();
        let address = args.address.parse::<Address>().unwrap();
        let generated_address = get_create2_address(address, salt, init_code);

        Ok(format!("{:?}", generated_address))
    }
}