# Rust Backend API Documentation

## Overview

The AIMonitor desktop application is built using Tauri 2.0 with a Rust backend. The backend provides secure API key management, database operations, and AI provider monitoring services.

## Architecture

```
src/
├── lib.rs           # Main application entry point and Tauri commands
├── main.rs          # Application bootstrap
├── db/              # Database layer
├── models/          # Data models and types
├── providers/       # AI provider implementations
└── services/        # Business logic services
```

## Core Components

### AppState

The application state manages shared resources:

```rust
pub struct AppState {
    pub db: Database,
    pub providers: Arc<RwLock<Vec<Provider>>>,
}
```

### Database Layer

Located in `src/db/mod.rs`, provides SQLite-based persistence:

- **Providers**: Store AI provider configurations
- **Usage History**: Track historical usage data
- **Settings**: Application configuration

### Provider System

Supports multiple AI providers through a common trait:

```rust
pub trait ProviderClient {
    async fn get_usage(&self, api_key: &str) -> Result<UsageData>;
    async fn get_balance(&self, api_key: &str) -> Result<Balance>;
}
```

**Supported Providers:**
- OpenAI (`src/providers/openai.rs`)
- Anthropic (`src/providers/anthropic.rs`)
- OpenRouter (`src/providers/openrouter.rs`)

## Tauri Commands

### add_provider

Add a new AI provider configuration.

**Request:**
```rust
struct AddProviderRequest {
    provider_type: ProviderType,
    api_key: String,
    name: String,
}
```

**Response:** `Result<(), String>`

**Example:**
```javascript
await invoke('add_provider', {
  request: {
    provider_type: 'OpenAI',
    api_key: 'sk-...',
    name: 'My OpenAI Account'
  }
});
```

### get_usage

Retrieve usage data for all enabled providers.

**Response:**
```rust
struct ProviderUsageResponse {
    providers: Vec<ProviderUsage>,
}
```

**Example:**
```javascript
const usage = await invoke('get_usage');
```

### remove_provider

Remove a provider and its stored API key.

**Parameters:**
- `provider_id: String` - Unique provider identifier

**Response:** `Result<(), String>`

### toggle_provider

Enable or disable a provider.

**Parameters:**
- `provider_id: String` - Unique provider identifier
- `enabled: bool` - New enabled state

**Response:** `Result<(), String>`

## Data Models

### Provider

```rust
pub struct Provider {
    pub id: String,
    pub name: String,
    pub provider_type: ProviderType,
    pub api_key_ref: Option<String>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}
```

### ProviderUsage

```rust
pub struct ProviderUsage {
    pub provider_id: String,
    pub name: String,
    pub provider_type: ProviderType,
    pub tokens_used: Option<u64>,
    pub tokens_limit: Option<u64>,
    pub cost_usd: Option<f64>,
    pub balance_usd: Option<f64>,
    pub last_updated: DateTime<Utc>,
}
```

### ProviderType

```rust
pub enum ProviderType {
    OpenAI,
    Anthropic,
    OpenRouter,
}
```

## Security

### API Key Storage

API keys are securely stored using the OS keychain via the `keyring` crate:

```rust
impl KeychainService {
    pub fn store_api_key(provider_id: &str, api_key: &str) -> Result<()>;
    pub fn get_api_key(provider_id: &str) -> Result<String>;
    pub fn delete_api_key(provider_id: &str) -> Result<()>;
}
```

**Storage Locations:**
- **macOS**: Keychain Access
- **Windows**: Windows Credential Store
- **Linux**: Secret Service API

### TLS Security

All API communications use TLS 1.2+ with certificate verification enabled.

## Services

### MonitorService

Handles background polling of AI providers:

```rust
impl MonitorService {
    pub async fn start_polling(&self);
    pub async fn get_all_usage(&self) -> Result<Vec<ProviderUsage>>;
    pub async fn poll_provider(&self, provider: &Provider) -> Result<ProviderUsage>;
}
```

**Features:**
- Configurable polling intervals
- Automatic retry with exponential backoff
- Error logging and notifications
- Rate limiting compliance

## System Tray Integration

The application runs in the system tray with the following menu items:

- **Show Dashboard**: Open the main window
- **Settings**: Open settings panel
- **Quit**: Exit application

## Error Handling

All operations use `Result<T, E>` types with comprehensive error handling:

```rust
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Keychain error: {0}")]
    Keychain(#[from] keyring::Error),
}
```

## Configuration

The application supports configuration through:

1. **Environment Variables**: For development overrides
2. **Config Files**: Stored in app data directory
3. **Database Settings**: Persistent user preferences

## Building

### Dependencies

Ensure you have the following installed:

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Tauri CLI
cargo install tauri-cli

# System dependencies (Ubuntu/Debian)
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

### Development

```bash
cd apps/desktop
cargo tauri dev
```

### Production Build

```bash
cd apps/desktop
cargo tauri build
```

## Testing

Run the test suite:

```bash
cargo test
```

For integration tests:

```bash
cargo test --test integration
```

## Logging

The application uses `tracing` for structured logging:

```rust
use tracing::{info, warn, error, debug};

info!("Provider added: {}", provider.name);
warn!("Rate limit approaching for provider: {}", provider.id);
error!("Failed to fetch usage data: {}", error);
```

Logs are written to:
- **Development**: Console output
- **Production**: App data directory logs folder

## Performance Considerations

- **Database**: SQLite with WAL mode for concurrent access
- **HTTP Client**: Connection pooling with `reqwest`
- **Memory**: Efficient data structures and minimal allocations
- **Background Tasks**: Non-blocking async operations

## Migration Guide

When updating provider schemas or database structure:

1. Create migration files in `src/db/migrations/`
2. Update model definitions
3. Increment schema version
4. Test migration paths thoroughly

## Troubleshooting

Common issues and solutions:

### Database Lock Errors
- Ensure proper async/await usage
- Check for unclosed database connections
- Consider transaction timeouts

### Keychain Access Denied
- Check application permissions
- Verify keychain service availability
- Handle graceful fallbacks

### Network Timeouts
- Implement retry mechanisms
- Check proxy configurations
- Validate API endpoints