use serde::{Serialize, Deserialize};
use serde_json::json;


/// NEED TO IMPLEMENT
/// I want to use this to be able to generate a more specific layout
/// steps, 1, .5, .25
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyboardStep {
    pub step: f32,
    pub keys: Vec<KeyData>,
}



/// Complete keyboard layout specification
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyboardLayout {
    pub name: String,
    pub author: String,
    pub layers: Vec<KeyboardLayer>,
    pub meta: Option<LayoutMeta>,
}

/// Individual layer specification
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyboardLayer {
    pub id: u8,
    pub name: String,
    pub keys: Vec<KeyData>,
}

/// Key specification matching your struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyData {
    #[serde(default = "generate_id")]
    pub id: String,
    
    pub label: String,
    
    #[serde(default, rename = "secondaryLabel")]
    pub secondary_label: String,
    
    #[serde(default = "default_width")]
    pub width: u8,
    
    #[serde(default = "default_height")]
    pub height: u8,
    
    #[serde(default)]
    pub x: u8,
    
    #[serde(default)]
    pub y: u8,
    
    #[serde(default = "default_color")]
    pub color: String,
    
    #[serde(default)]
    pub keycode: String,
    
    #[serde(default)]
    pub layer: u8,
    
    #[serde(skip)] // Don't serialize selected state
    pub selected: bool,
}

/// Additional metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutMeta {
    pub created: String,
    pub modified: String,
    pub description: Option<String>,
}

// Default values
fn generate_id() -> String {
    nanoid::nanoid!()
}

fn default_width() -> u8 { 1 }
fn default_height() -> u8 { 1 }
fn default_color() -> String { "#FFFFFF".into() }

/// Example layout matching your structure
pub fn example_layout() -> serde_json::Value {
    json!({
        "name": "Standard Keyboard Layout",
        "author": "Your Name",
        "layers": [{
            "id": 0,
            "name": "Base Layer",
            "keys": [
                {
                    "label": "Num Lock",
                    "x": 0, "y": 0,
                    "keycode": "NumLock"
                },
                {
                    "label": "/",
                    "x": 1, "y": 0,
                    "keycode": "NumpadDivide"
                },
                {
                    "label": "*",
                    "x": 2, "y": 0,
                    "keycode": "NumpadMultiply"
                },
                {
                    "label": "-",
                    "x": 3, "y": 0,
                    "keycode": "NumpadSubtract"
                },
                {
                    "label": "7",
                    "secondaryLabel": "Home",
                    "x": 0, "y": 1,
                    "keycode": "Numpad7"
                },
                {
                    "label": "8",
                    "secondaryLabel": "↑",
                    "x": 1, "y": 1,
                    "keycode": "Numpad8"
                },
                {
                    "label": "9",
                    "secondaryLabel": "PgUp",
                    "x": 2, "y": 1,
                    "keycode": "Numpad9"
                },
                {
                    "label": "+",
                    "height": 2,
                    "x": 3, "y": 1,
                    "keycode": "NumpadAdd"
                },
                // ... continue with all other keys
            ]
        }],
        "meta": {
            "created": "2023-07-20",
            "modified": "2023-07-20",
            "description": "Standard keyboard layout with numpad"
        }
    })
}

/// Helper function to parse layout JSON
pub fn parse_layout(json_str: &str) -> Result<KeyboardLayout, serde_json::Error> {
    serde_json::from_str(json_str)
}

/// Helper function to serialize layout
pub fn serialize_layout(layout: &KeyboardLayout) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(layout)
}