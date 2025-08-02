use dioxus::prelude::*;

#[component]
pub fn Wiring(
    rows: usize,
    columns: usize,
    pin_assignments: Vec<(String, String)>,
    on_adjust: EventHandler<(usize, String)>,
) -> Element {
    let show_backside = use_signal(|| false);
    let highlight_pin = use_signal(|| None::<usize>);

    rsx! {
        div { class: "space-y-6 p-4 bg-white rounded-lg shadow-sm",
            div { class: "flex justify-between items-center",
                h2 { class: "text-xl font-bold text-gray-800", "Keyboard Wiring Diagram" }
                button {
                    class: "px-3 py-1 bg-gray-200 rounded text-sm hover:bg-gray-300",
                    onclick: move |_| show_backside.toggle(),
                    if *show_backside.read() {
                        "Show Front View"
                    } else {
                        "Show Solder Side"
                    }
                }
            }

            p { class: "text-gray-600",
                "Visualize and adjust the wiring connections for your keyboard matrix."
            }

            // Wiring Configuration Controls
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                div { class: "space-y-2",
                    label { class: "block text-sm font-medium", "Rows: {rows}" }
                    input {
                        r#type: "range",
                        min: "1",
                        max: "10",
                        value: "{rows}",
                        class: "w-full",
                        disabled: true,
                    }
                }
                div { class: "space-y-2",
                    label { class: "block text-sm font-medium", "Columns: {columns}" }
                    input {
                        r#type: "range",
                        min: "1",
                        max: "15",
                        value: "{columns}",
                        class: "w-full",
                        disabled: true,
                    }
                }
                div { class: "space-y-2",
                    label { class: "block text-sm font-medium", "View Options" }
                    select {
                        class: "w-full border rounded px-2 py-1 text-sm",
                        onchange: move |e| highlight_pin.set(Some(e.value().parse().unwrap_or(0))),
                        option { value: "", "Highlight Pin..." }
                        for (i, (_, pin)) in pin_assignments.iter().enumerate() {
                            option { value: "{i}", "{pin}" }
                        }
                    }
                }
            }

            // Interactive Wiring Diagram
            div { class: "mt-6 border rounded-lg overflow-hidden",
                div { class: "p-4 bg-gray-50 border-b",
                    h3 { class: "font-medium",
                        if *show_backside.read() {
                            "Solder Side View"
                        } else {
                            "Component Side View"
                        }
                    }
                }
                div { class: "p-4 grid grid-cols-12 gap-2",
                    // Rows
                    for row in 0..rows {
                        div {
                            class: format!(
                                "col-span-2 p-2 rounded border text-center font-mono text-sm {}",
                                if highlight_pin().map_or(false, |p| p == row) {
                                    "bg-yellow-100 border-yellow-400"
                                } else {
                                    "bg-blue-50 border-blue-200"
                                }
                            ),
                            "R{row}: {}",
                            pin_assignments.get(row).map(|(_,p)| p).unwrap_or("?")
                        }
                    }
                    
                    // Matrix Grid
                    for col in 0..columns {
                        for row in 0..rows {
                            div {
                                class: format!(
                                    "col-span-1 p-2 border rounded text-center text-xs {}",
                                    if highlight_pin().map_or(false, |p| p == rows + col) {
                                        "bg-yellow-100 border-yellow-400"
                                    } else {
                                        "bg-gray-50 border-gray-200"
                                    }
                                ),
                                if *show_backside.read() {
                                    svg {
                                        class: "w-6 h-6 mx-auto",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke: "currentColor",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "1.5",
                                            d: "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                                        }
                                    }
                                } else {
                                    "SW{row}{col}"
                                }
                            }
                        }
                    }
                    
                    // Columns
                    for col in 0..columns {
                        div {
                            class: format!(
                                "col-span-2 p-2 rounded border text-center font-mono text-sm {}",
                                if highlight_pin().map_or(false, |p| p == rows + col) {
                                    "bg-yellow-100 border-yellow-400"
                                } else {
                                    "bg-green-50 border-green-200"
                                }
                            ),
                            "C{col}: {}",
                            pin_assignments.get(rows + col).map(|(_,p)| p).unwrap_or("?")
                        }
                    }
                }
            }

            // Connection Legend
            div { class: "mt-4 grid grid-cols-2 gap-4",
                div { class: "p-3 bg-blue-50 rounded border border-blue-200",
                    h4 { class: "font-medium text-sm mb-1", "Row Connections" }
                    div { class: "space-y-1",
                        for (i, (_, pin)) in pin_assignments.iter().take(rows).enumerate() {
                            div { class: "flex justify-between text-xs",
                                span { "Row {i}" }
                                span { class: "font-mono", "{pin}" }
                            }
                        }
                    }
                }
                div { class: "p-3 bg-green-50 rounded border border-green-200",
                    h4 { class: "font-medium text-sm mb-1", "Column Connections" }
                    div { class: "space-y-1",
                        for (i, (_, pin)) in pin_assignments.iter().skip(rows).take(columns).enumerate() {
                            div { class: "flex justify-between text-xs",
                                span { "Column {i}" }
                                span { class: "font-mono", "{pin}" }
                            }
                        }
                    }
                }
            }

            // Adjustment Controls
            div { class: "mt-6 p-4 bg-gray-50 rounded-lg",
                h3 { class: "font-medium text-gray-700 mb-3", "Adjust Wiring" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div {
                        label { class: "block text-sm font-medium mb-1", "Select Connection" }
                        select {
                            class: "w-full border rounded px-2 py-1 text-sm",
                            for (i, (pin_type, _)) in pin_assignments.iter().enumerate() {
                                option { value: "{i}", "{pin_type}" }
                            }
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium mb-1", "New Pin Assignment" }
                        div { class: "flex gap-2",
                            input {
                                class: "flex-1 border rounded px-2 py-1 text-sm",
                                placeholder: "e.g., D2, GP5",
                            }
                            button {
                                class: "px-3 py-1 bg-blue-600 text-white rounded text-sm hover:bg-blue-700",
                                "Update"
                            }
                        }
                    }
                }
            }
        }
    }
}