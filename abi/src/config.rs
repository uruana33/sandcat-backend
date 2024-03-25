// db config
// server config

use crate::errors::Error;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    // db config
    pub db: DbConfig,
    // server config
    pub server: ServerConfig,
    pub kafka: KafkaConfig,
    pub redis: RedisConfig,
    pub rpc: RpcServerConfig,
    pub websocket: WsServerConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DbConfig {
    // db config
    pub postgres: PostgresConfig,
    pub mongodb: MongoDbConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcServerConfig {
    pub ws: WsServerConfig,
    pub chat: ChatRpcServerConfig,
    pub db: DbRpcServerConfig,
    pub pusher: PusherRpcServerConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PusherRpcServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WsServerConfig {
    pub host: String,
    pub port: u16,
}

impl WsServerConfig {
    #[inline]
    pub fn rpc_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    #[inline]
    pub fn url(&self, https: bool) -> String {
        url(https, &self.host, self.port)
    }

    pub fn ws_url(&self, https: bool) -> String {
        if https {
            format!("wss://{}:{}", self.host, self.port)
        } else {
            format!("ws://{}:{}", self.host, self.port)
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DbRpcServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatRpcServerConfig {
    pub host: String,
    pub port: u16,
}

impl PusherRpcServerConfig {
    #[inline]
    pub fn rpc_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    #[inline]
    pub fn url(&self, https: bool) -> String {
        url(https, &self.host, self.port)
    }
}

impl DbRpcServerConfig {
    #[inline]
    pub fn rpc_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    #[inline]
    pub fn url(&self, https: bool) -> String {
        url(https, &self.host, self.port)
    }
}

impl ChatRpcServerConfig {
    #[inline]
    pub fn rpc_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    #[inline]
    pub fn url(&self, https: bool) -> String {
        url(https, &self.host, self.port)
    }
}

fn url(https: bool, host: &str, port: u16) -> String {
    if https {
        format!("https://{}:{}", host, port)
    } else {
        format!("http://{}:{}", host, port)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
}

impl RedisConfig {
    pub fn url(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KafkaConfig {
    pub hosts: Vec<String>,
    pub topic: String,
    pub group: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    #[serde(default = "default_conn")]
    pub max_connections: u32,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MongoDbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

fn default_conn() -> u32 {
    5
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn load(filename: impl AsRef<Path>) -> Result<Self, Error> {
        let content = fs::read_to_string(filename).map_err(|_| Error::ConfigReadError)?;
        serde_yaml::from_str(&content).map_err(|_| Error::ConfigParseError)
    }
}

impl PostgresConfig {
    pub fn server_url(&self) -> String {
        if self.password.is_empty() {
            return format!("postgres://{}@{}:{}", self.user, self.host, self.port);
        }
        format!(
            "postgres://{}:{}@{}:{}",
            self.user, self.password, self.host, self.port
        )
    }
    pub fn url(&self) -> String {
        format!("{}/{}", self.server_url(), self.database)
    }
}
impl MongoDbConfig {
    pub fn server_url(&self) -> String {
        match (self.user.is_empty(), self.password.is_empty()) {
            (true, _) => {
                format!("mongodb://{}:{}", self.host, self.port)
            }
            (false, true) => {
                format!("mongodb://{}@{}:{}", self.user, self.host, self.port)
            }
            (false, false) => {
                format!(
                    "mongodb://{}:{}@{}:{}",
                    self.user, self.password, self.host, self.port
                )
            }
        }
    }
    pub fn url(&self) -> String {
        format!("{}/{}", self.server_url(), self.database)
    }
}

impl ServerConfig {
    pub fn url(&self, https: bool) -> String {
        url(https, &self.host, self.port)
    }

    pub fn with_port(&self, port: u16) -> ServerConfig {
        ServerConfig {
            host: self.host.clone(),
            port,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let config = match Config::load("./fixtures/im.yml") {
            Ok(config) => config,
            Err(err) => {
                panic!("load config error: {:?}", err);
            }
        };
        println!("{:?}", config);
        assert_eq!(config.db.postgres.host, "localhost");
        assert_eq!(config.db.postgres.port, 5432);
        assert_eq!(config.db.postgres.user, "postgres");
        assert_eq!(config.db.postgres.password, "root");
    }
}
