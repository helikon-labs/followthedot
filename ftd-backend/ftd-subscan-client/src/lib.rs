use ftd_config::Config;
use ftd_types::subscan::SubscanAccountSearchResult;
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
}

// get top accounts
