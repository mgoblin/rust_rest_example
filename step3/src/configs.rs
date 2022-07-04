use config::{Config, ConfigError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub server: ServerConfig,
    pub store: Option<Store>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Store {
    pub inmemory: Option<InMemory>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InMemory {
    pub users: u16
}

impl Configuration {
    pub fn load() -> Result<Configuration, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name("application.yaml"))
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .build()?;

        config.try_deserialize::<Configuration>()
    }
}

