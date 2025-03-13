use std::env;
use serde::Deserialize;
use config::{Config, ConfigError, File};
use once_cell::sync::Lazy;
pub static SETTINGS: Lazy<Settings> =
    Lazy::new(|| Settings::new().expect("Failed to setup settings"));

#[derive(Deserialize)]
pub struct Server {
    pub port: u16
}

#[derive(Deserialize)]
pub struct Logger {
    pub level: String
}

#[derive(Deserialize)]
pub struct Settings {
    pub environment: String,
    pub server: Server,
    pub logger: Logger,
}
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode= env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            .add_source(File::with_name("config/default"));

        if let Ok(port) = env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }

        builder.build()?.try_deserialize()
    }
}