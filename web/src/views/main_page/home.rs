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
        move |_: ()| {
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





    rsx! {
        div { class: "flex flex-col min-h-screen p-4",
            // Instructions

            KeyboardOptions {
                on_add_keys: handle_add_keys,
                on_delete_key: handle_delete_key,
                on_undo: handle_undo,
                on_redo: handle_redo,
                on_export: handle_export,
            }
            // Main content area - always full width, stacked vertically
            div { class: "flex flex-col gap-4 mt-4",
                // Layout editor - always full width
                div { class: "w-full",
                    LayoutEditor { keys: keys.read().clone() }
                }
                // Tabs - always full width
                div { class: "w-full", KeyboardTabs {} }
            }
        }
    }
}






