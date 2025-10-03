# Polipo 🐙 - Octopus Energy Tariff Monitor

Polipo is a Rust application that automatically monitors Octopus Energy tariffs and notifies you when better deals become available. It compares your current electricity and gas tariffs with the latest 12-month fixed offers and sends email notifications when advantageous tariff changes are found.

> [!NOTE]
> This project has been developed and tested only for the italian market 🇮🇹.

## Features

- **Automatic Tariff Monitoring**: Fetches the latest Octopus Energy tariffs from their website
- **Account Integration**: Logs into your Octopus account to check your current tariffs
- **Smart Comparison**: Compares raw material prices and marketing costs to find better deals
- **Email Notifications**: Sends formatted emails to Octopus Energy requesting tariff adjustments
- **Configurable Scheduling**: Runs on a configurable cron schedule
- **Resource Efficient**: Built in Rust for minimal resource consumption

## Business Logic

The application follows this process:

1. **Fetch Latest Offers**: Retrieves the current 12-month fixed offers for electricity and gas from Octopus Energy's website
2. **Account Login**: Authenticates with your Octopus account using your credentials
3. **Current Tariff Check**: Fetches your current electricity and gas tariff details
4. **Comparison Logic**: For each supply point, checks if new offers have:
   - Lower or equal marketing price (annual standing charge)
   - Lower raw material price (consumption charge)
5. **Email Notification**: If better tariffs are found, sends a formatted email to Octopus Energy requesting the tariff change

## Installation

### Prerequisites

- Rust (latest stable version)
- Access to an SMTP server for sending emails
- Octopus Energy account credentials

### Build

```bash
git clone <repository-url>
cd polipo
cargo build --release
```

## Project Structure

The project is organized into modular components for better maintainability:

```
src/
├── main.rs             # Application entry point
├── lib.rs              # Library root and main application logic
├── config.rs           # Configuration management
├── octopus             # Code related to Octopus
   ├── models.rs        # Data structures for API responses
   ├── client.rs        # Octopus Energy API client
├── email.rs            # Email notification service
└── scheduler.rs        # Cron scheduling utilities
```

### Module Overview

- **`config`**: Handles environment variable configuration
- **`octpus`**: Contains the code related the communication with Octopus APIs
  - **`models`**: Defines all data structures for API responses and serialization
  - **`client`**: Contains the Octopus Energy API client with methods for login, tariff fetching, and user data retrieval
- **`email`**: Email service for sending SMTP notifications
- **`scheduler`**: Cron expression parsing and scheduling utilities
- **`lib`**: Main application logic and orchestration

## Configuration

Polipo uses environment variables for configuration. Create a `.env` file or set these variables in your environment:

### Required Environment Variables

```bash
# Octopus Energy account credentials
OCTOPUS_EMAIL=your-octopus-email@example.com
OCTOPUS_PASSWORD=your-octopus-password

# SMTP server configuration for sending emails
SMTP_SERVER=smtp.your-provider.com
SMTP_PORT=587
SMTP_USERNAME=your-smtp-username
SMTP_PASSWORD=your-smtp-password

# Optional: Cron schedule (defaults to "0 9 * * *" - daily at 9 AM)
CRON_SCHEDULE="0 9 * * *"

# Optional: Cache file location (defaults to "/tmp/polipo_cache.json")
CACHE_FILE_PATH=./cache.json
```

### Cron Schedule Format

The `CRON_SCHEDULE` uses standard cron format:
```
* * * * *
│ │ │ │ │
│ │ │ │ └─── Day of week (0-7, Sunday = 0 or 7)
│ │ │ └───── Month (1-12)
│ │ └─────── Day of month (1-31)
│ └───────── Hour (0-23)
└─────────── Minute (0-59)
```

Examples:
- `"0 9 * * *"` - Daily at 9:00 AM
- `"0 9 * * 1"` - Every Monday at 9:00 AM
- `"0 */6 * * *"` - Every 6 hours

## Usage

### Running the Application

```bash
# Using cargo
cargo run --release

# Or run the compiled binary
./target/release/polipo
```

### Docker (Optional)

Create a `Dockerfile`:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/polipo /usr/local/bin/polipo
CMD ["polipo"]
```

Build and run:

```bash
docker build -t polipo .
docker run -e OCTOPUS_EMAIL=your-email@example.com -e OCTOPUS_PASSWORD=your-password -e SMTP_SERVER=smtp.example.com -e SMTP_PORT=587 -e SMTP_USERNAME=smtp-user -e SMTP_PASSWORD=smtp-pass polipo
```

## Email Format

When a better tariff is found, Polipo sends an email with the following format:

**Subject**: `Richiesta adeguamento tariffa {TYPE} - account {ACCOUNT_NUMBER}`

**Body**:
```
Buongiorno,
con la presente richiedo l'adeguamento della mia tariffa {TYPE} con quella attualmente in commercio, per l'account {ACCOUNT_NUMBER}.
In dettaglio, vorrei passare dalla mia tariffa da {CURRENT_PRICE} a quella da {NEW_PRICE}.

Cordiali saluti,
{FULL_NAME}
```

Where:
- `{TYPE}` is "luce" (electricity) or "gas"
- `{ACCOUNT_NUMBER}` is your Octopus account number
- `{CURRENT_PRICE}` is your current consumption charge
- `{NEW_PRICE}` is the new, better consumption charge
- `{FULL_NAME}` is your full name from your Octopus account

## Logging

Polipo uses the `log` crate for logging. Set the `RUST_LOG` environment variable to control log levels:

```bash
export RUST_LOG=info  # Default level
export RUST_LOG=debug # More verbose
export RUST_LOG=error # Errors only
```

## Security Considerations

- Store sensitive environment variables (passwords, API keys) securely
- Consider using a secrets management system in production
- Ensure your SMTP credentials have limited permissions
- The application stores no persistent data and processes credentials in memory only

## Troubleshooting

### Common Issues

1. **Login Failure**: Check your Octopus Energy credentials
2. **Email Sending Failure**: Verify SMTP server settings and credentials
3. **Tariff Fetching Issues**: Ensure internet connectivity and check if Octopus Energy website structure has changed
4. **Cron Schedule**: Verify the cron expression format is correct

### Debug Mode

Run with debug logging to see detailed execution:

```bash
RUST_LOG=debug cargo run
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

This application is not affiliated with Octopus Energy. Use at your own risk and ensure compliance with Octopus Energy's terms of service. The authors are not responsible for any issues arising from the use of this software.