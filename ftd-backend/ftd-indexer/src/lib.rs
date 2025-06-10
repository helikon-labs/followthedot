use async_trait::async_trait;
use ftd_config::Config;
use ftd_persistence::relational::RelationalStorage;
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
        let storage = RelationalStorage::new().await?;
        let sidecar = SidecarClient::new(&CONFIG)?;

        let mut block_number =
            if let Some(config_start_block_number) = CONFIG.indexer.start_block_number {
                log::info!("Config start block number {config_start_block_number}.");
                if let Some(config_end_block_number) = CONFIG.indexer.end_block_number {
                    let block_number = storage
                        .get_max_block_number_in_range_inclusive((
                            config_start_block_number,
                            config_end_block_number,
                        ))
                        .await?;
                    if block_number > 0 {
                        (block_number + 1) as u64
                    } else {
                        config_start_block_number
                    }
                } else {
                    config_start_block_number
                }
            } else {
                let db_max_block_number = storage.get_max_block_number().await?;
                if db_max_block_number < 0 {
                    0
                } else {
                    (db_max_block_number as u64) + 1
                }
            };
        log::info!("Start @ block number {block_number}.");
        loop {
            let end_block_number =
                if let Some(config_end_block_number) = CONFIG.indexer.end_block_number {
                    log::info!("End @ block number {config_end_block_number}.");
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
                    log::info!("Fetch blocks {range_start_block_number}-{range_end_block_number}.");
                    let start = std::time::Instant::now();
                    let blocks = sidecar
                        .get_range_of_blocks(range_start_block_number, range_end_block_number)
                        .await?;
                    for block in &blocks {
                        storage.save_block(block.clone()).await?;
                        metrics::indexed_finalized_block_number().set(block.number as i64);
                        log::info!("Persisted block {}.", block.number);
                    }
                    let ms_per_block = (start.elapsed().as_millis() as f64) / (blocks.len() as f64);
                    metrics::block_indexing_time_ms().observe(ms_per_block);
                } else {
                    for block_number in block_numbers {
                        let start = std::time::Instant::now();
                        let block = sidecar.get_block_by_number(*block_number).await?;
                        let block_number = block.number;
                        storage.save_block(block).await?;
                        metrics::block_indexing_time_ms()
                            .observe(start.elapsed().as_millis() as f64);
                        metrics::indexed_finalized_block_number().set(block_number as i64);
                        log::info!("Persisted block {block_number}.");
                    }
                }
                block_number = range_end_block_number + 1;
            }
            if CONFIG.indexer.end_block_number.is_some() {
                return Ok(());
            }
            let delay_seconds = CONFIG.common.recovery_retry_seconds;
            log::info!("Reached chain head. Check new head in {delay_seconds} seconds.");
            tokio::time::sleep(std::time::Duration::from_secs(delay_seconds)).await;
        }
    }
}
