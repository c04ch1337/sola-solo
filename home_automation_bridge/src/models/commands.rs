//! Command and device state models for home automation

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Command from AGI to home automation system
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AGICommand {
    pub command_id: String,
    pub intent: String,
    pub parameters: serde_json::Value,
    pub source: String, // "agi"
    pub timestamp: Option<DateTime<Utc>>,
}

impl Default for AGICommand {
    fn default() -> Self {
        Self {
            command_id: uuid::Uuid::new_v4().to_string(),
            intent: String::new(),
            parameters: serde_json::json!({}),
            source: "agi".to_string(),
            timestamp: Some(Utc::now()),
        }
    }
}

/// Response from device operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

impl DeviceResponse {
    pub fn success(message: impl Into<String>, data: Option<serde_json::Value>) -> Self {
        Self {
            success: true,
            message: message.into(),
            data,
            timestamp: Utc::now(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: None,
            timestamp: Utc::now(),
        }
    }
}

/// Device types supported by the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    Light,
    Switch,
    Sensor,
    Speaker,
    Thermostat,
    Camera,
    Lock,
    Unknown,
}

/// Device state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceState {
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: DeviceType,
    pub state: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub bridge_type: String, // "hue", "alexa", "mqtt", etc.
}

/// Device discovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryResult {
    pub devices: Vec<DeviceInfo>,
    pub timestamp: DateTime<Utc>,
}

/// Basic device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_name: String,
    pub device_type: DeviceType,
    pub bridge_type: String,
    pub capabilities: Vec<String>,
    pub online: bool,
}

/// Automation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub rule_id: String,
    pub name: String,
    pub trigger: RuleTrigger,
    pub actions: Vec<RuleAction>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RuleTrigger {
    Time {
        schedule: String,
    },
    DeviceState {
        device_id: String,
        condition: String,
    },
    Sensor {
        sensor_id: String,
        threshold: f64,
    },
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAction {
    pub device_id: String,
    pub action: String,
    pub parameters: serde_json::Value,
}
