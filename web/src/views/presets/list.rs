use dioxus::prelude::*;
use crate::views::presets::preview::PhysicalLayoutType;

#[component]
pub fn PresetList(on_select: EventHandler<PhysicalLayoutType>) -> Element {
    rsx! {
        div { class: "p-4",
            h2 { class: "text-2xl font-bold mb-4", "Keyboard Layout Presets" }
            p { class: "text-gray-600 mb-6",
                "Select a preset to load into the editor. Each preview shows the general layout structure."
            }

            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                for (name , layout_type , description) in [
                    ("Full Size (104-key)", PhysicalLayoutType::FullSize, "Standard keyboard with numpad"),
                    ("TKL (87-key)", PhysicalLayoutType::TKL, "Tenkeyless without numpad"),
                    ("75% (84-key)", PhysicalLayoutType::SeventyFive, "Compact with function row"),
                    ("65% (68-key)", PhysicalLayoutType::SixtyFive, "No function row, dedicated arrows"),
                    ("60% (61-key)", PhysicalLayoutType::Sixty, "Minimalist without arrows or function row"),
                ].iter() {
                    div {
                        key: "{name}",
                        class: "border rounded-lg overflow-hidden hover:shadow-md transition-shadow cursor-pointer",
                        onclick: move |_| on_select.call(*layout_type),
                        // Preview placeholder
                        div { class: "bg-gray-100 h-32 flex items-center justify-center",
                            svg {
                                class: "w-24 h-24 text-gray-400",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "1.5",
                                    d: "M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 002 2z",
                                }
                            }
                        }
                        div { class: "p-4",
                            h3 { class: "font-semibold text-lg", {name} }
                            p { class: "text-gray-600 mt-1", {description} }
                            button { class: "mt-3 px-3 py-1 bg-blue-100 text-blue-600 rounded text-sm hover:bg-blue-200 transition-colors",
                                "Select Layout"
                            }
                        }
                    }
                }
            }
        }
    }
}