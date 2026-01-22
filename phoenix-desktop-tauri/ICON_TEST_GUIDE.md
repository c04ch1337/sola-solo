# Sola AGI - Icon Testing Guide

## Test Checklist

### Pre-Build Tests

- [ ] **Icon source exists**: `src-tauri/icons/icon.png` (1024x1024)
- [ ] **Generated icons exist**:
  - [ ] `src-tauri/icons/32x32.png`
  - [ ] `src-tauri/icons/128x128.png`
  - [ ] `src-tauri/icons/128x128@2x.png`
  - [ ] `src-tauri/icons/icon.icns` (macOS)
  - [ ] `src-tauri/icons/icon.ico` (Windows)
- [ ] **tauri.conf.json** has icon paths configured
- [ ] **Version updated** to 1.0.1 in:
  - [ ] `tauri.conf.json`
  - [ ] `Cargo.toml`
  - [ ] `package.json`

### Build Tests

#### Windows

```powershell
# Build
cd phoenix-desktop-tauri
npm run build

# Check installer exists
Test-Path "src-tauri\target\release\bundle\msi\Sola AGI_1.0.1_x64_en-US.msi"

# Check installer size (should be ~50-100MB)
(Get-Item "src-tauri\target\release\bundle\msi\Sola AGI_1.0.1_x64_en-US.msi").Length / 1MB

# Extract icon from MSI (optional)
# Use 7-Zip or similar to inspect MSI contents
```

**Test Checklist:**
- [ ] MSI installer created
- [ ] MSI has proper icon (not default)
- [ ] MSI size is reasonable (~50-100MB)

#### macOS

```bash
# Build
cd phoenix-desktop-tauri
npm run build

# Check DMG exists
ls -lh src-tauri/target/release/bundle/dmg/Sola\ AGI_1.0.1_x64.dmg

# Mount DMG and check icon
open src-tauri/target/release/bundle/dmg/Sola\ AGI_1.0.1_x64.dmg
# Visually inspect app icon in Finder
```

**Test Checklist:**
- [ ] DMG created
- [ ] DMG has proper icon (not default)
- [ ] App bundle has icon when mounted
- [ ] Icon shows in Finder

#### Linux (AppImage)

```bash
# Build
cd phoenix-desktop-tauri
npm run build

# Check AppImage exists
ls -lh src-tauri/target/release/bundle/appimage/Sola\ AGI_1.0.1_x86_64.AppImage

# Make executable
chmod +x src-tauri/target/release/bundle/appimage/Sola\ AGI_1.0.1_x86_64.AppImage

# Extract icon (optional)
./src-tauri/target/release/bundle/appimage/Sola\ AGI_1.0.1_x86_64.AppImage --appimage-extract
ls squashfs-root/*.png
```

**Test Checklist:**
- [ ] AppImage created
- [ ] AppImage is executable
- [ ] Icon embedded in AppImage
- [ ] Icon shows in file manager

#### Linux (Debian)

```bash
# Build
cd phoenix-desktop-tauri
npm run build

# Check .deb exists
ls -lh src-tauri/target/release/bundle/deb/sola-agi_1.0.1_amd64.deb

# Extract and inspect (optional)
dpkg-deb -x sola-agi_1.0.1_amd64.deb extracted/
ls extracted/usr/share/icons/
ls extracted/usr/share/pixmaps/
```

**Test Checklist:**
- [ ] .deb package created
- [ ] Icons in correct locations
- [ ] Desktop entry has icon reference

### Installation Tests

#### Windows

1. **Install MSI**
   ```powershell
   # Run installer
   Start-Process "src-tauri\target\release\bundle\msi\Sola AGI_1.0.1_x64_en-US.msi"
   ```

2. **Check installed icon**
   - [ ] Start Menu shortcut has icon
   - [ ] Desktop shortcut has icon (if created)
   - [ ] Taskbar shows icon when running
   - [ ] System tray shows icon
   - [ ] Alt+Tab shows icon

3. **Launch app**
   ```powershell
   # From Start Menu or
   & "C:\Program Files\Sola AGI\Sola AGI.exe"
   ```

4. **Verify**
   - [ ] Window title bar has icon
   - [ ] Taskbar icon is correct
   - [ ] System tray icon is correct
   - [ ] Tray tooltip shows "Sola AGI - v1.0.1"

#### macOS

1. **Install DMG**
   ```bash
   open src-tauri/target/release/bundle/dmg/Sola\ AGI_1.0.1_x64.dmg
   # Drag to Applications
   ```

2. **Check installed icon**
   - [ ] Applications folder shows icon
   - [ ] Launchpad shows icon
   - [ ] Dock shows icon when running
   - [ ] Spotlight shows icon

3. **Launch app**
   ```bash
   open /Applications/Sola\ AGI.app
   ```

4. **Verify**
   - [ ] Dock icon is correct
   - [ ] Window has icon
   - [ ] Menu bar icon (if applicable)
   - [ ] Cmd+Tab shows icon

#### Linux

1. **Install AppImage**
   ```bash
   chmod +x Sola\ AGI_1.0.1_x86_64.AppImage
   ./Sola\ AGI_1.0.1_x86_64.AppImage
   ```

2. **Check icon**
   - [ ] File manager shows icon
   - [ ] Application menu shows icon (if integrated)
   - [ ] Taskbar/panel shows icon when running

