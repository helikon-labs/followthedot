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

    pub async fn do_stuff(&self) -> anyhow::Result<()> {
        let head = self
            .http_client
            .get("https://sidecar.helikon.io/polkadot/blocks/head")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let hash = head["hash"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .trim_start_matches("0x")
            .to_string();
        log::info!("Block hash: {}", hash);
        let extrinsics = head["extrinsics"].as_array().unwrap();
        let mut block_event_index: u16 = 0;
        log::info!("{} extrinsics.", extrinsics.len());
        for (extrinsic_index, extrinsic) in extrinsics.iter().enumerate() {
            let events = extrinsic["events"].as_array().unwrap();
            log::info!(
                "Extrinsic #{} has {} events.",
                extrinsic_index,
                events.len()
            );
            for (extrinsic_event_index, event) in events.iter().enumerate() {
                let pallet = event["method"]["pallet"].as_str().unwrap();
                let method = event["method"]["method"].as_str().unwrap();
                log::info!("#{block_event_index} #{extrinsic_index}.#{extrinsic_event_index} {pallet}.{method}");
                block_event_index += 1;
            }
        }
        Ok(())
    }
}
