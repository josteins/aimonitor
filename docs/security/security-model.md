# Security Model

## Overview

AI Monitor prioritizes security and privacy by implementing defense-in-depth practices across all components.

## API Key Protection

### Storage

**Desktop (Tauri)**:
- **macOS**: Stored in Keychain using `keyring` crate
- **Windows**: Stored in Windows Credential Manager via DPAPI
- **Linux**: Stored in Secret Service (gnome-keyring/kwallet)

**Mobile (Flutter)**:
- **iOS**: Stored in iOS Keychain via `flutter_secure_storage`
- **Android**: Stored in Android Keystore with AES encryption

**Cloud (Optional)**:
- Stored in Cloudflare KV with encryption at rest
- Keys are encrypted with customer-managed keys
- Access via JWT authentication only

### Access Patterns

1. **Write-once principle**: API keys are stored once and never exposed in logs
2. **Least privilege**: Only the provider polling service can access keys
3. **Automatic cleanup**: Keys are removed when providers are deleted
4. **No plaintext storage**: Keys never written to disk unencrypted

## Data Privacy

### What We Store

**Metrics Only**:
- Token counts (input/output/cached)
- Cost in USD
- Balance/credits (where available)
- Timestamps and provider metadata

**What We DON'T Store**:
- ❌ Actual prompts or responses
- ❌ User conversation data
- ❌ Model responses or generated content
- ❌ Personal identifiable information in prompts

### Data Retention

- **Desktop**: 180 days local storage, automatic cleanup
- **Mobile**: Session-based caching only
- **Cloud**: 30 days for push notification history

## Network Security

### Transport Layer

- **TLS 1.2+** required for all communications
- **Certificate pinning** for provider APIs
- **HTTPS only** - no HTTP fallback
- **Request signing** for cloud poller authentication

### API Communication

```
Desktop/Mobile ─(HTTPS)─→ Provider APIs
      │
      └─(HTTPS + JWT)─→ Cloud Poller ─(HTTPS)─→ Provider APIs
```

### Rate Limiting

- Provider API calls limited to 1 request per minute per provider
- Cloud poller implements exponential backoff
- Burst protection to prevent API quota exhaustion

## Authentication & Authorization

### Desktop Application

- **No user accounts** - fully local operation
- **OS-level security** via keychain integration
- **Process isolation** - Rust backend separate from UI

### Mobile Application

- **Biometric unlock** optional for sensitive operations
- **App backgrounding protection** - secure storage locked when app backgrounded
- **Device binding** - API keys tied to device identifier

### Cloud Poller (Optional)

- **JWT-based authentication** with short-lived tokens
- **API key derivation** - cloud keys derived from user master key
- **Scoped access** - each user can only access their own data

## Threat Model

### Threats We Protect Against

1. **API Key Theft**:
   - Protection: Secure storage, no plaintext logs
   - Detection: Unusual usage pattern alerts

2. **Man-in-the-Middle Attacks**:
   - Protection: TLS with certificate pinning
   - Validation: Strong cipher suites only

3. **Local Data Exposure**:
   - Protection: Encrypted storage, automatic cleanup
   - Mitigation: No sensitive data in logs

4. **Unauthorized Access**:
   - Protection: OS-level security, biometric locks
   - Audit: Access logging (where possible)

### Threats Outside Scope

- Physical device compromise with root/admin access
- Provider API security (responsibility of OpenAI/Anthropic/etc.)
- Network infrastructure attacks (ISP/DNS level)

## Incident Response

### Data Breach Procedures

1. **Immediate**: Revoke affected API keys
2. **Short-term**: Notify users via app notifications
3. **Long-term**: Security audit and improvements

### User Actions

If you suspect compromise:
1. **Rotate API keys** in provider dashboards
2. **Clear local data** via app settings
3. **Review usage patterns** for anomalies

## Compliance

### Privacy Regulations

- **GDPR**: No personal data collection
- **CCPA**: Minimal data processing
- **SOC 2**: Security controls documented

### Industry Standards

- **OWASP**: Mobile and web security guidelines
- **NIST**: Cryptographic standards
- **RFC 7519**: JWT implementation

## Security Testing

### Regular Assessments

- **Static analysis** of Rust code via Clippy
- **Dependency scanning** for known vulnerabilities
- **Penetration testing** of cloud components

### Bug Bounty

We welcome responsible disclosure of security issues:
- **Email**: security@aimonitor.app
- **GPG Key**: Available on request
- **Response time**: 48 hours for initial response

## Configuration Recommendations

### For Maximum Security

1. **Use local monitoring only** (disable cloud poller)
2. **Enable biometric locks** on mobile
3. **Regular key rotation** (monthly)
4. **Monitor usage patterns** for anomalies
5. **Keep apps updated** for latest security patches

### For Enterprise

1. **Deploy cloud poller privately** on your infrastructure
2. **Use dedicated API keys** per environment
3. **Implement additional monitoring** via SIEM
4. **Regular security audits** of deployment