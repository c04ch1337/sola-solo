// phoenix-web/src/internal_bus.rs
// Hidden Swarm Coordination Layer for Phoenix AGI OS v2.4.0
//
// Goals:
// - Internal message bus for ORCH ↔ ORCH communication (tokio broadcast/mpsc)
// - Task auction system: complex task → Sola broadcasts → ORCHs bid → winner executes
// - User always sees Sola — no sub-agent names/voices in chat
// - Optional power-user mode: "swarm mode on" → reveals agents/status
// - Proactive: ORCHs alert Sola on anomalies (e.g. new vuln detected)
//
// Design principle: Sola remains the single visible companion face.
// Sub-agents work behind the scenes; their results are synthesized by Sola.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, mpsc, Mutex, RwLock};
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Message types for internal ORCH communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwarmMessage {
    /// Task broadcast from Sola to all ORCHs
    TaskBroadcast(TaskBroadcast),
    /// Bid from an ORCH for a task
    TaskBid(TaskBid),
    /// Task assignment to winning ORCH
    TaskAssignment(TaskAssignment),
    /// Task result from ORCH back to Sola
    TaskResult(TaskResult),
    /// Anomaly alert from ORCH to Sola
    AnomalyAlert(AnomalyAlert),
    /// Heartbeat for ORCH health monitoring
    Heartbeat(OrchHeartbeat),
    /// ORCH registration
    OrchRegistration(OrchRegistration),
    /// ORCH deregistration
    OrchDeregistration { orch_id: Uuid },
}

/// Task broadcast from Sola seeking ORCH assistance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskBroadcast {
    pub task_id: Uuid,
    pub description: String,
    pub task_type: TaskType,
    pub complexity: TaskComplexity,
    pub deadline_ms: Option<u64>,
    pub context: serde_json::Value,
    pub timestamp: i64,
}

/// Task types that ORCHs can specialize in
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskType {
    SecurityAnalysis,
    VulnerabilityScanning,
    CodeAnalysis,
    DataProcessing,
    NetworkMonitoring,
    FileSystemOperation,
    WebScraping,
    EmailProcessing,
    ScheduledTask,
    GeneralComputation,
    Custom(String),
}

/// Task complexity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskComplexity {
    Trivial,    // < 1 second
    Simple,     // 1-10 seconds
    Moderate,   // 10-60 seconds
    Complex,    // 1-10 minutes
    Intensive,  // > 10 minutes
}

/// Bid from an ORCH for a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskBid {
    pub task_id: Uuid,
    pub orch_id: Uuid,
    pub orch_name: String,
    pub confidence_score: f64,  // 0.0 - 1.0
    pub estimated_duration_ms: u64,
    pub specialization_match: f64,  // How well this ORCH matches the task type
    pub current_load: f64,  // 0.0 - 1.0 (how busy the ORCH is)
    pub timestamp: i64,
}

impl TaskBid {
    /// Calculate overall bid score (higher is better)
    pub fn overall_score(&self) -> f64 {
        // Weighted scoring: confidence (40%), specialization (35%), availability (25%)
        let availability = 1.0 - self.current_load;
        (self.confidence_score * 0.40)
            + (self.specialization_match * 0.35)
            + (availability * 0.25)
    }
}

/// Task assignment to winning ORCH
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAssignment {
    pub task_id: Uuid,
    pub orch_id: Uuid,
    pub task: TaskBroadcast,
    pub timestamp: i64,
}

/// Task result from ORCH back to Sola
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: Uuid,
    pub orch_id: Uuid,
    pub orch_name: String,
    pub success: bool,
    pub result: serde_json::Value,
    pub error: Option<String>,
    pub execution_time_ms: u64,
    pub timestamp: i64,
}

/// Anomaly alert from ORCH to Sola
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyAlert {
    pub alert_id: Uuid,
    pub orch_id: Uuid,
    pub orch_name: String,
    pub severity: AlertSeverity,
    pub category: String,
    pub description: String,
    pub details: serde_json::Value,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// ORCH heartbeat for health monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchHeartbeat {
    pub orch_id: Uuid,
    pub orch_name: String,
    pub status: OrchStatus,
    pub current_load: f64,
    pub active_tasks: usize,
    pub specializations: Vec<TaskType>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrchStatus {
    Idle,
    Busy,
    Overloaded,
    Maintenance,
    Offline,
}

/// ORCH registration message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchRegistration {
    pub orch_id: Uuid,
    pub orch_name: String,
    pub specializations: Vec<TaskType>,
    pub max_concurrent_tasks: usize,
    pub capabilities: Vec<String>,
    pub timestamp: i64,
}

