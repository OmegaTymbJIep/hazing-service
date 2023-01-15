pub mod data;
pub mod ipfs;

use crate::config::Config;

pub async fn run_service(config: &Config) {
    log::info!("Service starting")
}
