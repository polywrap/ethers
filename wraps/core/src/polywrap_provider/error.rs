use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum WrapperError {
//     #[error("Utils Encoder Error: {0:?}")]
//     EncodeError(ethers_utils::EncodeError),
//     #[error("Provider Error: {0:?}")]
//     ProviderError(ethers_providers::ProviderError),
//     /// Error type from abi parsing
//     #[error("ContractError Error: {0:?}")]
//     ContractError(String),
// }

// impl From<ethers_providers::ProviderError> for WrapperError {
//     fn from(src: ethers_providers::ProviderError) -> Self {
//         match src {
//             _ => WrapperError::ProviderError(src),
//         }
//     }
// }

// impl From<ethers_utils::EncodeError> for WrapperError {
//     fn from(src: ethers_utils::EncodeError) -> Self {
//         match src {
//             _ => WrapperError::EncodeError(src),
//         }
//     }
// }