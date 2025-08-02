use dioxus::prelude::*;
use serde_json::json;

use crate::views::main_page::{Properties,JSONOUT,KeySummary};

#[derive(PartialEq, Clone)]
enum ActiveTab {
    Summary,
    JsonOutput,
    Properties
}

/// Tab component for switching between keyboard views
#[component]
pub fn KeyboardTabs() -> Element {
    let mut active_tab = use_signal(|| ActiveTab::Properties);

    let properties_tab_class = if *active_tab.read() == ActiveTab::Properties {
        "px-4 py-2 font-medium text-sm focus:outline-none text-blue-600 border-b-2 border-blue-600"
    } else {
        "px-4 py-2 font-medium text-sm focus:outline-none text-gray-500 hover:text-gray-700"
    };

    let summary_tab_class = if *active_tab.read() == ActiveTab::Summary {
        "px-4 py-2 font-medium text-sm focus:outline-none text-blue-600 border-b-2 border-blue-600"
    } else {
        "px-4 py-2 font-medium text-sm focus:outline-none text-gray-500 hover:text-gray-700"
    };

    let json_tab_class = if *active_tab.read() == ActiveTab::JsonOutput {
        "px-4 py-2 font-medium text-sm focus:outline-none text-blue-600 border-b-2 border-blue-600"
    } else {
        "px-4 py-2 font-medium text-sm focus:outline-none text-gray-500 hover:text-gray-700"
    };

    rsx! {
        div { class: "flex flex-col w-full",
            // Tab headers
            div { class: "flex border-b border-gray-200",

                button {
                    class: properties_tab_class,
                    onclick: move |_| active_tab.set(ActiveTab::Properties),
                    "Properties"
                }
                button {
                    class: summary_tab_class,
                    onclick: move |_| active_tab.set(ActiveTab::Summary),
                    "Summary"
                }
                button {
                    class: json_tab_class,
                    onclick: move |_| active_tab.set(ActiveTab::JsonOutput),
                    "JSON Output"
                }
            }

            // Tab content
            div { class: "p-4",
                match *active_tab.read() {
                    ActiveTab::Properties => rsx! {
                        div { class: "bg-gray-100 p-4 rounded-lg",
                            p { class: "text-gray-700 text-sm mb-2", Properties {} }
                        }
                    },
                    ActiveTab::Summary => rsx! {
                        div { class: "bg-gray-100 p-4 rounded-lg",
                            p { class: "text-gray-700 text-sm mb-2",
                                KeySummary { key_sizes: vec![] }
                            }
                        }
                    },
                    ActiveTab::JsonOutput => rsx! {
                        div { class: "bg-gray-100 p-4 rounded-lg",
                            JSONOUT { keys: vec![] }
                        }
                    },
                }
            }
        }
    }
}

