use ftd_config::Config;
use sqlx::{Pool, Postgres, Transaction};
use std::time::Duration;

pub mod account;
pub mod block;
pub mod identity;
pub mod subscan;
pub mod transfer;
pub mod transfer_volume;

pub(crate) struct PostgreSQLStorage {
    connection_pool: Pool<Postgres>,
}

impl PostgreSQLStorage {
    pub async fn new(config: &Config) -> anyhow::Result<PostgreSQLStorage> {
        log::info!("Establishing PostgreSQL connection pool...");
        let connection_pool = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(
                config.postgres.connection_timeout_seconds,
            ))
            .max_connections(config.postgres.pool_max_connections)
            .connect(&config.get_postgres_url())
            .await?;
        log::info!("PostgreSQL connection pool established.");
        Ok(PostgreSQLStorage { connection_pool })
    }

    pub async fn begin_tx(&self) -> anyhow::Result<Transaction<Postgres>> {
        Ok(self.connection_pool.begin().await?)
    }

    pub async fn commit_tx(&self, tx: Transaction<'_, Postgres>) -> anyhow::Result<()> {
        tx.commit().await?;
        Ok(())
    }
}
