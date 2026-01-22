# GitHub Actions Setup Guide

This document describes the CI/CD workflows configured for the PAGI Twin Desktop project.

## Workflows Overview

| Workflow | File | Trigger | Purpose |
|----------|------|---------|---------|
| **CI** | `ci.yml` | Push/PR to main/develop | Build, test, lint |
| **Security** | `security.yml` | Push/PR + Weekly | Security audits |
| **Release** | `release.yml` | Tags (v*) or manual | Create releases |

## CI Workflow (`ci.yml`)

### Jobs

1. **rust-build** - Builds the Rust workspace on Linux and Windows
   - Compiles `pagi-sola-web` binary
   - Uploads artifacts for both platforms

2. **rust-test** - Runs all workspace tests
   - Depends on successful build
   - Uses `cargo test --workspace`

3. **rust-lint** - Code quality checks
   - `cargo fmt --check` - Formatting
   - `cargo clippy` - Linting with warnings as errors

4. **frontend-build** - Builds the React frontend
   - TypeScript type checking
   - Vite production build
   - Uploads dist artifact

5. **integration-test** - End-to-end verification
   - Downloads backend and frontend artifacts
   - Starts backend server
   - Runs health and API checks

### Triggers

```yaml
on:
  push:
    branches: [main, master, develop]
  pull_request:
    branches: [main, master, develop]
```

## Security Workflow (`security.yml`)

### Jobs

1. **rust-security-quality** - Rust security checks
   - `cargo audit` - Vulnerability scanning
   - Dangerous pattern scanning (informational)
   - Full test suite

2. **frontend-security** - Frontend security
   - `npm audit` - Dependency vulnerabilities
   - Outdated package check

3. **license-check** - License compliance
   - `cargo license` - Rust dependency licenses

### Triggers

```yaml
on:
  push:
    branches: [main, master, develop]
  pull_request:
    branches: [main, master, develop]
  schedule:
    - cron: '0 0 * * 0'  # Weekly on Sundays
```

## Release Workflow (`release.yml`)

### Jobs

1. **build-release** - Cross-platform release builds
   - Linux x64 binary
   - Windows x64 binary

2. **build-frontend** - Production frontend build

3. **create-release** - GitHub Release creation
   - Combines binaries and frontend
   - Creates platform-specific packages
   - Uploads to GitHub Releases

### Triggers

```yaml
on:
  push:
    tags:
      - 'v*'  # e.g., v1.0.0, v2.1.0-beta
  workflow_dispatch:
    inputs:
      version:
        description: 'Version tag (e.g., v1.0.0)'
        required: true
```

### Creating a Release

**Option 1: Tag-based (Recommended)**
```bash
git tag v1.0.0
git push origin v1.0.0
```

**Option 2: Manual Dispatch**
1. Go to Actions → Release workflow
2. Click "Run workflow"
3. Enter version (e.g., `v1.0.0`)
4. Click "Run workflow"

### Release Artifacts

| Artifact | Description |
|----------|-------------|
| `pagi-sola-web-linux-x64` | Linux binary |
| `pagi-sola-web-windows-x64.exe` | Windows binary |
| `frontend-dist.zip` | Frontend bundle |
| `pagi-twin-linux-x64.tar.gz` | Complete Linux package |
| `pagi-twin-windows-x64.zip` | Complete Windows package |

## Required Secrets

No additional secrets are required. The workflows use:

- `GITHUB_TOKEN` - Automatically provided by GitHub Actions

## Optional Configuration

### Environment Variables

Set these in repository settings → Secrets and variables → Actions:

| Variable | Purpose | Default |
|----------|---------|---------|
| `RUST_LOG` | Logging level | `info` |
| `CARGO_TERM_COLOR` | Terminal colors | `always` |

### Branch Protection

Recommended branch protection rules for `main`:

1. Require status checks to pass:
   - `rust-build (ubuntu-latest)`
   - `rust-build (windows-latest)`
   - `rust-test`
   - `rust-lint`
   - `frontend-build`

2. Require branches to be up to date

3. Require pull request reviews

## Local Testing

### Validate YAML Syntax

```bash
python -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml')); print('Valid')"
```

### Run Tests Locally

```bash
# Rust
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# Frontend
cd frontend_desktop
npm ci
npx tsc --noEmit
npm run build
```

### Using act (Local GitHub Actions)

Install [act](https://github.com/nektos/act) to run workflows locally:

```bash
# Run CI workflow
act push

# Run specific job
act -j rust-build

# Run with secrets
act -s GITHUB_TOKEN=your_token
```

## Troubleshooting

### Build Failures

1. **Rust compilation errors**
   - Check `cargo check --workspace` locally
   - Ensure Rust edition 2024 is supported

2. **Frontend build errors**
   - Run `npm ci` (not `npm install`)
   - Check Node.js version (requires 20+)

3. **Test failures**
   - Run `cargo test --workspace -- --nocapture` for verbose output

### Cache Issues

Clear caches in GitHub Actions:
1. Go to Actions → Caches
2. Delete relevant cache entries

Or add a cache key suffix:
```yaml
- uses: Swatinem/rust-cache@v2
  with:
    key: v2-${{ matrix.os }}  # Increment version to bust cache
```

## Workflow Status Badges

Add to README.md:

```markdown
[![CI](https://github.com/YOUR_ORG/pagi-twin-desktop/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_ORG/pagi-twin-desktop/actions/workflows/ci.yml)
[![Security](https://github.com/YOUR_ORG/pagi-twin-desktop/actions/workflows/security.yml/badge.svg)](https://github.com/YOUR_ORG/pagi-twin-desktop/actions/workflows/security.yml)
```
