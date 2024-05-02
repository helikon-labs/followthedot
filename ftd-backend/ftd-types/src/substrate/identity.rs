use crate::substrate::account_id::AccountId;
use crate::substrate::balance::Balance;
use frame_support::pallet_prelude::ConstU32;
use pallet_identity::legacy::IdentityInfo;
use pallet_identity::{Data, Judgement, Registration};
use parity_scale_codec::Decode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
    pub display: Option<String>,
    pub legal: Option<String>,
    pub web: Option<String>,
    pub riot: Option<String>,
    pub email: Option<String>,
    pub twitter: Option<String>,
    pub judgement: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubIdentity {
    pub super_address: Option<String>,
    pub sub_display: Option<String>,
}

pub fn data_to_string(data: Data) -> Option<String> {
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

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct IdentityRegistration {
    pub account_id: AccountId,
    pub display: Option<String>,
    pub email: Option<String>,
    pub riot: Option<String>,
    pub twitter: Option<String>,
    pub web: Option<String>,
    pub confirmed: bool,
}

impl IdentityRegistration {
    pub fn from_bytes(account_id: AccountId, mut bytes: &[u8]) -> anyhow::Result<Self> {
        let registration: Registration<
            Balance,
            ConstU32<{ u32::MAX }>,
            IdentityInfo<ConstU32<{ u32::MAX }>>,
        > = Decode::decode(&mut bytes)?;
        let display = data_to_string(registration.info.display);
        let email = data_to_string(registration.info.email);
        let riot = data_to_string(registration.info.riot);
        let twitter = data_to_string(registration.info.twitter);
        let web = data_to_string(registration.info.web);
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
        Ok(IdentityRegistration {
            account_id,
            display,
            email,
            riot,
            twitter,
            web,
            confirmed,
        })
    }
}

pub type SuperAccountId = (AccountId, Data);
