use crate::substrate::event::Transfer;
use serde::{Deserialize, Serialize};

pub mod event;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub timestamp: u64,
    pub number: u64,
    pub hash: String,
    pub parent_hash: String,
    pub author_address: String,
    pub transfers: Vec<Transfer>,
}
