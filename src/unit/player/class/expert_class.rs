use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

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
