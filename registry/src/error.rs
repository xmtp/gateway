//! Error variants for Registry

use std::num::TryFromIntError;

use ethers::{
    contract::ContractError,
    providers::{Middleware, ProviderError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContactOperationError<M: Middleware> {
    #[error("Invalid address {0}")]
    BadAddress(#[from] rustc_hex::FromHexError),
    #[error(transparent)]
    ContractError(#[from] ContractError<M>),
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
    #[error("Error converting from int: {0}")]
    IntConversion(#[from] TryFromIntError),
    #[error("Error Resolving {1}: {0}")]
    ResolutionError(lib_didethresolver::error::ResolverError<M>, String),
    #[error("The DID has been deactivated, and no longer valid")]
    DIDDeactivated,
    #[error("Type failed to convert")]
    Type(#[from] lib_didethresolver::error::TypeError),
    #[error("Error verifying key package: {0}")]
    KeyPackage(#[from] xmtp_mls::verified_key_package::KeyPackageVerificationError),
    #[error("Error parsing hex-encoded bytes: {0}")]
    ParseBytes(#[from] ethers::types::ParseBytesError),
}
