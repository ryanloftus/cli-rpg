use serde::{Deserialize, Serialize};

use crate::{
    prompt::PromptOption,
    unit::{skill::SkillType, stats::StatMultiplier},
};

use super::overpowered_class::OverpoweredClass;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MasterClass {
    MasterOfNaturalDisaster,
    MasterOfExplosions,
    FallenAngel,
    UltimateSwordmaster,
    BladeConjurer,
    BlessedCentaurianHero,
    CursedCentaurianSpecter,
    EnlightenedPegasusKnight,
    DraconianKnight,
}

impl MasterClass {
    pub fn description(&self) -> String {
        String::from(match self {
            MasterClass::MasterOfNaturalDisaster => {
                "Mage with masterful control over elemental magic"
            }
            MasterClass::MasterOfExplosions => {
                "Mage who has mastered explosion magic nearly to perfection"
            }
            MasterClass::FallenAngel => "Mage with powerful dark and light magic",
            MasterClass::UltimateSwordmaster => "Swordsman who has perfected their technique",
            MasterClass::BladeConjurer => {
                "Swordsman who uses magic to fight with an infinite number of blades"
            }
            MasterClass::BlessedCentaurianHero => {
                "Centaurian knight who has received power from above"
            }
            MasterClass::CursedCentaurianSpecter => {
                "Centaurian knight who has received power from below"
            }
            MasterClass::EnlightenedPegasusKnight => {
                "Pegasus knight able to wield both light and dark magic"
            }
            MasterClass::DraconianKnight => "Knight that fights atop a dragon",
        })
    }

    pub fn progressions(&self) -> Vec<OverpoweredClass> {
        let mut progressions = match self {
            MasterClass::MasterOfNaturalDisaster => vec![OverpoweredClass::GodOfWeather],
            MasterClass::MasterOfExplosions => vec![OverpoweredClass::GodOfExplosions],
            MasterClass::FallenAngel => vec![OverpoweredClass::GrimReaper],
            MasterClass::UltimateSwordmaster => vec![OverpoweredClass::GodOfSwords],
            MasterClass::BladeConjurer => vec![OverpoweredClass::GodOfSwords],
            MasterClass::BlessedCentaurianHero => vec![OverpoweredClass::CentaurianKing],
            MasterClass::CursedCentaurianSpecter => vec![OverpoweredClass::CentaurianKing],
            MasterClass::EnlightenedPegasusKnight => vec![OverpoweredClass::GodOfTheSky],
            MasterClass::DraconianKnight => vec![OverpoweredClass::DraconianHero],
        };
        progressions.push(OverpoweredClass::LegendaryHero);
        return progressions;
    }

    pub fn stat_gain_multipliers(&self) -> Vec<StatMultiplier> {
        return match self {
            MasterClass::MasterOfNaturalDisaster => vec![
                StatMultiplier::MaxHealth(2.0),
                StatMultiplier::Magic(3.0),
                StatMultiplier::Skill(2.5),
                StatMultiplier::MagicResist(2.5),
            ],
            MasterClass::MasterOfExplosions => vec![
                StatMultiplier::Magic(4.0),
                StatMultiplier::Skill(2.5),
                StatMultiplier::Defense(0.5),
                StatMultiplier::Strength(0.5),
            ],
            MasterClass::FallenAngel => vec![
                StatMultiplier::MaxHealth(2.0),
                StatMultiplier::Magic(2.5),
                StatMultiplier::Skill(2.5),
                StatMultiplier::MagicResist(2.5),
            ],
            MasterClass::UltimateSwordmaster => vec![
                StatMultiplier::MaxHealth(2.0),
                StatMultiplier::Strength(3.0),
                StatMultiplier::Speed(3.0),
                StatMultiplier::Skill(3.0),
                StatMultiplier::Defense(2.0),
                StatMultiplier::Magic(0.5),
            ],
            MasterClass::BladeConjurer => vec![
                StatMultiplier::MaxHealth(2.0),
                StatMultiplier::Strength(2.0),
                StatMultiplier::Skill(2.0),
                StatMultiplier::Defense(1.5),
                StatMultiplier::MagicResist(1.5),
                StatMultiplier::Magic(2.5),
            ],
            MasterClass::BlessedCentaurianHero => vec![
                StatMultiplier::MaxHealth(3.0),
                StatMultiplier::Strength(2.0),
                StatMultiplier::Defense(2.0),
                StatMultiplier::Magic(2.0),
                StatMultiplier::MagicResist(1.5),
                StatMultiplier::Speed(2.0),
            ],
            MasterClass::CursedCentaurianSpecter => vec![
                StatMultiplier::MaxHealth(3.0),
                StatMultiplier::Strength(2.0),
                StatMultiplier::Defense(2.0),
                StatMultiplier::Magic(2.0),
                StatMultiplier::MagicResist(1.5),
                StatMultiplier::Speed(2.0),
            ],
            MasterClass::EnlightenedPegasusKnight => vec![
                StatMultiplier::MaxHealth(3.0),
                StatMultiplier::Strength(1.75),
                StatMultiplier::Defense(1.75),
                StatMultiplier::Magic(2.0),
                StatMultiplier::Skill(2.0),
                StatMultiplier::MagicResist(1.5),
                StatMultiplier::Speed(2.0),
            ],
            MasterClass::DraconianKnight => vec![
                StatMultiplier::MaxHealth(4.0),
                StatMultiplier::Strength(3.0),
                StatMultiplier::Defense(2.0),
                StatMultiplier::Speed(2.0),
                StatMultiplier::Skill(1.5),
            ],
        };
    }

