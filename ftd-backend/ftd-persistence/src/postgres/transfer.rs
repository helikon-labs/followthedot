use super::PostgreSQLStorage;
use ftd_types::substrate::event::Transfer;
use ftd_types::substrate::Block;
use sqlx::{Postgres, Transaction};

impl PostgreSQLStorage {
    pub async fn get_max_transfer_id(&self) -> anyhow::Result<i32> {
        let id: (i32,) = sqlx::query_as(
            r#"
            SELECT COALESCE(MAX(id), 0) FROM ftd_transfer
            "#,
        )
        .fetch_one(&self.connection_pool)
        .await?;
        Ok(id.0)
    }

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

    pub async fn get_transfer_by_id(&self, id: i32) -> anyhow::Result<Transfer> {
        let result: (i32, i32, i32, String, String, String) = sqlx::query_as(
            r#"
            SELECT extrinsic_index, extrinsic_event_index, event_index, from_address, to_address, amount
            FROM ftd_transfer
            WHERE id = $1
            "#,
        )
            .bind(id)
            .fetch_one(&self.connection_pool)
            .await?;
        Ok(Transfer {
            extrinsic_index: result.0 as u16,
            extrinsic_event_index: result.1 as u16,
            event_index: result.2 as u16,
            from: result.3.clone(),
            to: result.4.clone(),
            amount: result.5.parse::<u128>()?,
        })
    }
}
