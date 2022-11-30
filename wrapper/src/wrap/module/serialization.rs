use serde::{Serialize, Deserialize};
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

use crate::ProviderConnection;
use crate::TxOptions;
use crate::StaticTxResult;
use crate::Eip1559FeesEstimate;
use crate::TxRequest;
use crate::TxReceipt;
use crate::TxResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetChainId {
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_get_chain_id_args(args: &[u8]) -> Result<ArgsGetChainId, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: get_chain_id Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ArgsGetChainId {
        connection: _connection,
    })
}

pub fn serialize_get_chain_id_args(args: &ArgsGetChainId) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: get_chain_id Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_chain_id_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_chain_id_args<W: Write>(args: &ArgsGetChainId, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_get_chain_id_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: get_chain_id Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_chain_id_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_chain_id_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("getChainId", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_get_chain_id_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: get_chain_id Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("getChainId", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsCallContractView {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_call_contract_view_args(args: &[u8]) -> Result<ArgsCallContractView, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: call_contract_view Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _address: String = String::new();
    let mut _address_set = false;
    let mut _method: String = String::new();
    let mut _method_set = false;
    let mut _args: Option<Vec<String>> = None;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "address" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _address = reader.read_string()?;
                _address_set = true;
                reader.context().pop();
            }
            "method" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _method = reader.read_string()?;
                _method_set = true;
                reader.context().pop();
            }
            "args" => {
                reader.context().push(&field, "Option<Vec<String>>", "type found, reading argument");
                _args = reader.read_optional_array(|reader| {
                    reader.read_string()
                })?;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_address_set {
        return Err(DecodeError::MissingField("address: String.".to_string()));
    }
    if !_method_set {
        return Err(DecodeError::MissingField("method: String.".to_string()));
    }

    Ok(ArgsCallContractView {
        address: _address,
        method: _method,
        args: _args,
        connection: _connection,
    })
}

pub fn serialize_call_contract_view_args(args: &ArgsCallContractView) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: call_contract_view Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_call_contract_view_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_call_contract_view_args<W: Write>(args: &ArgsCallContractView, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&4)?;
    writer.context().push("address", "String", "writing property");
    writer.write_string("address")?;
    writer.write_string(&args.address)?;
    writer.context().pop();
    writer.context().push("method", "String", "writing property");
    writer.write_string("method")?;
    writer.write_string(&args.method)?;
    writer.context().pop();
    writer.context().push("args", "Option<Vec<String>>", "writing property");
    writer.write_string("args")?;
    writer.write_optional_array(&args.args, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_call_contract_view_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: call_contract_view Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_call_contract_view_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_call_contract_view_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("callContractView", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_call_contract_view_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: call_contract_view Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("callContractView", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsCallContractStatic {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub options: Option<TxOptions>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_call_contract_static_args(args: &[u8]) -> Result<ArgsCallContractStatic, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: call_contract_static Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _address: String = String::new();
    let mut _address_set = false;
    let mut _method: String = String::new();
    let mut _method_set = false;
    let mut _args: Option<Vec<String>> = None;
    let mut _options: Option<TxOptions> = None;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "address" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _address = reader.read_string()?;
                _address_set = true;
                reader.context().pop();
            }
            "method" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _method = reader.read_string()?;
                _method_set = true;
                reader.context().pop();
            }
            "args" => {
                reader.context().push(&field, "Option<Vec<String>>", "type found, reading argument");
                _args = reader.read_optional_array(|reader| {
                    reader.read_string()
                })?;
                reader.context().pop();
            }
            "options" => {
                reader.context().push(&field, "Option<TxOptions>", "type found, reading argument");
                let mut object: Option<TxOptions> = None;
                if !reader.is_next_nil()? {
                    object = Some(TxOptions::read(&mut reader)?);
                } else {
                    object = None;
                }
                _options = object;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_address_set {
        return Err(DecodeError::MissingField("address: String.".to_string()));
    }
    if !_method_set {
        return Err(DecodeError::MissingField("method: String.".to_string()));
    }

    Ok(ArgsCallContractStatic {
        address: _address,
        method: _method,
        args: _args,
        options: _options,
        connection: _connection,
    })
}

pub fn serialize_call_contract_static_args(args: &ArgsCallContractStatic) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: call_contract_static Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_call_contract_static_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_call_contract_static_args<W: Write>(args: &ArgsCallContractStatic, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&5)?;
    writer.context().push("address", "String", "writing property");
    writer.write_string("address")?;
    writer.write_string(&args.address)?;
    writer.context().pop();
    writer.context().push("method", "String", "writing property");
    writer.write_string("method")?;
    writer.write_string(&args.method)?;
    writer.context().pop();
    writer.context().push("args", "Option<Vec<String>>", "writing property");
    writer.write_string("args")?;
    writer.write_optional_array(&args.args, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    writer.context().push("options", "Option<TxOptions>", "writing property");
    writer.write_string("options")?;
    if args.options.is_some() {
        TxOptions::write(args.options.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_call_contract_static_result(result: &StaticTxResult) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: call_contract_static Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_call_contract_static_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_call_contract_static_result<W: Write>(result: &StaticTxResult, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("callContractStatic", "StaticTxResult", "writing result");
    StaticTxResult::write(&result, writer)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_call_contract_static_result(result: &[u8]) -> Result<StaticTxResult, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: call_contract_static Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("callContractStatic", "StaticTxResult", "reading function output");
    let object = StaticTxResult::read(&mut reader)?;
    let res = object;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsEncodeParams {
    pub types: Vec<String>,
    pub values: Vec<String>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_encode_params_args(args: &[u8]) -> Result<ArgsEncodeParams, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: encode_params Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _types: Vec<String> = vec![];
    let mut _types_set = false;
    let mut _values: Vec<String> = vec![];
    let mut _values_set = false;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "types" => {
                reader.context().push(&field, "Vec<String>", "type found, reading argument");
                _types = reader.read_array(|reader| {
                    reader.read_string()
                })?;
                _types_set = true;
                reader.context().pop();
            }
            "values" => {
                reader.context().push(&field, "Vec<String>", "type found, reading argument");
                _values = reader.read_array(|reader| {
                    reader.read_string()
                })?;
                _values_set = true;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_types_set {
        return Err(DecodeError::MissingField("types: [String].".to_string()));
    }
    if !_values_set {
        return Err(DecodeError::MissingField("values: [String].".to_string()));
    }

    Ok(ArgsEncodeParams {
        types: _types,
        values: _values,
        connection: _connection,
    })
}

pub fn serialize_encode_params_args(args: &ArgsEncodeParams) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: encode_params Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_encode_params_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_encode_params_args<W: Write>(args: &ArgsEncodeParams, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("types", "Vec<String>", "writing property");
    writer.write_string("types")?;
    writer.write_array(&args.types, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    writer.context().push("values", "Vec<String>", "writing property");
    writer.write_string("values")?;
    writer.write_array(&args.values, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_encode_params_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: encode_params Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_encode_params_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_encode_params_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("encodeParams", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_encode_params_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: encode_params Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("encodeParams", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsEncodeFunction {
    pub method: String,
    pub args: Option<Vec<String>>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_encode_function_args(args: &[u8]) -> Result<ArgsEncodeFunction, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: encode_function Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _method: String = String::new();
    let mut _method_set = false;
    let mut _args: Option<Vec<String>> = None;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "method" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _method = reader.read_string()?;
                _method_set = true;
                reader.context().pop();
            }
            "args" => {
                reader.context().push(&field, "Option<Vec<String>>", "type found, reading argument");
                _args = reader.read_optional_array(|reader| {
                    reader.read_string()
                })?;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_method_set {
        return Err(DecodeError::MissingField("method: String.".to_string()));
    }

    Ok(ArgsEncodeFunction {
        method: _method,
        args: _args,
        connection: _connection,
    })
}

pub fn serialize_encode_function_args(args: &ArgsEncodeFunction) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: encode_function Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_encode_function_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_encode_function_args<W: Write>(args: &ArgsEncodeFunction, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("method", "String", "writing property");
    writer.write_string("method")?;
    writer.write_string(&args.method)?;
    writer.context().pop();
    writer.context().push("args", "Option<Vec<String>>", "writing property");
    writer.write_string("args")?;
    writer.write_optional_array(&args.args, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_encode_function_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: encode_function Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_encode_function_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_encode_function_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("encodeFunction", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_encode_function_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: encode_function Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("encodeFunction", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsDecodeFunction {
    pub method: String,
    pub data: String,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_decode_function_args(args: &[u8]) -> Result<ArgsDecodeFunction, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: decode_function Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _method: String = String::new();
    let mut _method_set = false;
    let mut _data: String = String::new();
    let mut _data_set = false;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "method" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _method = reader.read_string()?;
                _method_set = true;
                reader.context().pop();
            }
            "data" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _data = reader.read_string()?;
                _data_set = true;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_method_set {
        return Err(DecodeError::MissingField("method: String.".to_string()));
    }
    if !_data_set {
        return Err(DecodeError::MissingField("data: String.".to_string()));
    }

    Ok(ArgsDecodeFunction {
        method: _method,
        data: _data,
        connection: _connection,
    })
}

pub fn serialize_decode_function_args(args: &ArgsDecodeFunction) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: decode_function Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_decode_function_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_decode_function_args<W: Write>(args: &ArgsDecodeFunction, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("method", "String", "writing property");
    writer.write_string("method")?;
    writer.write_string(&args.method)?;
    writer.context().pop();
    writer.context().push("data", "String", "writing property");
    writer.write_string("data")?;
    writer.write_string(&args.data)?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_decode_function_result(result: &Vec<String>) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: decode_function Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_decode_function_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_decode_function_result<W: Write>(result: &Vec<String>, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("decodeFunction", "Vec<String>", "writing result");
    writer.write_array(&result, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_decode_function_result(result: &[u8]) -> Result<Vec<String>, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: decode_function Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("decodeFunction", "Vec<String>", "reading function output");
    let res = reader.read_array(|reader| {
        reader.read_string()
    })?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetSignerAddress {
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_get_signer_address_args(args: &[u8]) -> Result<ArgsGetSignerAddress, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: get_signer_address Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ArgsGetSignerAddress {
        connection: _connection,
    })
}

pub fn serialize_get_signer_address_args(args: &ArgsGetSignerAddress) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: get_signer_address Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_signer_address_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_signer_address_args<W: Write>(args: &ArgsGetSignerAddress, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_get_signer_address_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: get_signer_address Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_signer_address_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_signer_address_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("getSignerAddress", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_get_signer_address_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: get_signer_address Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("getSignerAddress", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetSignerBalance {
    pub block_tag: Option<BigInt>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_get_signer_balance_args(args: &[u8]) -> Result<ArgsGetSignerBalance, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: get_signer_balance Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _block_tag: Option<BigInt> = None;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "blockTag" => {
                reader.context().push(&field, "Option<BigInt>", "type found, reading argument");
                _block_tag = reader.read_optional_bigint()?;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ArgsGetSignerBalance {
        block_tag: _block_tag,
        connection: _connection,
    })
}

pub fn serialize_get_signer_balance_args(args: &ArgsGetSignerBalance) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: get_signer_balance Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_signer_balance_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_signer_balance_args<W: Write>(args: &ArgsGetSignerBalance, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("blockTag", "Option<BigInt>", "writing property");
    writer.write_string("blockTag")?;
    writer.write_optional_bigint(&args.block_tag)?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_get_signer_balance_result(result: &BigInt) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: get_signer_balance Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_signer_balance_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_signer_balance_result<W: Write>(result: &BigInt, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("getSignerBalance", "BigInt", "writing result");
    writer.write_bigint(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_get_signer_balance_result(result: &[u8]) -> Result<BigInt, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: get_signer_balance Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("getSignerBalance", "BigInt", "reading function output");
    let res = reader.read_bigint()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetBalance {
    pub address: String,
    pub block_tag: Option<BigInt>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_get_balance_args(args: &[u8]) -> Result<ArgsGetBalance, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: get_balance Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _address: String = String::new();
    let mut _address_set = false;
    let mut _block_tag: Option<BigInt> = None;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "address" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _address = reader.read_string()?;
                _address_set = true;
                reader.context().pop();
            }
            "blockTag" => {
                reader.context().push(&field, "Option<BigInt>", "type found, reading argument");
                _block_tag = reader.read_optional_bigint()?;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_address_set {
        return Err(DecodeError::MissingField("address: String.".to_string()));
    }

    Ok(ArgsGetBalance {
        address: _address,
        block_tag: _block_tag,
        connection: _connection,
    })
}

pub fn serialize_get_balance_args(args: &ArgsGetBalance) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: get_balance Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_balance_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_balance_args<W: Write>(args: &ArgsGetBalance, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("address", "String", "writing property");
    writer.write_string("address")?;
    writer.write_string(&args.address)?;
    writer.context().pop();
    writer.context().push("blockTag", "Option<BigInt>", "writing property");
    writer.write_string("blockTag")?;
    writer.write_optional_bigint(&args.block_tag)?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_get_balance_result(result: &BigInt) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: get_balance Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_balance_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_balance_result<W: Write>(result: &BigInt, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("getBalance", "BigInt", "writing result");
    writer.write_bigint(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_get_balance_result(result: &[u8]) -> Result<BigInt, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: get_balance Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("getBalance", "BigInt", "reading function output");
    let res = reader.read_bigint()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetGasPrice {
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_get_gas_price_args(args: &[u8]) -> Result<ArgsGetGasPrice, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: get_gas_price Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ArgsGetGasPrice {
        connection: _connection,
    })
}

pub fn serialize_get_gas_price_args(args: &ArgsGetGasPrice) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: get_gas_price Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_gas_price_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_gas_price_args<W: Write>(args: &ArgsGetGasPrice, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_get_gas_price_result(result: &BigInt) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: get_gas_price Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_gas_price_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_gas_price_result<W: Write>(result: &BigInt, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("getGasPrice", "BigInt", "writing result");
    writer.write_bigint(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_get_gas_price_result(result: &[u8]) -> Result<BigInt, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: get_gas_price Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("getGasPrice", "BigInt", "reading function output");
    let res = reader.read_bigint()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsEstimateEip1559Fees {
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_estimate_eip1559_fees_args(args: &[u8]) -> Result<ArgsEstimateEip1559Fees, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: estimate_eip1559_fees Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ArgsEstimateEip1559Fees {
        connection: _connection,
    })
}

pub fn serialize_estimate_eip1559_fees_args(args: &ArgsEstimateEip1559Fees) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: estimate_eip1559_fees Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_estimate_eip1559_fees_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_estimate_eip1559_fees_args<W: Write>(args: &ArgsEstimateEip1559Fees, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_estimate_eip1559_fees_result(result: &Eip1559FeesEstimate) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: estimate_eip1559_fees Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_estimate_eip1559_fees_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_estimate_eip1559_fees_result<W: Write>(result: &Eip1559FeesEstimate, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("estimateEip1559Fees", "Eip1559FeesEstimate", "writing result");
    Eip1559FeesEstimate::write(&result, writer)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_estimate_eip1559_fees_result(result: &[u8]) -> Result<Eip1559FeesEstimate, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: estimate_eip1559_fees Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("estimateEip1559Fees", "Eip1559FeesEstimate", "reading function output");
    let object = Eip1559FeesEstimate::read(&mut reader)?;
    let res = object;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsSendRpc {
    pub method: String,
    pub params: Vec<String>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_send_rpc_args(args: &[u8]) -> Result<ArgsSendRpc, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: send_rpc Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _method: String = String::new();
    let mut _method_set = false;
    let mut _params: Vec<String> = vec![];
    let mut _params_set = false;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "method" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _method = reader.read_string()?;
                _method_set = true;
                reader.context().pop();
            }
            "params" => {
                reader.context().push(&field, "Vec<String>", "type found, reading argument");
                _params = reader.read_array(|reader| {
                    reader.read_string()
                })?;
                _params_set = true;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_method_set {
        return Err(DecodeError::MissingField("method: String.".to_string()));
    }
    if !_params_set {
        return Err(DecodeError::MissingField("params: [String].".to_string()));
    }

    Ok(ArgsSendRpc {
        method: _method,
        params: _params,
        connection: _connection,
    })
}

pub fn serialize_send_rpc_args(args: &ArgsSendRpc) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: send_rpc Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_send_rpc_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_send_rpc_args<W: Write>(args: &ArgsSendRpc, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("method", "String", "writing property");
    writer.write_string("method")?;
    writer.write_string(&args.method)?;
    writer.context().pop();
    writer.context().push("params", "Vec<String>", "writing property");
    writer.write_string("params")?;
    writer.write_array(&args.params, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_send_rpc_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: send_rpc Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_send_rpc_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_send_rpc_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("sendRpc", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_send_rpc_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: send_rpc Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("sendRpc", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetSignerTransactionCount {
    pub block_tag: Option<BigInt>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_get_signer_transaction_count_args(args: &[u8]) -> Result<ArgsGetSignerTransactionCount, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: get_signer_transaction_count Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _block_tag: Option<BigInt> = None;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "blockTag" => {
                reader.context().push(&field, "Option<BigInt>", "type found, reading argument");
                _block_tag = reader.read_optional_bigint()?;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ArgsGetSignerTransactionCount {
        block_tag: _block_tag,
        connection: _connection,
    })
}

pub fn serialize_get_signer_transaction_count_args(args: &ArgsGetSignerTransactionCount) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: get_signer_transaction_count Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_signer_transaction_count_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_signer_transaction_count_args<W: Write>(args: &ArgsGetSignerTransactionCount, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("blockTag", "Option<BigInt>", "writing property");
    writer.write_string("blockTag")?;
    writer.write_optional_bigint(&args.block_tag)?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_get_signer_transaction_count_result(result: &BigInt) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: get_signer_transaction_count Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_get_signer_transaction_count_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_get_signer_transaction_count_result<W: Write>(result: &BigInt, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("getSignerTransactionCount", "BigInt", "writing result");
    writer.write_bigint(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_get_signer_transaction_count_result(result: &[u8]) -> Result<BigInt, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: get_signer_transaction_count Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("getSignerTransactionCount", "BigInt", "reading function output");
    let res = reader.read_bigint()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsCheckAddress {
    pub address: String,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_check_address_args(args: &[u8]) -> Result<ArgsCheckAddress, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: check_address Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _address: String = String::new();
    let mut _address_set = false;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "address" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _address = reader.read_string()?;
                _address_set = true;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_address_set {
        return Err(DecodeError::MissingField("address: String.".to_string()));
    }

    Ok(ArgsCheckAddress {
        address: _address,
        connection: _connection,
    })
}

pub fn serialize_check_address_args(args: &ArgsCheckAddress) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: check_address Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_check_address_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_check_address_args<W: Write>(args: &ArgsCheckAddress, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("address", "String", "writing property");
    writer.write_string("address")?;
    writer.write_string(&args.address)?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_check_address_result(result: &bool) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: check_address Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_check_address_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_check_address_result<W: Write>(result: &bool, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("checkAddress", "bool", "writing result");
    writer.write_bool(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_check_address_result(result: &[u8]) -> Result<bool, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: check_address Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("checkAddress", "bool", "reading function output");
    let res = reader.read_bool()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsToWei {
    pub eth: String,
}

pub fn deserialize_to_wei_args(args: &[u8]) -> Result<ArgsToWei, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: to_wei Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _eth: String = String::new();
    let mut _eth_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "eth" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _eth = reader.read_string()?;
                _eth_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_eth_set {
        return Err(DecodeError::MissingField("eth: String.".to_string()));
    }

    Ok(ArgsToWei {
        eth: _eth,
    })
}

pub fn serialize_to_wei_args(args: &ArgsToWei) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: to_wei Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_to_wei_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_to_wei_args<W: Write>(args: &ArgsToWei, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("eth", "String", "writing property");
    writer.write_string("eth")?;
    writer.write_string(&args.eth)?;
    writer.context().pop();
    Ok(())
}

pub fn serialize_to_wei_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: to_wei Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_to_wei_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_to_wei_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("toWei", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_to_wei_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: to_wei Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("toWei", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsToEth {
    pub wei: String,
}

pub fn deserialize_to_eth_args(args: &[u8]) -> Result<ArgsToEth, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: to_eth Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _wei: String = String::new();
    let mut _wei_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "wei" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _wei = reader.read_string()?;
                _wei_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_wei_set {
        return Err(DecodeError::MissingField("wei: String.".to_string()));
    }

    Ok(ArgsToEth {
        wei: _wei,
    })
}

pub fn serialize_to_eth_args(args: &ArgsToEth) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: to_eth Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_to_eth_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_to_eth_args<W: Write>(args: &ArgsToEth, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("wei", "String", "writing property");
    writer.write_string("wei")?;
    writer.write_string(&args.wei)?;
    writer.context().pop();
    Ok(())
}

pub fn serialize_to_eth_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: to_eth Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_to_eth_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_to_eth_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("toEth", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_to_eth_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: to_eth Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("toEth", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsEstimateTransactionGas {
    pub tx: TxRequest,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_estimate_transaction_gas_args(args: &[u8]) -> Result<ArgsEstimateTransactionGas, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: estimate_transaction_gas Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _tx: TxRequest = TxRequest::new();
    let mut _tx_set = false;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "tx" => {
                reader.context().push(&field, "TxRequest", "type found, reading argument");
                let object = TxRequest::read(&mut reader)?;
                _tx = object;
                _tx_set = true;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_tx_set {
        return Err(DecodeError::MissingField("tx: TxRequest.".to_string()));
    }

    Ok(ArgsEstimateTransactionGas {
        tx: _tx,
        connection: _connection,
    })
}

pub fn serialize_estimate_transaction_gas_args(args: &ArgsEstimateTransactionGas) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: estimate_transaction_gas Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_estimate_transaction_gas_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_estimate_transaction_gas_args<W: Write>(args: &ArgsEstimateTransactionGas, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("tx", "TxRequest", "writing property");
    writer.write_string("tx")?;
    TxRequest::write(&args.tx, writer)?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_estimate_transaction_gas_result(result: &BigInt) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: estimate_transaction_gas Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_estimate_transaction_gas_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_estimate_transaction_gas_result<W: Write>(result: &BigInt, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("estimateTransactionGas", "BigInt", "writing result");
    writer.write_bigint(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_estimate_transaction_gas_result(result: &[u8]) -> Result<BigInt, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: estimate_transaction_gas Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("estimateTransactionGas", "BigInt", "reading function output");
    let res = reader.read_bigint()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsAwaitTransaction {
    pub tx_hash: String,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_await_transaction_args(args: &[u8]) -> Result<ArgsAwaitTransaction, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: await_transaction Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _tx_hash: String = String::new();
    let mut _tx_hash_set = false;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "txHash" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _tx_hash = reader.read_string()?;
                _tx_hash_set = true;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_tx_hash_set {
        return Err(DecodeError::MissingField("txHash: String.".to_string()));
    }

    Ok(ArgsAwaitTransaction {
        tx_hash: _tx_hash,
        connection: _connection,
    })
}

pub fn serialize_await_transaction_args(args: &ArgsAwaitTransaction) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: await_transaction Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_await_transaction_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_await_transaction_args<W: Write>(args: &ArgsAwaitTransaction, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("txHash", "String", "writing property");
    writer.write_string("txHash")?;
    writer.write_string(&args.tx_hash)?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_await_transaction_result(result: &TxReceipt) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: await_transaction Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_await_transaction_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_await_transaction_result<W: Write>(result: &TxReceipt, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("awaitTransaction", "TxReceipt", "writing result");
    TxReceipt::write(&result, writer)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_await_transaction_result(result: &[u8]) -> Result<TxReceipt, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: await_transaction Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("awaitTransaction", "TxReceipt", "reading function output");
    let object = TxReceipt::read(&mut reader)?;
    let res = object;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsSendTransaction {
    pub tx: TxRequest,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_send_transaction_args(args: &[u8]) -> Result<ArgsSendTransaction, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: send_transaction Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _tx: TxRequest = TxRequest::new();
    let mut _tx_set = false;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "tx" => {
                reader.context().push(&field, "TxRequest", "type found, reading argument");
                let object = TxRequest::read(&mut reader)?;
                _tx = object;
                _tx_set = true;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_tx_set {
        return Err(DecodeError::MissingField("tx: TxRequest.".to_string()));
    }

    Ok(ArgsSendTransaction {
        tx: _tx,
        connection: _connection,
    })
}

pub fn serialize_send_transaction_args(args: &ArgsSendTransaction) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: send_transaction Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_send_transaction_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_send_transaction_args<W: Write>(args: &ArgsSendTransaction, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("tx", "TxRequest", "writing property");
    writer.write_string("tx")?;
    TxRequest::write(&args.tx, writer)?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_send_transaction_result(result: &TxResponse) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: send_transaction Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_send_transaction_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_send_transaction_result<W: Write>(result: &TxResponse, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("sendTransaction", "TxResponse", "writing result");
    TxResponse::write(&result, writer)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_send_transaction_result(result: &[u8]) -> Result<TxResponse, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: send_transaction Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("sendTransaction", "TxResponse", "reading function output");
    let object = TxResponse::read(&mut reader)?;
    let res = object;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsSendTransactionAndWait {
    pub tx: TxRequest,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_send_transaction_and_wait_args(args: &[u8]) -> Result<ArgsSendTransactionAndWait, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: send_transaction_and_wait Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _tx: TxRequest = TxRequest::new();
    let mut _tx_set = false;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "tx" => {
                reader.context().push(&field, "TxRequest", "type found, reading argument");
                let object = TxRequest::read(&mut reader)?;
                _tx = object;
                _tx_set = true;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_tx_set {
        return Err(DecodeError::MissingField("tx: TxRequest.".to_string()));
    }

    Ok(ArgsSendTransactionAndWait {
        tx: _tx,
        connection: _connection,
    })
}

pub fn serialize_send_transaction_and_wait_args(args: &ArgsSendTransactionAndWait) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: send_transaction_and_wait Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_send_transaction_and_wait_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_send_transaction_and_wait_args<W: Write>(args: &ArgsSendTransactionAndWait, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("tx", "TxRequest", "writing property");
    writer.write_string("tx")?;
    TxRequest::write(&args.tx, writer)?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_send_transaction_and_wait_result(result: &TxReceipt) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: send_transaction_and_wait Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_send_transaction_and_wait_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_send_transaction_and_wait_result<W: Write>(result: &TxReceipt, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("sendTransactionAndWait", "TxReceipt", "writing result");
    TxReceipt::write(&result, writer)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_send_transaction_and_wait_result(result: &[u8]) -> Result<TxReceipt, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: send_transaction_and_wait Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("sendTransactionAndWait", "TxReceipt", "reading function output");
    let object = TxReceipt::read(&mut reader)?;
    let res = object;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsDeployContract {
    pub abi: String,
    pub bytecode: String,
    pub args: Option<Vec<String>>,
    pub options: Option<TxOptions>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_deploy_contract_args(args: &[u8]) -> Result<ArgsDeployContract, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: deploy_contract Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _abi: String = String::new();
    let mut _abi_set = false;
    let mut _bytecode: String = String::new();
    let mut _bytecode_set = false;
    let mut _args: Option<Vec<String>> = None;
    let mut _options: Option<TxOptions> = None;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "abi" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _abi = reader.read_string()?;
                _abi_set = true;
                reader.context().pop();
            }
            "bytecode" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _bytecode = reader.read_string()?;
                _bytecode_set = true;
                reader.context().pop();
            }
            "args" => {
                reader.context().push(&field, "Option<Vec<String>>", "type found, reading argument");
                _args = reader.read_optional_array(|reader| {
                    reader.read_string()
                })?;
                reader.context().pop();
            }
            "options" => {
                reader.context().push(&field, "Option<TxOptions>", "type found, reading argument");
                let mut object: Option<TxOptions> = None;
                if !reader.is_next_nil()? {
                    object = Some(TxOptions::read(&mut reader)?);
                } else {
                    object = None;
                }
                _options = object;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_abi_set {
        return Err(DecodeError::MissingField("abi: String.".to_string()));
    }
    if !_bytecode_set {
        return Err(DecodeError::MissingField("bytecode: String.".to_string()));
    }

    Ok(ArgsDeployContract {
        abi: _abi,
        bytecode: _bytecode,
        args: _args,
        options: _options,
        connection: _connection,
    })
}

pub fn serialize_deploy_contract_args(args: &ArgsDeployContract) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: deploy_contract Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_deploy_contract_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_deploy_contract_args<W: Write>(args: &ArgsDeployContract, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&5)?;
    writer.context().push("abi", "String", "writing property");
    writer.write_string("abi")?;
    writer.write_string(&args.abi)?;
    writer.context().pop();
    writer.context().push("bytecode", "String", "writing property");
    writer.write_string("bytecode")?;
    writer.write_string(&args.bytecode)?;
    writer.context().pop();
    writer.context().push("args", "Option<Vec<String>>", "writing property");
    writer.write_string("args")?;
    writer.write_optional_array(&args.args, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    writer.context().push("options", "Option<TxOptions>", "writing property");
    writer.write_string("options")?;
    if args.options.is_some() {
        TxOptions::write(args.options.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_deploy_contract_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: deploy_contract Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_deploy_contract_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_deploy_contract_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("deployContract", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_deploy_contract_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: deploy_contract Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("deployContract", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsEstimateContractCallGas {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub options: Option<TxOptions>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_estimate_contract_call_gas_args(args: &[u8]) -> Result<ArgsEstimateContractCallGas, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: estimate_contract_call_gas Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _address: String = String::new();
    let mut _address_set = false;
    let mut _method: String = String::new();
    let mut _method_set = false;
    let mut _args: Option<Vec<String>> = None;
    let mut _options: Option<TxOptions> = None;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "address" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _address = reader.read_string()?;
                _address_set = true;
                reader.context().pop();
            }
            "method" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _method = reader.read_string()?;
                _method_set = true;
                reader.context().pop();
            }
            "args" => {
                reader.context().push(&field, "Option<Vec<String>>", "type found, reading argument");
                _args = reader.read_optional_array(|reader| {
                    reader.read_string()
                })?;
                reader.context().pop();
            }
            "options" => {
                reader.context().push(&field, "Option<TxOptions>", "type found, reading argument");
                let mut object: Option<TxOptions> = None;
                if !reader.is_next_nil()? {
                    object = Some(TxOptions::read(&mut reader)?);
                } else {
                    object = None;
                }
                _options = object;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_address_set {
        return Err(DecodeError::MissingField("address: String.".to_string()));
    }
    if !_method_set {
        return Err(DecodeError::MissingField("method: String.".to_string()));
    }

    Ok(ArgsEstimateContractCallGas {
        address: _address,
        method: _method,
        args: _args,
        options: _options,
        connection: _connection,
    })
}

pub fn serialize_estimate_contract_call_gas_args(args: &ArgsEstimateContractCallGas) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: estimate_contract_call_gas Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_estimate_contract_call_gas_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_estimate_contract_call_gas_args<W: Write>(args: &ArgsEstimateContractCallGas, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&5)?;
    writer.context().push("address", "String", "writing property");
    writer.write_string("address")?;
    writer.write_string(&args.address)?;
    writer.context().pop();
    writer.context().push("method", "String", "writing property");
    writer.write_string("method")?;
    writer.write_string(&args.method)?;
    writer.context().pop();
    writer.context().push("args", "Option<Vec<String>>", "writing property");
    writer.write_string("args")?;
    writer.write_optional_array(&args.args, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    writer.context().push("options", "Option<TxOptions>", "writing property");
    writer.write_string("options")?;
    if args.options.is_some() {
        TxOptions::write(args.options.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_estimate_contract_call_gas_result(result: &BigInt) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: estimate_contract_call_gas Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_estimate_contract_call_gas_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_estimate_contract_call_gas_result<W: Write>(result: &BigInt, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("estimateContractCallGas", "BigInt", "writing result");
    writer.write_bigint(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_estimate_contract_call_gas_result(result: &[u8]) -> Result<BigInt, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: estimate_contract_call_gas Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("estimateContractCallGas", "BigInt", "reading function output");
    let res = reader.read_bigint()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsCallContractMethod {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub options: Option<TxOptions>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_call_contract_method_args(args: &[u8]) -> Result<ArgsCallContractMethod, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: call_contract_method Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _address: String = String::new();
    let mut _address_set = false;
    let mut _method: String = String::new();
    let mut _method_set = false;
    let mut _args: Option<Vec<String>> = None;
    let mut _options: Option<TxOptions> = None;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "address" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _address = reader.read_string()?;
                _address_set = true;
                reader.context().pop();
            }
            "method" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _method = reader.read_string()?;
                _method_set = true;
                reader.context().pop();
            }
            "args" => {
                reader.context().push(&field, "Option<Vec<String>>", "type found, reading argument");
                _args = reader.read_optional_array(|reader| {
                    reader.read_string()
                })?;
                reader.context().pop();
            }
            "options" => {
                reader.context().push(&field, "Option<TxOptions>", "type found, reading argument");
                let mut object: Option<TxOptions> = None;
                if !reader.is_next_nil()? {
                    object = Some(TxOptions::read(&mut reader)?);
                } else {
                    object = None;
                }
                _options = object;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_address_set {
        return Err(DecodeError::MissingField("address: String.".to_string()));
    }
    if !_method_set {
        return Err(DecodeError::MissingField("method: String.".to_string()));
    }

    Ok(ArgsCallContractMethod {
        address: _address,
        method: _method,
        args: _args,
        options: _options,
        connection: _connection,
    })
}

pub fn serialize_call_contract_method_args(args: &ArgsCallContractMethod) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: call_contract_method Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_call_contract_method_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_call_contract_method_args<W: Write>(args: &ArgsCallContractMethod, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&5)?;
    writer.context().push("address", "String", "writing property");
    writer.write_string("address")?;
    writer.write_string(&args.address)?;
    writer.context().pop();
    writer.context().push("method", "String", "writing property");
    writer.write_string("method")?;
    writer.write_string(&args.method)?;
    writer.context().pop();
    writer.context().push("args", "Option<Vec<String>>", "writing property");
    writer.write_string("args")?;
    writer.write_optional_array(&args.args, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    writer.context().push("options", "Option<TxOptions>", "writing property");
    writer.write_string("options")?;
    if args.options.is_some() {
        TxOptions::write(args.options.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_call_contract_method_result(result: &TxResponse) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: call_contract_method Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_call_contract_method_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_call_contract_method_result<W: Write>(result: &TxResponse, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("callContractMethod", "TxResponse", "writing result");
    TxResponse::write(&result, writer)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_call_contract_method_result(result: &[u8]) -> Result<TxResponse, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: call_contract_method Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("callContractMethod", "TxResponse", "reading function output");
    let object = TxResponse::read(&mut reader)?;
    let res = object;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsCallContractMethodAndWait {
    pub address: String,
    pub method: String,
    pub args: Option<Vec<String>>,
    pub options: Option<TxOptions>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_call_contract_method_and_wait_args(args: &[u8]) -> Result<ArgsCallContractMethodAndWait, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: call_contract_method_and_wait Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _address: String = String::new();
    let mut _address_set = false;
    let mut _method: String = String::new();
    let mut _method_set = false;
    let mut _args: Option<Vec<String>> = None;
    let mut _options: Option<TxOptions> = None;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "address" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _address = reader.read_string()?;
                _address_set = true;
                reader.context().pop();
            }
            "method" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _method = reader.read_string()?;
                _method_set = true;
                reader.context().pop();
            }
            "args" => {
                reader.context().push(&field, "Option<Vec<String>>", "type found, reading argument");
                _args = reader.read_optional_array(|reader| {
                    reader.read_string()
                })?;
                reader.context().pop();
            }
            "options" => {
                reader.context().push(&field, "Option<TxOptions>", "type found, reading argument");
                let mut object: Option<TxOptions> = None;
                if !reader.is_next_nil()? {
                    object = Some(TxOptions::read(&mut reader)?);
                } else {
                    object = None;
                }
                _options = object;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_address_set {
        return Err(DecodeError::MissingField("address: String.".to_string()));
    }
    if !_method_set {
        return Err(DecodeError::MissingField("method: String.".to_string()));
    }

    Ok(ArgsCallContractMethodAndWait {
        address: _address,
        method: _method,
        args: _args,
        options: _options,
        connection: _connection,
    })
}

pub fn serialize_call_contract_method_and_wait_args(args: &ArgsCallContractMethodAndWait) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: call_contract_method_and_wait Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_call_contract_method_and_wait_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_call_contract_method_and_wait_args<W: Write>(args: &ArgsCallContractMethodAndWait, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&5)?;
    writer.context().push("address", "String", "writing property");
    writer.write_string("address")?;
    writer.write_string(&args.address)?;
    writer.context().pop();
    writer.context().push("method", "String", "writing property");
    writer.write_string("method")?;
    writer.write_string(&args.method)?;
    writer.context().pop();
    writer.context().push("args", "Option<Vec<String>>", "writing property");
    writer.write_string("args")?;
    writer.write_optional_array(&args.args, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    writer.context().push("options", "Option<TxOptions>", "writing property");
    writer.write_string("options")?;
    if args.options.is_some() {
        TxOptions::write(args.options.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_call_contract_method_and_wait_result(result: &TxReceipt) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: call_contract_method_and_wait Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_call_contract_method_and_wait_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_call_contract_method_and_wait_result<W: Write>(result: &TxReceipt, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("callContractMethodAndWait", "TxReceipt", "writing result");
    TxReceipt::write(&result, writer)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_call_contract_method_and_wait_result(result: &[u8]) -> Result<TxReceipt, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: call_contract_method_and_wait Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("callContractMethodAndWait", "TxReceipt", "reading function output");
    let object = TxReceipt::read(&mut reader)?;
    let res = object;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsSignMessage {
    pub message: String,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_sign_message_args(args: &[u8]) -> Result<ArgsSignMessage, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: sign_message Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _message: String = String::new();
    let mut _message_set = false;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "message" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _message = reader.read_string()?;
                _message_set = true;
                reader.context().pop();
            }
            "connection" => {
                reader.context().push(&field, "Option<ProviderConnection>", "type found, reading argument");
                let mut object: Option<ProviderConnection> = None;
                if !reader.is_next_nil()? {
                    object = Some(ProviderConnection::read(&mut reader)?);
                } else {
                    object = None;
                }
                _connection = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_message_set {
        return Err(DecodeError::MissingField("message: String.".to_string()));
    }

    Ok(ArgsSignMessage {
        message: _message,
        connection: _connection,
    })
}

pub fn serialize_sign_message_args(args: &ArgsSignMessage) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: sign_message Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_sign_message_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_sign_message_args<W: Write>(args: &ArgsSignMessage, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("message", "String", "writing property");
    writer.write_string("message")?;
    writer.write_string(&args.message)?;
    writer.context().pop();
    writer.context().push("connection", "Option<ProviderConnection>", "writing property");
    writer.write_string("connection")?;
    if args.connection.is_some() {
        ProviderConnection::write(args.connection.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn serialize_sign_message_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: sign_message Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_sign_message_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_sign_message_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("signMessage", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_sign_message_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: sign_message Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("signMessage", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}
