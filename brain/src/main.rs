//! # brain
//!
//! The brain of the home automation
#![warn(missing_docs)]

use actix_toolbox::logging::setup_logging;
use clap::{Parser, Subcommand};
use rorm::{cli, DatabaseDriver};

use crate::config::Config;

pub mod config;

/// The subcommands
#[derive(Subcommand)]
pub enum Command {
    /// Start the server
    Start,
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

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Command::Start => {
            let conf = Config::try_from(cli.config_path.as_str())?;
            setup_logging(&conf.logging)?;
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
