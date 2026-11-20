use tracing_subscriber::EnvFilter;

use crate::LogFormat;

/// The log filter when `RUST_LOG` is unset or invalid.
const DEFAULT_FILTER: &str = "info";

/// Sends `tracing` events to stdout in `format`, filtered by `RUST_LOG`.
/// Call it once, at startup.
pub fn init_tracing(format: LogFormat) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_FILTER));
    let subscriber = tracing_subscriber::fmt().with_env_filter(filter);

    match format {
        LogFormat::Json => subscriber.json().init(),
        LogFormat::Pretty => subscriber.pretty().init(),
    }
}
