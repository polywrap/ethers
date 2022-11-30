use serde::{Serialize, Deserialize};
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsRequest {
    pub method: String,
    pub params: Option<String>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_request_args(args: &[u8]) -> Result<ArgsRequest, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: request Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _method: String = String::new();
    let mut _method_set = false;
    let mut _params: Option<String> = None;
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
                reader.context().push(&field, "Option<String>", "type found, reading argument");
                _params = reader.read_optional_string()?;
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

    Ok(ArgsRequest {
        method: _method,
        params: _params,
        connection: _connection,
    })
}

pub fn serialize_request_args(args: &ArgsRequest) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: request Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_request_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_request_args<W: Write>(args: &ArgsRequest, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("method", "String", "writing property");
    writer.write_string("method")?;
    writer.write_string(&args.method)?;
    writer.context().pop();
    writer.context().push("params", "Option<String>", "writing property");
    writer.write_string("params")?;
    writer.write_optional_string(&args.params)?;
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

pub fn serialize_request_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: request Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_request_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_request_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("request", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_request_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: request Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("request", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsSignMessage {
    pub message: Vec<u8>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_sign_message_args(args: &[u8]) -> Result<ArgsSignMessage, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: sign_message Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _message: Vec<u8> = vec![];
    let mut _message_set = false;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "message" => {
                reader.context().push(&field, "Vec<u8>", "type found, reading argument");
                _message = reader.read_bytes()?;
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
        return Err(DecodeError::MissingField("message: Bytes.".to_string()));
    }

    Ok(ArgsSignMessage {
        message: _message,
        connection: _connection,
    })
}

pub fn serialize_sign_message_args(args: &ArgsSignMessage) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: sign_message Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_sign_message_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_sign_message_args<W: Write>(args: &ArgsSignMessage, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("message", "Vec<u8>", "writing property");
    writer.write_string("message")?;
    writer.write_bytes(&args.message)?;
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
    encoder_context.description = "Serializing (encoding) imported module-type: sign_message Result".to_string();
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
    context.description = "Deserializing imported module-type: sign_message Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("signMessage", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsSignTransaction {
    pub rlp: Vec<u8>,
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_sign_transaction_args(args: &[u8]) -> Result<ArgsSignTransaction, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: sign_transaction Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _rlp: Vec<u8> = vec![];
    let mut _rlp_set = false;
    let mut _connection: Option<ProviderConnection> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "rlp" => {
                reader.context().push(&field, "Vec<u8>", "type found, reading argument");
                _rlp = reader.read_bytes()?;
                _rlp_set = true;
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
    if !_rlp_set {
        return Err(DecodeError::MissingField("rlp: Bytes.".to_string()));
    }

    Ok(ArgsSignTransaction {
        rlp: _rlp,
        connection: _connection,
    })
}

pub fn serialize_sign_transaction_args(args: &ArgsSignTransaction) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: sign_transaction Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_sign_transaction_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_sign_transaction_args<W: Write>(args: &ArgsSignTransaction, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("rlp", "Vec<u8>", "writing property");
    writer.write_string("rlp")?;
    writer.write_bytes(&args.rlp)?;
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

pub fn serialize_sign_transaction_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: sign_transaction Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_sign_transaction_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_sign_transaction_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("signTransaction", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_sign_transaction_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: sign_transaction Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("signTransaction", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsAddress {
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_address_args(args: &[u8]) -> Result<ArgsAddress, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: address Args".to_string();

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

    Ok(ArgsAddress {
        connection: _connection,
    })
}

pub fn serialize_address_args(args: &ArgsAddress) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: address Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_address_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_address_args<W: Write>(args: &ArgsAddress, writer: &mut W) -> Result<(), EncodeError> {
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

pub fn serialize_address_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: address Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_address_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_address_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("address", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_address_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: address Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("address", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsChainId {
    pub connection: Option<ProviderConnection>,
}

pub fn deserialize_chain_id_args(args: &[u8]) -> Result<ArgsChainId, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: chain_id Args".to_string();

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

    Ok(ArgsChainId {
        connection: _connection,
    })
}

pub fn serialize_chain_id_args(args: &ArgsChainId) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: chain_id Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_chain_id_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_chain_id_args<W: Write>(args: &ArgsChainId, writer: &mut W) -> Result<(), EncodeError> {
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

pub fn serialize_chain_id_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: chain_id Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_chain_id_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_chain_id_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("chainId", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_chain_id_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: chain_id Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("chainId", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}
