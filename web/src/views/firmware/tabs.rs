use dioxus::prelude::*;
use serde_json::json;


#[derive(PartialEq, Clone)]
enum ActiveTab {
    Summary,
    JsonOutput,
    Properties
}

/// Tab component for switching between keyboard views
#[component]
pub fn FirmwareTabs() -> Element {
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
                            p { class: "text-gray-700 text-sm mb-2", "Properties will be displayed here." }
                        }
                    },
                    ActiveTab::Summary => rsx! {
                        div {
                            table { class: "min-w-full divide-y divide-gray-200",
                                thead { class: "bg-gray-50",
                                    tr {
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                            "Key Size Summary"
                                        }
                                    }
                                }
                                tbody { class: "bg-white divide-y divide-gray-200",
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "1x1" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "=2" }
                                    }
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "1x2" }
                                    }
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "2x1" }
                                    }
                                }
                                tfoot { class: "bg-gray-50",
                                    tr {
                                        td { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                            "Total"
                                        }
                                        td { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                            "3"
                                        }
                                    }
                                }
                            }
                        }
                    },
                    ActiveTab::JsonOutput => rsx! {
                        div { class: "bg-gray-100 p-4 rounded-lg", JSONOUT {} }
                    },
                }
            }
        }
    }
}

#[component]
pub fn JSONOUT() -> Element {

let my_json =  json!({
    "keys": [
        [
            {"label": "Num Lock"},
            {"label": "/"},
            {"label": "*"},
            {"label": "-"}
        ],
        [
            {"label": "7", "secondaryLabel": "Home"},
            {"label": "8", "secondaryLabel": "↑"},
            {"label": "9", "secondaryLabel": "PgUp"},
            {"height": 2, "label": "+"}
        ],
        [
            {"label": "4", "secondaryLabel": "←"},
            {"label": "5"},
            {"label": "6", "secondaryLabel": "→"}
        ],
        [
            {"label": "1", "secondaryLabel": "End"},
            {"label": "2", "secondaryLabel": "↓"},
            {"label": "3", "secondaryLabel": "PgDn"},
            {"height": 2, "label": "Enter"}
        ],
        [
            {"width": 2, "label": "0", "secondaryLabel": "Ins"},
            {"label": ".", "secondaryLabel": "Del"}
        ],
        [
            {"align": 7, "label": ""},
            {"label": ""},
            {"label": ""},
            {"label": ""},
            {"label": ""}
        ]
    ]
});

    rsx! {
        p { "{my_json}" }
    }
}
