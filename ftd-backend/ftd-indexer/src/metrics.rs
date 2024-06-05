use ftd_metrics::registry::{Histogram, IntGauge};
use once_cell::sync::Lazy;

const METRIC_PREFIX: &str = "ftd_indexer";

pub fn indexed_finalized_block_number() -> IntGauge {
    static METER: Lazy<IntGauge> = Lazy::new(|| {
        ftd_metrics::registry::register_int_gauge(
            METRIC_PREFIX,
            "indexed_finalized_block_number",
            "Number of the last processed block",
        )
        .unwrap()
    });
    METER.clone()
}

pub fn block_indexing_time_ms() -> Histogram {
    static METER: Lazy<Histogram> = Lazy::new(|| {
        ftd_metrics::registry::register_histogram(
            METRIC_PREFIX,
            "block_indexing_time_ms",
            "Block indexing time in milliseconds",
            vec![
                10.0, 25.0, 50.0, 75.0, 100.0, 150.0, 200.0, 250.0, 500.0, 750.0, 1_000.0, 1_500.0,
                2_000.0, 3_000.0, 4_000.0, 5_000.0, 7_500.0, 10_000.0, 15_000.0, 20_000.0,
            ],
        )
        .unwrap()
    });
    METER.clone()
}
