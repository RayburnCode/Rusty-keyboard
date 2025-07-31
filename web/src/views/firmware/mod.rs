mod compile;
mod pins;
mod wiring;

pub use compile::Compile;
use dioxus::html::mo;
pub use pins::Pins;
pub use wiring::Wiring;

mod firmware;
pub use firmware::Firmware;

mod tabs;
pub use tabs::FirmwareTabs;