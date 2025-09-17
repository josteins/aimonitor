# Desktop API Commands

The Tauri desktop app exposes several commands that can be called from the React frontend.

## Provider Management

### `add_provider`

Adds a new AI provider to monitor.

**Parameters:**
```typescript
interface AddProviderRequest {
  provider_type: 'openai' | 'anthropic' | 'openrouter';
  api_key: string;
  name: string;
}
```

**Usage:**
```typescript
import { invoke } from '@tauri-apps/api/core';

await invoke('add_provider', {
  request: {
    provider_type: 'openai',
    api_key: 'sk-...',
    name: 'Production OpenAI'
  }
});
```

**Returns:** `Promise<void>`

**Errors:**
- `"Invalid API key format"`
- `"Provider already exists"`
- `"Keychain storage failed"`

### `remove_provider`

Removes a provider and its stored API key.

**Parameters:**
- `provider_id: string` - The unique provider identifier

**Usage:**
```typescript
await invoke('remove_provider', { providerId: 'openai_12345' });
```

**Returns:** `Promise<void>`

### `toggle_provider`

Enables or disables polling for a provider.

**Parameters:**
- `provider_id: string` - The provider identifier
- `enabled: boolean` - Whether to enable polling

**Usage:**
```typescript
await invoke('toggle_provider', {
  providerId: 'openai_12345',
  enabled: false
});
```

**Returns:** `Promise<void>`

## Usage Data

### `get_usage`

Retrieves current usage data for all providers.

**Usage:**
```typescript
interface ProviderUsageResponse {
  providers: ProviderUsage[];
}

interface ProviderUsage {
  provider: {
    id: string;
    name: string;
    provider_type: string;
    enabled: boolean;
  };
  today_tokens: number;
  today_cost: number;
  mtd_tokens: number;
  mtd_cost: number;
  balance?: number;
  credits?: number;
  budget_used_percentage?: number;
}

const usage = await invoke<ProviderUsageResponse>('get_usage');
```

**Returns:** `Promise<ProviderUsageResponse>`

## Internal Backend APIs

### Database Operations

The Rust backend provides several internal APIs for data management:

#### Metrics Storage

```rust
impl Database {
    pub async fn insert_metric(&self, metric: &Metric) -> Result<()>;
    pub async fn get_recent_metrics(&self, provider_id: &str, hours: i64) -> Result<Vec<Metric>>;
    pub async fn cleanup_old_metrics(&self, days: i64) -> Result<()>;
}
```

#### Provider Adapters

Each provider implements the `ProviderAdapter` trait:

```rust
#[async_trait]
pub trait ProviderAdapter: Send + Sync {
    async fn fetch_usage(&self, api_key: &str, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<Metric>>;
    async fn fetch_balance(&self, api_key: &str) -> Result<Option<f64>>;
    async fn get_current_usage(&self, api_key: &str) -> Result<ProviderUsage>;
}
```

#### Keychain Service

Secure API key storage:

```rust
impl KeychainService {
    pub fn store_api_key(provider: &str, key: &str) -> Result<()>;
    pub fn get_api_key(provider: &str) -> Result<String>;
    pub fn delete_api_key(provider: &str) -> Result<()>;
    pub fn has_api_key(provider: &str) -> bool;
}
```

## Error Handling

All commands return `Result<T, String>` where errors are serialized as strings for the frontend.

Common error patterns:
- Network errors: `"Failed to connect to provider API"`
- Authentication: `"Invalid API key"`
- Storage: `"Keychain access denied"`
- Parsing: `"Invalid response format"`

## Event System

The app uses Tauri's event system for real-time updates:

```typescript
import { listen } from '@tauri-apps/api/event';

// Listen for usage updates
await listen('usage-updated', (event) => {
  console.log('New usage data:', event.payload);
});

// Listen for alerts
await listen('alert-triggered', (event) => {
  console.log('Alert:', event.payload);
});
```

## Configuration

The app stores configuration in:
- **macOS**: `~/Library/Application Support/com.aimonitor.desktop/`
- **Windows**: `%APPDATA%/com.aimonitor.desktop/`
- **Linux**: `~/.config/com.aimonitor.desktop/`

Files:
- `aimonitor.db` - SQLite database
- `config.json` - Application settings (if any)