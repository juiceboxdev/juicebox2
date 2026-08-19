use serde::{Deserialize, Serialize};
use std::fs;
use tracing::warn;

pub mod sentry;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    sentry: sentry::SentryConfig,
}

impl Config {
    const CONFIG_PATH: &str = ".juicehost.toml";
    const BACKUP_CONFIG_PATH: &str = "juicehost.toml";

    #[must_use]
    pub const fn sentry(&self) -> &sentry::SentryConfig {
        &self.sentry
    }

    /// Attempts to construct a [`Config`].
    ///
    /// # Errors
    ///
    /// Returns an error if a config file exists, but it is malformed. It is
    /// expected behavior to not fall-back to a default config, as the user has
    /// expressed the intent to define their own; this is contrary to defining
    /// no config at all, in which the default values are used.
    #[tracing::instrument]
    pub fn try_load() -> anyhow::Result<Self> {
        fs::read_to_string(Self::CONFIG_PATH)
            .or_else(|_| fs::read_to_string(Self::BACKUP_CONFIG_PATH))
            .inspect_err(|err| warn!("Failed to load config file: {err}"))
            .map_or_else(|_| Ok(Self::default()), |s| toml::from_str::<Self>(&s))
            .map_err(From::from)
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Self {
            sentry: sentry::SentryConfig::default(),
        }
    }
}
