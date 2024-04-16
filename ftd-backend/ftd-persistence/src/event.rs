use crate::PostgreSQLStorage;
use ftd_types::substrate::event::Transfer;
use ftd_types::substrate::Block;

impl PostgreSQLStorage {
    pub async fn save_transfer_event(
        &self,
        block: &Block,
        transfer: &Transfer,
    ) -> anyhow::Result<i32> {
        self.save_account(&transfer.from, &None, &None).await?;
        self.save_account(&transfer.to, &None, &None).await?;
        let result: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO ftd_transfer (block_hash, timestamp, extrinsic_index, extrinsic_event_index, event_index, from_address, to_address, amount)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#,
        )
            .bind(&block.hash)
            .bind(block.timestamp as i64)
            .bind(transfer.extrinsic_index as i32)
            .bind(transfer.extrinsic_event_index as i32)
            .bind(transfer.event_index as i32)
            .bind(&transfer.from)
            .bind(&transfer.to)
            .bind(&transfer.amount.to_string())
            .fetch_one(&self.connection_pool)
            .await?;
        Ok(result.0)
    }
}
