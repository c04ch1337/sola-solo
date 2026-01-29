use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod orchestrator;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct Viewport {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub scale: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ElementState {
    pub attributes: Vec<(String, String)>,
    pub r: Option<String>,
    pub name: String,
    pub metadata: ElementMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ElementMetadata {
    pub tag_name: String,
    pub r#type: Option<String>,
    pub has_value: bool,
    pub is_checked: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "action")]
pub enum Action {
    /// Navigates to a new URL.
    Navigate { url: String },
    /// Gets the current state of a page.
    State,
    /// Hovers over a single element. I is the element to hover over.
    Hover { i: usize },
    /// Clicks a single element. I is the element to click. Required to be inside the viewport.
    Click { i: usize },
    /// Types into a single element. I is the element to type into. Required to be inside the viewport.
    Type { i: usize, text: String },
    /// Scrolls the page by a given amount.
    Scroll { x: f64, y: f64 },
    /// Selects a value for a select element. I is the element to select.
    Select { i: usize, value: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct PageState {
    pub viewport: Viewport,
    pub elements: Vec<ElementState>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum DriverError {
    /// The element specified was not found.
    ElementNotFound,
    /// The action failed to execute.
    ActionFailed,
    /// The driver is not running.
    NotRunning,
    /// The viewport is not set.
    ViewportNotSet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum DriverResponse {
    /// The state of the page.
    State(PageState),
    /// The action was completed.
    Complete,
    /// An error occurred.
    Error(DriverError),
    /// The driver is ready to accept commands.
    Ready,
}
