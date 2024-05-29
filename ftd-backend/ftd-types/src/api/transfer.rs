use frame_support::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub block_hash: String,
    pub block_number: u64,
    pub timestamp: u64,
    pub extrinsic_index: u16,
    pub extrinsic_event_index: u16,
    pub event_index: u16,
    pub from_address: String,
    pub to_address: String,
    pub amount: u128,
}
