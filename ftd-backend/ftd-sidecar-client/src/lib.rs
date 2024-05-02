use ftd_config::Config;
use std::time::Duration;

mod block;

/// The client.
pub struct SidecarClient {
    base_url: String,
    http_client: reqwest::Client,
}

impl SidecarClient {
    /// Connect to the node and construct a new Substrate client.
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        log::info!("Constructing sidecar client.");
        let http_client: reqwest::Client = reqwest::Client::builder()
            .gzip(true)
            .brotli(true)
            .timeout(Duration::from_secs(
                config.substrate.connection_timeout_seconds,
            ))
            .build()
            .unwrap();
        log::info!("Sidecar client constructed.");
        Ok(Self {
            base_url: config.substrate.sidecar_url.clone(),
            http_client,
        })
    }
}
