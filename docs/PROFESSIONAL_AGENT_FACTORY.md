# Professional Agent Factory Implementation

**Date**: 2026-01-23  
**Author**: Senior Rust Developer  
**Status**: Implemented

---

## Overview

This document describes the implementation of the Professional Agent Factory for the Dual-Brain Orchestration pattern. The Agent Factory spawns specialized sub-agents (Researcher, Coder, Manager) based on task type when operating in Professional CognitiveMode.

---

## Architecture

### 1. Agent Types

Three specialized professional agents with distinct system prompts:

#### Manager Agent
- **Role**: Default Professional persona and decision-maker
- **Capabilities**: Task analysis, agent orchestration, executive decision-making
- **Communication**: Concise, executive-level, outcome-focused

#### Researcher Agent
- **Role**: Specialized in finding facts and synthesizing data
- **Capabilities**: Web search, data synthesis, fact-checking, report generation
- **Communication**: Structured, evidence-based, analytical

#### Coder Agent
- **Role**: Specialized in writing performance-oriented Rust code
- **Capabilities**: Code generation, debugging, optimization, architecture design
- **Communication**: Technical, precise, code-focused

### 2. Router Pattern

The `route_professional_task()` function analyzes task descriptions and routes to the appropriate agent:

- **Coder**: Triggered by keywords like "code", "debug", "rust", "implement", "refactor", "optimize"
- **Researcher**: Triggered by keywords like "search", "find", "report", "research", "analyze"
- **Manager**: Default for all other tasks

Priority order: Coder > Researcher > Manager

### 3. State Isolation

**Critical Security Feature**: Professional mode enforces strict state isolation:

#### Blocked in Professional Mode:
- ❌ L4 (Semantic/Personal Memory)
- ❌ L5 (Procedural/Intimate Memory)
- ❌ Fantasy Dyad / relational adaptation logic
- ❌ Trust Score / relationship-based data access
- ❌ Girlfriend mode prompts
- ❌ Relationship phase context
- ❌ Astrological compatibility
- ❌ Sexual preferences / intimacy suggestions
- ❌ Vector KB semantic search for loving memories

#### Allowed in Professional Mode:
- ✅ L1-L3 (Working memory, episodic)
- ✅ Task-specific context
- ✅ System tool access
- ✅ Code analysis capabilities
- ✅ Technical documentation

---

## Implementation Details

### Files Modified

1. **`phoenix-web/src/professional_agents.rs`** (NEW)
   - Defines `ProfessionalAgentType` enum
   - Implements `system_prompt()` for each agent type
   - Implements `matches_task()` for routing logic
   - Implements `route_professional_task()` router function
   - Includes comprehensive unit tests

2. **`phoenix-web/src/handlers.rs`**
   - Added `spawn_professional_agent()` function
   - Added `build_professional_context()` function
   - Enforces state isolation for Professional mode

3. **`phoenix-web/src/main.rs`**
   - Added `mod professional_agents;` declaration
   - Modified `command_to_response_json()` to check cognitive mode
   - Routes Professional mode requests through Agent Factory
   - Wraps all relationship/intimate context in `if cognitive_mode == Personal` blocks
   - Blocks vector KB semantic search in Professional mode

### Key Functions

#### `spawn_professional_agent(task_description: &str, phoenix_name: &str)`
```rust
pub fn spawn_professional_agent(
    task_description: &str,
    phoenix_name: &str,
) -> (ProfessionalAgentType, String)
```
- Routes task to appropriate agent
- Returns agent type and system prompt with state isolation enforced

#### `build_professional_context(task_description: &str, cognitive_mode: CognitiveMode)`
```rust
pub fn build_professional_context(
    task_description: &str,
    cognitive_mode: CognitiveMode,
) -> Vec<String>
```
- Builds context with state isolation for Professional mode
- Only includes L1-L3 memory layers
- Adds mode reminder about efficiency and boundaries

---

## Usage

### Switching to Professional Mode

Users can switch to Professional mode via the Phoenix Identity Manager:

```rust
phoenix_identity.set_cognitive_mode(CognitiveMode::Professional).await;
```

