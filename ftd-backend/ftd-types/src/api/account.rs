use crate::api::identity::{Identity, SubIdentity};
use crate::graph::TransferVolume;
use crate::substrate::balance::Balance;
use frame_support::{Deserialize, Serialize};
use crate::subscan::SubscanAccount;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub address: String,
    pub identity: Option<Identity>,
    pub sub_identity: Option<SubIdentity>,
    pub super_identity: Option<Identity>,
    pub balance: Option<Balance>,
    pub subscan_account: Option<SubscanAccount>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountGraph {
    pub accounts: Vec<Account>,
    pub transfer_volumes: Vec<TransferVolume>,
}
