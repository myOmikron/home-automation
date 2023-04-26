//! This module holds all config related structs

use std::fs::read_to_string;
use std::net::IpAddr;
use std::path::Path;

use actix_toolbox::logging::LoggingConfig;
use serde::{Deserialize, Serialize};

/// The server config of the brain
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerConfig {
    /// The listen address of the server
    pub listen_address: IpAddr,
    /// The listen port of the server
    pub listen_port: u16,
}

/// The database config of the brain
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DatabaseConfig {
    /// The host of the database
    pub host: String,
    /// The port of the database
    pub port: u16,
    /// The name of the database
    pub name: String,
    /// The user to connect to the database
    pub user: String,
    /// The password for the user
    pub password: String,
}

/// The config of the brain
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    /// The config for logging
    pub logging: LoggingConfig,
    /// The database config
    pub database: DatabaseConfig,
}

impl TryFrom<&str> for Config {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let p = Path::new(value);
        if !p.exists() {
            println!("Config file {value} does not exist");
        }

        if !p.is_file() {
            println!("{value} is no file");
        }

        let c = read_to_string(p).map_err(|e| format!("Could not read {value}: {e}"))?;
        let config = toml::from_str(&c).map_err(|e| format!("Could not parse {value}: {e}"))?;

        Ok(config)
    }
}
