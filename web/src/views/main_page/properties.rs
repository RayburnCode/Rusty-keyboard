use dioxus::prelude::*;

#[component]
pub fn Properties() -> Element {
    rsx! {
        form { class: "w-full max-w-2xl space-y-4 p-4",
            // Top Legend Section
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", "Top Legend" }
                div { class: "grid grid-cols-4 gap-2",
                    LegendInput {
                        index: 0,
                        hint: "Specify a top-left legend for the keycap",
                    }
                    LegendInput {
                        index: 1,
                        hint: "Specify a top-center legend for the keycap",
                    }
                    LegendInput {
                        index: 2,
                        hint: "Specify a top-right legend for the keycap",
                    }
                }
            }

            // Center Legend Section
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", "Center Legend" }
                div { class: "grid grid-cols-4 gap-2",
                    LegendInput {
                        index: 3,
                        hint: "Specify a center-left legend for the keycap",
                    }
                    LegendInput {
                        index: 4,
                        hint: "Specify a center legend for the keycap",
                    }
                    LegendInput {
                        index: 5,
                        hint: "Specify a center-right legend for the keycap",
                    }
                }
            }

            // Bottom Legend Section
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", "Bottom Legend" }
                div { class: "grid grid-cols-4 gap-2",
                    LegendInput {
                        index: 6,
                        hint: "Specify a bottom-left legend for the keycap",
                    }
                    LegendInput {
                        index: 7,
                        hint: "Specify a bottom-center legend for the keycap",
                    }
                    LegendInput {
                        index: 8,
                        hint: "Specify a bottom-right legend for the keycap",
                    }
                }
            }

            // Front Legend Section
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", "Front Legend" }
                div { class: "grid grid-cols-4 gap-2",
                    LegendInput {
                        index: 9,
                        hint: "Specify a front-left legend for the keycap",
                    }
                    LegendInput {
                        index: 10,
                        hint: "Specify a front-center legend for the keycap",
                    }
                    LegendInput {
                        index: 11,
                        hint: "Specify a front-right legend for the keycap",
                    }
                }
            }

            // Legend Size
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", "Legend Size" }
                div { class: "flex items-center gap-2",
                    input {
                        r#type: "number",
                        min: "1",
                        max: "9",
                        class: "w-16 px-2 py-1 border rounded text-sm",
                        placeholder: "Size",
                    }
                }
            }

            // Legend Color
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", "Legend Color" }
                div { class: "flex items-center gap-2",
                    input {
                        r#type: "color",
                        class: "w-8 h-8 border rounded cursor-pointer",
                    }
                    button { class: "px-2 py-1 text-xs border rounded hover:bg-gray-100",
                        "Swap Colors"
                    }
                }
            }

            // Key Color
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", "Key Color" }
                div { class: "flex items-center gap-2",
                    input {
                        r#type: "color",
                        class: "w-8 h-8 border rounded cursor-pointer",
                    }
                }
            }

            // Width
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", "Width" }
                div { class: "flex items-center gap-2",
                    input {
                        r#type: "number",
                        step: "0.25",
                        min: "0.5",
                        max: "24",
                        class: "w-20 px-2 py-1 border rounded text-sm",
                        placeholder: "Primary",
                    }
                    span { "/" }
                    input {
                        r#type: "number",
                        step: "0.25",
                        min: "0.5",
                        max: "24",
                        class: "w-20 px-2 py-1 border rounded text-sm",
                        placeholder: "Secondary",
                    }
                    button { class: "px-2 py-1 text-xs border rounded hover:bg-gray-100",
                        "Swap"
                    }
                }
            }

            // Height
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", "Height" }
                div { class: "flex items-center gap-2",
                    input {
                        r#type: "number",
                        step: "0.25",
                        min: "0.5",
                        max: "24",
                        class: "w-20 px-2 py-1 border rounded text-sm",
                        placeholder: "Primary",
                    }
                    span { "/" }
                    input {
                        r#type: "number",
                        step: "0.25",
                        min: "0.5",
                        max: "24",
                        class: "w-20 px-2 py-1 border rounded text-sm",
                        placeholder: "Secondary",
                    }
                }
            }

            // Position (X/Y)
            div { class: "grid grid-cols-2 gap-4",
                div { class: "space-y-2",
                    label { class: "block text-sm font-medium text-gray-700", "X Position" }
                    div { class: "flex items-center gap-2",
                        input {
                            r#type: "number",
                            step: "0.25",
                            min: "0",
                            max: "36",
                            class: "w-20 px-2 py-1 border rounded text-sm",
                            placeholder: "Position",
                        }
                        span { "+" }
                        input {
                            r#type: "number",
                            step: "0.25",
                            min: "-6",
                            max: "6",
                            class: "w-20 px-2 py-1 border rounded text-sm",
                            placeholder: "Offset",
                        }
                    }
                }
                div { class: "space-y-2",
                    label { class: "block text-sm font-medium text-gray-700", "Y Position" }
                    div { class: "flex items-center gap-2",
                        input {
                            r#type: "number",
                            step: "0.25",
                            min: "0",
                            max: "36",
                            class: "w-20 px-2 py-1 border rounded text-sm",
                            placeholder: "Position",
                        }
                        span { "+" }
                        input {
                            r#type: "number",
                            step: "0.25",
                            min: "-6",
                            max: "6",
                            class: "w-20 px-2 py-1 border rounded text-sm",
                            placeholder: "Offset",
                        }
                    }
                }
            }

            // Rotation
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", "Rotation" }
                div { class: "flex items-center gap-2",
                    input {
                        r#type: "number",
                        step: "15",
                        min: "-180",
                        max: "180",
                        class: "w-20 px-2 py-1 border rounded text-sm",
                        placeholder: "Degrees",
                    }
                    span { "°" }
                    input {
                        r#type: "number",
                        step: "0.25",
                        min: "0",
                        max: "36",
                        class: "w-20 px-2 py-1 border rounded text-sm",
                        placeholder: "X",
                    }
                    span { "," }
                    input {
                        r#type: "number",
                        step: "0.25",
                        min: "0",
                        max: "36",
                        class: "w-20 px-2 py-1 border rounded text-sm",
                        placeholder: "Y",
                    }
                }
            }

            // Profile/Row
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", "Profile / Row" }
                input {
                    r#type: "text",
                    class: "w-full px-2 py-1 border rounded text-sm",
                    placeholder: "e.g., DCS R1, DSA, etc.",
                }
            }

            // Switch Type
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", "Switch" }
                div { class: "flex gap-2",
                    select { class: "flex-1 px-2 py-1 border rounded text-sm",
                        option { value: "", "Mount" }
                        option { value: "alps", "Alps Mount" }
                        option { value: "cherry", "Cherry MX Mount" }
                    }
                    select { class: "flex-1 px-2 py-1 border rounded text-sm",
                        option { value: "", "Brand" }
                    }
                    select { class: "flex-1 px-2 py-1 border rounded text-sm",
                        option { value: "", "Type" }
                    }
                }
            }

            // Misc Options
            div { class: "space-y-2",
                label { class: "block text-sm font-medium text-gray-700", "Miscellaneous" }
                div { class: "grid grid-cols-2 gap-2",
                    label { class: "flex items-center gap-2 text-sm",
                        input { r#type: "checkbox", class: "rounded" }
                        span { "Ghosted" }
                    }
                    label { class: "flex items-center gap-2 text-sm",
                        input { r#type: "checkbox", class: "rounded" }
                        span { "Stepped" }
                    }
                    label { class: "flex items-center gap-2 text-sm",
                        input { r#type: "checkbox", class: "rounded" }
                        span { "Homing" }
                    }
                    label { class: "flex items-center gap-2 text-sm",
                        input { r#type: "checkbox", class: "rounded" }
                        span { "Decal" }
                    }
                }
            }
        }
    }
}

#[component]
fn LegendInput(index: u8, hint: &'static str) -> Element {
    rsx! {
        div { class: "space-y-1", title: "{hint}",
            div { class: "flex gap-1",
                input {
                    r#type: "text",
                    class: "flex-1 px-2 py-1 border rounded text-sm",
                    placeholder: "Legend",
                }
                button {
                    class: "w-8 h-8 border rounded",
                    style: "background-color: var(--legend-color);",
                    title: "Select color",
                }
                input {
                    r#type: "number",
                    min: "1",
                    max: "9",
                    class: "w-12 px-1 py-1 border rounded text-sm",
                    placeholder: "Size",
                }
            }
        }
    }
}