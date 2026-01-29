# GitHub Release Guide - Sola AGI v1.0.0

This guide walks you through creating a GitHub Release for Sola AGI v1.0.0.

## Prerequisites

1. **Build artifacts ready**
   - Run `npm run build` in `phoenix-desktop-tauri/`
   - Verify installers exist in `src-tauri/target/release/bundle/`

2. **Git tag ready**
   - Tag v1.0.0: `git tag -a v1.0.0 -m "Sola AGI v1.0.0"`
   - Push tag: `git push origin v1.0.0`

## Build Artifacts to Upload

After running `npm run build` in `phoenix-desktop-tauri/`, you'll find these files:

### Windows
- **Location**: `phoenix-desktop-tauri/src-tauri/target/release/bundle/msi/`
- **File**: `Sola AGI_1.0.0_x64_en-US.msi`
- **Size**: ~50-100MB (approximate)

### macOS
- **Location**: `phoenix-desktop-tauri/src-tauri/target/release/bundle/dmg/`
- **File**: `Sola AGI_1.0.0_x64.dmg`
- **Size**: ~50-100MB (approximate)

### Linux
- **AppImage**: `phoenix-desktop-tauri/src-tauri/target/release/bundle/appimage/Sola AGI_1.0.0_x86_64.AppImage`
- **Debian**: `phoenix-desktop-tauri/src-tauri/target/release/bundle/deb/sola-agi_1.0.0_amd64.deb`
- **Size**: ~50-100MB each (approximate)

## Creating the Release

### Step 1: Navigate to Releases

1. Go to your GitHub repository
2. Click **"Releases"** (right sidebar or `/releases` URL)
3. Click **"Draft a new release"** or **"Create a new release"**

### Step 2: Fill Release Details

**Tag version:**
- Select **"v1.0.0"** from dropdown (or create new tag)
- If creating new tag, use: `v1.0.0`

**Release title:**
```
Sola AGI v1.0.0
```

**Description:**
Copy the contents from `RELEASE_NOTES.md` and paste into the description field.

**Release type:**
- Select **"Release"** (not pre-release)

### Step 3: Upload Assets

1. Click **"Attach binaries"** or drag-and-drop area
2. Upload all installer files:
   - `Sola AGI_1.0.0_x64_en-US.msi` (Windows)
   - `Sola AGI_1.0.0_x64.dmg` (macOS)
   - `Sola AGI_1.0.0_x86_64.AppImage` (Linux)
   - `sola-agi_1.0.0_amd64.deb` (Linux Debian/Ubuntu)

**Note**: GitHub has a 2GB file size limit per asset. If files are too large, consider:
- Using GitHub LFS
- Compressing installers
- Hosting large files elsewhere

### Step 4: Publish

1. Review all details
2. Click **"Publish release"**

## Release Notes Template

If you prefer a shorter version, use this template:

```markdown
# Sola AGI v1.0.0

## 🕊️ First Stable Release

Sola AGI is your personal AI companion - emotionally intelligent, proactive, and voice-capable.

## ✨ Key Features

- 💬 **Chat-first interface** with streaming responses
- 🎤 **Voice interaction** (TTS/STT)
- 🌐 **Browser control** via Chrome CDP
- 💭 **Dreams panel** for emotional processing
- 🔔 **Proactive communication** with OS notifications
- 🎨 **Theme support** (dark/light mode)
- 💾 **Advanced memory system** (vaults, cortex, vector search)
- 🤖 **Ecosystem & agent management**

## 📦 Downloads

- **Windows**: [Sola AGI_1.0.0_x64_en-US.msi](link)
- **macOS**: [Sola AGI_1.0.0_x64.dmg](link)
- **Linux AppImage**: [Sola AGI_1.0.0_x86_64.AppImage](link)
- **Linux Debian**: [sola-agi_1.0.0_amd64.deb](link)

## 🚀 Quick Start

1. Download installer for your platform
2. Install and launch
3. Configure API key in Settings
4. Start chatting!

See [RELEASE_NOTES.md](RELEASE_NOTES.md) for full details.

## 📚 Documentation

- [Setup Guide](SETUP.md)
- [Build Instructions](phoenix-desktop-tauri/BUILD.md)
- [Architecture Docs](docs/)

---

**Sola AGI v1.0.0** - Your personal companion 🕊️
```

## Post-Release Checklist

- [ ] Verify all download links work
- [ ] Test installers on each platform
- [ ] Update README.md with release badge (already done)
- [ ] Announce on social media/community
- [ ] Monitor GitHub Issues for user feedback
- [ ] Prepare patch release if critical bugs found

## Troubleshooting

**"Tag already exists"**
- Delete tag: `git tag -d v1.0.0` (local) and `git push origin :refs/tags/v1.0.0` (remote)
- Or use a different version: `v1.0.1`

**"File too large"**
- Check file sizes (should be < 2GB each)
- Consider using GitHub LFS for large files
- Or host large files on external CDN

**"Build artifacts not found"**
- Run `npm run build` in `phoenix-desktop-tauri/`
- Check `src-tauri/target/release/bundle/` directory
- Verify build completed successfully

## Next Steps After Release

1. **Monitor feedback** - Watch GitHub Issues and Discussions
2. **Plan v1.0.1** - Address critical bugs if found
3. **Documentation** - Update docs based on user questions
4. **Community** - Engage with users, gather feature requests

---

**Ready to release?** Follow the steps above and publish Sola AGI v1.0.0! 🕊️
