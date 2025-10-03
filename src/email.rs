use anyhow::{Context, Result};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::info;

use crate::config::Config;

/// Email service for sending notifications
pub struct EmailService {
    config: Config,
}

impl EmailService {
    /// Create a new email service
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Send email notification about tariff change opportunity
    pub async fn send_notification_email(
        &self,
        account_number: &str,
        full_name: &str,
        tariff_type: &str,
        current_price: &str,
        new_price: &str,
    ) -> Result<()> {
        info!("Sending email notification for account {}", account_number);

        let subject = format!(
            "Richiesta adeguamento tariffa {} - account {}",
            tariff_type, account_number
        );

        let body = format!(
            "Buongiorno,\ncon la presente richiedo l'adeguamento della mia tariffa {} con quella attualmente in commercio, per l'account {}.\nIn dettaglio, vorrei passare dalla mia tariffa da {} a quella da {}.\n\nCordiali saluti,\n{}",
            tariff_type, account_number, current_price, new_price, full_name
        );

        let email = Message::builder()
            .from(
                format!("{} <{}>", full_name, self.config.smtp_username)
                    .parse()
                    .context("Invalid from address")?,
            )
            .to("ciao@octopusenergy.it"
                .parse()
                .context("Invalid to address")?)
            .subject(&subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)
            .context("Failed to build email")?;

        let credentials = Credentials::new(
            self.config.smtp_username.clone(),
            self.config.smtp_password.clone(),
        );

        let mailer = SmtpTransport::relay(&self.config.smtp_server)
            .context("Failed to create SMTP transport")?
            .credentials(credentials)
            .port(self.config.smtp_port)
            .build();

        match mailer.send(&email) {
            Ok(_) => {
                info!(
                    "Email sent successfully to Octopus Energy for account {}",
                    account_number
                );
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Failed to send email: {}", e)),
        }
    }
}
