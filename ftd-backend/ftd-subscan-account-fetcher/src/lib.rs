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

const PAGE_SIZE: u8 = 100;

#[derive(Default)]
pub struct SubscanAccountFetcher;

#[async_trait(? Send)]
impl Service for SubscanAccountFetcher {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (
            CONFIG.metrics.host.as_str(),
            CONFIG.metrics.subscan_account_fetcher_port,
        )
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        log::info!("Subscan account fetcher started.");
        let subscan_client = SubscanClient::new(&CONFIG)?;
        let storage = RelationalStorage::new().await?;
        let sleep_seconds = CONFIG.subscan.sleep_seconds;
        loop {
            fetched_account_count().set(0);
            let mut page_index = 0;
            loop {
                log::info!("Get page {}.", page_index + 1);
                let page = subscan_client
                    .get_account_list(page_index, PAGE_SIZE)
                    .await?;
                if let Some(account_list) = page.data.as_ref() {
                    log::info!(
                        "There are {} records on page {}.",
                        account_list.list.len(),
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
            log::info!("Completed processing. Sleep for {sleep_seconds} seconds.");
            fetched_account_count().add(PAGE_SIZE as i64);
            tokio::time::sleep(std::time::Duration::from_secs(sleep_seconds)).await;
        }
    }
}
