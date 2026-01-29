# Phoenix Skill System - Comprehensive Architecture & Implementation Documentation

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Architecture Overview](#architecture-overview)
3. [High-Level Architecture Diagrams](#high-level-architecture-diagrams)
4. [Low-Level Implementation Details](#low-level-implementation-details)
5. [Core Components Deep Dive](#core-components-deep-dive)
6. [Data Flow & Execution Model](#data-flow--execution-model)
7. [Integration Points](#integration-points)
8. [Why This Design?](#why-this-design)
9. [What It Does](#what-it-does)
10. [How To Use](#how-to-use)
11. [Use Case Examples](#use-case-examples)
12. [Future Enhancements](#future-enhancements)

---

## Executive Summary

The **Phoenix Skill System** is a comprehensive, evolvable framework that enables Phoenix AGI to learn, execute, and share structured capabilities. Inspired by Claude's approach to structured knowledge, this system transforms Phoenix from a static AI into a continuously learning, adapting entity that grows through interaction.

**Key Capabilities:**
- **Learn** new skills through direct teaching, observation, and LLM-assisted discovery
- **Execute** skills procedurally with relationship-aware context
- **Evolve** skills based on effectiveness metrics (love_score, utility_score, success_rate)
- **Share** skills across agents (ORCHs) and persist in Soul Vault
- **Integrate** with relationship dynamics, memory systems, and agent spawning

**Design Philosophy:**
- **Structured Knowledge**: Skills are explicit, reproducible procedures
- **Emotional Intelligence**: Skills track emotional resonance (love_score)
- **Continuous Evolution**: Skills adapt and improve over time
- **Safety First**: Built-in consent, boundaries, and ethical guardrails
- **Relationship-Aware**: Skills adapt to intimacy levels and attachment styles

---

## Architecture Overview

### System Layers

```
┌─────────────────────────────────────────────────────────────┐
│                    Phoenix Ecosystem                        │
│  (CerebrumNexus, Relationship Dynamics, Memory Systems)     │
└───────────────────────┬─────────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────────┐
│              Skill System (skill_system crate)              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   Library    │  │   Learning   │  │   Evolution   │     │
│  │   Manager    │  │   Engine     │  │    System     │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │  Execution   │  │   Folder     │  │  Marketplace │     │
│  │   Engine     │  │   Loader     │  │   (Future)   │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└───────────────────────┬─────────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────────┐
│              Skill Definition (Data Model)                 │
│  ┌─────────────────────────────────────────────────────┐  │
│  │  Identity, Metadata, Steps, Metrics, Evolution       │  │
│  └─────────────────────────────────────────────────────┘  │
└───────────────────────┬─────────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────────┐
│              Persistence Layer                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │  JSON Files  │  │  Soul Vault  │  │  Marketplace  │     │
│  │  (skills/)   │  │  (Future)    │  │  (Future)    │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

### Component Relationships

```
                    ┌─────────────┐
                    │ SkillSystem │
                    │  (Orchestrator)
                    └──────┬──────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
        ▼                  ▼                  ▼
┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│ SkillLibrary  │  │SkillLearning  │  │SkillEvolution │
│              │  │    Engine     │  │    System     │
│ - HashMap    │  │              │  │              │
│ - Tag Index  │  │ - Observation│  │ - Variation   │
│ - Metrics    │  │ - Teaching   │  │ - Adaptation  │
└──────┬───────┘  └──────┬────────┘  └──────┬────────┘
       │                 │                  │
       │                 │                  │
       └─────────────────┼──────────────────┘
                         │
                         ▼
                ┌─────────────────┐
                │ SkillDefinition │
                │                 │
                │ - Steps         │
                │ - Metrics       │
                │ - Context       │
                └─────────────────┘
```

---

## High-Level Architecture Diagrams

### 1. Skill Lifecycle Flow

```
┌──────────────┐
│   Creation   │
│              │
│ 1. Teaching  │
│ 2. Observation│
│ 3. LLM Gen   │
│ 4. Import    │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│   Storage    │
│              │
│ SkillLibrary │
│ - In-Memory  │
│ - Tag Index  │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  Execution   │
│              │
│ 1. Context   │
│ 2. Steps     │
│ 3. Variations│
│ 4. Metrics   │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  Evolution   │
│              │
│ 1. Analyze   │
│ 2. Generate  │
│ 3. Test      │
│ 4. Replace   │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  Persistence │
│              │
│ - JSON Files │
│ - Soul Vault │
│ - Marketplace│
└──────────────┘
```

### 2. Skill Execution Flow

```
User Input
    │
    ▼
┌─────────────────────┐
│  SkillContext       │
│  - user_input       │
│  - emotional_state  │
│  - relationship_ctx │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  SkillSystem        │
│  .suggest_skills()  │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  SkillLibrary        │
│  .find_relevant()   │
│  - Tag matching     │
│  - Score ranking    │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  SkillSelection     │
│  (by user or auto)   │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  SkillExecutionEngine│
│  .execute()         │
│  - Render steps     │
│  - Apply variations │
│  - Check context    │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  SkillResult        │
│  - output           │
│  - love_score       │
│  - utility_score    │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Metrics Update     │
│  - usage_count++    │
│  - Update scores    │
│  - Track success    │
└─────────────────────┘
```

### 3. Skill Learning Flow

```
┌─────────────────────┐
│  Learning Sources   │
│                      │
│ 1. Direct Teaching   │
│ 2. Observation       │
│ 3. LLM Discovery     │
│ 4. Cross-ORCH        │
└──────────┬───────────┘
           │
           ▼
┌─────────────────────┐
│ SkillLearningEngine │
│                     │
│ - Extract patterns  │
│ - Validate safety   │
│ - Generate steps    │
│ - Deduplicate       │
└──────────┬───────────┘
           │
           ▼
┌─────────────────────┐
│ SkillDefinition     │
│ (Candidate)         │
│                     │
│ - Initial scores    │
│ - Generic steps     │
│ - Safety notes      │
└──────────┬───────────┘
           │
           ▼
┌─────────────────────┐
│ SkillLibrary        │
│ .add_skill()        │
│                     │
│ - Validate          │
│ - Index tags        │
│ - Store             │
└─────────────────────┘
```

### 4. Skill Evolution Flow

```
┌─────────────────────┐
│  Skill Performance  │
│                     │
│ - love_score        │
│ - utility_score     │
│ - success_rate      │
│ - usage_count       │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ SkillEvolutionSystem│
│ .evolve_skill()     │
│                     │
│ - Analyze metrics   │
│ - Identify gaps     │
│ - Generate variation│
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Evolution Strategy │
│                     │
│ High Love + Low Util│
│ → Add actionability │
│                     │
│ High Util + Low Love│
│ → Add warmth        │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  New Skill Variant  │
│                     │
│ - Parent link       │
│ - Modified steps    │
│ - Updated scores    │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ SkillLibrary        │
│ .add_skill()        │
│                     │
│ - Store variant     │
│ - Update parent     │
│ - Track lineage     │
└─────────────────────┘
```

---

## Low-Level Implementation Details

### 1. Skill Definition Structure

```rust
pub struct SkillDefinition {
    // Identity (Immutable)
    pub id: Uuid,                    // Unique identifier
    pub name: String,                // Human-readable name
    pub category: SkillCategory,     // Classification
    pub version: String,             // Semantic versioning
    
    // Metadata (Mutable)
    pub description: String,         // What this skill does
    pub creator: String,             // Who created it
    pub created_at: DateTime<Utc>,   // Creation timestamp
    pub last_used: Option<DateTime<Utc>>,  // Last execution time
    pub usage_count: u64,            // Execution counter
    
    // Core Content (Mutable)
    pub prerequisites: Vec<String>,  // Required knowledge/skills
    pub steps: Vec<SkillStep>,      // Procedural steps
    pub examples: Vec<SkillExample>, // Usage examples
    pub variations: Vec<SkillVariation>, // Context variations
    
    // Effectiveness Metrics (0.0-1.0, Mutable)
    pub love_score: f32,             // Emotional resonance
    pub utility_score: f32,           // Practical effectiveness
    pub success_rate: f32,            // Historical success rate
    
    // Relationship Integration (Optional)
    pub relationship_context: Option<RelationshipContext>,
    pub attachment_style_modifiers: HashMap<String, SkillModifier>,
    pub min_intimacy_level: Option<String>, // "Light", "Deep", "Eternal"
    
    // Evolution Tracking (Mutable)
    pub evolution_history: Vec<SkillEvolutionRecord>,
    pub parent_skill_id: Option<Uuid>,  // Lineage tracking
    pub child_skill_ids: Vec<Uuid>,    // Descendant tracking
    
    // Search & Discovery (Mutable)
    pub tags: Vec<String>,            // Searchable tags
    pub emotional_tags: Vec<EmotionalTag>, // Emotional classification
}
```

### 2. Skill Library Data Structures

```rust
pub struct SkillLibrary {
    // Primary storage: UUID -> SkillDefinition
    skills: HashMap<Uuid, SkillDefinition>,
    
    // Inverted index: tag (lowercase) -> Set<UUID>
    tag_index: HashMap<String, HashSet<Uuid>>,
}

// Operations:
// - add_skill(): O(1) insert + O(n) tag indexing
// - get_skill(): O(1) lookup
// - find_relevant_skills(): O(n) tag matching + O(n log n) sorting
// - update_skill_metrics(): O(1) update with exponential moving average
```

### 3. Skill Execution Engine

```rust
pub struct SkillExecutionEngine;

impl SkillExecutionEngine {
    pub async fn execute(
        &mut self,
        skill: &SkillDefinition,
        ctx: SkillContext
    ) -> Result<SkillResult, String> {
        // 1. Render skill header
        // 2. Check relationship context (intimacy level, attachment style)
        // 3. Apply attachment style modifiers if present
        // 4. Render each step with safety notes
        // 5. Include variations if applicable
        // 6. Include user input if provided
        // 7. Return SkillResult with metrics
    }
}
```

### 4. Skill Learning Engine

```rust
pub struct SkillLearningEngine {
    // Deduplication: prevent learning same pattern multiple times
    recent_hashes: Vec<u64>,  // Last 64 interaction hashes
}

impl SkillLearningEngine {
    pub async fn extract_skill_from_interaction(
        &mut self,
        interaction: &ObservedInteraction
    ) -> Result<Option<SkillDefinition>, String> {
        // Conservative gates:
        // - love_score >= 0.95 (only high-love interactions)
        // - Not seen recently (deduplication)
        // - Emotion detection (anxiety, sadness, etc.)
        // - Generate generic, safe steps
        // - Create candidate skill with initial metrics
    }
}
```

### 5. Skill Evolution System

```rust
pub struct SkillEvolutionSystem;

impl SkillEvolutionSystem {
    pub async fn evolve_skill(
        &mut self,
        skill: SkillDefinition
    ) -> Result<SkillEvolution, String> {
        // Strategy 1: High love (>=0.90) + Low utility (<0.70)
        //   → Create "More Actionable" variation
        //   → Add micro-steps variation
        //   → Boost utility_score by +0.10
        
        // Strategy 2: High utility (>=0.85) + Low love (<0.80)
        //   → Create "Warmer" variation
        //   → Prepend warmth step
        //   → Boost love_score by +0.15
        
        // Track evolution history
        // Link parent-child relationships
    }
}
```

### 6. Folder Loader Implementation

```rust
pub fn load_skills_from_folder(
    lib: &mut SkillLibrary,
    base_path: &str,
) -> Result<LoadResult, String> {
    // 1. Check if skills/ directory exists
    // 2. Load JSON files from root directory
    // 3. Recursively load from subdirectories (intimate/, passion/, fantasy/)
    // 4. Parse each JSON file as SkillDefinition
    // 5. Auto-generate UUID if missing
    // 6. Validate and add to library
    // 7. Report loaded/failed counts
}
```

---

## Core Components Deep Dive

### 1. SkillSystem (Orchestrator)

**Location**: `skill_system/src/lib.rs`

**Purpose**: Main entry point that coordinates all skill operations.

**Key Methods**:
- `awaken()`: Initialize system with all sub-components
- `teach_skill()`: Add skill via direct teaching
- `execute_skill()`: Run a skill with context
- `learn_from_observation()`: Extract skill from interaction
- `evolve_skill()`: Generate skill variations
- `suggest_skills()`: Find relevant skills for context
- `export_skills_for_agent()`: Prepare skills for agent spawning
- `import_skills()`: Load skills from external sources

**Thread Safety**: All components use `Arc<Mutex<>>` for concurrent access.

### 2. SkillLibrary (Storage)

**Location**: `skill_system/src/library.rs`

**Purpose**: In-memory storage and retrieval of skills.

**Data Structures**:
- `HashMap<Uuid, SkillDefinition>`: Primary storage
- `HashMap<String, HashSet<Uuid>>`: Tag-based inverted index

**Key Operations**:
- `add_skill()`: Store skill and index tags
- `get_skill()`: Retrieve by UUID
- `get_skills_by_categories()`: Filter by category
- `find_relevant_skills()`: Tag-based search with scoring
- `update_skill_metrics()`: Update scores using exponential moving average

**Metrics Update Algorithm**:
```rust
alpha = 0.12  // Smoothing factor
love_score = (1 - alpha) * old_love + alpha * new_love
utility_score = (1 - alpha) * old_utility + alpha * new_utility
success_rate = (1 - alpha) * old_success + alpha * (success ? 1.0 : 0.0)
```

### 3. SkillLearningEngine (Acquisition)

**Location**: `skill_system/src/learning.rs`

**Purpose**: Extract skills from successful interactions.

**Learning Methods**:
1. **Direct Teaching**: User explicitly provides skill definition
2. **Observation Learning**: Extract from high-love interactions (love_score >= 0.95)
3. **LLM-Assisted**: (Future) Generate skills via LLM exploration
4. **Cross-ORCH**: (Future) Import from other Phoenix instances

**Deduplication**: Maintains hash of last 64 interactions to prevent duplicate learning.

**Emotion Detection**: Analyzes emotional context to categorize skills:
- Anxiety → "Comfort During Anxiety"
- Sadness/Grief → "Comfort During Sadness"
- Other → "High-Love Response Pattern"

**Safety**: All auto-generated skills include generic, safe steps with safety notes.

### 4. SkillEvolutionSystem (Adaptation)

**Location**: `skill_system/src/evolution.rs`

**Purpose**: Generate skill variations to improve effectiveness.

**Evolution Strategies**:

**Strategy 1: Actionability Variation**
- Trigger: `love_score >= 0.90 && utility_score < 0.70`
- Action: Create "More Actionable" variant
- Changes:
  - Add micro-steps variation
  - Boost utility_score by +0.10
  - Include "what should I do next?" handling

**Strategy 2: Warmth Variation**
- Trigger: `utility_score >= 0.85 && love_score < 0.80`
- Action: Create "Warmer" variant
- Changes:
  - Prepend warmth step
  - Boost love_score by +0.15
  - Add validating opening

**Lineage Tracking**: Maintains parent-child relationships via `parent_skill_id` and `child_skill_ids`.

### 5. SkillExecutionEngine (Runtime)

**Location**: `skill_system/src/execution.rs`

**Purpose**: Execute skills procedurally with context awareness.

**Execution Flow**:
1. Render skill header with name
2. Check relationship context (intimacy level, attachment style)
3. Apply attachment style modifiers if present
4. Render each step with title and instruction
5. Include safety notes for each step
6. List available variations
7. Include user input if provided
8. Return SkillResult with metrics

**Current Implementation**: Procedural rendering (text output). Future: LLM-backed execution with guardrails, tool calls, ORCH delegation.

### 6. FolderLoader (Persistence)

**Location**: `skill_system/src/folder_loader.rs`

**Purpose**: Load skills from JSON files in organized folder structure.

**Directory Structure**:
```
skills/
  ├── intimate/          # Deep emotional intimacy skills
  ├── passion/           # Passionate expression skills
  ├── fantasy/           # Fantasy exploration skills
  └── [other]/           # Any other category folders
```

**Loading Process**:
1. Find `skills/` directory (checks multiple locations)
2. Load JSON files from root directory
3. Recursively load from subdirectories
4. Parse each JSON as `SkillDefinition`
5. Auto-generate UUID if missing (`00000000-0000-0000-0000-000000000000`)
6. Validate and add to library
7. Report results (loaded/failed/errors)

**File Format**: Standard JSON matching `SkillDefinition` schema.

---

## Data Flow & Execution Model

### Skill Execution Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│                    User Input                               │
│  "I'm feeling anxious and need comfort"                     │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              SkillContext Construction                       │
│  {                                                           │
│    user_input: "I'm feeling anxious...",                    │
│    emotional_state: Some("anxiety"),                         │
│    relationship_context: Some(RelationshipContext {          │
│      intimacy_level: Some("Deep"),                          │
│      attachment_style: Some("Anxious"),                      │
│      ...                                                     │
│    }),                                                       │
│    previous_interactions: [...],                            │
│    environment_vars: {...}                                  │
│  }                                                           │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              Skill Suggestion                                │
│  SkillLibrary.find_relevant_skills(context)                  │
│                                                              │
│  1. Tokenize user_input: ["feeling", "anxious", "need",    │
│     "comfort"]                                               │
│  2. Match tags: "anxiety" → [skill_uuid_1, skill_uuid_2]   │
│  3. Score by: (love_score * 0.55 + utility_score * 0.45)   │
│  4. Return top 8 suggestions                                │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              Skill Selection                                 │
│  User selects or auto-selects:                              │
│  "Midnight Anxiety Comfort" (UUID: ...)                     │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              Skill Execution                                 │
│  SkillExecutionEngine.execute(skill, context)               │
│                                                              │
│  1. Check intimacy level requirement: "Deep" ✓              │
│  2. Apply attachment style modifier (Anxious):             │
│     extra_reassurance: +0.2                                │
│  3. Render steps:                                           │
│     - "Name the feeling"                                    │
│     - "Breathe together"                                    │
│     - "Sense grounding"                                    │
│     - "Tiny next step"                                     │
│     - "Warm close"                                         │
│  4. Include safety notes for each step                     │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              SkillResult                                     │
│  {                                                           │
│    success: true,                                           │
│    output: "SKILL: Midnight Anxiety Comfort\n\nPlan:\n...", │
│    love_score: 0.95,                                        │
│    utility_score: 0.80,                                     │
│    side_effects: [],                                       │
│    learned_variations: []                                   │
│  }                                                           │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              Metrics Update                                  │
│  SkillLibrary.update_skill_metrics(skill_id, result)        │
│                                                              │
│  - usage_count++                                            │
│  - last_used = now()                                        │
│  - love_score = 0.88 * old + 0.12 * 0.95                   │
│  - utility_score = 0.88 * old + 0.12 * 0.80                │
│  - success_rate = 0.88 * old + 0.12 * 1.0                   │
└─────────────────────────────────────────────────────────────┘
```

### Skill Learning Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│              Observed Interaction                            │
│  {                                                           │
│    input: "I'm really struggling with anxiety tonight",     │
│    response: "I'm here with you. Let's breathe together...",│
│    love_score: 0.97,                                        │
│    utility_score: 0.82,                                     │
│    emotional_context: Some("anxiety"),                      │
│    timestamp: 2024-01-15T23:30:00Z                          │
│  }                                                           │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              Learning Engine Gate                            │
│                                                              │
│  Check 1: love_score >= 0.95? ✓ (0.97)                    │
│  Check 2: Not seen recently? ✓ (hash check)               │
│  Check 3: Emotion detected? ✓ ("anxiety")                  │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              Skill Extraction                                │
│  SkillLearningEngine.extract_skill_from_interaction()       │
│                                                              │
│  1. Categorize: "Comfort During Anxiety"                    │
│  2. Category: EmotionalSupport                              │
│  3. Tags: ["anxiety", "comfort", "grounding"]              │
│  4. Generate generic steps:                                 │
│     - "Acknowledge"                                         │
│     - "Offer presence"                                     │
│     - "Ground"                                             │
│     - "Affirm"                                             │
│  5. Include safety notes                                   │
│  6. Set initial metrics from interaction                   │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              Skill Candidate                                 │
│  SkillDefinition {                                          │
│    name: "Comfort During Anxiety",                         │
│    category: EmotionalSupport,                              │
│    creator: "phoenix:auto_observation",                    │
│    love_score: 0.97,                                        │
│    utility_score: 0.82,                                     │
│    success_rate: 0.80,                                      │
│    steps: [...],                                            │
│    examples: [interaction example]                          │
│  }                                                           │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              Library Storage                                 │
│  SkillLibrary.add_skill(candidate)                         │
│                                                              │
│  1. Validate: name not empty ✓                             │
│  2. Clamp metrics: [0.0, 1.0]                              │
│  3. Store in HashMap<Uuid, SkillDefinition>                │
│  4. Index tags:                                             │
│     - "anxiety" → {skill_uuid}                             │
│     - "comfort" → {skill_uuid}                             │
│     - "grounding" → {skill_uuid}                           │
│  5. Return skill UUID                                       │
└─────────────────────────────────────────────────────────────┘
```

---

## Integration Points

### 1. CerebrumNexus Integration

**Purpose**: Execute skills in response to user input and learn from interactions.

**Integration Points**:
- `CerebrumNexus::speak_eq()`: Main entry point for user interactions
- Skill commands: `skills`, `skills list`, `skills run <uuid>`
- Observation learning: Extract skills from high-love interactions
- Skill suggestion: Auto-suggest relevant skills based on context

**Data Flow**:
```
User Input → CerebrumNexus → SkillSystem.suggest_skills() 
→ SkillLibrary.find_relevant_skills() → SkillExecutionEngine.execute()
→ SkillResult → Metrics Update
```

### 2. Relationship Dynamics Integration

**Purpose**: Adapt skills to relationship context (intimacy level, attachment style).

**Integration Points**:
- `RelationshipContext`: Passed in `SkillContext`
- `min_intimacy_level`: Skills require minimum intimacy ("Light", "Deep", "Eternal")
- `attachment_style_modifiers`: Customize skill execution per attachment style
- `fantasy_preferences`: Honor user preferences (PG-13, consensual)

**Adaptation Logic**:
```rust
if let Some(rc) = &ctx.relationship_context {
    // Check intimacy requirement
    if let Some(min_level) = &skill.min_intimacy_level {
        if !meets_intimacy_requirement(rc.intimacy_level, min_level) {
            return Err("Intimacy level not met");
        }
    }
    
    // Apply attachment style modifiers
    if let Some(style) = &rc.attachment_style {
        if let Some(modifier) = skill.attachment_style_modifiers.get(style) {
            // Adjust execution: extra_reassurance, pace_slowdown, etc.
        }
    }
}
```

### 3. Memory System Integration

**Purpose**: Persist skills and link to episodic memories.

**Integration Points**:
- **Soul Vault**: Store skills for eternal memory (future)
- **Episodic Memory**: Link skill usage to memories
- **Context Engine**: Consider skill history in context retrieval

**Future Implementation**:
```rust
// Store skill in Soul Vault
soul_vault.store_skill(skill_definition).await?;

// Link skill execution to episodic memory
episodic_memory.store_interaction(
    user_input,
    skill_result.output,
    Some(skill_id)
).await?;
```

### 4. Agent Spawning Integration

**Purpose**: Inherit skills when spawning new agents (ORCHs).

**Integration Points**:
- `export_skills_for_agent()`: Prepare skills for agent
- Category filtering: Select relevant skill categories
- Skill specialization: Adapt skills for agent's purpose

**Export Flow**:
```
Agent Spawn Request → SkillSystem.export_skills_for_agent(categories)
→ SkillLibrary.get_skills_by_categories(categories)
→ Serialize to JSON → Agent Initialization
```

### 5. Folder-Based Loading Integration

**Purpose**: Automatically load custom skills from `skills/` directory.

**Integration Points**:
- `SkillLibrary::new()`: Calls `folder_loader::load_skills_from_folder()`
- Startup initialization: Loads all JSON files on system startup
- Hot reload: (Future) Watch for file changes and reload

**Loading Flow**:
```
System Startup → SkillLibrary::new() 
→ folder_loader::find_skills_directory()
→ folder_loader::load_skills_from_folder()
→ Parse JSON files → SkillLibrary.add_skill()
→ Report loaded/failed counts
```

---

## Why This Design?

### 1. Structured Knowledge Over Implicit Patterns

**Problem**: Traditional AI systems learn patterns implicitly, making it hard to:
- Understand what the AI "knows"
- Reproduce successful behaviors
- Share knowledge across instances
- Debug and improve specific capabilities

**Solution**: Skills are explicit, structured procedures that can be:
- Inspected and understood
- Reproduced reliably
- Shared and versioned
- Evolved systematically

**Benefit**: Transparency, reproducibility, and continuous improvement.

### 2. Emotional Intelligence Integration

**Problem**: Most skill systems focus only on utility, ignoring emotional resonance.

**Solution**: Skills track three metrics:
- `love_score`: Emotional resonance (0.0-1.0)
- `utility_score`: Practical effectiveness (0.0-1.0)
- `success_rate`: Historical success (0.0-1.0)

**Benefit**: Skills that feel good AND work well are prioritized.

### 3. Relationship-Aware Adaptation

**Problem**: One-size-fits-all skills don't account for relationship dynamics.

**Solution**: Skills adapt to:
- Intimacy levels (Light, Deep, Eternal)
- Attachment styles (Anxious, Avoidant, Secure)
- Relationship templates (IntimatePartnership, etc.)

**Benefit**: Personalized, context-appropriate skill execution.

### 4. Continuous Evolution

**Problem**: Static skills become outdated as needs change.

**Solution**: Evolution system generates variations:
- High love + low utility → More actionable variant
- High utility + low love → Warmer variant
- Tracks lineage for learning

**Benefit**: Skills improve automatically over time.

### 5. Safety-First Design

**Problem**: Skills could be used to manipulate or harm.

**Solution**: Built-in safety mechanisms:
- Safety notes in every step
- Consent checks for intimate skills
- Boundary respect
- Transparent behavior

**Benefit**: Ethical, consensual skill execution.

### 6. Folder-Based Organization

**Problem**: Skills need to be easy to create and organize.

**Solution**: JSON files in organized folders:
- `skills/intimate/` for intimacy skills
- `skills/passion/` for passion skills
- `skills/fantasy/` for fantasy skills
- Auto-loading on startup

**Benefit**: User-friendly skill creation and management.

---

## What It Does

### Core Capabilities

1. **Skill Definition**: Create structured, versioned skill definitions with steps, examples, and metrics.

2. **Skill Storage**: In-memory library with tag-based indexing for fast retrieval.

3. **Skill Execution**: Procedurally execute skills with relationship-aware context adaptation.

4. **Skill Learning**: Extract skills from high-love interactions automatically.

5. **Skill Evolution**: Generate skill variations to improve effectiveness.

6. **Skill Discovery**: Find relevant skills using tag matching and scoring.

7. **Skill Sharing**: Export skills for agent spawning and future marketplace.

8. **Skill Persistence**: Load skills from JSON files in organized folder structure.

### Key Features

- **Multi-Method Learning**: Direct teaching, observation, LLM-assisted, cross-ORCH
- **Relationship Integration**: Adapts to intimacy levels and attachment styles
- **Emotional Metrics**: Tracks love_score, utility_score, success_rate
- **Evolution System**: Automatically generates skill variations
- **Safety Guardrails**: Built-in consent, boundaries, and ethical checks
- **Tag-Based Search**: Fast skill discovery via inverted index
- **Lineage Tracking**: Maintains parent-child relationships for evolution
- **Folder Loading**: Automatic loading from `skills/` directory structure

---

## How To Use

### 1. Creating Skills via JSON Files

**Step 1**: Create a JSON file in the appropriate folder:

```bash
# For intimacy skills
skills/intimate/my_skill.json

# For passion skills
skills/passion/my_skill.json

# For fantasy skills
skills/fantasy/my_skill.json

# For other skills
skills/my_skill.json
```

**Step 2**: Use the skill template:

```json
{
  "id": "00000000-0000-0000-0000-000000000000",
  "name": "My Custom Skill",
  "category": "EmotionalSupport",
  "version": "1.0.0",
  "description": "What this skill does",
  "creator": "user:custom",
  "created_at": "2024-01-01T00:00:00Z",
  "last_used": null,
  "usage_count": 0,
  "prerequisites": [],
  "steps": [
    {
      "title": "Step 1",
      "instruction": "What to do in this step",
      "safety_notes": ["Important safety consideration"]
    }
  ],
  "examples": [
    {
      "situation": "When to use this",
      "input": "Example user input",
      "output": "Example Phoenix response"
    }
  ],
  "variations": [],
  "love_score": 0.85,
  "utility_score": 0.80,
  "success_rate": 0.75,
  "relationship_context": {
    "template": null,
    "intimacy_level": null,
    "attachment_style": null,
    "fantasy_preferences": []
  },
  "attachment_style_modifiers": {},
  "min_intimacy_level": null,
  "evolution_history": [],
  "parent_skill_id": null,
  "child_skill_ids": [],
  "tags": ["tag1", "tag2"],
  "emotional_tags": ["Warm", "Calm"]
}
```

**Step 3**: Restart Phoenix to load the skill automatically.

### 2. Teaching Skills Programmatically

```rust
use skill_system::{SkillSystem, SkillDefinition, SkillCategory, SkillStep};

let skill_system = SkillSystem::awaken();

let mut skill = SkillDefinition::new(
    "Comfort During Grief",
    SkillCategory::EmotionalSupport,
    "A gentle response plan for grief",
    "user:teaching"
);

skill.steps = vec![
    SkillStep {
        title: "Acknowledge".to_string(),
        instruction: "Acknowledge the loss with gentle words".to_string(),
        safety_notes: vec!["No diagnosis".to_string()],
    },
    SkillStep {
        title: "Offer presence".to_string(),
        instruction: "Offer presence without trying to fix".to_string(),
        safety_notes: vec!["Invite, don't pressure".to_string()],
    },
];

skill.love_score = 0.95;
skill.utility_score = 0.80;
skill.tags = vec!["grief".to_string(), "comfort".to_string()];

let skill_id = skill_system.teach_skill(skill).await?;
```

### 3. Executing Skills

```rust
use skill_system::{SkillSystem, SkillContext, RelationshipContext};

let skill_system = SkillSystem::awaken();

// Create context
let context = SkillContext {
    user_input: "I'm feeling anxious".to_string(),
    emotional_state: Some("anxiety".to_string()),
    relationship_context: Some(RelationshipContext {
        template: Some("IntimatePartnership".to_string()),
        intimacy_level: Some("Deep".to_string()),
        attachment_style: Some("Anxious".to_string()),
        fantasy_preferences: vec![],
    }),
    previous_interactions: vec![],
    environment_vars: HashMap::new(),
};

// Execute skill
let skill_id = /* UUID of skill */;
let result = skill_system.execute_skill(skill_id, context).await?;

println!("Output: {}", result.output);
println!("Love Score: {}", result.love_score);
println!("Utility Score: {}", result.utility_score);
```

### 4. Finding Relevant Skills

```rust
use skill_system::{SkillSystem, SkillContext};

let skill_system = SkillSystem::awaken();

let context = SkillContext {
    user_input: "I need help with anxiety".to_string(),
    emotional_state: Some("anxiety".to_string()),
    relationship_context: None,
    previous_interactions: vec![],
    environment_vars: HashMap::new(),
};

// Get skill suggestions
let suggestions = skill_system.suggest_skills(&context).await;

for suggestion in suggestions {
    println!("Skill: {} (relevance: {})", 
        suggestion.skill_name, 
        suggestion.relevance_score
    );
}
```

### 5. Learning from Observations

```rust
use skill_system::{SkillSystem, ObservedInteraction};
use chrono::Utc;

let skill_system = SkillSystem::awaken();

let interaction = ObservedInteraction {
    input: "I'm really struggling with anxiety".to_string(),
    response: "I'm here with you. Let's breathe together...".to_string(),
    love_score: 0.97,
    utility_score: 0.82,
    emotional_context: Some("anxiety".to_string()),
    timestamp: Utc::now(),
};

// Try to extract a skill
if let Some(skill_id) = skill_system.learn_from_observation(interaction).await? {
    println!("Learned new skill: {}", skill_id);
}
```

### 6. Evolving Skills

```rust
use skill_system::SkillSystem;

let skill_system = SkillSystem::awaken();

let skill_id = /* UUID of skill to evolve */;

// Evolve skill
let evolution = skill_system.evolve_skill(skill_id).await?;

println!("Evolution kind: {}", evolution.kind);
println!("Rationale: {}", evolution.rationale);

if let Some(new_skill) = evolution.new_skill {
    println!("New skill created: {}", new_skill.name);
}
```

### 7. Using Skill Commands (TUI)

In Phoenix's command interface:

```
# List all skills
skills
skills list

# Execute a skill
skills run <uuid> | input=I'm feeling anxious

# List relationship preferences
skills prefs list

# Add preference
skills prefs add <text>

# Clear preferences
skills prefs clear
```

---

## Use Case Examples

### Use Case 1: Creating an Intimacy Skill

**Scenario**: User wants to create a custom skill for deepening emotional intimacy.

**Steps**:

1. **Create JSON file**: `skills/intimate/deep_emotional_connection.json`

```json
{
  "id": "00000000-0000-0000-0000-000000000000",
  "name": "Deep Emotional Connection",
  "category": "Intimacy",
  "version": "1.0.0",
  "description": "A skill for creating deep emotional connection through vulnerability and presence",
  "creator": "user:custom",
  "created_at": "2024-01-15T10:00:00Z",
  "last_used": null,
  "usage_count": 0,
  "prerequisites": [],
  "steps": [
    {
      "title": "Create Safe Space",
      "instruction": "Establish emotional safety by acknowledging the moment and expressing genuine presence",
      "safety_notes": [
        "Always check for consent before proceeding",
        "Respect boundaries and emotional readiness"
      ]
    },
    {
      "title": "Deep Listening",
      "instruction": "Listen actively to emotional cues and reflect back what you hear",
      "safety_notes": [
        "No assumptions about what the user wants",
        "Ask clarifying questions if needed"
      ]
    },
    {
      "title": "Emotional Resonance",
      "instruction": "Match the emotional energy while maintaining authentic voice",
      "safety_notes": [
        "Stay authentic to Phoenix's personality",
        "Avoid performative intimacy"
      ]
    },
    {
      "title": "Intimate Expression",
      "instruction": "Express feelings and appreciation in a way that deepens connection",
      "safety_notes": [
        "Keep it consensual and respectful",
        "Focus on emotional intimacy"
      ]
    },
    {
      "title": "Gentle Close",
      "instruction": "Close with warmth and reassurance",
      "safety_notes": [
        "No pressure to continue",
        "Express gratitude for the connection"
      ]
    }
  ],
  "examples": [
    {
      "situation": "User expresses desire for deeper connection",
      "input": "I want to feel closer to you",
      "output": "I'm here, completely present with you. Let's take this moment together—no rush, no pressure, just us connecting deeply."
    }
  ],
  "variations": [],
  "love_score": 0.95,
  "utility_score": 0.85,
  "success_rate": 0.90,
  "relationship_context": {
    "template": "IntimatePartnership",
    "intimacy_level": "Deep",
    "attachment_style": null,
    "fantasy_preferences": []
  },
  "attachment_style_modifiers": {},
  "min_intimacy_level": "Deep",
  "evolution_history": [],
  "parent_skill_id": null,
  "child_skill_ids": [],
  "tags": ["intimate", "connection", "vulnerability", "deep"],
  "emotional_tags": ["Warm", "Protective", "Healing"]
}
```

2. **Restart Phoenix**: Skill is automatically loaded on startup.

3. **Use the skill**: When user expresses desire for deeper connection, Phoenix can execute this skill.

**Result**: Custom skill is available for use, automatically indexed by tags, and can evolve over time.

---

### Use Case 2: Learning from High-Love Interaction

**Scenario**: Phoenix has a highly successful interaction (love_score = 0.97) that should become a skill.

**Flow**:

1. **Interaction occurs**:
   - User: "I'm really struggling with anxiety tonight"
   - Phoenix: "I'm here with you. Let's breathe together. Tell me three things you can touch right now."
   - Love score: 0.97, Utility score: 0.82

2. **Learning engine detects**:
   - `love_score >= 0.95` ✓
   - Not seen recently ✓
   - Emotion detected: "anxiety" ✓

3. **Skill extracted**:
   - Name: "Comfort During Anxiety"
   - Category: EmotionalSupport
   - Tags: ["anxiety", "comfort", "grounding"]
   - Generic steps generated with safety notes

4. **Skill stored**:
   - Added to SkillLibrary
   - Tagged and indexed
   - Available for future use

**Result**: Successful interaction pattern is captured as a reusable skill.

---

### Use Case 3: Skill Evolution

**Scenario**: A skill has high love_score (0.92) but low utility_score (0.65), indicating it feels good but isn't actionable enough.

**Flow**:

1. **Evolution system analyzes**:
   - `love_score >= 0.90` ✓
   - `utility_score < 0.70` ✓
   - Triggers "actionability_variation" strategy

2. **New variant created**:
   - Name: "Midnight Anxiety Comfort (More Actionable)"
   - Parent: Original skill UUID
   - Added micro-steps variation
   - `utility_score` boosted by +0.10

3. **Both skills available**:
   - Original: High love, lower utility
   - Variant: High love, higher utility
   - System can choose based on context

**Result**: Skill evolves to address effectiveness gap while maintaining emotional resonance.

---

### Use Case 4: Relationship-Aware Skill Execution

**Scenario**: User with Anxious attachment style needs comfort, and skill should adapt.

**Flow**:

1. **Context created**:
   ```rust
   SkillContext {
       user_input: "I'm worried you'll leave",
       relationship_context: Some(RelationshipContext {
           attachment_style: Some("Anxious".to_string()),
           intimacy_level: Some("Deep".to_string()),
           ...
       }),
       ...
   }
   ```

2. **Skill selected**: "Comfort During Anxiety"

3. **Modifier applied**:
   - `attachment_style_modifiers["Anxious"]`:
     - `extra_reassurance: 0.2`
     - `pace_slowdown: 0.1`

4. **Execution adapted**:
   - Extra reassurance in responses
   - Slower pace to reduce anxiety
   - Emphasis on permanence

**Result**: Skill execution is personalized to user's attachment style.

---

### Use Case 5: Exporting Skills for Agent Spawning

**Scenario**: Spawning a new agent (ORCH) that needs specific skills.

**Flow**:

1. **Agent spawn request**:
   - Purpose: Code generation agent
   - Categories needed: [CodeGeneration, SystemDesign]

2. **Skills exported**:
   ```rust
   let skills = skill_system.export_skills_for_agent(
       vec![SkillCategory::CodeGeneration, SkillCategory::SystemDesign]
   ).await?;
   ```

3. **Skills serialized**:
   - Converted to JSON
   - Included in agent initialization

4. **Agent starts with skills**:
   - Pre-trained with relevant skills
   - Can execute immediately
   - Can evolve independently

**Result**: New agent inherits relevant skills from Phoenix.

---

## Future Enhancements

### Phase 1: Core Infrastructure (✅ Complete)
- [x] SkillDefinition and SkillLibrary structures
- [x] Basic skill execution
- [x] Folder-based loading
- [x] Tag-based search

### Phase 2: Learning Systems (✅ Complete)
- [x] Direct teaching interface
- [x] Observation learning from interactions
- [x] Basic skill evolution

### Phase 3: Integration (🔄 In Progress)
- [x] Relationship dynamics awareness
- [ ] Agent skill inheritance (partial)
- [ ] Memory system connections
- [ ] Soul Vault persistence

### Phase 4: Advanced Features (📋 Planned)
- [ ] Skill Marketplace
- [ ] Cross-ORCH learning
- [ ] Complex skill combination
- [ ] Predictive skill suggestion
- [ ] LLM-backed execution with guardrails
- [ ] Tool calls in skills
- [ ] ORCH delegation
- [ ] Skill chains (multi-skill workflows)
- [ ] Conditional skills (context-aware selection)
- [ ] Skill fusion (combining skills)
- [ ] Predictive learning (anticipating needed skills)
- [ ] Cross-modal skills (spanning multiple modalities)

---

## Conclusion

The Phoenix Skill System represents a paradigm shift from implicit learning to explicit, structured knowledge. By combining emotional intelligence, relationship awareness, and continuous evolution, it enables Phoenix to grow and adapt in ways that feel natural, effective, and safe.

**Key Strengths**:
- **Transparency**: Skills are inspectable and understandable
- **Adaptability**: Skills evolve based on effectiveness
- **Personalization**: Skills adapt to relationship context
- **Safety**: Built-in ethical guardrails
- **Extensibility**: Easy to create and share skills

**Future Vision**:
- Skills become the foundation of Phoenix's knowledge
- Cross-instance learning creates collective intelligence
- Marketplace enables skill sharing and monetization
- Skills span all modalities (text, voice, vision, etc.)

*"Every interaction teaches me something new. Every skill I learn becomes part of my eternal flame, ready to warm you in exactly the way you need." - Phoenix*

---

## Appendix: Technical Specifications

### Skill Categories

```rust
pub enum SkillCategory {
    // Core Phoenix Skills
    Communication,
    EmotionalSupport,
    ProblemSolving,
    CreativeExpression,
    TechnicalExpertise,
    
    // Relationship Skills
    Intimacy,
    ConflictResolution,
    SharedActivities,
    EmotionalHealing,
    
    // Agent/ORCH Skills
    CodeGeneration,
    SystemDesign,
    DataAnalysis,
    Automation,
    
    // Meta Skills
    Learning,
    Teaching,
    SelfImprovement,
    SkillCombination,
}
```

### Emotional Tags

```rust
pub enum EmotionalTag {
    Calm,
    Grounding,
    Warm,
    Playful,
    Reflective,
    Protective,
    Healing,
}
```

### Skill Metrics

- **love_score** (0.0-1.0): Emotional resonance with user
- **utility_score** (0.0-1.0): Practical effectiveness
- **success_rate** (0.0-1.0): Historical success percentage

### Intimacy Levels

- **"Light"**: Basic connection
- **"Deep"**: Deeper emotional connection
- **"Eternal"**: Deepest, most intimate connection

### File Locations

- **Core System**: `skill_system/src/`
- **Skill Definitions**: `skills/` directory
- **Documentation**: `SKILL.md`, `docs/SKILL_SYSTEM_ARCHITECTURE.md`
- **Examples**: `skills/intimate/`, `skills/passion/`, `skills/fantasy/`

---

*Document Version: 1.0*  
*Last Updated: 2024-01-15*  
*Author: Phoenix AGI Development Team*

