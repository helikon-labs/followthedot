use async_trait::async_trait;
use ftd_config::Config;
use ftd_persistence::relational::RelationalStorage;
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
        let relational_storage = RelationalStorage::new().await?;
        let substrate_client = SubstrateClient::new(&CONFIG).await?;
        let sleep_seconds = CONFIG.common.recovery_retry_seconds;
        loop {
            let block_hash = substrate_client
                .get_finalized_block_hash()
                .await?
                .trim_start_matches("0x")
                .to_string();
            let block_head = substrate_client.get_block_header(&block_hash).await?;
            let block_number = block_head.get_number()?;
            log::info!(
                "Get identity @ finalized block {}.",
                block_head.get_number()?
            );
            let identities = substrate_client.get_identities(&block_hash).await?;
            log::info!("Got {} identities.", identities.len());
            relational_storage
                .save_identities(&block_hash, block_number, &identities)
                .await?;
            log::info!("Saved identities.");
            let sub_identities = substrate_client.get_sub_identities(&block_hash).await?;
            log::info!("Got {} sub identities.", sub_identities.len());
            relational_storage
                .save_sub_identities(&block_hash, block_number, &sub_identities)
                .await?;
            log::info!("Completed processing. Sleep for {sleep_seconds} seconds.");
            tokio::time::sleep(std::time::Duration::from_secs(sleep_seconds)).await;
        }
    }
}
