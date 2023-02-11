use serde::{Deserialize, Serialize};

use crate::{prompt::PromptOption, unit::stats::StatMultiplier};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OverpoweredClass {
    LegendaryHero,
    GodOfWeather,
    GodOfExplosions,
    BenevolentDeity,
    GrimReaper,
    GodOfSwords,
    CentaurianKing,
    GodOfTheSky,
    DraconianHero,
}

impl OverpoweredClass {
    pub fn description(&self) -> String {
        String::from(match self {
            OverpoweredClass::LegendaryHero => "A being of unrivaled power, this title is bestowed only upon those who have conquered the greatest evils this world has to offer",
            OverpoweredClass::GodOfWeather => "A being with godlike power over the elements and weather itself",
            OverpoweredClass::GodOfExplosions => "A being with godlike explosive powers",
            OverpoweredClass::BenevolentDeity => "A being with godlike power over light magic",
            OverpoweredClass::GrimReaper => "A being whose purpose in this world is too collect the souls of the villains plaguing this world",
            OverpoweredClass::GodOfSwords => "A being who has surpassed perfection in the art of swordsmanship",
            OverpoweredClass::CentaurianKing => "A being revered as the \"King of Beasts\" for their strength and magic ability",
            OverpoweredClass::GodOfTheSky => "A being with godlike authority over the sky and its inhabitants",
            OverpoweredClass::DraconianHero => "A being with unrivaled strength and the ability to call upon powerful draconian allies",
        })
    }

    pub fn stat_gain_multipliers(&self) -> Vec<StatMultiplier> {
        return match self {
            OverpoweredClass::LegendaryHero => vec![
                StatMultiplier::MaxHealth(5.0),
                StatMultiplier::Strength(5.0),
                StatMultiplier::Magic(5.0),
                StatMultiplier::Defense(5.0),
                StatMultiplier::MagicResist(5.0),
                StatMultiplier::Speed(5.0),
                StatMultiplier::Skill(5.0),
                StatMultiplier::Luck(5.0),
            ],
            OverpoweredClass::GodOfWeather => vec![
                StatMultiplier::MaxHealth(5.0),
                StatMultiplier::Magic(10.0),
                StatMultiplier::Skill(6.0),
                StatMultiplier::MagicResist(10.0),
            ],
            OverpoweredClass::GodOfExplosions => {
                vec![StatMultiplier::Magic(12.0), StatMultiplier::Skill(6.0)]
            }
            OverpoweredClass::BenevolentDeity => vec![
                StatMultiplier::MaxHealth(5.0),
                StatMultiplier::Magic(10.0),
                StatMultiplier::Skill(5.0),
                StatMultiplier::MagicResist(10.0),
            ],
            OverpoweredClass::GrimReaper => vec![
                StatMultiplier::MaxHealth(5.0),
                StatMultiplier::Magic(10.0),
                StatMultiplier::Skill(8.0),
                StatMultiplier::MagicResist(10.0),
            ],
            OverpoweredClass::GodOfSwords => vec![
                StatMultiplier::MaxHealth(5.0),
                StatMultiplier::Strength(8.0),
                StatMultiplier::Magic(8.0),
                StatMultiplier::Defense(4.0),
                StatMultiplier::MagicResist(4.0),
                StatMultiplier::Speed(8.0),
                StatMultiplier::Skill(8.0),
            ],
            OverpoweredClass::CentaurianKing => vec![
                StatMultiplier::MaxHealth(8.0),
                StatMultiplier::Strength(8.0),
                StatMultiplier::Magic(8.0),
                StatMultiplier::Defense(8.0),
                StatMultiplier::MagicResist(8.0),
                StatMultiplier::Speed(8.0),
                StatMultiplier::Skill(2.0),
            ],
            OverpoweredClass::GodOfTheSky => vec![
                StatMultiplier::MaxHealth(8.0),
                StatMultiplier::Strength(8.0),
                StatMultiplier::Magic(8.0),
                StatMultiplier::Defense(8.0),
                StatMultiplier::MagicResist(8.0),
                StatMultiplier::Speed(8.0),
                StatMultiplier::Skill(2.0),
            ],
            OverpoweredClass::DraconianHero => vec![
                StatMultiplier::MaxHealth(8.0),
                StatMultiplier::Strength(8.0),
                StatMultiplier::Magic(8.0),
                StatMultiplier::Defense(8.0),
                StatMultiplier::MagicResist(8.0),
                StatMultiplier::Speed(8.0),
                StatMultiplier::Skill(2.0),
            ],
        };
    }
}

impl PromptOption for OverpoweredClass {
    fn option_name(&self) -> String {
        String::from(match self {
            OverpoweredClass::LegendaryHero => "Demon Lord",
            OverpoweredClass::GodOfWeather => "God of Weather",
            OverpoweredClass::GodOfExplosions => "God of Explosions",
            OverpoweredClass::BenevolentDeity => "Benevolent Deity",
            OverpoweredClass::GrimReaper => "Grim Reaper",
            OverpoweredClass::GodOfSwords => "God of Swords",
            OverpoweredClass::CentaurianKing => "Centaurian King",
            OverpoweredClass::GodOfTheSky => "God of the Sky",
            OverpoweredClass::DraconianHero => "Draconian Hero",
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            OverpoweredClass::LegendaryHero => "DL",
            OverpoweredClass::GodOfWeather => "GW",
            OverpoweredClass::GodOfExplosions => "GE",
            OverpoweredClass::BenevolentDeity => "BD",
            OverpoweredClass::GrimReaper => "GR",
            OverpoweredClass::GodOfSwords => "GS",
            OverpoweredClass::CentaurianKing => "CK",
            OverpoweredClass::GodOfTheSky => "SK",
            OverpoweredClass::DraconianHero => "DH",
        }))
    }
}
