use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

use super::advanced_class::AdvancedClass;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IntermediateClass {
    ElementalMage,
    HolyMage,
    DarkMage,
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
            IntermediateClass::Duelist => "Swordsman that focuses on 1-on-1 battles",
            IntermediateClass::MagicSwordsman => "Swordsman that branches out to use magic as well",
            IntermediateClass::HonourableKnight => "Knight that walks the path of justice",
            IntermediateClass::SavageKnight => "Knight that values victory at all costs",
            IntermediateClass::MountedKnight => "Knight that rides a horse",
        });
    }

    pub fn progressions(&self) -> Vec<AdvancedClass> {
        return match self {
            IntermediateClass::ElementalMage => vec![
                AdvancedClass::PowerfulElementalMage,
                AdvancedClass::ExplosiveMage,
            ],
            IntermediateClass::HolyMage => vec![AdvancedClass::StudentOfTheLight],
            IntermediateClass::DarkMage => vec![AdvancedClass::AcolyteOfDarkness],
            IntermediateClass::Duelist => vec![
                AdvancedClass::ImprovingSwordsman,
                AdvancedClass::DuelWielder,
            ],
            IntermediateClass::MagicSwordsman => vec![AdvancedClass::EnchantedBladeWielder],
            IntermediateClass::HonourableKnight => vec![AdvancedClass::HolyKnight],
            IntermediateClass::SavageKnight => vec![AdvancedClass::DarkKnight],
            IntermediateClass::MountedKnight => vec![AdvancedClass::Centaur],
        };
    }
}

impl PromptOption for IntermediateClass {
    fn option_name(&self) -> String {
        String::from(match self {
            IntermediateClass::ElementalMage => "Elemental Mage",
            IntermediateClass::HolyMage => "Holy Mage",
            IntermediateClass::DarkMage => "Dark Mage",
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
            IntermediateClass::Duelist => "D",
            IntermediateClass::MagicSwordsman => "MS",
            IntermediateClass::HonourableKnight => "HK",
            IntermediateClass::SavageKnight => "SK",
            IntermediateClass::MountedKnight => "MK",
        }))
    }
}
