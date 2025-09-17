# AI Monitor - Multi-Platform AI Usage & Balance Monitor

A comprehensive cross-platform application to monitor usage, spend, and balance across multiple AI providers (OpenAI, Anthropic, OpenRouter).

## Architecture

- **Desktop App** (Tauri): macOS/Windows/Linux menubar app with local polling
- **Mobile App** (Flutter): iOS/Android app with push notifications
- **Cloud Poller** (Cloudflare Worker): Optional cloud service for reliable mobile alerts

## Project Structure

```
AImonitor/
├── apps/
│   ├── desktop/       # Tauri desktop app (Rust + React)
│   ├── mobile/        # Flutter mobile app
│   └── cloud-poller/  # Cloudflare Worker
├── packages/
│   ├── shared/        # Shared utilities
│   └── types/         # TypeScript type definitions
└── package.json       # Monorepo root
```

## Features

- ✅ Monitor multiple AI providers (OpenAI, Anthropic, OpenRouter)
- ✅ Real-time usage tracking (tokens, cost, balance)
- ✅ Desktop menubar/tray with quick stats
- ✅ Mobile dashboard with widgets
- ✅ Configurable budget alerts
- ✅ Secure API key storage (OS keychain)
- ✅ Historical data & export
- ✅ Push notifications for alerts

## Getting Started

### Prerequisites

- Node.js 20+
- pnpm 9+
- Rust & Cargo
- Flutter SDK
- Tauri CLI

### Installation

```bash
# Install dependencies
pnpm install

# Desktop app
cd apps/desktop
pnpm tauri dev

# Mobile app
cd apps/mobile
flutter run

# Cloud poller
cd apps/cloud-poller
pnpm dev
```

## Building

### Desktop

```bash
cd apps/desktop
pnpm tauri build
```

### Mobile

```bash
cd apps/mobile

# iOS
flutter build ios

# Android
flutter build apk
```

### Cloud Poller

```bash
cd apps/cloud-poller
pnpm deploy
```

## Configuration

### Desktop App

API keys are stored securely in the OS keychain. Add providers through the UI.

### Mobile App

Configure providers in Settings. Enable push notifications for alerts.

### Cloud Poller

Update `wrangler.toml` with your Cloudflare credentials and deploy.

## Security

- API keys stored in OS keychain (desktop) or secure storage (mobile)
- No plaintext secrets on disk
- TLS 1.2+ for all API communications
- Optional self-hosted cloud poller for privacy

## License

MIT