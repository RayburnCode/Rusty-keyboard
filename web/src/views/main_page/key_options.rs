use dioxus::prelude::*;



/// Home page
#[component]
pub fn KeyboardOptions() -> Element {

    rsx! {
        // Hero section - primary brand section
        p { "Add Key" }
        p { "Delete Key" }
        p { "Undo, ctl Z" }
        p { "Redo, Ctl shift Z" }
        p { "Cut" }
        p { "Copy" }
        p { "Paste" }
    }
    }





