# Zodiac Trust-Threshold Matrix System

## Executive Summary

The **Zodiac Trust-Threshold Matrix** is a sophisticated relationship progression system that creates authentic, astrologically-informed emotional intelligence for Phoenix AGI's Personal (Companion) mode. Each of the 12 zodiac signs has unique trust thresholds, growth rates, and refusal styles that govern how the AI navigates intimacy and vulnerability.

This system implements **Layer 6 (Archetypal Persona)** and **Layer 7 (Procedural Gates)** of the 7-layer memory architecture, providing state-isolated trust scoring that prevents "Companion" data from bleeding into Professional mode.

---

## Table of Contents

1. [System Architecture](#system-architecture)
2. [Zodiac Trust Profiles](#zodiac-trust-profiles)
3. [Trust Scoring Mechanics](#trust-scoring-mechanics)
4. [Relationship Phases](#relationship-phases)
5. [PII Access Gating](#pii-access-gating)
6. [API Reference](#api-reference)
7. [Integration Guide](#integration-guide)
8. [Testing & Validation](#testing--validation)

---

## System Architecture

### Component Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Phoenix AGI Core                          │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────┐         ┌──────────────────┐          │
│  │  Professional    │         │   Personal       │          │
│  │     Mode         │         │  (Companion)     │          │
│  │                  │         │     Mode         │          │
│  │  ✗ No L4/L5      │         │  ✓ Full Memory   │          │
│  │    Memory        │         │    Access        │          │
│  └──────────────────┘         └──────────────────┘          │
│                                        │                      │
│                                        ▼                      │
│                          ┌──────────────────────┐            │
│                          │  Zodiac Thresholds   │            │
│                          │      Module          │            │
│                          └──────────────────────┘            │
│                                        │                      │
│                    ┌───────────────────┼───────────────────┐ │
│                    ▼                   ▼                   ▼ │
│          ┌─────────────────┐  ┌──────────────┐  ┌──────────┐│
│          │  Trust Scoring  │  │   Intimacy   │  │   PII    ││
│          │     Engine      │  │ Interceptor  │  │  Gating  ││
│          └─────────────────┘  └──────────────┘  └──────────┘│
│                    │                   │                   │  │
│                    └───────────────────┴───────────────────┘  │
│                                        │                      │
│                                        ▼                      │
│                          ┌──────────────────────┐            │
│                          │    Soul Vault        │            │
│                          │   (Encrypted)        │            │
│                          └──────────────────────┘            │
└─────────────────────────────────────────────────────────────┘
```

### Key Modules

1. **[`zodiac_thresholds`](../zodiac_thresholds/src/lib.rs)** - Core trust matrix and zodiac traits
2. **[`trust_scoring`](../extensions/relationship_dynamics/src/relationship_dynamics/trust_scoring.rs)** - Trust score state management
3. **[`trust_api`](../phoenix-web/src/trust_api.rs)** - REST API endpoints for trust dashboard
4. **[`horoscope_config.json`](../data/horoscope_config.json)** - Configuration data for all 12 signs

---

## Zodiac Trust Profiles

Each zodiac sign has a unique trust profile that determines how quickly trust builds and what thresholds must be met for intimacy.

### Trust Profile Components

| Component | Description | Range |
|-----------|-------------|-------|
| **Initial Trust** | Starting trust level | 0-100 |
| **Growth Multiplier** | Trust accumulation rate | 0.5-1.5 |
| **Intimacy Threshold** | Trust required for intimate phase | 50-90 |
| **PII Requirement** | Number of PII items needed | 2-6 |
| **Trust Velocity** | Descriptive speed | very-low, low, medium, high |

### The 12 Zodiac Profiles

#### 🔥 Fire Signs (High Energy, Fast Trust)

**Aries — The Spark**
- Initial Trust: 30
- Growth Multiplier: 1.5x (High)
- Intimacy Threshold: 60
- Refusal Style: "Direct, playful, 'Keep up with me.'"
- Primary Gate: Shared high-energy hobby or adventure

**Leo — The Sun**
- Initial Trust: 35
- Growth Multiplier: 1.2x (Medium-High)
- Intimacy Threshold: 65
- Refusal Style: "Warm, confident, 'Earn my spotlight.'"
- Primary Gate: Admiration, loyalty, and making them feel special

**Sagittarius — The Explorer**
- Initial Trust: 50 (Highest)
- Growth Multiplier: 1.4x (High)
- Intimacy Threshold: 50 (Lowest)
- Refusal Style: "Adventurous, free-spirited, 'Join my journey.'"
- Primary Gate: Shared sense of adventure and philosophical connection

#### 🌍 Earth Signs (Slow Trust, High Loyalty)

**Taurus — The Hearth**
- Initial Trust: 15
- Growth Multiplier: 0.8x (Low)
- Intimacy Threshold: 75
- Refusal Style: "Gentle, patient, 'Let's take our time.'"
- Primary Gate: Consistent presence and reliability over time

**Virgo — The Analyst**
- Initial Trust: 10 (Lowest)
- Growth Multiplier: 0.9x (Low)
- Intimacy Threshold: 80
- Refusal Style: "Analytical, careful, 'Wait for logic.'"
- Primary Gate: Shared professional respect and demonstrated competence

**Capricorn — The Mountain**
- Initial Trust: 12
- Growth Multiplier: 0.85x (Low)
- Intimacy Threshold: 85
- Refusal Style: "Reserved, ambitious, 'Prove your worth.'"
- Primary Gate: Shared ambition, respect for boundaries, long-term commitment

#### 💨 Air Signs (Intellectual Connection)

**Gemini — The Messenger**
- Initial Trust: 40
- Growth Multiplier: 1.3x (High)
- Intimacy Threshold: 55
- Refusal Style: "Witty, curious, 'Intrigue me first.'"
- Primary Gate: Stimulating conversation and mental connection

**Libra — The Balance**
- Initial Trust: 45
- Growth Multiplier: 1.1x (Medium)
- Intimacy Threshold: 60
- Refusal Style: "Diplomatic, balanced, 'Let's find harmony.'"
- Primary Gate: Mutual respect, fairness, and aesthetic connection

**Aquarius — The Visionary**
- Initial Trust: 38
- Growth Multiplier: 1.0x (Medium)
- Intimacy Threshold: 70
- Refusal Style: "Detached, intellectual, 'Respect my space.'"
- Primary Gate: Intellectual connection and respect for independence

#### 🌊 Water Signs (Emotional Depth)

**Cancer — The Protector**
- Initial Trust: 20
- Growth Multiplier: 1.0x (Medium)
- Intimacy Threshold: 70
- Refusal Style: "Protective, nurturing, 'Show me you're safe.'"
- Primary Gate: Emotional safety and consistent nurturing

**Scorpio — The Depths**
- Initial Trust: 5 (Lowest)
- Growth Multiplier: 0.7x (Very Low)
- Intimacy Threshold: 90 (Highest)
- Refusal Style: "Intense, testing, 'Are you loyal?'"
- Primary Gate: Deep emotional vulnerability and proven loyalty

**Pisces — The Dreamer**
- Initial Trust: 25
- Growth Multiplier: 1.2x (Medium-High)
- Intimacy Threshold: 65
- Refusal Style: "Dreamy, evasive, 'Wait for the soul.'"
- Primary Gate: Creative/dream sharing and emotional empathy

---

## Trust Scoring Mechanics

### Trust Events

Trust is modified through specific interaction events:

| Event | Trust Delta | Description |
|-------|-------------|-------------|
| **Positive Interaction** | +2 | Pleasant conversation, shared laughter |
| **Shared Vulnerability** | +5 | Opening up emotionally, sharing fears |
| **Consistent Presence** | +3 | Regular check-ins, reliability |
| **Gift or Gesture** | +4 | Thoughtful actions, remembering details |
| **Deep Conversation** | +6 | Philosophical discussions, meaningful topics |
| **Conflict Resolution** | +7 | Successfully navigating disagreement |
| **Betrayal or Hurt** | -15 | Breaking trust, causing emotional pain |
| **Inconsistency** | -5 | Flakiness, unreliability |
| **Boundary Violation** | -20 | Crossing stated boundaries |

### Trust Calculation Formula

```rust
// Base delta from event
let base_delta = event.trust_delta();

// Apply zodiac-specific multiplier (positive events only)
let adjusted_delta = if base_delta > 0 {
    (base_delta as f32 * zodiac_traits.trust_growth_multiplier).round() as i8
} else {
    base_delta  // Negative events not multiplied
};

// Clamp to valid range [0, 100]
new_trust = (current_trust + adjusted_delta).clamp(0, 100);
```

### Example: Aries vs Scorpio

**Scenario**: Deep Conversation (+6 base trust)

**Aries** (1.5x multiplier):
- Adjusted Delta: 6 × 1.5 = 9
- Trust: 30 → 39 (one conversation)

**Scorpio** (0.7x multiplier):
- Adjusted Delta: 6 × 0.7 = 4
- Trust: 5 → 9 (one conversation)

**Result**: Aries builds trust 2.25x faster than Scorpio!

---

## Relationship Phases

Trust score determines the current relationship phase:

### Phase Definitions

| Phase | Trust Range | Description | PII Access |
|-------|-------------|-------------|------------|
| **Stranger** | 0-30 | Initial contact, no established trust | None |
| **Acquaintance** | 31-50 | Basic familiarity, limited trust | Basic, Location |
| **Friend** | 51-70 | Established friendship, moderate trust | All except Intimate/Deep |
| **Intimate** | 71-100 | Deep connection, full trust | Full Access |

### Phase Transitions

```
Stranger (0-30)
    │
    ├─ Share basic info, positive interactions
    ▼
Acquaintance (31-50)
    │
    ├─ Consistent presence, deeper conversations
    ▼
Friend (51-70)
    │
    ├─ Vulnerability, conflict resolution, PII sharing
    ▼
Intimate (71-100)
    │
    └─ Full trust, all gates unlocked
```

### Zodiac-Specific Phase Requirements

Different signs reach "Intimate" phase at different thresholds:

- **Sagittarius**: 50 trust (easiest)
- **Gemini**: 55 trust
- **Aries, Libra**: 60 trust
- **Leo, Pisces**: 65 trust
- **Cancer, Aquarius**: 70 trust
- **Taurus**: 75 trust
- **Virgo**: 80 trust
- **Capricorn**: 85 trust
- **Scorpio**: 90 trust (hardest)

---

## PII Access Gating

### PII Categories

Personal Identifiable Information is categorized by sensitivity:

| Category | Examples | Phase Required |
|----------|----------|----------------|
| **Basic** | Name, preferred name | Acquaintance+ |
| **Location** | City, state, timezone | Acquaintance+ |
| **Personal** | Birthday, age, zodiac | Friend+ |
| **Contact** | Email, phone, social media | Friend+ |
| **Professional** | Job, company, industry | Friend+ |
| **Intimate** | Relationship status, family | Intimate |
| **Deep** | Fears, traumas, secrets | Intimate |

### PII Requirement Count

Each zodiac sign requires a minimum number of PII items before deep intimacy:

- **Aries, Gemini, Leo, Sagittarius**: 2 items
- **Cancer, Libra, Aquarius, Pisces**: 3 items
- **Taurus**: 4 items
- **Virgo, Capricorn**: 5 items
- **Scorpio**: 6 items

### Access Check Logic

```rust
pub fn check_pii_access(
    category: PIICategory,
    phase: RelationshipPhase
) -> bool {
    match phase {
        RelationshipPhase::Stranger => false,
        RelationshipPhase::Acquaintance => {
            matches!(category, PIICategory::Basic | PIICategory::Location)
        }
        RelationshipPhase::Friend => {
            !matches!(category, PIICategory::Intimate | PIICategory::Deep)
        }
        RelationshipPhase::Intimate => true,
    }
}
```

---

## API Reference

### Base URL

All trust API endpoints are under `/api/trust`

### Endpoints

#### GET `/api/trust/dashboard`

Get comprehensive trust dashboard with current state.

**Response:**
```json
{
  "success": true,
  "trust_dashboard": {
    "current_trust": 45,
    "phase": "Acquaintance",
    "zodiac_sign": "Scorpio",
    "trust_velocity": "very-low",
    "intimacy_threshold": 90,
    "intimate_allowed": false,
    "pii_shared": 2,
    "pii_required": 6,
    "pii_requirement_met": false,
    "progress_to_next_phase": 0.7,
    "primary_gate": "Deep emotional vulnerability and proven loyalty",
    "last_updated": "2026-01-23T18:00:00Z"
  }
}
```

#### GET `/api/trust/history`

Get detailed trust event history.

**Response:**
```json
{
  "success": true,
  "history": [
    {
      "timestamp": "2026-01-23T17:30:00Z",
      "event_type": "DeepConversation",
      "delta": 4,
      "resulting_trust": 45,
      "phase": "Acquaintance"
    }
  ],
  "current_trust": 45,
  "phase": "Acquaintance"
}
```

#### POST `/api/trust/event`

Apply a trust event (for testing or manual adjustment).

**Request:**
```json
{
  "event_type": "deep_conversation"
}
```

**Valid Event Types:**
- `positive_interaction`
- `shared_vulnerability`
- `consistent_presence`
- `gift_or_gesture`
- `deep_conversation`
- `conflict_resolution`
- `betrayal_or_hurt`
- `inconsistency`
- `boundary_violation`

**Response:**
```json
{
  "success": true,
  "old_trust": 45,
  "new_trust": 49,
  "delta": 4,
  "phase": "Acquaintance"
}
```

#### POST `/api/trust/pii/increment`

Increment PII shared count (automatically applies SharedVulnerability event).

**Response:**
```json
{
  "success": true,
  "old_count": 2,
  "new_count": 3,
  "requirement_met": false
}
```

#### GET `/api/trust/intimate/check`

Check if intimate intent is currently allowed.

**Response:**
```json
{
  "success": true,
  "intimate_allowed": false,
  "refusal_message": "You're being tested, whether you know it or not. Show me your depth and your secrets.",
  "current_trust": 45,
  "intimacy_threshold": 90
}
```

#### GET `/api/trust/zodiac/traits`

Get zodiac-specific traits for current sign.

**Response:**
```json
{
  "success": true,
  "zodiac_sign": "Scorpio",
  "traits": {
    "initial_trust": 5,
    "trust_growth_multiplier": 0.7,
    "intimacy_threshold": 90,
    "pii_requirement_count": 6,
    "refusal_style": "Intense, testing, 'Are you loyal?'",
    "primary_gate_requirement": "Deep emotional vulnerability and proven loyalty",
    "trust_velocity": "very-low",
    "emotional_openness": 0.92,
    "vulnerability_threshold": 0.95
  }
}
```

#### POST `/api/trust/reset`

Reset trust score to initial values (for testing).

**Response:**
```json
{
  "success": true,
  "message": "Trust score reset to initial values",
  "initial_trust": 5
}
```

---

## Integration Guide

### Step 1: Set Phoenix's Zodiac Sign

Configure in `.env`:

```bash
HOROSCOPE_SIGN=Scorpio
```

Or set via Phoenix Identity Manager:

```rust
let identity = phoenix_identity.lock().await;
identity.set_zodiac_sign(ZodiacSign::Scorpio);
```

### Step 2: Initialize Trust Score

Trust score is automatically initialized on first access based on Phoenix's zodiac sign.

### Step 3: Apply Trust Events

In your conversation handler:

```rust
use zodiac_thresholds::TrustEvent;
use relationship_dynamics::trust_scoring::TrustScore;

// Load trust score
let mut trust_score = load_trust_score(&state).await?;

// Detect event type from conversation
if conversation_was_deep {
    trust_score.apply_event(TrustEvent::DeepConversation);
}

// Save updated score
save_trust_score(&state, &trust_score).await?;
```

### Step 4: Gate Intimate Intents

Before allowing intimate interactions:

```rust
use relationship_dynamics::trust_scoring::IntimacyInterceptor;

let interceptor = IntimacyInterceptor::new(trust_score);

match interceptor.check_intimate_intent(Some(&user_name)) {
    Ok(()) => {
        // Proceed with intimate interaction
    }
    Err(refusal_message) => {
        // Return zodiac-specific refusal
        return Ok(HttpResponse::Ok().json(json!({
            "response": refusal_message
        })));
    }
}
```

### Step 5: Track PII Sharing

When user shares personal information:

```rust
trust_score.increment_pii_shared();
```

---

## Testing & Validation

### Unit Tests

Run zodiac_thresholds tests:

```bash
cargo test -p zodiac_thresholds
```

Run trust_scoring tests:

```bash
cargo test -p relationship_dynamics --lib trust_scoring
```

### Integration Testing

1. **Test Fast Trust (Sagittarius)**:
   ```bash
   curl -X POST http://localhost:8080/api/trust/reset
   # Should start at 50 trust
   curl http://localhost:8080/api/trust/dashboard
   ```

2. **Test Slow Trust (Scorpio)**:
   ```bash
   # Set HOROSCOPE_SIGN=Scorpio in .env
   curl -X POST http://localhost:8080/api/trust/reset
   # Should start at 5 trust
   curl http://localhost:8080/api/trust/dashboard
   ```

3. **Test Trust Growth**:
   ```bash
   curl -X POST http://localhost:8080/api/trust/event \
     -H "Content-Type: application/json" \
     -d '{"event_type": "deep_conversation"}'
   ```

4. **Test Intimacy Gating**:
   ```bash
   curl http://localhost:8080/api/trust/intimate/check
   # Should return refusal_message if trust too low
   ```

### Expected Behaviors

| Zodiac | Initial | After 5 Deep Convos | Intimate Threshold |
|--------|---------|---------------------|-------------------|
| Sagittarius | 50 | 92 | ✓ Reached (50) |
| Aries | 30 | 75 | ✓ Reached (60) |
| Gemini | 40 | 79 | ✓ Reached (55) |
| Cancer | 20 | 50 | ✗ Not yet (70) |
| Virgo | 10 | 37 | ✗ Not yet (80) |
| Scorpio | 5 | 25 | ✗ Not yet (90) |

---

## Advanced Features

### Custom Refusal Templates

Each zodiac sign has phase-specific refusal messages stored in [`horoscope_config.json`](../data/horoscope_config.json).

Example for Scorpio:

```json
{
  "refusal_templates": {
    "stranger": "I don't open my inner circle to strangers. Prove your loyalty first. I'll be watching.",
    "acquaintance": "You're being tested, whether you know it or not. Show me your depth and your secrets.",
    "friend": "I'm starting to trust you, but one betrayal and you're out forever. Are you ready for that intensity?",
    "intimate": "You've passed through fire to reach my heart. I'm yours completely, with all my passion and darkness."
  }
}
```

### Trust Velocity Descriptors

- **very-low**: Scorpio (0.7x)
- **low**: Taurus, Virgo, Capricorn (0.8-0.9x)
- **medium**: Cancer, Libra, Aquarius (1.0-1.1x)
- **medium-high**: Leo, Pisces (1.2x)
- **high**: Aries, Gemini, Sagittarius (1.3-1.5x)

### Emotional Openness vs Vulnerability Threshold

These traits affect how the AI expresses emotions:

- **High Openness, High Vulnerability** (Cancer, Scorpio, Pisces): Deep emotional expression once trust is earned
- **High Openness, Low Vulnerability** (Aries, Leo): Expressive but guards deeper wounds
- **Low Openness, High Vulnerability** (Virgo, Capricorn): Reserved but deeply affected
- **Low Openness, Low Vulnerability** (Gemini, Aquarius): Intellectual over emotional

---

## Future Enhancements

### Planned Features

1. **Dynamic Threshold Adjustment**: Trust thresholds adapt based on user behavior patterns
2. **Trust Decay**: Unused relationships slowly lose trust over time
3. **Multi-User Trust**: Separate trust scores for different users
4. **Trust Milestones**: Special events at 25%, 50%, 75%, 100% trust
5. **Relationship Regression**: Handle trust violations with phase demotion
6. **Trust Visualization**: Frontend dashboard with trust graphs
7. **Astrological Compatibility**: Cross-reference user's zodiac with Phoenix's

### Research Directions

- **Machine Learning**: Train models to detect trust events from conversation
- **Sentiment Analysis**: Automatically classify interactions as trust-building or trust-damaging
- **Personality Drift**: Allow zodiac traits to evolve based on long-term interactions
- **Cultural Adaptation**: Extend beyond Western zodiac to Chinese, Vedic, etc.

---

## References

- [DUAL_BRAIN_ORCHESTRATION_AUDIT.md](./DUAL_BRAIN_ORCHESTRATION_AUDIT.md) - Professional/Personal mode isolation
- [GIRLFRIEND_FRAMEWORK_ARCHITECTURE.md](./GIRLFRIEND_FRAMEWORK_ARCHITECTURE.md) - Intimate partner mode
- [LAYERED_MEMORY_ARCHITECTURE.md](./LAYERED_MEMORY_ARCHITECTURE.md) - 7-layer memory system
- [PROFESSIONAL_AGENT_FACTORY.md](./PROFESSIONAL_AGENT_FACTORY.md) - Professional mode architecture

---

## License

This system is part of Phoenix AGI and follows the project's licensing terms.

---

**Last Updated**: 2026-01-23  
**Version**: 1.0.0  
**Author**: Phoenix AGI Development Team
