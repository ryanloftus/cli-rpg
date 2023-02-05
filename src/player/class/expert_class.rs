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

impl ExpertClass {
    pub fn description(&self) -> String {
        return String::from(match self {
            ExpertClass::StormSummoner => "Mage with incredible elemental spells",
            ExpertClass::ExplosionExpert => "Mage with incredible explosion magic",
            ExpertClass::InsaneIncanter => "Mage that mumbles bizarre incantations",
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
