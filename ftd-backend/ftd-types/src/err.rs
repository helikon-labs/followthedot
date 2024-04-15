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
    #[error("Block author id not found.")]
    AuthorIdNotFound,
    #[error("Block number not found.")]
    BlockNumberNotFound,
    #[error("Timestamp not found.")]
    TimestampNotFound,
}
