# Sola AGI - Icon Generation Script (PowerShell)
# Generates placeholder icon and all platform-specific formats

$ErrorActionPreference = "Stop"

Write-Host "🎨 Sola AGI Icon Generation" -ForegroundColor Cyan
Write-Host "============================" -ForegroundColor Cyan
Write-Host ""

# Check if we're in the right directory
if (-not (Test-Path "package.json") -or -not (Test-Path "src-tauri")) {
    Write-Host "❌ Error: Must run from phoenix-desktop-tauri directory" -ForegroundColor Red
    exit 1
}

# Check for Python
$pythonCmd = $null
foreach ($cmd in @("python", "python3", "py")) {
    if (Get-Command $cmd -ErrorAction SilentlyContinue) {
        $pythonCmd = $cmd
        break
    }
}

if (-not $pythonCmd) {
    Write-Host "❌ Error: Python not found" -ForegroundColor Red
    Write-Host "Install Python 3: https://www.python.org/downloads/"
    exit 1
}

# Check if Pillow is installed
$pillowInstalled = & $pythonCmd -c "import PIL" 2>$null
if ($LASTEXITCODE -ne 0) {
    Write-Host "⚠️  Pillow (PIL) not installed" -ForegroundColor Yellow
    Write-Host "Installing Pillow..."
    & $pythonCmd -m pip install Pillow
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to install Pillow" -ForegroundColor Red
        Write-Host "Install manually: pip install Pillow"
        exit 1
    }
}

# Create icons directory if it doesn't exist
if (-not (Test-Path "src-tauri\icons")) {
    New-Item -ItemType Directory -Path "src-tauri\icons" | Out-Null
}

# Check for existing icon
if (Test-Path "src-tauri\icons\icon.png") {
    Write-Host "⚠️  Icon already exists: src-tauri\icons\icon.png" -ForegroundColor Yellow
    $overwrite = Read-Host "Overwrite? (y/N)"
    if ($overwrite -ne "y" -and $overwrite -ne "Y") {
        Write-Host "Using existing icon..."
    } else {
        Write-Host "🎨 Generating new placeholder icon..." -ForegroundColor Green
        & $pythonCmd generate-placeholder-icon.py
    }
} else {
    Write-Host "🎨 Generating placeholder icon..." -ForegroundColor Green
    & $pythonCmd generate-placeholder-icon.py
}

# Verify icon exists
if (-not (Test-Path "src-tauri\icons\icon.png")) {
    Write-Host "❌ Error: Icon not created" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "📦 Generating platform-specific icons..." -ForegroundColor Green

# Check if cargo tauri is available
$tauriCmd = $null
if (Get-Command "cargo-tauri" -ErrorAction SilentlyContinue) {
    $tauriCmd = "cargo-tauri"
} elseif (Get-Command "npx" -ErrorAction SilentlyContinue) {
    $tauriCmd = "npx"
}

if ($tauriCmd -eq "cargo-tauri") {
    cargo tauri icon src-tauri\icons\icon.png
} elseif ($tauriCmd -eq "npx") {
    npx @tauri-apps/cli icon src-tauri\icons\icon.png
} else {
    Write-Host "❌ Error: Neither cargo-tauri nor npx found" -ForegroundColor Red
    Write-Host "Install Tauri CLI: cargo install tauri-cli"
    Write-Host "Or ensure npx is available: npm install -g npx"
    exit 1
}

# Verify generated icons
Write-Host ""
Write-Host "✅ Icon generation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Generated files:" -ForegroundColor Cyan
Get-ChildItem src-tauri\icons\ | Format-Table Name, Length, LastWriteTime

Write-Host ""
Write-Host "📋 Next steps:" -ForegroundColor Cyan
Write-Host "1. Review icons: explorer src-tauri\icons\"
Write-Host "2. Rebuild app: npm run build"
Write-Host "3. Test installer with new icons"
Write-Host ""
Write-Host "🕊️ Ready to build with new icons!" -ForegroundColor Cyan
