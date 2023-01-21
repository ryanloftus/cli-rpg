use serde::{Serialize, Deserialize};

use crate::prompt::PromptOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntermediateClass {
    ElementalMage,
    HolyMage,
    DarkMage,
    // TODO: add Intermediate Classes
}

impl PromptOption for IntermediateClass {
    fn option_name(&self) -> String {
        String::from(
            match self {
                IntermediateClass::ElementalMage => "Elemental Mage",
                IntermediateClass::HolyMage => "Holy Mage",
                IntermediateClass::DarkMage => "Dark Mage",
            }
        )
    }

    fn short_option_name(&self) -> Option<String> {
        Some(
            String::from(
                match self {
                    IntermediateClass::ElementalMage => "E",
                    IntermediateClass::HolyMage => "H",
                    IntermediateClass::DarkMage => "D",
                }
            )
        )
    }
}
