Here’s a full Software Requirements Document (SRD) for your cross-platform AI usage monitor (Option A: Tauri desktop + Flutter mobile + optional cloud poller). I’ve structured it like a professional SRD so you or your devs can hand it directly into implementation.

⸻

Software Requirements Document (SRD)

Project: Multi-Platform AI Usage & Balance Monitor
Author: [Your name / team]
Version: 1.0
Date: 17 September 2025

⸻

1. Introduction

1.1 Purpose

This system monitors usage, spend, and balance across multiple AI providers (OpenAI, Anthropic, OpenRouter). It provides real-time insights, alerts when thresholds are hit, and works seamlessly across macOS, Windows, iOS, and Android.

Desktop apps (Tauri) focus on menubar/tray UX + local polling, while mobile apps (Flutter) emphasise dashboard + push notifications. An optional lightweight cloud poller provides reliable mobile alerts.

1.2 Scope
	•	Monitor: Tokens consumed, cost (daily & MTD), balance/credits (where available).
	•	Alert: System notifications and push/webhooks when soft/hard thresholds are crossed.
	•	Visualise: Compact menubar badge + detailed dashboard with sparklines.
	•	Secure: Keys stored in OS keychain/secure storage; no plaintext secrets on disk.
	•	Cross-platform: macOS/Windows/Linux (desktop), iOS/Android (mobile).

⸻

2. Functional Requirements

2.1 Providers Supported
	1.	OpenAI
	•	✅ Usage & cost: via official Usage API.
	•	⚠️ No official balance → app uses user-defined budgets.
	2.	Anthropic
	•	✅ Usage & cost: via Admin API (/usage_report, /cost_report).
	•	⚠️ No balance → budgets only.
	3.	OpenRouter
	•	✅ Credits/balance via /api/v1/key and /api/v1/credits.
	•	✅ Usage & cost via credit deltas.

⸻

2.2 Core Features
	•	Dashboard
	•	Per-provider cards with today/MTD tokens, cost, balance (if available).
	•	24h sparklines for spend/usage.
	•	Menubar/Tray (desktop)
	•	Badge shows either remaining credits (OpenRouter) or MTD spend (others).
	•	Dropdown with provider summaries.
	•	Mobile
	•	Flutter dashboard with same provider cards.
	•	iOS/Android home widgets.
	•	Alerts via push notifications (APNs/FCM).
	•	Alerts
	•	Soft & hard thresholds: absolute ($, credits) or percentage of budget.
	•	Projected run-out date alerts.
	•	Channels: native notification, push, Slack/Discord webhook.
	•	History & Export
	•	SQLite storage (180 days).
	•	Export to CSV/JSON.

⸻

2.3 Non-Functional Requirements
	•	Performance: Desktop idle RAM < 20 MB. Poll intervals ≥ 60s.
	•	Security: API keys in Keychain/Keystore. HTTPS only. Local purge option.
	•	Reliability: Cached values displayed when offline; alerts debounced.
	•	Extensibility: Provider adapters modular; new providers added easily.

⸻

3. System Architecture

3.1 High-Level Overview
	•	Desktop App (Tauri)
	•	Rust core: polling, alerting, storage.
	•	Front-end: React/TypeScript (Tauri webview).
	•	Local SQLite for metrics, budgets, events.
	•	Mobile App (Flutter)
	•	Flutter UI for iOS/Android.
	•	Secure storage for keys.
	•	Push notifications via APNs/FCM.
	•	Optional: query cloud poller for historical metrics.
	•	Cloud Poller (optional)
	•	Runs on Cloudflare Worker or Fly.io.
	•	Polls providers with stored keys (if user opts in).
	•	Pushes alerts to mobile via APNs/FCM.
	•	Stores only aggregates (no prompts/PII).

⸻

3.2 Component Diagram

graph TD
  subgraph Desktop[Tauri Desktop App]
    A1[Tray UI]
    A2[Rust Core]
    A3[SQLite]
    A4[Keychain]
  end

  subgraph Mobile[Flutter Mobile App]
    B1[Flutter UI]
    B2[Secure Storage]
    B3[Push Notifications]
  end

  subgraph Cloud[Optional Poller]
    C1[Worker Cron]
    C2[Provider APIs]
    C3[Push Service]
  end

  A2 --> A1
  A2 --> A3
  A2 --> A4
  B1 --> B2
  C1 --> C2
  C1 --> C3
  C3 --> B3


⸻

4. Data Model

4.1 Tables
	•	providers(id, type, label, created_at)
	•	api_keys(id, provider_id, alias, keychain_ref, created_at)
	•	metrics(id, provider_id, metric_type, ts, value, unit, dimensions)
	•	budgets(provider_id, period, soft_limit, hard_limit, notes)
	•	alerts(id, provider_id, rule_json, last_fired_at, status)
	•	events(id, provider_id, ts, kind, payload)

4.2 Metrics
	•	tokens_in, tokens_out, tokens_cached
	•	cost_usd
	•	credits_remaining

⸻

5. API Integration

5.1 OpenAI
	•	GET /v1/usage (Usage API) → tokens/cost.
	•	No balance → app budgets.

5.2 Anthropic
	•	GET /v1/organizations/usage_report/messages
	•	GET /v1/organizations/cost_report

5.3 OpenRouter
	•	GET /api/v1/key → credits & rate limits.
	•	GET /api/v1/credits → purchased vs used.

⸻

6. Security
	•	Storage: All keys in Keychain (macOS/iOS), DPAPI/Windows Vault, Keystore (Android).
	•	Transport: TLS 1.2+ only.
	•	Privacy: No prompt/response data stored, only usage/spend numbers.
	•	Controls: User can purge all data & revoke keys instantly.

⸻

7. User Experience

7.1 Desktop UX
	•	Menubar icon:
	•	Green = healthy
	•	Yellow = soft alert
	•	Red = hard alert
	•	Dropdown: provider cards, budget %, sparklines.

7.2 Mobile UX
	•	Dashboard: provider tiles with sparkline + budget bar.
	•	Widgets: “Remaining credits” or “MTD spend.”
	•	Notifications: “⚠️ OpenAI spend at 90% budget ($1,800 / $2,000).”

⸻

8. Alerting Logic
	•	Soft Alert: when spend ≥ soft_limit OR credits ≤ soft_threshold.
	•	Hard Alert: when spend ≥ hard_limit OR credits ≤ hard_threshold.
	•	Projection: if current run-rate implies breach before reset date.
	•	Debounce: once per 6h unless condition worsens by ≥10%.

⸻

9. Deployment
	•	Desktop:
	•	macOS DMG (notarised).
	•	Windows MSIX installer.
	•	Auto-updates via Tauri updater.
	•	Mobile:
	•	App Store (iOS).
	•	Google Play (Android).
	•	Cloud poller:
	•	Deployable on Cloudflare Workers, Fly.io, or AWS Lambda.
	•	Encrypted secret store (KMS, Vault, or 1Password Connect).

⸻

10. Future Extensions
	•	Additional providers: xAI, Mistral, AWS Bedrock.
	•	Team accounts: per-workspace spend dashboards.
	•	Advanced forecasting: ML-based cost prediction.
	•	SSO (Google, Microsoft) for enterprise rollout.

⸻

✅ This SRD is implementation-ready: devs can scaffold the repo (Rust core, Tauri shell, Flutter shell, optional Worker) and start coding.

Use monorepo