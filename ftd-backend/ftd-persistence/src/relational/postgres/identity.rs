use super::PostgreSQLStorage;
use ftd_types::substrate::block::Block;
use ftd_types::substrate::event::IdentityChange;
use ftd_types::substrate::identity::{Identity, SubIdentity};
use sqlx::{Postgres, Transaction};

impl PostgreSQLStorage {
    pub async fn get_max_identity_change_id(&self) -> anyhow::Result<i32> {
        let id: (i32,) = sqlx::query_as(
            r#"
            SELECT COALESCE(MAX(id), 0) FROM ftd_identity_change
            "#,
        )
        .fetch_one(&self.connection_pool)
        .await?;
        Ok(id.0)
    }

    pub async fn save_identity_change(
        &self,
        block: &Block,
        identity_change: &IdentityChange,
        identity: &Option<Identity>,
        sub_identity: &Option<SubIdentity>,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<i32> {
        let result: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO ftd_identity_change (block_hash, block_number, timestamp, extrinsic_index, extrinsic_event_index, event_index, address, display, legal, web, riot, email, twitter, judgement, super_address, sub_display)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT (block_hash, extrinsic_index, event_index) DO NOTHING
            RETURNING id
            "#,
        )
            .bind(&block.hash)
            .bind(block.number as i64)
            .bind(block.timestamp as i64)
            .bind(identity_change.extrinsic_index as i32)
            .bind(identity_change.extrinsic_event_index as i32)
            .bind(identity_change.event_index as i32)
            .bind(&identity_change.address)
            .bind(identity.as_ref().map(|identity| &identity.display))
            .bind(identity.as_ref().map(|identity| &identity.legal))
            .bind(identity.as_ref().map(|identity| &identity.web))
            .bind(identity.as_ref().map(|identity| &identity.riot))
            .bind(identity.as_ref().map(|identity| &identity.email))
            .bind(identity.as_ref().map(|identity| &identity.twitter))
            .bind(identity.as_ref().map(|identity| &identity.judgement))
            .bind(sub_identity.as_ref().map(|sub_identity| &sub_identity.super_address))
            .bind(sub_identity.as_ref().map(|sub_identity| &sub_identity.sub_display))
            .fetch_one(&mut **transaction)
            .await?;
        Ok(result.0)
    }

    pub async fn get_identity_change_by_id(
        &self,
        id: i32,
    ) -> anyhow::Result<Option<(IdentityChange, Identity, SubIdentity)>> {
        #[allow(clippy::type_complexity)]
        let result: Option<(i32, i32, i32, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
            r#"
            SELECT extrinsic_index, extrinsic_event_index, event_index, address, display, legal, web, riot, email, twitter, judgement, super_address, sub_display
            FROM ftd_identity_change
            WHERE id = $1
            "#,
        )
            .bind(id)
            .fetch_optional(&self.connection_pool)
            .await?;
        if let Some(row) = result {
            let identity_change = IdentityChange {
                extrinsic_index: row.0 as u16,
                extrinsic_event_index: row.1 as u16,
                event_index: row.2 as u16,
                address: row.3.clone(),
            };
            let identity = Identity {
                display: row.4,
                legal: row.5,
                web: row.6,
                riot: row.7,
                email: row.8,
                twitter: row.9,
                judgement: row.10,
            };
            let sub_identity = SubIdentity {
                super_address: row.11,
                sub_display: row.12,
            };
            Ok(Some((identity_change, identity, sub_identity)))
        } else {
            Ok(None)
        }
    }
}
