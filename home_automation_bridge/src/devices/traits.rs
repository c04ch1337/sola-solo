//! Traits for device controllers

use crate::models::{DeviceResponse, DeviceState};
use async_trait::async_trait;

/// Trait for controlling lights
#[async_trait]
pub trait LightController {
    async fn turn_on(
        &self,
        device_id: &str,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>>;
    async fn turn_off(
        &self,
        device_id: &str,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>>;
    async fn set_brightness(
        &self,
        device_id: &str,
        brightness: u8,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>>;
    async fn set_color(
        &self,
        device_id: &str,
        hue: Option<u16>,
        saturation: Option<u8>,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_state(
        &self,
        device_id: &str,
    ) -> Result<DeviceState, Box<dyn std::error::Error + Send + Sync>>;
}

/// Trait for voice assistants
#[async_trait]
pub trait VoiceAssistant {
    async fn execute_command(
        &self,
        command: &str,
    ) -> Result<DeviceResponse, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_response(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}

/// Trait for device discovery
#[async_trait]
pub trait DeviceDiscovery {
    async fn discover_devices(
        &self,
    ) -> Result<Vec<crate::models::DeviceInfo>, Box<dyn std::error::Error + Send + Sync>>;
}
