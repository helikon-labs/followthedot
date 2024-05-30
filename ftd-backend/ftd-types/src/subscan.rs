use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanGetAccountByAddressBody {
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanAccountSearchResult {
    pub data: SubscanAccountData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanAccountData {
    pub account: SubscanAccount,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanAccount {
    pub address: String,
    pub display: Option<String>,
    #[serde(rename(serialize = "accountDisplay"))]
    pub account_display: Option<SubscanAccountDisplay>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanAccountDisplay {
    pub address: String,
    pub display: Option<String>,
    pub identity: Option<bool>,
    pub parent: Option<SubscanParentAccountDisplay>,
    pub merkle: Option<SubscanMerkleScienceAccountInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanParentAccountDisplay {
    pub address: String,
    pub display: Option<String>,
    #[serde(rename(serialize = "subSymbol"))]
    pub sub_symbol: Option<String>,
    pub identity: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanMerkleScienceAccountInfo {
    #[serde(rename(serialize = "addressType"))]
    pub address_type: String,
    #[serde(rename(serialize = "tagType"))]
    pub tag_type: Option<String>,
    #[serde(rename(serialize = "tagSubtype"))]
    pub tag_subtype: Option<String>,
    #[serde(rename(serialize = "tagName"))]
    pub tag_name: String,
}
