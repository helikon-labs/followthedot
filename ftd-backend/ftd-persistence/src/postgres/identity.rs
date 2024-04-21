use crate::postgres::PostgreSQLStorage;
use ftd_types::substrate::event::IdentityChange;
use ftd_types::substrate::{Block, Identity, SubIdentity};
use sqlx::{Postgres, Transaction};

impl PostgreSQLStorage {
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
}
