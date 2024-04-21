use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transfer {
    pub extrinsic_index: u16,
    pub extrinsic_event_index: u16,
    pub event_index: u16,
    pub from: String,
    pub to: String,
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdentityChange {
    pub extrinsic_index: u16,
    pub extrinsic_event_index: u16,
    pub event_index: u16,
    pub address: String,
}
