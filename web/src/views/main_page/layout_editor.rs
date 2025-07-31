use dioxus::prelude::*;
use crate::views::components::Key;

/// Keyboard Layout Editor
#[component]
pub fn LayoutEditor() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 p-8 max-w-4xl mx-auto",

            Key {
                label: "A".to_string(),
                secondary_label: "".to_string(),
                width: 1,
                height: 1,
            }
            // Visual keyboard layout area
            div {
                class: "w-full min-h-[500px] bg-gray-100 border-2 border-dashed border-gray-400 rounded-lg",
                id: "keyboard-layout-area",
                // Placeholder grid to visualize key positions
                div { class: "grid grid-cols-12 gap-2 p-4",
                    // Top function row placeholder
                    div { class: "col-span-12 h-12 bg-gray-200 rounded flex items-center justify-center text-gray-500",
                        "Function Keys Row"
                    }
                    // Main keyboard area placeholder
                    div { class: "col-span-10 grid grid-cols-10 gap-1",
                        // This would be replaced with actual key components later
                        {(0..30).map(|_| rsx! {
                            div { class: "h-12 bg-white border border-gray-300 rounded flex items-center justify-center text-sm",
                                "Key"
                            }
                        })}
                    }
                    // Numpad placeholder
                    div { class: "col-span-2 grid grid-cols-2 gap-1",
                        {(0..10).map(|_| rsx! {
                            div { class: "h-12 bg-white border border-gray-300 rounded flex items-center justify-center text-sm",
                                "Num"
                            }
                        })}
                    }
                    // Spacebar placeholder
                    div { class: "col-span-12 h-16 bg-gray-300 rounded mt-4 flex items-center justify-center",
                        "Spacebar"
                    }
                }
            }
        }
    }
}