use super::provider::PolywrapProvider;
use super::signer::PolywrapSigner;
use ethers_middleware::{signer::SignerMiddlewareError, SignerMiddleware};
use thiserror::Error;

#[derive(Error, Debug)]
/// Error thrown when sending an HTTP request
pub enum WrapperError {
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
    #[error("Provider Error: {0:?}")]
    ProviderError(ethers_providers::ProviderError),
    /// Error type from abi parsing
    #[error("Middleware Error: {0:?}")]
    MiddlewareError(
        SignerMiddlewareError<ethers_providers::Provider<PolywrapProvider>, PolywrapSigner>,
    ),
    /// Error type from abi parsing
    #[error("ContractError Error: {0:?}")]
    ContractError(String),
}

impl From<ethers_core::abi::Error> for WrapperError {
    fn from(src: ethers_core::abi::Error) -> Self {
        match src {
            _ => WrapperError::EthabiError(src),
        }
    }
}

impl From<ethers_core::abi::ParseError> for WrapperError {
    fn from(src: ethers_core::abi::ParseError) -> Self {
        match src {
            _ => WrapperError::ParseError(src),
        }
    }
}

impl From<ethers_providers::ProviderError> for WrapperError {
    fn from(src: ethers_providers::ProviderError) -> Self {
        match src {
            _ => WrapperError::ProviderError(src),
        }
    }
}

impl From<SignerMiddlewareError<ethers_providers::Provider<PolywrapProvider>, PolywrapSigner>>
    for WrapperError
{
    fn from(
        src: SignerMiddlewareError<ethers_providers::Provider<PolywrapProvider>, PolywrapSigner>,
    ) -> Self {
        match src {
            _ => WrapperError::MiddlewareError(src),
        }
    }
}
