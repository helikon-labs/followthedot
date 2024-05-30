use super::PostgreSQLStorage;
use ftd_types::api::transfer::Transfer;
use ftd_types::substrate::block::Block;
use ftd_types::substrate::event::TransferEvent;
use sqlx::{Postgres, Transaction};

type TransferRow = (String, i64, i64, i32, i32, i32, String, String, String);

fn row_into_transfer(row: &TransferRow) -> anyhow::Result<Transfer> {
    Ok(Transfer {
        block_hash: row.0.clone(),
        block_number: row.1 as u64,
        timestamp: row.2 as u64,
        extrinsic_index: row.3 as u16,
        extrinsic_event_index: row.4 as u16,
        event_index: row.5 as u16,
        from_address: row.6.clone(),
        to_address: row.7.clone(),
        amount: row.8.parse()?,
    })
}

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
        transfer: &TransferEvent,
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
            .bind(transfer.amount.to_string())
            .fetch_one(&mut **transaction)
            .await?;
        Ok(result.0)
    }

    pub async fn get_transfer_by_id(&self, id: i32) -> anyhow::Result<Option<TransferEvent>> {
        let result: Option<(i32, i32, i32, String, String, String)> = sqlx::query_as(
            r#"
            SELECT extrinsic_index, extrinsic_event_index, event_index, from_address, to_address, amount
            FROM ftd_transfer
            WHERE id = $1
            "#,
        )
            .bind(id)
            .fetch_optional(&self.connection_pool)
            .await?;
        if let Some(row) = result {
            Ok(Some(TransferEvent {
                extrinsic_index: row.0 as u16,
                extrinsic_event_index: row.1 as u16,
                event_index: row.2 as u16,
                from: row.3.clone(),
                to: row.4.clone(),
                amount: row.5.parse::<u128>()?,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_transfers_by_sender_and_recipient(
        &self,
        from: &str,
        to: &str,
    ) -> anyhow::Result<Vec<Transfer>> {
        let rows: Vec<TransferRow> = sqlx::query_as(
            r#"
            SELECT block_hash, block_number, timestamp, extrinsic_index, extrinsic_event_index, event_index, from_address, to_address, amount
            FROM ftd_transfer
            WHERE from_address = $1 AND to_address = $2
            ORDER BY block_number DESC, extrinsic_index DESC
            "#,
        )
            .bind(from)
            .bind(to)
            .fetch_all(&self.connection_pool)
            .await?;
        let mut transfers = Vec::new();
        for row in rows.iter() {
            transfers.push(row_into_transfer(row)?);
        }
        Ok(transfers)
    }
}
