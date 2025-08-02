use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum AddKeyAmount {
    One,
    Five,
    Ten,
}

#[component]
pub fn KeyboardOptions(
    on_add_keys: EventHandler<usize>,
    on_delete_key: EventHandler<()>,
    on_undo: EventHandler<()>,
    on_redo: EventHandler<()>,
    on_export: EventHandler<()>,
) -> Element {
    let mut add_amount = use_signal(|| AddKeyAmount::One);
    let mut show_dropdown = use_signal(|| false);

    rsx! {
        div { class: "flex flex-row flex-wrap gap-2 p-3 bg-white rounded-lg border border-gray-200 items-center shadow-sm",
            // Add Key button with dropdown
            div { class: "relative",
                button {
                    class: "cursor-pointer  px-3 py-1.5 text-sm bg-white text-gray-700 rounded-md border border-gray-300 hover:bg-gray-50 transition-all flex items-center gap-1",
                    onclick: move |_| show_dropdown.toggle(),
                    "Add Key"
                    svg {
                        class: "w-4 h-4 text-gray-500",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 20 20",
                        fill: "currentColor",
                        path { d: "M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" }
                    }
                }
                if *show_dropdown.read() {
                    div {
                        class: " cursor-pointer  absolute z-10 mt-1 w-32 bg-white rounded-md shadow-lg border border-gray-200",
                        onclick: move |e| e.stop_propagation(),
                        div { class: "py-1",
                            for (amount , label) in [
                                (AddKeyAmount::One, "1 Key"),
                                (AddKeyAmount::Five, "5 Keys"),
                                (AddKeyAmount::Ten, "10 Keys"),
                            ]
                                .iter()
                            {
                                button {
                                    class: format!(
                                        "cursor-pointer  block w-full text-left px-3 py-1.5 text-sm hover:bg-gray-100 {}",
                                        if *add_amount.read() == *amount {
                                            "cursor-pointer  bg-blue-50 text-blue-600"
                                        } else {
                                            "text-gray-700"
                                        },
                                    ),
                                    onclick: move |_| {
                                        add_amount.set(amount.clone());
                                        show_dropdown.set(false);
                                        let count = match amount {
                                            AddKeyAmount::One => 1,
                                            AddKeyAmount::Five => 5,
                                            AddKeyAmount::Ten => 10,
                                        };
                                        on_add_keys.call(count);
                                    },
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }

            // Action buttons
            button {
                class: "cursor-pointer  px-3 py-1.5 text-sm bg-white text-red-600 rounded-md border border-red-200 hover:bg-red-50 transition-all",
                onclick: move |_| on_delete_key.call(()),
                "Delete"
            }
            button {
                class: "cursor-pointer px-3 py-1.5 text-sm bg-white text-gray-600 rounded-md border border-gray-200 hover:bg-gray-50 transition-all",
                onclick: move |_| on_undo.call(()),
                "Undo"
            }
            button {
                class: "cursor-pointer px-3 py-1.5 text-sm bg-white text-gray-600 rounded-md border border-gray-200 hover:bg-gray-50 transition-all",
                onclick: move |_| on_redo.call(()),
                "Redo"
            }
            button {
                class: "cursor-pointer px-3 py-1.5 text-sm bg-blue-600 text-white rounded-md border border-blue-600 hover:bg-blue-700 transition-all",
                onclick: move |_| on_export.call(()),
                "Export"
            }
        }
    }
}