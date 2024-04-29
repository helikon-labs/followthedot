use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GraphUpdaterState {
    pub last_processed_transfer_id: i32,
    pub last_processed_identity_change_id: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransferVolume {
    pub from: String,
    pub to: String,
    pub count: u128,
    pub volume: u32,
}
