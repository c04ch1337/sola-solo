# Sola AGI - Icon Generation Guide

## Overview

This guide explains how to generate and integrate proper app icons for Sola AGI v1.0.1.

## Quick Start

```bash
# 1. Create or place your 1024x1024 PNG icon
cp /path/to/your/icon.png src-tauri/icons/icon.png

# 2. Generate all icon sizes
cargo tauri icon src-tauri/icons/icon.png

# 3. Rebuild with new icons
npm run build
```

## Step-by-Step Instructions

### Step 1: Create Base Icon (1024x1024 PNG)

You need a **1024x1024 PNG** image as the source. This will be used to generate all platform-specific icons.

#### Option A: Use Existing Logo/Design

If you have a logo or design:
1. Export as PNG at 1024x1024 resolution
2. Ensure transparent background (recommended)
3. Save as `icon.png`

#### Option B: Create Placeholder Icon

If you don't have an icon yet, create a simple placeholder:

**Using ImageMagick:**
```bash
# Install ImageMagick first (https://imagemagick.org/)

# Create a simple flame/circle icon
convert -size 1024x1024 xc:transparent \
  -fill "#FF6B35" \
  -draw "circle 512,512 512,100" \
  -fill "#FFD23F" \
  -draw "circle 512,400 512,200" \
  src-tauri/icons/icon.png
```

**Using Python (PIL):**
```python
from PIL import Image, ImageDraw

# Create 1024x1024 transparent image
img = Image.new('RGBA', (1024, 1024), (0, 0, 0, 0))
draw = ImageDraw.Draw(img)

# Draw flame-like circles
draw.ellipse([100, 100, 924, 924], fill='#FF6B35')  # Outer circle (orange)
draw.ellipse([200, 200, 824, 824], fill='#FFD23F')  # Inner circle (yellow)
draw.ellipse([350, 350, 674, 674], fill='#FFFFFF')  # Center (white)

# Save
img.save('src-tauri/icons/icon.png')
```

