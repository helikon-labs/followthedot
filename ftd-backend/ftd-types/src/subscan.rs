use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanAccountListBody {
    pub order: String,
    pub order_field: String,
    pub page: u32,
    pub row: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanAccountListResult {
    pub code: u32,
    pub data: Option<SubscanAccountList>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanAccountList {
    pub count: u64,
    pub list: Vec<SubscanAccount>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename(serialize = "accountDisplay"))]
    pub account_display: SubscanAccountDisplay,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanAccountDisplay {
    pub address: String,
    #[serde(rename(serialize = "accountIndex"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<SubscanParentAccountDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkle: Option<SubscanMerkleScienceAccountInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanParentAccountDisplay {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename(serialize = "subSymbol"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscanMerkleScienceAccountInfo {
    #[serde(rename(serialize = "addressType"))]
    pub address_type: String,
    #[serde(rename(serialize = "tagType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_type: Option<String>,
    #[serde(rename(serialize = "tagSubtype"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_subtype: Option<String>,
    #[serde(rename(serialize = "tagName"))]
    pub tag_name: String,
}
