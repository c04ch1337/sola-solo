//! Persistent task scheduler.
//!
//! Goals:
//! - Sled-backed persistence (tasks survive restart)
//! - In-memory tick loop that checks due tasks and marks them as executed
//! - Minimal surface area; actions are emitted as events for higher layers to handle
//!
//! ## Security Integration
//!
//! When the `sensory` feature is enabled, the scheduler can run periodic
//! presence scans to maintain the security state. The default interval is
//! 15 minutes, but this can be configured.

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use cron::Schedule;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::str::FromStr;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use uuid::Uuid;

const SLED_TREE: &str = "scheduled_tasks";

/// Default presence scan interval in minutes when sensory feature is enabled.
pub const DEFAULT_PRESENCE_SCAN_INTERVAL_MINS: i64 = 15;

/// A persisted scheduled task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: Uuid,
    pub name: String,
    /// Cron expression (e.g. "0 0 3 * * *").
    ///
    /// Note: we validate syntax at insert-time, but execution uses a simple `next_run_at` model.
    pub cron: String,
    /// Opaque action payload (e.g. JSON command for the agent).
    pub payload: serde_json::Value,

    pub created_at: DateTime<Utc>,
    pub next_run_at: DateTime<Utc>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub enabled: bool,

    /// If true, a missed run should be executed immediately on startup/tick.
    #[serde(default)]
    pub critical: bool,
}

/// Events emitted by the scheduler.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SchedulerEvent {
    Fired {
        task_id: Uuid,
        task_name: String,
        payload: serde_json::Value,
        fired_at: DateTime<Utc>,
    },
    MisfireDetected {
        task_id: Uuid,
        task_name: String,
        payload: serde_json::Value,
        scheduled_for: DateTime<Utc>,
        detected_at: DateTime<Utc>,
        critical: bool,
    },
    /// Presence scan event - triggers identity verification
    PresenceScanDue {
        scheduled_at: DateTime<Utc>,
        scan_type: PresenceScanType,
    },
    /// Security level changed event
    SecurityLevelChanged {
        old_level: u8,
        new_level: u8,
        reason: String,
        changed_at: DateTime<Utc>,
    },
}

/// Type of presence scan to perform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PresenceScanType {
    /// Regular periodic scan (every 15 minutes by default)
    Periodic,
    /// Hourly full scan with multi-factor verification
    HourlyFull,
    /// Triggered by security event (e.g., after unknown face detection)
    SecurityTriggered,
}

#[derive(Clone)]
pub struct Scheduler {
    db: Arc<sled::Db>,
}

impl Scheduler {
    pub fn new(db: Arc<sled::Db>) -> Self {
        Self { db }
    }

    fn tree(&self) -> Result<sled::Tree> {
        self.db
            .open_tree(SLED_TREE)
            .with_context(|| format!("open sled tree {SLED_TREE}"))
    }

    fn next_occurrence(cron: &str, now: DateTime<Utc>) -> DateTime<Utc> {
        Schedule::from_str(cron)
            .ok()
            .and_then(|s| s.upcoming(Utc).next())
            .unwrap_or_else(|| now + Duration::minutes(1))
    }

    /// Detect tasks with `next_run_at` in the past.
    ///
    /// - Emits a [`SchedulerEvent::MisfireDetected`]
    /// - If the task is marked `critical`, it is also executed immediately (fires `Fired`)
    /// - Otherwise the task is rescheduled to its next cron occurrence (no execution)
    fn check_for_misfires(&self) -> Result<Vec<(SchedulerEvent, Option<SchedulerEvent>)>> {
        let now = Utc::now();
        let tasks = self.list_tasks()?;

        let mut out = Vec::new();
        for mut task in tasks.into_iter() {
            if !task.enabled {
                continue;
            }
            if task.next_run_at >= now {
                continue;
            }

            let misfire = SchedulerEvent::MisfireDetected {
                task_id: task.id,
                task_name: task.name.clone(),
                payload: task.payload.clone(),
                scheduled_for: task.next_run_at,
                detected_at: now,
                critical: task.critical,
            };

            if task.critical {
                let fired = SchedulerEvent::Fired {
                    task_id: task.id,
                    task_name: task.name.clone(),
                    payload: task.payload.clone(),
                    fired_at: now,
                };
                task.last_run_at = Some(now);
                task.next_run_at = Self::next_occurrence(&task.cron, now);
                let _ = self.add_task(task);
                out.push((misfire, Some(fired)));
            } else {
                // Non-critical misfire: just reschedule.
                task.next_run_at = Self::next_occurrence(&task.cron, now);
                let _ = self.add_task(task);
                out.push((misfire, None));
            }
        }

        Ok(out)
    }

