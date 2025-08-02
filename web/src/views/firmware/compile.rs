// client/components/layout/app_layout.rs
use dioxus::prelude::*;


#[component]
pub fn Compile() -> Element {
    rsx! {

        p {
            "Download the .hex file to flash to your keyboard."
            p { "Download .zip" }
            p {
                "Or download the source files. This will have the Wiring Configuration and the Pin Configuration."
            }
            p { "Download .zip" }
        }
    }
}