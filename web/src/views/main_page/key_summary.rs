use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct KeySizeSummary {
    size: String,
    count: usize,
}

#[component]
pub fn KeySummary(key_sizes: Vec<KeySizeSummary>) -> Element {
    let total_keys = key_sizes.iter().map(|k| k.count).sum::<usize>();

    rsx! {
        div { class: "overflow-hidden shadow ring-1 ring-black ring-opacity-5 rounded-lg",
            table { class: "min-w-full divide-y divide-gray-200",
                thead { class: "bg-gray-50",
                    tr {
                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-900 uppercase tracking-wider",
                            "Key Size"
                        }
                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-900 uppercase tracking-wider",
                            "Count"
                        }
                    }
                }
                tbody { class: "bg-white divide-y divide-gray-200",
                    for key in key_sizes.iter() {
                        tr { key: "{key.size}",
                            td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900",
                                "{key.size}"
                            }
                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                "{key.count}"
                            }
                        }
                    }
                }
                tfoot { class: "bg-gray-50",
                    tr {
                        td { class: "px-6 py-3 text-left text-xs font-medium text-gray-900 uppercase tracking-wider",
                            "Total Keys"
                        }
                        td { class: "px-6 py-3 text-left text-xs font-medium text-gray-900",
                            "{total_keys}"
                        }
                    }
                }
            }
        }
    }
}