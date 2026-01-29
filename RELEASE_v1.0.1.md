# Sola AGI v1.0.1 – Tauri v2 Migration & Final Polish

**Sola AGI** is your personal, emotionally-aware AI companion — built for privacy, voice, and real-world interaction.

## What's New in v1.0.1

- Migrated to **Tauri v2** (modern APIs, improved security, better tray/notifications)
- Full feature polish: icons, help system, onboarding, analytics
- All 30 phases complete
- Production-ready installers

## Key Features

- **Chat-first interface** with token-by-token streaming
- **Voice interaction** (TTS/STT) with modulation
- **Local browser control** (navigate, scrape, screenshot, click, type)
- **Dreams** (lucid, shared, healing, recordings) with emotional depth
- **Proactive outreach** — Sola reaches out based on curiosity/time/emotion
- **Collapsible panels** (Memory, Dreams, Agents, Ecosystem, WebGuard)
- **Tauri native desktop** with tray icon + OS notifications
- **Agent spawning** & ecosystem repo management via chat
- **Theme support** (dark/light), onboarding, opt-in analytics

## Installation

Download the installer for your platform:

- **Windows**: `Sola AGI_1.0.1_x64_en-US.msi`
- **macOS**: `Sola AGI_1.0.1_x64.dmg`
- **Linux**: `Sola AGI_1.0.1_x86_64.AppImage` or `.deb`

## Quick Start

1. Install and run
2. Type anything in chat — Sola responds
3. Try: `voice on`, `system browser navigate https://duckduckgo.com`, `show dreams`, `notify test`, `help`

## Technical Changes (v1.0.1)

### Tauri v2 Migration

- Replaced deprecated v1 APIs with modern v2 equivalents
- `SystemTray` → `TrayIconBuilder` with `.setup()` callback
- `CustomMenuItem` → `MenuItem::with_id()` with app context
- `get_window()` → `get_webview_window()`
- Async commands with `State` now return `Result` types
- Improved security and performance

### Features Preserved

✅ Tray icon with show/hide/quit menu  
✅ Window constraints (800x600 min, 2560x1440 max)  
✅ 13 Tauri commands (recording, emotion, notifications)  
✅ Multi-modal recording integration  
✅ All v1.0.0 functionality maintained

## Known Issues / Coming Soon

- Icons placeholder (generate your own 1024x1024 PNG)
- Code signing not configured (self-signed builds)
- Future: voice wake words, ecosystem UI logs, agent panel polish

## Documentation

- [Tauri v2 Migration Guide](phoenix-desktop-tauri/TAURI_V2_MIGRATION.md)
- [Build Instructions](docs/build-guides/BUILD_INSTRUCTIONS.md)
- [Phase 30 Build Notes](PHASE_30_BUILD_NOTES.md)

## Support

Thank you for trying Sola AGI. Type `help` in chat for commands. Feedback welcome!

🕊️❤️

---

**Full Changelog**: https://github.com/c04ch1337/pagi-twin-desktop/compare/v1.0.0...v1.0.1
