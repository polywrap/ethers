use std::convert::TryFrom;
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    Context,
    DecodeError,
    EncodeError,
    Read,
    ReadDecoder,
    Write,
    WriteEncoder,
    JSON,
};
use crate::TxReceipt;

use crate::Log;

pub fn serialize_tx_receipt(args: &TxReceipt) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: TxReceipt".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_tx_receipt(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_tx_receipt<W: Write>(args: &TxReceipt, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&16)?;
    writer.context().push("to", "String", "writing property");
    writer.write_string("to")?;
    writer.write_string(&args.to)?;
    writer.context().pop();
    writer.context().push("from", "String", "writing property");
    writer.write_string("from")?;
    writer.write_string(&args.from)?;
    writer.context().pop();
    writer.context().push("contractAddress", "String", "writing property");
    writer.write_string("contractAddress")?;
    writer.write_string(&args.contract_address)?;
    writer.context().pop();
    writer.context().push("transactionIndex", "u32", "writing property");
    writer.write_string("transactionIndex")?;
    writer.write_u32(&args.transaction_index)?;
    writer.context().pop();
    writer.context().push("root", "Option<String>", "writing property");
    writer.write_string("root")?;
    writer.write_optional_string(&args.root)?;
    writer.context().pop();
    writer.context().push("gasUsed", "BigInt", "writing property");
    writer.write_string("gasUsed")?;
    writer.write_bigint(&args.gas_used)?;
    writer.context().pop();
    writer.context().push("logsBloom", "String", "writing property");
    writer.write_string("logsBloom")?;
    writer.write_string(&args.logs_bloom)?;
    writer.context().pop();
    writer.context().push("transactionHash", "String", "writing property");
    writer.write_string("transactionHash")?;
    writer.write_string(&args.transaction_hash)?;
    writer.context().pop();
    writer.context().push("logs", "Vec<Log>", "writing property");
    writer.write_string("logs")?;
    writer.write_array(&args.logs, |writer, item| {
        Log::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("blockNumber", "BigInt", "writing property");
    writer.write_string("blockNumber")?;
    writer.write_bigint(&args.block_number)?;
    writer.context().pop();
    writer.context().push("blockHash", "String", "writing property");
    writer.write_string("blockHash")?;
    writer.write_string(&args.block_hash)?;
    writer.context().pop();
    writer.context().push("confirmations", "u32", "writing property");
    writer.write_string("confirmations")?;
    writer.write_u32(&args.confirmations)?;
    writer.context().pop();
    writer.context().push("cumulativeGasUsed", "BigInt", "writing property");
    writer.write_string("cumulativeGasUsed")?;
    writer.write_bigint(&args.cumulative_gas_used)?;
    writer.context().pop();
    writer.context().push("effectiveGasPrice", "BigInt", "writing property");
    writer.write_string("effectiveGasPrice")?;
    writer.write_bigint(&args.effective_gas_price)?;
    writer.context().pop();
    writer.context().push("type", "u32", "writing property");
    writer.write_string("type")?;
    writer.write_u32(&args._type)?;
    writer.context().pop();
    writer.context().push("status", "Option<u32>", "writing property");
    writer.write_string("status")?;
    writer.write_optional_u32(&args.status)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_tx_receipt(args: &[u8]) -> Result<TxReceipt, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: TxReceipt".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_tx_receipt(&mut reader)
}

pub fn read_tx_receipt<R: Read>(reader: &mut R) -> Result<TxReceipt, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _to: String = String::new();
    let mut _to_set = false;
    let mut _from: String = String::new();
    let mut _from_set = false;
    let mut _contract_address: String = String::new();
    let mut _contract_address_set = false;
    let mut _transaction_index: u32 = 0;
    let mut _transaction_index_set = false;
    let mut _root: Option<String> = None;
    let mut _gas_used: BigInt = BigInt::default();
    let mut _gas_used_set = false;
    let mut _logs_bloom: String = String::new();
    let mut _logs_bloom_set = false;
    let mut _transaction_hash: String = String::new();
    let mut _transaction_hash_set = false;
    let mut _logs: Vec<Log> = vec![];
    let mut _logs_set = false;
    let mut _block_number: BigInt = BigInt::default();
    let mut _block_number_set = false;
    let mut _block_hash: String = String::new();
    let mut _block_hash_set = false;
    let mut _confirmations: u32 = 0;
    let mut _confirmations_set = false;
    let mut _cumulative_gas_used: BigInt = BigInt::default();
    let mut _cumulative_gas_used_set = false;
    let mut _effective_gas_price: BigInt = BigInt::default();
    let mut _effective_gas_price_set = false;
    let mut _type: u32 = 0;
    let mut _type_set = false;
    let mut _status: Option<u32> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "to" => {
                reader.context().push(&field, "String", "type found, reading property");
                _to = reader.read_string()?;
                _to_set = true;
                reader.context().pop();
            }
            "from" => {
                reader.context().push(&field, "String", "type found, reading property");
                _from = reader.read_string()?;
                _from_set = true;
                reader.context().pop();
            }
            "contractAddress" => {
                reader.context().push(&field, "String", "type found, reading property");
                _contract_address = reader.read_string()?;
                _contract_address_set = true;
                reader.context().pop();
            }
            "transactionIndex" => {
                reader.context().push(&field, "u32", "type found, reading property");
                _transaction_index = reader.read_u32()?;
                _transaction_index_set = true;
                reader.context().pop();
            }
            "root" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _root = reader.read_optional_string()?;
                reader.context().pop();
            }
            "gasUsed" => {
                reader.context().push(&field, "BigInt", "type found, reading property");
                _gas_used = reader.read_bigint()?;
                _gas_used_set = true;
                reader.context().pop();
            }
            "logsBloom" => {
                reader.context().push(&field, "String", "type found, reading property");
                _logs_bloom = reader.read_string()?;
                _logs_bloom_set = true;
                reader.context().pop();
            }
            "transactionHash" => {
                reader.context().push(&field, "String", "type found, reading property");
                _transaction_hash = reader.read_string()?;
                _transaction_hash_set = true;
                reader.context().pop();
            }
            "logs" => {
                reader.context().push(&field, "Vec<Log>", "type found, reading property");
                _logs = reader.read_array(|reader| {
                    let object = Log::read(reader)?;
                    Ok(object)
                })?;
                _logs_set = true;
                reader.context().pop();
            }
            "blockNumber" => {
                reader.context().push(&field, "BigInt", "type found, reading property");
                _block_number = reader.read_bigint()?;
                _block_number_set = true;
                reader.context().pop();
            }
            "blockHash" => {
                reader.context().push(&field, "String", "type found, reading property");
                _block_hash = reader.read_string()?;
                _block_hash_set = true;
                reader.context().pop();
            }
            "confirmations" => {
                reader.context().push(&field, "u32", "type found, reading property");
                _confirmations = reader.read_u32()?;
                _confirmations_set = true;
                reader.context().pop();
            }
            "cumulativeGasUsed" => {
                reader.context().push(&field, "BigInt", "type found, reading property");
                _cumulative_gas_used = reader.read_bigint()?;
                _cumulative_gas_used_set = true;
                reader.context().pop();
            }
            "effectiveGasPrice" => {
                reader.context().push(&field, "BigInt", "type found, reading property");
                _effective_gas_price = reader.read_bigint()?;
                _effective_gas_price_set = true;
                reader.context().pop();
            }
            "type" => {
                reader.context().push(&field, "u32", "type found, reading property");
                _type = reader.read_u32()?;
                _type_set = true;
                reader.context().pop();
            }
            "status" => {
                reader.context().push(&field, "Option<u32>", "type found, reading property");
                _status = reader.read_optional_u32()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_to_set {
        return Err(DecodeError::MissingField("to: String.".to_string()));
    }
    if !_from_set {
        return Err(DecodeError::MissingField("from: String.".to_string()));
    }
    if !_contract_address_set {
        return Err(DecodeError::MissingField("contractAddress: String.".to_string()));
    }
    if !_transaction_index_set {
        return Err(DecodeError::MissingField("transactionIndex: UInt32.".to_string()));
    }
    if !_gas_used_set {
        return Err(DecodeError::MissingField("gasUsed: BigInt.".to_string()));
    }
    if !_logs_bloom_set {
        return Err(DecodeError::MissingField("logsBloom: String.".to_string()));
    }
    if !_transaction_hash_set {
        return Err(DecodeError::MissingField("transactionHash: String.".to_string()));
    }
    if !_logs_set {
        return Err(DecodeError::MissingField("logs: [Log].".to_string()));
    }
    if !_block_number_set {
        return Err(DecodeError::MissingField("blockNumber: BigInt.".to_string()));
    }
    if !_block_hash_set {
        return Err(DecodeError::MissingField("blockHash: String.".to_string()));
    }
    if !_confirmations_set {
        return Err(DecodeError::MissingField("confirmations: UInt32.".to_string()));
    }
    if !_cumulative_gas_used_set {
        return Err(DecodeError::MissingField("cumulativeGasUsed: BigInt.".to_string()));
    }
    if !_effective_gas_price_set {
        return Err(DecodeError::MissingField("effectiveGasPrice: BigInt.".to_string()));
    }
    if !_type_set {
        return Err(DecodeError::MissingField("type: UInt32.".to_string()));
    }

    Ok(TxReceipt {
        to: _to,
        from: _from,
        contract_address: _contract_address,
        transaction_index: _transaction_index,
        root: _root,
        gas_used: _gas_used,
        logs_bloom: _logs_bloom,
        transaction_hash: _transaction_hash,
        logs: _logs,
        block_number: _block_number,
        block_hash: _block_hash,
        confirmations: _confirmations,
        cumulative_gas_used: _cumulative_gas_used,
        effective_gas_price: _effective_gas_price,
        _type: _type,
        status: _status,
    })
}
