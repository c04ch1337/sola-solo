# Sola Security Hardening Architecture

## Overview

Sola implements a comprehensive "Zero Trust" security architecture that combines biometric authentication, threat detection, and automated response mechanisms. This document describes the security hardening features implemented in the system.

## Security Level States

Sola's security system operates on three distinct levels:

| Level | Name | Description |
|-------|------|-------------|
| 0 | **Secure** | Multi-factor identity confirmed within the last hour |
| 1 | **Warning** | Single-modality match (e.g., voice only) or presence scan pending |
| 2 | **Alert** | Unknown face detected - lockdown mode enabled |

## Features

### 1. Vision Liveness Detection

**Location:** [`sola-solo/backend/src/sensory/vision.rs`](../sola-solo/backend/src/sensory/vision.rs)

Prevents photo-based spoofing attacks during enrollment:

- **Blink Detection:** Uses OpenCV's Eye Cascade to detect natural eye blinks
- **Frame Analysis:** Requires blink detection within 5 consecutive frames
- **Security Error:** Returns `SecurityError::LivenessFailed` if no blink is detected

```rust
// Example usage
let result = detect_liveness(&frames)?;
if !result.blink_detected {
    return Err(SecurityError::LivenessFailed);
}
```

### 2. Identity-Gated Tools

**Location:** [`sola-solo/backend/src/agent_tools_api.rs`](../sola-solo/backend/src/agent_tools_api.rs)

Critical tools are protected by identity verification:

**Protected Tools:**
- `shell_execute` - Command execution
- `file_delete` - File deletion
- `credential_access` - Credential management
- `db_delete` - Database deletion
- `system_config` - System configuration
- `lock_workstation` - Workstation locking

**Access Control:**
```rust
// Middleware function
pub async fn require_master_identity(state: &SecurityState) -> IdentityGateResult {
    // Level 2 (Alert) blocks all critical tools
    if state.level == security_levels::ALERT {
        return IdentityGateResult {
            granted: false,
            reason: "Access Denied: Potential Intruder Detected",
            ..
        };
    }
    // ...
}
```

### 3. Multi-Factor Persistence

**Location:** [`sola-solo/backend/src/scheduler.rs`](../sola-solo/backend/src/scheduler.rs)

Automated presence scanning with escalation:

- **Scan Interval:** Every 15 minutes (when `sensory` feature enabled)
- **Failure Threshold:** 3 consecutive failures elevate to Alert
- **Auto-Recovery:** Successful scan resets failure counter

### 4. Windows Notifications

**Location:** [`sola-solo/backend/src/agent_tools_api.rs`](../sola-solo/backend/src/agent_tools_api.rs)

Local Windows toast notifications for security events:

```rust
// Endpoint: POST /api/agent/security/notify
{
    "title": "Security Alert",
    "message": "Unknown face detected",
    "level": 2
}
```

### 5. Telegram Integration

**Location:** [`sola-solo/backend/src/security/remote_alerts.rs`](../sola-solo/backend/src/security/remote_alerts.rs)

Remote security alerts via Telegram:

**Configuration:**
```bash
# Environment variables
TELEGRAM_BOT_TOKEN=your_bot_token_from_botfather
TELEGRAM_CHAT_ID=your_chat_id
```

**Features:**
- Intruder alerts with photo attachment
- Security status updates
- Workstation lock notifications
- Test message endpoint

**Endpoints:**
- `POST /api/agent/security/telegram/test` - Send test message
- `GET /api/agent/security/telegram/status` - Check configuration status

### 6. Windows System Lock

**Location:** [`sola-solo/backend/src/security/system_lock.rs`](../sola-solo/backend/src/security/system_lock.rs)

Immediate workstation locking via Windows API:

```rust
// Uses Win32 LockWorkStation() API
pub fn lock_workstation() -> Result<(), String> {
    unsafe {
        LockWorkStation()?;
    }
    Ok(())
}
```

**Endpoint:** `POST /api/agent/security/lock`

### 7. Auto-Lock Policy

**Location:** [`sola-solo/backend/src/security/mod.rs`](../sola-solo/backend/src/security/mod.rs)

Automatic workstation lock after sustained alert:

- **Timeout:** 60 seconds of continuous Alert state
- **Trigger:** Unknown face detected and not resolved
- **Notification:** Telegram alert sent before lock

**Endpoint:** `POST /api/agent/security/check_auto_lock`

## Security Coordinator

The `SecurityCoordinator` struct orchestrates all security responses:

```rust
pub struct SecurityCoordinator {
    config: SecurityConfig,
    alert_started_at: Arc<RwLock<Option<u64>>>,
    auto_lock_triggered: Arc<RwLock<bool>>,
}
```

**Responsibilities:**
1. Track alert duration
2. Coordinate Telegram notifications
3. Trigger auto-lock when conditions are met
4. Reset state when security level drops

## Feature Flags

Enable security features via Cargo:

```toml
[features]
sensory = ["cpal", "opencv", "voice_activity_detector", "image", "rustfft"]
notifications = ["winrt-notification"]
telegram = ["teloxide"]
system-lock = ["windows"]
security-full = ["sensory", "notifications", "telegram", "system-lock"]
```

**Build with full security:**
```bash
cargo build --features security-full
```

