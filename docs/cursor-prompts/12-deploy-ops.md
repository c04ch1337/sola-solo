Implement deploy/ops features in backend + frontend_desktop.

Task: [e.g. "Add production deploy script, CI/CD for release, performance monitoring, port config"]

Rules:
- Backend: Add health endpoints, configurable bind addresses
- Frontend: Tauri bundle script + tray for monitoring
- Docker optional: add Dockerfile only if requested
- Keep UI moderate: ops status in hidden debug panel or chat ("status ops")

First:
1. Duplication check (search for deploy/health/monitor in main.rs, tauri.conf.json)
2. If clean → generate:
   - scripts/deploy.sh (bare-metal + optional Docker)
   - .github/workflows/release.yml (CI/CD for builds)
   - phoenix-web/main.rs diff (configurable port/bind, /health)
   - phoenix-desktop-tauri/tauri.conf.json diff (tray + notifications)
3. Integration: App.tsx debug panel for ops status
4. Tests: Run deploy script → check running on new port; query /health

Output only code + integration + tests.