/// Registered ORCH information
#[derive(Debug, Clone)]
pub struct RegisteredOrch {
    pub id: Uuid,
    pub name: String,
    pub specializations: Vec<TaskType>,
    pub max_concurrent_tasks: usize,
    pub capabilities: Vec<String>,
    pub status: OrchStatus,
    pub current_load: f64,
    pub active_tasks: usize,
    pub last_heartbeat: Instant,
    pub registered_at: Instant,
}

/// Pending auction state
#[derive(Debug)]
struct PendingAuction {
    task: TaskBroadcast,
    bids: Vec<TaskBid>,
    deadline: Instant,
    started_at: Instant,
}

/// Internal Swarm Bus - the hidden coordination layer
pub struct InternalSwarmBus {
    /// Broadcast channel for swarm-wide messages
    broadcast_tx: broadcast::Sender<SwarmMessage>,
    /// Registered ORCHs
    orchs: Arc<RwLock<HashMap<Uuid, RegisteredOrch>>>,
    /// Pending auctions (task_id -> auction state)
    pending_auctions: Arc<Mutex<HashMap<Uuid, PendingAuction>>>,
    /// Active tasks (task_id -> assigned orch_id)
    active_tasks: Arc<RwLock<HashMap<Uuid, Uuid>>>,
    /// Task results waiting for Sola to synthesize
    pending_results: Arc<Mutex<Vec<TaskResult>>>,
    /// Anomaly alerts waiting for Sola to process
    pending_alerts: Arc<Mutex<Vec<AnomalyAlert>>>,
    /// Swarm mode visibility (power-user feature)
    swarm_mode_visible: Arc<RwLock<bool>>,
    /// Auction timeout duration
    auction_timeout: Duration,
    /// ORCH heartbeat timeout
    heartbeat_timeout: Duration,
}