    pub fn add_task(&self, mut task: ScheduledTask) -> Result<ScheduledTask> {
        // Basic cron validation.
        Schedule::from_str(&task.cron)
            .map_err(|e| anyhow::anyhow!("invalid cron expression: {e}"))?;

        let tree = self.tree()?;

        // Ensure deterministic next_run_at if caller passed a time in the past.
        let now = Utc::now();
        if task.next_run_at < now {
            task.next_run_at = Self::next_occurrence(&task.cron, now);
        }

        let key = task.id.as_bytes();
        let val = serde_json::to_vec(&task).context("serialize task")?;
        tree.insert(key, val).context("insert task")?;
        tree.flush().ok();
        Ok(task)
    }

    pub fn get_task(&self, id: Uuid) -> Result<Option<ScheduledTask>> {
        let tree = self.tree()?;
        let key = id.as_bytes();
        let v = tree.get(key).context("get task")?;
        let Some(ivec) = v else { return Ok(None) };
        let task: ScheduledTask = serde_json::from_slice(&ivec).context("deserialize task")?;
        Ok(Some(task))
    }

    pub fn list_tasks(&self) -> Result<Vec<ScheduledTask>> {
        let tree = self.tree()?;
        let mut out = Vec::new();
        for item in tree.iter() {
            let (_k, v) = item.context("iter task")?;
            let task: ScheduledTask = serde_json::from_slice(&v).context("deserialize task")?;
            out.push(task);
        }
        out.sort_by_key(|t| t.next_run_at);
        Ok(out)
    }

    pub fn cancel_task(&self, id: Uuid) -> Result<bool> {
        let tree = self.tree()?;
        let existed = tree.remove(id.as_bytes()).context("remove task")?.is_some();
        tree.flush().ok();
        Ok(existed)
    }

    pub fn set_enabled(&self, id: Uuid, enabled: bool) -> Result<Option<ScheduledTask>> {
        let Some(mut task) = self.get_task(id)? else { return Ok(None) };
        task.enabled = enabled;
        self.add_task(task.clone())?;
        Ok(Some(task))
    }

    /// Start the scheduler tick loop.
    ///
    /// Emits [`SchedulerEvent`]s via the returned receiver.
    pub fn start(self, tick_every: Duration) -> Result<(mpsc::Receiver<SchedulerEvent>, JoinHandle<()>)> {
        let (tx, rx) = mpsc::channel::<SchedulerEvent>(64);
        let handle = tokio::spawn(async move {
            // Startup misfire recovery.
            if let Ok(events) = self.check_for_misfires() {
                for (misfire, fired_opt) in events {
                    let _ = tx.send(misfire).await;
                    if let Some(fired) = fired_opt {
                        let _ = tx.send(fired).await;
                    }
                }
            }

            let mut interval = tokio::time::interval(
                tick_every
                    .to_std()
                    .unwrap_or_else(|_| std::time::Duration::from_secs(60)),
            );

            loop {
                interval.tick().await;
                let now = Utc::now();

                // Best-effort tick. Errors are intentionally ignored to keep loop alive.
                let tasks = match self.list_tasks() {
                    Ok(t) => t,
                    Err(_) => continue,
                };

                for mut task in tasks.into_iter() {
                    if !task.enabled {
                        continue;
                    }
                    if task.next_run_at > now {
                        continue;
                    }

                    // Fire event.
                    let _ = tx
                        .send(SchedulerEvent::Fired {
                            task_id: task.id,
                            task_name: task.name.clone(),
                            payload: task.payload.clone(),
                            fired_at: now,
                        })
                        .await;

                    // Update last/next. For now, advance by 60s as a placeholder.
                    task.last_run_at = Some(now);
                    task.next_run_at = Self::next_occurrence(&task.cron, now);
                    let _ = self.add_task(task);
                }
            }
        });

        Ok((rx, handle))
    }

