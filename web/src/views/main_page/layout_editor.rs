use dioxus::prelude::*;
use crate::views::components::{Key, KeyProps};

#[derive(Clone, PartialEq)]
struct KeyPosition {
    pub key: KeyProps,
    pub row: u8,
    pub col: u8,
    pub x_offset: f32, // Fine positioning within row/col
    pub y_offset: f32,
}

/// Convert KeyProps to JSON string representation
fn key_props_to_json_string(key: &KeyProps, index: usize) -> String {
    format!(r#"{{
  "id": "key_{}",
  "label": "{}",
  "secondaryLabel": "{}",
  "width": {},
  "height": {},
  "x": {},
  "y": {},
  "color": "{}",
  "keycode": "{}",
  "layer": 0
}}"#, 
        index,
        key.label,
        key.secondary_label,
        key.width,
        key.height,
        key.x,
        key.y,
        key.color,
        key.label // Default keycode to label
    )
}

#[derive(PartialEq, Props, Clone)]
pub struct LayoutEditorProps {
    pub keys: Vec<KeyProps>,
}

/// Keyboard Layout Editor
#[component]
pub fn LayoutEditor(props: LayoutEditorProps) -> Element {
    let mut zoom_level = use_signal(|| 1.0_f64);
    let mut grid_snap = use_signal(|| 0.25_f32);

    let mut selected_key_json = use_signal(|| String::new());
    let mut selected_key_index = use_signal(|| None::<usize>);
    let mut key_positions = use_signal(|| {
        // Convert input keys to positioned keys with default row/col layout
        props.keys.iter().enumerate().map(|(i, key)| {
            let row = (i / 12) as u8; // 12 keys per row default
            let col = (i % 12) as u8;
            KeyPosition {
                key: key.clone(),
                row,
                col,
                x_offset: 0.0,
                y_offset: 0.0,
            }
        }).collect::<Vec<_>>()
    });

    rsx! {
        div { class: "flex gap-4 p-4 mx-auto w-full max-w-8xl",
            // Left side - Keyboard editor
            div { class: "flex-1 flex flex-col gap-4",
                h2 { class: "text-xl font-bold mb-4", "Keyboard Layout Editor" }
                // Controls row
                div { class: "flex gap-2 items-center justify-between",
                    // Instructions
                    div { class: "text-sm text-gray-600", "Click to select • Drag to position" }
                    // Zoom controls
                    div { class: "flex gap-2 items-center",
                        button {
                            class: "px-3 py-1 bg-gray-200 rounded hover:bg-gray-300",
                            onclick: move |_| grid_snap.set((grid_snap() * 0.9_f32).max(0.1_f32)),
                            "Grid Snap - 0.25 + "
                        }
                        button {
                            class: "px-3 py-1 bg-gray-200 rounded hover:bg-gray-300",
                            onclick: move |_| zoom_level.set((zoom_level() * 0.9_f64).max(0.5_f64)),
                            "Zoom Out -"
                        }
                        span { class: "text-sm w-12 text-center", "{(zoom_level() * 100.0) as i32}%" }
                        button {
                            class: "px-3 py-1 bg-gray-200 rounded hover:bg-gray-300",
                            onclick: move |_| zoom_level.set((zoom_level() * 1.1_f64).min(2.0_f64)),
                            "Zoom In +"
                        }
                    }
                }

                // Arrow key controls for selected key
                if let Some(selected_idx) = *selected_key_index.read() {
                    div { class: "flex gap-2 items-center justify-center p-2 bg-blue-50 rounded border",
                        span { class: "text-sm font-medium", "Move Key #{selected_idx + 1}:" }
                        button {
                            class: "p-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                            onclick: move |_| {
                                let mut positions = key_positions.write();
                                if let Some(key_pos) = positions.get_mut(selected_idx) {
                                    key_pos.row = key_pos.row.saturating_sub(1);
                                    let json_output = key_props_to_json_string(&key_pos.key, selected_idx);
                                    selected_key_json.set(json_output);
                                }
                            },
                            "↑"
                        }
                        button {
                            class: "p-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                            onclick: move |_| {
                                let mut positions = key_positions.write();
                                if let Some(key_pos) = positions.get_mut(selected_idx) {
                                    key_pos.col = key_pos.col.saturating_sub(1);
                                    let json_output = key_props_to_json_string(&key_pos.key, selected_idx);
                                    selected_key_json.set(json_output);
                                }
                            },
                            "←"
                        }
                        button {
                            class: "p-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                            onclick: move |_| {
                                let mut positions = key_positions.write();
                                if let Some(key_pos) = positions.get_mut(selected_idx) {
                                    key_pos.row = (key_pos.row + 1).min(10);
                                    let json_output = key_props_to_json_string(&key_pos.key, selected_idx);
                                    selected_key_json.set(json_output);
                                }
                            },
                            "↓"
                        }
                        button {
                            class: "p-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                            onclick: move |_| {
                                let mut positions = key_positions.write();
                                if let Some(key_pos) = positions.get_mut(selected_idx) {
                                    key_pos.col = (key_pos.col + 1).min(20);
                                    let json_output = key_props_to_json_string(&key_pos.key, selected_idx);
                                    selected_key_json.set(json_output);
                                }
                            },
                            "→"
                        }
                    }
                }

                // Keyboard container with grid
                div {
                    class: "relative overflow-auto bg-gray-100 border-2 border-dashed border-gray-400 rounded-lg p-4",
                    style: "min-height: 400px;",
                    // Grid background
                    div {
                        class: "absolute inset-4 pointer-events-none",
                        style: "
                            background-image: 
                                linear-gradient(to right, #e5e7eb 1px, transparent 1px),
                                linear-gradient(to bottom, #e5e7eb 1px, transparent 1px);
                            background-size: {60.0 * zoom_level()}px {60.0 * zoom_level()}px;
                            transform: scale({zoom_level()});
                            transform-origin: 0 0;
                        ",
                    }
                    div {
                        class: "keyboard-layout relative",
                        style: "transform: scale({zoom_level()}); transform-origin: 0 0;",
                        // Render positioned keys
                        for (index , key_pos) in key_positions.read().iter().enumerate() {
                            div {
                                key: "{index}",
                                class: format!(
                                    "absolute cursor-pointer transition-all duration-200 {}",
                                    if *selected_key_index.read() == Some(index) {
                                        "ring-2 ring-blue-500 ring-offset-2"
                                    } else {
                                        ""
                                    },
                                ),
                                style: "
                                    left: {(key_pos.col as f32 * 60.0) + key_pos.x_offset}px;
                                    top: {(key_pos.row as f32 * 60.0) + key_pos.y_offset}px;
                                ",
                                onclick: {
                                    let key_clone = key_pos.key.clone();
                                    move |_| {
                                        let json_output = key_props_to_json_string(&key_clone, index);
                                        selected_key_json.set(json_output);
                                        selected_key_index.set(Some(index));
                                    }
                                },
                                Key {
                                    label: key_pos.key.label.clone(),
                                    secondary_label: key_pos.key.secondary_label.clone(),
                                    width: key_pos.key.width,
                                    height: key_pos.key.height,
                                    x: key_pos.col,
                                    y: key_pos.row,
                                    color: key_pos.key.color.clone(),
                                    on_click: move |_| {},
                                }
                            }
                        }
                    }
                }
            }
            // Right side - JSON output and position info
            div { class: "w-96 flex flex-col gap-4",
                h3 { class: "text-lg font-bold", "Selected Key" }
                if let Some(index) = *selected_key_index.read() {
                    if let Some(key_pos) = key_positions.read().get(index) {
                        div { class: "space-y-2 p-3 bg-gray-50 rounded border",
                            div { class: "text-sm text-gray-600",
                                "Key #{index + 1} - {key_pos.key.label}"
                            }
                            div { class: "text-xs text-gray-500",
                                "Position: Row {key_pos.row}, Col {key_pos.col}"
                            }
                            div { class: "text-xs text-gray-500",
                                "Offset: ({key_pos.x_offset:.1}, {key_pos.y_offset:.1})"
                            }
                        }
                    }
                }
                div { class: "flex-1",
                    textarea {
                        class: "w-full h-64 p-3 border rounded-lg font-mono text-sm resize-none",
                        placeholder: "Click on a key to see its JSON representation...",
                        readonly: true,
                        value: "{selected_key_json}",
                    }
                }
                div { class: "flex gap-2",
                    button {
                        class: "px-3 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50",
                        disabled: selected_key_json.read().is_empty(),
                        onclick: move |_| {
                            log::info!("Copy JSON to clipboard");
                        },
                        "Copy JSON"
                    }
                    button {
                        class: "px-3 py-2 bg-gray-500 text-white rounded hover:bg-gray-600",
                        onclick: move |_| {
                            selected_key_json.set(String::new());
                            selected_key_index.set(None);
                        },
                        "Clear"
                    }
                }
                // Grid controls
                div { class: "space-y-2 p-3 bg-gray-50 rounded border",
                    h4 { class: "font-medium text-sm", "Grid Settings" }
                    div { class: "text-xs text-gray-600",
                        "Grid: 60px spacing • Max: 20 cols × 10 rows"
                    }
                    button {
                        class: "w-full px-3 py-2 bg-green-500 text-white rounded hover:bg-green-600 text-sm",
                        onclick: move |_| {
                            let mut positions = key_positions.write();
                            for (i, pos) in positions.iter_mut().enumerate() {
                                pos.row = (i / 12) as u8;
                                pos.col = (i % 12) as u8;
                                pos.x_offset = 0.0;
                                pos.y_offset = 0.0;
                            }
                            selected_key_json.set(String::new());
                            selected_key_index.set(None);
                        },
                        "Reset Layout"
                    }
                }
            }
        }
    }
}