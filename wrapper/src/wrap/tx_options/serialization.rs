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
use crate::TxOptions;

pub fn serialize_tx_options(args: &TxOptions) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: TxOptions".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_tx_options(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_tx_options<W: Write>(args: &TxOptions, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&6)?;
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

pub fn deserialize_tx_options(args: &[u8]) -> Result<TxOptions, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: TxOptions".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_tx_options(&mut reader)
}

pub fn read_tx_options<R: Read>(reader: &mut R) -> Result<TxOptions, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

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

    Ok(TxOptions {
        gas_limit: _gas_limit,
        max_fee_per_gas: _max_fee_per_gas,
        max_priority_fee_per_gas: _max_priority_fee_per_gas,
        gas_price: _gas_price,
        value: _value,
        nonce: _nonce,
    })
}
