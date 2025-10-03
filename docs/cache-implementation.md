# Cache Implementation

## Overview

The cache system in Polipo stores the price of offers that have been notified to avoid sending duplicate emails for the same price.

## Cache Structure

The cache is a simple JSON file with the following structure:

```json
{
  "offers": {
    "{account_number}_{utility_type}": {
      "price": 0.1089
    }
  }
}
```

### Example

```json
{
  "offers": {
    "ACC123_luce": {
      "price": 0.1089
    },
    "ACC123_gas": {
      "price": 0.4295
    }
  }
}
```

## How It Works

1. **Before sending an email**: The system checks if there's a cached entry for the account/utility combination
2. **Price comparison**: If a cached entry exists, it compares the new offer price with the cached price using floating-point comparison with epsilon (0.0001) for accuracy
3. **Decision**:
   - If prices match (within epsilon) → Skip notification (already sent for this price)
   - If prices differ → Send notification and update cache with new price
   - If no cache entry → Send notification and create new cache entry

## Configuration

Set the cache file path using the `CACHE_FILE_PATH` environment variable:

```bash
CACHE_FILE_PATH=/path/to/cache.json
```

Default: `/tmp/polipo_cache.json`
