use crate::substrate::balance::Balance;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub address: String,
    pub display: Option<String>,
    pub email: Option<String>,
    pub legal: Option<String>,
    pub riot: Option<String>,
    pub twitter: Option<String>,
    pub web: Option<String>,
    pub is_confirmed: bool,
    pub is_invalid: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubIdentity {
    pub address: String,
    pub super_address: String,
    pub sub_display: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub address: String,
    pub identity: Option<Identity>,
    pub sub_identity: Option<SubIdentity>,
    pub super_identity: Option<Identity>,
    pub balance: Option<Balance>,
}
