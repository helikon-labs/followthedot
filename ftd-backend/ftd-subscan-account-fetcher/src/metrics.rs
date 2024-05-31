use ftd_metrics::registry::IntCounter;
use once_cell::sync::Lazy;

const METRIC_PREFIX: &str = "ftd_subscan_account_fetcher";

pub fn fetched_account_count() -> IntCounter {
    static METER: Lazy<IntCounter> = Lazy::new(|| {
        ftd_metrics::registry::register_int_counter(
            METRIC_PREFIX,
            "fetched_account_count",
            "Number of fetched accounts",
        )
        .unwrap()
    });
    METER.clone()
}
