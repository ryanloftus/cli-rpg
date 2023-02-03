use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IntermediateClass {
    ElementalMage,
    HolyMage,
    DarkMage,
    WackyWizard,
    Duelist,
    MagicSwordsman,
    HonourableKnight,
    SavageKnight,
    MountedKnight,
}

impl IntermediateClass {
    pub fn description(&self) -> String {
        return String::from(match self {
            IntermediateClass::ElementalMage => "Mage that learns elemental magic",
            IntermediateClass::HolyMage => "Mage that learns holy magic",
            IntermediateClass::DarkMage => "Mage that learns dark magic",
            IntermediateClass::WackyWizard => "Mage that learns confusing and unpredictable magic",
            IntermediateClass::Duelist => "Swordsman that focuses on 1-on-1 battles",
            IntermediateClass::MagicSwordsman => "Swordsman that branches out to use magic as well",
            IntermediateClass::HonourableKnight => "Knight that walks the path of justice",
            IntermediateClass::SavageKnight => "Knight that values victory at all costs",
            IntermediateClass::MountedKnight => "Knight that rides a horse",
        });
    }
}

impl PromptOption for IntermediateClass {
    fn option_name(&self) -> String {
        String::from(match self {
            IntermediateClass::ElementalMage => "Elemental Mage",
            IntermediateClass::HolyMage => "Holy Mage",
            IntermediateClass::DarkMage => "Dark Mage",
            IntermediateClass::WackyWizard => "Wacky Wizard",
            IntermediateClass::Duelist => "Duelist",
            IntermediateClass::MagicSwordsman => "Magic Swordsman",
            IntermediateClass::HonourableKnight => "Honourable Knight",
            IntermediateClass::SavageKnight => "Savage Knight",
            IntermediateClass::MountedKnight => "Mounted Knight",
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            IntermediateClass::ElementalMage => "EM",
            IntermediateClass::HolyMage => "HM",
            IntermediateClass::DarkMage => "DM",
            IntermediateClass::WackyWizard => "WW",
            IntermediateClass::Duelist => "D",
            IntermediateClass::MagicSwordsman => "MS",
            IntermediateClass::HonourableKnight => "HK",
            IntermediateClass::SavageKnight => "SK",
            IntermediateClass::MountedKnight => "MK",
        }))
    }
}
