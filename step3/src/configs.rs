use config::{Config, ConfigError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Configuration {
    pub server: ServerConfig,
    pub store: Option<Store>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Store {
    pub inmemory: Option<InMemory>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InMemory {
    pub users: u16
}

impl Configuration {
    pub fn load_from_file(file_name: &str) -> Result<Configuration, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name(file_name))
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .build()?;

        config.try_deserialize::<Configuration>()
    }
}

#[cfg(test)]
mod tests {
    use crate::configs::{ServerConfig, Store, InMemory};
    use super::Configuration;

    #[test]
    fn load_full_config_from_existing_file() {
        let cfg = Configuration::load_from_file("tests/application.yaml").unwrap();
        assert_eq!(
            Configuration {
                server: ServerConfig {
                    host: "0.0.0.0".to_string(), 
                    port: 9090
                },
                store: Some(Store {
                    inmemory: Some(InMemory{
                        users: 10,
                    })
                }),
            },  
            cfg
        ); 
    }

    #[test]
    fn load_from_unexisted_file() {
        let result = Configuration::load_from_file("not_existed.yaml").unwrap_err();
        assert_eq!("configuration file \"not_existed.yaml\" not found", result.to_string());
    }

    #[test]
    fn load_not_yaml() {
        let result = Configuration::load_from_file("tests/corrupted.yaml").unwrap_err();
        assert_eq!("invalid type: unit value, expected struct ServerConfig", result.to_string());
    }

    #[test]
    fn load_empty_config_file() {
        let cfg = Configuration::load_from_file("tests/empty.yaml").unwrap();
        assert_eq!(
            Configuration {
                server: ServerConfig {
                    host: "0.0.0.0".to_string(), 
                    port: 8080
                },
                store: None
            },  
            cfg
        );
    }

    #[test]
    fn load_server_with_empty_body() {
        let result = Configuration::load_from_file("tests/server_empty.yaml").unwrap_err();
        assert_eq!("invalid type: unit value, expected struct ServerConfig", result.to_string());
    }

    #[test]
    fn load_default_port() {
        let cfg = Configuration::load_from_file("tests/default_port.yaml").unwrap();
        assert_eq!(
            Configuration {
                server: ServerConfig {
                    host: "123".to_string(), 
                    port: 8080
                },
                store: None
            },  
            cfg
        );
    }

    #[test]
    fn load_default_host() {
        let cfg = Configuration::load_from_file("tests/default_host.yaml").unwrap();
        assert_eq!(
            Configuration {
                server: ServerConfig {
                    host: "0.0.0.0".to_string(), 
                    port: 9999
                },
                store: None
            },  
            cfg
        ); 
    }

    #[test]
    fn load_server_full() {
        let cfg = Configuration::load_from_file("tests/server_full.yaml").unwrap();
        assert_eq!(
            Configuration {
                server: ServerConfig {
                    host: "345".to_string(), 
                    port: 1234
                },
                store: None
            },  
            cfg
        );
    }

    #[test]
    fn invalid_port_format() {
        let result = Configuration::load_from_file("tests/server_empty.yaml").unwrap_err();
        assert_eq!("invalid type: unit value, expected struct ServerConfig", result.to_string());
    }
}