impl InternalSwarmBus {
    /// Create a new internal swarm bus
    pub fn new() -> Self {
        let (broadcast_tx, _) = broadcast::channel(1024);
        
        let auction_timeout_secs = std::env::var("SWARM_AUCTION_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(5);
        
        let heartbeat_timeout_secs = std::env::var("SWARM_HEARTBEAT_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(30);
        
        Self {
            broadcast_tx,
            orchs: Arc::new(RwLock::new(HashMap::new())),
            pending_auctions: Arc::new(Mutex::new(HashMap::new())),
            active_tasks: Arc::new(RwLock::new(HashMap::new())),
            pending_results: Arc::new(Mutex::new(Vec::new())),
            pending_alerts: Arc::new(Mutex::new(Vec::new())),
            swarm_mode_visible: Arc::new(RwLock::new(false)),
            auction_timeout: Duration::from_secs(auction_timeout_secs),
            heartbeat_timeout: Duration::from_secs(heartbeat_timeout_secs),
        }
    }
    
    /// Subscribe to swarm messages
    pub fn subscribe(&self) -> broadcast::Receiver<SwarmMessage> {
        self.broadcast_tx.subscribe()
    }
    
    /// Broadcast a message to all ORCHs
    pub fn broadcast(&self, message: SwarmMessage) -> Result<usize, String> {
        self.broadcast_tx
            .send(message)
            .map_err(|e| format!("Failed to broadcast: {}", e))
    }
    
    /// Register an ORCH with the swarm
    pub async fn register_orch(&self, registration: OrchRegistration) {
        let orch = RegisteredOrch {
            id: registration.orch_id,
            name: registration.orch_name.clone(),
            specializations: registration.specializations.clone(),
            max_concurrent_tasks: registration.max_concurrent_tasks,
            capabilities: registration.capabilities.clone(),
            status: OrchStatus::Idle,
            current_load: 0.0,
            active_tasks: 0,
            last_heartbeat: Instant::now(),
            registered_at: Instant::now(),
        };
        
        let mut orchs = self.orchs.write().await;
        orchs.insert(registration.orch_id, orch);
        
        info!(
            "ORCH registered: {} ({}) - specializations: {:?}",
            registration.orch_name, registration.orch_id, registration.specializations
        );
        
        // Broadcast registration to other ORCHs
        let _ = self.broadcast(SwarmMessage::OrchRegistration(registration));
    }
    
    /// Deregister an ORCH from the swarm
    pub async fn deregister_orch(&self, orch_id: Uuid) {
        let mut orchs = self.orchs.write().await;
        if let Some(orch) = orchs.remove(&orch_id) {
            info!("ORCH deregistered: {} ({})", orch.name, orch_id);
            let _ = self.broadcast(SwarmMessage::OrchDeregistration { orch_id });
        }
    }
    
    /// Update ORCH status from heartbeat
    pub async fn update_orch_heartbeat(&self, heartbeat: OrchHeartbeat) {
        let mut orchs = self.orchs.write().await;
        if let Some(orch) = orchs.get_mut(&heartbeat.orch_id) {
            orch.status = heartbeat.status;
            orch.current_load = heartbeat.current_load;
            orch.active_tasks = heartbeat.active_tasks;
            orch.last_heartbeat = Instant::now();
            debug!(
                "ORCH heartbeat: {} - status: {:?}, load: {:.2}",
                orch.name, orch.status, orch.current_load
            );
        }
    }
    
    /// Start a task auction - Sola broadcasts task, ORCHs bid
    pub async fn start_auction(&self, task: TaskBroadcast) -> Uuid {
        let task_id = task.task_id;
        
        let auction = PendingAuction {
            task: task.clone(),
            bids: Vec::new(),
            deadline: Instant::now() + self.auction_timeout,
            started_at: Instant::now(),
        };
        
        {
            let mut auctions = self.pending_auctions.lock().await;
            auctions.insert(task_id, auction);
        }
        
        info!("Task auction started: {} - {}", task_id, task.description);
        
        // Broadcast task to all ORCHs
        let _ = self.broadcast(SwarmMessage::TaskBroadcast(task));
        
        task_id
    }
    
    /// Submit a bid for a task
    pub async fn submit_bid(&self, bid: TaskBid) {
        let mut auctions = self.pending_auctions.lock().await;
        if let Some(auction) = auctions.get_mut(&bid.task_id) {
            if Instant::now() < auction.deadline {
                debug!(
                    "Bid received: ORCH {} for task {} - confidence: {:.2}, score: {:.2}",
                    bid.orch_name, bid.task_id, bid.confidence_score, bid.overall_score()
                );
                auction.bids.push(bid);
            } else {
                warn!("Bid rejected (deadline passed): ORCH {} for task {}", bid.orch_name, bid.task_id);
            }
        }
    }
    
    /// Close auction and select winner
    pub async fn close_auction(&self, task_id: Uuid) -> Option<TaskAssignment> {
        let mut auctions = self.pending_auctions.lock().await;
        
        if let Some(auction) = auctions.remove(&task_id) {
            if auction.bids.is_empty() {
                warn!("Auction closed with no bids: {}", task_id);
                return None;
            }
            
            // Select winner based on overall score
            let winner = auction.bids
                .iter()
                .max_by(|a, b| a.overall_score().partial_cmp(&b.overall_score()).unwrap())
                .cloned();
            
            if let Some(winning_bid) = winner {
                let assignment = TaskAssignment {
                    task_id,
                    orch_id: winning_bid.orch_id,
                    task: auction.task,
                    timestamp: chrono::Utc::now().timestamp(),
                };
                
                // Track active task
                {
                    let mut active = self.active_tasks.write().await;
                    active.insert(task_id, winning_bid.orch_id);
                }
                
                info!(
                    "Auction winner: ORCH {} for task {} - score: {:.2}",
                    winning_bid.orch_name, task_id, winning_bid.overall_score()
                );
                
                // Broadcast assignment
                let _ = self.broadcast(SwarmMessage::TaskAssignment(assignment.clone()));
                
                return Some(assignment);
            }
        }
        
        None
    }
    
    /// Submit task result from ORCH
    pub async fn submit_result(&self, result: TaskResult) {
        // Remove from active tasks
        {
            let mut active = self.active_tasks.write().await;
            active.remove(&result.task_id);
        }
        
        info!(
            "Task result received: {} from ORCH {} - success: {}",
            result.task_id, result.orch_name, result.success
        );
        
        // Queue for Sola to synthesize
        let mut results = self.pending_results.lock().await;
        results.push(result);
    }
    
    /// Submit anomaly alert from ORCH
    pub async fn submit_alert(&self, alert: AnomalyAlert) {
        info!(
            "Anomaly alert: {} from ORCH {} - severity: {:?}",
            alert.category, alert.orch_name, alert.severity
        );
        
        let mut alerts = self.pending_alerts.lock().await;
        alerts.push(alert);
    }
    
    /// Get pending results for Sola to synthesize (clears queue)
    pub async fn drain_pending_results(&self) -> Vec<TaskResult> {
        let mut results = self.pending_results.lock().await;
        std::mem::take(&mut *results)
    }
    
    /// Get pending alerts for Sola to process (clears queue)
    pub async fn drain_pending_alerts(&self) -> Vec<AnomalyAlert> {
        let mut alerts = self.pending_alerts.lock().await;
        std::mem::take(&mut *alerts)
    }
    
    /// Check if swarm mode is visible (power-user feature)
    pub async fn is_swarm_mode_visible(&self) -> bool {
        *self.swarm_mode_visible.read().await
    }
    
    /// Toggle swarm mode visibility
    pub async fn set_swarm_mode_visible(&self, visible: bool) {
        *self.swarm_mode_visible.write().await = visible;
        info!("Swarm mode visibility: {}", visible);
    }
    
    /// Get swarm status for power-user display
    pub async fn get_swarm_status(&self) -> SwarmStatus {
        let orchs = self.orchs.read().await;
        let active_tasks = self.active_tasks.read().await;
        let pending_auctions = self.pending_auctions.lock().await;
        
        let orch_statuses: Vec<OrchStatusSummary> = orchs
            .values()
            .map(|o| OrchStatusSummary {
                id: o.id,
                name: o.name.clone(),
                status: o.status.clone(),
                current_load: o.current_load,
                active_tasks: o.active_tasks,
                specializations: o.specializations.clone(),
                last_heartbeat_ago_secs: o.last_heartbeat.elapsed().as_secs(),
            })
            .collect();
        
        SwarmStatus {
            total_orchs: orchs.len(),
            active_orchs: orchs.values().filter(|o| o.status != OrchStatus::Offline).count(),
            pending_auctions: pending_auctions.len(),
            active_tasks: active_tasks.len(),
            orchs: orch_statuses,
        }
    }
    
    /// Prune stale ORCHs (no heartbeat within timeout)
    pub async fn prune_stale_orchs(&self) {
        let mut orchs = self.orchs.write().await;
        let stale_ids: Vec<Uuid> = orchs
            .iter()
            .filter(|(_, o)| o.last_heartbeat.elapsed() > self.heartbeat_timeout)
            .map(|(id, _)| *id)
            .collect();
        
        for id in stale_ids {
            if let Some(orch) = orchs.remove(&id) {
                warn!("Pruned stale ORCH: {} ({})", orch.name, id);
            }
        }
    }
    
    /// Get best ORCH for a task type (without auction, for simple tasks)
    pub async fn get_best_orch_for_task(&self, task_type: &TaskType) -> Option<Uuid> {
        let orchs = self.orchs.read().await;
        
        orchs
            .values()
            .filter(|o| {
                o.status == OrchStatus::Idle || o.status == OrchStatus::Busy
            })
            .filter(|o| o.specializations.contains(task_type))
            .min_by(|a, b| a.current_load.partial_cmp(&b.current_load).unwrap())
            .map(|o| o.id)
    }
}

impl Default for InternalSwarmBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Swarm status for power-user display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmStatus {
    pub total_orchs: usize,
    pub active_orchs: usize,
    pub pending_auctions: usize,
    pub active_tasks: usize,
    pub orchs: Vec<OrchStatusSummary>,
}

/// Individual ORCH status summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchStatusSummary {
    pub id: Uuid,
    pub name: String,
    pub status: OrchStatus,
    pub current_load: f64,
    pub active_tasks: usize,
    pub specializations: Vec<TaskType>,
    pub last_heartbeat_ago_secs: u64,
}

/// Hidden Auction Coordinator - runs auctions behind the scenes
pub struct HiddenAuctionCoordinator {
    bus: Arc<InternalSwarmBus>,
    /// Channel for receiving auction requests
    auction_rx: mpsc::Receiver<AuctionRequest>,
    /// Channel for sending auction results
    result_tx: mpsc::Sender<AuctionResult>,
}

/// Request to run an auction
#[derive(Debug)]
pub struct AuctionRequest {
    pub task: TaskBroadcast,
    pub response_tx: tokio::sync::oneshot::Sender<Option<TaskResult>>,
}

/// Result of an auction
#[derive(Debug)]
pub struct AuctionResult {
    pub task_id: Uuid,
    pub result: Option<TaskResult>,
}

impl HiddenAuctionCoordinator {
    pub fn new(
        bus: Arc<InternalSwarmBus>,
        auction_rx: mpsc::Receiver<AuctionRequest>,
        result_tx: mpsc::Sender<AuctionResult>,
    ) -> Self {
        Self {
            bus,
            auction_rx,
            result_tx,
        }
    }
    
