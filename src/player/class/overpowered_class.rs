use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverpoweredClass {
    DemonLord,
    GodOfWeather,
    GodOfExplosions,
    ArchitectOfChaos,
    BenevolentDeity,
    GrimReaper,
    GodOfSwords,
    HolyCentaurianKing,
    DarkCentaurianKing,
    GodOfTheSky,
    DraconianHero,
}

impl PromptOption for OverpoweredClass {
    fn option_name(&self) -> String {
        String::from(match self {
            OverpoweredClass::DemonLord => "Demon Lord",
            OverpoweredClass::GodOfWeather => "God of Weather",
            OverpoweredClass::GodOfExplosions => "God of Explosions",
            OverpoweredClass::ArchitectOfChaos => "Architect of Chaos",
            OverpoweredClass::BenevolentDeity => "Benevolent Deity",
            OverpoweredClass::GrimReaper => "Grim Reaper",
            OverpoweredClass::GodOfSwords => "God of Swords",
            OverpoweredClass::HolyCentaurianKing => "Holy Centaurian King",
            OverpoweredClass::DarkCentaurianKing => "Dark Centaurian King",
            OverpoweredClass::GodOfTheSky => "God of the Sky",
            OverpoweredClass::DraconianHero => "Draconian Hero",
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            OverpoweredClass::DemonLord => "DL",
            OverpoweredClass::GodOfWeather => "GW",
            OverpoweredClass::GodOfExplosions => "GE",
            OverpoweredClass::ArchitectOfChaos => "AC",
            OverpoweredClass::BenevolentDeity => "BD",
            OverpoweredClass::GrimReaper => "GR",
            OverpoweredClass::GodOfSwords => "GS",
            OverpoweredClass::HolyCentaurianKing => "HC",
            OverpoweredClass::DarkCentaurianKing => "DC",
            OverpoweredClass::GodOfTheSky => "SK",
            OverpoweredClass::DraconianHero => "DH",
        }))
    }
}
