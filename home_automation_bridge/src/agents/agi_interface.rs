//! AGI Integration Interface
//!
//! Processes commands from Phoenix AGI and routes them to appropriate device controllers

use crate::devices::traits::{DeviceDiscovery, LightController, VoiceAssistant};
use crate::devices::{AlexaLocalController, HueBridge};
use crate::models::{AGICommand, DeviceResponse, DeviceState, DiscoveryResult};
use async_trait::async_trait;
use neural_cortex_strata::{MemoryLayer, NeuralCortexStrata};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use vital_organ_vaults::VitalOrganVaults;

/// Main AGI integration for home automation
pub struct AGIIntegration {
    pub hue_bridge: Option<Arc<HueBridge>>,
    pub alexa_controller: Option<Arc<AlexaLocalController>>,
    device_registry: Arc<RwLock<HashMap<String, DeviceState>>>,
    neural_cortex: Arc<NeuralCortexStrata>,
    vaults: Arc<VitalOrganVaults>,
}

impl AGIIntegration {
    /// Create a new AGI integration
    pub fn new(neural_cortex: Arc<NeuralCortexStrata>, vaults: Arc<VitalOrganVaults>) -> Self {
        Self {
            hue_bridge: None,
            alexa_controller: None,
            device_registry: Arc::new(RwLock::new(HashMap::new())),
            neural_cortex,
            vaults,
        }
    }

    /// Set the Hue bridge
    pub fn with_hue_bridge(mut self, bridge: HueBridge) -> Self {
        self.hue_bridge = Some(Arc::new(bridge));
        self
    }

    /// Set the Alexa controller
    pub fn with_alexa_controller(mut self, controller: AlexaLocalController) -> Self {
        self.alexa_controller = Some(Arc::new(controller));
        self
    }

