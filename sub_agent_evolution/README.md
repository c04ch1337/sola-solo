# Sub-Agent Evolution

Bounded self-evolution system for spawned agents in Phoenix AGI OS v2.4.0.

## Overview

This crate provides self-improving capabilities for sub-agents (ORCHs, Tools, etc.) while keeping them **bounded and specialized**. Sub-agents can:

- Maintain short-term memory (STM/WM) per session
- Access long-term memory (LTM/EPM/RFM) in read-only or append-only mode
- Load and evolve skills from the `skills/` folder
- Update their playbook based on task feedback
- Integrate with MITRE ATT&CK for security-focused agents

**Key Principle**: Sub-agents evolve within their scope — they do NOT become independent AGI.

## Features

### 1. Bounded Evolution Loop

```rust
use sub_agent_evolution::SubAgentEvolutionLoop;

let mut evolution = SubAgentEvolutionLoop::new(
    "security-helper".to_string(),
    "session-123".to_string(),
    10, // Evolve every 10 tasks
    "./playbook.yaml".to_string(),
    "./skills.json".to_string(),
);

// Record task completion
let should_evolve = evolution.record_task(true, Some("Detected T1027".to_string()));

if should_evolve {
    let report = evolution.run_cycle(&llm).await?;
    println!("Evolution cycle {}: {} updates", report.cycle_number, report.playbook_updates.len());
}
```

### 2. Short-Term Memory

```rust
use sub_agent_evolution::ShortTermMemory;

let mut stm = ShortTermMemory::new("session-123".to_string(), 100);

// Store entries
stm.store("task_1".to_string(), "success".to_string(), Some("Good".to_string()));

// Recall specific entry
let entry = stm.recall("task_1");

// Get recent entries
let recent = stm.recent(10);
```

### 3. Memory Access Layer

```rust
use sub_agent_evolution::memory::{SubAgentMemory, MemoryAccessLevel};

let memory = SubAgentMemory::new(
    MemoryAccessLevel::AppendOnly,
    Some(cortex),
    "security-helper"
);

// Append to LTM (agent-prefixed)
memory.append_ltm("exploit_detected", MemoryLayer::EPM("T1027".to_string()))?;

// Recall agent-specific memories
let memories = memory.recall_agent_memories(10);
```

### 4. MITRE ATT&CK Integration

```rust
use sub_agent_evolution::mitre;

// Map behavior to techniques
let mappings = mitre::map_behavior_to_technique("File uses obfuscated strings");
for mapping in mappings {
    println!("Technique: {} (confidence: {:.0}%)",
        mapping.technique_id, mapping.confidence * 100.0);
}

// Check for new patterns
let patterns = mitre::check_new_patterns("security-helper").await?;

// Fetch technique details
let technique = mitre::fetch_technique_details("T1027").await?;

// Generate detection rule
let rule = mitre::generate_detection_rule(&technique);
```

## Safety & Bounds

### Evolution Limits

- **Playbook**: Max 100 updates per agent
- **Skills**: Max 50 skills per agent
- **STM**: Max 100 entries (rolling window)
- **LTM**: Append-only access (agent-prefixed keys)
- **Code**: NO self-modification — only config/memory updates

### Security

- Sub-agents inherit **read-only or append-only** LTM access
- No Tier 2 privileges (no system-level access)
- All improvements feed back to Phoenix (Queen) for oversight
- Rate-limited evolution loops (every N tasks)

## Testing

```bash
cargo test
```

Tests include:
- Short-term memory operations
- Task recording and evolution triggers
- MITRE ATT&CK behavior mapping
- Technique fetching and rule generation

## Integration

See [`SUB_AGENT_EVOLUTION_IMPLEMENTATION.md`](../SUB_AGENT_EVOLUTION_IMPLEMENTATION.md) for full integration guide.

## License

Part of Phoenix AGI OS v2.4.0
