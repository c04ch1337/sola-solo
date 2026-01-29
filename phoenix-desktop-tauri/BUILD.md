# Sola AGI - Building & Running

This guide explains how to build and package Sola AGI as a desktop application using Tauri.

## Prerequisites

### Required Tools

- **Rust** (latest stable) - [Install via rustup](https://rustup.rs/)
- **Node.js** (v18 or later) - [Download](https://nodejs.org/)
- **npm** or **yarn** - Comes with Node.js

### Platform-Specific Requirements

#### Windows
- Microsoft Visual C++ Build Tools
- Windows SDK

#### macOS
- Xcode Command Line Tools: `xcode-select --install`

#### Linux
- `libwebkit2gtk-4.0-dev`
- `build-essential`
- `curl`
- `wget`
- `libssl-dev`
- `libgtk-3-dev`
- `libayatana-appindicator3-dev`
- `librsvg2-dev`

Install on Ubuntu/Debian:
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

## Development Setup

### 1. Install Dependencies

```bash
# Install frontend dependencies
cd frontend_desktop
npm install

# Install Tauri CLI (globally or use npx)
npm install -g @tauri-apps/cli
# OR use npx: npx @tauri-apps/cli dev
```

### 2. Build Frontend

```bash
cd frontend_desktop
npm run build
```

This creates the `dist` folder that Tauri will bundle.

### 3. Run in Development Mode

From the `phoenix-desktop-tauri` directory:

```bash
# Using Tauri CLI
tauri dev

# OR using npx
npx @tauri-apps/cli dev
```

This will:
- Start the backend server (if configured)
- Launch the Tauri app with hot-reload
- Open the app window

## Building for Production

### Build Release

From the `phoenix-desktop-tauri` directory:

```bash
# Build release
tauri build

# OR using npx
npx @tauri-apps/cli build
```

This will:
1. Build the frontend (`frontend_desktop/dist`)
2. Compile the Rust backend
3. Bundle everything into platform-specific installers

### Output Files

After building, you'll find installers in:

- **Windows**: `src-tauri/target/release/bundle/msi/Sola AGI_1.0.0_x64_en-US.msi`
- **macOS**: `src-tauri/target/release/bundle/dmg/Sola AGI_1.0.0_x64.dmg`
- **Linux**: `src-tauri/target/release/bundle/appimage/Sola AGI_1.0.0_amd64.AppImage`
- **Linux (Debian)**: `src-tauri/target/release/bundle/deb/sola-agi_1.0.0_amd64.deb`

## Configuration

### Icons

Icons should be placed in `src-tauri/icons/`. See [ICON_GENERATION.md](ICON_GENERATION.md) for detailed instructions.

**Quick Start:**

```bash
# Option 1: Generate placeholder icon automatically
npm run icon:generate

# Option 2: Use your own 1024x1024 PNG
cp /path/to/your/icon.png src-tauri/icons/icon.png
npm run icon

# Option 3: Manual generation
python generate-placeholder-icon.py
cargo tauri icon src-tauri/icons/icon.png
```

This generates all required formats:
- `icon.ico` (Windows, multiple sizes)
- `icon.icns` (macOS, multiple sizes)
- `32x32.png`, `128x128.png`, `128x128@2x.png` (Linux)

**Required files:**
- Source: `src-tauri/icons/icon.png` (1024x1024 PNG)
- Generated: All platform-specific formats

If icons are missing, Tauri will use default placeholder icons.

### Environment Variables

The app uses environment variables from `.env` file (backend) and `import.meta.env` (frontend).

Backend `.env` should be in the project root:
```env
PHOENIX_NAME=Sola
USER_NAME=User
OPENROUTER_API_KEY=your_key_here
# ... other config
```

## Troubleshooting

### Build Errors

**"Cannot find frontend dist folder"**
- Make sure you've run `npm run build` in `frontend_desktop` first
- Check that `frontendDist` in `tauri.conf.json` points to the correct path

**"Rust compilation errors"**
- Run `cargo clean` and try again
- Ensure all dependencies are up to date: `cargo update`

**"Icon not found"**
- Icons are optional for development
- For release builds, add icons to `src-tauri/icons/`

### Runtime Issues

**"Backend not connecting"**
- Ensure the backend server is running on the configured port
- Check `VITE_PHOENIX_API_URL` in frontend environment

**"Window not showing"**
- Check system tray - the app may be minimized there
- Try right-clicking the tray icon and selecting "Show Window"

## Package Scripts

Add these to `package.json` in `phoenix-desktop-tauri` (if using npm):

```json
{
  "scripts": {
    "tauri": "tauri",
    "dev": "tauri dev",
    "build": "tauri build"
  }
}
```

Then you can use:
- `npm run dev` - Development mode
- `npm run build` - Production build

## Distribution

### Code Signing (Optional but Recommended)

For production releases:

**Windows:**
- Obtain a code signing certificate
- Set `certificateThumbprint` in `tauri.conf.json` → `bundle.windows`

**macOS:**
- Apple Developer account required
- Set `signingIdentity` in `tauri.conf.json` → `bundle.macOS`

**Linux:**
- No code signing required
- Consider GPG signing for .deb packages

### Auto-Updates (Future)

Tauri supports auto-updates via:
- Tauri's update server
- Custom update mechanism
- See [Tauri Updater](https://tauri.app/v1/guides/distribution/updater) docs

## CI/CD

### GitHub Actions Example

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        platform: [windows-latest, macos-latest, ubuntu-latest]
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: 18
      - uses: dtolnay/rust-toolchain@stable
      - name: Install dependencies
        run: |
          cd frontend_desktop
          npm install
      - name: Build frontend
        run: |
          cd frontend_desktop
          npm run build
      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        with:
          projectPath: phoenix-desktop-tauri
          tagName: ${{ github.ref_name }}
          releaseName: 'Sola AGI v__VERSION__'
          releaseBody: 'See the assets to download this version and install.'
          releaseDraft: true
          prerelease: false
```

## Version Management

Update version in:
1. `tauri.conf.json` → `version`
2. `Cargo.toml` → `version`
3. `package.json` (if exists) → `version`

Use semantic versioning: `MAJOR.MINOR.PATCH`

## Next Steps

- Add icons to `src-tauri/icons/`
- Configure code signing for production
- Set up CI/CD for automated releases
- Test installers on target platforms
- Create release notes and changelog
