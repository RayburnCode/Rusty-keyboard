pub mod qwerty;
pub mod dvorak;
pub mod colemak;

use crate::views::components::KeyProps;

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutType {
    Qwerty,
    Dvorak,
    Colemak,
}

impl LayoutType {
    pub fn get_keys(&self) -> Vec<KeyProps> {
        match self {
            LayoutType::Qwerty => qwerty::get_qwerty_layout(),
            LayoutType::Dvorak => dvorak::get_dvorak_layout(),
            LayoutType::Colemak => colemak::get_colemak_layout(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            LayoutType::Qwerty => "QWERTY",
            LayoutType::Dvorak => "Dvorak",
            LayoutType::Colemak => "Colemak",
        }
    }
}
