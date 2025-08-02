// client/components/layout/app_layout.rs
use dioxus::prelude::*;


#[component]
pub fn Wiring() -> Element {
    rsx! {
        p { "Ability to flip the wiring view to see the backside like when you solder it" }

        p { "Wiring view allows you to visualize the wiring of your keyboard." }
        p { "# Rows" }
        p { "# Columns" }
        p { "Ability to adjust where the wiring is showing on the Layout" }
    }
}