3. **Install .deb (Debian/Ubuntu)**
   ```bash
   sudo dpkg -i sola-agi_1.0.1_amd64.deb
   ```

4. **Verify**
   - [ ] Application menu has icon
   - [ ] Desktop file has icon
   - [ ] Running app shows icon in panel

### Runtime Tests

#### System Tray

1. **Launch app**
2. **Check tray icon**
   - [ ] Icon appears in system tray
   - [ ] Icon is clear and recognizable
   - [ ] Hover shows tooltip: "Sola AGI - v1.0.1"

3. **Right-click tray icon**
   - [ ] Menu appears
   - [ ] "Show Window" works
   - [ ] "Hide Window" works
   - [ ] "Quit" works

4. **Double-click tray icon**
   - [ ] Window shows/focuses

#### Window Icon

1. **Main window**
   - [ ] Title bar has icon (Windows/Linux)
   - [ ] Dock has icon (macOS)
   - [ ] Taskbar has icon (Windows/Linux)

2. **Alt+Tab / Cmd+Tab**
   - [ ] Icon shows in app switcher
   - [ ] Icon is clear at small size

#### Notifications

1. **Trigger notification**
   ```javascript
   // In app, or via backend
   notificationService.sendNotification('Test', 'Icon test')
   ```

2. **Check notification**
   - [ ] Notification shows app icon
   - [ ] Icon is clear and recognizable

### Visual Quality Tests

#### Size Tests

Test icon at different sizes:

- [ ] **16x16** - Clear and recognizable
- [ ] **32x32** - Clear and recognizable
- [ ] **48x48** - Clear and recognizable
- [ ] **128x128** - Clear and recognizable
- [ ] **256x256** - Clear and recognizable
- [ ] **512x512** - Clear and recognizable
- [ ] **1024x1024** - Clear and recognizable

#### Theme Tests

- [ ] **Light theme** - Icon visible and clear
- [ ] **Dark theme** - Icon visible and clear
- [ ] **High contrast** - Icon visible and clear

#### Display Tests

- [ ] **Standard DPI** - Icon clear
- [ ] **High DPI (Retina/4K)** - Icon clear and sharp
- [ ] **Multiple monitors** - Icon consistent

### Troubleshooting

#### Icon not showing

**Windows:**
```powershell
# Clear icon cache
ie4uinit.exe -show
ie4uinit.exe -ClearIconCache

# Restart Explorer
Stop-Process -Name explorer -Force
```

**macOS:**
```bash
# Clear icon cache
sudo rm -rf /Library/Caches/com.apple.iconservices.store
killall Dock
killall Finder
```

**Linux:**
```bash
# Update icon cache
gtk-update-icon-cache -f -t ~/.local/share/icons/hicolor/
```

#### Icon looks wrong

1. Verify source icon quality (1024x1024)
2. Regenerate icons: `npm run icon`
3. Clean build: `cargo clean && npm run build`
4. Check icon paths in `tauri.conf.json`

#### Build includes old icon

1. Clean build artifacts: `cargo clean`
2. Delete old installers
3. Regenerate icons: `npm run icon`
4. Rebuild: `npm run build`

### Automated Tests (Optional)

Create `test-icons.sh`:

```bash
#!/bin/bash
# Automated icon tests

echo "🧪 Testing Sola AGI Icons"

# Check source icon
if [ ! -f "src-tauri/icons/icon.png" ]; then
    echo "❌ Source icon missing"
    exit 1
fi

# Check generated icons
for icon in 32x32.png 128x128.png 128x128@2x.png icon.icns icon.ico; do
    if [ ! -f "src-tauri/icons/$icon" ]; then
        echo "❌ Missing: $icon"
        exit 1
    fi
done

echo "✅ All icon files present"

# Check icon sizes
size=$(identify -format "%wx%h" src-tauri/icons/icon.png)
if [ "$size" != "1024x1024" ]; then
    echo "❌ Icon size incorrect: $size (expected 1024x1024)"
    exit 1
fi

echo "✅ Icon size correct"

# Check tauri.conf.json
if ! grep -q "icons/icon.ico" src-tauri/tauri.conf.json; then
    echo "❌ Icon not configured in tauri.conf.json"
    exit 1
fi

echo "✅ Icon configured in tauri.conf.json"

echo "✅ All icon tests passed!"
```

### Test Report Template

```markdown
# Icon Test Report - v1.0.1

**Date**: [Date]
**Tester**: [Name]
**Platform**: [Windows/macOS/Linux]

## Pre-Build
- [ ] Source icon exists (1024x1024)
- [ ] All generated icons present
- [ ] Configuration updated

## Build
- [ ] Installer created successfully
- [ ] Installer size: [Size]MB
- [ ] Build time: [Time]

## Installation
- [ ] Installer runs without errors
- [ ] App installs to correct location
- [ ] Shortcuts created with icons

## Runtime
- [ ] Window icon correct
- [ ] Tray icon correct
- [ ] Tray tooltip shows v1.0.1
- [ ] Notification icon correct

## Visual Quality
- [ ] Icon clear at all sizes
- [ ] Icon visible in light/dark themes
- [ ] Icon sharp on high DPI displays

## Issues Found
[List any issues]

## Overall Result
[ ] Pass
[ ] Fail (see issues)

**Notes**: [Additional notes]
```

---

**Ready to test?** Follow the checklist above! 🧪
