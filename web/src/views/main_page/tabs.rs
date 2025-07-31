use dioxus::prelude::*;
use serde_json::json;

/// Home page
#[component]
pub fn KeyboardTabs() -> Element {

    rsx! {
        // Hero section - primary brand section
        p { "JSON Output" }

        p { "Summary" }
        table {
            thead {
                tr {
                    th { "Key Size Summary" }
                }
            }
            tbody {
                tr {
                    td { "1x1" }
                    td { "1x2" }
                    td { "2x1" }
                }
                tr {
                    td { "C" }
                    td { "Copy" }
                    td { "Ctrl + C" }
                }
                tfoot {
                    tr {
                        td { "Total" }
                        td { "3" }
                    }
                }
            }
        }
    }}



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
