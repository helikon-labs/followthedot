use ftd_metrics::registry::IntGauge;
use once_cell::sync::Lazy;

const METRIC_PREFIX: &str = "ftd_transfer_volume_updater";

pub fn processed_transfer_id() -> IntGauge {
    static METER: Lazy<IntGauge> = Lazy::new(|| {
        ftd_metrics::registry::register_int_gauge(
            METRIC_PREFIX,
            "processed_transfer_id",
            "Id of the last processed transfer",
        )
        .unwrap()
    });
    METER.clone()
}
