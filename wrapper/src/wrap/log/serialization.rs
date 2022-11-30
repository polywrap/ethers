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
use crate::Log;

pub fn serialize_log(args: &Log) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: Log".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_log(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_log<W: Write>(args: &Log, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&9)?;
    writer.context().push("blockNumber", "BigInt", "writing property");
    writer.write_string("blockNumber")?;
    writer.write_bigint(&args.block_number)?;
    writer.context().pop();
    writer.context().push("blockHash", "String", "writing property");
    writer.write_string("blockHash")?;
    writer.write_string(&args.block_hash)?;
    writer.context().pop();
    writer.context().push("transactionIndex", "u32", "writing property");
    writer.write_string("transactionIndex")?;
    writer.write_u32(&args.transaction_index)?;
    writer.context().pop();
    writer.context().push("removed", "bool", "writing property");
    writer.write_string("removed")?;
    writer.write_bool(&args.removed)?;
    writer.context().pop();
    writer.context().push("address", "String", "writing property");
    writer.write_string("address")?;
    writer.write_string(&args.address)?;
    writer.context().pop();
    writer.context().push("data", "String", "writing property");
    writer.write_string("data")?;
    writer.write_string(&args.data)?;
    writer.context().pop();
    writer.context().push("topics", "Vec<String>", "writing property");
    writer.write_string("topics")?;
    writer.write_array(&args.topics, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    writer.context().push("transactionHash", "String", "writing property");
    writer.write_string("transactionHash")?;
    writer.write_string(&args.transaction_hash)?;
    writer.context().pop();
    writer.context().push("logIndex", "u32", "writing property");
    writer.write_string("logIndex")?;
    writer.write_u32(&args.log_index)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_log(args: &[u8]) -> Result<Log, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: Log".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_log(&mut reader)
}

pub fn read_log<R: Read>(reader: &mut R) -> Result<Log, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _block_number: BigInt = BigInt::default();
    let mut _block_number_set = false;
    let mut _block_hash: String = String::new();
    let mut _block_hash_set = false;
    let mut _transaction_index: u32 = 0;
    let mut _transaction_index_set = false;
    let mut _removed: bool = false;
    let mut _removed_set = false;
    let mut _address: String = String::new();
    let mut _address_set = false;
    let mut _data: String = String::new();
    let mut _data_set = false;
    let mut _topics: Vec<String> = vec![];
    let mut _topics_set = false;
    let mut _transaction_hash: String = String::new();
    let mut _transaction_hash_set = false;
    let mut _log_index: u32 = 0;
    let mut _log_index_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
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
            "transactionIndex" => {
                reader.context().push(&field, "u32", "type found, reading property");
                _transaction_index = reader.read_u32()?;
                _transaction_index_set = true;
                reader.context().pop();
            }
            "removed" => {
                reader.context().push(&field, "bool", "type found, reading property");
                _removed = reader.read_bool()?;
                _removed_set = true;
                reader.context().pop();
            }
            "address" => {
                reader.context().push(&field, "String", "type found, reading property");
                _address = reader.read_string()?;
                _address_set = true;
                reader.context().pop();
            }
            "data" => {
                reader.context().push(&field, "String", "type found, reading property");
                _data = reader.read_string()?;
                _data_set = true;
                reader.context().pop();
            }
            "topics" => {
                reader.context().push(&field, "Vec<String>", "type found, reading property");
                _topics = reader.read_array(|reader| {
                    reader.read_string()
                })?;
                _topics_set = true;
                reader.context().pop();
            }
            "transactionHash" => {
                reader.context().push(&field, "String", "type found, reading property");
                _transaction_hash = reader.read_string()?;
                _transaction_hash_set = true;
                reader.context().pop();
            }
            "logIndex" => {
                reader.context().push(&field, "u32", "type found, reading property");
                _log_index = reader.read_u32()?;
                _log_index_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_block_number_set {
        return Err(DecodeError::MissingField("blockNumber: BigInt.".to_string()));
    }
    if !_block_hash_set {
        return Err(DecodeError::MissingField("blockHash: String.".to_string()));
    }
    if !_transaction_index_set {
        return Err(DecodeError::MissingField("transactionIndex: UInt32.".to_string()));
    }
    if !_removed_set {
        return Err(DecodeError::MissingField("removed: Boolean.".to_string()));
    }
    if !_address_set {
        return Err(DecodeError::MissingField("address: String.".to_string()));
    }
    if !_data_set {
        return Err(DecodeError::MissingField("data: String.".to_string()));
    }
    if !_topics_set {
        return Err(DecodeError::MissingField("topics: [String].".to_string()));
    }
    if !_transaction_hash_set {
        return Err(DecodeError::MissingField("transactionHash: String.".to_string()));
    }
    if !_log_index_set {
        return Err(DecodeError::MissingField("logIndex: UInt32.".to_string()));
    }

    Ok(Log {
        block_number: _block_number,
        block_hash: _block_hash,
        transaction_index: _transaction_index,
        removed: _removed,
        address: _address,
        data: _data,
        topics: _topics,
        transaction_hash: _transaction_hash,
        log_index: _log_index,
    })
}
