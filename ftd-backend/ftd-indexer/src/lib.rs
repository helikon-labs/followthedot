use async_trait::async_trait;
use ftd_config::Config;
use ftd_persistence::Storage;
use ftd_service::Service;
use ftd_sidecar_client::SidecarClient;
use lazy_static::lazy_static;
use std::cmp::min;

mod metrics;

lazy_static! {
    static ref CONFIG: Config = Config::default();
}

#[derive(Default)]
pub struct Indexer;

#[async_trait(? Send)]
impl Service for Indexer {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (CONFIG.metrics.host.as_str(), CONFIG.metrics.indexer_port)
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        log::info!("Indexer started.");
        let storage = Storage::new().await?;
        let sidecar = SidecarClient::new(&CONFIG)?;

        let mut block_number =
            if let Some(config_start_block_number) = CONFIG.indexer.start_block_number {
                config_start_block_number
            } else {
                let db_max_block_number = storage.get_max_block_number().await?;
                if db_max_block_number < 0 {
                    0
                } else {
                    (db_max_block_number as u64) + 1
                }
            };
        loop {
            let end_block_number =
                if let Some(config_end_block_number) = CONFIG.indexer.end_block_number {
                    config_end_block_number
                } else {
                    let head = sidecar.get_head().await?;
                    log::info!("Chain head is @ {}.", head.number);
                    head.number
                };
            while block_number <= end_block_number {
                let chunk_block_number_range = block_number
                    ..=min(
                        block_number + CONFIG.indexer.chunk_size as u64 - 1,
                        end_block_number,
                    );
                let range_start_block_number = *chunk_block_number_range.start();
                let range_end_block_number = *chunk_block_number_range.end();
                let chunk_block_numbers = chunk_block_number_range.collect::<Vec<u64>>();
                let mut block_numbers = Vec::new();
                for chunk_block_number in &chunk_block_numbers {
                    if !storage.block_exists_by_number(*chunk_block_number).await? {
                        block_numbers.push(chunk_block_number);
                    }
                }
                if block_numbers.len() > 1 && block_numbers.len() == chunk_block_numbers.len() {
                    log::info!(
                        "Fetch blocks {}-{}.",
                        range_start_block_number,
                        range_end_block_number
                    );
                    let blocks = sidecar
                        .get_range_of_blocks(range_start_block_number, range_end_block_number)
                        .await?;
                    for block in blocks {
                        storage
                            .save_block(
                                block.clone(),
                                sidecar.get_block_identity_updates(&block).await?,
                            )
                            .await?;
                        log::info!("Persisted block {}.", block.number);
                    }
                } else {
                    for block_number in block_numbers {
                        let block = sidecar.get_block_by_number(*block_number).await?;
                        let block_number = block.number;
                        let identity_changes = sidecar.get_block_identity_updates(&block).await?;
                        storage.save_block(block, identity_changes).await?;
                        log::info!("Persisted block {}.", block_number);
                    }
                }
                block_number = range_end_block_number + 1;
            }
            if CONFIG.indexer.end_block_number.is_some() {
                return Ok(());
            }
            let delay_seconds = CONFIG.common.recovery_retry_seconds;
            log::info!(
                "Reached chain head. Check new head in {} seconds.",
                delay_seconds
            );
            tokio::time::sleep(std::time::Duration::from_secs(delay_seconds)).await;
        }
    }
}
