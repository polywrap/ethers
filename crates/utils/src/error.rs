use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeError {
    /// Error type from ethabi (ethers re-export)
    #[error("Ethabi Error: {0:?}")]
    EthabiError(ethers_core::abi::Error),
    /// Error type from abi parsing
    #[error("Parsing Error: {0:?}")]
    ParseError(ethers_core::abi::ParseError),
    /// Error type from abi parsing
    #[error("LexerError Error: {0:?}")]
    LexerError(String),
    /// Error type from abi parsing
    #[error("SerdeError Error: {0:?}")]
    SerdeError(String),
}

impl From<ethers_core::abi::Error> for EncodeError {
    fn from(src: ethers_core::abi::Error) -> Self {
        match src {
            _ => EncodeError::EthabiError(src),
        }
    }
}

impl From<ethers_core::abi::ParseError> for EncodeError {
    fn from(src: ethers_core::abi::ParseError) -> Self {
        match src {
            _ => EncodeError::ParseError(src),
        }
    }
}