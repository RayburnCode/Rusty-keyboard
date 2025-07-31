use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum AddKeyAmount {
    One,
    Five,
    Ten,
}

/// Home page
#[component]
pub fn KeyboardOptions(
    on_add_keys: EventHandler<usize>, // Takes number of keys to add
    on_delete_key: EventHandler<()>,
    on_undo: EventHandler<()>,
    on_redo: EventHandler<()>,
    on_export: EventHandler<()>,
) -> Element {
    let mut add_amount = use_signal(|| AddKeyAmount::One);
    let mut show_dropdown = use_signal(|| false);

    rsx! {
        div { class: "flex flex-row flex-wrap gap-2 p-4 bg-gray-100 rounded-lg items-center",
            // Add Key button with dropdown
            div { class: "relative",
                button {
                    class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors flex items-center gap-1",
                    onclick: move |_| {
                        let count = match *add_amount.read() {
                            AddKeyAmount::One => 1,
                            AddKeyAmount::Five => 5,
                            AddKeyAmount::Ten => 10,
                        };
                        on_add_keys.call(count);
                    },
                    "Add Key"
                    svg {
                        class: "w-4 h-4",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 20 20",
                        fill: "currentColor",
                        path { d: "M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" }
                    }
                }
                // Dropdown menu
                if *show_dropdown.read() {
                    div {
                        class: "absolute z-10 mt-1 w-32 bg-white rounded-md shadow-lg",
                        onclick: move |e| e.stop_propagation(),
                        div { class: "py-1",
                            button {
                                class: {
                                    format!(
                                        "block w-full text-left px-4 py-2 text-sm hover:bg-gray-100 {}",
                                        if *add_amount.read() == AddKeyAmount::One {
                                            "bg-blue-50 text-blue-600"
                                        } else {
                                            "text-gray-700"
                                        },
                                    )
                                },
                                onclick: move |_| {
                                    add_amount.set(AddKeyAmount::One);
                                    show_dropdown.set(false);
                                },
                                "Add 1 Key"
                            }
                            button {
                                class: {
                                    format!(
                                        "block w-full text-left px-4 py-2 text-sm hover:bg-gray-100 {}",
                                        if *add_amount.read() == AddKeyAmount::Five {
                                            "bg-blue-50 text-blue-600"
                                        } else {
                                            "text-gray-700"
                                        },
                                    )
                                },
                                onclick: move |_| {
                                    add_amount.set(AddKeyAmount::Five);
                                    show_dropdown.set(false);
                                },
                                "Add 5 Keys"
                            }
                            button {
                                class: {
                                    format!(
                                        "block w-full text-left px-4 py-2 text-sm hover:bg-gray-100 {}",
                                        if *add_amount.read() == AddKeyAmount::Ten {
                                            "bg-blue-50 text-blue-600"
                                        } else {
                                            "text-gray-700"
                                        },
                                    )
                                },
                                onclick: move |_| {
                                    add_amount.set(AddKeyAmount::Ten);
                                    show_dropdown.set(false);
                                },
                                "Add 10 Keys"
                            }
                        }
                    }
                }
            }

            // Other controls
            button {
                class: "px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition-colors",
                disabled: false, // You'd set this based on selection state
                onclick: move |_| on_delete_key.call(()),
                "Delete Key"
            }
            button {
                class: "px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600 transition-colors",
                onclick: move |_| on_undo.call(()),
                "Undo (Ctrl+Z)"
            }
            button {
                class: "px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600 transition-colors",
                onclick: move |_| on_redo.call(()),
                "Redo (Ctrl+Shift+Z)"
            }
            button {
                class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 transition-colors",
                onclick: move |_| on_export.call(()),
                "Export"
            }
        }
    }
}