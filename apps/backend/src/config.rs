use std::{fmt, net::SocketAddr};

use thiserror::Error;

/// Loopback only: the server has no auth or TLS yet. Containers set
/// `GRPC_ADDR=0.0.0.0:50051` and keep the port on their internal network.
const DEFAULT_GRPC_ADDR: &str = "127.0.0.1:50051";

/// The backend's settings, read from environment variables. `Debug` hides the
/// database URL, which holds the password.
#[derive(Clone)]
pub struct Config {
    /// `DATABASE_URL`: the PostgreSQL connection URL. Required.
    pub database_url: String,
    /// `GRPC_ADDR`: the address the gRPC server listens on.
    pub grpc_addr: SocketAddr,
    /// `LOG_FORMAT`: `json` or `pretty`.
    pub log_format: LogFormat,
}

/// How log lines are written to stdout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    /// One JSON object per line, for log collectors. The default.
    Json,
    /// Multi-line, human-readable output for local use.
    Pretty,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("database_url", &"[redacted]")
            .field("grpc_addr", &self.grpc_addr)
            .field("log_format", &self.log_format)
            .finish()
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("environment variable {0} is not set")]
    Missing(&'static str),

    #[error("environment variable {name} has the invalid value {value:?}")]
    Invalid { name: &'static str, value: String },
}

impl Config {
    /// Reads the settings from the environment.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|name| std::env::var(name).ok())
    }

    /// Reads the settings through `lookup`, so tests don't touch the process
    /// environment. An empty value counts as not set.
    fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let get = |name: &str| lookup(name).filter(|value| !value.is_empty());

        let database_url = get("DATABASE_URL").ok_or(ConfigError::Missing("DATABASE_URL"))?;

        let grpc_addr = get("GRPC_ADDR").unwrap_or_else(|| DEFAULT_GRPC_ADDR.to_owned());
        let grpc_addr = grpc_addr.parse().map_err(|_| ConfigError::Invalid {
            name: "GRPC_ADDR",
            value: grpc_addr.clone(),
        })?;

        let log_format = match get("LOG_FORMAT").as_deref() {
            None | Some("json") => LogFormat::Json,
            Some("pretty") => LogFormat::Pretty,
            Some(other) => {
                return Err(ConfigError::Invalid {
                    name: "LOG_FORMAT",
                    value: other.to_owned(),
                });
            }
        };

        Ok(Self {
            database_url,
            grpc_addr,
            log_format,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn config_from(vars: &[(&str, &str)]) -> Result<Config, ConfigError> {
        let vars: HashMap<String, String> = vars
            .iter()
            .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
            .collect();
        Config::from_lookup(|name| vars.get(name).cloned())
    }

    #[test]
    fn from_lookup_applies_the_defaults() {
        let config = config_from(&[("DATABASE_URL", "postgres://localhost/evie")]).unwrap();

        assert_eq!(config.database_url, "postgres://localhost/evie");
        assert_eq!(config.grpc_addr, DEFAULT_GRPC_ADDR.parse().unwrap());
        assert_eq!(config.log_format, LogFormat::Json);
    }

    #[test]
    fn from_lookup_reads_every_setting() {
        let config = config_from(&[
            ("DATABASE_URL", "postgres://localhost/evie"),
            ("GRPC_ADDR", "127.0.0.1:6000"),
            ("LOG_FORMAT", "pretty"),
        ])
        .unwrap();

        assert_eq!(config.grpc_addr, "127.0.0.1:6000".parse().unwrap());
        assert_eq!(config.log_format, LogFormat::Pretty);
    }

    #[test]
    fn debug_hides_the_database_url() {
        let config =
            config_from(&[("DATABASE_URL", "postgres://evie:secret@localhost/evie")]).unwrap();

        let debug = format!("{config:?}");

        assert!(!debug.contains("secret"));
        assert!(debug.contains("[redacted]"));
    }

    #[test]
    fn from_lookup_requires_a_database_url() {
        let err = config_from(&[("DATABASE_URL", "")]).unwrap_err();

        assert!(matches!(err, ConfigError::Missing("DATABASE_URL")));
    }

    #[test]
    fn from_lookup_rejects_an_invalid_grpc_addr() {
        let err = config_from(&[
            ("DATABASE_URL", "postgres://localhost/evie"),
            ("GRPC_ADDR", "localhost"),
        ])
        .unwrap_err();

        assert!(matches!(
            err,
            ConfigError::Invalid {
                name: "GRPC_ADDR",
                ..
            }
        ));
    }

    #[test]
    fn from_lookup_rejects_an_unknown_log_format() {
        let err = config_from(&[
            ("DATABASE_URL", "postgres://localhost/evie"),
            ("LOG_FORMAT", "xml"),
        ])
        .unwrap_err();

        assert!(matches!(
            err,
            ConfigError::Invalid {
                name: "LOG_FORMAT",
                ..
            }
        ));
    }
}
