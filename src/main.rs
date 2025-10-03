use anyhow::{Context, Result};
use log::info;

use polipo::PolipoApp;
use polipo::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting Polipo - Octopus Energy Tariff Monitor");

    let config = Config::from_env().context("Failed to load configuration")?;
    let mut app = PolipoApp::new(config);

    app.run_with_schedule().await
}
