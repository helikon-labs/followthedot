use async_trait::async_trait;
use ftd_config::Config;
use ftd_service::Service;
use lazy_static::lazy_static;
use ftd_substrate_client::SubstrateClient;

mod metrics;

lazy_static! {
    static ref CONFIG: Config = Config::default();
}

#[derive(Default)]
pub struct BalanceUpdater;

#[async_trait(? Send)]
impl Service for BalanceUpdater {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (
            CONFIG.metrics.host.as_str(),
            CONFIG.metrics.transfer_volume_updater_port,
        )
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        log::info!("Balance updater started.");
        let substrate_client = SubstrateClient::new(&CONFIG).await?;
        let sleep_seconds = CONFIG.common.recovery_retry_seconds;
        loop {
            let block_hash = substrate_client
                .get_finalized_block_hash()
                .await?
                .trim_start_matches("0x")
                .to_string();
            substrate_client.get_balances(block_hash.as_str()).await?;
            log::info!("Completed processing. Sleep for {sleep_seconds} seconds.");
            tokio::time::sleep(std::time::Duration::from_secs(sleep_seconds)).await;
        }
    }
}