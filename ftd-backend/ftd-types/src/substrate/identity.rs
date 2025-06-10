use crate::substrate::account_id::AccountId;
use frame_support::pallet_prelude::{ConstU32, Encode};
use frame_support::BoundedVec;
use pallet_identity::{Data, Judgement};
use parity_scale_codec::Decode;
use serde::{Deserialize, Serialize};

pub fn identity_data_to_string(data: Data) -> Option<String> {
    match data {
        Data::Raw(raw) => {
            let maybe_string = String::from_utf8(raw.into_inner());
            maybe_string.ok()
        }
        _ => None,
    }
}

#[derive(Clone, Default, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Identity {
    pub account_id: AccountId,
    pub display: Option<String>,
    pub email: Option<String>,
    pub legal: Option<String>,
    pub riot: Option<String>,
    pub twitter: Option<String>,
    pub web: Option<String>,
    pub is_confirmed: bool,
    pub is_invalid: bool,
}

#[derive(Clone, Debug, Decode, Encode)]
pub struct IdentityInfo {
    pub display: Data,
    pub legal: Data,
    pub web: Data,
    pub riot: Data,
    pub email: Data,
    pub pgp_fingerprint: Option<[u8; 20]>,
    pub image: Data,
    pub twitter: Data,
    pub github: Data,
    pub discord: Data,
}

#[derive(Clone, Debug, Decode, Encode)]
struct Registration {
    pub judgements: BoundedVec<(u32, Judgement<u128>), ConstU32<{ u32::MAX }>>,
    pub deposit: u128,
    pub info: IdentityInfo,
}

impl Identity {
    pub fn from_bytes(account_id: AccountId, mut bytes: &[u8]) -> anyhow::Result<Self> {
        let registration: Registration = Decode::decode(&mut bytes)?;
        let display = identity_data_to_string(registration.info.display);
        let email = identity_data_to_string(registration.info.email);
        let legal = identity_data_to_string(registration.info.legal);
        let riot = identity_data_to_string(registration.info.riot);
        let twitter = identity_data_to_string(registration.info.twitter);
        let web = identity_data_to_string(registration.info.web);
        let mut is_confirmed = false;
        let mut is_invalid = false;
        for judgement in registration.judgements.iter() {
            match judgement.1 {
                Judgement::Reasonable | Judgement::KnownGood => {
                    is_confirmed |= true;
                    is_invalid |= false;
                }
                Judgement::Unknown => {
                    is_confirmed |= false;
                    is_invalid |= false;
                }
                Judgement::FeePaid(_) => {
                    is_confirmed |= false;
                    is_invalid |= false;
                }
                Judgement::OutOfDate => {
                    is_confirmed |= false;
                    is_invalid |= true;
                }
                Judgement::LowQuality => {
                    is_confirmed |= false;
                    is_invalid |= true;
                }
                Judgement::Erroneous => {
                    is_confirmed |= false;
                    is_invalid |= true;
                }
            };
        }
        Ok(Identity {
            account_id,
            display,
            email,
            legal,
            riot,
            twitter,
            web,
            is_confirmed,
            is_invalid,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubIdentity {
    pub account_id: AccountId,
    pub super_account_id: AccountId,
    pub sub_display: Option<String>,
}

impl SubIdentity {
    pub fn from_bytes(account_id: AccountId, mut bytes: &[u8]) -> anyhow::Result<Self> {
        let sub_info: (AccountId, Data) = Decode::decode(&mut bytes)?;
        Ok(SubIdentity {
            account_id,
            super_account_id: sub_info.0,
            sub_display: identity_data_to_string(sub_info.1),
        })
    }
}
