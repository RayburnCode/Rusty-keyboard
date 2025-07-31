use dioxus::prelude::*;
use crate::views::main_page::{KeyboardOptions, LayoutEditor, KeyboardTabs};

/// Home page with responsive layout
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen p-4",
            // Instructions
            p { class: "text-red-500 text-lg font-medium",
                "Use arrow keys or mouse to move around the keyboard layout"
            }
            // Control options at top
            KeyboardOptions {}
            // Main content area - responsive row/column layout
            div { class: "flex flex-col lg:flex-row gap-4 mt-4",
                // Layout editor takes 2/3 space on desktop, full width on mobile
                div { class: "w-full lg:w-2/3", LayoutEditor {} }
                // Tabs take 1/3 space on desktop, full width on mobile
                div { class: "w-full lg:w-1/3", KeyboardTabs {} }
            }
        }
    }
}