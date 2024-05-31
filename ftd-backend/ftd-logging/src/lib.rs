#![warn(clippy::disallowed_types)]
use env_logger::{Builder, Env, Target, WriteStyle};
use ftd_config::Config;
use std::str::FromStr;

pub fn init(config: &Config) {
    let other_modules_log_level = log::LevelFilter::from_str(config.log.other_level.as_str())
        .expect("Cannot read log level configuration for outside modules.");
    let log_level = log::LevelFilter::from_str(config.log.ftd_level.as_str())
        .expect("Cannot read log level configuration for FTD modules.");
    let mut builder = Builder::from_env(Env::default());
    builder.target(Target::Stdout);
    builder.filter(None, other_modules_log_level);
    builder.filter(Some("ftd_api_service"), log_level);
    builder.filter(Some("ftd_graph_updater"), log_level);
    builder.filter(Some("ftd_identity_updater"), log_level);
    builder.filter(Some("ftd_indexer"), log_level);
    builder.filter(Some("ftd_metrics"), log_level);
    builder.filter(Some("ftd_metrics_server"), log_level);
    builder.filter(Some("ftd_persistence"), log_level);
    builder.filter(Some("ftd_sidecar_client"), log_level);
    builder.filter(Some("ftd_subscan_account_fetcher"), log_level);
    builder.filter(Some("ftd_subscan_client"), log_level);
    builder.filter(Some("ftd_substrate_client"), log_level);
    builder.filter(Some("ftd_transfer_volume_updater"), log_level);
    builder.filter(Some("ftd_types"), log_level);
    builder.write_style(WriteStyle::Always);
    builder.init();
}
