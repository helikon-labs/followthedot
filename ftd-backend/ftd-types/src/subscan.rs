use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanGetAccountByAddressBody {
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename(serialize = "camelCase"))]
pub struct SubscanAccountSearchResult {
    pub data: SubscanAccountData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename(serialize = "camelCase"))]
pub struct SubscanAccountData {
    pub account: SubscanAccount,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename(serialize = "camelCase"))]
pub struct SubscanAccount {
    pub address: String,
    pub display: Option<String>,
    pub account_display: Option<SubscanAccountDisplay>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename(serialize = "camelCase"))]
pub struct SubscanAccountDisplay {
    pub address: String,
    pub display: Option<String>,
    pub identity: Option<bool>,
    pub parent: Option<SubscanParentAccountDisplay>,
    pub merkle: Option<SubscanMerkleScienceAccountInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename(serialize = "camelCase"))]
pub struct SubscanParentAccountDisplay {
    pub address: String,
    pub display: Option<String>,
    pub sub_symbol: Option<String>,
    pub identity: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename(serialize = "camelCase"))]
pub struct SubscanMerkleScienceAccountInfo {
    pub address_type: String,
    pub tag_type: Option<String>,
    pub tag_subtype: Option<String>,
    pub tag_name: String,
}
