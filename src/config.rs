use anyhow::{Context, Result};
use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    pub email: String,
    pub password: String,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub cron_schedule: String,
    pub cache_file_path: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        Ok(Config {
            email: env::var("OCTOPUS_EMAIL").context("OCTOPUS_EMAIL not set")?,
            password: env::var("OCTOPUS_PASSWORD").context("OCTOPUS_PASSWORD not set")?,
            smtp_server: env::var("SMTP_SERVER").context("SMTP_SERVER not set")?,
            smtp_port: env::var("SMTP_PORT")
                .context("SMTP_PORT not set")?
                .parse()
                .context("Invalid SMTP_PORT")?,
            smtp_username: env::var("SMTP_USERNAME").context("SMTP_USERNAME not set")?,
            smtp_password: env::var("SMTP_PASSWORD").context("SMTP_PASSWORD not set")?,
            cron_schedule: env::var("CRON_SCHEDULE").unwrap_or_else(|_| "0 9 * * *".to_string()),
            cache_file_path: env::var("CACHE_FILE_PATH")
                .unwrap_or_else(|_| "/tmp/polipo_cache.json".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    // Global mutex to ensure tests don't run in parallel and interfere with each other
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    /// Helper struct to manage environment variables for testing
    /// Automatically cleans up when dropped
    struct EnvGuard {
        vars: Vec<String>,
        _lock: std::sync::MutexGuard<'static, ()>,
    }

    impl EnvGuard {
        fn new() -> Self {
            EnvGuard {
                vars: Vec::new(),
                _lock: TEST_MUTEX.lock().unwrap(),
            }
        }

        fn set(&mut self, key: &str, value: &str) {
            unsafe {
                env::set_var(key, value);
            }
            self.vars.push(key.to_string());
        }

        fn remove(&mut self, key: &str) {
            unsafe {
                env::remove_var(key);
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for var in &self.vars {
                unsafe {
                    env::remove_var(var);
                }
            }
        }
    }

    fn setup_valid_env() -> EnvGuard {
        let mut guard = EnvGuard::new();
        guard.set("OCTOPUS_EMAIL", "test@example.com");
        guard.set("OCTOPUS_PASSWORD", "password123");
        guard.set("SMTP_SERVER", "smtp.example.com");
        guard.set("SMTP_PORT", "587");
        guard.set("SMTP_USERNAME", "smtp_user");
        guard.set("SMTP_PASSWORD", "smtp_pass");
        guard
    }

    #[test]
    fn test_config_from_env_all_required_vars() {
        let _guard = setup_valid_env();

        let config = Config::from_env().expect("Config should load successfully");

        assert_eq!(config.email, "test@example.com");
        assert_eq!(config.password, "password123");
        assert_eq!(config.smtp_server, "smtp.example.com");
        assert_eq!(config.smtp_port, 587);
        assert_eq!(config.smtp_username, "smtp_user");
        assert_eq!(config.smtp_password, "smtp_pass");
        assert_eq!(config.cron_schedule, "0 9 * * *"); // default value
        assert_eq!(config.cache_file_path, "/tmp/polipo_cache.json"); // default value
    }

    #[test]
    fn test_config_with_custom_optional_vars() {
        let mut guard = setup_valid_env();
        guard.set("CRON_SCHEDULE", "0 12 * * *");
        guard.set("CACHE_FILE_PATH", "/custom/path/cache.json");

        let config = Config::from_env().expect("Config should load successfully");

        assert_eq!(config.cron_schedule, "0 12 * * *");
        assert_eq!(config.cache_file_path, "/custom/path/cache.json");
    }

    #[test]
    fn test_config_missing_octopus_email() {
        let mut guard = setup_valid_env();
        guard.remove("OCTOPUS_EMAIL");

        let result = Config::from_env();

        assert!(result.is_err());
        let error_message = format!("{:?}", result.unwrap_err());
        assert!(error_message.contains("OCTOPUS_EMAIL not set"));
    }

    #[test]
    fn test_config_missing_octopus_password() {
        let mut guard = setup_valid_env();
        guard.remove("OCTOPUS_PASSWORD");

        let result = Config::from_env();

        assert!(result.is_err());
        let error_message = format!("{:?}", result.unwrap_err());
        assert!(error_message.contains("OCTOPUS_PASSWORD not set"));
    }

    #[test]
    fn test_config_missing_smtp_server() {
        let mut guard = setup_valid_env();
        guard.remove("SMTP_SERVER");

        let result = Config::from_env();

        assert!(result.is_err());
        let error_message = format!("{:?}", result.unwrap_err());
        assert!(error_message.contains("SMTP_SERVER not set"));
    }

    #[test]
    fn test_config_missing_smtp_port() {
        let mut guard = setup_valid_env();
        guard.remove("SMTP_PORT");

        let result = Config::from_env();

        assert!(result.is_err());
        let error_message = format!("{:?}", result.unwrap_err());
        assert!(error_message.contains("SMTP_PORT not set"));
    }

    #[test]
    fn test_config_missing_smtp_username() {
        let mut guard = setup_valid_env();
        guard.remove("SMTP_USERNAME");

        let result = Config::from_env();

        assert!(result.is_err());
        let error_message = format!("{:?}", result.unwrap_err());
        assert!(error_message.contains("SMTP_USERNAME not set"));
    }

    #[test]
    fn test_config_missing_smtp_password() {
        let mut guard = setup_valid_env();
        guard.remove("SMTP_PASSWORD");

        let result = Config::from_env();

        assert!(result.is_err());
        let error_message = format!("{:?}", result.unwrap_err());
        assert!(error_message.contains("SMTP_PASSWORD not set"));
    }

    #[test]
    fn test_config_invalid_smtp_port_non_numeric() {
        let mut guard = setup_valid_env();
        guard.set("SMTP_PORT", "not_a_number");

        let result = Config::from_env();

        assert!(result.is_err());
        let error_message = format!("{:?}", result.unwrap_err());
        assert!(error_message.contains("Invalid SMTP_PORT"));
    }

    #[test]
    fn test_config_invalid_smtp_port_negative() {
        let mut guard = setup_valid_env();
        guard.set("SMTP_PORT", "-1");

        let result = Config::from_env();

        assert!(result.is_err());
        let error_message = format!("{:?}", result.unwrap_err());
        assert!(error_message.contains("Invalid SMTP_PORT"));
    }

    #[test]
    fn test_config_invalid_smtp_port_too_large() {
        let mut guard = setup_valid_env();
        guard.set("SMTP_PORT", "99999");

        let result = Config::from_env();

        assert!(result.is_err());
        let error_message = format!("{:?}", result.unwrap_err());
        assert!(error_message.contains("Invalid SMTP_PORT"));
    }

    #[test]
    fn test_config_valid_smtp_port_edge_cases() {
        let test_cases = vec![
            ("1", 1),
            ("25", 25),
            ("465", 465),
            ("587", 587),
            ("2525", 2525),
            ("65535", 65535),
        ];

        for (port_str, expected_port) in test_cases {
            let mut guard = setup_valid_env();
            guard.set("SMTP_PORT", port_str);

            let config =
                Config::from_env().expect(&format!("Config should load with port {}", port_str));

            assert_eq!(config.smtp_port, expected_port);
        }
    }

    #[test]
    fn test_config_with_empty_strings() {
        let mut guard = setup_valid_env();
        guard.set("OCTOPUS_EMAIL", "");

        let config = Config::from_env().expect("Config should load with empty email");

        // Empty strings are valid, just not useful
        assert_eq!(config.email, "");
    }

    #[test]
    fn test_config_with_special_characters() {
        let mut guard = setup_valid_env();
        guard.set("OCTOPUS_EMAIL", "user+test@example.com");
        guard.set("OCTOPUS_PASSWORD", "p@$$w0rd!#%&*");
        guard.set("SMTP_PASSWORD", "smtp_p@ss!123");

        let config = Config::from_env().expect("Config should load with special characters");

        assert_eq!(config.email, "user+test@example.com");
        assert_eq!(config.password, "p@$$w0rd!#%&*");
        assert_eq!(config.smtp_password, "smtp_p@ss!123");
    }

    #[test]
    fn test_config_with_unicode_characters() {
        let mut guard = setup_valid_env();
        guard.set("OCTOPUS_PASSWORD", "пароль123");
        guard.set("CACHE_FILE_PATH", "/tmp/测试/cache.json");

        let config = Config::from_env().expect("Config should load with unicode");

        assert_eq!(config.password, "пароль123");
        assert_eq!(config.cache_file_path, "/tmp/测试/cache.json");
    }

    #[test]
    fn test_config_with_whitespace() {
        let mut guard = setup_valid_env();
        guard.set("OCTOPUS_EMAIL", "  test@example.com  ");
        guard.set("SMTP_SERVER", "\tsmtp.example.com\n");

        let config = Config::from_env().expect("Config should load");

        // Environment variables preserve whitespace
        assert_eq!(config.email, "  test@example.com  ");
        assert_eq!(config.smtp_server, "\tsmtp.example.com\n");
    }

    #[test]
    fn test_config_clone() {
        let _guard = setup_valid_env();
        let config1 = Config::from_env().expect("Config should load");
        let config2 = config1.clone();

        assert_eq!(config1.email, config2.email);
        assert_eq!(config1.password, config2.password);
        assert_eq!(config1.smtp_server, config2.smtp_server);
        assert_eq!(config1.smtp_port, config2.smtp_port);
        assert_eq!(config1.smtp_username, config2.smtp_username);
        assert_eq!(config1.smtp_password, config2.smtp_password);
        assert_eq!(config1.cron_schedule, config2.cron_schedule);
        assert_eq!(config1.cache_file_path, config2.cache_file_path);
    }

    #[test]
    fn test_config_debug_trait() {
        let _guard = setup_valid_env();
        let config = Config::from_env().expect("Config should load");

        let debug_str = format!("{:?}", config);

        // Verify Debug trait works and contains expected fields
        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("email"));
        assert!(debug_str.contains("smtp_server"));
    }

    #[test]
    fn test_config_default_cron_schedule_format() {
        let _guard = setup_valid_env();
        let config = Config::from_env().expect("Config should load");

        // Verify default cron schedule is valid
        assert_eq!(config.cron_schedule, "0 9 * * *");
        assert!(config.cron_schedule.split_whitespace().count() == 5);
    }

    #[test]
    fn test_config_default_cache_path_format() {
        let _guard = setup_valid_env();
        let config = Config::from_env().expect("Config should load");

        // Verify default cache path
        assert_eq!(config.cache_file_path, "/tmp/polipo_cache.json");
        assert!(config.cache_file_path.ends_with(".json"));
    }
}
