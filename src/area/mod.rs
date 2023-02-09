mod caves;
mod forest;
mod icefields;
mod islands;
mod kingdom;
mod mountains;
mod plains;

use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;
use crate::unit::enemy::soldier::SoldierType;
use crate::unit::enemy::{Enemy, EnemyType};

use rand::{self, Rng};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Area {
    Kingdom = 0,
    Plains,
    Forest,
    Islands,
    Caves,
    Icefields,
    Mountains,
}

impl Area {
    pub fn area_at(idx: usize) -> Area {
        return match idx {
            0 => Area::Kingdom,
            1 => Area::Plains,
            2 => Area::Forest,
            3 => Area::Islands,
            4 => Area::Caves,
            5 => Area::Icefields,
            6 => Area::Mountains,
            _ => panic!("Invalid area idx"),
        };
    }

    pub fn story(&self) -> Vec<StoryComponent> {
        return match self {
            Area::Kingdom => kingdom::story(),
            Area::Plains => plains::story(),
            Area::Forest => forest::story(),
            Area::Islands => islands::story(),
            Area::Caves => caves::story(),
            Area::Icefields => icefields::story(),
            Area::Mountains => mountains::story(),
        };
    }

    pub fn generate_training_enemies(&self, num_enemies: usize, player_level: u16) -> Vec<Enemy> {
        let mut training_enemies = Vec::new();
        let level = std::cmp::max(6, player_level) - 5;
        for _ in 0..num_enemies {
            let soldier_type = match rand::thread_rng().gen_range(0..=4) {
                0 => SoldierType::Footsoldier,
                1 => SoldierType::Archer,
                2 => SoldierType::Knight,
                3 => SoldierType::Guard,
                4 => SoldierType::Lieutenant,
                _ => panic!(),
            };
            training_enemies.push(Enemy::new(
                EnemyType::Soldier {
                    faction: self.option_name(),
                    soldier_type,
                },
                level,
                0,
            ));
        }
        return training_enemies;
    }
}

impl PromptOption for Area {
    fn option_name(&self) -> String {
        return String::from(match self {
            Area::Kingdom => "The Kingdom",
            Area::Plains => "The Plains",
            Area::Forest => "The Forest",
            Area::Islands => "The Islands",
            Area::Caves => "The Caves",
            Area::Icefields => "The Icefields",
            Area::Mountains => "The Mountains",
        });
    }

    fn short_option_name(&self) -> Option<String> {
        return Some(String::from(match self {
            Area::Kingdom => "K",
            Area::Plains => "P",
            Area::Forest => "F",
            Area::Islands => "I",
            Area::Caves => "C",
            Area::Icefields => "I",
            Area::Mountains => "M",
        }));
    }
}

#[derive(Debug, Clone)]
pub enum StoryComponent {
    Text(String),
    Enemy(Enemy),
    Boss(Enemy),
    TutorialBattle(Enemy),
    // TODO: GainAttribute once player attributes are added (i.e. DemonSlayer after progressing forest)
}

pub const NUM_AREAS: usize = 7;
