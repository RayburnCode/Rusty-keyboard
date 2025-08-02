// client/components/layout/app_layout.rs
use dioxus::prelude::*;
use crate::views::firmware::{ FirmwareTabs};


#[component]
pub fn Firmware() -> Element {
    rsx! {
        // Main content area - responsive row/column layout
        div { class: "flex flex-col lg:flex-row gap-4 mt-4",
            // Layout editor takes 2/3 space on desktop, full width on mobile
            div { class: "w-full lg:w-2/3",
                p { class: "text-lg font-semibold mb-2", "Firmware Editor" }
            }
            p { "Section that will house the display" }
            // Tabs take 1/3 space on desktop, full width on mobile
            div { class: "w-full lg:w-1/3", FirmwareTabs {} }
        }
    }
}