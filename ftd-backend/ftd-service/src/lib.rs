#![warn(clippy::disallowed_types)]
use async_trait::async_trait;
use ftd_config::Config;

pub mod err;

#[async_trait(?Send)]
pub trait Service {
    fn get_metrics_server_addr() -> (&'static str, u16);

    async fn run(&'static self) -> anyhow::Result<()>;

    async fn start(&'static self) {
        let config = Config::default();
        ftd_logging::init(&config);
        log::info!("Starting service...");
        tokio::spawn(ftd_metrics::server::start(Self::get_metrics_server_addr()));
        let delay_seconds = config.common.recovery_retry_seconds;
        loop {
            let result = self.run().await;
            if let Err(error) = result {
                log::error!("{:?}", error);
            }
            log::error!(
                "Process exited. Will try again in {} seconds.",
                delay_seconds,
            );
            tokio::time::sleep(std::time::Duration::from_secs(delay_seconds)).await;
        }
    }
}
