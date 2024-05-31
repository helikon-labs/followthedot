use crate::metrics::fetched_account_count;
use async_trait::async_trait;
use ftd_config::Config;
use ftd_persistence::relational::RelationalStorage;
use ftd_service::Service;
use ftd_subscan_client::SubscanClient;
use lazy_static::lazy_static;

mod metrics;

lazy_static! {
    static ref CONFIG: Config = Config::default();
}

#[derive(Default)]
pub struct SubscanAccountFetcher;

#[async_trait(? Send)]
impl Service for SubscanAccountFetcher {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (
            CONFIG.metrics.host.as_str(),
            CONFIG.metrics.transfer_volume_updater_port,
        )
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        log::info!("Subscan account fetcher started.");
        let subscan_client = SubscanClient::new(&CONFIG)?;
        let storage = RelationalStorage::new().await?;
        let sleep_seconds = CONFIG.common.recovery_retry_seconds;
        loop {
            fetched_account_count().reset();
            let mut page_index = 0;
            loop {
                log::info!("Get page {}.", page_index + 1);
                let page = subscan_client.get_account_list(page_index).await?;
                if let Some(account_list) = page.data.as_ref() {
                    log::info!(
                        "There are {} records on page {}.",
                        account_list.count,
                        page_index + 1
                    );
                    for account in account_list.list.iter() {
                        storage.save_subscan_account(account).await?;
                    }
                    log::info!("Persisted {} records.", account_list.list.len());
                } else {
                    log::info!("No records found on page {}.", page_index + 1);
                    break;
                }
                page_index += 1;
            }
            // get page of 100 & save each account, go to next page
            log::info!("Completed processing. Sleep for {sleep_seconds} seconds.");
            fetched_account_count().inc();
            tokio::time::sleep(std::time::Duration::from_secs(sleep_seconds)).await;
        }
    }
}
