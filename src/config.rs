use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    pub server: ServerConfig
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub name: String,
    pub graphics: GraphicsConfig
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            graphics: Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct GraphicsConfig {
    pub vfr: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Deserialize(#[from] toml::de::Error),
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
        Ok(toml::from_slice(&std::fs::read(path)?)?)
    }
}
