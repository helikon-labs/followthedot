use super::PostgreSQLStorage;
use ftd_types::substrate::event::Transfer;
use sqlx::{Postgres, Transaction};

impl PostgreSQLStorage {
    pub async fn update_transfer_volume(
        &self,
        transfer: &Transfer,
        transaction: &mut Transaction<'_, Postgres>,
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
        .fetch_optional(&mut **transaction)
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
        .bind(&volume.to_string())
        .fetch_one(&mut **transaction)
        .await?;
        Ok((volume, result.0 as u32))
    }
}
