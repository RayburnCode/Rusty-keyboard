use dioxus::prelude::*;
use crate::views::main_page::{ KeyboardOptions, LayoutEditor, KeyboardTabs };



/// Home page
#[component]
pub fn Home() -> Element {

    rsx! {
        // Hero section - primary brand section
        section { class: " ", KeyboardOptions {} }
        section { class: " ", LayoutEditor {} }
        section { class: " ", KeyboardTabs {} }
    }
    }