    /// Run the auction coordinator loop
    pub async fn run(mut self) {
        info!("Hidden Auction Coordinator started");
        
        while let Some(request) = self.auction_rx.recv().await {
            let bus = self.bus.clone();
            let result_tx = self.result_tx.clone();
            
            // Spawn auction handling in background
            tokio::spawn(async move {
                let task_id = bus.start_auction(request.task).await;
                
                // Wait for auction timeout
                tokio::time::sleep(bus.auction_timeout).await;
                
                // Close auction and get winner
                if let Some(assignment) = bus.close_auction(task_id).await {
                    // Wait for result (with timeout)
                    let result_timeout = Duration::from_secs(300); // 5 minutes max
                    let start = Instant::now();
                    
                    loop {
                        if start.elapsed() > result_timeout {
                            warn!("Task {} timed out waiting for result", task_id);
                            let _ = request.response_tx.send(None);
                            break;
                        }
                        
                        // Check for result
                        let results = bus.drain_pending_results().await;
                        if let Some(result) = results.into_iter().find(|r| r.task_id == task_id) {
                            let _ = request.response_tx.send(Some(result.clone()));
                            let _ = result_tx.send(AuctionResult {
                                task_id,
                                result: Some(result),
                            }).await;
                            break;
                        }
                        
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                } else {
                    // No winner - Sola handles task herself
                    let _ = request.response_tx.send(None);
                }
            });
        }
    }
}

/// Sola's interface to the hidden swarm
pub struct SolaSwarmInterface {
    bus: Arc<InternalSwarmBus>,
    auction_tx: mpsc::Sender<AuctionRequest>,
}

impl SolaSwarmInterface {
    pub fn new(bus: Arc<InternalSwarmBus>, auction_tx: mpsc::Sender<AuctionRequest>) -> Self {
        Self { bus, auction_tx }
    }
    
