use dioxus::prelude::*;
mod constants;
use crate::constants::*;

mod key_layouts;
mod views; 
use crate::views::{AppLayout, };
use crate::views::main_page::Home;
use crate::views::firmware::Firmware;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    Home {},

    #[route("/firmware")]
    Firmware {},
 

}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
