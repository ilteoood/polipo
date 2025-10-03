pub mod cache;
pub mod config;
pub mod email;
pub mod octopus;
pub mod scheduler;

use anyhow::{Context, Result};
use chrono::Utc;
use log::{error, info};
use tokio::time::sleep;

use crate::cache::{OfferCache, UtilityType};
use crate::config::Config;
use crate::email::EmailService;
use crate::octopus::client::OctopusClient;
use crate::octopus::models::{Product, SupplyPoint, Viewer};
use crate::scheduler::parse_cron_interval;

/// Main application structure
pub struct PolipoApp {
    octopus_client: OctopusClient,
    email_service: EmailService,
    cache: OfferCache,
    config: Config,
}

impl PolipoApp {
    /// Create a new Polipo application instance
    pub fn new(config: Config) -> Self {
        let octopus_client = OctopusClient::new(config.clone());
        let email_service = EmailService::new(config.clone());
        let cache =
            OfferCache::load_from_file(&config.cache_file_path).expect("Failed to load cache");

        Self {
            octopus_client,
            email_service,
            cache,
            config,
        }
    }

    /// Compare tariffs and send email if better offer found
    async fn check_and_notify(&mut self, user_data: &Viewer) -> Result<()> {
        info!("Comparing tariffs and checking for better offers");

        let available_products = self.octopus_client.fetch_tariffs().await?;

        for account in &user_data.accounts {
            for property in &account.properties {
                // Check electricity supply points
                for electricity_sp in &property.electricity_supply_points {
                    if electricity_sp.status == "ON_SUPPLY" {
                        self.check_tariff(
                            &account.number,
                            &user_data.full_name,
                            electricity_sp,
                            &available_products,
                            UtilityType::Luce,
                        )
                        .await?;
                    }
                }

                // Check gas supply points
                for gas_sp in &property.gas_supply_points {
                    if gas_sp.status == "ON_SUPPLY" {
                        self.check_tariff(
                            &account.number,
                            &user_data.full_name,
                            gas_sp,
                            &available_products,
                            UtilityType::Gas,
                        )
                        .await?;
                    }
                }
            }
        }

        Ok(())
    }

    async fn check_tariff(
        &mut self,
        account_number: &str,
        full_name: &str,
        current_sp: &SupplyPoint,
        available_products: &[Product],
        utility_type: UtilityType,
    ) -> Result<()> {
        let (current_consumption, current_standing) = current_sp.product.params.parse_charges()?;

        for product in available_products {
            // Skip non-matching product types
            if !product.is_same_type(utility_type) {
                continue;
            }

            let (new_consumption, new_standing) = product.params.parse_charges()?;

            if new_consumption < current_consumption && new_standing <= current_standing {
                // Check cache before sending notification
                if self.cache.should_skip_notification(
                    account_number,
                    utility_type,
                    new_consumption,
                ) {
                    info!(
                        "Skipping {} notification for account {} - already sent for this offer",
                        utility_type.to_string(),
                        account_number
                    );
                    continue;
                }

                info!(
                    "Better {} tariff found for account {}: {} -> {}",
                    utility_type.to_string(),
                    account_number,
                    current_consumption,
                    new_consumption
                );

                self.email_service
                    .send_notification_email(
                        account_number,
                        full_name,
                        utility_type.to_string(),
                        &current_sp.product.params.consumption_charge,
                        &product.params.consumption_charge,
                    )
                    .await?;

                // Cache the notification after sending
                self.cache
                    .cache_notification(account_number, utility_type, new_consumption);
                self.cache.save_to_file(&self.config.cache_file_path)?;
            }
        }

        info!(
            "No better {} tariff found for account {}",
            utility_type.to_string(),
            account_number
        );
        Ok(())
    }

    /// Main execution flow
    async fn run_check(&mut self) -> Result<()> {
        info!("Starting Polipo tariff check at {}", Utc::now());

        let access_token = self.octopus_client.login().await?;

        let user_data = self.octopus_client.fetch_user_data(&access_token).await?;

        self.check_and_notify(&user_data).await?;

        info!("Polipo tariff check completed successfully");
        Ok(())
    }

    /// Run the application with cron scheduling
    pub async fn run_with_schedule(&mut self) -> Result<()> {
        info!(
            "Polipo configured with cron schedule: {}",
            self.config.cron_schedule
        );

        loop {
            // Parse cron schedule for next run
            let interval = parse_cron_interval(&self.config.cron_schedule)
                .context("Failed to parse cron schedule")?;

            info!("Next check scheduled in {:?}", interval);
            sleep(interval).await;

            match self.run_check().await {
                Ok(_) => info!("Check completed successfully"),
                Err(e) => error!("Check failed: {}", e),
            }
        }
    }
}