    /// Delegate a complex task to the swarm (hidden from user)
    /// Returns synthesized result that Sola can present as her own
    pub async fn delegate_task(
        &self,
        description: &str,
        task_type: TaskType,
        complexity: TaskComplexity,
        context: serde_json::Value,
    ) -> Option<serde_json::Value> {
        let task = TaskBroadcast {
            task_id: Uuid::new_v4(),
            description: description.to_string(),
            task_type,
            complexity,
            deadline_ms: None,
            context,
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        let (response_tx, response_rx) = tokio::sync::oneshot::channel();
        
        let request = AuctionRequest {
            task,
            response_tx,
        };
        
        if self.auction_tx.send(request).await.is_err() {
            warn!("Failed to send auction request");
            return None;
        }
        
        match response_rx.await {
            Ok(Some(result)) if result.success => Some(result.result),
            Ok(Some(result)) => {
                warn!("Task failed: {:?}", result.error);
                None
            }
            Ok(None) => {
                debug!("No ORCH available for task");
                None
            }
            Err(_) => {
                warn!("Auction response channel closed");
                None
            }
        }
    }
    
    /// Check for anomaly alerts from ORCHs
    pub async fn check_alerts(&self) -> Vec<AnomalyAlert> {
        self.bus.drain_pending_alerts().await
    }
    
    /// Toggle swarm mode visibility (power-user command)
    pub async fn toggle_swarm_mode(&self, visible: bool) {
        self.bus.set_swarm_mode_visible(visible).await;
    }
    
    /// Get swarm status (only shown if swarm mode is visible)
    pub async fn get_swarm_status(&self) -> Option<SwarmStatus> {
        if self.bus.is_swarm_mode_visible().await {
            Some(self.bus.get_swarm_status().await)
        } else {
            None
        }
    }
    
    /// Check if task should be delegated based on complexity
    pub fn should_delegate(&self, complexity: &TaskComplexity) -> bool {
        matches!(complexity, TaskComplexity::Complex | TaskComplexity::Intensive)
    }
}

/// Create the swarm coordination system
pub fn create_swarm_system() -> (Arc<InternalSwarmBus>, SolaSwarmInterface, mpsc::Sender<AuctionRequest>) {
    let bus = Arc::new(InternalSwarmBus::new());
    let (auction_tx, auction_rx) = mpsc::channel(100);
    let (result_tx, _result_rx) = mpsc::channel(100);
    
    // Start the hidden auction coordinator
    let coordinator = HiddenAuctionCoordinator::new(bus.clone(), auction_rx, result_tx);
    tokio::spawn(coordinator.run());
    
    // Start ORCH pruning task
    let bus_clone = bus.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            bus_clone.prune_stale_orchs().await;
        }
    });
    