**Using Online Tools:**
- [Canva](https://www.canva.com/) - Free design tool
- [Figma](https://www.figma.com/) - Design and export
- [IconKitchen](https://icon.kitchen/) - Icon generator

#### Option C: Use Text-Based Icon

Simple text-based icon with "S" for Sola:

```python
from PIL import Image, ImageDraw, ImageFont

img = Image.new('RGBA', (1024, 1024), (0, 0, 0, 0))
draw = ImageDraw.Draw(img)

# Background circle
draw.ellipse([50, 50, 974, 974], fill='#6B46C1')  # Purple

# Try to use a font, or use default
try:
    font = ImageFont.truetype("arial.ttf", 600)
except:
    font = ImageFont.load_default()

# Draw "S" in center
draw.text((512, 512), "S", fill='white', font=font, anchor='mm')

img.save('src-tauri/icons/icon.png')
```

### Step 2: Generate All Icon Formats

Once you have `icon.png`, generate all required formats:

```bash
# From phoenix-desktop-tauri directory
cargo tauri icon src-tauri/icons/icon.png
```

This creates:
- `icon.ico` (Windows, multiple sizes)
- `icon.icns` (macOS, multiple sizes)
- `32x32.png` (Linux)
- `128x128.png` (Linux)
- `128x128@2x.png` (Linux retina)
- `icon.png` (Base 1024x1024)

**Output:**
```
✔ Successfully created icons:
  - src-tauri/icons/32x32.png
  - src-tauri/icons/128x128.png
  - src-tauri/icons/128x128@2x.png
  - src-tauri/icons/icon.icns
  - src-tauri/icons/icon.ico
  - src-tauri/icons/icon.png
```

### Step 3: Update Tauri Configuration

The `tauri.conf.json` will automatically use icons from `src-tauri/icons/` if they exist.

**Verify configuration** (already set in tauri.conf.json):
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

### Step 4: Update System Tray Icon (Optional)

For system tray, add tray-specific configuration:

**In `src-tauri/src/main.rs`:**
```rust
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show Window"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"));

    let tray = SystemTray::new()
        .with_menu(tray_menu)
        .with_tooltip("Sola AGI - v1.0.1");

    tauri::Builder::default()
        .system_tray(tray)
        // ... rest of config
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 5: Rebuild Application

```bash
# Development build (test icons)
npm run dev

# Production build (with icons)
npm run build
```

### Step 6: Verify Icons

After building:

**Windows:**
```powershell
# Check installer has icon
Get-ItemProperty "src-tauri\target\release\bundle\msi\Sola AGI_1.0.1_x64_en-US.msi"
```

**macOS:**
```bash
# Check DMG has icon
ls -lh src-tauri/target/release/bundle/dmg/
```

**Linux:**
```bash
# Check AppImage has icon
file src-tauri/target/release/bundle/appimage/Sola\ AGI_1.0.1_x86_64.AppImage
```

## Icon Requirements by Platform

### Windows (.ico)
- Multiple sizes embedded: 16x16, 32x32, 48x48, 256x256
- Format: ICO with transparency
- Generated automatically by `cargo tauri icon`

### macOS (.icns)
- Multiple sizes embedded: 16x16 to 1024x1024
- Format: ICNS
- Generated automatically by `cargo tauri icon`

### Linux (.png)
- Sizes: 32x32, 128x128, 128x128@2x
- Format: PNG with transparency
- Generated automatically by `cargo tauri icon`

## Troubleshooting

### "cargo tauri icon command not found"

Install Tauri CLI:
```bash
cargo install tauri-cli
```

Or use via npx:
```bash
npx @tauri-apps/cli icon src-tauri/icons/icon.png
```

### "Icon not showing in installer"

1. Verify icons exist in `src-tauri/icons/`
2. Clean build: `cargo clean`
3. Rebuild: `npm run build`
4. Check `tauri.conf.json` has correct icon paths

### "Icon looks pixelated"

- Ensure source PNG is high quality (1024x1024)
- Use vector graphics if possible
- Avoid scaling up small images

### "Transparent background not working"

- Save PNG with alpha channel
- Use RGBA color mode
- Test with image viewer that supports transparency

## Icon Design Tips

### Best Practices
1. **Simple & Recognizable** - Clear at small sizes (16x16)
2. **Transparent Background** - Looks good on any desktop theme
3. **High Contrast** - Visible in both light/dark modes
4. **Centered** - Leave padding around edges
5. **Consistent Style** - Match your brand

### Recommended Tools
- **Figma** - Professional design (free)
- **Inkscape** - Vector graphics (free)
- **GIMP** - Raster editing (free)
- **ImageMagick** - Command-line processing
- **Icon Kitchen** - Quick icon generation

### Color Suggestions for Sola AGI
- **Primary**: Purple/Violet (#6B46C1) - Wisdom, spirituality
- **Accent**: Orange/Flame (#FF6B35) - Energy, warmth
- **Highlight**: Yellow/Gold (#FFD23F) - Light, hope
- **Background**: Transparent or white

## Quick Icon Templates

### Minimalist "S" Icon
```
Purple circle background
White "S" letter (bold, centered)
Transparent padding
```

### Flame Icon
```
Orange outer flame shape
Yellow inner flame
White core/highlight
Transparent background
```

### Geometric Icon
```
Purple hexagon
Orange triangle inside
Yellow dot center
Transparent background
```

## Automation Script

Save as `generate-icons.sh`:

```bash
#!/bin/bash
# Generate Sola AGI icons

echo "🎨 Generating Sola AGI icons..."

# Check if source icon exists
if [ ! -f "src-tauri/icons/icon.png" ]; then
    echo "❌ Error: src-tauri/icons/icon.png not found"
    echo "Please create a 1024x1024 PNG icon first"
    exit 1
fi

# Generate all formats
echo "📦 Running cargo tauri icon..."
cargo tauri icon src-tauri/icons/icon.png

if [ $? -eq 0 ]; then
    echo "✅ Icons generated successfully!"
    echo ""
    echo "Generated files:"
    ls -lh src-tauri/icons/
    echo ""
    echo "Next steps:"
    echo "1. Review icons: ls src-tauri/icons/"
    echo "2. Rebuild app: npm run build"
    echo "3. Test installer with new icons"
else
    echo "❌ Icon generation failed"
    exit 1
fi
```

Make executable:
```bash
chmod +x generate-icons.sh
./generate-icons.sh
```

## Version Update Checklist

When updating to v1.0.1:

- [ ] Update `tauri.conf.json` → `version: "1.0.1"`
- [ ] Update `Cargo.toml` → `version = "1.0.1"`
- [ ] Update `package.json` → `version: "1.0.1"`
- [ ] Generate new icons (if changed)
- [ ] Update tray tooltip: "Sola AGI - v1.0.1"
- [ ] Rebuild: `npm run build`
- [ ] Test installers on all platforms

## Resources

- [Tauri Icon Documentation](https://tauri.app/v1/guides/features/icons)
- [Icon Design Guidelines](https://developer.apple.com/design/human-interface-guidelines/app-icons)
- [Windows Icon Guidelines](https://docs.microsoft.com/en-us/windows/apps/design/style/iconography/app-icon-design)
- [Linux Icon Theme Spec](https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html)

---

**Ready to generate icons?** Follow Step 1-6 above! 🎨
