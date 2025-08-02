use dioxus::prelude::*;
use crate::json::schema::KeyboardLayout;

// Physical layout types to match your JSON files
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhysicalLayoutType {
    FullSize,   // ansi_full_size.json
    TKL,        // ansi_tkl.json
    SeventyFive, // ansi_75.json
    SixtyFive,  // ansi_65.json
    Sixty,      // ansi_60.json
}

impl PhysicalLayoutType {
    pub fn get_name(&self) -> &'static str {
        match self {
            PhysicalLayoutType::FullSize => "ANSI Full Size Layout",
            PhysicalLayoutType::TKL => "ANSI TKL Layout", 
            PhysicalLayoutType::SeventyFive => "ANSI 75% Layout",
            PhysicalLayoutType::SixtyFive => "ANSI 65% Layout",
            PhysicalLayoutType::Sixty => "ANSI 60% Layout",
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            PhysicalLayoutType::FullSize => "Full Size (104-key) - Standard keyboard with numpad",
            PhysicalLayoutType::TKL => "TKL (87-key) - Tenkeyless without numpad",
            PhysicalLayoutType::SeventyFive => "75% (84-key) - Compact with function row",
            PhysicalLayoutType::SixtyFive => "65% (68-key) - No function row, dedicated arrows",
            PhysicalLayoutType::Sixty => "60% (61-key) - Minimalist without arrows or function row",
        }
    }

    pub fn get_json_file(&self) -> &'static str {
        match self {
            PhysicalLayoutType::FullSize => include_str!("../../json/layouts/ansi_full_size.json"),
            PhysicalLayoutType::TKL => include_str!("../../json/layouts/ansi_tkl.json"),
            PhysicalLayoutType::SeventyFive => include_str!("../../json/layouts/ansi_75.json"),
            PhysicalLayoutType::SixtyFive => include_str!("../../json/layouts/ansi_65.json"),
            PhysicalLayoutType::Sixty => include_str!("../../json/layouts/ansi_60.json"),
        }
    }

    pub fn load_layout(&self) -> Result<KeyboardLayout, serde_json::Error> {
        serde_json::from_str(self.get_json_file())
    }

    pub fn get_all() -> Vec<PhysicalLayoutType> {
        vec![
            PhysicalLayoutType::FullSize,
            PhysicalLayoutType::TKL,
            PhysicalLayoutType::SeventyFive,
            PhysicalLayoutType::SixtyFive,
            PhysicalLayoutType::Sixty,
        ]
    }
}

#[component]
pub fn Preview(selected_layout: PhysicalLayoutType, on_select: EventHandler<PhysicalLayoutType>) -> Element {
    // Generate visual representations from JSON data
    let layouts = vec![
        PhysicalLayoutType::FullSize,
        PhysicalLayoutType::TKL,
        PhysicalLayoutType::SeventyFive,
        PhysicalLayoutType::SixtyFive,
        PhysicalLayoutType::Sixty,
    ];

    rsx! {
        div { class: "p-6 bg-white rounded-lg shadow-sm",
            h2 { class: "text-2xl font-bold mb-6 text-gray-800", "Keyboard Layout Previews" }
            div { class: "flex flex-wrap gap-6 justify-center",
                for layout in layouts {
                    div {
                        key: "{layout.get_name()}",
                        class: "cursor-pointer transition-all duration-200 hover:scale-105",
                        onclick: move |_| on_select.call(layout),
                        div {
                            class: format!(
                                "p-4 border-2 rounded-lg {}",
                                if layout == selected_layout {
                                    "border-blue-500 bg-blue-50"
                                } else {
                                    "border-gray-200 hover:border-blue-300"
                                },
                            ),
                            h3 { class: "text-lg font-semibold mb-3 text-center",
                                "{layout.get_description()}"
                            }
                            // Display JSON-based visual representation
                            div { class: "border rounded p-4 bg-gray-50",
                                match layout.load_layout() {
                                    Ok(keyboard_layout) => rsx! {
                                        div { class: "space-y-2",
                                            div { class: "text-sm font-medium text-gray-700", "{keyboard_layout.name}" }
                                            div { class: "text-xs text-gray-500", "By {keyboard_layout.author}" }
                                            if let Some(first_layer) = keyboard_layout.layers.first() {
                                                div { class: "text-xs text-gray-600", "{first_layer.keys.len()} keys" }
                                                // Simple visual representation based on key positions
                                                KeyLayoutVisualization { keys: first_layer.keys.clone() }
                                            }
                                        }
                                    },
                                    Err(_) => rsx! {
                                        div { class: "text-red-500 text-sm", "Failed to load layout" }
                                    },
                                }
                            }
                        }
                    }
                }
            }
            div { class: "mt-8 p-4 bg-blue-50 border border-blue-200 rounded-lg",
                h3 { class: "font-medium text-blue-800 mb-2", "Selected Layout" }
                p { class: "text-blue-700",
                    "You've selected: {selected_layout.get_description()}. Click any layout above to change selection."
                }
            }
        }
    }
}

// Component to visualize the key layout based on JSON data
#[derive(PartialEq, Props, Clone)]
struct KeyLayoutVisualizationProps {
    keys: Vec<crate::json::schema::KeyData>,
}

#[component]
fn KeyLayoutVisualization(props: KeyLayoutVisualizationProps) -> Element {
    let keys = &props.keys;
    
    // Calculate layout bounds
    let max_x = keys.iter().map(|k| k.x).max().unwrap_or(0) as usize;
    let max_y = keys.iter().map(|k| k.y).max().unwrap_or(0) as usize;
    
    rsx! {
        div {
            class: "relative",
            style: "width: 100%; aspect-ratio: {(max_x + 1) as f32 / (max_y + 1) as f32}; min-height: 120px;",
            // Render keys at their positions
            for (index , key) in keys.iter().enumerate() {
                div {
                    key: "{index}",
                    class: "absolute bg-gray-200 border border-gray-300 rounded text-xs flex items-center justify-center font-mono",
                    style: "
                        left: {(key.x as f32 / (max_x + 1) as f32) * 100.0}%;
                        top: {(key.y as f32 / (max_y + 1) as f32) * 100.0}%;
                        width: {100.0 / (max_x + 1) as f32 * 0.9}%;
                        height: {100.0 / (max_y + 1) as f32 * 0.8}%;
                    ",
                    title: "{key.keycode}",
                    div { class: "text-center",
                        div { class: "text-[8px] leading-none", "{key.label}" }
                        if !key.secondary_label.is_empty() {
                            div { class: "text-[6px] text-gray-500 leading-none",
                                "{key.secondary_label}"
                            }
                        }
                    }
                }
            }
        }
    }
}