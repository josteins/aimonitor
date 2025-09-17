# Contributing to AI Monitor

We love your input! We want to make contributing to AI Monitor as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## Development Process

We use GitHub to host code, to track issues and feature requests, as well as accept pull requests.

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code lints
6. Issue that pull request!

## Development Setup

### Prerequisites

- Node.js 20+
- pnpm 9+
- Rust & Cargo
- Flutter SDK (for mobile development)

### Setup

```bash
git clone https://github.com/yourusername/aimonitor.git
cd aimonitor
pnpm install
```

### Running Tests

```bash
# Desktop
cd apps/desktop
pnpm test
cargo test

# Mobile
cd apps/mobile
flutter test

# Cloud
cd apps/cloud-poller
pnpm test
```

### Code Style

#### Rust
- Follow `rustfmt` formatting
- Use `clippy` for linting
- Write comprehensive doc comments for public APIs

#### TypeScript/JavaScript
- Use Prettier for formatting
- Follow ESLint configuration
- Prefer functional programming patterns

#### Dart/Flutter
- Follow `dart format` styling
- Use `dart analyze` for linting
- Follow Flutter best practices

## Architecture Guidelines

### Desktop App (Tauri)

- **Rust Backend**: Handle all provider communication and data storage
- **React Frontend**: Focus on UI/UX, minimal business logic
- **Security First**: Never expose API keys to the frontend

### Mobile App (Flutter)

- **Provider Pattern**: Use providers for state management
- **Secure Storage**: Always use flutter_secure_storage for sensitive data
- **Platform Channels**: When needed for native functionality

### Cloud Poller

- **Serverless**: Keep functions small and focused
- **Error Handling**: Comprehensive error handling and logging
- **Security**: JWT authentication for all endpoints

## Commit Messages

Use conventional commits format:

```
feat: add support for Claude-3 models
fix: resolve memory leak in polling service
docs: update API documentation
style: format code with prettier
refactor: extract provider adapters
test: add unit tests for usage calculation
chore: update dependencies
```

## Coding Standards

### Security

- Never log API keys or sensitive data
- Use secure storage for all credentials
- Validate all inputs from external APIs
- Follow principle of least privilege

### Performance

- Cache API responses appropriately
- Use efficient data structures
- Minimize memory allocations in hot paths
- Profile before optimizing

### Error Handling

- Use Result types in Rust
- Provide meaningful error messages
- Log errors with appropriate levels
- Graceful degradation when possible

## Documentation

### Code Documentation

- Document all public APIs
- Include usage examples
- Explain complex algorithms
- Keep docs up to date with code

### User Documentation

- Write clear setup instructions
- Provide troubleshooting guides
- Include screenshots where helpful
- Test all documented procedures

## Testing

### Unit Tests

- Test all business logic
- Mock external dependencies
- Test error conditions
- Aim for 80%+ coverage

### Integration Tests

- Test complete user workflows
- Test across platform boundaries
- Verify security properties
- Test with real (test) API keys

### Manual Testing

- Test on all target platforms
- Verify UI/UX flows
- Test with real usage patterns
- Performance testing under load

## Feature Requests

### Before Proposing

1. Check existing issues and discussions
2. Consider if it fits the project scope
3. Think about implementation complexity
4. Consider security implications

### Proposal Format

```markdown
## Problem
What problem does this solve?

## Solution
What is your proposed solution?

## Alternatives
What other approaches did you consider?

## Implementation
How would this be implemented?

## Testing
How would this be tested?
```

## Bug Reports

### Before Reporting

1. Update to the latest version
2. Search existing issues
3. Try to reproduce consistently
4. Gather relevant logs/screenshots

### Bug Report Format

```markdown
## Description
Brief description of the issue

## Steps to Reproduce
1. Step one
2. Step two
3. Step three

## Expected Behavior
What should happen?

## Actual Behavior
What actually happens?

## Environment
- OS: [e.g., macOS 14.0]
- App Version: [e.g., 1.0.0]
- Provider: [e.g., OpenAI]

## Logs
Relevant log output (redact sensitive info)
```

## Release Process

### Versioning

We use [Semantic Versioning](https://semver.org/):
- `MAJOR.MINOR.PATCH`
- Major: Breaking changes
- Minor: New features
- Patch: Bug fixes

### Release Checklist

- [ ] Update version numbers
- [ ] Update CHANGELOG.md
- [ ] Test on all platforms
- [ ] Update documentation
- [ ] Create release notes
- [ ] Tag release
- [ ] Deploy cloud components
- [ ] Publish app store updates

## Community

### Code of Conduct

Be respectful and inclusive. We follow the [Contributor Covenant](https://www.contributor-covenant.org/).

### Getting Help

- **GitHub Discussions**: For questions and ideas
- **GitHub Issues**: For bugs and feature requests
- **Discord**: [Community chat](https://discord.gg/aimonitor) (if available)

### Recognition

Contributors are recognized in:
- CONTRIBUTORS.md file
- Release notes
- GitHub repository insights
- Special thanks in major releases

## License

By contributing, you agree that your contributions will be licensed under the MIT License.