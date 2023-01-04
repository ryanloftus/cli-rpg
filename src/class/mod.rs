use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Class {
    pub name: Cow<'static, str>,
    pub unique_id: Cow<'static, str>,
    // TODO: add stat modifiers and skills and class progressions
}

impl Class {
    pub fn to_option_string(&self) -> String {
        format!("({}) {}", self.unique_id, self.name)
    }

    const fn new_const(name: &'static str, unique_id: &'static str) -> Self {
        Self {
            name: Cow::Borrowed(name),
            unique_id: Cow::Borrowed(unique_id),
        }
    }
}

mod starter {
    use super::Class;

    pub const SWORDSMAN: Class = Class::new_const("Swordsman", "S");
    pub const KNIGHT: Class = Class::new_const("Knight", "K");
    pub const BRAWLER: Class = Class::new_const("Brawler", "B");
    pub const MAGE: Class = Class::new_const("Mage", "M");
    pub const HEALER: Class = Class::new_const("Healer", "H");
    pub const GAMBLER: Class = Class::new_const("Gambler", "G");
}

pub const STARTER_CLASSES: [Class; 6] = [
    starter::SWORDSMAN,
    starter::KNIGHT,
    starter::BRAWLER,
    starter::MAGE,
    starter::HEALER,
    starter::GAMBLER,
];
