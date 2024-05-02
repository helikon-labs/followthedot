use crate::substrate::account_id::AccountId;
use crate::substrate::balance::Balance;
use frame_support::pallet_prelude::ConstU32;
use pallet_identity::legacy::IdentityInfo;
use pallet_identity::{Data, Judgement, Registration};
use parity_scale_codec::Decode;
use serde::{Deserialize, Serialize};

pub fn identity_data_to_string(data: Data) -> Option<String> {
    match data {
        Data::Raw(raw) => {
            let maybe_string = String::from_utf8(raw.into_inner());
            if let Ok(string) = maybe_string {
                Some(string)
            } else {
                None
            }
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
}

impl Identity {
    pub fn from_bytes(account_id: AccountId, mut bytes: &[u8]) -> anyhow::Result<Self> {
        let registration: Registration<
            Balance,
            ConstU32<{ u32::MAX }>,
            IdentityInfo<ConstU32<{ u32::MAX }>>,
        > = Decode::decode(&mut bytes)?;
        let display = identity_data_to_string(registration.info.display);
        let email = identity_data_to_string(registration.info.email);
        let legal = identity_data_to_string(registration.info.legal);
        let riot = identity_data_to_string(registration.info.riot);
        let twitter = identity_data_to_string(registration.info.twitter);
        let web = identity_data_to_string(registration.info.web);
        let mut confirmed = true;
        for judgement in registration.judgements {
            confirmed &= match judgement.1 {
                Judgement::Reasonable | Judgement::KnownGood => true,
                Judgement::Unknown => false,
                Judgement::FeePaid(_) => false,
                Judgement::OutOfDate => false,
                Judgement::LowQuality => false,
                Judgement::Erroneous => false,
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
            is_confirmed: confirmed,
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
