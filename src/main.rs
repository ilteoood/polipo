use std::sync::Arc;

use anyhow::{Context, Result};
use log::info;
use tokio::sync::Mutex;

use chrono::Local;
use cron_tab::AsyncCron;
use polipo::PolipoApp;
use polipo::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting Polipo - Octopus Energy Tariff Monitor");

    let config = Config::from_env().context("Failed to load configuration")?;
    let cron_schedule = config.cron_schedule.clone();
    let app = Arc::new(Mutex::new(PolipoApp::new(config)));

    let local_tz = Local::now().offset().to_owned();
    let mut cron = AsyncCron::new(local_tz);
    cron.add_fn(&cron_schedule, move || {
        let app = app.clone();
        async move {
            match app.lock().await.run_check().await {
                Ok(_) => log::info!("Scheduled check completed successfully"),
                Err(e) => {
                    log::error!("Error during scheduled check: {:?}", e);
                    println!("Error during scheduled check: {:?}", e);
                }
            }
        }
    })
    .await?;

    cron.start_blocking().await;

    Ok(())
}
