use config::{Config, ConfigError, Environment, File};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{env, fmt};
use std::fmt::Formatter;

pub static SETTINGS: Lazy<Settings> =
    Lazy::new(|| Settings::new().expect("Failed to setup settings"));

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u16
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
    pub level: String
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub url: String,
    pub name: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct Auth {
    pub secret: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub environment: String,
    pub server: Server,
    pub logger: Logger,
    pub database: Database,
    pub auth: Auth
}
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode= env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{run_mode}")).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::default().separator("__"));

        if let Ok(port) = env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }

        builder.build()?.try_deserialize()
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "http://localhost:{}", self.port)
    }
}