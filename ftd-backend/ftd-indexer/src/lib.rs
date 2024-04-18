use async_trait::async_trait;
use ftd_config::Config;
use ftd_persistence::postgres::PostgreSQLStorage;
use ftd_service::Service;
use ftd_sidecar_client::SidecarClient;
use ftd_types::substrate::Block;
use lazy_static::lazy_static;
use std::cmp::min;
use std::sync::atomic::AtomicBool;

mod metrics;

const REDENOMINATION_BLOCK_NUMBER: u64 = 1_205_128;

lazy_static! {
    static ref CONFIG: Config = Config::default();
    static ref IS_BUSY: AtomicBool = AtomicBool::new(false);
}

async fn save_block(
    storage: &PostgreSQLStorage,
    sidecar_client: &SidecarClient,
    block: &Block,
) -> anyhow::Result<()> {
    let mut transaction = storage.begin_tx().await?;
    if block.number < REDENOMINATION_BLOCK_NUMBER {
        storage
            .save_block(&block.convert_to_old_dot(), &mut transaction)
            .await?;
    } else {
        storage.save_block(block, &mut transaction).await?;
    }
    for address in block.update_identities_of.iter() {
        let identity = sidecar_client.get_identity_of(address, &block.hash).await?;
        let sub_identity = sidecar_client
            .get_sub_identity_of(address, &block.hash)
            .await?;
        storage
            .save_account(
                address,
                &identity,
                &sub_identity,
                block.number,
                &mut transaction,
            )
            .await?;
    }
    storage.commit_tx(transaction).await?;
    Ok(())
}

#[derive(Default)]
pub struct Indexer;

#[async_trait(?Send)]
impl Service for Indexer {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (CONFIG.metrics.host.as_str(), CONFIG.metrics.indexer_port)
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        log::info!("Indexer started.");
        let storage = PostgreSQLStorage::new(&CONFIG).await?;
        let sidecar_client = SidecarClient::new(&CONFIG)?;

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
                    let head = sidecar_client.get_head().await?;
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
                    let blocks = sidecar_client
                        .get_range_of_blocks(range_start_block_number, range_end_block_number)
                        .await?;
                    for block in blocks {
                        save_block(&storage, &sidecar_client, &block).await?;
                        log::info!("Persisted block {}.", block.number);
                    }
                } else {
                    for block_number in block_numbers {
                        let block = sidecar_client.get_block_by_number(*block_number).await?;
                        save_block(&storage, &sidecar_client, &block).await?;
                        log::info!("Persisted block {}.", block.number);
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
