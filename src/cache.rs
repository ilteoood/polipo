use anyhow::{Context, Result};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a cached offer notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedOffer {
    /// The price of the offer that was notified
    pub price: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum UtilityType {
    Luce,
    Gas,
}

impl UtilityType {
    pub fn to_string(&self) -> &str {
        match self {
            UtilityType::Luce => "luce",
            UtilityType::Gas => "gas",
        }
    }
}

/// Cache structure to store all offer notifications
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OfferCache {
    /// Map of cache key to cached offer
    /// Key format: "{account_number}_{utility_type}"
    pub offers: HashMap<String, CachedOffer>,
}

impl OfferCache {
    /// Create a new empty cache
    pub fn new() -> Self {
        Self {
            offers: HashMap::new(),
        }
    }

    /// Load cache from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            info!("Cache file does not exist, creating new cache");
            return Ok(Self::new());
        }

        let content = fs::read_to_string(path)
            .context(format!("Failed to read cache file: {}", path.display()))?;

        if content.trim().is_empty() {
            info!("Cache file is empty, creating new cache");
            return Ok(Self::new());
        }

        let cache: OfferCache = serde_json::from_str(&content)
            .context(format!("Failed to parse cache file: {}", path.display()))?;

        info!("Loaded cache with {} entries", cache.offers.len());
        Ok(cache)
    }

    /// Save cache to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context(format!(
                "Failed to create cache directory: {}",
                parent.display()
            ))?;
        }

        let json = serde_json::to_string_pretty(self).context("Failed to serialize cache")?;

        fs::write(path, json).context(format!("Failed to write cache file: {}", path.display()))?;

        info!(
            "Saved cache with {} entries to {}",
            self.offers.len(),
            path.display()
        );
        Ok(())
    }

    /// Generate cache key for an offer
    fn generate_key(account_number: &str, utility_type: &UtilityType) -> String {
        format!("{}_{}", account_number, utility_type.to_string())
    }

    /// Check if an offer has already been cached with the same price
    /// Returns true if the offer should NOT be sent (same price already notified)
    pub fn should_skip_notification(
        &self,
        account_number: &str,
        utility_type: UtilityType,
        new_price: f64,
    ) -> bool {
        let key = Self::generate_key(account_number, &utility_type);

        if let Some(cached_offer) = self.offers.get(&key) {
            // Compare with small epsilon for floating point comparison
            const EPSILON: f64 = 0.0001;

            // Check if the new price is the same as the cached one
            if (cached_offer.price - new_price).abs() < EPSILON {
                info!(
                    "Skipping notification for {} {} - same price ({}) already notified",
                    account_number,
                    utility_type.to_string(),
                    new_price
                );
                return true;
            } else {
                // Different price, allow notification
                info!(
                    "Allowing notification for {} {} - price changed from {} to {}",
                    account_number,
                    utility_type.to_string(),
                    cached_offer.price,
                    new_price
                );
                return false;
            }
        }

        // No cached entry, allow notification
        false
    }

    /// Add a new offer notification to the cache
    pub fn cache_notification(
        &mut self,
        account_number: &str,
        utility_type: UtilityType,
        price: f64,
    ) {
        let key = Self::generate_key(account_number, &utility_type);

        let cached_offer = CachedOffer { price };

        info!(
            "Caching notification for {} {} with price {}",
            account_number,
            utility_type.to_string(),
            price
        );

        self.offers.insert(key, cached_offer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cache_key_generation() {
        let key = OfferCache::generate_key("ACC123", &UtilityType::Luce);
        assert_eq!(key, "ACC123_luce");
    }

    #[test]
    fn test_cache_notification() {
        let mut cache = OfferCache::new();

        cache.cache_notification("ACC123", UtilityType::Luce, 0.1089);

        assert_eq!(cache.offers.len(), 1);
        let cached = cache.offers.get("ACC123_luce").unwrap();
        assert_eq!(cached.price, 0.1089);
    }

    #[test]
    fn test_should_skip_notification_same_price() {
        let mut cache = OfferCache::new();

        // Cache a notification
        cache.cache_notification("ACC123", UtilityType::Gas, 0.4295);

        // Should be skipped since same price is cached
        assert!(cache.should_skip_notification("ACC123", UtilityType::Gas, 0.4295));
    }

    #[test]
    fn test_should_skip_notification_different_price() {
        let mut cache = OfferCache::new();

        // Cache a notification
        cache.cache_notification("ACC123", UtilityType::Gas, 0.4295);

        // Should NOT be skipped since price is different
        assert!(!cache.should_skip_notification("ACC123", UtilityType::Gas, 0.3800));
    }

    #[test]
    fn test_should_skip_notification_no_cache() {
        let cache = OfferCache::new();

        // No cached entry should not be skipped
        assert!(!cache.should_skip_notification("ACC123", UtilityType::Luce, 0.1089));
    }

    #[test]
    fn test_price_update() {
        let mut cache = OfferCache::new();

        // Cache initial price
        cache.cache_notification("ACC123", UtilityType::Luce, 0.1089);
        assert_eq!(cache.offers.get("ACC123_luce").unwrap().price, 0.1089);

        // Update with new price
        cache.cache_notification("ACC123", UtilityType::Luce, 0.0950);
        assert_eq!(cache.offers.get("ACC123_luce").unwrap().price, 0.0950);

        // Still only one entry
        assert_eq!(cache.offers.len(), 1);
    }

    #[test]
    fn test_multiple_accounts() {
        let mut cache = OfferCache::new();

        // Cache different accounts and utilities
        cache.cache_notification("ACC123", UtilityType::Luce, 0.1089);
        cache.cache_notification("ACC123", UtilityType::Gas, 0.4295);
        cache.cache_notification("ACC456", UtilityType::Luce, 0.1100);

        assert_eq!(cache.offers.len(), 3);
        assert_eq!(cache.offers.get("ACC123_luce").unwrap().price, 0.1089);
        assert_eq!(cache.offers.get("ACC123_gas").unwrap().price, 0.4295);
        assert_eq!(cache.offers.get("ACC456_luce").unwrap().price, 0.1100);
    }

    #[test]
    fn test_save_and_load_cache() {
        let temp_dir = TempDir::new().unwrap();
        let cache_path = temp_dir.path().join("test_cache.json");

        // Create and save cache
        let mut cache = OfferCache::new();
        cache.cache_notification("ACC123", UtilityType::Luce, 0.1089);
        cache.cache_notification("ACC456", UtilityType::Gas, 0.4295);
        cache.save_to_file(&cache_path).unwrap();

        // Load cache
        let loaded_cache = OfferCache::load_from_file(&cache_path).unwrap();

        assert_eq!(loaded_cache.offers.len(), 2);
        assert_eq!(
            loaded_cache.offers.get("ACC123_luce").unwrap().price,
            0.1089
        );
        assert_eq!(loaded_cache.offers.get("ACC456_gas").unwrap().price, 0.4295);
    }

    #[test]
    fn test_floating_point_comparison() {
        let mut cache = OfferCache::new();

        // Cache a price
        cache.cache_notification("ACC123", UtilityType::Luce, 0.1089);

        // Test with very close value (should be considered same due to epsilon)
        assert!(cache.should_skip_notification("ACC123", UtilityType::Luce, 0.10890001));

        // Test with clearly different value
        assert!(!cache.should_skip_notification("ACC123", UtilityType::Luce, 0.1090));
    }

    #[test]
    fn test_load_nonexistent_cache() {
        let cache = OfferCache::load_from_file("/tmp/nonexistent_cache_12345.json").unwrap();
        assert_eq!(cache.offers.len(), 0);
    }
}
