use dioxus::prelude::*;
use serde_json::json;

use crate::views::firmware::{Wiring, Pins, Compile, Firmware};


#[derive(PartialEq, Clone)]
enum ActiveTab {
    Wiring,
    Pins,
    Compile,
    //Firmware
}

/// Tab component for switching between keyboard views
#[component]
pub fn FirmwareTabs() -> Element {
    let mut active_tab = use_signal(|| ActiveTab::Wiring);

    let wiring_tab_class = if *active_tab.read() == ActiveTab::Wiring {
        "px-4 py-2 font-medium text-sm focus:outline-none text-blue-600 border-b-2 border-blue-600"
    } else {
        "px-4 py-2 font-medium text-sm focus:outline-none text-gray-500 hover:text-gray-700"
    };

    let pins_tab_class = if *active_tab.read() == ActiveTab::Pins {
        "px-4 py-2 font-medium text-sm focus:outline-none text-blue-600 border-b-2 border-blue-600"
    } else {
        "px-4 py-2 font-medium text-sm focus:outline-none text-gray-500 hover:text-gray-700"
    };

    let compile_tab_class = if *active_tab.read() == ActiveTab::Compile {
        "px-4 py-2 font-medium text-sm focus:outline-none text-blue-600 border-b-2 border-blue-600"
    } else {
        "px-4 py-2 font-medium text-sm focus:outline-none text-gray-500 hover:text-gray-700"
    };

    rsx! {
        div { class: "flex flex-col w-full",
            // Tab headers
            div { class: "flex border-b border-gray-200",

                button {
                    class: wiring_tab_class,
                    onclick: move |_| active_tab.set(ActiveTab::Wiring),
                    "Wiring"
                }
                button {
                    class: pins_tab_class,
                    onclick: move |_| active_tab.set(ActiveTab::Pins),
                    "Pins"
                }
                button {
                    class: compile_tab_class,
                    onclick: move |_| active_tab.set(ActiveTab::Compile),
                    "Compile"
                }
            }

            // Tab content
            div { class: "p-4",
                match *active_tab.read() {
                    ActiveTab::Wiring => rsx! {
                        div { class: "bg-gray-100 p-4 rounded-lg", Wiring {} }
                    },
                    ActiveTab::Pins => rsx! {
                        div { class: "bg-gray-100 p-4 rounded-lg",
                            p { class: "text-gray-700 text-sm mb-2", Pins {} }
                        }
                    },
                    ActiveTab::Compile => rsx! {
                        div { class: "bg-gray-100 p-4 rounded-lg", Compile {} }
                    },
                }
            }
        }
    }
}

