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
use crate::Eip1559FeesEstimate;

pub fn serialize_eip1559_fees_estimate(args: &Eip1559FeesEstimate) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: Eip1559FeesEstimate".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_eip1559_fees_estimate(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_eip1559_fees_estimate<W: Write>(args: &Eip1559FeesEstimate, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("maxFeePerGas", "BigInt", "writing property");
    writer.write_string("maxFeePerGas")?;
    writer.write_bigint(&args.max_fee_per_gas)?;
    writer.context().pop();
    writer.context().push("maxPriorityFeePerGas", "BigInt", "writing property");
    writer.write_string("maxPriorityFeePerGas")?;
    writer.write_bigint(&args.max_priority_fee_per_gas)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_eip1559_fees_estimate(args: &[u8]) -> Result<Eip1559FeesEstimate, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: Eip1559FeesEstimate".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_eip1559_fees_estimate(&mut reader)
}

pub fn read_eip1559_fees_estimate<R: Read>(reader: &mut R) -> Result<Eip1559FeesEstimate, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _max_fee_per_gas: BigInt = BigInt::default();
    let mut _max_fee_per_gas_set = false;
    let mut _max_priority_fee_per_gas: BigInt = BigInt::default();
    let mut _max_priority_fee_per_gas_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "maxFeePerGas" => {
                reader.context().push(&field, "BigInt", "type found, reading property");
                _max_fee_per_gas = reader.read_bigint()?;
                _max_fee_per_gas_set = true;
                reader.context().pop();
            }
            "maxPriorityFeePerGas" => {
                reader.context().push(&field, "BigInt", "type found, reading property");
                _max_priority_fee_per_gas = reader.read_bigint()?;
                _max_priority_fee_per_gas_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_max_fee_per_gas_set {
        return Err(DecodeError::MissingField("maxFeePerGas: BigInt.".to_string()));
    }
    if !_max_priority_fee_per_gas_set {
        return Err(DecodeError::MissingField("maxPriorityFeePerGas: BigInt.".to_string()));
    }

    Ok(Eip1559FeesEstimate {
        max_fee_per_gas: _max_fee_per_gas,
        max_priority_fee_per_gas: _max_priority_fee_per_gas,
    })
}
