use crate::substrate::event::Transfer;
use serde::{Deserialize, Serialize};

pub mod event;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub timestamp: u64,
    pub number: u64,
    pub hash: String,
    pub parent_hash: String,
    pub author_address: Option<String>,
    pub transfers: Vec<Transfer>,
    pub update_identities_of: Vec<String>,
}

impl Block {
    pub fn convert_to_old_dot(&self) -> Block {
        let mut block = self.clone();
        for transfer in self.transfers.iter() {
            let mut transfer = transfer.clone();
            transfer.amount /= 100;
            block.transfers.push(transfer);
        }
        block
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
    pub display: Option<String>,
    pub legal: Option<String>,
    pub web: Option<String>,
    pub riot: Option<String>,
    pub email: Option<String>,
    pub twitter: Option<String>,
    pub judgement: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubIdentity {
    pub super_address: Option<String>,
    pub sub_display: Option<String>,
}
