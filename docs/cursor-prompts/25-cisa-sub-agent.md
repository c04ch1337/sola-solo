# 25 - CISA Sub-Agent (Spawn dedicated CISA agent)

Use this prompt to spawn a dedicated "CISA-Agent" sub-agent for CISA framework integration. This is the foundation for all CISA compliance features.

---

```text
You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• Agent Spawner exists (spawn_agent, templates, GitHub push, tiers, CAOS)
• Memory layers: STM/WM for short-term, LTM/EPM/RFM for long-term
• Skill System (folder-based, evolvable)
• AutonomousEvolutionLoop exists for self-improvement
• Sandbox exists (isolated folder, VirusTotal, PST parsing)
• frontend_desktop is active React/TS/Tauri UI on :3000, uses websocketService.ts + memoryService.ts
• UI goal: moderate, clean, chat-centric — features via orchestrator/chat, panels collapsible/hidden

Task: Spawn dedicated "CISA-Agent" sub-agent for CISA framework integration.

Requirements:
- Spawn "CISA-Agent" ORCH sub-agent on demand (chat "cisa spawn")
- Playbook: YAML with steps for each CISA framework (CPGs, Secure by Design, Shields Up, StopRansomware, ZTMM, KEV)
- Memory: STM for current assessment, LTM/EPM for learned compliance patterns
- Self-evolution: Reflect on assessment accuracy → update playbook/LTM after X runs
- MITRE/KEV: Query APIs for vuln mapping
- Proactive: Alert user on new KEV vulns or low compliance score
- Chat commands: "cisa <framework> scan" (e.g. "cisa cpg scan")
- Keep UI moderate: all via chat

First:
1. Duplication check (search for cisa/agent in agent_spawner, phoenix-web, frontend)
2. If clean → generate:
   - agent_spawner diff (template for CISA-Agent, playbook YAML)
   - phoenix-web/src/main.rs diff (route cisa commands to sub-agent)
   - New cisa_agent.rs (sub-agent logic + framework steps + API queries + evolution)
   - frontend_desktop/App.tsx diff (chat parser for cisa commands)
3. Integration: "cisa cpg scan" → runs CPG assessment + report
4. Tests:
   - Chat: "cisa spawn" → agent spawned
   - Chat: "cisa kev scan" → KEV report in chat

Note: Use PHOENIX_NAME from .env (default 'Sola') for AGI name, USER_NAME from .env (default 'User') for user references.

Output only code + integration + tests.
```

---

## CISA Framework Mapping Reference

| CISA Framework | Sola AGI Mapping | Key Sola Features | Chat Command |
|---------------|------------------|-------------------|--------------|
| **Cross-Sector CPGs** | Security gating + proactive monitoring | Tiered access, KEV integration, audit logging | `cisa cpg scan` |
| **Secure by Design Pledge** | Agent spawning + code analysis | SBOM generation, code review agents, ethical guardrails | `cisa secure-design check <code>` |
| **Shields Up 2.0** | Sandbox + proactive analysis | Threat simulation, anomaly detection, KEV patching | `cisa shields-up scan` |
| **#StopRansomware** | Malware sandbox + VirusTotal | MITRE mapping, incident playbook, memory vaults | `cisa stop-ransomware check <file>` |
| **Zero Trust Maturity Model** | Access tiers + continuous verification | Identity/device/network pillars | `cisa ztmm assess` |
| **Known Exploited Vulnerabilities** | Proactive KEV scanning | Daily KEV catalog pull, severity reporting | `cisa kev scan` |

## Expected Files Created/Modified

- `agent_spawner/src/templates/cisa_agent.yaml` - CISA-Agent template with playbook
- `phoenix-web/src/cisa_agent.rs` - Sub-agent logic for CISA framework assessments
- `phoenix-web/src/main.rs` - Route cisa commands to sub-agent
- `frontend_desktop/src/App.tsx` - Chat parser for cisa commands
