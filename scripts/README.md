# SOLA Scripts

This directory contains utility scripts for building, deploying, and managing SOLA.

## Directory Structure

```
scripts/
├── build/          # Build and release scripts
├── setup/          # Setup and configuration scripts
├── auto_setup_orchs.py
├── clone_orch.sh
├── launch_phoenix_web.cmd
├── launch_phoenix_web.sh
├── launch_phoenix.sh
└── refactor-prompts.js
```

## Build Scripts

Located in [`build/`](build/):

### Windows Build
- `build_windows.cmd` - Build SOLA on Windows
- `build_installer.cmd` - Create Windows MSI installer

### Release Scripts
- `release-v1.0.0.ps1` - Create v1.0.0 release (PowerShell)
- `release-v1.0.0.sh` - Create v1.0.0 release (Bash)

**Usage:**
```bash
# Windows
./scripts/build/build_windows.cmd

# Unix/Linux/macOS
./scripts/build/release-v1.0.0.sh
```

## Setup Scripts

Located in [`setup/`](setup/):

### Environment Setup
- `setup-env.ps1` - Configure environment variables
- `quick-setup.ps1` - Quick setup wizard

### Application Launch
- `start-backend.ps1` - Start backend server
- `launcher.cmd` - Launch SOLA application

**Usage:**
```powershell
# Setup environment
./scripts/setup/setup-env.ps1

# Quick setup
./scripts/setup/quick-setup.ps1

# Start backend
./scripts/setup/start-backend.ps1
```

## Utility Scripts

### Phoenix Launcher
Launch the Phoenix backend server:

**Windows:**
```cmd
scripts\launch_phoenix_web.cmd
```

**Unix/Linux/macOS:**
```bash
./scripts/launch_phoenix_web.sh
# or
./scripts/launch_phoenix.sh
```

### ORCH Management
- `auto_setup_orchs.py` - Automatically setup ORCH repositories
- `clone_orch.sh` - Clone ORCH repository

**Usage:**
```bash
# Auto-setup ORCHs
python scripts/auto_setup_orchs.py

# Clone specific ORCH
./scripts/clone_orch.sh <orch-name>
```

### Development Tools
- `refactor-prompts.js` - Refactor Cursor AI prompts

**Usage:**
```bash
node scripts/refactor-prompts.js
```

## Script Categories

### Build & Release
Build artifacts and create releases:
- Windows installer (MSI)
- macOS disk image (DMG)
- Linux AppImage
- GitHub releases

### Setup & Configuration
Initial setup and environment configuration:
- Environment variables
- API keys
- Database initialization
- Dependency installation

### Launch & Run
Start services and applications:
- Backend server
- Frontend development server
- Desktop application
- ORCH services

### Maintenance
Ongoing maintenance tasks:
- ORCH repository management
- Code refactoring
- Prompt updates

## Common Tasks

### First-Time Setup
```powershell
# 1. Setup environment
./scripts/setup/setup-env.ps1

# 2. Quick setup wizard
./scripts/setup/quick-setup.ps1

# 3. Start backend
./scripts/setup/start-backend.ps1
```

### Building for Release
```bash
# Windows
./scripts/build/build_windows.cmd
./scripts/build/build_installer.cmd

# Unix/Linux/macOS
./scripts/build/release-v1.0.0.sh
```

### Daily Development
```bash
# Start backend
./scripts/launch_phoenix_web.sh

# In another terminal, start frontend
cd frontend_desktop
npm run dev
```

## Script Requirements

### Prerequisites
- **PowerShell**: Windows scripts (`.ps1`, `.cmd`)
- **Bash**: Unix/Linux/macOS scripts (`.sh`)
- **Python 3.8+**: Python scripts (`.py`)
- **Node.js 18+**: JavaScript scripts (`.js`)

### Environment Variables
Scripts may require environment variables. See:
- [`.env.example`](../.env.example) - Environment template
- [`docs/setup-guides/ENV_SETUP_README.md`](../docs/setup-guides/ENV_SETUP_README.md) - Configuration guide

## Permissions

### Unix/Linux/macOS
Make scripts executable:
```bash
chmod +x scripts/*.sh
chmod +x scripts/build/*.sh
chmod +x scripts/setup/*.ps1
```

### Windows
PowerShell execution policy:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

## Troubleshooting

### Script Fails to Run

**PowerShell:**
```powershell
# Check execution policy
Get-ExecutionPolicy

# Set policy
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
```

**Bash:**
```bash
# Check permissions
ls -la scripts/

# Make executable
chmod +x scripts/script-name.sh
```

### Missing Dependencies

**Python:**
```bash
pip install -r requirements.txt
```

**Node.js:**
```bash
npm install
```

**Rust:**
```bash
rustup update stable
cargo build --workspace
```

## Contributing

When adding new scripts:
1. Place in appropriate subdirectory
2. Add documentation header
3. Update this README
4. Test on target platforms
5. Set appropriate permissions

### Script Template

**Bash:**
```bash
#!/bin/bash
# Script Name: example.sh
# Description: What this script does
# Usage: ./example.sh [arguments]

set -e  # Exit on error

# Script content here
```

**PowerShell:**
```powershell
# Script Name: example.ps1
# Description: What this script does
# Usage: .\example.ps1 [arguments]

$ErrorActionPreference = "Stop"

# Script content here
```

## Related Documentation

- **Setup**: [`docs/setup-guides/`](../docs/setup-guides/)
- **Build**: [`docs/build-guides/`](../docs/build-guides/)
- **Testing**: [`tests/`](../tests/)
- **Release**: [`docs/releases/`](../docs/releases/)

---

*For general documentation, see [`docs/`](../docs/)*
