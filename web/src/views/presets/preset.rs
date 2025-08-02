// client/components/layout/app_layout.rs
use dioxus::prelude::*;
use crate::Route;
use crate::key_layouts::LayoutType;
use crate::views::presets::PresetList;
use crate::views::presets::Preview;


#[component]
pub fn Preset() -> Element {
    rsx! {
        PresetList {}
        Preview {}
    }
}