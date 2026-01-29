@echo off
setlocal enabledelayedexpansion

REM Start backend in a new window
start "sola-solo-backend" cmd /c "cd /d %~dp0backend && cargo run"

REM Start frontend in a new window
start "sola-solo-frontend" cmd /c "cd /d %~dp0frontend && npm run dev -- --host 127.0.0.1 --port 3000 --strictPort"

echo Sola Solo started:
echo   Backend  http://127.0.0.1:8888/health
echo   Frontend http://127.0.0.1:3000/
endlocal

