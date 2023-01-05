#[derive(Debug, Clone)]
pub struct Class {
    pub name: &'static str,
    pub unique_id: &'static str,
    // TODO: add stat modifiers and skills and class progressions
}

impl Class {
    pub fn to_option_string(&self) -> String {
        format!("({}) {}", self.unique_id, self.name)
    }
}

mod starter {
    use super::Class;

    pub const SWORDSMAN: Class = Class { name: "Swordsman", unique_id: "S" };
    pub const KNIGHT: Class = Class { name: "Knight", unique_id: "K" };
    pub const BRAWLER: Class = Class { name: "Brawler", unique_id: "B" };
    pub const MAGE: Class = Class { name: "Mage", unique_id: "M" };
    pub const HEALER: Class = Class { name: "Healer", unique_id: "H" };
    pub const GAMBLER: Class = Class { name: "Gambler", unique_id: "G" };
}

pub const STARTER_CLASSES: [Class; 6] = [
    starter::SWORDSMAN,
    starter::KNIGHT,
    starter::BRAWLER,
    starter::MAGE,
    starter::HEALER,
    starter::GAMBLER,
];
