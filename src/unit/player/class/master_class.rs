use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

use super::overpowered_class::OverpoweredClass;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MasterClass {
    MasterOfNaturalDisaster,
    MasterOfExplosions,
    ChosenAngel,
    FallenAngel,
    MasterOfTheDarkArts,
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
            MasterClass::ChosenAngel => {
                "Mage with powerful light magic bestowed upon them by the heavens"
            }
            MasterClass::FallenAngel => "Mage with powerful dark and light magic",
            MasterClass::MasterOfTheDarkArts => "Mage with masterful control of dark magic",
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
            MasterClass::ChosenAngel => vec![OverpoweredClass::BenevolentDeity],
            MasterClass::FallenAngel => vec![
                OverpoweredClass::BenevolentDeity,
                OverpoweredClass::GrimReaper,
            ],
            MasterClass::MasterOfTheDarkArts => vec![OverpoweredClass::GrimReaper],
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
}

impl PromptOption for MasterClass {
    fn option_name(&self) -> String {
        String::from(match self {
            MasterClass::MasterOfNaturalDisaster => "Master of Natural Disaster",
            MasterClass::MasterOfExplosions => "Master of Explosions",
            MasterClass::ChosenAngel => "Chosen Angel",
            MasterClass::FallenAngel => "Fallen Angel",
            MasterClass::MasterOfTheDarkArts => "Master of the Dark Arts",
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
            MasterClass::ChosenAngel => "CA",
            MasterClass::FallenAngel => "FA",
            MasterClass::MasterOfTheDarkArts => "DA",
            MasterClass::UltimateSwordmaster => "US",
            MasterClass::BladeConjurer => "BL",
            MasterClass::BlessedCentaurianHero => "BC",
            MasterClass::CursedCentaurianSpecter => "CC",
            MasterClass::EnlightenedPegasusKnight => "EP",
            MasterClass::DraconianKnight => "DK",
        }))
    }
}
