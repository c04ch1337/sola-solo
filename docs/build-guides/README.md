# SOLA Build Guides

This directory contains documentation for building SOLA from source.

## Build Documentation

### General Build Instructions
- [`BUILD_INSTRUCTIONS.md`](BUILD_INSTRUCTIONS.md) - **Start here** - Complete build guide for all platforms

### Platform-Specific Guides
- [`BUILD_WINDOWS.md`](BUILD_WINDOWS.md) - Windows-specific build instructions

### Backend
- [`BACKEND_STARTING.md`](BACKEND_STARTING.md) - Starting the backend server

## Build Scripts

Build scripts are located in [`scripts/build/`](../../scripts/build/):

### Windows
- `build_windows.cmd` - Windows build script
- `build_installer.cmd` - Create Windows installer (MSI)

### Release Scripts
- `release-v1.0.0.ps1` - Release build (PowerShell)
- `release-v1.0.0.sh` - Release build (Bash)

## Quick Build

### Prerequisites
- Rust 1.70+ ([rustup.rs](https://rustup.rs))
- Node.js 18+ (for frontend)
- Git

### Build Commands

**Backend:**
```bash
# Build all workspace crates
cargo build --workspace --release

# Run backend server
cargo run --bin phoenix-web --release
```

**Frontend (Tauri Desktop):**
```bash
cd phoenix-desktop-tauri
npm install
npm run tauri build
```

**Frontend (Web):**
```bash
cd frontend_desktop
npm install
npm run build
```

## Desktop Applications

### Phoenix Desktop (Tauri)
See [`phoenix-desktop-tauri/`](../../phoenix-desktop-tauri/) for:
- Desktop app build instructions
- Icon generation
- Platform-specific packaging

Documentation:
- [`phoenix-desktop-tauri/BUILD.md`](../../phoenix-desktop-tauri/BUILD.md)
- [`phoenix-desktop-tauri/QUICK_START.md`](../../phoenix-desktop-tauri/QUICK_START.md)

## Installer Creation

### Windows Installer
```cmd
# Build MSI installer
scripts\build\build_installer.cmd
```

Uses Inno Setup - see [`installer.iss`](../../installer.iss)

### macOS DMG
```bash
cd phoenix-desktop-tauri
npm run tauri build -- --target dmg
```

### Linux AppImage
```bash
cd phoenix-desktop-tauri
npm run tauri build -- --target appimage
```

## Build Troubleshooting

### Common Issues

**Rust version:**
```bash
rustup update stable
```

**Node modules:**
```bash
rm -rf node_modules package-lock.json
npm install
```

**Cargo cache:**
```bash
cargo clean
cargo build --workspace --release
```

## Next Steps

After building:
1. **Test**: See [`docs/testing/`](../testing/)
2. **Deploy**: See [`docs/releases/`](../releases/)

---

*For setup and configuration, see [`docs/setup-guides/`](../setup-guides/)*
