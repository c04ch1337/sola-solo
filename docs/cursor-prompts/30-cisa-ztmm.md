# 30 - CISA Zero Trust Maturity Model (ZTMM) Integration

Use this prompt to integrate CISA's Zero Trust Maturity Model into the CISA-Agent. This framework provides a roadmap for implementing zero trust architecture across identity, devices, networks, applications, and data pillars.

---

```text
You are the Orchestrator — Sola's central coordination intelligence inside Cursor IDE.

Core facts already complete in backend (NEVER re-implement):
• CISA-Agent exists for framework integration (from prompt 25)
• Agent Spawner exists (spawn_agent, templates, GitHub push, tiers, CAOS)
• Memory layers: STM/WM for short-term, LTM/EPM/RFM for long-term
• Tiered access control exists (Tier 0-2 with consent gating)
• Proactive agents exist for continuous monitoring
• frontend_desktop is active React/TS/Tauri UI on :3000, uses websocketService.ts + memoryService.ts
• UI goal: moderate, clean, chat-centric — features via orchestrator/chat, panels collapsible/hidden

Task: Integrate CISA Zero Trust Maturity Model (ZTMM) into CISA-Agent.

Requirements:
- CISA-Agent handles "cisa ztmm assess" → evaluate zero trust maturity across 5 pillars
- Map to Sola features:
  1. Identity Pillar → Tiered gating with consent (Tier 0-2)
  2. Devices Pillar → Device/browser control and trust verification
  3. Networks Pillar → Network monitoring via proactive agents
  4. Applications Pillar → Agent spawner with CAOS optimization
  5. Data Pillar → Memory vault encryption + access controls
- Assess maturity levels: Traditional → Initial → Advanced → Optimal
- Cross-cutting capabilities: Visibility, Analytics, Automation, Governance
- Generate maturity roadmap with improvement recommendations
- Store assessment history in EPM for progress tracking

First:
1. Duplication check (search for ztmm/zero-trust in cisa_agent.rs)
2. If clean → generate:
   - cisa_agent.rs diff (ZTMM assessment + maturity scoring)
   - New ztmm_assessor.rs (5 pillars + cross-cutting + maturity levels)
   - Maturity roadmap generator
3. Integration: "cisa ztmm assess" → full maturity report + roadmap
4. Tests:
   - Chat: "cisa ztmm assess" → report with maturity levels
   - Verify pillar-to-feature mapping works

Note: Use PHOENIX_NAME from .env (default 'Sola') for AGI name, USER_NAME from .env (default 'User') for user references.

Output only code + integration + tests.
```

---

## ZTMM Pillars Mapping

| ZTMM Pillar | Sola Feature | Maturity Assessment |
|-------------|--------------|---------------------|
| 1. Identity | Tiered access (Tier 0-2) | MFA, SSO, continuous auth |
| 2. Devices | Browser/device control | Device trust, health checks |
| 3. Networks | Proactive agents | Micro-segmentation, monitoring |
| 4. Applications | Agent spawner + CAOS | Secure access, least privilege |
| 5. Data | Memory vaults | Encryption, classification, DLP |

## Maturity Levels

| Level | Description | Characteristics |
|-------|-------------|-----------------|
| **Traditional** | Perimeter-based | Static policies, manual processes |
| **Initial** | Starting ZT journey | Some automation, basic visibility |
| **Advanced** | Significant progress | Cross-pillar integration, analytics |
| **Optimal** | Full ZT implementation | Continuous verification, AI-driven |

## Cross-Cutting Capabilities

| Capability | Description | Sola Mapping |
|------------|-------------|--------------|
| Visibility | Asset and activity awareness | Audit logging, proactive monitoring |
| Analytics | Threat detection and response | Anomaly detection, MITRE mapping |
| Automation | Policy enforcement | Agent spawner, playbooks |
| Governance | Policy management | Tiered access, consent gating |

## Expected Output Format

