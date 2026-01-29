# Sola AGI Icon Generation Script (Windows PowerShell)
# Converts SVG to PNG and generates all platform icons

param(
    [string]$SvgPath = "src-tauri/icons/icon.svg",
    [string]$OutputDir = "src-tauri/icons"
)

Write-Host "Sola AGI Icon Generator" -ForegroundColor Cyan
Write-Host "=========================" -ForegroundColor Cyan

# Check for required tools
$inkscapePath = Get-Command inkscape -ErrorAction SilentlyContinue
$magickPath = Get-Command magick -ErrorAction SilentlyContinue

if (-not $inkscapePath -and -not $magickPath) {
    Write-Host "Error: Neither Inkscape nor ImageMagick found." -ForegroundColor Red
    Write-Host ""
    Write-Host "Please install one of the following:" -ForegroundColor Yellow
    Write-Host "  - Inkscape: https://inkscape.org/release/" -ForegroundColor White
    Write-Host "  - ImageMagick: https://imagemagick.org/script/download.php" -ForegroundColor White
    Write-Host ""
    Write-Host "Or use an online tool:" -ForegroundColor Yellow
    Write-Host "  1. Open icon.svg in a browser" -ForegroundColor White
    Write-Host "  2. Take a screenshot or use https://svgtopng.com/" -ForegroundColor White
    Write-Host "  3. Save as icon.png (1024x1024)" -ForegroundColor White
    Write-Host "  4. Run: cargo tauri icon src-tauri/icons/icon.png" -ForegroundColor White
    exit 1
}

# Convert SVG to PNG
$pngPath = Join-Path $OutputDir "icon.png"

if ($inkscapePath) {
    Write-Host "Converting SVG to PNG using Inkscape..." -ForegroundColor Yellow
    & inkscape $SvgPath --export-type=png --export-filename=$pngPath --export-width=1024 --export-height=1024
}
elseif ($magickPath) {
    Write-Host "Converting SVG to PNG using ImageMagick..." -ForegroundColor Yellow
    & magick convert $SvgPath -resize 1024x1024 $pngPath
}

if (-not (Test-Path $pngPath)) {
    Write-Host "Error: Failed to create icon.png" -ForegroundColor Red
    exit 1
}

Write-Host "Created icon.png (1024x1024)" -ForegroundColor Green

# Check for Tauri CLI
$tauriPath = Get-Command cargo-tauri -ErrorAction SilentlyContinue
if (-not $tauriPath) {
    Write-Host "Installing Tauri CLI..." -ForegroundColor Yellow
    cargo install tauri-cli
}

# Generate platform icons
Write-Host "Generating platform icons..." -ForegroundColor Yellow
cargo tauri icon $pngPath

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "Icon generation complete!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Generated files:" -ForegroundColor Cyan
    Get-ChildItem $OutputDir -Filter "*.png" | ForEach-Object { Write-Host "  $($_.Name)" -ForegroundColor White }
    Get-ChildItem $OutputDir -Filter "*.ico" | ForEach-Object { Write-Host "  $($_.Name)" -ForegroundColor White }
    Get-ChildItem $OutputDir -Filter "*.icns" | ForEach-Object { Write-Host "  $($_.Name)" -ForegroundColor White }
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Yellow
    Write-Host "  1. Review the generated icons" -ForegroundColor White
    Write-Host "  2. Rebuild Tauri: npm run build" -ForegroundColor White
    Write-Host "  3. Test the installer" -ForegroundColor White
}
else {
    Write-Host "Error: Icon generation failed" -ForegroundColor Red
    Write-Host ""
    Write-Host "Manual fallback:" -ForegroundColor Yellow
    Write-Host "  1. Open icon.svg in a browser or editor" -ForegroundColor White
    Write-Host "  2. Export as 1024x1024 PNG" -ForegroundColor White
    Write-Host "  3. Run: cargo tauri icon src-tauri/icons/icon.png" -ForegroundColor White
}
