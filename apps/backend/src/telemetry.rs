use tracing_subscriber::{
    EnvFilter,
    filter::{FromEnvError, LevelFilter},
};

use crate::LogFormat;

/// Sends `tracing` events to stdout in `format`, filtered by `RUST_LOG`, or at
/// `info` when it is unset. Fails on an invalid `RUST_LOG`, so a typo doesn't
/// silently change what gets logged. Call it once, at startup.
pub fn init_tracing(format: LogFormat) -> Result<(), FromEnvError> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()?;
    let subscriber = tracing_subscriber::fmt().with_env_filter(filter);

    match format {
        LogFormat::Json => subscriber.json().init(),
        LogFormat::Pretty => subscriber.pretty().init(),
    }
    Ok(())
}
