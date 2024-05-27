use ftd_metrics::registry::IntGauge;
use once_cell::sync::Lazy;

const _METRIC_PREFIX: &str = "ftd_balance_updater";

pub fn _processed_account() -> IntGauge {
    static METER: Lazy<IntGauge> = Lazy::new(|| {
        ftd_metrics::registry::register_int_gauge(
            _METRIC_PREFIX,
            "processed_account",
            "Last processed account",
        )
        .unwrap()
    });
    METER.clone()
}
