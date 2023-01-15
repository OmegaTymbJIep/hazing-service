extern crate core;

mod cli;
mod config;
mod logger;
mod service;

use crate::cli::{Cli, Commands, MigrateMode};
use crate::config::Config;
use crate::service::data::migrate;

#[tokio::main]
async fn main() {
    let config = Config::new(Cli::config()).expect("failed to parse config");
    logger::init(&config.logger.level);

    match Cli::command() {
        Commands::Run => {
            service::run_service(&config).await;
        }
        Commands::Migrate { migrate_mod } => migrate(&config, migrate_mod).await,
    }
}
