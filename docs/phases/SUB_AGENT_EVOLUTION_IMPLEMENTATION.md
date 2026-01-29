# Sub-Agent Self-Evolution Implementation

## Overview

Sub-agents and spawned agents (ORCHs, Tools, etc.) now have self-evolving/self-improving capabilities, including:
- **Short-term memory (STM/WM)**: Per-session, in-memory cache
- **Long-term memory (LTM/EPM/RFM)**: Append-only access to shared Phoenix memory
- **Skills**: Load from `skills/` folder + evolve via love/utility scores
- **Playbook**: YAML copy from Phoenix + evolve based on feedback
- **Bounded evolution**: Update playbook/memory after N tasks, no code self-modification
- **MITRE ATT&CK integration**: For security-focused agents

## Architecture

### Inheritance Model

Spawned agents inherit from Phoenix's core systems:

```
Phoenix AGI OS v2.4.0 (Queen/Supreme Layer)
    ↓
Sub-Agent Evolution System
    ↓
Spawned Agents (Bounded, Specialized)
```

**Key Principle**: Sub-agents evolve within their scope — they do NOT become independent AGI.

### Components

1. **[`sub_agent_evolution`](sub_agent_evolution/src/lib.rs)** - Core evolution loop
2. **[`sub_agent_evolution::memory`](sub_agent_evolution/src/memory.rs)** - Memory access layer
3. **[`sub_agent_evolution::playbook`](sub_agent_evolution/src/playbook.rs)** - Playbook management
4. **[`sub_agent_evolution::skills`](sub_agent_evolution/src/skills.rs)** - Skills management
5. **[`sub_agent_evolution::mitre`](sub_agent_evolution/src/mitre.rs)** - MITRE ATT&CK integration

## Usage

### 1. Spawning an Agent with Evolution

```rust
use agent_spawner::{AgentSpawner, AgentTier, AgentTemplateOverrides};
use sub_agent_evolution::{AgentInheritance, LTMAccess};

let spawner = AgentSpawner::awaken()?;

let overrides = AgentTemplateOverrides {
    zodiac_sign: Some("Leo".to_string()),
    evolution: Some(AgentInheritance {
        ltm_access: LTMAccess::AppendOnly,
        inherited_skills: vec!["security_scan".to_string()],
        playbook_template: "playbook_template.yaml".to_string(),
        evolution_interval: 10, // Evolve every 10 tasks
    }),
};

let agent = spawner.spawn_agent(
    "security-helper",
    "Detects exploits and malware",
    &generated_code,
    AgentTier::Free,
    overrides,
).await?;
```

### 2. Running the Evolution Loop

```rust
use sub_agent_evolution::SubAgentEvolutionLoop;
use llm_orchestrator::LLMOrchestrator;

let mut evolution = SubAgentEvolutionLoop::new(
    "security-helper".to_string(),
    "session-123".to_string(),
    10, // Evolve every 10 tasks
    "./playbook.yaml".to_string(),
    "./skills.json".to_string(),
);

// Record task completion
let should_evolve = evolution.record_task(true, Some("Detected T1027 obfuscation".to_string()));

if should_evolve {
    let llm = LLMOrchestrator::awaken()?;
    let report = evolution.run_cycle(&llm).await?;
    
    println!("Evolution cycle {}: {} playbook updates, {} skills learned",
        report.cycle_number,
        report.playbook_updates.len(),
        report.skills_learned.len()
    );
}
```

### 3. MITRE ATT&CK Integration

```rust
use sub_agent_evolution::mitre;

// Map file behavior to MITRE techniques
let mappings = mitre::map_behavior_to_technique("File uses obfuscated strings");
for mapping in mappings {
    println!("Technique: {} (confidence: {:.0}%)",
        mapping.technique_id,
        mapping.confidence * 100.0
    );
}

// Check for new patterns
let patterns = mitre::check_new_patterns("security-helper").await?;
println!("New MITRE patterns: {:?}", patterns);

// Fetch technique details
let technique = mitre::fetch_technique_details("T1027").await?;
println!("Technique: {} - {}", technique.id, technique.name);

// Generate detection rule
let rule = mitre::generate_detection_rule(&technique);
println!("Detection rule:\n{}", rule);
```