    /// Start the scheduler with presence scan support.
    ///
    /// This variant adds periodic presence scans for security verification.
    /// When the `sensory` feature is enabled, it will emit `PresenceScanDue`
    /// events at the configured interval (default: 15 minutes).
    ///
    /// # Arguments
    ///
    /// * `tick_every` - How often to check for due tasks
    /// * `presence_scan_interval` - How often to trigger presence scans (None = disabled)
    /// * `hourly_full_scan` - Whether to trigger a full MFA scan every hour
    pub fn start_with_presence_scan(
        self,
        tick_every: Duration,
        presence_scan_interval: Option<Duration>,
        hourly_full_scan: bool,
    ) -> Result<(mpsc::Receiver<SchedulerEvent>, JoinHandle<()>)> {
        let (tx, rx) = mpsc::channel::<SchedulerEvent>(64);
        
        let presence_interval = presence_scan_interval
            .unwrap_or_else(|| Duration::minutes(DEFAULT_PRESENCE_SCAN_INTERVAL_MINS));
        
        let handle = tokio::spawn(async move {
            // Startup misfire recovery.
            if let Ok(events) = self.check_for_misfires() {
                for (misfire, fired_opt) in events {
                    let _ = tx.send(misfire).await;
                    if let Some(fired) = fired_opt {
                        let _ = tx.send(fired).await;
                    }
                }
            }

            let tick_duration = tick_every
                .to_std()
                .unwrap_or_else(|_| std::time::Duration::from_secs(60));
            
            let presence_duration = presence_interval
                .to_std()
                .unwrap_or_else(|_| std::time::Duration::from_secs(900)); // 15 min default
            
            let hourly_duration = std::time::Duration::from_secs(3600); // 1 hour

            let mut task_interval = tokio::time::interval(tick_duration);
            let mut presence_interval = tokio::time::interval(presence_duration);
            let mut hourly_interval = tokio::time::interval(hourly_duration);
            
            // Track last presence scan for failure counting
            let mut consecutive_failures: u32 = 0;
            const MAX_CONSECUTIVE_FAILURES: u32 = 3;

            loop {
                tokio::select! {
                    _ = task_interval.tick() => {
                        let now = Utc::now();

                        // Best-effort tick. Errors are intentionally ignored to keep loop alive.
                        let tasks = match self.list_tasks() {
                            Ok(t) => t,
                            Err(_) => continue,
                        };

                        for mut task in tasks.into_iter() {
                            if !task.enabled {
                                continue;
                            }
                            if task.next_run_at > now {
                                continue;
                            }

                            // Fire event.
                            let _ = tx
                                .send(SchedulerEvent::Fired {
                                    task_id: task.id,
                                    task_name: task.name.clone(),
                                    payload: task.payload.clone(),
                                    fired_at: now,
                                })
                                .await;

                            // Update last/next.
                            task.last_run_at = Some(now);
                            task.next_run_at = Self::next_occurrence(&task.cron, now);
                            let _ = self.add_task(task);
                        }
                    }
                    
                    _ = presence_interval.tick() => {
                        let now = Utc::now();
                        eprintln!("[scheduler] Periodic presence scan due at {}", now);
                        
                        let _ = tx
                            .send(SchedulerEvent::PresenceScanDue {
                                scheduled_at: now,
                                scan_type: PresenceScanType::Periodic,
                            })
                            .await;
                    }
                    
                    _ = hourly_interval.tick(), if hourly_full_scan => {
                        let now = Utc::now();
                        eprintln!("[scheduler] Hourly full presence scan due at {}", now);
                        
                        let _ = tx
                            .send(SchedulerEvent::PresenceScanDue {
                                scheduled_at: now,
                                scan_type: PresenceScanType::HourlyFull,
                            })
                            .await;
                    }
                }
            }
        });

        Ok((rx, handle))
    }

    /// Create a presence scan task with the given interval.
    ///
    /// This is a convenience method for creating a scheduled task that
    /// triggers presence scans at a regular interval.
    pub fn create_presence_scan_task(&self, interval_mins: i64) -> Result<ScheduledTask> {
        let cron = format!("0 */{} * * * *", interval_mins); // Every N minutes
        let now = Utc::now();
        
        let task = ScheduledTask {
            id: Uuid::new_v4(),
            name: "presence_scan".to_string(),
            cron,
            payload: serde_json::json!({
                "action": "identify_presence",
                "scan_type": "periodic",
            }),
            created_at: now,
            next_run_at: now + Duration::minutes(interval_mins),
            last_run_at: None,
            enabled: true,
            critical: true, // Presence scans are critical for security
        };
        
        self.add_task(task)
    }

    /// Create an hourly full scan task for multi-factor verification.
    pub fn create_hourly_full_scan_task(&self) -> Result<ScheduledTask> {
        let now = Utc::now();
        
        let task = ScheduledTask {
            id: Uuid::new_v4(),
            name: "hourly_full_scan".to_string(),
            cron: "0 0 * * * *".to_string(), // Every hour at minute 0
            payload: serde_json::json!({
                "action": "identify_presence",
                "scan_type": "hourly_full",
                "require_mfa": true,
            }),
            created_at: now,
            next_run_at: now + Duration::hours(1),
            last_run_at: None,
            enabled: true,
            critical: true,
        };
        
        self.add_task(task)
    }
}

