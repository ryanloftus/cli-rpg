use serde::{Deserialize, Serialize};

use crate::{prompt::PromptOption, unit::stats::StatMultiplier};

use super::master_class::MasterClass;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ExpertClass {
    StormSummoner,
    ExplosionExpert,
    Angel,
    DarkAngel,
    Swordmaster,
    MagicDuelWielder,
    HolyCentaur,
    DarkCentaur,
    HolyPegasusKnight,
    DarkPegasusKnight,
    WyvernKnight,
}

impl ExpertClass {
    pub fn description(&self) -> String {
        return String::from(match self {
            ExpertClass::StormSummoner => "Mage with incredible elemental spells",
            ExpertClass::ExplosionExpert => "Mage with incredible explosion magic",
            ExpertClass::Angel => "Powerful light magic user",
            ExpertClass::DarkAngel => "Powerful dark magic user",
            ExpertClass::Swordmaster => "Sword user who has truly mastered the weapon",
            ExpertClass::MagicDuelWielder => {
                "Duel-wielding swordsman who enhances their weapons with magic"
            }
            ExpertClass::HolyCentaur => "Centaurian knight that is able to use light magic",
            ExpertClass::DarkCentaur => "Centaurian knight that is able to use dark magic",
            ExpertClass::HolyPegasusKnight => "Pegasus knight that is able to use light magic",
            ExpertClass::DarkPegasusKnight => "Pegasus knight that is able to use dark magic",
            ExpertClass::WyvernKnight => "Knight that fights atop a wyvern",
        });
    }

    pub fn progressions(&self) -> Vec<MasterClass> {
        return match self {
            ExpertClass::StormSummoner => vec![MasterClass::MasterOfNaturalDisaster],
            ExpertClass::ExplosionExpert => vec![MasterClass::MasterOfExplosions],
            ExpertClass::Angel => vec![MasterClass::ChosenAngel, MasterClass::FallenAngel],
            ExpertClass::DarkAngel => {
                vec![MasterClass::FallenAngel, MasterClass::MasterOfTheDarkArts]
            }
            ExpertClass::Swordmaster => vec![MasterClass::UltimateSwordmaster],
            ExpertClass::MagicDuelWielder => vec![MasterClass::BladeConjurer],
            ExpertClass::HolyCentaur => vec![MasterClass::BlessedCentaurianHero],
            ExpertClass::DarkCentaur => vec![MasterClass::CursedCentaurianSpecter],
            ExpertClass::HolyPegasusKnight => vec![MasterClass::EnlightenedPegasusKnight],
            ExpertClass::DarkPegasusKnight => vec![MasterClass::EnlightenedPegasusKnight],
            ExpertClass::WyvernKnight => vec![MasterClass::DraconianKnight],
        };
    }

    pub fn stat_gain_multipliers(&self) -> Vec<StatMultiplier> {
        return match self {
            ExpertClass::StormSummoner => vec![
                StatMultiplier::Magic(2.5),
                StatMultiplier::Skill(2.0),
                StatMultiplier::MagicResist(2.0),
            ],
            ExpertClass::ExplosionExpert => vec![
                StatMultiplier::Magic(3.0),
                StatMultiplier::Skill(2.0),
                StatMultiplier::Defense(0.5),
                StatMultiplier::Strength(0.5),
            ],
            ExpertClass::Angel => vec![
                StatMultiplier::MaxHealth(1.5),
                StatMultiplier::Strength(0.5),
                StatMultiplier::Magic(2.0),
                StatMultiplier::Skill(2.0),
                StatMultiplier::MagicResist(2.0),
            ],
            ExpertClass::DarkAngel => vec![
                StatMultiplier::Magic(2.0),
                StatMultiplier::Skill(2.25),
                StatMultiplier::MagicResist(2.0),
            ],
            ExpertClass::Swordmaster => vec![
                StatMultiplier::MaxHealth(1.5),
                StatMultiplier::Strength(2.0),
                StatMultiplier::Speed(2.0),
                StatMultiplier::Skill(2.5),
                StatMultiplier::Defense(1.5),
                StatMultiplier::Magic(0.5),
            ],
            ExpertClass::MagicDuelWielder => vec![
                StatMultiplier::MaxHealth(1.5),
                StatMultiplier::Strength(1.5),
                StatMultiplier::Speed(1.5),
                StatMultiplier::Skill(1.5),
                StatMultiplier::Defense(1.5),
                StatMultiplier::MagicResist(1.5),
                StatMultiplier::Magic(1.5),
            ],
            ExpertClass::HolyCentaur => vec![
                StatMultiplier::MaxHealth(3.0),
                StatMultiplier::Strength(1.75),
                StatMultiplier::Defense(1.75),
                StatMultiplier::Magic(1.5),
                StatMultiplier::MagicResist(1.5),
                StatMultiplier::Speed(2.0),
            ],
            ExpertClass::DarkCentaur => vec![
                StatMultiplier::MaxHealth(3.0),
                StatMultiplier::Strength(1.75),
                StatMultiplier::Defense(1.75),
                StatMultiplier::Magic(1.5),
                StatMultiplier::MagicResist(1.5),
                StatMultiplier::Speed(2.0),
            ],
            ExpertClass::HolyPegasusKnight => vec![
                StatMultiplier::MaxHealth(3.0),
                StatMultiplier::Strength(1.75),
                StatMultiplier::Defense(1.75),
                StatMultiplier::Magic(1.5),
                StatMultiplier::MagicResist(1.5),
                StatMultiplier::Speed(2.0),
            ],
            ExpertClass::DarkPegasusKnight => vec![
                StatMultiplier::MaxHealth(3.0),
                StatMultiplier::Strength(1.75),
                StatMultiplier::Defense(1.75),
                StatMultiplier::Magic(1.5),
                StatMultiplier::MagicResist(1.5),
                StatMultiplier::Speed(2.0),
            ],
            ExpertClass::WyvernKnight => vec![
                StatMultiplier::MaxHealth(3.5),
                StatMultiplier::Strength(2.5),
                StatMultiplier::Defense(2.0),
                StatMultiplier::Speed(2.0),
                StatMultiplier::Skill(1.5),
            ],
        };
    }
}

impl PromptOption for ExpertClass {
    fn option_name(&self) -> String {
        String::from(match self {
            ExpertClass::StormSummoner => "Storm Summoner",
            ExpertClass::ExplosionExpert => "Explosion Expert",
            ExpertClass::Angel => "Angel",
            ExpertClass::DarkAngel => "Dark Angel",
            ExpertClass::Swordmaster => "Swordmaster",
            ExpertClass::MagicDuelWielder => "Magic Duel Wielder",
            ExpertClass::HolyCentaur => "Holy Centaur",
            ExpertClass::DarkCentaur => "Dark Centaur",
            ExpertClass::HolyPegasusKnight => "Holy Pegasus Knight",
            ExpertClass::DarkPegasusKnight => "Dark Pegasus Knight",
            ExpertClass::WyvernKnight => "Wyvern Knight",
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            ExpertClass::StormSummoner => "SS",
            ExpertClass::ExplosionExpert => "EE",
            ExpertClass::Angel => "A",
            ExpertClass::DarkAngel => "DA",
            ExpertClass::Swordmaster => "S",
            ExpertClass::MagicDuelWielder => "MD",
            ExpertClass::HolyCentaur => "HC",
            ExpertClass::DarkCentaur => "DC",
            ExpertClass::HolyPegasusKnight => "HP",
            ExpertClass::DarkPegasusKnight => "DP",
            ExpertClass::WyvernKnight => "WK",
        }))
    }
}
