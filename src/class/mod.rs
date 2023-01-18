pub mod prompt;
use serde::{Deserialize, Serialize};

use crate::prompt::{ToPromptOption, PromptOption};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: &'static str,
    pub unique_id: &'static str,
    // TODO: add stat modifiers and skills and class progressions
}

impl ToPromptOption for Class {
    fn to_prompt_option(&self) -> PromptOption {
        return PromptOption {
            name: String::from(self.name),
            short_name: Some(String::from(self.unique_id)),
        };
    }
}

mod starter {
    use super::Class;

    pub const SWORDSMAN: Class = Class { name: "Swordsman", unique_id: "S" };
    pub const KNIGHT: Class = Class { name: "Knight", unique_id: "K" };
    pub const BRAWLER: Class = Class { name: "Brawler", unique_id: "B" };
    pub const MAGE: Class = Class { name: "Mage", unique_id: "M" };
}

pub const STARTER_CLASSES: [Class; 4] = [
    starter::SWORDSMAN,
    starter::KNIGHT,
    starter::BRAWLER,
    starter::MAGE,
];
