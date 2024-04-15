use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transfer {
    pub extrinsic_index: u16,
    pub extrinsic_event_index: u16,
    pub event_index: u16,
    pub from: String,
    pub to: String,
    pub amount: u128,
}
