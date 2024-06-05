use async_trait::async_trait;
use ftd_config::Config;
use ftd_persistence::relational::RelationalStorage;
use ftd_service::Service;
use lazy_static::lazy_static;

mod metrics;

lazy_static! {
    static ref CONFIG: Config = Config::default();
}

#[derive(Default)]
pub struct TransferVolumeUpdater;

#[async_trait(? Send)]
impl Service for TransferVolumeUpdater {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (
            CONFIG.metrics.host.as_str(),
            CONFIG.metrics.transfer_volume_updater_port,
        )
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        log::info!("Transfer volume updater started.");
        let storage = RelationalStorage::new().await?;
        let sleep_seconds = CONFIG.common.recovery_retry_seconds;
        loop {
            let last_processed_id = storage
                .get_transfer_volume_updater_last_processed_transfer_id()
                .await?;
            let start_id = last_processed_id + 1;
            let max_id = storage.get_max_transfer_id().await?;
            if max_id >= start_id {
                log::info!("Process transfers {start_id}-{max_id}.");
                for id in start_id..=max_id {
                    if id == 0 {
                        continue;
                    }
                    log::info!("Process transfer {id}.");
                    if let Some(transfer) = storage.get_transfer_by_id(id).await? {
                        storage.update_transfer_volume(&transfer).await?;
                        storage
                            .set_transfer_volume_updater_last_processed_transfer_id(id)
                            .await?;
                        metrics::processed_transfer_id().set(id as i64);
                    } else {
                        log::warn!("Transfer id {id} not found.");
                    }
                }
            }
            log::info!("Completed processing. Sleep for {sleep_seconds} seconds.");
            tokio::time::sleep(std::time::Duration::from_secs(sleep_seconds)).await;
        }
    }
}
