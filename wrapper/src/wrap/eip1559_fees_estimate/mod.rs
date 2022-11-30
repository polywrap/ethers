use serde::{Serialize, Deserialize};
pub mod serialization;
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    DecodeError,
    EncodeError,
    Read,
    Write,
    JSON,
};
pub use serialization::{
    deserialize_eip1559_fees_estimate,
    read_eip1559_fees_estimate,
    serialize_eip1559_fees_estimate,
    write_eip1559_fees_estimate
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Eip1559FeesEstimate {
    pub max_fee_per_gas: BigInt,
    pub max_priority_fee_per_gas: BigInt,
}

impl Eip1559FeesEstimate {
    pub fn new() -> Eip1559FeesEstimate {
        Eip1559FeesEstimate {
            max_fee_per_gas: BigInt::default(),
            max_priority_fee_per_gas: BigInt::default(),
        }
    }

    pub fn to_buffer(args: &Eip1559FeesEstimate) -> Result<Vec<u8>, EncodeError> {
        serialize_eip1559_fees_estimate(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<Eip1559FeesEstimate, DecodeError> {
        deserialize_eip1559_fees_estimate(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &Eip1559FeesEstimate, writer: &mut W) -> Result<(), EncodeError> {
        write_eip1559_fees_estimate(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Eip1559FeesEstimate, DecodeError> {
        read_eip1559_fees_estimate(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
