# Getting Started Guide

## Prerequisites

Before you begin, ensure you have the following installed:

### For Desktop Development
- **Node.js** 20+ ([Download](https://nodejs.org/))
- **pnpm** 9+ (`npm install -g pnpm`)
- **Rust** & **Cargo** ([rustup.rs](https://rustup.rs/))
- **Tauri CLI** (`cargo install tauri-cli`)

### For Mobile Development
- **Flutter SDK** 3.24+ ([Install Guide](https://flutter.dev/docs/get-started/install))
- **Android Studio** (for Android development)
- **Xcode** (for iOS development on macOS)

### For Cloud Development
- **Wrangler CLI** (`npm install -g wrangler`)
- **Cloudflare Account** ([Sign up](https://dash.cloudflare.com/sign-up))

## Quick Start

### 1. Clone and Install

```bash
git clone https://github.com/yourusername/aimonitor.git
cd aimonitor
pnpm install
```

### 2. Desktop App

```bash
cd apps/desktop
pnpm tauri dev
```

The desktop app will open with the dashboard interface.

### 3. Mobile App

```bash
cd apps/mobile
flutter pub get
flutter run
```

Select your device/simulator when prompted.

### 4. Cloud Poller (Optional)

```bash
cd apps/cloud-poller
wrangler login
wrangler deploy
```

## Configuration

### Adding AI Providers

1. **Desktop**: Click "Add Provider" in the dashboard
2. **Mobile**: Tap the "+" button on the main screen

Supported providers:
- **OpenAI**: Requires API key starting with `sk-`
- **Anthropic**: Requires API key starting with `sk-ant-`
- **OpenRouter**: Requires API key starting with `sk-or-`

### Setting Up Budgets

1. Navigate to provider settings
2. Set soft and hard limits in USD
3. Configure alert preferences

### Cloud Poller Setup

1. Create KV namespaces in Cloudflare:
   ```bash
   wrangler kv:namespace create "KV"
   wrangler kv:namespace create "SECRETS"
   ```

2. Update `wrangler.toml` with your namespace IDs

3. Set environment variables:
   ```bash
   wrangler secret put JWT_SECRET
   wrangler secret put PUSH_KEY
   ```

## Development Workflow

### Making Changes

1. **Desktop**: Edit React components in `apps/desktop/src/`
2. **Backend**: Modify Rust code in `apps/desktop/src-tauri/src/`
3. **Mobile**: Edit Flutter code in `apps/mobile/lib/`

### Testing

```bash
# Desktop
cd apps/desktop
pnpm tauri dev

# Mobile
cd apps/mobile
flutter test

# Cloud
cd apps/cloud-poller
pnpm test
```

### Building for Production

```bash
# Desktop
cd apps/desktop
pnpm tauri build

# Mobile
cd apps/mobile
flutter build apk  # Android
flutter build ios  # iOS

# Cloud
cd apps/cloud-poller
pnpm deploy
```

## Troubleshooting

### Common Issues

1. **Desktop app won't start**:
   - Check Rust installation: `rustc --version`
   - Verify Tauri CLI: `cargo tauri --version`
   - Clear cache: `cargo clean`

2. **Mobile build fails**:
   - Run `flutter doctor` to check setup
   - Ensure proper SDK versions
   - Clear Flutter cache: `flutter clean`

3. **Cloud deployment errors**:
   - Verify Wrangler authentication: `wrangler whoami`
   - Check `wrangler.toml` configuration
   - Ensure all secrets are set

### Getting Help

- **GitHub Issues**: [Report bugs](https://github.com/yourusername/aimonitor/issues)
- **Discussions**: [Community support](https://github.com/yourusername/aimonitor/discussions)
- **Wiki**: [Detailed documentation](https://github.com/yourusername/aimonitor/wiki)