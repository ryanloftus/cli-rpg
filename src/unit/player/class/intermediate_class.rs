use serde::{Deserialize, Serialize};

use crate::{
    prompt::PromptOption,
    unit::{skill::SkillType, stats::StatMultiplier},
};

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

    pub fn stat_gain_multipliers(&self) -> Vec<StatMultiplier> {
        return match self {
            IntermediateClass::ElementalMage => vec![
                StatMultiplier::Strength(0.75),
                StatMultiplier::Defense(0.75),
                StatMultiplier::Magic(1.75),
                StatMultiplier::Skill(1.25),
                StatMultiplier::MagicResist(1.5),
            ],
            IntermediateClass::HolyMage => vec![
                StatMultiplier::Strength(0.75),
                StatMultiplier::Magic(1.5),
                StatMultiplier::Skill(1.25),
                StatMultiplier::MagicResist(1.75),
            ],
            IntermediateClass::DarkMage => vec![
                StatMultiplier::Magic(1.75),
                StatMultiplier::Skill(1.5),
                StatMultiplier::MagicResist(1.75),
            ],
            IntermediateClass::Duelist => vec![
                StatMultiplier::Strength(1.5),
                StatMultiplier::Speed(2.0),
                StatMultiplier::Skill(2.0),
                StatMultiplier::Defense(1.5),
                StatMultiplier::Magic(0.5),
            ],
            IntermediateClass::MagicSwordsman => vec![
                StatMultiplier::Strength(1.25),
                StatMultiplier::Speed(1.25),
                StatMultiplier::Skill(1.5),
                StatMultiplier::Defense(1.25),
                StatMultiplier::MagicResist(1.25),
                StatMultiplier::Magic(1.25),
            ],
            IntermediateClass::HonourableKnight => vec![
                StatMultiplier::MaxHealth(2.0),
                StatMultiplier::Strength(1.75),
                StatMultiplier::Defense(1.75),
                StatMultiplier::Magic(0.75),
            ],
            IntermediateClass::SavageKnight => vec![
                StatMultiplier::MaxHealth(1.5),
                StatMultiplier::Strength(2.0),
                StatMultiplier::Defense(1.5),
                StatMultiplier::Magic(0.75),
            ],
            IntermediateClass::MountedKnight => vec![
                StatMultiplier::MaxHealth(2.0),
                StatMultiplier::Strength(1.5),
                StatMultiplier::Defense(1.5),
                StatMultiplier::Magic(0.75),
                StatMultiplier::Speed(1.5),
            ],
        };
    }

    pub fn skills(&self) -> Vec<SkillType> {
        return match self {
            IntermediateClass::ElementalMage => vec![
                SkillType::FireMagic,
                SkillType::WaterMagic,
                SkillType::WindMagic,
                SkillType::LightningMagic,
                SkillType::IceMagic,
            ],
            IntermediateClass::HolyMage => vec![SkillType::LightMagic],
            IntermediateClass::DarkMage => vec![SkillType::DarkMagic],
            IntermediateClass::Duelist => vec![SkillType::SwordStrike, SkillType::DuelStrike],
            IntermediateClass::MagicSwordsman => vec![SkillType::SwordStrike, SkillType::FireMagic],
            IntermediateClass::HonourableKnight => vec![SkillType::Shield],
            IntermediateClass::SavageKnight => vec![SkillType::PiercingStrike],
            IntermediateClass::MountedKnight => vec![SkillType::Shield],
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
