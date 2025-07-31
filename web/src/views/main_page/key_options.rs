use dioxus::prelude::*;

/// Home page
#[component]
pub fn KeyboardOptions() -> Element {
    rsx! {
        div { class: "flex flex-row gap-2 p-4 bg-gray-100 rounded-lg", // Tailwind classes for row layout
            // Each button with Tailwind styling
            button { class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors",
                "Add Key"
            }
            button { class: "px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition-colors",
                "Delete Key"
            }
            button { class: "px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600 transition-colors",
                "Undo (Ctrl+Z)"
            }
            button { class: "px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600 transition-colors",
                "Redo (Ctrl+Shift+Z)"
            }
        
        }
    }
}