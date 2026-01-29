# Start Phoenix Backend with Environment Variables
# This script loads .env file values and starts the backend

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "   Starting Phoenix Backend" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Load .env file
if (Test-Path ".env") {
    Write-Host "Loading .env file..." -ForegroundColor Yellow
    
    Get-Content .env | ForEach-Object {
        $line = $_.Trim()
        # Skip comments and empty lines
        if ($line -and -not $line.StartsWith("#")) {
            if ($line -match "^([^=]+)=(.*)$") {
                $key = $matches[1].Trim()
                $value = $matches[2].Trim()
                
                # Remove quotes if present
                if ($value.StartsWith('"') -and $value.EndsWith('"')) {
                    $value = $value.Substring(1, $value.Length - 2)
                }
                if ($value.StartsWith("'") -and $value.EndsWith("'")) {
                    $value = $value.Substring(1, $value.Length - 2)
                }
                
                # Set environment variable
                [System.Environment]::SetEnvironmentVariable($key, $value, [System.EnvironmentVariableTarget]::Process)
                
                # Show loaded keys (mask sensitive values)
                if ($key -like "*KEY*" -or $key -like "*SECRET*" -or $key -like "*TOKEN*" -or $key -like "*PAT*") {
                    if ($value.Length -gt 0) {
                        $preview = $value.Substring(0, [Math]::Min(15, $value.Length))
                        Write-Host "  $key = $preview... " -ForegroundColor Green
                    }
                } else {
                    Write-Host "  $key = $value" -ForegroundColor Gray
                }
            }
        }
    }
    Write-Host ""
    Write-Host ".env loaded successfully!" -ForegroundColor Green
    Write-Host ""
} else {
    Write-Host "WARNING: .env file not found!" -ForegroundColor Red
    Write-Host ""
}

# Start backend
Write-Host "Starting Phoenix backend..." -ForegroundColor Cyan
cd phoenix-web
cargo run --release
