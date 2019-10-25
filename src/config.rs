use serde::Deserialize;
use config::{ConfigError, Config};
use diesel::PgConnection;
use std::path::Path;
use lazy_static::lazy_static;
use log::error;
use std::sync::Mutex;

lazy_static! {
    pub static ref BOT_CONFIG: Settings = Settings::new().unwrap();
}

pub const DEFAULT_PREFIX: &str = ".";

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub token: String
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::new();

        if !Path::new("config.toml").exists() {
            error!("No server config file");
        }

        settings.merge(config::File::with_name("config"))?;
        settings.try_into()
    }
}