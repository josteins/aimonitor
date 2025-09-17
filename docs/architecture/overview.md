# Architecture Overview

## System Architecture

AI Monitor follows a multi-platform architecture with three main components:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Desktop App   │    │   Mobile App    │    │  Cloud Poller   │
│    (Tauri)      │    │   (Flutter)     │    │ (Cloudflare)    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Local SQLite  │    │ Secure Storage  │    │  KV Store/D1    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                                 ▼
                    ┌─────────────────┐
                    │  AI Providers   │
                    │ OpenAI/Anthropic│
                    │   OpenRouter    │
                    └─────────────────┘
```

## Component Responsibilities

### Desktop App (Tauri)
- **Primary**: Local monitoring and tray/menubar interface
- **Technology**: Rust backend + React frontend
- **Features**:
  - Real-time polling of AI providers
  - System tray notifications
  - Local data storage (SQLite)
  - Secure keychain integration
  - Dashboard UI

### Mobile App (Flutter)
- **Primary**: Mobile dashboard and push notifications
- **Technology**: Dart/Flutter (iOS/Android)
- **Features**:
  - Provider usage dashboard
  - Home screen widgets
  - Push notification handling
  - Secure storage for API keys
  - Cross-platform native performance

### Cloud Poller (Cloudflare Worker)
- **Primary**: Reliable cloud-based monitoring (optional)
- **Technology**: TypeScript on Cloudflare Workers
- **Features**:
  - Scheduled provider polling (cron)
  - Push notification dispatch
  - Aggregated data storage
  - JWT-based authentication
  - Global edge computing

## Data Flow

1. **Provider Registration**:
   - User adds API keys through UI
   - Keys stored securely (OS keychain/secure storage)
   - Provider configurations saved locally

2. **Usage Monitoring**:
   - Desktop: Local polling every 60s
   - Mobile: Retrieves data from cloud poller
   - Cloud: Scheduled polling every 5 minutes

3. **Alert Processing**:
   - Desktop: Local notifications
   - Mobile: Push notifications via FCM/APNs
   - Both: Configurable thresholds and budgets

4. **Data Persistence**:
   - Desktop: SQLite database (180 days retention)
   - Mobile: Temporary caching only
   - Cloud: KV store for aggregates, D1 for history

## Security Model

- **API Keys**: Stored in OS-native secure storage
- **Authentication**: JWT tokens for cloud services
- **Transport**: TLS 1.2+ for all communications
- **Privacy**: No prompt/response data stored, only usage metrics
- **Audit**: Local data purge capabilities

## Scalability Considerations

- **Desktop**: Handles up to 10 providers efficiently
- **Mobile**: Optimized for battery life with smart caching
- **Cloud**: Scales to thousands of users via Cloudflare edge network
- **Storage**: Automatic cleanup of old data (180-day retention)