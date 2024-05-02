use async_trait::async_trait;
use ftd_config::Config;
use ftd_service::Service;
use ftd_substrate_client::SubstrateClient;
use lazy_static::lazy_static;

mod metrics;

lazy_static! {
    static ref CONFIG: Config = Config::default();
}

#[derive(Default)]
pub struct IdentityUpdater;

#[async_trait(? Send)]
impl Service for IdentityUpdater {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (
            CONFIG.metrics.host.as_str(),
            CONFIG.metrics.identity_updater_port,
        )
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        log::info!("Identity updater started.");
        let substrate_client = SubstrateClient::new(&CONFIG).await?;
        let sleep_seconds = CONFIG.common.recovery_retry_seconds;
        loop {
            let hash = substrate_client.get_finalized_block_hash().await?;
            let head = substrate_client.get_block_header(hash.as_str()).await?;
            log::info!("Get identity @ finalized block {}.", head.get_number()?);
            let identities = substrate_client.get_identities(hash.as_str()).await?;
            log::info!("Got {} identities.", identities.len());
            let sub_identities = substrate_client.get_sub_identities(hash.as_str()).await?;
            log::info!("Got {} sub identities.", sub_identities.len());
            log::info!("Completed processing. Sleep for {sleep_seconds} seconds.");
            tokio::time::sleep(std::time::Duration::from_secs(sleep_seconds)).await;
        }
    }
}
