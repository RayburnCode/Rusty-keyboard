use dioxus::prelude::*;
use serde_json::{json, Value};
use std::str::FromStr;

#[component]
pub fn JSONOUT(keys: Vec<KeyData>) -> Element {
    let mut json_text = use_signal(|| {
        json!({
            "meta": {
                "name": "My Keyboard Layout",
                "author": "",
                "notes": "",
                "version": "1.0"
            },
            "keys": keys.iter().map(|key| {
                json!({
                    "x": key.x,
                    "y": key.y,
                    "width": key.width,
                    "height": key.height,
                    "labels": key.labels,
                    "color": key.color,
                    "text_color": key.text_color,
                    "profile": key.profile,
                    "rotation": key.rotation
                })
            }).collect::<Vec<_>>()
        }).to_string()
    });

    let mut edit_mode = use_signal(|| false);
    let mut raw_json = use_signal(|| json_text());

    rsx! {
        div { class: "flex flex-col h-full",
            div { class: "flex justify-between items-center mb-4",
                h2 { class: "text-xl font-bold", "JSON Output" }
                button {
                    class: "px-3 py-1 bg-gray-200 rounded hover:bg-gray-300",
                    onclick: move |_| edit_mode.toggle(),
                    if edit_mode() {
                        "View Mode"
                    } else {
                        "Edit Mode"
                    }
                }
            }

            if edit_mode() {
                textarea {
                    class: "flex-1 font-mono text-sm p-2 border rounded",
                    value: "{raw_json}",
                    oninput: move |e| raw_json.set(e.value()),
                }
            } else {
                pre { class: "flex-1 overflow-auto p-2 bg-gray-50 rounded font-mono text-sm",
                    "{json_text}"
                }
            }

            div { class: "flex gap-2 mt-4 justify-end",
                button {
                    class: "px-3 py-1 bg-gray-200 rounded hover:bg-gray-300",
                    disabled: !edit_mode(),
                    onclick: move |_| {
                        if let Ok(parsed) = Value::from_str(&raw_json()) {
                            json_text.set(parsed.to_string());
                        }
                    },
                    "Apply Changes"
                }
                button {
                    class: "px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600",
                    onclick: move |_| {
                        log::info!("Downloading JSON...");
                    },
                    "Download JSON"
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct KeyData {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub labels: Vec<String>,
    pub color: String,
    pub text_color: String,
    pub profile: String,
    pub rotation: f32,
}