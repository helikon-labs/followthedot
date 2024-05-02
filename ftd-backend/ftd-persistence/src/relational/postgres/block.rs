use super::PostgreSQLStorage;
use ftd_types::substrate::block::Block;
use sqlx::{Postgres, Transaction};

impl PostgreSQLStorage {
    pub async fn _block_exists_by_hash(&self, hash: &str) -> anyhow::Result<bool> {
        let record_count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(hash) FROM ftd_block
            WHERE hash = $1
            "#,
        )
        .bind(hash)
        .fetch_one(&self.connection_pool)
        .await?;
        Ok(record_count.0 > 0)
    }

    pub async fn block_exists_by_number(&self, number: u64) -> anyhow::Result<bool> {
        let record_count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(number) FROM ftd_block
            WHERE number = $1
            "#,
        )
        .bind(number as i64)
        .fetch_one(&self.connection_pool)
        .await?;
        Ok(record_count.0 > 0)
    }

    pub async fn _get_block_hash(&self, block_number: u64) -> anyhow::Result<Option<String>> {
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

    pub async fn get_max_block_number(&self) -> anyhow::Result<i64> {
        let max_block_number: (i64,) = sqlx::query_as(
            r#"
            SELECT COALESCE(MAX(number), -1) FROM ftd_block
            "#,
        )
        .fetch_one(&self.connection_pool)
        .await?;
        Ok(max_block_number.0)
    }

    pub async fn get_max_block_number_in_range_inclusive(
        &self,
        range: (u64, u64),
    ) -> anyhow::Result<i64> {
        let max_block_number: (i64,) = sqlx::query_as(
            r#"
            SELECT COALESCE(MAX(number), -1) FROM ftd_block WHERE number >= $1 AND number <= $2
            "#,
        )
        .bind(range.0 as i64)
        .bind(range.1 as i64)
        .fetch_one(&self.connection_pool)
        .await?;
        Ok(max_block_number.0)
    }

    pub async fn _get_min_block_number(&self) -> anyhow::Result<u64> {
        let min_block_number: (i64,) = sqlx::query_as(
            r#"
            SELECT COALESCE(MIN(number), 0) from ftd_block
            "#,
        )
        .fetch_one(&self.connection_pool)
        .await?;
        Ok(min_block_number.0 as u64)
    }

    pub async fn save_block(
        &self,
        block: &Block,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<Option<String>> {
        let maybe_result: Option<(String,)> = sqlx::query_as(
            r#"
            INSERT INTO ftd_block (hash, number, timestamp, parent_hash)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (hash) DO NOTHING
            RETURNING hash
            "#,
        )
        .bind(&block.hash)
        .bind(block.number as i64)
        .bind(block.timestamp as i64)
        .bind(&block.parent_hash)
        .fetch_optional(&mut **transaction)
        .await?;
        if let Some(result) = maybe_result {
            Ok(Some(result.0))
        } else {
            Ok(None)
        }
    }
}
