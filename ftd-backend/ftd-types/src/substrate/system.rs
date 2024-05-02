use frame_support::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemProperties {
    pub ss_58_format: u8,
    pub token_decimals: u32,
    pub token_symbol: String,
}
