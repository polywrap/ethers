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

pub fn serialize_provider_connection(args: &ProviderConnection) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: ProviderConnection".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_provider_connection(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_provider_connection<W: Write>(args: &ProviderConnection, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("node", "Option<String>", "writing property");
    writer.write_string("node")?;
    writer.write_optional_string(&args.node)?;
    writer.context().pop();
    writer.context().push("networkNameOrChainId", "Option<String>", "writing property");
    writer.write_string("networkNameOrChainId")?;
    writer.write_optional_string(&args.network_name_or_chain_id)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_provider_connection(args: &[u8]) -> Result<ProviderConnection, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: ProviderConnection".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_provider_connection(&mut reader)
}

pub fn read_provider_connection<R: Read>(reader: &mut R) -> Result<ProviderConnection, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _node: Option<String> = None;
    let mut _network_name_or_chain_id: Option<String> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "node" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _node = reader.read_optional_string()?;
                reader.context().pop();
            }
            "networkNameOrChainId" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _network_name_or_chain_id = reader.read_optional_string()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ProviderConnection {
        node: _node,
        network_name_or_chain_id: _network_name_or_chain_id,
    })
}
