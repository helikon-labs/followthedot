use super::PostgreSQLStorage;
use ftd_types::substrate::event::Transfer;
use sqlx::{Postgres, Transaction};

impl PostgreSQLStorage {
    pub async fn update_transfer_volume(
        &self,
        transfer: &Transfer,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<()> {
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
        let updated_volume = if let Some(transfer_volume) = maybe_transfer_volume {
            transfer_volume.0.parse::<u128>()? + transfer.amount
        } else {
            transfer.amount
        };
        sqlx::query(
            r#"
            INSERT INTO ftd_transfer_volume (from_address, to_address, volume)
            VALUES ($1, $2, $3)
            ON CONFLICT (from_address, to_address) DO UPDATE
            SET
                volume = EXCLUDED.volume,
                updated_at = now()
            "#,
        )
        .bind(&transfer.from)
        .bind(&transfer.to)
        .bind(&updated_volume.to_string())
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }
}
