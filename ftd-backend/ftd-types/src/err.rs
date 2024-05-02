//! Error types.
use serde::{Deserialize, Serialize};
use sp_core::bytes::FromHexError;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ServiceError {
    pub description: String,
}

impl ServiceError {
    pub fn from(description: &str) -> ServiceError {
        ServiceError {
            description: description.to_string(),
        }
    }
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum BlockDataError {
    #[error("Block array not found.")]
    BlockArrayNotFound,
    #[error("Block hash not found.")]
    HashNotFound,
    #[error("Block parent hash not found.")]
    ParentHashNotFound,
    #[error("Block number not found.")]
    BlockNumberNotFound,
    #[error("Timestamp not found.")]
    TimestampNotFound,
    #[error("Extrinsics not found.")]
    ExtrinsicsNotFound,
    #[error("Extrinsic events not found.")]
    ExtrinsicEventsNotFound,
    #[error("Event module not found.")]
    EventModuleNotFound,
    #[error("Event name not found.")]
    EventNameNotFound,
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum TransferEventDataError {
    #[error("From account not found.")]
    FromNotFound,
    #[error("To account not found.")]
    ToNotFound,
    #[error("Transfer amount not found.")]
    AmountNotFound,
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum DecodeError {
    #[error("Decode error: {0}")]
    Error(String),
}

impl From<FromHexError> for DecodeError {
    fn from(error: FromHexError) -> Self {
        Self::Error(error.to_string())
    }
}

impl From<parity_scale_codec::Error> for DecodeError {
    fn from(error: parity_scale_codec::Error) -> Self {
        Self::Error(error.to_string())
    }
}
