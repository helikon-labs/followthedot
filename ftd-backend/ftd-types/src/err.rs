//! Error types.
use serde::{Deserialize, Serialize};

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
pub enum IdentityEventDataError {
    #[error("Identity event account address not found.")]
    AccountNotFound,
    #[error("Identity event super account address not found.")]
    SuperAccountNotFound,
    #[error("Identity event sub account address not found.")]
    SubAccountNotFound,
    #[error("Identity event deposit not found.")]
    DepositNotFound,
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum IdentityDataError {
    #[error("Identity judgements not found.")]
    JudgementsNotFound,
    #[error("Unexpected judgement data structure.")]
    JudgementDataError,
}
