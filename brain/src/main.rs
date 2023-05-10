//! # brain
//!
//! The brain of the home automation
#![warn(missing_docs)]
#![cfg_attr(feature = "rorm-main", allow(dead_code, unused_imports))]

use actix_toolbox::logging::setup_logging;
use actix_web::cookie::Key;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use clap::{Parser, Subcommand};
use log::{error, info};
use rorm::{cli, Database, DatabaseConfiguration, DatabaseDriver};

use crate::chan::start_mqtt_client;
use crate::config::Config;
use crate::server::start_server;

pub mod chan;
pub mod config;
pub mod handler;
pub(crate) mod middleware;
pub mod models;
mod server;
pub mod swagger;

/// The subcommands
#[derive(Subcommand)]
pub enum Command {
    /// Start the server
    Start,
    /// Generate a SecretKey
    Keygen,
    /// Apply migrations to the database
    Migrate {
        /// The directory where the migrations are
        migration_dir: String,
    },
}

/// The brains of the home-automation
#[derive(Parser)]
pub struct Cli {
    /// The configuration path for the brain
    #[clap(long, default_value_t = String::from("/etc/brain/config.toml"))]
    config_path: String,

    #[clap(subcommand)]
    command: Command,
}

#[rorm::rorm_main]
#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Command::Start => {
            let conf = Config::try_from(cli.config_path.as_str())?;
            setup_logging(&conf.logging)?;

            let db = match get_db(&conf).await {
                Ok(db) => db,
                Err(err) => {
                    error!("Error initializing db: {err}");
                    return Err(format!("Error initializing db: {err}"));
                }
            };

            info!("Initializing mqtt client");
            let mqtt_client = match start_mqtt_client(&conf).await {
                Ok(c) => c,
                Err(err) => {
                    error!("Error initializing connection to mqtt broker: {err}");
                    return Err(format!(
                        "Error initializing connection to mqtt broker: {err}"
                    ));
                }
            };

            if let Err(err) = start_server(&conf, db, mqtt_client).await {
                error!("Error starting server: {err}");
            }
        }
        Command::Keygen => {
            println!("{}", BASE64_STANDARD.encode(Key::generate().master()));
        }
        Command::Migrate { migration_dir } => {
            let conf = Config::try_from(cli.config_path.as_str())?;
            cli::migrate::run_migrate_custom(
                cli::config::DatabaseConfig {
                    last_migration_table_name: None,
                    driver: DatabaseDriver::Postgres {
                        host: conf.database.host,
                        port: conf.database.port,
                        name: conf.database.name,
                        user: conf.database.user,
                        password: conf.database.password,
                    },
                },
                migration_dir,
                false,
                None,
            )
            .await
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

async fn get_db(conf: &Config) -> Result<Database, rorm::Error> {
    Database::connect(DatabaseConfiguration::new(DatabaseDriver::Postgres {
        name: conf.database.name.clone(),
        host: conf.database.host.clone(),
        port: conf.database.port,
        user: conf.database.user.clone(),
        password: conf.database.password.clone(),
    }))
    .await
}
