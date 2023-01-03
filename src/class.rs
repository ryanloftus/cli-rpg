#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub unique_id: String,
    // TODO: add stat modifiers and skills and class progressions
}

impl Class {
    fn to_string(&self) -> String {
        format!("({}) {}", self.unique_id, self.name)
    }
}

mod starter {
    use super::Class;

    pub const swordsman: Class = Class {
        name: "Swordsman".to_string(),
        unique_id: "S".to_string(),
    };

    pub const knight: Class = Class {
        name: "Knight".to_string(),
        unique_id: "K".to_string(),
    };

    pub const brawler: Class = Class {
        name: "Brawler".to_string(),
        unique_id: "B".to_string(),
    };

    pub const mage: Class = Class {
        name: "Mage".to_string(),
        unique_id: "M".to_string(),
    };

    pub const healer: Class = Class {
        name: "Healer".to_string(),
        unique_id: "H".to_string(),
    };

    pub const gambler: Class = Class {
        name: "Gambler".to_string(),
        unique_id: "G".to_string(),
    };
}

pub const starter_classes: [Class; 6] = [
    starter::swordsman,
    starter::knight,
    starter::brawler,
    starter::mage,
    starter::healer,
    starter::gambler,
];
