use dioxus::prelude::*;
use crate::views::components::{Key, KeyProps};

#[derive(Clone, PartialEq)]
struct KeyPosition {
    pub key: KeyProps,
    pub row: u8,
    pub col: u8,
    pub x_offset: f32, // Fine positioning within row/col
    pub y_offset: f32,
}

/// Check if two keys overlap based on their position and size
fn keys_overlap(key1: &KeyPosition, key2: &KeyPosition) -> bool {
    let x1 = key1.col as f32 + key1.x_offset;
    let y1 = key1.row as f32 + key1.y_offset;
    let w1 = key1.key.width as f32;
    let h1 = key1.key.height as f32;
    
    let x2 = key2.col as f32 + key2.x_offset;
    let y2 = key2.row as f32 + key2.y_offset;
    let w2 = key2.key.width as f32;
    let h2 = key2.key.height as f32;
    
    // Check if rectangles overlap
    !(x1 + w1 <= x2 || x2 + w2 <= x1 || y1 + h1 <= y2 || y2 + h2 <= y1)
}

/// Find a safe position for a key that doesn't overlap with others
fn find_safe_position(target_key: &KeyPosition, all_keys: &[KeyPosition], target_index: usize) -> (u8, u8, f32, f32) {
    let mut test_position = target_key.clone();
    
    // Try the current position first
    let mut has_overlap = false;
    for (i, other_key) in all_keys.iter().enumerate() {
        if i != target_index && keys_overlap(&test_position, other_key) {
            has_overlap = true;
            break;
        }
    }
    
    if !has_overlap {
        return (test_position.row, test_position.col, test_position.x_offset, test_position.y_offset);
    }
    
    // Search for a safe position in a spiral pattern
    let original_row = target_key.row;
    let original_col = target_key.col;
    
    for distance in 1..=5 {
        for dr in -(distance as i8)..=(distance as i8) {
            for dc in -(distance as i8)..=(distance as i8) {
                let new_row = (original_row as i8 + dr).max(0).min(9) as u8;
                let new_col = (original_col as i8 + dc).max(0).min(19) as u8;
                
                test_position.row = new_row;
                test_position.col = new_col;
                test_position.x_offset = 0.0;
                test_position.y_offset = 0.0;
                
                let mut safe = true;
                for (i, other_key) in all_keys.iter().enumerate() {
                    if i != target_index && keys_overlap(&test_position, other_key) {
                        safe = false;
                        break;
                    }
                }
                
                if safe {
                    return (new_row, new_col, 0.0, 0.0);
                }
            }
        }
    }
    
    // If no safe position found, return original with warning
    log::warn!("Could not find safe position for key, potential overlap!");
    (target_key.row, target_key.col, target_key.x_offset, target_key.y_offset)
}

/// Convert KeyProps to JSON string representation with proper positioning
fn key_props_to_json_string(key_pos: &KeyPosition, index: usize) -> String {
    // Calculate actual position including offsets
    let actual_x = key_pos.col as f32 + key_pos.x_offset;
    let actual_y = key_pos.row as f32 + key_pos.y_offset;
    
    format!(r#"{{
  "id": "key_{}",
  "label": "{}",
  "secondaryLabel": "{}",
  "width": {},
  "height": {},
  "x": {:.3},
  "y": {:.3},
  "color": "{}",
  "keycode": "{}",
  "layer": 0
}}"#, 
        index,
        key_pos.key.label,
        key_pos.key.secondary_label,
        key_pos.key.width,
        key_pos.key.height,
        actual_x,
        actual_y,
        key_pos.key.color,
        key_pos.key.label // Default keycode to label
    )
}

#[derive(PartialEq, Props, Clone)]
pub struct LayoutEditorProps {
    pub keys: Vec<KeyProps>,
}

