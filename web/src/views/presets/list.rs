// client/components/layout/app_layout.rs
use dioxus::prelude::*;
use crate::Route;
use crate::key_layouts::LayoutType;

#[component]
pub fn PresetList() -> Element {
    rsx! {
        p { "Lists the presets in the Key_layouts folder" }

        p {
            "Preview of the preset will have the ability to show a quick preview of the preset, and allow the user to select it to load it into the editor.  "
        }
    }
}