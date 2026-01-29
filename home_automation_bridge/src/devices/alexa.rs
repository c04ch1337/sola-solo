//! Alexa Local Control Integration
//!
//! Provides integration with Alexa devices via local HTTP API or emulator

use crate::devices::traits::{DeviceDiscovery, VoiceAssistant};
use crate::models::{DeviceInfo, DeviceResponse, DeviceState, DeviceType};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct AlexaLocalController {
    client: Client,
    base_url: String,
}

impl AlexaLocalController {
    /// Create a new Alexa controller
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Send a voice command to Alexa
    pub async fn send_voice_command(&self, text: &str) -> Result<DeviceResponse, reqwest::Error> {
        let url = format!("{}/api/alexa/command", self.base_url);
        let payload = json!({
            "type": "voice_command",
            "text": text,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        info!("Sending Alexa command: {}", text);
        let response = self.client.post(&url).json(&payload).send().await?;

        let status = response.status();
        let success = status.is_success();
        let data = if success {
            Some(response.json().await?)
        } else {
            None
        };

        Ok(DeviceResponse {
            success,
            message: if success {
                "Command sent to Alexa".to_string()
            } else {
                format!("Failed to send command: {}", status)
            },
            data,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Get device status
    pub async fn get_device_status(
        &self,
        device_name: &str,
    ) -> Result<DeviceState, reqwest::Error> {
        let url = format!("{}/api/alexa/devices/{}", self.base_url, device_name);
        let state: serde_json::Value = self.client.get(&url).send().await?.json().await?;

        Ok(DeviceState {
            device_id: device_name.to_string(),
            device_name: Some(device_name.to_string()),
            device_type: DeviceType::Switch,
            state,
            timestamp: chrono::Utc::now(),
            bridge_type: "alexa".to_string(),
        })
    }
}

#[async_trait]
impl VoiceAssistant for AlexaLocalController {
    async fn execute_command(
        &self,
        command: &str,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.send_voice_command(command).await.map_err(|e| e.into())
    }

    async fn get_response(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/api/alexa/response", self.base_url);
        let response: serde_json::Value = self.client.get(&url).send().await?.json().await?;

        Ok(response["text"].as_str().unwrap_or("").to_string())
    }
}

#[async_trait]
impl DeviceDiscovery for AlexaLocalController {
    async fn discover_devices(
        &self,
    ) -> Result<Vec<DeviceInfo>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual Alexa device discovery
        // For now, return empty list
        warn!("Alexa device discovery not yet implemented");
        Ok(Vec::new())
    }
}
