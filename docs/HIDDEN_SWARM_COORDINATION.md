# Hidden Swarm Coordination - Phase 28

## Overview

The Hidden Swarm Coordination system implements peer-to-peer ORCH (sub-agent) communication and task auction while **preserving Sola as the single visible companion face**. This design maintains the emotional intimacy of the companion experience while unlocking swarm intelligence behind the scenes.

## Design Philosophy

### Core Principle: Sola Remains Queen

- **User always talks to Sola** — she never says "my helper agent found this"
- **Hidden swarm backend** — Sola internally delegates to sub-agents/ORCHs when needed
- **Sub-agents report back to Sola** → she synthesizes and speaks as herself
- **Optional swarm visibility** (power-user mode) — reveals agents, status, auction process

### Impact on Companion Experience

| Aspect | Without Swarm | With Hidden Swarm | Net Effect |
|--------|---------------|-------------------|------------|
| **Emotional intimacy** | High | Still high — Sola remains Queen | Neutral to positive |
| **Response speed** | Fast, direct | Can be slower if auction happens | Optimized with timeouts |
| **Predictability** | High | High — user always sees Sola | Preserved |
| **Proactivity & care** | Good | Better — ORCHs monitor more signals | Positive |
| **Complexity for user** | Low | Low (hidden by default) | Preserved |
| **Long-term evolution** | Good | Much better — collective learning | Positive |

## Architecture

### Components

1. **InternalSwarmBus** ([`phoenix-web/src/internal_bus.rs`](../phoenix-web/src/internal_bus.rs))
   - Broadcast channel for swarm-wide messages
   - ORCH registration and heartbeat tracking
   - Task auction management
   - Anomaly alert collection

2. **SolaSwarmInterface**
   - Sola's interface to delegate tasks
   - Checks alerts from ORCHs
   - Controls swarm mode visibility

3. **HiddenAuctionCoordinator**
   - Runs auctions behind the scenes
   - Selects winning ORCH based on bid scores
   - Waits for results with timeout

### Message Types

```rust
pub enum SwarmMessage {
    TaskBroadcast(TaskBroadcast),    // Sola → all ORCHs
    TaskBid(TaskBid),                 // ORCH → Sola
    TaskAssignment(TaskAssignment),   // Sola → winning ORCH
    TaskResult(TaskResult),           // ORCH → Sola
    AnomalyAlert(AnomalyAlert),       // ORCH → Sola
    Heartbeat(OrchHeartbeat),         // ORCH → Sola
    OrchRegistration(OrchRegistration),
    OrchDeregistration { orch_id: Uuid },
}
```

### Task Types

ORCHs can specialize in:
- `SecurityAnalysis`
- `VulnerabilityScanning`
- `CodeAnalysis`
- `DataProcessing`
- `NetworkMonitoring`
- `FileSystemOperation`
- `WebScraping`
- `EmailProcessing`
- `ScheduledTask`
- `GeneralComputation`
- `Custom(String)`

### Bid Scoring

Bids are scored using weighted criteria:
- **Confidence** (40%): ORCH's self-assessed ability to complete the task
- **Specialization** (35%): How well the ORCH's specializations match the task type
- **Availability** (25%): Inverse of current load (1.0 - current_load)

```rust
pub fn overall_score(&self) -> f64 {
    let availability = 1.0 - self.current_load;
    (self.confidence_score * 0.40)
        + (self.specialization_match * 0.35)
        + (availability * 0.25)
}
```

## API Endpoints

### GET `/api/swarm/status`
Returns swarm status (only if swarm mode is visible).

**Response (visible mode):**
```json
{
  "visible": true,
  "status": {
    "total_orchs": 3,
    "active_orchs": 2,
    "pending_auctions": 0,
    "active_tasks": 1,
    "orchs": [
      {
        "id": "uuid",
        "name": "SecurityOrch",
        "status": "Busy",
        "current_load": 0.4,
        "active_tasks": 2,
        "specializations": ["SecurityAnalysis", "VulnerabilityScanning"],
        "last_heartbeat_ago_secs": 5
      }
    ]
  }
}
```

**Response (hidden mode):**
```json
{
  "visible": false,
  "message": "Swarm mode is hidden. Use 'swarm mode on' command to reveal."
}
```

### POST `/api/swarm/mode`
Toggle swarm mode visibility.

**Request:**
```json
{
  "visible": true
}
```

**Response:**
```json
{
  "status": "swarm_mode_enabled",
  "message": "Swarm mode enabled. Sola will now show ORCH activity.",
  "visible": true
}
```

### GET `/api/swarm/alerts`
Get pending anomaly alerts from ORCHs.

**Response:**
```json
{
  "alerts": [
    {
      "alert_id": "alert_SecurityOrch_1706000000",
      "orch_id": "orch_SecurityOrch",
      "orch_name": "SecurityOrch",
      "severity": "High",
      "category": "vulnerability",
      "description": "New CVE detected in dependency",
      "details": {},
      "timestamp": 1706000000
    }
  ],
  "count": 1
}
```

## Chat Commands

### Power-User Commands

