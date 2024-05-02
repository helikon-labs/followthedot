use crate::SidecarClient;
use ftd_types::err::IdentityDataError;
use ftd_types::substrate::block::Block;
use ftd_types::substrate::event::IdentityChange;
use ftd_types::substrate::identity::{Identity, SubIdentity};
use serde_json::Value;

fn get_judgement(value: &Value) -> anyhow::Result<Option<String>> {
    let maybe_judgement = value["judgements"]
        .as_array()
        .ok_or(IdentityDataError::JudgementsNotFound)?
        .first();
    if maybe_judgement.is_none() {
        return Ok(None);
    }
    let judgement = maybe_judgement
        .unwrap()
        .as_array()
        .ok_or(IdentityDataError::JudgementDataError)?
        .get(1)
        .ok_or(IdentityDataError::JudgementDataError)?
        .as_object()
        .ok_or(IdentityDataError::JudgementDataError)?
        .iter()
        .next()
        .ok_or(IdentityDataError::JudgementDataError)?
        .0;
    Ok(Some(judgement.clone()))
}

fn get_raw_from_info(info: &Value, key: &str) -> anyhow::Result<Option<String>> {
    let maybe_field_raw = info[key]["raw"].as_str();
    match maybe_field_raw {
        Some(field_raw) => {
            let raw_bytes: &[u8] = &hex::decode(field_raw.trim_start_matches("0x"))?;
            let raw = if let Ok(raw) = std::str::from_utf8(raw_bytes) {
                raw.to_string()
            } else {
                field_raw.to_string()
            };
            Ok(Some(raw.to_string()))
        }
        None => Ok(None),
    }
}

impl SidecarClient {
    async fn get_identity_of(&self, address: &str, at: &str) -> anyhow::Result<Option<Identity>> {
        let json = self
            .http_client
            .get(&format!(
                "{}/pallets/identity/storage/IdentityOf",
                self.base_url,
            ))
            .query(&[("keys[]", address), ("at", &format!("0x{at}"))])
            .send()
            .await?
            .json::<Value>()
            .await?;
        let mut value = &json["value"];
        if let Some(array) = value.as_array() {
            if let Some(array_element) = array.first() {
                value = array_element;
            } else {
                return Ok(None);
            }
        }
        if value.is_null() || !value.is_object() {
            return Ok(None);
        }
        let judgement = get_judgement(value)?;
        let info = &value["info"];
        if info.is_null() || !info.is_object() {
            return Ok(None);
        }

        let display = get_raw_from_info(info, "display")?;
        let legal = get_raw_from_info(info, "legal")?;
        let web = get_raw_from_info(info, "web")?;
        let riot = get_raw_from_info(info, "riot")?;
        let email = get_raw_from_info(info, "email")?;
        let twitter = get_raw_from_info(info, "twitter")?;
        Ok(Some(Identity {
            display,
            legal,
            web,
            riot,
            email,
            twitter,
            judgement,
        }))
    }

    async fn get_sub_identity_of(
        &self,
        address: &str,
        at: &str,
    ) -> anyhow::Result<Option<SubIdentity>> {
        let json = self
            .http_client
            .get(&format!(
                "{}/pallets/identity/storage/SuperOf",
                self.base_url,
            ))
            .query(&[("keys[]", address), ("at", &format!("0x{at}"))])
            .send()
            .await?
            .json::<Value>()
            .await?;
        let value = &json["value"];
        if value.is_null() || !value.is_array() {
            return Ok(None);
        }

        let super_address = value[0].as_str().map(|str| str.to_string());
        let sub_display = if let Some(sub_display_raw) = value[1]["raw"].as_str() {
            let raw_bytes: &[u8] = &hex::decode(sub_display_raw.trim_start_matches("0x"))?;
            if let Ok(raw) = std::str::from_utf8(raw_bytes) {
                Some(raw.to_string())
            } else {
                Some(sub_display_raw.to_string())
            }
        } else {
            None
        };
        Ok(Some(SubIdentity {
            super_address,
            sub_display,
        }))
    }

    pub async fn get_block_identity_updates(
        &self,
        block: &Block,
    ) -> anyhow::Result<Vec<(IdentityChange, Option<Identity>, Option<SubIdentity>)>> {
        let mut identity_updates: Vec<(IdentityChange, Option<Identity>, Option<SubIdentity>)> =
            Vec::new();
        for identity_change in block.identity_changes.iter() {
            identity_updates.push((
                identity_change.clone(),
                self.get_identity_of(&identity_change.address, &block.hash)
                    .await?,
                self.get_sub_identity_of(&identity_change.address, &block.hash)
                    .await?,
            ));
        }
        Ok(identity_updates)
    }
}
