use crate::PostgreSQLStorage;

impl PostgreSQLStorage {
    pub async fn get_block_hash(&self, block_number: u64) -> anyhow::Result<Option<String>> {
        Ok(sqlx::query_as(
            r#"
            SELECT hash FROM ftd_block
            WHERE "number" = $1
            "#,
        )
        .bind(block_number as i64)
        .fetch_optional(&self.connection_pool)
        .await?
        .map(|hash: (String,)| hash.0))
    }
}