    pub fn skills(&self) -> Vec<SkillType> {
        return match self {
            MasterClass::MasterOfNaturalDisaster => {
                vec![SkillType::WeatherForecast, SkillType::ExplosionMagic]
            }
            MasterClass::MasterOfExplosions => vec![SkillType::ExplosionMagic],
            MasterClass::FallenAngel => vec![
                SkillType::LightMagic,
                SkillType::DarkMagic,
                SkillType::DivineBlessing,
                SkillType::UnholyCurse,
            ],
            MasterClass::UltimateSwordmaster => {
                vec![SkillType::SwordStrike, SkillType::DefensiveForm]
            }
            MasterClass::BladeConjurer => vec![SkillType::SwordSummoning],
            MasterClass::BlessedCentaurianHero => vec![
                SkillType::LightMagic,
                SkillType::LanceOfLight,
                SkillType::DivineBlessing,
                SkillType::PiercingStrike,
                SkillType::Shield,
            ],
            MasterClass::CursedCentaurianSpecter => vec![
                SkillType::DarkMagic,
                SkillType::DarkSpear,
                SkillType::UnholyCurse,
                SkillType::PiercingStrike,
                SkillType::Shield,
            ],
            MasterClass::EnlightenedPegasusKnight => vec![
                SkillType::LightMagic,
                SkillType::DivineBlessing,
                SkillType::DarkMagic,
                SkillType::UnholyCurse,
                SkillType::PiercingStrike,
                SkillType::Shield,
                SkillType::DeathFromAbove,
                SkillType::LanceOfLight,
                SkillType::DarkSpear,
            ],
            MasterClass::DraconianKnight => vec![
                SkillType::PiercingStrike,
                SkillType::Shield,
                SkillType::DeathFromAbove,
            ],
        };
    }
}

impl PromptOption for MasterClass {
    fn option_name(&self) -> String {
        String::from(match self {
            MasterClass::MasterOfNaturalDisaster => "Master of Natural Disaster",
            MasterClass::MasterOfExplosions => "Master of Explosions",
            MasterClass::FallenAngel => "Fallen Angel",
            MasterClass::UltimateSwordmaster => "Ultimate Swordmaster",
            MasterClass::BladeConjurer => "Blade Conjurer",
            MasterClass::BlessedCentaurianHero => "Blessed Centaurian Hero",
            MasterClass::CursedCentaurianSpecter => "Cursed Centaurian Specter",
            MasterClass::EnlightenedPegasusKnight => "Enlightened Pegasus Knight",
            MasterClass::DraconianKnight => "Draconian Knight",
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            MasterClass::MasterOfNaturalDisaster => "ME",
            MasterClass::MasterOfExplosions => "EX",
            MasterClass::FallenAngel => "FA",
            MasterClass::UltimateSwordmaster => "US",
            MasterClass::BladeConjurer => "BL",
            MasterClass::BlessedCentaurianHero => "BC",
            MasterClass::CursedCentaurianSpecter => "CC",
            MasterClass::EnlightenedPegasusKnight => "EP",
            MasterClass::DraconianKnight => "DK",
        }))
    }
}
