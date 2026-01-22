# Sola AGI v1.0.0 Release Script (PowerShell)
# Run this to create and push the release tag

$ErrorActionPreference = "Stop"

Write-Host "🕊️ Sola AGI v1.0.0 Release Script" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

# Check if we're in the right directory
if (-not (Test-Path "README.md") -or -not (Test-Path "phoenix-desktop-tauri")) {
    Write-Host "❌ Error: Must run from project root directory" -ForegroundColor Red
    exit 1
}

# Check if working tree is clean
$status = git status --porcelain
if ($status) {
    Write-Host "⚠️  Warning: Working tree has uncommitted changes" -ForegroundColor Yellow
    Write-Host ""
    git status --short
    Write-Host ""
    $continue = Read-Host "Continue anyway? (y/N)"
    if ($continue -ne "y" -and $continue -ne "Y") {
        exit 1
    }
}

# Check if tag already exists
$tagExists = git rev-parse v1.0.0 2>$null
if ($tagExists) {
    Write-Host "⚠️  Tag v1.0.0 already exists" -ForegroundColor Yellow
    Write-Host ""
    $recreate = Read-Host "Delete and recreate? (y/N)"
    if ($recreate -eq "y" -or $recreate -eq "Y") {
        Write-Host "🗑️  Deleting local tag..." -ForegroundColor Yellow
        git tag -d v1.0.0
        Write-Host "🗑️  Deleting remote tag..." -ForegroundColor Yellow
        try {
            git push origin :refs/tags/v1.0.0 2>$null
        } catch {
            Write-Host "   (remote tag didn't exist)" -ForegroundColor Gray
        }
    } else {
        exit 1
    }
}

# Create tag
Write-Host "🏷️  Creating tag v1.0.0..." -ForegroundColor Green
git tag -a v1.0.0 -m "Sola AGI v1.0.0 - First Stable Release"

# Show tag info
Write-Host ""
Write-Host "✅ Tag created successfully!" -ForegroundColor Green
Write-Host ""
git show v1.0.0 --quiet

# Push tag
Write-Host ""
$push = Read-Host "Push tag to origin? (Y/n)"
if ($push -ne "n" -and $push -ne "N") {
    Write-Host "📤 Pushing tag to origin..." -ForegroundColor Green
    git push origin v1.0.0
    Write-Host ""
    Write-Host "✅ Tag pushed successfully!" -ForegroundColor Green
} else {
    Write-Host "⏸️  Tag not pushed. Push manually with: git push origin v1.0.0" -ForegroundColor Yellow
}

# Next steps
Write-Host ""
Write-Host "🎉 Tag v1.0.0 is ready!" -ForegroundColor Cyan
Write-Host ""
Write-Host "📋 Next Steps:" -ForegroundColor Cyan
Write-Host "1. Go to: https://github.com/c04ch1337/phoenix-2.0/releases/new"
Write-Host "2. Select tag: v1.0.0"
Write-Host "3. Title: Sola AGI v1.0.0"
Write-Host "4. Description: Copy from RELEASE_QUICK_REFERENCE.md"
Write-Host "5. Upload 4 installer files from phoenix-desktop-tauri/src-tauri/target/release/bundle/"
Write-Host "6. Publish release"
Write-Host ""
Write-Host "📦 Asset Locations:" -ForegroundColor Cyan
Write-Host "   - msi\Sola AGI_1.0.0_x64_en-US.msi"
Write-Host "   - dmg\Sola AGI_1.0.0_x64.dmg"
Write-Host "   - appimage\Sola AGI_1.0.0_x86_64.AppImage"
Write-Host "   - deb\sola-agi_1.0.0_amd64.deb"
Write-Host ""
Write-Host "🕊️ Good luck with the release!" -ForegroundColor Cyan
