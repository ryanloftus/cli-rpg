use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpertClass {
    StormSummoner,
    ExplosionExpert,
    InsaneIncanter,
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

impl PromptOption for ExpertClass {
    fn option_name(&self) -> String {
        String::from(match self {
            ExpertClass::StormSummoner => "Storm Summoner",
            ExpertClass::ExplosionExpert => "Explosion Expert",
            ExpertClass::InsaneIncanter => "Insane Incanter",
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
            ExpertClass::InsaneIncanter => "II",
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
