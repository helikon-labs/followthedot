use super::PostgreSQLStorage;
use ftd_types::substrate::event::TransferEvent;

impl PostgreSQLStorage {
    pub async fn get_transfer_volume_updater_last_processed_transfer_id(
        &self,
    ) -> anyhow::Result<i32> {
        let last_processed_transfer_id: (i32,) = sqlx::query_as(
            r#"
            SELECT last_processed_transfer_id FROM ftd_transfer_volume_updater_state LIMIT 1
            "#,
        )
        .fetch_one(&self.connection_pool)
        .await?;
        Ok(last_processed_transfer_id.0)
    }

    pub async fn set_transfer_volume_updater_last_processed_transfer_id(
        &self,
        id: i32,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE ftd_transfer_volume_updater_state
            SET last_processed_transfer_id = $1, updated_at = now()
            WHERE id = 1
            "#,
        )
        .bind(id)
        .execute(&self.connection_pool)
        .await?;
        Ok(())
    }

    pub async fn update_transfer_volume(
        &self,
        transfer: &TransferEvent,
    ) -> anyhow::Result<(u128, u32)> {
        let maybe_transfer_volume: Option<(String,)> = sqlx::query_as(
            r#"
            SELECT volume
            FROM ftd_transfer_volume
            WHERE from_address = $1 AND to_address = $2
            "#,
        )
        .bind(&transfer.from)
        .bind(&transfer.to)
        .fetch_optional(&self.connection_pool)
        .await?;
        let volume = if let Some(transfer_volume) = maybe_transfer_volume {
            transfer_volume.0.parse::<u128>()? + transfer.amount
        } else {
            transfer.amount
        };
        let result: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO ftd_transfer_volume (from_address, to_address, volume)
            VALUES ($1, $2, $3)
            ON CONFLICT (from_address, to_address) DO UPDATE
            SET
                volume = EXCLUDED.volume,
                count = ftd_transfer_volume.count + 1,
                updated_at = now()
            RETURNING count
            "#,
        )
        .bind(&transfer.from)
        .bind(&transfer.to)
        .bind(volume.to_string())
        .fetch_one(&self.connection_pool)
        .await?;
        Ok((volume, result.0 as u32))
    }
}
