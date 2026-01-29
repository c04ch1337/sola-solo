//! Home Automation Bridge for Phoenix AGI
//!
//! Provides integration between Phoenix AGI and home automation systems
//! including Philips Hue, Alexa, and other IoT devices.
//!
//! Features:
//! - Device discovery and control
//! - State management with memory integration
//! - Automation rule engine
//! - Multi-bridge support (Hue, Alexa, MQTT)

pub mod agents;
pub mod devices;
pub mod models;

pub use agents::*;
pub use devices::*;
pub use models::*;
