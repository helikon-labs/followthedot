use ftd_config::Config;
use sqlx::{Pool, Postgres};
use std::time::Duration;

pub mod block;

pub struct PostgreSQLStorage {
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
}