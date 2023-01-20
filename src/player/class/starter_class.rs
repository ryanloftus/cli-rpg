use serde::{Serialize, Deserialize};

use crate::prompt::PromptOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StarterClass {
    Swordsman,
    Knight,
    Brawler,
    Mage,
}

impl PromptOption for StarterClass {
    fn option_name(&self) -> String {
        String::from(
            match self {
                StarterClass::Swordsman => "Swordsman",
                StarterClass::Knight => "Knight",
                StarterClass::Brawler => "Brawler",
                StarterClass::Mage => "Mage",
            }
        )
    }

    fn short_option_name(&self) -> Option<String> {
        Some(
            String::from(
                match self {
                    StarterClass::Swordsman => "S",
                    StarterClass::Knight => "K",
                    StarterClass::Brawler => "B",
                    StarterClass::Mage => "M",
                }
            )
        )
    }
}
