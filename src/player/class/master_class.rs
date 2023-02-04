use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MasterClass {
    MasterOfTheElements,
    MasterOfExplosions,
    PsychoSorcerer,
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

impl PromptOption for MasterClass {
    fn option_name(&self) -> String {
        String::from(match self {
            MasterClass::MasterOfTheElements => "Master of the Elements",
            MasterClass::MasterOfExplosions => "Master of Explosions",
            MasterClass::PsychoSorcerer => "Psycho Sorcerer",
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
            MasterClass::MasterOfTheElements => "ME",
            MasterClass::MasterOfExplosions => "EX",
            MasterClass::PsychoSorcerer => "PS",
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
