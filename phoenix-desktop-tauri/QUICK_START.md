# Sola AGI - Quick Start Guide

## Development

```bash
# 1. Install dependencies
cd frontend_desktop
npm install

# 2. Build frontend
npm run build

# 3. Run Tauri dev mode
cd ../phoenix-desktop-tauri
npm install
npm run dev
```

## Production Build

```bash
# From phoenix-desktop-tauri directory
npm run build
```

This will:
- Build the frontend automatically (via `prebuild` script)
- Compile the Rust backend
- Create platform-specific installers

## Output

Installers will be in:
- Windows: `src-tauri/target/release/bundle/msi/`
- macOS: `src-tauri/target/release/bundle/dmg/`
- Linux: `src-tauri/target/release/bundle/appimage/` or `deb/`

## Icons

Icons are optional for development. For release builds, add icons to `src-tauri/icons/`:
- `icon.ico` (Windows)
- `icon.icns` (macOS)
- `32x32.png`, `128x128.png`, `128x128@2x.png` (Linux)

See `BUILD.md` for detailed instructions.
