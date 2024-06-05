use ftd_metrics::registry::IntGauge;
use once_cell::sync::Lazy;

const METRIC_PREFIX: &str = "ftd_identity_updater";

pub fn last_run_timestamp_ms() -> IntGauge {
    static METER: Lazy<IntGauge> = Lazy::new(|| {
        ftd_metrics::registry::register_int_gauge(
            METRIC_PREFIX,
            "last_run_timestamp_ms",
            "Timestamp (ms) for the last run",
        )
        .unwrap()
    });
    METER.clone()
}

pub fn last_identity_list_fetch_timestamp_ms() -> IntGauge {
    static METER: Lazy<IntGauge> = Lazy::new(|| {
        ftd_metrics::registry::register_int_gauge(
            METRIC_PREFIX,
            "last_identity_list_fetch_timestamp_ms",
            "Timestamp (ms) for the last candidate list fetch operation",
        )
        .unwrap()
    });
    METER.clone()
}

pub fn last_identity_list_persist_timestamp_ms() -> IntGauge {
    static METER: Lazy<IntGauge> = Lazy::new(|| {
        ftd_metrics::registry::register_int_gauge(
            METRIC_PREFIX,
            "last_identity_list_persist_timestamp_ms",
            "Timestamp (ms) for the last candidate list fetch operation",
        )
        .unwrap()
    });
    METER.clone()
}

pub fn last_sub_identity_list_fetch_timestamp_ms() -> IntGauge {
    static METER: Lazy<IntGauge> = Lazy::new(|| {
        ftd_metrics::registry::register_int_gauge(
            METRIC_PREFIX,
            "last_sub_identity_list_fetch_timestamp_ms",
            "Timestamp (ms) for the last candidate list fetch operation",
        )
        .unwrap()
    });
    METER.clone()
}

pub fn last_sub_identity_list_persist_timestamp_ms() -> IntGauge {
    static METER: Lazy<IntGauge> = Lazy::new(|| {
        ftd_metrics::registry::register_int_gauge(
            METRIC_PREFIX,
            "last_sub_identity_list_persist_timestamp_ms",
            "Timestamp (ms) for the last candidate list fetch operation",
        )
        .unwrap()
    });
    METER.clone()
}

pub fn last_success_status() -> IntGauge {
    static METER: Lazy<IntGauge> = Lazy::new(|| {
        ftd_metrics::registry::register_int_gauge(
            METRIC_PREFIX,
            "last_success_status",
            "Boolean value for the success status of the last process",
        )
        .unwrap()
    });
    METER.clone()
}
