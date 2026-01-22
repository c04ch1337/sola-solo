#!/bin/bash
# Sola AGI v1.0.0 Release Script
# Run this to create and push the release tag

set -e

echo "🕊️ Sola AGI v1.0.0 Release Script"
echo "=================================="
echo ""

# Check if we're in the right directory
if [ ! -f "README.md" ] || [ ! -d "phoenix-desktop-tauri" ]; then
    echo "❌ Error: Must run from project root directory"
    exit 1
fi

# Check if working tree is clean
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  Warning: Working tree has uncommitted changes"
    echo ""
    git status --short
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if tag already exists
if git rev-parse v1.0.0 >/dev/null 2>&1; then
    echo "⚠️  Tag v1.0.0 already exists"
    echo ""
    read -p "Delete and recreate? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "🗑️  Deleting local tag..."
        git tag -d v1.0.0
        echo "🗑️  Deleting remote tag..."
        git push origin :refs/tags/v1.0.0 2>/dev/null || echo "   (remote tag didn't exist)"
    else
        exit 1
    fi
fi

# Create tag
echo "🏷️  Creating tag v1.0.0..."
git tag -a v1.0.0 -m "Sola AGI v1.0.0 - First Stable Release"

# Show tag info
echo ""
echo "✅ Tag created successfully!"
echo ""
git show v1.0.0 --quiet

# Push tag
echo ""
read -p "Push tag to origin? (Y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Nn]$ ]]; then
    echo "📤 Pushing tag to origin..."
    git push origin v1.0.0
    echo ""
    echo "✅ Tag pushed successfully!"
else
    echo "⏸️  Tag not pushed. Push manually with: git push origin v1.0.0"
fi

# Next steps
echo ""
echo "🎉 Tag v1.0.0 is ready!"
echo ""
echo "📋 Next Steps:"
echo "1. Go to: https://github.com/c04ch1337/phoenix-2.0/releases/new"
echo "2. Select tag: v1.0.0"
echo "3. Title: Sola AGI v1.0.0"
echo "4. Description: Copy from RELEASE_QUICK_REFERENCE.md"
echo "5. Upload 4 installer files from phoenix-desktop-tauri/src-tauri/target/release/bundle/"
echo "6. Publish release"
echo ""
echo "📦 Asset Locations:"
echo "   - msi/Sola AGI_1.0.0_x64_en-US.msi"
echo "   - dmg/Sola AGI_1.0.0_x64.dmg"
echo "   - appimage/Sola AGI_1.0.0_x86_64.AppImage"
echo "   - deb/sola-agi_1.0.0_amd64.deb"
echo ""
echo "🕊️ Good luck with the release!"
