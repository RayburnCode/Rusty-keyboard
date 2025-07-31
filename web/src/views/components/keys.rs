use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct KeyProps {
    #[props(default)]
    pub label: String,
    
    #[props(default)]
    pub secondary_label: String,
    
    #[props(default = 1)]
    pub width: u8,  // in units of standard key width (e.g., 1 or 2)
    
    #[props(default = 1)]
    pub height: u8, // in units of standard key height
    
    #[props(default)]
    pub on_click: EventHandler<()>,
}

#[component]
pub fn Key(props: KeyProps) -> Element {
    let width_class = match props.width {
        2 => "w-24",  // 2x standard width
        _ => "w-12", // standard width
    };

    let height_class = match props.height {
        2 => "h-24",  // 2x standard height
        _ => "h-12",   // standard height
    };

    rsx! {
        button {
            class: "flex flex-col items-center justify-center border border-gray-400 rounded m-0.5 
                   bg-white hover:bg-gray-100 active:bg-gray-200 transition-colors 
                   {width_class} {height_class}",
            onclick: move |_| props.on_click.call(()),
            // Main label
            span { class: "text-sm font-medium", "{props.label}" }
            // Secondary label (if exists)
            if !props.secondary_label.is_empty() {
                span { class: "text-xs text-gray-500", "{props.secondary_label}" }
            }
        }
    }
}