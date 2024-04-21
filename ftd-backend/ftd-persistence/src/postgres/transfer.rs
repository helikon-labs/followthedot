use super::PostgreSQLStorage;
use ftd_types::substrate::event::Transfer;
use ftd_types::substrate::Block;
use sqlx::{Postgres, Transaction};

impl PostgreSQLStorage {
    pub async fn save_transfer(
        &self,
        block: &Block,
        transfer: &Transfer,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<i32> {
        let result: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO ftd_transfer (block_hash, block_number, timestamp, extrinsic_index, extrinsic_event_index, event_index, from_address, to_address, amount)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (block_hash, extrinsic_index, event_index) DO NOTHING
            RETURNING id
            "#,
        )
            .bind(&block.hash)
            .bind(block.number as i64)
            .bind(block.timestamp as i64)
            .bind(transfer.extrinsic_index as i32)
            .bind(transfer.extrinsic_event_index as i32)
            .bind(transfer.event_index as i32)
            .bind(&transfer.from)
            .bind(&transfer.to)
            .bind(&transfer.amount.to_string())
            .fetch_one(&mut **transaction)
            .await?;
        Ok(result.0)
    }
}
