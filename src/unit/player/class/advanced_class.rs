use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

use super::expert_class::ExpertClass;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
}

impl AdvancedClass {
    pub fn description(&self) -> String {
        return String::from(match self {
            AdvancedClass::PowerfulElementalMage => "Mage that casts a variety of powerful spells",
            AdvancedClass::ExplosiveMage => "Mage specializing in explosive spells",
            AdvancedClass::ErraticEnchanter => {
                "Mage that casts spells with unpredictable and confusing effects"
            }
            AdvancedClass::StudentOfTheLight => "Mage training to wield powerful light magic",
            AdvancedClass::AcolyteOfDarkness => "Mage training to wield powerful dark magic",
            AdvancedClass::ImprovingSwordsman => "Swordsman training to hone his fighting skills",
            AdvancedClass::DuelWielder => "Swordsman that fights with two swords",
            AdvancedClass::EnchantedBladeWielder => {
                "Swordsman that uses magic to enhance their weapon"
            }
            AdvancedClass::HolyKnight => "Knight that can cast light magic",
            AdvancedClass::DarkKnight => "Knight that can cast dark magic",
            AdvancedClass::Centaur => "Knight that has truly become one with their steed",
            AdvancedClass::PegasusKnight => "Knight that fights atop a powerful winged horse",
        });
    }

    pub fn progressions(&self) -> Vec<ExpertClass> {
        return match self {
            AdvancedClass::PowerfulElementalMage => vec![ExpertClass::StormSummoner],
            AdvancedClass::ExplosiveMage => vec![ExpertClass::ExplosionExpert],
            AdvancedClass::ErraticEnchanter => vec![ExpertClass::InsaneIncanter],
            AdvancedClass::StudentOfTheLight => vec![ExpertClass::Angel],
            AdvancedClass::AcolyteOfDarkness => vec![ExpertClass::DarkAngel],
            AdvancedClass::ImprovingSwordsman => vec![ExpertClass::Swordmaster],
            AdvancedClass::DuelWielder => vec![ExpertClass::MagicDuelWielder],
            AdvancedClass::EnchantedBladeWielder => vec![ExpertClass::MagicDuelWielder],
            AdvancedClass::HolyKnight => {
                vec![ExpertClass::HolyCentaur, ExpertClass::HolyPegasusKnight]
            }
            AdvancedClass::DarkKnight => {
                vec![ExpertClass::DarkCentaur, ExpertClass::DarkPegasusKnight]
            }
            AdvancedClass::Centaur => vec![ExpertClass::DarkCentaur, ExpertClass::HolyCentaur],
            AdvancedClass::PegasusKnight => vec![
                ExpertClass::HolyPegasusKnight,
                ExpertClass::DarkPegasusKnight,
            ],
        };
    }
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
        }))
    }
}