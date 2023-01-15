pub mod data;

use crate::config::Config;

pub async fn run_service(config: &Config) {
    log::info!("Service starting")
}