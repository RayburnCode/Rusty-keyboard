// This page will the properties of the keyboard and be able to generate JSON output
// Should sync back and forth if the code is editred here, or if the actual Layout display is changed

use dioxus::prelude::*;


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
