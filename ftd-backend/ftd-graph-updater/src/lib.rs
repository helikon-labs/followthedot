use async_trait::async_trait;
use ftd_config::Config;
use ftd_persistence::graph::GraphStorage;
use ftd_persistence::relational::RelationalStorage;
use ftd_service::Service;
use lazy_static::lazy_static;

mod metrics;

lazy_static! {
    static ref CONFIG: Config = Config::default();
}

#[derive(Default)]
pub struct GraphUpdater;

impl GraphUpdater {
    async fn process_transfers(
        &self,
        relational_storage: &RelationalStorage,
        graph_storage: &GraphStorage,
    ) -> anyhow::Result<()> {
        let state = graph_storage.get_state().await?;
        let first_transfer_id = state.last_processed_transfer_id + 1;
        let max_transfer_id = relational_storage.get_max_transfer_id().await?;
        if first_transfer_id <= max_transfer_id {
            log::info!("Process transfers {first_transfer_id}-{max_transfer_id}.");
            for id in first_transfer_id..=max_transfer_id {
                let mut tx = graph_storage.begin_tx().await?;
                if let Some(transfer) = relational_storage.get_transfer_by_id(id).await? {
                    log::info!("Process transfer {id}.");
                    graph_storage.save_transfer(&mut tx, &transfer).await?;
                } else {
                    log::warn!("Transfer id {id} not found.");
                }
                graph_storage
                    .update_last_processed_transfer_id(&mut tx, id)
                    .await?;
                graph_storage.commit_tx(tx).await?;
                metrics::processed_transfer_id().set(id as i64);
            }
        }
        log::info!("Max transfer id {max_transfer_id} is processed.");
        Ok(())
    }
}

#[async_trait(? Send)]
impl Service for GraphUpdater {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (
            CONFIG.metrics.host.as_str(),
            CONFIG.metrics.graph_updater_port,
        )
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        log::info!("Graph updater started.");
        let relational_storage = RelationalStorage::new().await?;
        let graph_storage = GraphStorage::new().await?;
        let sleep_seconds = CONFIG.common.recovery_retry_seconds;
        loop {
            self.process_transfers(&relational_storage, &graph_storage)
                .await?;
            log::info!("Completed processing. Sleep for {sleep_seconds} seconds.");
            tokio::time::sleep(std::time::Duration::from_secs(sleep_seconds)).await;
        }
    }
}
