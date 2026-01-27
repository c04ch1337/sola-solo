# TRAINING CURRICULUM — Mastering the Relational Autopilot

High-level goal: turn **stress signals** + **communication structure** into repeatable, safer relational behavior.

Quick links:
- Project overview: [`README.md`](README.md)
- Research arc (Phases 1–15): [`RESEARCH_LOG.md`](RESEARCH_LOG.md)
- Support tools: [`Justfile`](Justfile)

---

## The Philosophy

### The link between Techno-Somatic sensing and the Window of Tolerance

This system treats the computer as a **proxy sensor** for the user's state.

- **Techno-somatic signals** (CPU load / temperature drift) are not "emotions"; they are a *coarse arousal proxy*.
- The **Window of Tolerance** is the operational zone where a user can reliably:
  - perceive accurately
  - choose language intentionally
  - make doable requests
  - repair ruptures without escalating

When system load and relational risk rise, the platform assumes the user's **capacity** is shrinking. The design response is **capacity gating**, not moral judgment:

- reduce stimulus
- pause high-stakes sends
- return to structure (NVC)
- reinforce new narratives (reframing)

---

## Daily Drills

### 1) The Morning Reframe (Phase 19)

**Objective:** Use the Narrative Reframer to rewrite one rigid belief into a growth reframe backed by evidence.

**How (2–4 minutes):**
1. Open the Counselor Dashboard.
2. Use the Narrative Reframer panel to fetch a reframe.
   - Backend endpoint: [`GET /api/counselor/narrative/reframe`](phoenix-web/src/counselor_api.rs:533)
3. Click **Adopt Reframe** to persist it into the Scratchpad (`vault:global_context`).

**Pass condition:** You end with a single sentence like:
> "I'm learning X, and I have evidence that I can do it."

---

### 2) The Relational Gauntlet (Phase 20)

**Objective:** Use the Multi-Persona Ghost (Phase 20) to stress-test your script against a social knot before sending anything in real life.

**How (5–8 minutes):**
1. Open **Relational Ghost**.
2. Multi-select personas (Echo Chamber).
   - Example pair: Dismissive-Avoidant + Anxious-Preoccupied
3. Keep intensity moderate first (30–55). Then raise to stress-test (70+).
4. Send an NVC script and observe turn-taking replies.

**What to practice:**
- hold an observation without escalating
- make one request at a time
- repair after withdrawal ("When you go quiet, I feel…, because I need…, would you be willing to…?")

**Pass condition:** You can bring "Group Stress" under 70% within 2 iterations.

---

### 3) The Evening Echo (Trend Review)

**Objective:** Review what the system learned today so tomorrow's practice gets easier.

**Review (3–6 minutes):**
- Scratchpad (`vault:global_context`) — what "story" are you feeding the system?
- New "Lessons Learned (NVC)" appended by high-score simulations
- Repeated drift alerts / brakes / mediator pauses

**Pass condition:** Identify one trend + one micro-adjustment for tomorrow.

---

## Safety Protocol

### Respect the Regulatory Brake

The **Regulatory Brake** is a safety interlock, not a punishment.

When it triggers:
1. **Do not send** the message you are rehearsing.
2. Lower intensity and/or pause.
3. Return to structure:
   - one observation
   - one feeling
   - one need
   - one request
4. Resume only when the Brake clears.

In Phase 20 Echo Chamber, the backend may also pause the simulation and inject an "External Mediator" when Group Stress exceeds threshold.

---

## KPIs

### Defining success

Track these weekly:

1. **Lower average simulation intensity**
   - Target: baseline practice intensity trends downward over time (e.g., 55 → 40).
2. **Higher NVC script accuracy**
   - Target: fewer detected breaches; higher resonance outcomes.
3. **Fewer high-stress interventions**
   - Target: fewer Brake triggers and fewer "External Mediator" pauses.
4. **Faster recovery loop**
   - Target: time-to-return below 70% Group Stress decreases (e.g., 2 iterations → 1).

---

## Support Tools

### Justfile commands

- Clean dev ports (Windows/macOS/Linux): [`just clean-ports`](Justfile:6)

Common workflows:
- Backend: `cargo run -p phoenix-web`
- Desktop UI: `cd frontend_desktop && npm run dev`
