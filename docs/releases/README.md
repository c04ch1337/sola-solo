# SOLA Releases

This directory contains release documentation and deployment guides.

## Current Release

**Version 1.0.0** - Production Ready

- [`RELEASE_NOTES.md`](RELEASE_NOTES.md) - **Start here** - Complete release notes
- [`RELEASE_QUICK_REFERENCE.md`](RELEASE_QUICK_REFERENCE.md) - Quick reference guide

## Release Documentation

### Version-Specific
- [`GITHUB_RELEASE_v1.0.0.md`](GITHUB_RELEASE_v1.0.0.md) - v1.0.0 release details
- [`GITHUB_RELEASE_COMPLETE.md`](GITHUB_RELEASE_COMPLETE.md) - Release completion status

### Release Process
- [`GITHUB_RELEASE_GUIDE.md`](GITHUB_RELEASE_GUIDE.md) - Creating releases

## Deployment

### Consumer Deployment
- [`CONSUMER_DEPLOYMENT_READY.md`](CONSUMER_DEPLOYMENT_READY.md) - Consumer-ready deployment guide

### Rebranding
- [`REBRAND_COMPLETE.md`](REBRAND_COMPLETE.md) - Rebranding completion (PAGI → SOLA)

## Installation

### Windows
Download `Sola AGI_1.0.0_x64_en-US.msi` from [releases](https://github.com/c04ch1337/pagi-twin-desktop/releases/latest)

```cmd
# Run installer
Sola AGI_1.0.0_x64_en-US.msi
```

### macOS
Download `Sola AGI_1.0.0_x64.dmg`

```bash
# Mount and install
open Sola\ AGI_1.0.0_x64.dmg
# Drag to Applications folder
```

### Linux
Download `Sola AGI_1.0.0_x86_64.AppImage`

```bash
# Make executable and run
chmod +x Sola\ AGI_1.0.0_x86_64.AppImage
./Sola\ AGI_1.0.0_x86_64.AppImage
```

## Release Scripts

Release scripts are located in [`scripts/build/`](../../scripts/build/):
- `release-v1.0.0.ps1` - PowerShell release script
- `release-v1.0.0.sh` - Bash release script

## Creating a Release

### Prerequisites
1. All tests passing
2. Version bumped in `Cargo.toml` and `package.json`
3. `RELEASE_NOTES.md` updated
4. Changelog updated

### Release Steps

1. **Build Release Artifacts**
```bash
# Windows
./scripts/build/release-v1.0.0.ps1

# Unix/Linux/macOS
./scripts/build/release-v1.0.0.sh
```

2. **Create GitHub Release**
- Tag: `v1.0.0`
- Title: `SOLA v1.0.0 - Production Release`
- Upload artifacts (MSI, DMG, AppImage)

3. **Update Documentation**
- Update `README.md` with new version
- Update `DOCUMENTATION_INDEX.md`

## Version History

### v1.0.0 (2026-01-22)
- Initial production release
- Core orchestration engine
- Multi-layered memory system
- Agent spawning and lifecycle management
- Browser automation
- Emotion detection and voice I/O
- Desktop app with Tauri
- GitHub integration

## Download Links

**Latest Release**: [GitHub Releases](https://github.com/c04ch1337/pagi-twin-desktop/releases/latest)

### Checksums
Checksums for release artifacts are included in each GitHub release.

## Support

- **Issues**: [GitHub Issues](https://github.com/c04ch1337/pagi-twin-desktop/issues)
- **Documentation**: [Main README](../../README.md)
- **Security**: [SECURITY.md](../../SECURITY.md)

---

*For build instructions, see [`docs/build-guides/`](../build-guides/)*