### 4. Memory Access

```rust
use sub_agent_evolution::memory::{SubAgentMemory, MemoryAccessLevel};
use neural_cortex_strata::{NeuralCortexStrata, MemoryLayer};
use std::sync::Arc;

let cortex = Arc::new(NeuralCortexStrata::awaken());
let memory = SubAgentMemory::new(
    MemoryAccessLevel::AppendOnly,
    Some(cortex),
    "security-helper"
);

// Append to LTM (agent-prefixed)
memory.append_ltm("exploit_detected", MemoryLayer::EPM("T1027 obfuscation".to_string()))?;

// Recall agent-specific memories
let memories = memory.recall_agent_memories(10);
for (key, layer) in memories {
    println!("Memory: {} -> {:?}", key, layer);
}
```

## Chat Commands

### Spawn Agent with Evolution

```
spawn agent security-helper detects exploits
```

This will:
1. Generate agent code via LLM
2. Create GitHub repository
3. Push code with evolution support
4. Initialize playbook, skills, and memory
5. Return agent details

### Check Agent Status

```
agent status security-helper
```

Returns:
- Evolution cycle number
- Tasks completed
- Next evolution at (task count)
- STM entries count

## Configuration

### Environment Variables

```bash
# Evolution settings
EVOLUTION_INTERVAL=10              # Tasks between evolution cycles
MAX_PLAYBOOK_UPDATES=100           # Bounded: max playbook updates
MAX_SKILLS=50                      # Bounded: max skills per agent

# Memory settings
LTM_ACCESS=append_only             # read_only | append_only | none
STM_MAX_ENTRIES=100                # Max short-term memory entries

# MITRE ATT&CK
MITRE_API_BASE=https://raw.githubusercontent.com/mitre/cti/master/enterprise-attack
```

### Playbook Template

```yaml
# playbook.yaml
version: 1
updates: []
telemetry: {}
```

### Skills Template

```json
{
  "schema": "phoenix.skill_system.v1",
  "notes": "Seed skill library for spawned agent",
  "skills": []
}
```

## Safety & Bounds

### Evolution Limits

1. **Playbook**: Max 100 updates per agent
2. **Skills**: Max 50 skills per agent
3. **STM**: Max 100 entries (rolling window)
4. **LTM**: Append-only access (agent-prefixed keys)
5. **Code**: NO self-modification — only config/memory updates

### Security

- Sub-agents inherit **read-only or append-only** LTM access
- No Tier 2 privileges (no system-level access)
- All improvements feed back to Phoenix (Queen) for oversight
- Rate-limited evolution loops (every N tasks)

### Alignment

- Evolution bounded to playbook/memory updates
- No autonomous code changes
- User feedback drives skill/playbook improvements
- Phoenix retains supreme oversight

## Testing

### Unit Tests

```bash
cd sub_agent_evolution
cargo test
```

### Integration Tests

```bash
# Test spawning with evolution
cargo test --test spawn_with_evolution

# Test evolution cycle
cargo test --test evolution_cycle

# Test MITRE integration
cargo test --test mitre_integration
```

### Manual Testing

1. **Spawn agent**:
   ```bash
   curl -X POST http://localhost:3000/api/command \
     -H "Content-Type: application/json" \
     -d '{"command": "spawn agent test-helper does testing"}'
   ```

2. **Simulate tasks**:
   ```rust
   for i in 0..15 {
       let success = i % 3 != 0; // 66% success rate
       let should_evolve = evolution.record_task(success, Some(format!("Task {}", i)));
       if should_evolve {
           let report = evolution.run_cycle(&llm).await?;
           println!("Evolution report: {:?}", report);
       }
   }
   ```

3. **Check evolution**:
   - Verify playbook updates in `playbook.yaml`
   - Verify skills in `skills.json`
   - Check logs for evolution cycles

## Frontend Integration

### Chat Parser

Add to [`frontend_desktop/App.tsx`](frontend_desktop/App.tsx):

