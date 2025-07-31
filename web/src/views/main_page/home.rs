use dioxus::prelude::*;
use crate::views::main_page::{KeyboardOptions, LayoutEditor, KeyboardTabs};
use crate::views::components::KeyProps;
use crate::key_layouts::{LayoutType, LayoutType::Qwerty};
 
/// Home page with responsive layout
#[component]
pub fn Home() -> Element {
    let keys = use_signal(|| Qwerty.get_keys());
    let history = use_signal(Vec::<Vec<KeyProps>>::new);
    let history_position = use_signal(|| 0_usize);
    let current_layout = use_signal(|| Qwerty);

    // Add keys handler
    let handle_add_keys = {
        let mut keys = keys.clone();
        let mut history = history.clone();
        let mut history_position = history_position.clone();
        move |count: usize| {
            let mut new_keys = keys.write();
            for _ in 0..count {
                new_keys.push(KeyProps::default());
            }
            
            // Save to history
            let mut history_vec = history.write();
            let pos = *history_position.read();
            if pos < history_vec.len().saturating_sub(1) {
                history_vec.truncate(pos + 1);
            }
            history_vec.push(new_keys.clone());
            history_position.set(history_vec.len() - 1);
        }
    };

    // Delete key handler
    let handle_delete_key = {
        let mut keys = keys.clone();
        let mut history = history.clone();
        let mut history_position = history_position.clone();
        move |_| {
            let mut current_keys = keys.write();
            if !current_keys.is_empty() {
                current_keys.pop();
                
                // Save to history
                let mut history_vec = history.write();
                let pos = *history_position.read();
                if pos < history_vec.len().saturating_sub(1) {
                    history_vec.truncate(pos + 1);
                }
                history_vec.push(current_keys.clone());
                history_position.set(history_vec.len() - 1);
            }
        }
    };

    // Undo handler
    let handle_undo = {
        let mut keys = keys.clone();
        let history = history.clone();
        let mut history_position = history_position.clone();
        move |_| {
            let history_vec = history.read();
            let current_pos = *history_position.read();
            
            if current_pos > 0 {
                let new_pos = current_pos - 1;
                if let Some(previous_state) = history_vec.get(new_pos) {
                    keys.set(previous_state.clone());
                    history_position.set(new_pos);
                }
            }
        }
    };

    // Redo handler
    let handle_redo = {
        let mut keys = keys.clone();
        let history = history.clone();
        let mut history_position = history_position.clone();
        move |_| {
            let history_vec = history.read();
            let current_pos = *history_position.read();
            
            if current_pos < history_vec.len().saturating_sub(1) {
                let new_pos = current_pos + 1;
                if let Some(next_state) = history_vec.get(new_pos) {
                    keys.set(next_state.clone());
                    history_position.set(new_pos);
                }
            }
        }
    };

    // Export handler
    let handle_export = move |_| {
        // TODO: Implement export functionality
        println!("Export functionality to be implemented");
    };

    // Layout change handlers
    let handle_layout_change_qwerty = {
        let mut keys = keys.clone();
        let mut current_layout = current_layout.clone();
        let mut history = history.clone();
        let mut history_position = history_position.clone();
        move |_| {
            let new_keys = Qwerty.get_keys();
            keys.set(new_keys.clone());
            current_layout.set(Qwerty);
            
            // Save to history
            let mut history_vec = history.write();
            let pos = *history_position.read();
            if pos < history_vec.len().saturating_sub(1) {
                history_vec.truncate(pos + 1);
            }
            history_vec.push(new_keys);
            history_position.set(history_vec.len() - 1);
        }
    };

    let handle_layout_change_dvorak = {
        let mut keys = keys.clone();
        let mut current_layout = current_layout.clone();
        let mut history = history.clone();
        let mut history_position = history_position.clone();
        move |_| {
            let new_keys = LayoutType::Dvorak.get_keys();
            keys.set(new_keys.clone());
            current_layout.set(LayoutType::Dvorak);
            
            // Save to history
            let mut history_vec = history.write();
            let pos = *history_position.read();
            if pos < history_vec.len().saturating_sub(1) {
                history_vec.truncate(pos + 1);
            }
            history_vec.push(new_keys);
            history_position.set(history_vec.len() - 1);
        }
    };

    let handle_layout_change_colemak = {
        let mut keys = keys.clone();
        let mut current_layout = current_layout.clone();
        let mut history = history.clone();
        let mut history_position = history_position.clone();
        move |_| {
            let new_keys = LayoutType::Colemak.get_keys();
            keys.set(new_keys.clone());
            current_layout.set(LayoutType::Colemak);
            
            // Save to history
            let mut history_vec = history.write();
            let pos = *history_position.read();
            if pos < history_vec.len().saturating_sub(1) {
                history_vec.truncate(pos + 1);
            }
            history_vec.push(new_keys);
            history_position.set(history_vec.len() - 1);
        }
    };

    rsx! {
        div { class: "flex flex-col min-h-screen p-4",
            // Instructions
            p { class: "text-red-500 text-lg font-medium mb-4",
                "Use arrow keys or mouse to move around the keyboard layout"
            }
            // Layout selector
            div { class: "mb-4 p-4 bg-white rounded-lg shadow",
                h3 { class: "text-lg font-semibold mb-2", "Keyboard Layout" }
                div { class: "flex gap-2",
                    button {
                        class: if *current_layout.read() == Qwerty { "px-4 py-2 bg-blue-500 text-white rounded" } else { "px-4 py-2 bg-gray-200 rounded hover:bg-gray-300" },
                        onclick: handle_layout_change_qwerty,
                        "QWERTY"
                    }
                    button {
                        class: if *current_layout.read() == LayoutType::Dvorak { "px-4 py-2 bg-blue-500 text-white rounded" } else { "px-4 py-2 bg-gray-200 rounded hover:bg-gray-300" },
                        onclick: handle_layout_change_dvorak,
                        "Dvorak"
                    }
                    button {
                        class: if *current_layout.read() == LayoutType::Colemak { "px-4 py-2 bg-blue-500 text-white rounded" } else { "px-4 py-2 bg-gray-200 rounded hover:bg-gray-300" },
                        onclick: handle_layout_change_colemak,
                        "Colemak"
                    }
                }
            }
            // Control options at top
            KeyboardOptions {
                on_add_keys: handle_add_keys,
                on_delete_key: handle_delete_key,
                on_undo: handle_undo,
                on_redo: handle_redo,
                on_export: handle_export,
            }
            // Main content area - responsive row/column layout
            div { class: "flex flex-col lg:flex-row gap-4 mt-4",
                // Layout editor takes 2/3 space on desktop, full width on mobile
                div { class: "w-full lg:w-2/3",
                    LayoutEditor { keys: keys.read().clone() }
                }
                // Tabs take 1/3 space on desktop, full width on mobile
                div { class: "w-full lg:w-1/3", KeyboardTabs {} }
            }
        }
    }
}