    /// Process a command from AGI
    pub async fn process_agi_command(
        &self,
        command: AGICommand,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>> {
        info!(
            "Processing AGI command: intent={}, id={}",
            command.intent, command.command_id
        );

        let command_clone = command.clone();
        let result = match command.intent.as_str() {
            "control_light" | "turn_on_light" | "turn_off_light" | "set_brightness" => {
                self.handle_light_command(command).await
            }
            "alexa_command" | "voice_command" => self.handle_alexa_command(command).await,
            "get_device_status" | "device_status" => self.get_device_status(command).await,
            "discover_devices" | "list_devices" => self.discover_devices().await,
            _ => {
                warn!("Unknown intent: {}", command_clone.intent);
                Ok(DeviceResponse::error(format!(
                    "Unknown intent: {}",
                    command_clone.intent
                )))
            }
        };

        // Store command and result in memory
        let response_clone = result.as_ref().ok().cloned();
        if let Some(ref response) = response_clone {
            self.store_command_result(&command_clone, response).await?;
        }

        result
    }

    /// Handle light control commands
    async fn handle_light_command(
        &self,
        command: AGICommand,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>> {
        let Some(hue) = &self.hue_bridge else {
            return Ok(DeviceResponse::error("Hue bridge not configured"));
        };

        let device_id = command
            .parameters
            .get("device_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing device_id parameter")?;

        let action = command.intent.as_str();

        match action {
            "control_light" => {
                let on = command.parameters.get("on").and_then(|v| v.as_bool());

                if let Some(on_state) = on {
                    if on_state {
                        LightController::turn_on(hue.as_ref(), device_id).await
                    } else {
                        LightController::turn_off(hue.as_ref(), device_id).await
                    }
                } else {
                    Ok(DeviceResponse::error("Missing 'on' parameter"))
                }
            }
            "turn_on_light" => LightController::turn_on(hue.as_ref(), device_id).await,
            "turn_off_light" => LightController::turn_off(hue.as_ref(), device_id).await,
            "set_brightness" => {
                let brightness = command
                    .parameters
                    .get("brightness")
                    .and_then(|v| v.as_u64())
                    .ok_or("Missing brightness parameter")? as u8;
                LightController::set_brightness(hue.as_ref(), device_id, brightness).await
            }
            _ => Ok(DeviceResponse::error(format!(
                "Unknown light action: {}",
                action
            ))),
        }
    }

    /// Handle Alexa voice commands
    async fn handle_alexa_command(
        &self,
        command: AGICommand,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>> {
        let Some(alexa) = &self.alexa_controller else {
            return Ok(DeviceResponse::error("Alexa controller not configured"));
        };

        let text = command
            .parameters
            .get("text")
            .or_else(|| command.parameters.get("command"))
            .and_then(|v| v.as_str())
            .ok_or("Missing command text")?;

        VoiceAssistant::execute_command(alexa.as_ref(), text).await
    }

    /// Get device status
    async fn get_device_status(
        &self,
        command: AGICommand,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>> {
        let device_id = command
            .parameters
            .get("device_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing device_id parameter")?;

        // Check registry first
        {
            let registry = self.device_registry.read().await;
            if let Some(state) = registry.get(device_id) {
                return Ok(DeviceResponse::success(
                    "Device status retrieved from cache",
                    Some(serde_json::to_value(state)?),
                ));
            }
        }

        // Try Hue first
        if let Some(hue) = &self.hue_bridge {
            if let Ok(state) = LightController::get_state(hue.as_ref(), device_id).await {
                let mut registry = self.device_registry.write().await;
                registry.insert(device_id.to_string(), state.clone());

                return Ok(DeviceResponse::success(
                    "Device status retrieved",
                    Some(serde_json::to_value(state)?),
                ));
            }
        }

        // Try Alexa devices
        if let Some(alexa) = &self.alexa_controller {
            if let Ok(state) = alexa.get_device_status(device_id).await {
                let mut registry = self.device_registry.write().await;
                registry.insert(device_id.to_string(), state.clone());

                return Ok(DeviceResponse::success(
                    "Device status retrieved",
                    Some(serde_json::to_value(state)?),
                ));
            }
        }

        Ok(DeviceResponse::error("Device not found"))
    }

    /// Discover all devices
    async fn discover_devices(
        &self,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut all_devices = Vec::new();

        // Discover Hue lights
        if let Some(hue) = &self.hue_bridge {
            match DeviceDiscovery::discover_devices(hue.as_ref()).await {
                Ok(mut devices) => all_devices.append(&mut devices),
                Err(e) => warn!("Failed to discover Hue devices: {}", e),
            }
        }

        // Discover Alexa devices
        if let Some(alexa) = &self.alexa_controller {
            match DeviceDiscovery::discover_devices(alexa.as_ref()).await {
                Ok(mut devices) => all_devices.append(&mut devices),
                Err(e) => warn!("Failed to discover Alexa devices: {}", e),
            }
        }

        let discovery = DiscoveryResult {
            devices: all_devices.clone(),
            timestamp: chrono::Utc::now(),
        };

        // Store discovery result in memory
        let key = format!(
            "body:home_automation:discovery:{}",
            discovery.timestamp.timestamp()
        );
        let value = serde_json::to_string(&discovery)?;
        self.vaults
            .store_body(&key, &value)
            .map_err(|e| format!("Failed to store discovery: {}", e))?;

        Ok(DeviceResponse::success(
            format!("Discovered {} devices", all_devices.len()),
            Some(serde_json::to_value(discovery)?),
        ))
    }

    /// Store command result in memory
    async fn store_command_result(
        &self,
        command: &AGICommand,
        response: &DeviceResponse,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Store in Body Vault for operational data
        let key = format!("body:home_automation:command:{}", command.command_id);
        let value = serde_json::json!({
            "command": command,
            "response": response,
        });
        let value_str = serde_json::to_string(&value)?;

        self.vaults
            .store_body(&key, &value_str)
            .map_err(|e| format!("Failed to store command: {}", e))?;

        // Store device state updates in EPM for episodic memory
        if response.success && response.data.is_some() {
            let epm_key = format!(
                "epm:home_automation:event:{}",
                chrono::Utc::now().timestamp()
            );
            let epm_value = serde_json::to_string(&value)?;

            self.neural_cortex
                .etch(MemoryLayer::EPM(epm_value), &epm_key)
                .map_err(|e| format!("Failed to store in EPM: {}", e))?;
        }

        Ok(())
    }

    /// Get all registered devices (async)
    pub async fn get_all_devices_async(&self) -> Vec<DeviceState> {
        let registry = self.device_registry.read().await;
        registry.values().cloned().collect()
    }
}

#[async_trait]
pub trait AgenticInterface {
    async fn send_command(
        &self,
        command: AGICommand,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_all_devices(
        &self,
    ) -> Result<Vec<DeviceState>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
impl AgenticInterface for AGIIntegration {
    async fn send_command(
        &self,
        command: AGICommand,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.process_agi_command(command).await
    }

    async fn get_all_devices(
        &self,
    ) -> Result<Vec<DeviceState>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.get_all_devices_async().await)
    }
}
