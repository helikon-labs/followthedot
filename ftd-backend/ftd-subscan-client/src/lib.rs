use ftd_config::Config;
use ftd_types::subscan::{
    SubscanAccountListBody, SubscanAccountListResult, SubscanAccountSearchResult,
};
use reqwest::Client;
use rustc_hash::FxHashMap as HashMap;

fn get_http_client(config: &Config) -> anyhow::Result<Client> {
    Ok(Client::builder()
        .gzip(true)
        .brotli(true)
        .timeout(std::time::Duration::from_secs(
            config.api.request_timeout_seconds,
        ))
        .build()?)
}

pub struct SubscanClient {
    config: Config,
    http_client: Client,
}

impl SubscanClient {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            config: config.clone(),
            http_client: get_http_client(config)?,
        })
    }

    pub async fn get_account(
        &self,
        account_address: &str,
    ) -> anyhow::Result<SubscanAccountSearchResult> {
        let mut map = HashMap::default();
        map.insert("key", account_address);
        Ok(self
            .http_client
            .post(self.config.subscan.account_data_url.as_str())
            .header("x-api-key", self.config.subscan.api_key.as_str())
            .json(&map)
            .send()
            .await?
            .json::<SubscanAccountSearchResult>()
            .await?)
    }

    pub async fn get_account_list(
        &self,
        page_index: u32,
        page_size: u8,
    ) -> anyhow::Result<SubscanAccountListResult> {
        let body = SubscanAccountListBody {
            order: "desc".to_string(),
            order_field: "balance".to_string(),
            page: page_index,
            row: page_size,
        };
        Ok(self
            .http_client
            .post(self.config.subscan.account_list_url.as_str())
            .header("x-api-key", self.config.subscan.api_key.as_str())
            .json(&body)
            .send()
            .await?
            .json::<SubscanAccountListResult>()
            .await?)
    }
}

// get top accounts
