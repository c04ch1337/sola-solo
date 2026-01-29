# Icon Generation - Quick Start Guide

## 🚀 Quick Commands

```bash
cd phoenix-desktop-tauri

# Generate placeholder icon + all formats
npm run icon:generate

# OR if icon.png already exists
npm run icon

# Verify icon exists
npm run icon:verify
```

## 📋 Step-by-Step

### 1. Generate Source Icon (if missing)

```bash
# Generate 1024x1024 placeholder icon
python generate-placeholder-icon.py
```

**Requirements:**
- Python 3 with Pillow: `pip install Pillow`
- Output: `src-tauri/icons/icon.png` (1024x1024 PNG)

### 2. Generate All Platform Formats

```bash
# Generate Windows .ico, macOS .icns, Linux .png
cargo tauri icon src-tauri/icons/icon.png
```

**Generated Files:**
- `icon.ico` (Windows)
- `icon.icns` (macOS)
- `32x32.png`, `128x128.png`, `128x128@2x.png` (Linux)

### 3. Verify Configuration

**Check `tauri.conf.json`:**
```json
{
  "bundle": {
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

**Check `main.rs` tray tooltip:**
```rust
.tooltip("Sola AGI - v1.0.1")
```

### 4. Test Icons

```bash
# Development mode
npm run dev
# → Verify window icon, tray icon, tooltip

# Production build
npm run build
# → Verify installers include icons
```

## ✅ Verification Checklist

- [ ] `src-tauri/icons/icon.png` exists (1024x1024)
- [ ] `src-tauri/icons/icon.ico` exists
- [ ] `src-tauri/icons/icon.icns` exists
- [ ] `src-tauri/icons/32x32.png` exists
- [ ] `src-tauri/icons/128x128.png` exists
- [ ] `src-tauri/icons/128x128@2x.png` exists
- [ ] `tauri.conf.json` has correct icon paths
- [ ] Tray tooltip is "Sola AGI - v1.0.1"

## 🐛 Troubleshooting

**Icon not found?**
```bash
npm run icon:generate
```

**Pillow not installed?**
```bash
pip install Pillow
```

**Tauri CLI not found?**
```bash
npm install -g @tauri-apps/cli
```

## 📚 Full Documentation

- **Icon Generation Guide:** `src-tauri/icons/ICON_GENERATION.md`
- **Build Guide:** `docs/BUILD.md` (Icon Generation section)
- **Phase Documentation:** `docs/phases/PHASE_32_ICON_GENERATION.md`

---

**Quick Reference:** Run `npm run icon:generate` to generate everything in one command! 🎨
