use serde::{Deserialize, Serialize};

use crate::{prompt::PromptOption, unit::stats::StatMultiplier};

use super::intermediate_class::IntermediateClass;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StarterClass {
    Swordsman,
    Knight,
    Mage,
}

impl StarterClass {
    pub fn description(&self) -> String {
        return String::from(match self {
            StarterClass::Swordsman => "A hero who fights a sword and values skill and speed",
            StarterClass::Knight => {
                "A hero who fights with a lance and values strength and defense"
            }
            StarterClass::Mage => "A hero who casts spells and values magic ability",
        });
    }

    pub fn progressions(&self) -> Vec<IntermediateClass> {
        return match self {
            StarterClass::Swordsman => vec![
                IntermediateClass::Duelist,
                IntermediateClass::MagicSwordsman,
            ],
            StarterClass::Knight => vec![
                IntermediateClass::HonourableKnight,
                IntermediateClass::SavageKnight,
                IntermediateClass::MountedKnight,
            ],
            StarterClass::Mage => vec![
                IntermediateClass::ElementalMage,
                IntermediateClass::HolyMage,
                IntermediateClass::DarkMage,
            ],
        };
    }

    pub fn stat_gain_multipliers(&self) -> Vec<StatMultiplier> {
        return match self {
            StarterClass::Swordsman => vec![
                StatMultiplier::Strength(1.25),
                StatMultiplier::Speed(1.5),
                StatMultiplier::Skill(1.5),
                StatMultiplier::Defense(1.25),
                StatMultiplier::Magic(0.75),
            ],
            StarterClass::Knight => vec![
                StatMultiplier::MaxHealth(1.5),
                StatMultiplier::Strength(1.5),
                StatMultiplier::Defense(1.5),
                StatMultiplier::Magic(0.75),
            ],
            StarterClass::Mage => vec![
                StatMultiplier::Strength(0.75),
                StatMultiplier::Defense(0.75),
                StatMultiplier::Magic(1.5),
                StatMultiplier::Skill(1.25),
                StatMultiplier::MagicResist(1.5),
            ],
        };
    }
}

impl PromptOption for StarterClass {
    fn option_name(&self) -> String {
        String::from(match self {
            StarterClass::Swordsman => "Swordsman",
            StarterClass::Knight => "Knight",
            StarterClass::Mage => "Mage",
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            StarterClass::Swordsman => "S",
            StarterClass::Knight => "K",
            StarterClass::Mage => "M",
        }))
    }
}
