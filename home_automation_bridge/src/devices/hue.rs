//! Philips Hue Bridge Integration

use crate::devices::traits::{DeviceDiscovery, LightController};
use crate::models::{DeviceInfo, DeviceResponse, DeviceState, DeviceType};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use tracing::info;

#[derive(Debug, Clone)]
pub struct HueBridge {
    client: Client,
    bridge_ip: String,
    username: String,
}

impl HueBridge {
    /// Create a new Hue Bridge connection
    pub fn new(bridge_ip: String, username: String) -> Self {
        Self {
            client: Client::new(),
            bridge_ip,
            username,
        }
    }

    /// Get the base URL for API requests
    fn base_url(&self) -> String {
        format!("http://{}/api/{}", self.bridge_ip, self.username)
    }

    /// Discover all lights on the bridge
    pub async fn discover_lights(&self) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/lights", self.base_url());
        info!("Discovering Hue lights from {}", url);
        self.client.get(&url).send().await?.json().await
    }

    /// Set light state with various parameters
    pub async fn set_light_state(
        &self,
        light_id: u32,
        on: Option<bool>,
        brightness: Option<u8>,
        hue: Option<u16>,
        saturation: Option<u8>,
    ) -> Result<DeviceResponse, reqwest::Error> {
        let mut state = serde_json::Map::new();

        if let Some(on_state) = on {
            state.insert("on".to_string(), json!(on_state));
        }
        if let Some(bri) = brightness {
            state.insert("bri".to_string(), json!(bri));
        }
        if let Some(hue_val) = hue {
            state.insert("hue".to_string(), json!(hue_val));
        }
        if let Some(sat) = saturation {
            state.insert("sat".to_string(), json!(sat));
        }

        let url = format!("{}/lights/{}/state", self.base_url(), light_id);
        info!("Setting Hue light {} state: {:?}", light_id, state);

        let response = self.client.put(&url).json(&state).send().await?;

        let response_data: Vec<serde_json::Value> = response.json().await?;

        let success = response_data.iter().any(|r| r.get("success").is_some());

        Ok(DeviceResponse {
            success,
            message: if success {
                "Light state updated".to_string()
            } else {
                "Failed to update light state".to_string()
            },
            data: Some(json!(response_data)),
            timestamp: chrono::Utc::now(),
        })
    }

    /// Get current light state
    pub async fn get_light_state(&self, light_id: u32) -> Result<DeviceState, reqwest::Error> {
        let url = format!("{}/lights/{}", self.base_url(), light_id);
        let state: serde_json::Value = self.client.get(&url).send().await?.json().await?;

        let name = state
            .get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(DeviceState {
            device_id: light_id.to_string(),
            device_name: name,
            device_type: DeviceType::Light,
            state,
            timestamp: chrono::Utc::now(),
            bridge_type: "hue".to_string(),
        })
    }
}

#[async_trait]
impl LightController for HueBridge {
    async fn turn_on(
        &self,
        device_id: &str,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>> {
        let light_id = device_id
            .parse::<u32>()
            .map_err(|e| format!("Invalid light ID: {}", e))?;
        self.set_light_state(light_id, Some(true), None, None, None)
            .await
            .map_err(|e| e.into())
    }

    async fn turn_off(
        &self,
        device_id: &str,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>> {
        let light_id = device_id
            .parse::<u32>()
            .map_err(|e| format!("Invalid light ID: {}", e))?;
        self.set_light_state(light_id, Some(false), None, None, None)
            .await
            .map_err(|e| e.into())
    }

    async fn set_brightness(
        &self,
        device_id: &str,
        brightness: u8,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>> {
        let light_id = device_id
            .parse::<u32>()
            .map_err(|e| format!("Invalid light ID: {}", e))?;
        self.set_light_state(light_id, None, Some(brightness), None, None)
            .await
            .map_err(|e| e.into())
    }

    async fn set_color(
        &self,
        device_id: &str,
        hue: Option<u16>,
        saturation: Option<u8>,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>> {
        let light_id = device_id
            .parse::<u32>()
            .map_err(|e| format!("Invalid light ID: {}", e))?;
        self.set_light_state(light_id, None, None, hue, saturation)
            .await
            .map_err(|e| e.into())
    }

    async fn get_state(
        &self,
        device_id: &str,
    ) -> Result<DeviceState, Box<dyn std::error::Error + Send + Sync>> {
        let light_id = device_id
            .parse::<u32>()
            .map_err(|e| format!("Invalid light ID: {}", e))?;
        self.get_light_state(light_id).await.map_err(|e| e.into())
    }
}

#[async_trait]
impl DeviceDiscovery for HueBridge {
    async fn discover_devices(
        &self,
    ) -> Result<Vec<DeviceInfo>, Box<dyn std::error::Error + Send + Sync>> {
        let lights = self.discover_lights().await?;
        let mut devices = Vec::new();

        if let Some(lights_map) = lights.as_object() {
            for (light_id, light_data) in lights_map {
                let name = light_data
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string();

                let state = light_data.get("state").and_then(|s| s.as_object());

                let online = state
                    .and_then(|s| s.get("reachable"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                devices.push(DeviceInfo {
                    device_id: light_id.clone(),
                    device_name: name,
                    device_type: DeviceType::Light,
                    bridge_type: "hue".to_string(),
                    capabilities: vec![
                        "on_off".to_string(),
                        "brightness".to_string(),
                        "color".to_string(),
                    ],
                    online,
                });
            }
        }

        Ok(devices)
    }
}
