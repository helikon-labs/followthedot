use super::PostgreSQLStorage;
use sqlx::{Postgres, Transaction};

impl PostgreSQLStorage {
    pub async fn save_account(
        &self,
        address: &str,
        tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<Option<String>> {
        let maybe_result: Option<(String,)> = sqlx::query_as(
            r#"
            INSERT INTO ftd_account (address)
            VALUES ($1)
            ON CONFLICT (address) DO NOTHING
            RETURNING address
            "#,
        )
        .bind(address)
        .fetch_optional(&mut **tx)
        .await?;
        if let Some(result) = maybe_result {
            Ok(Some(result.0))
        } else {
            Ok(None)
        }
    }
}