```typescript
// Parse spawn command
if (message.startsWith('spawn agent ')) {
  const parts = message.slice(12).split(' ');
  const agentName = parts[0];
  const description = parts.slice(1).join(' ');
  
  const response = await fetch('http://localhost:3000/api/command', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      command: `spawn agent ${agentName} ${description}`
    })
  });
  
  const result = await response.json();
  // Display agent details
}

// Parse agent status command
if (message.startsWith('agent status ')) {
  const agentName = message.slice(13);
  
  const response = await fetch(`http://localhost:3000/api/agents/${agentName}/status`);
  const status = await response.json();
  // Display evolution status
}
```

### Agents Panel (Optional)

```typescript
interface AgentStatus {
  name: string;
  cycle_number: number;
  tasks_completed: number;
  next_evolution_at: number;
  stm_entries: number;
}

function AgentsPanel({ agents }: { agents: AgentStatus[] }) {
  return (
    <div className="agents-panel">
      <h3>Active Agents</h3>
      {agents.map(agent => (
        <div key={agent.name} className="agent-card">
          <h4>{agent.name}</h4>
          <p>Evolution Cycle: {agent.cycle_number}</p>
          <p>Tasks: {agent.tasks_completed}</p>
          <p>Next Evolution: {agent.next_evolution_at}</p>
        </div>
      ))}
    </div>
  );
}
```

## API Endpoints

### POST `/api/agents/spawn`

Spawn a new agent with evolution support.

**Request**:
```json
{
  "name": "security-helper",
  "description": "Detects exploits and malware",
  "tier": "free",
  "evolution": {
    "ltm_access": "append_only",
    "inherited_skills": ["security_scan"],
    "playbook_template": "playbook_template.yaml",
    "evolution_interval": 10
  }
}
```

**Response**:
```json
{
  "id": "uuid",
  "name": "security-helper",
  "repo_url": "https://github.com/user/security-helper",
  "tier": "free",
  "evolution_enabled": true
}
```

### GET `/api/agents/:name/status`

Get agent evolution status.

**Response**:
```json
{
  "agent_name": "security-helper",
  "cycle_number": 3,
  "tasks_completed": 30,
  "next_evolution_at": 40,
  "stm_entries": 30
}
```

### POST `/api/agents/:name/task`

Record task completion and trigger evolution if needed.

**Request**:
```json
{
  "success": true,
  "feedback": "Detected T1027 obfuscation"
}
```

**Response**:
```json
{
  "should_evolve": true,
  "report": {
    "cycle_number": 4,
    "playbook_updates": ["Improve obfuscation detection"],
    "skills_learned": [],
    "accuracy_score": 0.85
  }
}
```

## Proactive Features

Sub-agents can send proactive messages back to Phoenix or user:

```rust
// In agent code
if new_exploit_pattern_detected {
    send_proactive_message(
        "New exploit pattern detected — recommend re-scan",
        "security-helper"
    );
}
```

## Future Enhancements

1. **Semantic Memory**: Use vector_kb for semantic recall of successful patterns
2. **Cross-Agent Learning**: Share learned patterns across agent swarm
3. **Hierarchical Evolution**: Parent agents guide child agent evolution
4. **Automated Skill Discovery**: LLM-driven skill generation from task patterns
5. **Evolution Metrics Dashboard**: Visualize agent improvement over time

## References

- [Agent Spawner](agent_spawner/src/lib.rs)
- [Autonomous Evolution Loop](autonomous_evolution_loop/src/lib.rs)
- [Neural Cortex Strata](neural_cortex_strata/src/lib.rs)
- [Skills System](skills/README.md)
- [MITRE ATT&CK](https://attack.mitre.org/)

## Support

For issues or questions:
1. Check logs in `./logs/sub_agent_evolution.log`
2. Verify playbook/skills files are valid YAML/JSON
3. Ensure evolution interval is reasonable (10-50 tasks)
4. Check Phoenix memory access permissions

---

**Implementation Status**: ✅ Complete

**Phase**: 27 - Sub-Agent Evolution

**Date**: 2026-01-22
