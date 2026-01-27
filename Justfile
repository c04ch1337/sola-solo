# Just task runner for local dev
#
# Note: Linter errors are false positives - this file uses Justfile syntax, not PowerShell.
# The '@' prefix is valid Justfile syntax that suppresses command echoing.

clean-ports:
    #!/usr/bin/env sh
    set -e
    echo "Cleaning ports 8888, 5173, 3000..."
    if [ "$OS" = "Windows_NT" ] || [ -n "$WINDIR" ]; then
        powershell -NoProfile -Command "foreach($p in @(8888,5173,3000)){ try { $c = Get-NetTCPConnection -LocalPort $p -State Listen -ErrorAction SilentlyContinue; foreach($x in $c){ Stop-Process -Id $x.OwningProcess -Force -ErrorAction SilentlyContinue } } catch {} }"
    else
        lsof -ti:8888,5173,3000 | xargs kill -9 2>/dev/null || true
    fi

