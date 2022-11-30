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
use crate::TxResponse;

use crate::AccessItem;

pub fn serialize_tx_response(args: &TxResponse) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: TxResponse".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_tx_response(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_tx_response<W: Write>(args: &TxResponse, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&18)?;
    writer.context().push("hash", "String", "writing property");
    writer.write_string("hash")?;
    writer.write_string(&args.hash)?;
    writer.context().pop();
    writer.context().push("to", "Option<String>", "writing property");
    writer.write_string("to")?;
    writer.write_optional_string(&args.to)?;
    writer.context().pop();
    writer.context().push("from", "String", "writing property");
    writer.write_string("from")?;
    writer.write_string(&args.from)?;
    writer.context().pop();
    writer.context().push("nonce", "u32", "writing property");
    writer.write_string("nonce")?;
    writer.write_u32(&args.nonce)?;
    writer.context().pop();
    writer.context().push("gasLimit", "BigInt", "writing property");
    writer.write_string("gasLimit")?;
    writer.write_bigint(&args.gas_limit)?;
    writer.context().pop();
    writer.context().push("maxFeePerGas", "Option<BigInt>", "writing property");
    writer.write_string("maxFeePerGas")?;
    writer.write_optional_bigint(&args.max_fee_per_gas)?;
    writer.context().pop();
    writer.context().push("maxPriorityFeePerGas", "Option<BigInt>", "writing property");
    writer.write_string("maxPriorityFeePerGas")?;
    writer.write_optional_bigint(&args.max_priority_fee_per_gas)?;
    writer.context().pop();
    writer.context().push("gasPrice", "Option<BigInt>", "writing property");
    writer.write_string("gasPrice")?;
    writer.write_optional_bigint(&args.gas_price)?;
    writer.context().pop();
    writer.context().push("value", "BigInt", "writing property");
    writer.write_string("value")?;
    writer.write_bigint(&args.value)?;
    writer.context().pop();
    writer.context().push("chainId", "BigInt", "writing property");
    writer.write_string("chainId")?;
    writer.write_bigint(&args.chain_id)?;
    writer.context().pop();
    writer.context().push("blockNumber", "Option<BigInt>", "writing property");
    writer.write_string("blockNumber")?;
    writer.write_optional_bigint(&args.block_number)?;
    writer.context().pop();
    writer.context().push("blockHash", "Option<String>", "writing property");
    writer.write_string("blockHash")?;
    writer.write_optional_string(&args.block_hash)?;
    writer.context().pop();
    writer.context().push("timestamp", "Option<u32>", "writing property");
    writer.write_string("timestamp")?;
    writer.write_optional_u32(&args.timestamp)?;
    writer.context().pop();
    writer.context().push("r", "Option<String>", "writing property");
    writer.write_string("r")?;
    writer.write_optional_string(&args.r)?;
    writer.context().pop();
    writer.context().push("s", "Option<String>", "writing property");
    writer.write_string("s")?;
    writer.write_optional_string(&args.s)?;
    writer.context().pop();
    writer.context().push("v", "Option<u32>", "writing property");
    writer.write_string("v")?;
    writer.write_optional_u32(&args.v)?;
    writer.context().pop();
    writer.context().push("type", "Option<u32>", "writing property");
    writer.write_string("type")?;
    writer.write_optional_u32(&args._type)?;
    writer.context().pop();
    writer.context().push("accessList", "Option<Vec<AccessItem>>", "writing property");
    writer.write_string("accessList")?;
    writer.write_optional_array(&args.access_list, |writer, item| {
        AccessItem::write(item, writer)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_tx_response(args: &[u8]) -> Result<TxResponse, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: TxResponse".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_tx_response(&mut reader)
}

pub fn read_tx_response<R: Read>(reader: &mut R) -> Result<TxResponse, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _hash: String = String::new();
    let mut _hash_set = false;
    let mut _to: Option<String> = None;
    let mut _from: String = String::new();
    let mut _from_set = false;
    let mut _nonce: u32 = 0;
    let mut _nonce_set = false;
    let mut _gas_limit: BigInt = BigInt::default();
    let mut _gas_limit_set = false;
    let mut _max_fee_per_gas: Option<BigInt> = None;
    let mut _max_priority_fee_per_gas: Option<BigInt> = None;
    let mut _gas_price: Option<BigInt> = None;
    let mut _value: BigInt = BigInt::default();
    let mut _value_set = false;
    let mut _chain_id: BigInt = BigInt::default();
    let mut _chain_id_set = false;
    let mut _block_number: Option<BigInt> = None;
    let mut _block_hash: Option<String> = None;
    let mut _timestamp: Option<u32> = None;
    let mut _r: Option<String> = None;
    let mut _s: Option<String> = None;
    let mut _v: Option<u32> = None;
    let mut _type: Option<u32> = None;
    let mut _access_list: Option<Vec<AccessItem>> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "hash" => {
                reader.context().push(&field, "String", "type found, reading property");
                _hash = reader.read_string()?;
                _hash_set = true;
                reader.context().pop();
            }
            "to" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _to = reader.read_optional_string()?;
                reader.context().pop();
            }
            "from" => {
                reader.context().push(&field, "String", "type found, reading property");
                _from = reader.read_string()?;
                _from_set = true;
                reader.context().pop();
            }
            "nonce" => {
                reader.context().push(&field, "u32", "type found, reading property");
                _nonce = reader.read_u32()?;
                _nonce_set = true;
                reader.context().pop();
            }
            "gasLimit" => {
                reader.context().push(&field, "BigInt", "type found, reading property");
                _gas_limit = reader.read_bigint()?;
                _gas_limit_set = true;
                reader.context().pop();
            }
            "maxFeePerGas" => {
                reader.context().push(&field, "Option<BigInt>", "type found, reading property");
                _max_fee_per_gas = reader.read_optional_bigint()?;
                reader.context().pop();
            }
            "maxPriorityFeePerGas" => {
                reader.context().push(&field, "Option<BigInt>", "type found, reading property");
                _max_priority_fee_per_gas = reader.read_optional_bigint()?;
                reader.context().pop();
            }
            "gasPrice" => {
                reader.context().push(&field, "Option<BigInt>", "type found, reading property");
                _gas_price = reader.read_optional_bigint()?;
                reader.context().pop();
            }
            "value" => {
                reader.context().push(&field, "BigInt", "type found, reading property");
                _value = reader.read_bigint()?;
                _value_set = true;
                reader.context().pop();
            }
            "chainId" => {
                reader.context().push(&field, "BigInt", "type found, reading property");
                _chain_id = reader.read_bigint()?;
                _chain_id_set = true;
                reader.context().pop();
            }
            "blockNumber" => {
                reader.context().push(&field, "Option<BigInt>", "type found, reading property");
                _block_number = reader.read_optional_bigint()?;
                reader.context().pop();
            }
            "blockHash" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _block_hash = reader.read_optional_string()?;
                reader.context().pop();
            }
            "timestamp" => {
                reader.context().push(&field, "Option<u32>", "type found, reading property");
                _timestamp = reader.read_optional_u32()?;
                reader.context().pop();
            }
            "r" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _r = reader.read_optional_string()?;
                reader.context().pop();
            }
            "s" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _s = reader.read_optional_string()?;
                reader.context().pop();
            }
            "v" => {
                reader.context().push(&field, "Option<u32>", "type found, reading property");
                _v = reader.read_optional_u32()?;
                reader.context().pop();
            }
            "type" => {
                reader.context().push(&field, "Option<u32>", "type found, reading property");
                _type = reader.read_optional_u32()?;
                reader.context().pop();
            }
            "accessList" => {
                reader.context().push(&field, "Option<Vec<AccessItem>>", "type found, reading property");
                _access_list = reader.read_optional_array(|reader| {
                    let object = AccessItem::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_hash_set {
        return Err(DecodeError::MissingField("hash: String.".to_string()));
    }
    if !_from_set {
        return Err(DecodeError::MissingField("from: String.".to_string()));
    }
    if !_nonce_set {
        return Err(DecodeError::MissingField("nonce: UInt32.".to_string()));
    }
    if !_gas_limit_set {
        return Err(DecodeError::MissingField("gasLimit: BigInt.".to_string()));
    }
    if !_value_set {
        return Err(DecodeError::MissingField("value: BigInt.".to_string()));
    }
    if !_chain_id_set {
        return Err(DecodeError::MissingField("chainId: BigInt.".to_string()));
    }

    Ok(TxResponse {
        hash: _hash,
        to: _to,
        from: _from,
        nonce: _nonce,
        gas_limit: _gas_limit,
        max_fee_per_gas: _max_fee_per_gas,
        max_priority_fee_per_gas: _max_priority_fee_per_gas,
        gas_price: _gas_price,
        value: _value,
        chain_id: _chain_id,
        block_number: _block_number,
        block_hash: _block_hash,
        timestamp: _timestamp,
        r: _r,
        s: _s,
        v: _v,
        _type: _type,
        access_list: _access_list,
    })
}