/// Keyboard Layout Editor
#[component]
pub fn LayoutEditor(props: LayoutEditorProps) -> Element {
    let mut zoom_level = use_signal(|| 1.0_f64);
    let mut grid_snap = use_signal(|| 0.25_f32);

    let mut selected_key_json = use_signal(|| String::new());
    let mut selected_key_index = use_signal(|| None::<usize>);
    let mut key_positions = use_signal(|| {
        // Convert input keys to positioned keys with default row/col layout
        props.keys.iter().enumerate().map(|(i, key)| {
            let row = (i / 12) as u8; // 12 keys per row default
            let col = (i % 12) as u8;
            KeyPosition {
                key: key.clone(),
                row,
                col,
                x_offset: 0.0,
                y_offset: 0.0,
            }
        }).collect::<Vec<_>>()
    });

    rsx! {
        div { class: "flex gap-4 p-4 mx-auto w-full max-w-8xl",
            // Left side - Keyboard editor
            div { class: "flex-1 flex flex-col gap-4",
                h2 { class: "text-xl font-bold mb-4", "Keyboard Layout Editor" }
                // Controls row
                div { class: "flex gap-2 items-center justify-between",
                    // Instructions
                    div { class: "text-sm text-gray-600",
                        "Click to select • Use fine controls for precise positioning"
                    }
                    // Zoom and Grid controls
                    div { class: "flex gap-2 items-center",
                        span { class: "text-xs text-gray-600", "Grid: {grid_snap()}" }
                        button {
                            class: "px-2 py-1 bg-purple-200 rounded hover:bg-purple-300 text-xs",
                            onclick: move |_| {
                                let new_snap = match grid_snap() {
                                    x if x >= 1.0 => 0.5,
                                    x if x >= 0.5 => 0.25,
                                    x if x >= 0.25 => 0.125,
                                    _ => 1.0,
                                };
                                grid_snap.set(new_snap);
                            },
                            "Grid {grid_snap()}"
                        }
                        button {
                            class: "px-3 py-1 bg-gray-200 rounded hover:bg-gray-300",
                            onclick: move |_| zoom_level.set((zoom_level() * 0.9_f64).max(0.5_f64)),
                            "Zoom Out -"
                        }
                        span { class: "text-sm w-12 text-center", "{(zoom_level() * 100.0) as i32}%" }
                        button {
                            class: "px-3 py-1 bg-gray-200 rounded hover:bg-gray-300",
                            onclick: move |_| zoom_level.set((zoom_level() * 1.1_f64).min(2.0_f64)),
                            "Zoom In +"
                        }
                    }
                }

                // Arrow key controls for selected key with fine positioning
                if let Some(selected_idx) = *selected_key_index.read() {
                    div { class: "flex flex-col gap-2 p-3 bg-blue-50 rounded border",
                        div { class: "flex gap-2 items-center justify-center",
                            span { class: "text-sm font-medium", "Move Key #{selected_idx + 1}:" }
                            button {
                                class: "p-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                                onclick: move |_| {
                                    let current_positions = key_positions.read().clone();
                                    if let Some(current_key) = current_positions.get(selected_idx) {
                                        let new_row = current_key.row.saturating_sub(1);
                                        let mut test_key = current_key.clone();
                                        test_key.row = new_row;
                                        let mut has_overlap = false;
                                        for (i, other_key) in current_positions.iter().enumerate() {
                                            if i != selected_idx && keys_overlap(&test_key, other_key) {
                                                has_overlap = true;
                                                break;
                                            }
                                        }
                                        if !has_overlap {
                                            let mut positions = key_positions.write();
                                            if let Some(key_pos) = positions.get_mut(selected_idx) {
                                                key_pos.row = new_row;
                                                let json_output = key_props_to_json_string(key_pos, selected_idx);
                                                selected_key_json.set(json_output);
                                            }
                                        } else {
                                            log::warn!("Cannot move key up - would cause overlap!");
                                        }
                                    }
                                },
                                "↑"
                            }
                            button {
                                class: "p-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                                onclick: move |_| {
                                    let current_positions = key_positions.read().clone();
                                    if let Some(current_key) = current_positions.get(selected_idx) {
                                        let new_col = current_key.col.saturating_sub(1);
                                        let mut test_key = current_key.clone();
                                        test_key.col = new_col;
                                        let mut has_overlap = false;
                                        for (i, other_key) in current_positions.iter().enumerate() {
                                            if i != selected_idx && keys_overlap(&test_key, other_key) {
                                                has_overlap = true;
                                                break;
                                            }
                                        }
                                        if !has_overlap {
                                            let mut positions = key_positions.write();
                                            if let Some(key_pos) = positions.get_mut(selected_idx) {
                                                key_pos.col = new_col;
                                                let json_output = key_props_to_json_string(key_pos, selected_idx);
                                                selected_key_json.set(json_output);
                                            }
                                        } else {
                                            log::warn!("Cannot move key left - would cause overlap!");
                                        }
                                    }
                                },
                                "←"
                            }
                            button {
                                class: "p-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                                onclick: move |_| {
                                    let current_positions = key_positions.read().clone();
                                    if let Some(current_key) = current_positions.get(selected_idx) {
                                        let new_row = (current_key.row + 1).min(10);
                                        let mut test_key = current_key.clone();
                                        test_key.row = new_row;
                                        let mut has_overlap = false;
                                        for (i, other_key) in current_positions.iter().enumerate() {
                                            if i != selected_idx && keys_overlap(&test_key, other_key) {
                                                has_overlap = true;
                                                break;
                                            }
                                        }
                                        if !has_overlap {
                                            let mut positions = key_positions.write();
                                            if let Some(key_pos) = positions.get_mut(selected_idx) {
                                                key_pos.row = new_row;
                                                let json_output = key_props_to_json_string(key_pos, selected_idx);
                                                selected_key_json.set(json_output);
                                            }
                                        } else {
                                            log::warn!("Cannot move key down - would cause overlap!");
                                        }
                                    }
                                },
                                "↓"
                            }
                            button {
                                class: "p-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                                onclick: move |_| {
                                    let current_positions = key_positions.read().clone();
                                    if let Some(current_key) = current_positions.get(selected_idx) {
                                        let new_col = (current_key.col + 1).min(20);
                                        let mut test_key = current_key.clone();
                                        test_key.col = new_col;
                                        let mut has_overlap = false;
                                        for (i, other_key) in current_positions.iter().enumerate() {
                                            if i != selected_idx && keys_overlap(&test_key, other_key) {
                                                has_overlap = true;
                                                break;
                                            }
                                        }
                                        if !has_overlap {
                                            let mut positions = key_positions.write();
                                            if let Some(key_pos) = positions.get_mut(selected_idx) {
                                                key_pos.col = new_col;
                                                let json_output = key_props_to_json_string(key_pos, selected_idx);
                                                selected_key_json.set(json_output);
                                            }
                                        } else {
                                            log::warn!("Cannot move key right - would cause overlap!");
                                        }
                                    }
                                },
                                "→"
                            }
                        }
                        // Fine positioning controls
                        div { class: "flex gap-2 items-center justify-center",
                            span { class: "text-xs font-medium", "Fine Position:" }
                            button {
                                class: "px-2 py-1 bg-green-500 text-white rounded hover:bg-green-600 text-xs",
                                onclick: move |_| {
                                    let current_positions = key_positions.read().clone();
                                    if let Some(current_key) = current_positions.get(selected_idx) {
                                        let new_y_offset = (current_key.y_offset - grid_snap()).max(-2.0);
                                        let mut test_key = current_key.clone();
                                        test_key.y_offset = new_y_offset;
                                        let mut has_overlap = false;
                                        for (i, other_key) in current_positions.iter().enumerate() {
                                            if i != selected_idx && keys_overlap(&test_key, other_key) {
                                                has_overlap = true;
                                                break;
                                            }
                                        }
                                        if !has_overlap {
                                            let mut positions = key_positions.write();
                                            if let Some(key_pos) = positions.get_mut(selected_idx) {
                                                key_pos.y_offset = new_y_offset;
                                                let json_output = key_props_to_json_string(key_pos, selected_idx);
                                                selected_key_json.set(json_output);
                                            }
                                        } else {
                                            log::warn!("Cannot fine-move key up - would cause overlap!");
                                        }
                                    }
                                },
                                "↑ {grid_snap()}"
                            }
                            button {
                                class: "px-2 py-1 bg-green-500 text-white rounded hover:bg-green-600 text-xs",
                                onclick: move |_| {
                                    let current_positions = key_positions.read().clone();
                                    if let Some(current_key) = current_positions.get(selected_idx) {
                                        let new_x_offset = (current_key.x_offset - grid_snap()).max(-2.0);
                                        let mut test_key = current_key.clone();
                                        test_key.x_offset = new_x_offset;
                                        let mut has_overlap = false;
                                        for (i, other_key) in current_positions.iter().enumerate() {
                                            if i != selected_idx && keys_overlap(&test_key, other_key) {
                                                has_overlap = true;
                                                break;
                                            }
                                        }
                                        if !has_overlap {
                                            let mut positions = key_positions.write();
                                            if let Some(key_pos) = positions.get_mut(selected_idx) {
                                                key_pos.x_offset = new_x_offset;
                                                let json_output = key_props_to_json_string(key_pos, selected_idx);
                                                selected_key_json.set(json_output);
                                            }
                                        } else {
                                            log::warn!("Cannot fine-move key left - would cause overlap!");
                                        }
                                    }
                                },
                                "← {grid_snap()}"
                            }
                            button {
                                class: "px-2 py-1 bg-green-500 text-white rounded hover:bg-green-600 text-xs",
                                onclick: move |_| {
                                    let current_positions = key_positions.read().clone();
                                    if let Some(current_key) = current_positions.get(selected_idx) {
                                        let new_y_offset = (current_key.y_offset + grid_snap()).min(2.0);
                                        let mut test_key = current_key.clone();
                                        test_key.y_offset = new_y_offset;
                                        let mut has_overlap = false;
                                        for (i, other_key) in current_positions.iter().enumerate() {
                                            if i != selected_idx && keys_overlap(&test_key, other_key) {
                                                has_overlap = true;
                                                break;
                                            }
                                        }
                                        if !has_overlap {
                                            let mut positions = key_positions.write();
                                            if let Some(key_pos) = positions.get_mut(selected_idx) {
                                                key_pos.y_offset = new_y_offset;
                                                let json_output = key_props_to_json_string(key_pos, selected_idx);
                                                selected_key_json.set(json_output);
                                            }
                                        } else {
                                            log::warn!("Cannot fine-move key down - would cause overlap!");
                                        }
                                    }
                                },
                                "↓ {grid_snap()}"
                            }
                            button {
                                class: "px-2 py-1 bg-green-500 text-white rounded hover:bg-green-600 text-xs",
                                onclick: move |_| {
                                    let current_positions = key_positions.read().clone();
                                    if let Some(current_key) = current_positions.get(selected_idx) {
                                        let new_x_offset = (current_key.x_offset + grid_snap()).min(2.0);
                                        let mut test_key = current_key.clone();
                                        test_key.x_offset = new_x_offset;
                                        let mut has_overlap = false;
                                        for (i, other_key) in current_positions.iter().enumerate() {
                                            if i != selected_idx && keys_overlap(&test_key, other_key) {
                                                has_overlap = true;
                                                break;
                                            }
                                        }
                                        if !has_overlap {
                                            let mut positions = key_positions.write();
                                            if let Some(key_pos) = positions.get_mut(selected_idx) {
                                                key_pos.x_offset = new_x_offset;
                                                let json_output = key_props_to_json_string(key_pos, selected_idx);
                                                selected_key_json.set(json_output);
                                            }
                                        } else {
                                            log::warn!("Cannot fine-move key right - would cause overlap!");
                                        }
                                    }
                                },
                                "→ {grid_snap()}"
                            }
                        }
                    
                    }
                }

                // Keyboard container with grid
                div {
                    class: "relative overflow-auto bg-gray-100 border-2 border-dashed border-gray-400 rounded-lg p-4",
                    style: "min-height: 400px;",
                    // Grid background
                    div {
                        class: "absolute inset-4 pointer-events-none",
                        style: "
                            background-image: 
                                linear-gradient(to right, #e5e7eb 1px, transparent 1px),
                                linear-gradient(to bottom, #e5e7eb 1px, transparent 1px);
                            background-size: {60.0 * zoom_level()}px {60.0 * zoom_level()}px;
                            transform: scale({zoom_level()});
                            transform-origin: 0 0;
                        ",
                    }
                    div {
                        class: "keyboard-layout relative",
                        style: "transform: scale({zoom_level()}); transform-origin: 0 0;",
                        // Render positioned keys
                        for (index , key_pos) in key_positions.read().iter().enumerate() {
                            // Check if this key overlaps with any other key outside the rsx! macro
                            div {
                                key: "{index}",
                                class: {
                                    let positions = key_positions.read();
                                    let has_overlap = positions
                                        .iter()
                                        .enumerate()
                                        .any(|(i, other_key)| { i != index && keys_overlap(key_pos, other_key) });
                                    format!(
                                        "absolute cursor-pointer transition-all duration-200 {}{}",
                                        if *selected_key_index.read() == Some(index) {
                                            "ring-2 ring-blue-500 ring-offset-2 "
                                        } else {
                                            ""
                                        },
                                        if has_overlap { "ring-2 ring-red-500 ring-offset-1 bg-red-100" } else { "" },
                                    )
                                },
                                style: "
                                    left: {(key_pos.col as f32 * 60.0) + key_pos.x_offset * 60.0}px;
                                    top: {(key_pos.row as f32 * 60.0) + key_pos.y_offset * 60.0}px;
                                ",
                                onclick: {
                                    let key_pos_clone = key_pos.clone();
                                    let index_copy = index;
                                    move |_| {
                                        let json_output = key_props_to_json_string(&key_pos_clone, index_copy);
                                        selected_key_json.set(json_output);
                                        selected_key_index.set(Some(index_copy));
                                    }
                                },
                                Key {
                                    label: key_pos.key.label.clone(),
                                    secondary_label: key_pos.key.secondary_label.clone(),
                                    width: key_pos.key.width,
                                    height: key_pos.key.height,
                                    x: key_pos.col,
                                    y: key_pos.row,
                                    color: key_pos.key.color.clone(),
                                    on_click: move |_| {},
                                }
                            }
                        }
                    }
                }
            }
            // Right side - JSON output and position info
            div { class: "w-96 flex flex-col gap-4",
                h3 { class: "text-lg font-bold", "Selected Key" }
                // Overlap warning
                {
                    let positions = key_positions.read();
                    let overlap_count = positions
                        .iter()
                        .enumerate()
                        .map(|(i, key)| {
                            positions
                                .iter()
                                .enumerate()
                                .filter(|(j, other)| i != *j && keys_overlap(key, other))
                                .count()
                        })
                        .filter(|count| *count > 0)
                        .count();
                    if overlap_count > 0 {
                        rsx! {
                            div { class: "p-3 bg-red-100 border border-red-400 rounded text-red-700",
                                div { class: "font-bold text-sm", "⚠️ Key Overlaps Detected!" }
                                div { class: "text-xs", "{overlap_count} key(s) have overlapping positions" }
                                div { class: "text-xs", "Overlapping keys are highlighted in red" }
                            }
                        }
                    } else {
                        rsx! {
                            div { class: "p-3 bg-green-100 border border-green-400 rounded text-green-700",
                                div { class: "font-bold text-sm", "✓ No Overlaps" }
                                div { class: "text-xs", "All keys are positioned correctly" }
                            }
                        }
                    }
                }
                if let Some(index) = *selected_key_index.read() {
                    if let Some(key_pos) = key_positions.read().get(index) {
                        div { class: "space-y-2 p-3 bg-gray-50 rounded border",
                            div { class: "text-sm text-gray-600",
                                "Key #{index + 1} - {key_pos.key.label}"
                            }
                            div { class: "text-xs text-gray-500",
                                "Position: Row {key_pos.row}, Col {key_pos.col}"
                            }
                            div { class: "text-xs text-gray-500",
                                "Offset: ({key_pos.x_offset:.1}, {key_pos.y_offset:.1})"
                            }
                        }
                    }
                }
                div { class: "flex-1",
                    textarea {
                        class: "w-full h-64 p-3 border rounded-lg font-mono text-sm resize-none",
                        placeholder: "Click on a key to see its JSON representation...",
                        readonly: true,
                        value: "{selected_key_json}",
                    }
                }

                // Grid controls
                div { class: "space-y-2 p-3 bg-gray-50 rounded border",
                    h4 { class: "font-medium text-sm", "Grid Settings" }
                    div { class: "text-xs text-gray-600",
                        "Grid: 60px spacing • Max: 20 cols × 10 rows"
                    }
                    button {
                        class: "w-full px-3 py-2 bg-green-500 text-white rounded hover:bg-green-600 text-sm",
                        onclick: move |_| {
                            let mut positions = key_positions.write();
                            for (i, pos) in positions.iter_mut().enumerate() {
                                pos.row = (i / 12) as u8;
                                pos.col = (i % 12) as u8;
                                pos.x_offset = 0.0;
                                pos.y_offset = 0.0;
                            }
                            selected_key_json.set(String::new());
                            selected_key_index.set(None);
                        },
                        "Reset Layout"
                    }
                }
            }
        }
    }
}