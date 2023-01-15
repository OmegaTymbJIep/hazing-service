use config::{ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub listener: Listener,
    pub logger: Logger,
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Listener {
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub address: String,
}

impl Config {
    pub fn new(config_path: String) -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(File::with_name(config_path.as_str()).required(true))
            .build()?
            .try_deserialize()
    }
}
