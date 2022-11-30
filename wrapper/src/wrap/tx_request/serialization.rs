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
use crate::TxRequest;

use crate::AccessItem;

pub fn serialize_tx_request(args: &TxRequest) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: TxRequest".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_tx_request(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_tx_request<W: Write>(args: &TxRequest, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&12)?;
    writer.context().push("to", "Option<String>", "writing property");
    writer.write_string("to")?;
    writer.write_optional_string(&args.to)?;
    writer.context().pop();
    writer.context().push("from", "Option<String>", "writing property");
    writer.write_string("from")?;
    writer.write_optional_string(&args.from)?;
    writer.context().pop();
    writer.context().push("data", "Option<String>", "writing property");
    writer.write_string("data")?;
    writer.write_optional_string(&args.data)?;
    writer.context().pop();
    writer.context().push("type", "Option<u32>", "writing property");
    writer.write_string("type")?;
    writer.write_optional_u32(&args._type)?;
    writer.context().pop();
    writer.context().push("chainId", "Option<BigInt>", "writing property");
    writer.write_string("chainId")?;
    writer.write_optional_bigint(&args.chain_id)?;
    writer.context().pop();
    writer.context().push("accessList", "Option<Vec<AccessItem>>", "writing property");
    writer.write_string("accessList")?;
    writer.write_optional_array(&args.access_list, |writer, item| {
        AccessItem::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("gasLimit", "Option<BigInt>", "writing property");
    writer.write_string("gasLimit")?;
    writer.write_optional_bigint(&args.gas_limit)?;
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
    writer.context().push("value", "Option<BigInt>", "writing property");
    writer.write_string("value")?;
    writer.write_optional_bigint(&args.value)?;
    writer.context().pop();
    writer.context().push("nonce", "Option<u32>", "writing property");
    writer.write_string("nonce")?;
    writer.write_optional_u32(&args.nonce)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_tx_request(args: &[u8]) -> Result<TxRequest, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: TxRequest".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_tx_request(&mut reader)
}

pub fn read_tx_request<R: Read>(reader: &mut R) -> Result<TxRequest, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _to: Option<String> = None;
    let mut _from: Option<String> = None;
    let mut _data: Option<String> = None;
    let mut _type: Option<u32> = None;
    let mut _chain_id: Option<BigInt> = None;
    let mut _access_list: Option<Vec<AccessItem>> = None;
    let mut _gas_limit: Option<BigInt> = None;
    let mut _max_fee_per_gas: Option<BigInt> = None;
    let mut _max_priority_fee_per_gas: Option<BigInt> = None;
    let mut _gas_price: Option<BigInt> = None;
    let mut _value: Option<BigInt> = None;
    let mut _nonce: Option<u32> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "to" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _to = reader.read_optional_string()?;
                reader.context().pop();
            }
            "from" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _from = reader.read_optional_string()?;
                reader.context().pop();
            }
            "data" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _data = reader.read_optional_string()?;
                reader.context().pop();
            }
            "type" => {
                reader.context().push(&field, "Option<u32>", "type found, reading property");
                _type = reader.read_optional_u32()?;
                reader.context().pop();
            }
            "chainId" => {
                reader.context().push(&field, "Option<BigInt>", "type found, reading property");
                _chain_id = reader.read_optional_bigint()?;
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
            "gasLimit" => {
                reader.context().push(&field, "Option<BigInt>", "type found, reading property");
                _gas_limit = reader.read_optional_bigint()?;
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
                reader.context().push(&field, "Option<BigInt>", "type found, reading property");
                _value = reader.read_optional_bigint()?;
                reader.context().pop();
            }
            "nonce" => {
                reader.context().push(&field, "Option<u32>", "type found, reading property");
                _nonce = reader.read_optional_u32()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(TxRequest {
        to: _to,
        from: _from,
        data: _data,
        _type: _type,
        chain_id: _chain_id,
        access_list: _access_list,
        gas_limit: _gas_limit,
        max_fee_per_gas: _max_fee_per_gas,
        max_priority_fee_per_gas: _max_priority_fee_per_gas,
        gas_price: _gas_price,
        value: _value,
        nonce: _nonce,
    })
}
