use frame_support::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Balance {
    pub free: u128,
    pub reserved: u128,
    pub frozen: u128,
}