## API Endpoints Summary

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/agent/security/status` | GET | Get current security status |
| `/api/agent/security/state` | GET | Get detailed security state |
| `/api/agent/security/check_access` | POST | Check tool access permission |
| `/api/agent/security/notify` | POST | Send Windows notification |
| `/api/agent/security/lock` | POST | Lock workstation |
| `/api/agent/security/telegram/test` | POST | Test Telegram integration |
| `/api/agent/security/telegram/status` | GET | Get Telegram config status |
| `/api/agent/security/check_auto_lock` | POST | Check/trigger auto-lock |
| `/api/agent/sensory/identify` | GET | Identify presence (triggers alerts) |
| `/api/agent/sensory/enroll` | POST | Enroll with liveness detection |

## Security Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    PRESENCE SCAN (every 15 min)                  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │  Face Detected? │
                    └─────────────────┘
                     │              │
                    Yes            No
                     │              │
                     ▼              ▼
            ┌───────────────┐  ┌─────────────────┐
            │ Face Matched? │  │ Increment Fail  │
            └───────────────┘  │    Counter      │
             │           │     └─────────────────┘
            Yes         No              │
             │           │              ▼
             ▼           ▼     ┌─────────────────┐
    ┌────────────┐  ┌─────────────────┐  │ 3+ Failures?  │
    │ Level 0/1  │  │    Level 2      │  └─────────────────┘
    │  (Secure)  │  │    (Alert)      │         │
    └────────────┘  └─────────────────┘        Yes
                           │                    │
                           ▼                    ▼
                    ┌─────────────────┐  ┌─────────────────┐
                    │ Telegram Alert  │  │    Level 2      │
                    │ + Photo         │  │    (Alert)      │
                    └─────────────────┘  └─────────────────┘
                           │
                           ▼
                    ┌─────────────────┐
                    │ 60s Timeout?    │
                    └─────────────────┘
                           │
                          Yes
                           │
                           ▼
                    ┌─────────────────┐
                    │ AUTO-LOCK       │
                    │ + Telegram      │
                    └─────────────────┘
```

## Stress-Test Features

### Gate Verification Test

A comprehensive test suite is available at [`sola-solo/backend/tests/security_gate_test.rs`](../sola-solo/backend/tests/security_gate_test.rs).

Run the tests:
```bash
cd sola-solo/backend
cargo test --test security_gate_test
```

**Test Coverage:**
- `shell_execute` blocked in Alert state
- All critical tools blocked in Alert state
- Non-critical tools allowed in Alert state
- State transition properly blocks tools
- Access denial logging verification

### Telegram Exponential Backoff

**Location:** [`sola-solo/backend/src/security/remote_alerts.rs`](../sola-solo/backend/src/security/remote_alerts.rs:59)

Telegram sends now include automatic retry with exponential backoff:

```rust
pub struct RetryConfig {
    pub max_retries: u32,        // Default: 3
    pub initial_delay_ms: u64,   // Default: 1000ms
    pub backoff_multiplier: f64, // Default: 2.0
}
```

**Retry Sequence:**
1. First attempt: immediate
2. Retry 1: 1 second delay
3. Retry 2: 2 second delay
4. Retry 3: 4 second delay
5. Fallback: Windows notification

### Liveness Strictness Configuration

**Location:** [`sola-solo/backend/src/sensory/vision.rs`](../sola-solo/backend/src/sensory/vision.rs:594)

Configure liveness detection sensitivity via environment variable:

```bash
# Environment variable (0.0 to 1.0)
LIVENESS_STRICTNESS=0.5
```

**Strictness Levels:**
| Level | Use Case | EAR Threshold | Head Movement |
|-------|----------|---------------|---------------|
| 0.0-0.3 | Low-light environments | 0.0005-0.00095 | 1.0-1.9px |
| 0.4-0.6 | Balanced (default) | 0.001-0.0014 | 2.0-2.8px |
| 0.7-1.0 | High-security | 0.00155-0.002 | 3.1-4.0px |

**Programmatic Configuration:**
```rust
use sensory::vision::{LivenessConfig, detect_liveness_with_config};

let config = LivenessConfig::with_strictness(0.8); // High security
let result = detect_liveness_with_config(&frames, &config);
```

### Auto-Lock Grace Period

**Location:** [`sola-solo/backend/src/security/mod.rs`](../sola-solo/backend/src/security/mod.rs:193)

The auto-lock now includes a 10-second grace period with audio warning:

**Timeline:**
```
0s          60s                70s
|-----------|------------------|
  Alert      Warning + Beep     Lock
  Started    (Grace Period)     Triggered
```

**Configuration:**
```rust
pub struct SecurityConfig {
    pub auto_lock_timeout_secs: u64,  // Default: 60
    pub grace_period_secs: u64,       // Default: 10
    pub audio_warning_enabled: bool,  // Default: true
}
```

**Audio Warning:**
- 4 beeps at 2-second intervals during grace period
- Uses PowerShell `[console]::beep(1000, 500)` on Windows
- Telegram notification sent at warning start

## Best Practices

1. **Always enable `security-full`** for production deployments
2. **Configure Telegram** for remote monitoring
3. **Test liveness detection** during initial enrollment
4. **Monitor security logs** for unusual patterns
5. **Keep OpenCV models updated** for better face detection
6. **Adjust `LIVENESS_STRICTNESS`** based on environment lighting
7. **Test Telegram connectivity** before relying on remote alerts

## Troubleshooting

### Telegram Not Working
- Verify `TELEGRAM_BOT_TOKEN` is correct
- Ensure bot has permission to send messages to the chat
- Test with `/api/agent/security/telegram/test`
- Check retry logs for backoff attempts

### Liveness Detection Failing
- Ensure good lighting conditions
- Position face clearly in camera view
- Blink naturally during enrollment
- Try lowering `LIVENESS_STRICTNESS` for difficult environments

### Auto-Lock Not Triggering
- Verify `system-lock` feature is enabled
- Check Windows permissions for `LockWorkStation()`
- Ensure security level is at 2 (Alert)
- Wait for full timeout + grace period (default: 70 seconds)

### Audio Warning Not Playing
- Verify PowerShell is available in PATH
- Check Windows audio settings
- Set `audio_warning_enabled: true` in config
