// client/components/layout/app_layout.rs
use dioxus::prelude::*;


#[component]
pub fn Pins() -> Element {
    rsx! {
        p { "Choose the Microcontroller" }

        p { "Configure the row and column pins.
Rows
0
1
2
3
Columns
0
1
2
3
4
5
6
7
8
9
10
11
." }
    }
}