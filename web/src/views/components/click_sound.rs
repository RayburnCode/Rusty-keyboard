use dioxus::prelude::*;



#[component]
pub fn ClickSound() -> Element {

    fn play_sound(sound: &str) {
        // TODO: Implement sound playing logic, e.g., using web_sys or another audio API
        // For now, just log to the console as a placeholder
        web_sys::console::log_1(&format!("Playing sound: {}", sound).into());
    }

    rsx! {
        // this will have a repository of the various keyboard sounds so that users can try what they like and have a list of different keys
        div { class: "p-4",
            h2 { class: "text-lg font-semibold mb-2", "Keyboard Sounds" }
            // Here you can map over a list of sounds and create a button for each
            for sound in &["Click", "Pop", "Typewriter"] {
                button {
                    class: "border border-gray-300 rounded p-2 m-1",
                    onclick: move |_| play_sound(sound),
                    "{sound} Sound"
                }
            }
        }
    }
}