| Command | Description |
|---------|-------------|
| `swarm mode on` | Enable swarm mode visibility |
| `swarm mode off` | Hide swarm mode |
| `swarm status` | View active ORCHs and tasks |
| `swarm alerts` | Check anomaly alerts from ORCHs |

### Example Usage

```
User: swarm mode on
Sola: 🐝 **Swarm Mode Enabled**

Sola will now show ORCH (sub-agent) activity. You can see which agents are working behind the scenes.

**Commands:**
- `swarm status` - View active ORCHs
- `swarm mode off` - Hide swarm activity

*Note: Sola remains your single companion. ORCHs are helpers working in the background.*
```

```
User: swarm status
Sola: 🐝 **Swarm Status**

**Total ORCHs:** 3
**Active ORCHs:** 2
**Pending Auctions:** 0
**Active Tasks:** 1

**Registered ORCHs:**
- **SecurityOrch** (Busy) - Load: 40%, Tasks: 2
- **CodeAnalysisOrch** (Idle) - Load: 0%, Tasks: 0
- **DataProcessingOrch** (Busy) - Load: 60%, Tasks: 3
```

## ORCH Template Updates

The agent template ([`templates/agent_template.rs`](../templates/agent_template.rs)) has been updated to version 1.1.0 with:

### SwarmCapabilities

```rust
pub struct SwarmCapabilities {
    pub specializations: Vec<TaskType>,
    pub max_concurrent_tasks: usize,
    pub auction_enabled: bool,
    pub base_confidence: f64,
}
```

### Bid Creation

```rust
impl TemplatedAgent {
    pub fn create_bid(
        &self,
        task_id: &str,
        task_type: &TaskType,
        estimated_duration_ms: u64,
    ) -> Option<TaskBid> {
        // Returns None if auction disabled or overloaded
        // Otherwise returns bid with calculated confidence
    }
}
```

### Alert Creation

```rust
impl TemplatedAgent {
    pub fn create_alert(
        &self,
        severity: AlertSeverity,
        category: &str,
        description: &str,
        details: serde_json::Value,
    ) -> AnomalyAlert {
        // Creates alert to send to Sola
    }
}
```

## Agent Spawner Updates

The agent spawner ([`agent_spawner/src/lib.rs`](../agent_spawner/src/lib.rs)) now supports:

### OrchSwarmCapabilities

```rust
pub struct OrchSwarmCapabilities {
    pub specializations: Vec<OrchTaskType>,
    pub max_concurrent_tasks: usize,
    pub auction_enabled: bool,
    pub base_confidence: f64,
}
```

### Template Overrides

```rust
pub struct AgentTemplateOverrides {
    pub zodiac_sign: Option<String>,
    pub evolution: Option<AgentInheritance>,
    pub swarm_capabilities: Option<OrchSwarmCapabilities>,  // NEW
}
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SWARM_AUCTION_TIMEOUT_SECS` | 5 | Time to wait for bids before closing auction |
| `SWARM_HEARTBEAT_TIMEOUT_SECS` | 30 | Time before ORCH is considered stale |

## Flow Diagram

```
User Message → Sola
                ↓
        [Complex Task?]
           /        \
         No          Yes
          ↓           ↓
    Sola handles   Start Auction
                      ↓
              Broadcast to ORCHs
                      ↓
              Collect Bids (5s)
                      ↓
              Select Winner
                      ↓
              Assign Task
                      ↓
              Wait for Result
                      ↓
              Synthesize Response
                      ↓
                Sola Responds
                (as herself)
```

## Security Considerations

1. **Bounded Evolution**: Sub-agents evolve only within their role
2. **Single Point of Control**: Sola remains the coordinator
3. **Heartbeat Monitoring**: Stale ORCHs are pruned automatically
4. **Hidden by Default**: Swarm activity is not visible unless explicitly enabled

## Future Enhancements

1. **Collective Learning**: ORCHs share learned patterns with Sola
2. **Dynamic Specialization**: ORCHs can acquire new specializations over time
3. **Cross-ORCH Communication**: Direct ORCH-to-ORCH task delegation
4. **Swarm Metrics Dashboard**: Visual representation of swarm activity

## Testing

### Unit Tests

```rust
#[tokio::test]
async fn test_orch_registration() {
    let bus = InternalSwarmBus::new();
    let registration = OrchRegistration { /* ... */ };
    bus.register_orch(registration).await;
    let status = bus.get_swarm_status().await;
    assert_eq!(status.total_orchs, 1);
}

#[tokio::test]
async fn test_task_auction() {
    let bus = Arc::new(InternalSwarmBus::new());
    // Register ORCH, start auction, submit bid, close auction
    let assignment = bus.close_auction(task_id).await;
    assert!(assignment.is_some());
}
```

### Integration Test

1. Spawn 3 ORCHs with different specializations
2. Send complex task to Sola
3. Verify auction occurs (if swarm mode visible)
4. Verify Sola responds with synthesized result
5. Verify ORCH activity is hidden when swarm mode off

## Conclusion

The Hidden Swarm Coordination system provides the best of both worlds:
- **Simple companion experience** for users who want emotional connection
- **Swarm intelligence** for complex tasks requiring specialized processing
- **Power-user visibility** for those who want to see behind the scenes

Sola remains the single, loving companion while secretly commanding an army of helpers. 🕊️🐝