    let interface = SolaSwarmInterface::new(bus.clone(), auction_tx.clone());
    
    (bus, interface, auction_tx)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_orch_registration() {
        let bus = InternalSwarmBus::new();
        
        let registration = OrchRegistration {
            orch_id: Uuid::new_v4(),
            orch_name: "TestOrch".to_string(),
            specializations: vec![TaskType::SecurityAnalysis],
            max_concurrent_tasks: 5,
            capabilities: vec!["vuln_scan".to_string()],
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        bus.register_orch(registration.clone()).await;
        
        let status = bus.get_swarm_status().await;
        assert_eq!(status.total_orchs, 1);
        assert_eq!(status.orchs[0].name, "TestOrch");
    }
    
    #[tokio::test]
    async fn test_task_auction() {
        let bus = Arc::new(InternalSwarmBus::new());
        
        // Register an ORCH
        let orch_id = Uuid::new_v4();
        let registration = OrchRegistration {
            orch_id,
            orch_name: "SecurityOrch".to_string(),
            specializations: vec![TaskType::SecurityAnalysis],
            max_concurrent_tasks: 5,
            capabilities: vec![],
            timestamp: chrono::Utc::now().timestamp(),
        };
        bus.register_orch(registration).await;
        
        // Start auction
        let task = TaskBroadcast {
            task_id: Uuid::new_v4(),
            description: "Scan for vulnerabilities".to_string(),
            task_type: TaskType::SecurityAnalysis,
            complexity: TaskComplexity::Moderate,
            deadline_ms: None,
            context: serde_json::json!({}),
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        let task_id = bus.start_auction(task).await;
        
        // Submit bid
        let bid = TaskBid {
            task_id,
            orch_id,
            orch_name: "SecurityOrch".to_string(),
            confidence_score: 0.9,
            estimated_duration_ms: 5000,
            specialization_match: 1.0,
            current_load: 0.2,
            timestamp: chrono::Utc::now().timestamp(),
        };
        bus.submit_bid(bid).await;
        
        // Close auction
        let assignment = bus.close_auction(task_id).await;
        assert!(assignment.is_some());
        assert_eq!(assignment.unwrap().orch_id, orch_id);
    }
    
    #[tokio::test]
    async fn test_swarm_mode_toggle() {
        let bus = InternalSwarmBus::new();
        
        assert!(!bus.is_swarm_mode_visible().await);
        
        bus.set_swarm_mode_visible(true).await;
        assert!(bus.is_swarm_mode_visible().await);
        
        bus.set_swarm_mode_visible(false).await;
        assert!(!bus.is_swarm_mode_visible().await);
    }
}
