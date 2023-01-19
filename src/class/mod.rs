use serde::{Deserialize, Serialize};

use crate::prompt::{PromptOption, InputPrompt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: &'static str,
    pub unique_id: &'static str,
    // TODO: add stat modifiers and skills and class progressions
}

impl PromptOption for Class {
    fn option_name(&self) -> String {
        String::from(self.name)
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(self.unique_id))
    }
}

pub fn choose_class_prompt(class_options: &[Class]) -> Class {
    let prompt = InputPrompt {
        initial_prompt: String::from("Choose a class."),
        options: class_options.to_vec(),
    };
    let class = prompt.show_and_get_selection();
    return class;
}

mod starter {
    use super::Class;

    pub const SWORDSMAN: Class = Class {
        name: "Swordsman",
        unique_id: "S",
    };
    pub const KNIGHT: Class = Class {
        name: "Knight",
        unique_id: "K",
    };
    pub const BRAWLER: Class = Class {
        name: "Brawler",
        unique_id: "B",
    };
    pub const MAGE: Class = Class {
        name: "Mage",
        unique_id: "M",
    };
}

pub const STARTER_CLASSES: [Class; 4] = [
    starter::SWORDSMAN,
    starter::KNIGHT,
    starter::BRAWLER,
    starter::MAGE,
];
