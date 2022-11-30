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
use crate::StaticTxResult;

pub fn serialize_static_tx_result(args: &StaticTxResult) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: StaticTxResult".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_static_tx_result(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_static_tx_result<W: Write>(args: &StaticTxResult, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("result", "String", "writing property");
    writer.write_string("result")?;
    writer.write_string(&args.result)?;
    writer.context().pop();
    writer.context().push("error", "bool", "writing property");
    writer.write_string("error")?;
    writer.write_bool(&args.error)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_static_tx_result(args: &[u8]) -> Result<StaticTxResult, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: StaticTxResult".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_static_tx_result(&mut reader)
}

pub fn read_static_tx_result<R: Read>(reader: &mut R) -> Result<StaticTxResult, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _result: String = String::new();
    let mut _result_set = false;
    let mut _error: bool = false;
    let mut _error_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "result" => {
                reader.context().push(&field, "String", "type found, reading property");
                _result = reader.read_string()?;
                _result_set = true;
                reader.context().pop();
            }
            "error" => {
                reader.context().push(&field, "bool", "type found, reading property");
                _error = reader.read_bool()?;
                _error_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_result_set {
        return Err(DecodeError::MissingField("result: String.".to_string()));
    }
    if !_error_set {
        return Err(DecodeError::MissingField("error: Boolean.".to_string()));
    }

    Ok(StaticTxResult {
        result: _result,
        error: _error,
    })
}