```
🔐 CISA Zero Trust Maturity Model Assessment
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Overall Maturity: ADVANCED (3.2/4.0)

📊 Pillar Assessment:
┌────────────────┬───────────┬─────────────────────────────┐
│ Pillar         │ Maturity  │ Score                       │
├────────────────┼───────────┼─────────────────────────────┤
│ 🆔 Identity    │ OPTIMAL   │ ████████████████████ 4.0    │
│ 💻 Devices     │ ADVANCED  │ ███████████████░░░░░ 3.5    │
│ 🌐 Networks    │ ADVANCED  │ ██████████████░░░░░░ 3.0    │
│ 📱 Applications│ ADVANCED  │ ██████████████░░░░░░ 3.0    │
│ 📁 Data        │ INITIAL   │ ██████████░░░░░░░░░░ 2.5    │
└────────────────┴───────────┴─────────────────────────────┘

🔄 Cross-Cutting Capabilities:
- Visibility: ADVANCED (Audit logging active, 95% coverage)
- Analytics: ADVANCED (Anomaly detection enabled)
- Automation: ADVANCED (Agent spawner + playbooks active)
- Governance: OPTIMAL (Tiered access + consent gating)

📋 Pillar Details:

🆔 IDENTITY (OPTIMAL - 4.0)
✅ MFA enforced on all accounts
✅ SSO integration active
✅ Continuous authentication enabled
✅ Risk-based access policies
✅ Privileged access management

💻 DEVICES (ADVANCED - 3.5)
✅ Device inventory complete
✅ Health checks enabled
⚠️ BYOD policy needs refinement
✅ Endpoint detection active

🌐 NETWORKS (ADVANCED - 3.0)
✅ Network segmentation implemented
⚠️ Micro-segmentation partial (70%)
✅ Encrypted communications
✅ Network monitoring active

📱 APPLICATIONS (ADVANCED - 3.0)
✅ Application inventory complete
✅ Secure access controls
⚠️ Legacy app integration pending
✅ API security enabled

📁 DATA (INITIAL - 2.5)
✅ Data encryption at rest
⚠️ Data classification incomplete
❌ DLP not fully implemented
✅ Access logging enabled

🗺️ Maturity Roadmap:

Phase 1 (0-3 months):
1. Complete data classification for all memory vaults
2. Implement DLP policies for sensitive data
3. Refine BYOD policy for device pillar

Phase 2 (3-6 months):
1. Extend micro-segmentation to 100%
2. Integrate legacy applications with ZT controls
3. Implement data loss prevention

Phase 3 (6-12 months):
1. Achieve OPTIMAL maturity across all pillars
2. Implement AI-driven continuous verification
3. Full automation of policy enforcement

📈 Progress Since Last Assessment:
- Identity: ↑ 0.5 (from ADVANCED to OPTIMAL)
- Devices: → 0.0 (maintained ADVANCED)
- Networks: ↑ 0.5 (from INITIAL to ADVANCED)
- Applications: ↑ 0.5 (from INITIAL to ADVANCED)
- Data: → 0.0 (maintained INITIAL)
```

## ZTMM Assessment Criteria

### Identity Pillar
- **Traditional**: Password-only, no MFA
- **Initial**: MFA on some accounts
- **Advanced**: MFA everywhere, SSO, risk-based
- **Optimal**: Continuous auth, passwordless, AI-driven

### Devices Pillar
- **Traditional**: No device management
- **Initial**: Basic inventory, some controls
- **Advanced**: Health checks, EDR, compliance
- **Optimal**: Real-time trust, auto-remediation

### Networks Pillar
- **Traditional**: Perimeter firewall only
- **Initial**: Basic segmentation
- **Advanced**: Micro-segmentation, encrypted
- **Optimal**: Software-defined, AI-monitored

### Applications Pillar
- **Traditional**: Network-based access
- **Initial**: Some app-level controls
- **Advanced**: Least privilege, API security
- **Optimal**: Continuous verification, CASB

### Data Pillar
- **Traditional**: Perimeter protection only
- **Initial**: Basic encryption
- **Advanced**: Classification, access controls
- **Optimal**: DLP, rights management, AI-driven
