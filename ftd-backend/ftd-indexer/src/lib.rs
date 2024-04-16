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

impl Indexer {
    async fn update_identity_of(
        &self,
        storage: &PostgreSQLStorage,
        sidecar_client: &SidecarClient,
        address: &str,
        block_hash: &str,
    ) -> anyhow::Result<()> {
        let identity = sidecar_client.get_identity_of(address, block_hash).await?;
        let sub_identity = sidecar_client
            .get_sub_identity_of(address, block_hash)
            .await?;
        storage
            .save_account(address, &identity, &sub_identity)
            .await?;
        log::info!("Updated identity of {address}.");
        Ok(())
    }
}

#[async_trait(?Send)]
impl Service for Indexer {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (CONFIG.metrics.host.as_str(), CONFIG.metrics.indexer_port)
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        log::info!("Indexer started.");
        let storage = PostgreSQLStorage::new(&CONFIG).await?;
        let sidecar_client = SidecarClient::new(&CONFIG)?;
        loop {
            // process block
            let head = sidecar_client.get_head().await?;
            log::info!("Chain head is @ {}.", head.number);
            let db_max_block_number = storage.get_max_block_number().await?;
            let start_block_number = if db_max_block_number < 0 {
                0
            } else {
                db_max_block_number + 1
            };
            let mut block_number = start_block_number as u64;
            while block_number < head.number {
                log::info!("Fetch block {}.", block_number);
                let block = sidecar_client.get_block_by_number(block_number).await?;
                storage.save_block(&block).await?;
                log::info!("Persisted block {}.", block.number);
                for address in block.update_identities_of.iter() {
                    self.update_identity_of(&storage, &sidecar_client, address, &block.hash)
                        .await?;
                }
                block_number += 1;
            }
            if !storage.block_exists_by_hash(&head.hash).await? {
                storage.save_block(&head).await?;
                log::info!("Persisted block {}.", head.number);
                for address in head.update_identities_of.iter() {
                    self.update_identity_of(&storage, &sidecar_client, address, &head.hash)
                        .await?;
                }
            }
            let delay_seconds = CONFIG.common.recovery_retry_seconds;
            log::info!(
                "Indexer completed. Will restart in {} seconds.",
                delay_seconds
            );
            tokio::time::sleep(std::time::Duration::from_secs(delay_seconds)).await;
        }
    }
}
