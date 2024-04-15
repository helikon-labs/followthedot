use async_trait::async_trait;
use ftd_config::Config;
use ftd_persistence::PostgreSQLStorage;
use ftd_service::Service;
use ftd_sidecar_client::SidecarClient;
use lazy_static::lazy_static;
use std::sync::atomic::AtomicBool;

mod metrics;

lazy_static! {
    static ref CONFIG: Config = Config::default();
    static ref IS_BUSY: AtomicBool = AtomicBool::new(false);
}

#[derive(Default)]
pub struct Indexer;

impl Indexer {}

#[async_trait(?Send)]
impl Service for Indexer {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (CONFIG.metrics.host.as_str(), CONFIG.metrics.indexer_port)
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        loop {
            log::info!("Indexer started.");
            let storage = PostgreSQLStorage::new(&CONFIG).await?;
            let hash = storage.get_block_hash(15).await?;
            log::info!("Block hash: {}", hash.unwrap_or("NULL".to_string()));
            let sidecar_client = SidecarClient::new(&CONFIG)?;
            sidecar_client.do_stuff().await?;
            let delay_seconds = CONFIG.common.recovery_retry_seconds;
            log::info!(
                "Indexer ended. Will restart after {} seconds.",
                delay_seconds
            );
            tokio::time::sleep(std::time::Duration::from_secs(delay_seconds)).await;
        }
    }
}
