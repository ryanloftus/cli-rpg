use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdvancedClass {
    PowerfulElementalMage,
    ExplosiveMage,
    ErraticEnchanter,
    StudentOfTheLight,
    AcolyteOfDarkness,
    ImprovingSwordsman,
    DuelWielder,
    EnchantedBladeWielder,
    HolyKnight,
    DarkKnight,
    Centaur,
    PegasusKnight,
    DragonKnight,
}

impl PromptOption for AdvancedClass {
    fn option_name(&self) -> String {
        String::from(match self {
            AdvancedClass::PowerfulElementalMage => "Powerful Elemental Mage",
            AdvancedClass::ExplosiveMage => "Explosive Mage",
            AdvancedClass::ErraticEnchanter => "Erratic Enchanter",
            AdvancedClass::StudentOfTheLight => "Student of the Light",
            AdvancedClass::AcolyteOfDarkness => "Acolyte of Darkness",
            AdvancedClass::ImprovingSwordsman => "Improving Swordsman",
            AdvancedClass::DuelWielder => "Duel Wielder",
            AdvancedClass::EnchantedBladeWielder => "Enchanted Blade Wielder",
            AdvancedClass::HolyKnight => "Holy Knight",
            AdvancedClass::DarkKnight => "Dark Knight",
            AdvancedClass::Centaur => "Centaur",
            AdvancedClass::PegasusKnight => "Pegasus Knight",
            AdvancedClass::DragonKnight => "Dragon Knight",
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            AdvancedClass::PowerfulElementalMage => "PM",
            AdvancedClass::ExplosiveMage => "EM",
            AdvancedClass::ErraticEnchanter => "EE",
            AdvancedClass::StudentOfTheLight => "SL",
            AdvancedClass::AcolyteOfDarkness => "AD",
            AdvancedClass::ImprovingSwordsman => "IS",
            AdvancedClass::DuelWielder => "DW",
            AdvancedClass::EnchantedBladeWielder => "EB",
            AdvancedClass::HolyKnight => "HK",
            AdvancedClass::DarkKnight => "DK",
            AdvancedClass::Centaur => "C",
            AdvancedClass::PegasusKnight => "PK",
            AdvancedClass::DragonKnight => "DR",
        }))
    }
}