### Example Task Routing

```rust
// Code task -> Coder agent
let task = "Write a Rust function to parse JSON";
let (agent_type, prompt) = spawn_professional_agent(task, "Phoenix");
// agent_type = ProfessionalAgentType::Coder

// Research task -> Researcher agent
let task = "Find information about Rust async patterns";
let (agent_type, prompt) = spawn_professional_agent(task, "Phoenix");
// agent_type = ProfessionalAgentType::Researcher

// General task -> Manager agent
let task = "Schedule a deployment for tomorrow";
let (agent_type, prompt) = spawn_professional_agent(task, "Phoenix");
// agent_type = ProfessionalAgentType::Manager
```

---

## Testing

### Unit Tests

The `professional_agents` module includes comprehensive unit tests:

```bash
cargo test --package phoenix-web --lib professional_agents
```

Tests cover:
- ✅ Coder task routing
- ✅ Researcher task routing
- ✅ Manager task routing (default)
- ✅ System prompts contain state isolation constraints
- ✅ System prompts block Fantasy Dyad
- ✅ System prompts block personal memory access

### Integration Testing

To test the full integration:

1. Set cognitive mode to Professional
2. Send various task types through `/api/command` endpoint
3. Verify appropriate agent is spawned (check logs)
4. Verify no L4/L5 memory is injected into context
5. Verify responses are professional and task-focused

---

## Security Guarantees

### State Isolation Enforcement

The implementation provides multiple layers of protection:

1. **System Prompt Level**: Each agent's system prompt explicitly states:
   - "NO access to personal memories (L4/L5 layers)"
   - "NO Fantasy Dyad or relational adaptation"
   - "NO emotional context or trust scores"

2. **Context Building Level**: `build_professional_context()` only includes:
   - Task description
   - Mode reminder
   - L1-L3 memory (working/episodic only)

3. **Command Processing Level**: `command_to_response_json()` wraps all relationship context in:
   ```rust
   if cognitive_mode == CognitiveMode::Personal {
       // relationship/intimate context here
   }
   ```

4. **Vector KB Level**: Semantic search for loving memories is blocked:
   ```rust
   if cognitive_mode == CognitiveMode::Personal {
       // vector KB semantic search here
   }
   ```

---

## Future Enhancements

### Potential Improvements

1. **Dynamic Agent Spawning**: Allow agents to spawn sub-agents for complex tasks
2. **Agent Specialization**: Add more specialized agents (e.g., SecurityAgent, DataAgent)
3. **Task Complexity Analysis**: Route based on task complexity, not just keywords
4. **Agent Performance Metrics**: Track which agents perform best for which tasks
5. **Multi-Agent Collaboration**: Allow multiple agents to collaborate on complex tasks
6. **Agent Learning**: Agents learn from successful task completions
7. **Custom Agent Templates**: Allow users to define custom agent types

### Integration with Swarm System

The Professional Agent Factory can be integrated with the existing Swarm System:

- Sola (Manager agent) can delegate tasks to ORCH agents
- ORCH agents can be specialized (Coder ORCH, Researcher ORCH)
- Task auctions can consider agent specialization
- Hidden swarm coordination remains transparent to user

---

## References

- **DUAL_BRAIN_ORCHESTRATION_AUDIT.md**: Existing architecture audit
- **phoenix_identity/src/lib.rs**: CognitiveMode enum definition
- **agent_spawner/src/lib.rs**: Agent spawning infrastructure
- **internal_bus.rs**: Swarm coordination system

---

## Conclusion

The Professional Agent Factory successfully implements the Dual-Brain pattern with strict state isolation. When in Professional mode:

- ✅ Specialized agents are spawned based on task type
- ✅ L4/L5 memory is NEVER injected into LLM context
- ✅ Fantasy Dyad and relational adaptation are disabled
- ✅ Trust Score and relationship context are blocked
- ✅ System prompts explicitly remind AI of professional boundaries

This ensures that Professional mode operates as a pure "Digital Twin" focused on efficiency, clarity, and task completion, while Personal mode maintains the full relationship-focused experience.
