use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Microcontroller {
    ProMicro,
    EliteC,
    RP2040,
    STM32,
    Teensy,
    Custom(String),
}

#[component]
pub fn Pins(
    rows: usize,
    columns: usize,
    on_pin_config: EventHandler<Vec<(String, String)>>,
) -> Element {
    let selected_mcu = use_signal(|| Microcontroller::ProMicro);
    let pin_assignments = use_signal(|| vec![("".to_string(), "".to_string()); rows + columns]);

    // Common pin layouts for different MCUs
    let mcu_pins = {
        let mut map = std::collections::HashMap::new();
        map.insert(
            Microcontroller::ProMicro,
            vec!["D3", "D2", "D1", "D0", "D4", "C6", "D7", "E6", "B4", "B5", "B6", "B2", "B3", "B1", "F7", "F6", "F5", "F4"],
        );
        map.insert(
            Microcontroller::EliteC,
            vec!["D3", "D2", "D1", "D0", "D4", "C6", "D7", "E6", "B4", "B5", "B6", "B2", "B3", "B1", "F7", "F6", "F5", "F4"],
        );
        map.insert(
            Microcontroller::RP2040,
            vec!["GP0", "GP1", "GP2", "GP3", "GP4", "GP5", "GP6", "GP7", "GP8", "GP9", 
                 "GP10", "GP11", "GP12", "GP13", "GP14", "GP15", "GP16", "GP17", "GP18", "GP19"],
        );
        map
    };

    // Auto-assign pins when MCU changes
    use_effect(move || {
        if let Some(pins) = mcu_pins.get(&selected_mcu.read()) {
            let mut assignments = Vec::new();
            
            // Assign rows first
            for i in 0..rows {
                if i < pins.len() {
                    assignments.push((format!("Row {}", i), pins[i].to_string()));
                }
            }
            
            // Then assign columns
            for i in 0..columns {
                if rows + i < pins.len() {
                    assignments.push((format!("Col {}", i), pins[rows + i].to_string()));
                }
            }
            
            pin_assignments.set(assignments);
        }
    });

    rsx! {
        div { class: "space-y-6 p-4 bg-white rounded-lg shadow-sm",
            h2 { class: "text-xl font-bold text-gray-800", "Microcontroller & Pin Configuration" }
            // MCU Selection
            div { class: "space-y-2",
                h3 { class: "font-medium text-gray-700", "Select Microcontroller" }
                div { class: "grid grid-cols-2 md:grid-cols-3 gap-2",
                    for mcu in [
                        Microcontroller::ProMicro,
                        Microcontroller::EliteC,
                        Microcontroller::RP2040,
                        Microcontroller::STM32,
                        Microcontroller::Teensy,
                        Microcontroller::Custom("Custom".to_string()),
                    ]
                        .iter()
                    {
                        button {
                            class: format!(
                                "p-2 border rounded text-sm {}",
                                if *mcu == *selected_mcu.read() {
                                    "bg-blue-100 border-blue-500"
                                } else {
                                    "border-gray-300 hover:bg-gray-50"
                                },
                            ),
                            onclick: move |_| selected_mcu.set(mcu.clone()),
                            match mcu {
                                Microcontroller::ProMicro => "Pro Micro",
                                Microcontroller::EliteC => "Elite-C",
                                Microcontroller::RP2040 => "RP2040",
                                Microcontroller::STM32 => "STM32",
                                Microcontroller::Teensy => "Teensy",
                                Microcontroller::Custom(_) => "Custom",
                            }
                        }
                    }
                }
            }

            // Pin Assignment Table
            div { class: "space-y-4",
                h3 { class: "font-medium text-gray-700",
                    "Pin Assignment (Rows: {rows}, Columns: {columns})"
                }
                table { class: "min-w-full divide-y divide-gray-200",
                    thead { class: "bg-gray-50",
                        tr {
                            th { class: "px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Type"
                            }
                            th { class: "px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "Number"
                            }
                            th { class: "px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                "MCU Pin"
                            }
                        }
                    }
                    tbody { class: "bg-white divide-y divide-gray-200",
                        for (i , (pin_type , pin)) in pin_assignments.read().iter().enumerate() {
                            tr { key: "{i}",
                                td { class: "px-4 py-2 whitespace-nowrap text-sm font-medium",
                                    if i < rows {
                                        "Row"
                                    } else {
                                        "Column"
                                    }
                                }
                                td { class: "px-4 py-2 whitespace-nowrap text-sm",
                                    if i < rows {
                                        i {}
                                    } else {
                                        i-rows {}
                                    }
                                }
                                td { class: "px-4 py-2 whitespace-nowrap text-sm",
                                    select {
                                        class: "border rounded px-2 py-1 text-sm",
                                        value: "{pin}",
                                        onchange: move |e| {
                                            let mut assignments = pin_assignments();
                                            assignments[i].1 = e.value();
                                            pin_assignments.set(assignments);
                                        },
                                        option { value: "", "Unassigned" }
                                        for available_pin in mcu_pins.get(&selected_mcu.read()).unwrap_or(&vec![]) {
                                            option {
                                                value: "{available_pin}",
                                                selected: pin == available_pin,
                                                "{available_pin}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Save Configuration
            button {
                class: "mt-4 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",
                onclick: move |_| on_pin_config.call(pin_assignments()),
                "Save Pin Configuration"
            }
        }
    }
}