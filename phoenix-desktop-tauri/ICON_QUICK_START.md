# Sola AGI - Icon Quick Start

## 🚀 Generate Icons in 3 Commands

### Windows
```powershell
cd phoenix-desktop-tauri
.\generate-icons.ps1
npm run build
```

### Linux/macOS
```bash
cd phoenix-desktop-tauri
chmod +x generate-icons.sh
./generate-icons.sh
npm run build
```

### Using npm (All Platforms)
```bash
cd phoenix-desktop-tauri
npm run icon:generate
npm run build
```

## ✅ Verify Icons

```bash
# Check generated files
ls -lh src-tauri/icons/

# Should see:
# - icon.png (1024x1024)
# - 32x32.png
# - 128x128.png
# - 128x128@2x.png
# - icon.icns (macOS)
# - icon.ico (Windows)
```

## 📦 Build Output

Installers with icons:
- Windows: `src-tauri/target/release/bundle/msi/Sola AGI_1.0.1_x64_en-US.msi`
- macOS: `src-tauri/target/release/bundle/dmg/Sola AGI_1.0.1_x64.dmg`
- Linux: `src-tauri/target/release/bundle/appimage/Sola AGI_1.0.1_x86_64.AppImage`
- Linux: `src-tauri/target/release/bundle/deb/sola-agi_1.0.1_amd64.deb`

## 🎨 Custom Icon

Replace placeholder with your own:

```bash
# 1. Create or get 1024x1024 PNG
cp /path/to/your/icon.png src-tauri/icons/icon.png

# 2. Generate all formats
npm run icon

# 3. Build
npm run build
```

## 🧪 Test

1. Install built package
2. Check icon in:
   - Start Menu/Applications
   - Desktop shortcut
   - System tray
   - Window title bar
   - Taskbar/Dock

3. Hover system tray → Should show "Sola AGI - v1.0.1"

## 📚 Full Documentation

- **Complete Guide**: [ICON_GENERATION.md](ICON_GENERATION.md)
- **Testing**: [ICON_TEST_GUIDE.md](ICON_TEST_GUIDE.md)
- **Build Instructions**: [BUILD.md](BUILD.md)

## 🐛 Troubleshooting

**"Python not found"**
```bash
# Install Python 3
pip install Pillow
```

**"cargo tauri icon not found"**
```bash
cargo install tauri-cli
# or use: npx @tauri-apps/cli icon src-tauri/icons/icon.png
```

**"Icon not showing"**
```bash
cargo clean
npm run icon
npm run build
```

---

**Ready?** Run the commands above! 🕊️
