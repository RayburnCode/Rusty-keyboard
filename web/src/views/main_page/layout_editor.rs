use dioxus::prelude::*;
use crate::views::components::{Key, KeyProps};

#[derive(PartialEq, Props, Clone)]
pub struct LayoutEditorProps {
    pub keys: Vec<KeyProps>,
}

/// Keyboard Layout Editor
#[component]
pub fn LayoutEditor(props: LayoutEditorProps) -> Element {
    let mut zoom_level = use_signal(|| 1.0_f64);

    rsx! {
        div { class: "flex flex-col gap-4 p-4 mx-auto w-full max-w-8xl",
            h2 { class: "text-xl font-bold mb-4", "Keyboard Layout Editor" }
            // Zoom controls
            div { class: "flex gap-2 items-center justify-end",
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

            // Keyboard container
            div {
                class: "relative overflow-auto bg-gray-100 border-2 border-dashed border-gray-400 rounded-lg p-4",
                style: "height: 500px;",
                div {
                    class: "keyboard-layout",
                    style: "transform: scale({zoom_level()}); transform-origin: 0 0;",
                    // Dynamic keyboard layout based on props
                    div { class: "flex flex-wrap gap-1 w-fit max-w-4xl",
                        for key in &props.keys {
                            Key {
                                label: key.label.clone(),
                                secondary_label: key.secondary_label.clone(),
                                width: key.width,
                                height: key.height,
                                on_click: key.on_click.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}