#!/bin/bash
# Sola AGI - Icon Generation Script
# Generates placeholder icon and all platform-specific formats

set -e

echo "🎨 Sola AGI Icon Generation"
echo "============================"
echo ""

# Check if we're in the right directory
if [ ! -f "package.json" ] || [ ! -d "src-tauri" ]; then
    echo "❌ Error: Must run from phoenix-desktop-tauri directory"
    exit 1
fi

# Check for Python and PIL
if ! command -v python3 &> /dev/null; then
    echo "❌ Error: python3 not found"
    echo "Install Python 3: https://www.python.org/downloads/"
    exit 1
fi

# Check if Pillow is installed
if ! python3 -c "import PIL" 2>/dev/null; then
    echo "⚠️  Pillow (PIL) not installed"
    echo "Installing Pillow..."
    pip3 install Pillow || {
        echo "❌ Failed to install Pillow"
        echo "Install manually: pip3 install Pillow"
        exit 1
    }
fi

# Create icons directory if it doesn't exist
mkdir -p src-tauri/icons

# Check for existing icon
if [ -f "src-tauri/icons/icon.png" ]; then
    echo "⚠️  Icon already exists: src-tauri/icons/icon.png"
    read -p "Overwrite? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Using existing icon..."
    else
        echo "🎨 Generating new placeholder icon..."
        python3 generate-placeholder-icon.py
    fi
else
    echo "🎨 Generating placeholder icon..."
    python3 generate-placeholder-icon.py
fi

# Verify icon exists
if [ ! -f "src-tauri/icons/icon.png" ]; then
    echo "❌ Error: Icon not created"
    exit 1
fi

echo ""
echo "📦 Generating platform-specific icons..."

# Check if cargo tauri is available
if command -v cargo-tauri &> /dev/null; then
    cargo tauri icon src-tauri/icons/icon.png
elif command -v npx &> /dev/null; then
    npx @tauri-apps/cli icon src-tauri/icons/icon.png
else
    echo "❌ Error: Neither cargo-tauri nor npx found"
    echo "Install Tauri CLI: cargo install tauri-cli"
    echo "Or ensure npx is available: npm install -g npx"
    exit 1
fi

# Verify generated icons
echo ""
echo "✅ Icon generation complete!"
echo ""
echo "Generated files:"
ls -lh src-tauri/icons/

echo ""
echo "📋 Next steps:"
echo "1. Review icons: open src-tauri/icons/"
echo "2. Rebuild app: npm run build"
echo "3. Test installer with new icons"
echo ""
echo "🕊️ Ready to build with new icons!"
