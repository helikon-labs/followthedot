use ftd_metrics::registry::IntGauge;
use once_cell::sync::Lazy;

const METRIC_PREFIX: &str = "ftd_subscan_account_fetcher";

pub fn fetched_account_count() -> IntGauge {
    static METER: Lazy<IntGauge> = Lazy::new(|| {
        ftd_metrics::registry::register_int_gauge(
            METRIC_PREFIX,
            "fetched_account_count",
            "Number of fetched accounts",
        )
        .unwrap()
    });
    METER.clone()
}
