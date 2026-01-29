Implement backend ecosystem APIs in phoenix-web.

Task: [e.g. "Add api_ecosystem_import, api_ecosystem_list, api_ecosystem_build, api_ecosystem_start/stop + route 'ecosystem' commands in main.rs"]

Rules:
- Backend-only: extend main.rs routes + handlers
- Reuse existing ecosystem_manager crate if present
- Keep Tauri in mind for future frontend, but no UI changes here
- Docker optional (only if repo has Dockerfile)

First:
1. Duplication check (search for ecosystem in main.rs, routes)
2. If clean → generate:
   - phoenix-web/src/main.rs diff (new routes + handlers)
   - Any new lib.rs in ecosystem_manager
3. Integration: How frontend can call (e.g. POST /api/ecosystem/import {repo_url})
4. Tests: curl for each endpoint (e.g. curl -X POST /api/ecosystem/import -d '{"repo_url":"https://github.com/x"}')

Output only code + integration + tests.
