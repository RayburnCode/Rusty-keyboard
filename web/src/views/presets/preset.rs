use dioxus::prelude::*;
use crate::views::presets::preview::{PhysicalLayoutType, Preview};
use crate::views::presets::list::PresetList;

#[component]
pub fn Preset() -> Element {
    let mut selected_layout = use_signal(|| PhysicalLayoutType::TKL);

    rsx! {
        div { class: "flex flex-col lg:flex-row gap-8 p-6",
            // Left panel - Preset list (compact view)
            div { class: "lg:w-1/3",
                div { class: "sticky top-6", // Makes it stick when scrolling
                    h2 { class: "text-xl font-bold mb-4 text-gray-800", "Available Layouts" }
                    PresetList {
                        on_select: move |layout| selected_layout.set(layout),
                    }
                }
            }

            // Right panel - Detailed preview
            div { class: "lg:w-2/3",
                Preview {
                    selected_layout: *selected_layout.read(),
                    on_select: move |layout| selected_layout.set(layout),
                }
            }
        }
    }
}