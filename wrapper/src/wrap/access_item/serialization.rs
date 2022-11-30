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
use crate::AccessItem;

pub fn serialize_access_item(args: &AccessItem) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: AccessItem".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_access_item(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_access_item<W: Write>(args: &AccessItem, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("address", "String", "writing property");
    writer.write_string("address")?;
    writer.write_string(&args.address)?;
    writer.context().pop();
    writer.context().push("storageKeys", "Vec<String>", "writing property");
    writer.write_string("storageKeys")?;
    writer.write_array(&args.storage_keys, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_access_item(args: &[u8]) -> Result<AccessItem, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: AccessItem".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_access_item(&mut reader)
}

pub fn read_access_item<R: Read>(reader: &mut R) -> Result<AccessItem, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _address: String = String::new();
    let mut _address_set = false;
    let mut _storage_keys: Vec<String> = vec![];
    let mut _storage_keys_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "address" => {
                reader.context().push(&field, "String", "type found, reading property");
                _address = reader.read_string()?;
                _address_set = true;
                reader.context().pop();
            }
            "storageKeys" => {
                reader.context().push(&field, "Vec<String>", "type found, reading property");
                _storage_keys = reader.read_array(|reader| {
                    reader.read_string()
                })?;
                _storage_keys_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_address_set {
        return Err(DecodeError::MissingField("address: String.".to_string()));
    }
    if !_storage_keys_set {
        return Err(DecodeError::MissingField("storageKeys: [String].".to_string()));
    }

    Ok(AccessItem {
        address: _address,
        storage_keys: _storage_keys,
    })
}
