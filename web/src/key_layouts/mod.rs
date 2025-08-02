pub mod qwerty;


use crate::views::components::KeyProps;

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutType {
    Qwerty,

}

impl LayoutType {
    pub fn get_keys(&self) -> Vec<KeyProps> {
        match self {
            LayoutType::Qwerty => qwerty::get_qwerty_layout(),

        }
    }
}
