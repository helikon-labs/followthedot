use frame_support::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub free: u128,
    pub reserved: u128,
    pub frozen: u128,
}
